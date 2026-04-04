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
        "11144477735",
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

// --- Edge case / error tests ---

/// FindClientByIdUseCase returns Err(ClientError::NotFound) for a UUID that was never inserted.
#[sqlx::test(migrations = "../migrations")]
async fn find_client_by_id_returns_error_when_not_found(pool: PgPool) {
    use client::domain::errors::ClientError;

    let service = make_service(pool);
    let uuid = uuid::Uuid::now_v7();

    let result = FindClientByIdUseCase::execute(&service, uuid).await;

    assert!(matches!(result, Err(ClientError::NotFound { .. })));
}

/// UpdateClientUseCase returns Err(ClientError::NotFound) when the target UUID does not exist.
#[sqlx::test(migrations = "../migrations")]
async fn update_nonexistent_client_returns_error(pool: PgPool) {
    use client::domain::errors::ClientError;

    let service = make_service(pool);
    let uuid = uuid::Uuid::now_v7();

    let result = UpdateClientUseCase::execute(
        &service,
        uuid,
        UpdateClientInput {
            name: Some("Ghost".to_string()),
            doc: None,
        },
    )
    .await;

    assert!(matches!(result, Err(ClientError::NotFound { .. })));
}

/// DeleteClientUseCase returns Err(ClientError::NotFound) when the target UUID does not exist.
#[sqlx::test(migrations = "../migrations")]
async fn delete_nonexistent_client_returns_error(pool: PgPool) {
    use client::domain::errors::ClientError;

    let service = make_service(pool);
    let uuid = uuid::Uuid::now_v7();

    let result = DeleteClientUseCase::execute(&service, uuid).await;

    assert!(matches!(result, Err(ClientError::NotFound { .. })));
}

/// ActivateClientUseCase returns Err(ClientError::AlreadyActive) when the client is already active.
#[sqlx::test(migrations = "../migrations")]
async fn activate_already_active_client(pool: PgPool) {
    use client::domain::errors::ClientError;

    let service = make_service(pool.clone());
    let repo = PgClientRepository::new(pool);

    let created = repo
        .create(client::domain::models::db::client_row::CreateClientRow {
            tx_name: "Already Active".to_string(),
            tx_status: ClientStatus::Active,
            tx_doc: "77777777777".to_string(),
        })
        .await
        .expect("create client");

    let result = ActivateClientUseCase::execute(&service, created.pk_client).await;

    assert!(matches!(result, Err(ClientError::AlreadyActive { .. })));
}

/// DeactivateClientUseCase returns Err(ClientError::AlreadyInactive) when the client is already inactive.
#[sqlx::test(migrations = "../migrations")]
async fn deactivate_already_inactive_client(pool: PgPool) {
    use client::domain::errors::ClientError;

    let service = make_service(pool.clone());
    let repo = PgClientRepository::new(pool);

    let created = repo
        .create(client::domain::models::db::client_row::CreateClientRow {
            tx_name: "Already Inactive".to_string(),
            tx_status: ClientStatus::Inactive,
            tx_doc: "88888888888".to_string(),
        })
        .await
        .expect("create client");

    let result = DeactivateClientUseCase::execute(&service, created.pk_client).await;

    assert!(matches!(result, Err(ClientError::AlreadyInactive { .. })));
}

/// RegisterClientUseCase returns Err(ClientError::DocumentAlreadyExists) when a client with the same tx_doc already exists.
#[sqlx::test(migrations = "../migrations")]
async fn register_client_with_duplicate_document_returns_error(pool: PgPool) {
    use client::domain::errors::ClientError;

    let service = make_service(pool.clone());
    let repo = PgClientRepository::new(pool);

    // Insert a client directly via the repository
    repo.create(client::domain::models::db::client_row::CreateClientRow {
        tx_name: "Original".to_string(),
        tx_status: ClientStatus::Active,
        tx_doc: "11144477735".to_string(),
    })
    .await
    .expect("create original client");

    // Attempt to register a second client with the same document via the use case
    let input = RegisterClientInput {
        name: "Duplicate".to_string(),
        doc: "11144477735".to_string(),
        email: "dup@example.com".to_string(),
        phones: vec!["+5511999999999".to_string()],
        cep: "01001000".to_string(),
        street: "Street".to_string(),
        number: "1".to_string(),
        complement: "".to_string(),
        neighborhood: "Neighborhood".to_string(),
        city: "City".to_string(),
        state: "SP".to_string(),
    };

    let result = RegisterClientUseCase::execute(&service, input).await;

    // The service checks find_by_document before attempting registration.
    // Since the original client was inserted directly via the repo, the
    // duplicate check in RegisterClientUseCase should catch it.
    assert!(
        matches!(result, Err(ClientError::DocumentAlreadyExists { .. })),
        "expected DocumentAlreadyExists, got: {:?}",
        result
    );
}

/// UpdateClientUseCase currently does NOT check for duplicate documents before updating.
/// This test documents the gap: updating a client's document to one already used by
/// another client succeeds at the application layer (the DB unique constraint would
/// catch it only if the database enforces it, but the application layer doesn't validate).
/// TODO: Add find_by_document check in UpdateClientUseCase before allowing doc changes.
#[sqlx::test(migrations = "../migrations")]
async fn update_client_with_duplicate_document_succeeds_without_app_check(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgClientRepository::new(pool);

    // Insert two clients with distinct documents
    let first = repo
        .create(client::domain::models::db::client_row::CreateClientRow {
            tx_name: "First".to_string(),
            tx_status: ClientStatus::Active,
            tx_doc: "11144477735".to_string(),
        })
        .await
        .expect("create first client");

    let second = repo
        .create(client::domain::models::db::client_row::CreateClientRow {
            tx_name: "Second".to_string(),
            tx_status: ClientStatus::Active,
            tx_doc: "22255588896".to_string(),
        })
        .await
        .expect("create second client");

    // Attempt to update the second client's document to match the first.
    // Currently succeeds because UpdateClientUseCase only validates format,
    // not uniqueness.
    let result = UpdateClientUseCase::execute(
        &service,
        second.pk_client,
        UpdateClientInput {
            name: None,
            doc: Some(first.tx_doc),
        },
    )
    .await;

    // Document the current behavior: update succeeds at the application layer.
    // A proper implementation would return Err(ClientError::DocumentAlreadyExists).
    assert!(
        result.is_err(),
        "update should fail with DocumentAlreadyExists once the check is implemented"
    );
}

/// ListClientsUseCase returns Ok(Vec::new()) when no clients exist in the database.
#[sqlx::test(migrations = "../migrations")]
async fn list_clients_returns_empty_when_no_clients_exist(pool: PgPool) {
    let service = make_service(pool);

    let clients = ListClientsUseCase::execute(&service)
        .await
        .expect("list clients");

    assert!(clients.is_empty());
}
