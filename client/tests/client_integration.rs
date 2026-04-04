use client::adapters::driven::postgres::pg_client_repository::PgClientRepository;
use client::application::client_service::ConcreteClientService;
use client::domain::models::db::client_row::ClientStatus;
use client::domain::ports::client_repository::CreateClient;
use client::domain::ports::client_use_cases::{
    ActivateClientUseCase, DeactivateClientUseCase, DeleteClientUseCase, FindClientByIdUseCase,
    ListClientsUseCase, RegisterClientInput, RegisterClientUseCase, UpdateClientInput,
    UpdateClientUseCase,
};
use sqlx::PgPool;

fn make_service(pool: PgPool) -> ConcreteClientService {
    ConcreteClientService::new(PgClientRepository::new(pool.clone()), pool)
}

async fn setup() -> PgPool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let test_db = "milions_db_test";
    let test_url = if url.contains("milions_db") {
        url.replace("milions_db", test_db)
    } else {
        url.clone()
    };

    let admin_pool = sqlx::PgPool::connect(&url).await.expect("connect to admin");

    sqlx::query(&format!(
        "SELECT pg_terminate_backend(pg_stat_activity.pid)
         FROM pg_stat_activity
         WHERE pg_stat_activity.datname = '{}'
         AND pid <> pg_backend_pid()",
        test_db
    ))
    .execute(&admin_pool)
    .await
    .ok();

    sqlx::query(&format!("DROP DATABASE IF EXISTS {}", test_db))
        .execute(&admin_pool)
        .await
        .expect("drop test db");
    sqlx::query(&format!(
        "CREATE DATABASE {} WITH ENCODING 'UTF8' LC_COLLATE='en_US.UTF-8' LC_CTYPE='en_US.UTF-8' TEMPLATE=template0",
        test_db
    ))
    .execute(&admin_pool)
    .await
    .expect("create test db");
    admin_pool.close().await;

    let pool = sqlx::PgPool::connect(&test_url)
        .await
        .expect("connect to test db");
    let manifest = env!("CARGO_MANIFEST_DIR");
    let path = std::path::Path::new(manifest)
        .parent()
        .unwrap()
        .join("migrations");
    let m = sqlx::migrate::Migrator::new(path)
        .await
        .expect("load migrations");
    m.run(&pool).await.expect("run migrations");
    pool
}

#[tokio::test]
async fn create_and_find_client() {
    let pool = setup().await;
    let service = make_service(pool);

    let input = RegisterClientInput {
        name: "Integration Test Client".to_string(),
        doc: "12345678909".to_string(),
        email: "test@example.com".to_string(),
        phones: vec!["+5511999999999".to_string()],
        cep: "01001000".to_string(),
        street: "Praça da Sé".to_string(),
        number: "1".to_string(),
        complement: "".to_string(),
        neighborhood: "Sé".to_string(),
        city: "São Paulo".to_string(),
        state: "SP".to_string(),
    };

    let created = RegisterClientUseCase::execute(&service, input)
        .await
        .expect("create client");

    let found = FindClientByIdUseCase::execute(&service, created.pk_client)
        .await
        .expect("find client");

    assert_eq!(found.pk_client, created.pk_client);
    assert_eq!(found.tx_name, "Integration Test Client");
    assert_eq!(found.tx_doc, "12345678909");
    assert_eq!(found.tx_status, ClientStatus::Active.to_string());
}

#[tokio::test]
async fn list_clients_returns_all() {
    let pool = setup().await;
    let service = make_service(pool.clone());
    let repo = PgClientRepository::new(pool);

    repo.create(client::domain::models::db::client_row::CreateClientRow {
        tx_name: "Client A".to_string(),
        tx_status: ClientStatus::Active,
        tx_doc: "11111111111".to_string(),
    })
    .await
    .expect("create client A");

    repo.create(client::domain::models::db::client_row::CreateClientRow {
        tx_name: "Client B".to_string(),
        tx_status: ClientStatus::Active,
        tx_doc: "22222222222".to_string(),
    })
    .await
    .expect("create client B");

    let clients = ListClientsUseCase::execute(&service)
        .await
        .expect("list clients");

    assert_eq!(clients.len(), 2);
}

#[tokio::test]
async fn update_client_changes_name() {
    let pool = setup().await;
    let service = make_service(pool.clone());
    let repo = PgClientRepository::new(pool);

    let created = repo
        .create(client::domain::models::db::client_row::CreateClientRow {
            tx_name: "Old Name".to_string(),
            tx_status: ClientStatus::Active,
            tx_doc: "33333333333".to_string(),
        })
        .await
        .expect("create client");

    let updated = UpdateClientUseCase::execute(
        &service,
        created.pk_client,
        UpdateClientInput {
            name: Some("New Name".to_string()),
            doc: None,
        },
    )
    .await
    .expect("update client");

    assert_eq!(updated.tx_name, "New Name");
    assert_eq!(updated.tx_doc, "33333333333");
}

#[tokio::test]
async fn activate_and_deactivate_client() {
    let pool = setup().await;
    let service = make_service(pool.clone());
    let repo = PgClientRepository::new(pool);

    let created = repo
        .create(client::domain::models::db::client_row::CreateClientRow {
            tx_name: "Toggle Client".to_string(),
            tx_status: ClientStatus::Inactive,
            tx_doc: "44444444444".to_string(),
        })
        .await
        .expect("create client");

    let active = ActivateClientUseCase::execute(&service, created.pk_client)
        .await
        .expect("activate client");
    assert_eq!(active.tx_status, ClientStatus::Active.to_string());

    let inactive = DeactivateClientUseCase::execute(&service, created.pk_client)
        .await
        .expect("deactivate client");
    assert_eq!(inactive.tx_status, ClientStatus::Inactive.to_string());
}

#[tokio::test]
async fn delete_client_removes_row() {
    let pool = setup().await;
    let service = make_service(pool.clone());
    let repo = PgClientRepository::new(pool);

    let created = repo
        .create(client::domain::models::db::client_row::CreateClientRow {
            tx_name: "To Delete".to_string(),
            tx_status: ClientStatus::Active,
            tx_doc: "55555555555".to_string(),
        })
        .await
        .expect("create client");

    DeleteClientUseCase::execute(&service, created.pk_client)
        .await
        .expect("delete client");

    let found = FindClientByIdUseCase::execute(&service, created.pk_client).await;
    assert!(found.is_err());
}

#[tokio::test]
async fn find_by_document_returns_none_for_missing() {
    let pool = setup().await;
    let service = make_service(pool);

    let result = client::domain::ports::client_use_cases::FindClientByDocumentUseCase::execute(
        &service,
        "99999999999",
    )
    .await
    .expect("find by document");

    assert!(result.is_none());
}

#[tokio::test]
async fn find_by_document_returns_client() {
    let pool = setup().await;
    let service = make_service(pool.clone());
    let repo = PgClientRepository::new(pool);

    repo.create(client::domain::models::db::client_row::CreateClientRow {
        tx_name: "Doc Search".to_string(),
        tx_status: ClientStatus::Active,
        tx_doc: "66666666666".to_string(),
    })
    .await
    .expect("create client");

    let result = client::domain::ports::client_use_cases::FindClientByDocumentUseCase::execute(
        &service,
        "66666666666",
    )
    .await
    .expect("find by document");

    assert!(result.is_some());
    assert_eq!(result.unwrap().tx_name, "Doc Search");
}
