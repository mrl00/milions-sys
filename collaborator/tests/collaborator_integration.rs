// Integration tests for collaborator bounded context.
// Uses #[sqlx::test] to create isolated databases with migrations applied.

use collaborator::adapters::driven::postgres::pg_collaborator_repository::PgCollaboratorRepository;
use collaborator::application::collaborator_service::ConcreteCollaboratorService;
use collaborator::domain::errors::collaborator_error::CollaboratorError;
use collaborator::domain::models::db::collaborator_row::{
    CollaboratorLevel, CollaboratorStatus, CreateCollaboratorRow,
};
use collaborator::domain::ports::collaborator_repository::CreateCollaborator;
use collaborator::domain::ports::collaborator_use_cases::{
    ActivateCollaboratorUseCase, DeactivateCollaboratorUseCase, DeleteCollaboratorUseCase,
    FindCollaboratorByDocumentUseCase, FindCollaboratorUseCase, ListCollaboratorsUseCase,
    RegisterCollaboratorInput, RegisterCollaboratorUseCase, UpdateCollaboratorInput,
    UpdateCollaboratorUseCase,
};
use sqlx::PgPool;

fn make_service(pool: PgPool) -> ConcreteCollaboratorService {
    ConcreteCollaboratorService::new(PgCollaboratorRepository::new(pool))
}

// --- Collaborator CRUD tests ---

#[sqlx::test(migrations = "../migrations")]
async fn create_and_find_collaborator(pool: PgPool) {
    let service = make_service(pool);

    let input = RegisterCollaboratorInput {
        name: "João Silva".to_string(),
        cpf: "11144477735".to_string(),
    };

    let created = RegisterCollaboratorUseCase::execute(&service, input)
        .await
        .expect("create collaborator");

    let found = FindCollaboratorUseCase::execute(&service, created.pk_collaborator)
        .await
        .expect("find collaborator");

    assert_eq!(found.pk_collaborator, created.pk_collaborator);
    assert_eq!(found.tx_name, "Joao Silva");
    assert_eq!(found.tx_cpf, "11144477735");
    assert_eq!(found.tx_status, "active");
}

#[sqlx::test(migrations = "../migrations")]
async fn create_collaborator_removes_accents(pool: PgPool) {
    let service = make_service(pool);

    let input = RegisterCollaboratorInput {
        name: "São João da Silva".to_string(),
        cpf: "33366699957".to_string(),
    };

    let created = RegisterCollaboratorUseCase::execute(&service, input)
        .await
        .expect("create collaborator");

    assert_eq!(created.tx_name, "Sao Joao da Silva");
}

#[sqlx::test(migrations = "../migrations")]
async fn register_collaborator_with_duplicate_cpf_returns_error(pool: PgPool) {
    let service = make_service(pool);

    let _input = RegisterCollaboratorInput {
        name: "First".to_string(),
        cpf: "11144477735".to_string(),
    };

    let first = RegisterCollaboratorUseCase::execute(
        &service,
        RegisterCollaboratorInput {
            name: "First".to_string(),
            cpf: "11144477735".to_string(),
        },
    )
    .await
    .expect("create first collaborator");

    let result = RegisterCollaboratorUseCase::execute(
        &service,
        RegisterCollaboratorInput {
            name: "First".to_string(),
            cpf: "11144477735".to_string(),
        },
    )
    .await;

    assert!(matches!(
        result,
        Err(CollaboratorError::CpfAlreadyExists { .. })
    ));
    let _ = first;
}

#[sqlx::test(migrations = "../migrations")]
async fn register_collaborator_with_invalid_cpf_returns_error(pool: PgPool) {
    use types::cpf::CpfError;

    let service = make_service(pool);

    let input = RegisterCollaboratorInput {
        name: "Invalid CPF".to_string(),
        cpf: "00000000000".to_string(),
    };

    let result = RegisterCollaboratorUseCase::execute(&service, input).await;

    assert!(matches!(
        result,
        Err(CollaboratorError::InvalidCpf(CpfError::InvalidCpf { .. }))
    ));
}

