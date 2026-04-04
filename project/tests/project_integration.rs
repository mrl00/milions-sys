// Integration tests for project bounded context.
// Uses #[sqlx::test] to create isolated databases with migrations applied.

use project::adapters::driven::postgres::PgProjectRepository;
use project::application::project_service::ConcreteProjectService;
use project::domain::errors::ProjectError;
use project::domain::models::db::project_rows::{CreateProjectRow, ProjectStatus};
use project::domain::ports::project_repository::CreateProject;
use project::domain::ports::project_use_cases::{
    CancelProjectUseCase, CompleteProjectUseCase, CreateAllocationInput, CreateAllocationUseCase,
    CreateProjectInput, CreateProjectUseCase, CreateStageInput, CreateStageUseCase,
    DeleteProjectUseCase, FindProjectUseCase, GetCostReportUseCase, GetHistoryReportUseCase,
    GetProgressReportUseCase, ListAllocationsUseCase, ListProjectsUseCase, PauseProjectUseCase,
    StartProjectUseCase, UpdateAllocationInput, UpdateAllocationUseCase, UpdateProjectInput,
    UpdateProjectUseCase, UpdateStageInput, UpdateStageUseCase,
};
use sqlx::PgPool;
use sqlx::types::BigDecimal;
use std::str::FromStr;

fn make_service(pool: PgPool) -> ConcreteProjectService {
    ConcreteProjectService::new(PgProjectRepository::new(pool.clone()))
}

/// Creates a test client and location, returning their IDs for use in project creation.
async fn create_test_fixtures(pool: &PgPool) -> (uuid::Uuid, uuid::Uuid) {
    let client_id = uuid::Uuid::now_v7();
    let location_id = uuid::Uuid::now_v7();

    sqlx::query(
        "INSERT INTO clients.tb_client (pk_client, idx_client, tx_name, tx_status, tx_doc)
         VALUES ($1, 1, 'Test Client', 'active', '11144477735')",
    )
    .bind(client_id)
    .execute(pool)
    .await
    .expect("create test client");

    sqlx::query(
        r#"INSERT INTO locations.tb_location (
            pk_location, idx_location, tx_street, tx_number, tx_city, tx_state, tx_zipcode,
            tx_public_space, tx_address_complement, tx_unit, tx_neighborhood, tx_locality,
            tx_region, tx_ibge, tx_gia, tx_ddd, tx_siafi
        ) VALUES ($1, 1, 'Test St', '1', 'Test City', 'SP', '00000000', 'Rua', '', '', 'Test', 'Test City', 'SP', null, null, '11', null)"#,
    )
    .bind(location_id)
    .execute(pool)
    .await
    .expect("create test location");

    (client_id, location_id)
}

/// Creates a test collaborator, returning its ID for use in allocation creation.
async fn create_test_collaborator(pool: &PgPool) -> uuid::Uuid {
    let collaborator_id = uuid::Uuid::now_v7();

    sqlx::query(
        "INSERT INTO collaborators.tb_collaborator (pk_collaborator, idx_collaborator, tx_name, tx_cpf, tx_level, tx_status)
         VALUES ($1, 1, 'Test Collaborator', '22255588896', 'P0', 'active')",
    )
    .bind(collaborator_id)
    .execute(pool)
    .await
    .expect("create test collaborator");

    collaborator_id
}

fn make_project_row(name: &str, client_id: uuid::Uuid, address_id: uuid::Uuid) -> CreateProjectRow {
    CreateProjectRow {
        tx_name: name.to_string(),
        tx_description: None,
        tx_status: ProjectStatus::Planning,
        dt_start_date: None,
        dt_estimated_end_date: None,
        nr_total_area_m2: None,
        nr_estimated_cost: None,
        tx_notes: None,
        fk_client: client_id,
        fk_address: address_id,
    }
}

// --- Project CRUD tests ---

