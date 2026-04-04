// Migrated to #[sqlx::test] — manual database setup/teardown removed.
// The macro creates an isolated test database, runs migrations, and
// injects a fresh PgPool into each test. Each test gets its own database
// for full isolation.

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

#[sqlx::test(migrations = "../migrations")]
async fn create_and_find_client(pool: PgPool) {
    let service = make_service(pool);

    let input = RegisterClientInput {
        name: "Integration Test Client".to_string(),
        doc: "12345678909".to_string(),
        email: "test@example.com".to_string(),
        phones: vec!["+5511999999999".to_string()],
        cep: "01001000".to_string(),
        street: "Praca da Se".to_string(),
        number: "1".to_string(),
        complement: "".to_string(),
        neighborhood: "Se".to_string(),
        city: "Sao Paulo".to_string(),
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

#[sqlx::test(migrations = "../migrations")]
async fn list_clients_returns_all(pool: PgPool) {
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

#[sqlx::test(migrations = "../migrations")]
async fn update_client_changes_name(pool: PgPool) {
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

#[sqlx::test(migrations = "../migrations")]
async fn activate_and_deactivate_client(pool: PgPool) {
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

#[sqlx::test(migrations = "../migrations")]
async fn delete_client_removes_row(pool: PgPool) {
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

#[sqlx::test(migrations = "../migrations")]
async fn find_by_document_returns_none_for_missing(pool: PgPool) {
    let service = make_service(pool);

    let result = client::domain::ports::client_use_cases::FindClientByDocumentUseCase::execute(
        &service,
        "99999999999",
    )
    .await
    .expect("find by document");

    assert!(result.is_none());
}

#[sqlx::test(migrations = "../migrations")]
async fn find_by_document_returns_client(pool: PgPool) {
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