#[sqlx::test(migrations = "../migrations")]
async fn update_collaborator_changes_name(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgCollaboratorRepository::new(pool);

    let created = repo
        .create(CreateCollaboratorRow {
            tx_name: "Old Name".to_string(),
            tx_cpf: "11144477735".to_string(),
            tx_level: CollaboratorLevel::P0,
            tx_status: CollaboratorStatus::Active,
        })
        .await
        .expect("create collaborator");

    let updated = UpdateCollaboratorUseCase::execute(
        &service,
        created.pk_collaborator,
        UpdateCollaboratorInput {
            name: Some("New Name".to_string()),
            cpf: None,
            level: None,
        },
    )
    .await
    .expect("update collaborator");

    assert_eq!(updated.tx_name, "New Name");
    assert_eq!(updated.tx_cpf, "11144477735");
}

#[sqlx::test(migrations = "../migrations")]
async fn update_nonexistent_collaborator_returns_error(pool: PgPool) {
    let service = make_service(pool);

    let uuid = uuid::Uuid::now_v7();
    let result = UpdateCollaboratorUseCase::execute(
        &service,
        uuid,
        UpdateCollaboratorInput {
            name: Some("Ghost".to_string()),
            cpf: None,
            level: None,
        },
    )
    .await;

    assert!(matches!(result, Err(CollaboratorError::NotFound { .. })));
}

#[sqlx::test(migrations = "../migrations")]
async fn delete_collaborator_removes_row(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgCollaboratorRepository::new(pool);

    let created = repo
        .create(CreateCollaboratorRow {
            tx_name: "To Delete".to_string(),
            tx_cpf: "11144477735".to_string(),
            tx_level: CollaboratorLevel::P0,
            tx_status: CollaboratorStatus::Active,
        })
        .await
        .expect("create collaborator");

    DeleteCollaboratorUseCase::execute(&service, created.pk_collaborator)
        .await
        .expect("delete collaborator");

    let found = FindCollaboratorUseCase::execute(&service, created.pk_collaborator).await;
    assert!(found.is_err());
}

#[sqlx::test(migrations = "../migrations")]
async fn delete_nonexistent_collaborator_returns_error(pool: PgPool) {
    let service = make_service(pool);

    let uuid = uuid::Uuid::now_v7();
    let result = DeleteCollaboratorUseCase::execute(&service, uuid).await;

    assert!(matches!(result, Err(CollaboratorError::NotFound { .. })));
}

#[sqlx::test(migrations = "../migrations")]
async fn list_collaborators_returns_all(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgCollaboratorRepository::new(pool);

    repo.create(CreateCollaboratorRow {
        tx_name: "Collaborator A".to_string(),
        tx_cpf: "11144477735".to_string(),
        tx_level: CollaboratorLevel::P0,
        tx_status: CollaboratorStatus::Active,
    })
    .await
    .expect("create collaborator A");

    repo.create(CreateCollaboratorRow {
        tx_name: "Collaborator B".to_string(),
        tx_cpf: "22255588896".to_string(),
        tx_level: CollaboratorLevel::P0,
        tx_status: CollaboratorStatus::Active,
    })
    .await
    .expect("create collaborator B");

    let collaborators = ListCollaboratorsUseCase::execute(&service)
        .await
        .expect("list collaborators");

    assert_eq!(collaborators.len(), 2);
}

#[sqlx::test(migrations = "../migrations")]
async fn list_collaborators_returns_empty_when_none_exist(pool: PgPool) {
    let service = make_service(pool);

    let collaborators = ListCollaboratorsUseCase::execute(&service)
        .await
        .expect("list collaborators");

    assert!(collaborators.is_empty());
}