#[sqlx::test(migrations = "../migrations")]
async fn create_and_find_project(pool: PgPool) {
    let service = make_service(pool.clone());
    let (client_id, address_id) = create_test_fixtures(&pool).await;

    let input = CreateProjectInput {
        name: "Test Project".to_string(),
        description: Some("A test project".to_string()),
        start_date: None,
        estimated_end_date: None,
        total_area_m2: None,
        estimated_cost: None,
        notes: None,
        client_id,
        address_id,
    };

    let created = CreateProjectUseCase::execute(&service, input)
        .await
        .expect("create project");

    let found = FindProjectUseCase::execute(&service, created.pk_project)
        .await
        .expect("find project");

    assert_eq!(found.pk_project, created.pk_project);
    assert_eq!(found.tx_name, "Test Project");
    assert_eq!(found.tx_description, Some("A test project".to_string()));
    assert_eq!(found.tx_status, "planning");
}

#[sqlx::test(migrations = "../migrations")]
async fn create_project_removes_accents(pool: PgPool) {
    let service = make_service(pool.clone());
    let (client_id, address_id) = create_test_fixtures(&pool).await;

    let input = CreateProjectInput {
        name: "Projeto São João".to_string(),
        description: Some("Descrição com acentos: café, ação".to_string()),
        start_date: None,
        estimated_end_date: None,
        total_area_m2: None,
        estimated_cost: None,
        notes: None,
        client_id,
        address_id,
    };

    let created = CreateProjectUseCase::execute(&service, input)
        .await
        .expect("create project");

    assert_eq!(created.tx_name, "Projeto Sao Joao");
    assert_eq!(
        created.tx_description,
        Some("Descricao com acentos: cafe, acao".to_string())
    );
}

#[sqlx::test(migrations = "../migrations")]
async fn update_project_changes_name(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgProjectRepository::new(pool.clone());
    let (client_id, address_id) = create_test_fixtures(&pool).await;

    let created = repo
        .create(make_project_row("Old Name", client_id, address_id))
        .await
        .expect("create project");

    let updated = UpdateProjectUseCase::execute(
        &service,
        created.pk_project,
        UpdateProjectInput {
            active: None,
            actual_cost: None,
            actual_end_date: None,
            name: Some("New Name".to_string()),
            description: None,
            start_date: None,
            estimated_end_date: None,
            total_area_m2: None,
            estimated_cost: None,
            notes: None,
        },
    )
    .await
    .expect("update project");

    assert_eq!(updated.tx_name, "New Name");
}

#[sqlx::test(migrations = "../migrations")]
async fn delete_project_removes_row(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgProjectRepository::new(pool.clone());
    let (client_id, address_id) = create_test_fixtures(&pool).await;

    let created = repo
        .create(make_project_row("To Delete", client_id, address_id))
        .await
        .expect("create project");

    DeleteProjectUseCase::execute(&service, created.pk_project)
        .await
        .expect("delete project");

    let found = FindProjectUseCase::execute(&service, created.pk_project).await;
    assert!(found.is_err());
}

#[sqlx::test(migrations = "../migrations")]
async fn list_projects_returns_all(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgProjectRepository::new(pool.clone());
    let (client_id, address_id) = create_test_fixtures(&pool).await;

    repo.create(make_project_row("Project A", client_id, address_id))
        .await
        .expect("create project A");

    repo.create(make_project_row("Project B", client_id, address_id))
        .await
        .expect("create project B");

    let projects = ListProjectsUseCase::execute(&service)
        .await
        .expect("list projects");

    assert_eq!(projects.len(), 2);
}

#[sqlx::test(migrations = "../migrations")]
async fn list_projects_returns_empty_when_none_exist(pool: PgPool) {
    let service = make_service(pool);

    let projects = ListProjectsUseCase::execute(&service)
        .await
        .expect("list projects");

    assert!(projects.is_empty());
}

#[sqlx::test(migrations = "../migrations")]
async fn find_project_returns_not_found_for_missing(pool: PgPool) {
    let service = make_service(pool);

    let uuid = uuid::Uuid::now_v7();
    let result: Result<project::domain::models::db::project_rows::ProjectRow, ProjectError> =
        FindProjectUseCase::execute(&service, uuid).await;

    assert!(matches!(result, Err(ProjectError::NotFound { .. })));
}

#[sqlx::test(migrations = "../migrations")]
async fn update_nonexistent_project_returns_error(pool: PgPool) {
    let service = make_service(pool);

    let uuid = uuid::Uuid::now_v7();
    let result = UpdateProjectUseCase::execute(
        &service,
        uuid,
        UpdateProjectInput {
            active: None,
            actual_cost: None,
            actual_end_date: None,
            name: Some("Ghost".to_string()),
            description: None,
            start_date: None,
            estimated_end_date: None,
            total_area_m2: None,
            estimated_cost: None,
            notes: None,
        },
    )
    .await;

    assert!(matches!(result, Err(ProjectError::NotFound { .. })));
}

#[sqlx::test(migrations = "../migrations")]
async fn delete_nonexistent_project_returns_error(pool: PgPool) {
    let service = make_service(pool);

    let uuid = uuid::Uuid::now_v7();
    let result = DeleteProjectUseCase::execute(&service, uuid).await;

    assert!(matches!(result, Err(ProjectError::NotFound { .. })));
}

// --- Project status tests ---

#[sqlx::test(migrations = "../migrations")]
async fn start_project(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgProjectRepository::new(pool.clone());
    let (client_id, address_id) = create_test_fixtures(&pool).await;

    let created = repo
        .create(make_project_row("Start Me", client_id, address_id))
        .await
        .expect("create project");

    let started = StartProjectUseCase::execute(&service, created.pk_project)
        .await
        .expect("start project");

    assert_eq!(started.tx_status, "in_progress");
}

#[sqlx::test(migrations = "../migrations")]
async fn pause_project(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgProjectRepository::new(pool.clone());
    let (client_id, address_id) = create_test_fixtures(&pool).await;

    let created = repo
        .create(make_project_row("Pause Me", client_id, address_id))
        .await
        .expect("create project");

    StartProjectUseCase::execute(&service, created.pk_project)
        .await
        .expect("start project");

    let paused = PauseProjectUseCase::execute(&service, created.pk_project)
        .await
        .expect("pause project");

    assert_eq!(paused.tx_status, "paused");
}

#[sqlx::test(migrations = "../migrations")]
async fn complete_project(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgProjectRepository::new(pool.clone());
    let (client_id, address_id) = create_test_fixtures(&pool).await;

    let created = repo
        .create(make_project_row("Complete Me", client_id, address_id))
        .await
        .expect("create project");

    StartProjectUseCase::execute(&service, created.pk_project)
        .await
        .expect("start project");

    let completed = CompleteProjectUseCase::execute(&service, created.pk_project)
        .await
        .expect("complete project");

    assert_eq!(completed.tx_status, "completed");
}

#[sqlx::test(migrations = "../migrations")]
async fn cancel_project(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgProjectRepository::new(pool.clone());
    let (client_id, address_id) = create_test_fixtures(&pool).await;

    let created = repo
        .create(make_project_row("Cancel Me", client_id, address_id))
        .await
        .expect("create project");

    let cancelled = CancelProjectUseCase::execute(&service, created.pk_project)
        .await
        .expect("cancel project");

    assert_eq!(cancelled.tx_status, "cancelled");
}

#[sqlx::test(migrations = "../migrations")]
async fn start_already_started_project_returns_error(pool: PgPool) {
    use project::domain::models::db::project_rows::ProjectStatus;

    let service = make_service(pool.clone());
    let repo = PgProjectRepository::new(pool.clone());
    let (client_id, address_id) = create_test_fixtures(&pool).await;

    let created = repo
        .create(CreateProjectRow {
            tx_name: "Already Started".to_string(),
            tx_description: None,
            tx_status: ProjectStatus::InProgress,
            dt_start_date: None,
            dt_estimated_end_date: None,
            nr_total_area_m2: None,
            nr_estimated_cost: None,
            tx_notes: None,
            fk_client: client_id,
            fk_address: address_id,
        })
        .await
        .expect("create project");

    let result = StartProjectUseCase::execute(&service, created.pk_project).await;

    assert!(matches!(result, Err(ProjectError::AlreadyInStatus { .. })));
}