#[sqlx::test(migrations = "../migrations")]
async fn find_collaborator_returns_not_found_for_missing(pool: PgPool) {
    let service = make_service(pool);

    let uuid = uuid::Uuid::now_v7();
    let result: Result<_, _> = FindCollaboratorUseCase::execute(&service, uuid).await;

    assert!(matches!(result, Err(CollaboratorError::NotFound { .. })));
}

#[sqlx::test(migrations = "../migrations")]
async fn find_collaborator_by_document_returns_none_for_missing(pool: PgPool) {
    let service = make_service(pool);

    let result = FindCollaboratorByDocumentUseCase::execute(&service, "11144477735")
        .await
        .expect("find by document");

    assert!(result.is_none());
}

#[sqlx::test(migrations = "../migrations")]
async fn find_collaborator_by_document_returns_collaborator(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgCollaboratorRepository::new(pool);

    repo.create(CreateCollaboratorRow {
        tx_name: "Doc Search".to_string(),
        tx_cpf: "11144477735".to_string(),
        tx_level: CollaboratorLevel::P0,
        tx_status: CollaboratorStatus::Active,
    })
    .await
    .expect("create collaborator");

    let result = FindCollaboratorByDocumentUseCase::execute(&service, "11144477735")
        .await
        .expect("find by document");

    assert!(result.is_some());
    assert_eq!(result.unwrap().tx_name, "Doc Search");
}

// --- Status tests ---

#[sqlx::test(migrations = "../migrations")]
async fn activate_collaborator(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgCollaboratorRepository::new(pool);

    let created = repo
        .create(CreateCollaboratorRow {
            tx_name: "Inactive".to_string(),
            tx_cpf: "11144477735".to_string(),
            tx_level: CollaboratorLevel::P0,
            tx_status: CollaboratorStatus::Inactive,
        })
        .await
        .expect("create collaborator");

    let active = ActivateCollaboratorUseCase::execute(&service, created.pk_collaborator)
        .await
        .expect("activate collaborator");

    assert_eq!(active.tx_status, "active");
}

#[sqlx::test(migrations = "../migrations")]
async fn deactivate_collaborator(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgCollaboratorRepository::new(pool);

    let created = repo
        .create(CreateCollaboratorRow {
            tx_name: "Active".to_string(),
            tx_cpf: "11144477735".to_string(),
            tx_level: CollaboratorLevel::P0,
            tx_status: CollaboratorStatus::Active,
        })
        .await
        .expect("create collaborator");

    let inactive = DeactivateCollaboratorUseCase::execute(&service, created.pk_collaborator)
        .await
        .expect("deactivate collaborator");

    assert_eq!(inactive.tx_status, "inactive");
}

#[sqlx::test(migrations = "../migrations")]
async fn activate_already_active_collaborator_returns_error(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgCollaboratorRepository::new(pool);

    let created = repo
        .create(CreateCollaboratorRow {
            tx_name: "Already Active".to_string(),
            tx_cpf: "11144477735".to_string(),
            tx_level: CollaboratorLevel::P0,
            tx_status: CollaboratorStatus::Active,
        })
        .await
        .expect("create collaborator");

    let result = ActivateCollaboratorUseCase::execute(&service, created.pk_collaborator).await;

    assert!(matches!(
        result,
        Err(CollaboratorError::AlreadyActive { .. })
    ));
}

#[sqlx::test(migrations = "../migrations")]
async fn deactivate_already_inactive_collaborator_returns_error(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgCollaboratorRepository::new(pool);

    let created = repo
        .create(CreateCollaboratorRow {
            tx_name: "Already Inactive".to_string(),
            tx_cpf: "11144477735".to_string(),
            tx_level: CollaboratorLevel::P0,
            tx_status: CollaboratorStatus::Inactive,
        })
        .await
        .expect("create collaborator");

    let result = DeactivateCollaboratorUseCase::execute(&service, created.pk_collaborator).await;

    assert!(matches!(
        result,
        Err(CollaboratorError::AlreadyInactive { .. })
    ));
}