// --- Stage tests ---

#[sqlx::test(migrations = "../migrations")]
async fn create_stage(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgProjectRepository::new(pool.clone());
    let (client_id, address_id) = create_test_fixtures(&pool).await;

    let project = repo
        .create(make_project_row("With Stage", client_id, address_id))
        .await
        .expect("create project");

    let input = CreateStageInput {
        description: None,
        name: "Foundation".to_string(),
        order: 1,
        start_date: None,
        end_date: None,
    };

    let stage = CreateStageUseCase::execute(&service, project.pk_project, input)
        .await
        .expect("create stage");

    assert_eq!(stage.tx_name, "Foundation");
    assert_eq!(stage.nr_order, 1);
    assert_eq!(stage.tx_status, "pending");
}

#[sqlx::test(migrations = "../migrations")]
async fn update_stage(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgProjectRepository::new(pool.clone());
    let (client_id, address_id) = create_test_fixtures(&pool).await;

    let project = repo
        .create(make_project_row("Update Stage", client_id, address_id))
        .await
        .expect("create project");

    let stage_input = CreateStageInput {
        description: None,
        name: "Old Stage".to_string(),
        order: 1,
        start_date: None,
        end_date: None,
    };

    let stage = CreateStageUseCase::execute(&service, project.pk_project, stage_input)
        .await
        .expect("create stage");

    let updated = UpdateStageUseCase::execute(
        &service,
        project.pk_project,
        stage.pk_project_stage,
        UpdateStageInput {
            description: None,
            status: None,
            name: Some("New Stage".to_string()),
            order: Some(2),
            start_date: None,
            end_date: None,
        },
    )
    .await
    .expect("update stage");

    assert_eq!(updated.tx_name, "New Stage");
    assert_eq!(updated.nr_order, 2);
}

// --- Allocation tests ---

#[sqlx::test(migrations = "../migrations")]
async fn create_allocation(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgProjectRepository::new(pool.clone());
    let (client_id, address_id) = create_test_fixtures(&pool).await;
    let collaborator_id = create_test_collaborator(&pool).await;

    let project = repo
        .create(make_project_row("With Allocation", client_id, address_id))
        .await
        .expect("create project");

    let input = CreateAllocationInput {
        notes: None,
        collaborator_id,
        work_date: chrono::NaiveDate::from_ymd_opt(2026, 1, 15).unwrap(),
        hours_worked: Some(BigDecimal::from_str("8.0").unwrap()),
        hourly_rate_snapshot: Some(BigDecimal::from_str("50.00").unwrap()),
        present: true,
    };

    let allocation = CreateAllocationUseCase::execute(&service, project.pk_project, input)
        .await
        .expect("create allocation");

    assert_eq!(allocation.fk_project, project.pk_project);
    assert!(allocation.bl_present);
}

#[sqlx::test(migrations = "../migrations")]
async fn list_allocations_for_project(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgProjectRepository::new(pool.clone());
    let (client_id, address_id) = create_test_fixtures(&pool).await;
    let collaborator_id = create_test_collaborator(&pool).await;

    let project = repo
        .create(make_project_row("List Allocations", client_id, address_id))
        .await
        .expect("create project");

    let input1 = CreateAllocationInput {
        notes: None,
        collaborator_id,
        work_date: chrono::NaiveDate::from_ymd_opt(2026, 1, 15).unwrap(),
        hours_worked: Some(BigDecimal::from_str("8.0").unwrap()),
        hourly_rate_snapshot: Some(BigDecimal::from_str("50.00").unwrap()),
        present: true,
    };

    let input2 = CreateAllocationInput {
        notes: None,
        collaborator_id,
        work_date: chrono::NaiveDate::from_ymd_opt(2026, 1, 16).unwrap(),
        hours_worked: Some(BigDecimal::from_str("6.0").unwrap()),
        hourly_rate_snapshot: Some(BigDecimal::from_str("50.00").unwrap()),
        present: true,
    };

    CreateAllocationUseCase::execute(&service, project.pk_project, input1)
        .await
        .expect("create allocation 1");

    CreateAllocationUseCase::execute(&service, project.pk_project, input2)
        .await
        .expect("create allocation 2");

    let allocations = ListAllocationsUseCase::execute(&service, project.pk_project)
        .await
        .expect("list allocations");

    assert_eq!(allocations.len(), 2);
}

#[sqlx::test(migrations = "../migrations")]
async fn update_allocation(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgProjectRepository::new(pool.clone());
    let (client_id, address_id) = create_test_fixtures(&pool).await;
    let collaborator_id = create_test_collaborator(&pool).await;

    let project = repo
        .create(make_project_row("Update Allocation", client_id, address_id))
        .await
        .expect("create project");

    let input = CreateAllocationInput {
        notes: None,
        collaborator_id,
        work_date: chrono::NaiveDate::from_ymd_opt(2026, 1, 15).unwrap(),
        hours_worked: Some(BigDecimal::from_str("8.0").unwrap()),
        hourly_rate_snapshot: Some(BigDecimal::from_str("50.00").unwrap()),
        present: true,
    };

    let allocation = CreateAllocationUseCase::execute(&service, project.pk_project, input)
        .await
        .expect("create allocation");

    let updated = UpdateAllocationUseCase::execute(
        &service,
        project.pk_project,
        allocation.pk_project_daily_allocation,
        UpdateAllocationInput {
            notes: None,
            hours_worked: Some(BigDecimal::from_str("4.0").unwrap()),
            hourly_rate_snapshot: Some(BigDecimal::from_str("60.00").unwrap()),
            present: Some(false),
        },
    )
    .await
    .expect("update allocation");

    assert_eq!(
        updated.nr_hours_worked,
        Some(BigDecimal::from_str("4.0").unwrap())
    );
    assert_eq!(
        updated.nr_hourly_rate_snapshot,
        Some(BigDecimal::from_str("60.00").unwrap())
    );
    assert!(!updated.bl_present);
}

// --- Report tests ---

#[sqlx::test(migrations = "../migrations")]
async fn get_cost_report_for_empty_project(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgProjectRepository::new(pool.clone());
    let (client_id, address_id) = create_test_fixtures(&pool).await;

    let project = repo
        .create(make_project_row("Cost Report", client_id, address_id))
        .await
        .expect("create project");

    let report = GetCostReportUseCase::execute(&service, project.pk_project)
        .await
        .expect("get cost report");

    assert_eq!(report.actual_cost, BigDecimal::from(0));
}

#[sqlx::test(migrations = "../migrations")]
async fn get_progress_report_for_empty_project(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgProjectRepository::new(pool.clone());
    let (client_id, address_id) = create_test_fixtures(&pool).await;

    let project = repo
        .create(make_project_row("Progress Report", client_id, address_id))
        .await
        .expect("create project");

    let report = GetProgressReportUseCase::execute(&service, project.pk_project)
        .await
        .expect("get progress report");

    assert_eq!(report.total_stages, 0);
    assert_eq!(report.completed_stages, 0);
}

#[sqlx::test(migrations = "../migrations")]
async fn get_history_report_for_collaborator(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgProjectRepository::new(pool.clone());
    let (client_id, address_id) = create_test_fixtures(&pool).await;
    let collaborator_id = create_test_collaborator(&pool).await;

    let project = repo
        .create(make_project_row("History Report", client_id, address_id))
        .await
        .expect("create project");

    let input = CreateAllocationInput {
        notes: None,
        collaborator_id,
        work_date: chrono::NaiveDate::from_ymd_opt(2026, 1, 15).unwrap(),
        hours_worked: Some(BigDecimal::from_str("8.0").unwrap()),
        hourly_rate_snapshot: Some(BigDecimal::from_str("50.00").unwrap()),
        present: true,
    };

    CreateAllocationUseCase::execute(&service, project.pk_project, input)
        .await
        .expect("create allocation");

    let report = GetHistoryReportUseCase::execute(&service, collaborator_id)
        .await
        .expect("get history report");

    assert_eq!(report.collaborator_id, collaborator_id);
    assert_eq!(report.total_days, 1);
}
