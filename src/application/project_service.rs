use async_trait::async_trait;
use sqlx::types::BigDecimal;
use uuid::Uuid;

use crate::adapters::driven::pg_project_repository::PgProjectRepository;
use crate::domain::errors::project_error::ProjectError;
use crate::domain::models::db;
use crate::domain::models::db::project_rows::{
    CreateProjectDailyAllocationRow, CreateProjectRow, CreateProjectStageRow,
    ProjectDailyAllocationRow, ProjectRow, ProjectStageRow, ProjectStageStatus, ProjectStatus,
    UpdateProjectRow,
};
use crate::domain::ports::repositories::project_repository::ProjectRepository;
use crate::domain::ports::use_cases::project_use_cases::{
    CancelProjectUseCase, CompleteProjectUseCase, CreateAllocationInput, CreateAllocationUseCase,
    CreateProjectInput, CreateProjectUseCase, CreateStageInput, CreateStageUseCase,
    DeleteProjectUseCase, FindProjectUseCase, GetCostReportUseCase, GetHistoryReportUseCase,
    GetProgressReportUseCase, ListAllocationsUseCase, ListProjectsUseCase, PauseProjectUseCase,
    StartProjectUseCase, UpdateAllocationInput, UpdateAllocationUseCase, UpdateProjectInput,
    UpdateProjectUseCase, UpdateStageInput, UpdateStageUseCase,
};

pub struct ProjectService<R> {
    repo: R,
}

impl<R: ProjectRepository> ProjectService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

pub type PgProjectService = ProjectService<PgProjectRepository>;

/// Parseia `Option<String>` para `Option<BigDecimal>`.
/// Retorna `ProjectError::InvalidField` se a string estiver presente
/// mas não for um número decimal válido.
fn parse_decimal(
    val: Option<String>,
    field: &'static str,
) -> Result<Option<BigDecimal>, ProjectError> {
    match val {
        None => Ok(None),
        Some(s) => s
            .parse::<BigDecimal>()
            .map(Some)
            .map_err(|_| ProjectError::InvalidField {
                field,
                reason: format!("'{}' is not a valid decimal number", s),
            }),
    }
}

#[async_trait]
impl<R: ProjectRepository> FindProjectUseCase for ProjectService<R> {
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ProjectError::NotFound { uuid })
    }
}

#[async_trait]
impl<R: ProjectRepository> ListProjectsUseCase for ProjectService<R> {
    async fn execute(&self) -> Result<Vec<ProjectRow>, ProjectError> {
        self.repo.find_all().await
    }
}

#[async_trait]
impl<R: ProjectRepository> CreateProjectUseCase for ProjectService<R> {
    async fn execute(&self, input: CreateProjectInput) -> Result<ProjectRow, ProjectError> {
        let row = CreateProjectRow {
            tx_name: input.name,
            tx_description: input.description,
            tx_status: ProjectStatus::Planning,
            dt_start_date: input.start_date,
            dt_estimated_end_date: input.estimated_end_date,
            nr_total_area_m2: parse_decimal(input.total_area_m2, "total_area_m2")?,
            nr_estimated_cost: parse_decimal(input.estimated_cost, "estimated_cost")?,
            tx_notes: input.notes,
            fk_address: input.address_id,
        };

        self.repo.create(row).await
    }
}

#[async_trait]
impl<R: ProjectRepository> UpdateProjectUseCase for ProjectService<R> {
    async fn execute(
        &self,
        uuid: Uuid,
        input: UpdateProjectInput,
    ) -> Result<ProjectRow, ProjectError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ProjectError::NotFound { uuid })?;

        self.repo
            .update(
                uuid,
                UpdateProjectRow {
                    tx_name: input.name,
                    tx_description: input.description,
                    tx_status: None,
                    dt_start_date: input.start_date,
                    dt_estimated_end_date: input.estimated_end_date,
                    dt_actual_end_date: input.actual_end_date,
                    nr_total_area_m2: parse_decimal(input.total_area_m2, "total_area_m2")?,
                    nr_estimated_cost: parse_decimal(input.estimated_cost, "estimated_cost")?,
                    nr_actual_cost: parse_decimal(input.actual_cost, "actual_cost")?,
                    tx_notes: input.notes,
                    bl_active: input.active,
                },
            )
            .await
    }
}

/// State machine de transições de status do projeto.
///
/// Transições válidas:
/// - `planning`    → `in_progress`, `cancelled`
/// - `in_progress` → `paused`, `completed`, `cancelled`
/// - `paused`      → `in_progress`, `cancelled`
/// - `completed`   → (terminal — nenhuma transição)
/// - `cancelled`   → (terminal — nenhuma transição)
fn valid_transition(from: &str, to: &ProjectStatus) -> bool {
    match (from, to) {
        ("planning", ProjectStatus::InProgress)
        | ("planning", ProjectStatus::Cancelled)
        | ("in_progress", ProjectStatus::Paused)
        | ("in_progress", ProjectStatus::Completed)
        | ("in_progress", ProjectStatus::Cancelled)
        | ("paused", ProjectStatus::InProgress)
        | ("paused", ProjectStatus::Cancelled) => true,
        _ => false,
    }
}

/// Valida a transição e retorna o erro adequado se inválida.
/// - Status igual ao alvo → `AlreadyInStatus`
/// - Estado terminal ou transição não permitida → `InvalidTransition`
fn check_transition(
    uuid: Uuid,
    current_status: &str,
    target: &ProjectStatus,
) -> Result<(), ProjectError> {
    if current_status == target.to_string() {
        return Err(ProjectError::AlreadyInStatus {
            uuid,
            status: target.to_string(),
        });
    }
    if !valid_transition(current_status, target) {
        return Err(ProjectError::InvalidTransition {
            from: current_status.to_string(),
            to: target.to_string(),
        });
    }
    Ok(())
}

#[async_trait]
impl<R: ProjectRepository> StartProjectUseCase for ProjectService<R> {
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError> {
        let current = self
            .repo
            .find_by_id(uuid)
            .await?
            .ok_or(ProjectError::NotFound { uuid })?;

        check_transition(uuid, &current.tx_status, &ProjectStatus::InProgress)?;

        self.repo
            .update(
                uuid,
                UpdateProjectRow {
                    tx_name: None,
                    tx_description: None,
                    tx_status: Some(ProjectStatus::InProgress),
                    dt_start_date: None,
                    dt_estimated_end_date: None,
                    dt_actual_end_date: None,
                    nr_total_area_m2: None,
                    nr_estimated_cost: None,
                    nr_actual_cost: None,
                    tx_notes: None,
                    bl_active: None,
                },
            )
            .await
    }
}

#[async_trait]
impl<R: ProjectRepository> PauseProjectUseCase for ProjectService<R> {
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError> {
        let current = self
            .repo
            .find_by_id(uuid)
            .await?
            .ok_or(ProjectError::NotFound { uuid })?;

        check_transition(uuid, &current.tx_status, &ProjectStatus::Paused)?;

        self.repo
            .update(
                uuid,
                UpdateProjectRow {
                    tx_name: None,
                    tx_description: None,
                    tx_status: Some(ProjectStatus::Paused),
                    dt_start_date: None,
                    dt_estimated_end_date: None,
                    dt_actual_end_date: None,
                    nr_total_area_m2: None,
                    nr_estimated_cost: None,
                    nr_actual_cost: None,
                    tx_notes: None,
                    bl_active: None,
                },
            )
            .await
    }
}

#[async_trait]
impl<R: ProjectRepository> CompleteProjectUseCase for ProjectService<R> {
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError> {
        let current = self
            .repo
            .find_by_id(uuid)
            .await?
            .ok_or(ProjectError::NotFound { uuid })?;

        check_transition(uuid, &current.tx_status, &ProjectStatus::Completed)?;

        self.repo
            .update(
                uuid,
                UpdateProjectRow {
                    tx_name: None,
                    tx_description: None,
                    tx_status: Some(ProjectStatus::Completed),
                    dt_start_date: None,
                    dt_estimated_end_date: None,
                    dt_actual_end_date: None,
                    nr_total_area_m2: None,
                    nr_estimated_cost: None,
                    nr_actual_cost: None,
                    tx_notes: None,
                    bl_active: None,
                },
            )
            .await
    }
}

#[async_trait]
impl<R: ProjectRepository> CancelProjectUseCase for ProjectService<R> {
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError> {
        let current = self
            .repo
            .find_by_id(uuid)
            .await?
            .ok_or(ProjectError::NotFound { uuid })?;

        check_transition(uuid, &current.tx_status, &ProjectStatus::Cancelled)?;

        self.repo
            .update(
                uuid,
                UpdateProjectRow {
                    tx_name: None,
                    tx_description: None,
                    tx_status: Some(ProjectStatus::Cancelled),
                    dt_start_date: None,
                    dt_estimated_end_date: None,
                    dt_actual_end_date: None,
                    nr_total_area_m2: None,
                    nr_estimated_cost: None,
                    nr_actual_cost: None,
                    tx_notes: None,
                    bl_active: None,
                },
            )
            .await
    }
}

#[async_trait]
impl<R: ProjectRepository> DeleteProjectUseCase for ProjectService<R> {
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ProjectError::NotFound { uuid })?;

        self.repo.delete(uuid).await
    }
}

#[async_trait]
impl<R: ProjectRepository> CreateStageUseCase for ProjectService<R> {
    async fn execute(
        &self,
        project_id: Uuid,
        input: CreateStageInput,
    ) -> Result<ProjectStageRow, ProjectError> {
        self.repo
            .find_by_id(project_id)
            .await?
            .ok_or(ProjectError::NotFound { uuid: project_id })?;

        let row = CreateProjectStageRow {
            fk_project: project_id,
            tx_name: input.name,
            tx_description: input.description,
            nr_order: input.order,
            tx_status: ProjectStageStatus::Pending,
            dt_start_date: input.start_date,
            dt_end_date: input.end_date,
        };

        self.repo.create_stage(row).await
    }
}

#[async_trait]
impl<R: ProjectRepository> UpdateStageUseCase for ProjectService<R> {
    async fn execute(
        &self,
        project_id: Uuid,
        stage_id: Uuid,
        input: UpdateStageInput,
    ) -> Result<ProjectStageRow, ProjectError> {
        self.repo
            .find_by_id(project_id)
            .await?
            .ok_or(ProjectError::NotFound { uuid: project_id })?;

        let current = self
            .repo
            .find_stage_by_id(stage_id)
            .await?
            .ok_or(ProjectError::StageNotFound { uuid: stage_id })?;

        if current.fk_project != project_id {
            return Err(ProjectError::StageNotFound { uuid: stage_id });
        }

        self.repo
            .update_stage(
                stage_id,
                crate::domain::models::db::project_rows::UpdateProjectStageRow {
                    tx_name: input.name,
                    tx_description: input.description,
                    nr_order: input.order,
                    tx_status: input.status.map(|s| match s.as_str() {
                        "pending" => ProjectStageStatus::Pending,
                        "in_progress" => ProjectStageStatus::InProgress,
                        "completed" => ProjectStageStatus::Completed,
                        "skipped" => ProjectStageStatus::Skipped,
                        _ => ProjectStageStatus::Pending,
                    }),
                    dt_start_date: input.start_date,
                    dt_end_date: input.end_date,
                },
            )
            .await
    }
}

#[async_trait]
impl<R: ProjectRepository> CreateAllocationUseCase for ProjectService<R> {
    async fn execute(
        &self,
        project_id: Uuid,
        input: CreateAllocationInput,
    ) -> Result<ProjectDailyAllocationRow, ProjectError> {
        self.repo
            .find_by_id(project_id)
            .await?
            .ok_or(ProjectError::NotFound { uuid: project_id })?;

        let collaborator_id = input.collaborator_id;
        if !self.repo.collaborator_exists(collaborator_id).await? {
            return Err(ProjectError::CollaboratorNotFound {
                uuid: collaborator_id,
            });
        }

        let row = CreateProjectDailyAllocationRow {
            fk_project: project_id,
            fk_collaborator: collaborator_id,
            dt_work_date: input.work_date,
            nr_hours_worked: parse_decimal(input.hours_worked, "hours_worked")?,
            nr_hourly_rate_snapshot: parse_decimal(input.hourly_rate_snapshot, "hourly_rate_snapshot")?,
            tx_notes: input.notes,
            bl_present: input.present,
        };

        self.repo.create_allocation(row).await
    }
}

#[async_trait]
impl<R: ProjectRepository> ListAllocationsUseCase for ProjectService<R> {
    async fn execute(
        &self,
        project_id: Uuid,
    ) -> Result<Vec<ProjectDailyAllocationRow>, ProjectError> {
        self.repo
            .find_by_id(project_id)
            .await?
            .ok_or(ProjectError::NotFound { uuid: project_id })?;

        self.repo.find_allocations_by_project_id(project_id).await
    }
}

#[async_trait]
impl<R: ProjectRepository> UpdateAllocationUseCase for ProjectService<R> {
    async fn execute(
        &self,
        project_id: Uuid,
        allocation_id: Uuid,
        input: UpdateAllocationInput,
    ) -> Result<ProjectDailyAllocationRow, ProjectError> {
        self.repo
            .find_by_id(project_id)
            .await?
            .ok_or(ProjectError::NotFound { uuid: project_id })?;

        let current = self
            .repo
            .find_allocation_by_id(allocation_id)
            .await?
            .ok_or(ProjectError::AllocationNotFound {
                uuid: allocation_id,
            })?;

        if current.fk_project != project_id {
            return Err(ProjectError::AllocationNotFound {
                uuid: allocation_id,
            });
        }

        self.repo
            .update_allocation(
                allocation_id,
                db::project_rows::UpdateProjectDailyAllocationRow {
                    nr_hours_worked: parse_decimal(input.hours_worked, "hours_worked")?,
                    nr_hourly_rate_snapshot: parse_decimal(input.hourly_rate_snapshot, "hourly_rate_snapshot")?,
                    tx_notes: input.notes,
                    bl_present: input.present,
                },
            )
            .await
    }
}

#[async_trait]
impl<R: ProjectRepository> GetCostReportUseCase for ProjectService<R> {
    async fn execute(
        &self,
        project_id: Uuid,
    ) -> Result<crate::domain::ports::use_cases::project_use_cases::CostReportData, ProjectError>
    {
        let project = self
            .repo
            .find_by_id(project_id)
            .await?
            .ok_or(ProjectError::NotFound { uuid: project_id })?;

        let estimated_cost = project
            .nr_estimated_cost
            .clone()
            .unwrap_or_else(|| BigDecimal::from(0));
        let actual_cost = project
            .nr_actual_cost
            .clone()
            .unwrap_or_else(|| BigDecimal::from(0));
        let variance = &actual_cost - &estimated_cost;
        let variance_pct = if estimated_cost != 0 {
            Some((&variance / &estimated_cost) * BigDecimal::from(100))
        } else {
            None
        };

        Ok(
            crate::domain::ports::use_cases::project_use_cases::CostReportData {
                project_id: project.pk_project,
                project_name: project.tx_name,
                estimated_cost: project.nr_estimated_cost,
                actual_cost,
                variance,
                variance_pct,
            },
        )
    }
}

#[async_trait]
impl<R: ProjectRepository> GetProgressReportUseCase for ProjectService<R> {
    async fn execute(
        &self,
        project_id: Uuid,
    ) -> Result<crate::domain::ports::use_cases::project_use_cases::ProgressReportData, ProjectError>
    {
        let project = self
            .repo
            .find_by_id(project_id)
            .await?
            .ok_or(ProjectError::NotFound { uuid: project_id })?;

        let stages = self.repo.find_stages_by_project_id(project_id).await?;
        let total_stages = stages.len() as i32;
        let completed_stages = stages.iter().filter(|s| s.tx_status == "completed").count() as i32;
        let progress_pct = if total_stages > 0 {
            (BigDecimal::from(completed_stages) / BigDecimal::from(total_stages))
                * BigDecimal::from(100)
        } else {
            BigDecimal::from(0)
        };

        Ok(
            crate::domain::ports::use_cases::project_use_cases::ProgressReportData {
                project_id: project.pk_project,
                project_name: project.tx_name,
                stages,
                total_stages,
                completed_stages,
                progress_pct,
            },
        )
    }
}

#[async_trait]
impl<R: ProjectRepository> GetHistoryReportUseCase for ProjectService<R> {
    async fn execute(
        &self,
        collaborator_id: Uuid,
    ) -> Result<crate::domain::ports::use_cases::project_use_cases::HistoryReportData, ProjectError>
    {
        let allocations = self
            .repo
            .find_allocations_by_collaborator_id(collaborator_id)
            .await?;

        let total_days = allocations.len() as i32;
        let total_hours: BigDecimal = allocations
            .iter()
            .filter_map(|a| a.nr_hours_worked.as_ref())
            .fold(BigDecimal::from(0), |acc, h| acc + h);

        let history_entries: Vec<
            crate::domain::ports::use_cases::project_use_cases::AllocationHistoryEntry,
        > = allocations
            .into_iter()
            .map(
                |a| crate::domain::ports::use_cases::project_use_cases::AllocationHistoryEntry {
                    allocation_id: a.pk_project_daily_allocation,
                    project_id: a.fk_project,
                    project_name: a.project_name,
                    work_date: a.dt_work_date,
                    hours_worked: a.nr_hours_worked,
                    hourly_rate_snapshot: a.nr_hourly_rate_snapshot,
                    present: a.bl_present,
                },
            )
            .collect();

        Ok(
            crate::domain::ports::use_cases::project_use_cases::HistoryReportData {
                collaborator_id,
                collaborator_name: String::new(),
                allocations: history_entries,
                total_days,
                total_hours,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::db::project_rows::{
        AllocationWithProjectName, CreateProjectDailyAllocationRow, CreateProjectRow,
        CreateProjectStageRow, ProjectDailyAllocationRow, ProjectRow, ProjectStageRow,
        UpdateProjectDailyAllocationRow, UpdateProjectRow, UpdateProjectStageRow,
    };
    use crate::domain::ports::repositories::project_repository::{
        CreateAllocation, CreateProject, CreateStage, DeleteProject, FindAllProjects,
        FindAllocationById, FindAllocationsByCollaboratorId, FindAllocationsByProjectId,
        FindCollaboratorById, FindProjectById, FindStageById, FindStagesByProjectId,
        UpdateAllocation, UpdateProject, UpdateStage,
    };
    use crate::domain::ports::use_cases::project_use_cases::{
        CancelProjectUseCase, CompleteProjectUseCase, CreateAllocationInput, CreateAllocationUseCase,
        DeleteProjectUseCase, FindProjectUseCase, ListProjectsUseCase, PauseProjectUseCase,
        StartProjectUseCase, UpdateProjectUseCase,
    };

    #[derive(Default)]
    struct MockRepo {
        find_by_id_result: Option<ProjectRow>,
        find_all_result: Vec<ProjectRow>,
        collaborator_exists_result: bool,
    }

    impl MockRepo {
        fn new() -> Self {
            Self {
                collaborator_exists_result: true, // default: collaborator exists
                ..Self::default()
            }
        }
    }

    use sqlx::types::chrono::NaiveDateTime;

    fn now() -> NaiveDateTime {
        NaiveDateTime::default()
    }

    fn make_project_row() -> ProjectRow {
        ProjectRow {
            pk_project: Uuid::now_v7(),
            idx_project: 1,
            tx_name: "Test Project".to_string(),
            tx_description: None,
            tx_status: "planning".to_string(),
            dt_start_date: None,
            dt_estimated_end_date: None,
            dt_actual_end_date: None,
            nr_total_area_m2: None,
            nr_estimated_cost: None,
            nr_actual_cost: None,
            tx_notes: None,
            bl_active: true,
            ts_project_created_at: now(),
            ts_project_updated_at: now(),
            fk_address: Uuid::now_v7(),
        }
    }

    #[async_trait]
    impl FindProjectById for MockRepo {
        async fn find_by_id(&self, _uuid: Uuid) -> Result<Option<ProjectRow>, ProjectError> {
            Ok(self.find_by_id_result.clone())
        }
    }

    #[async_trait]
    impl FindAllProjects for MockRepo {
        async fn find_all(&self) -> Result<Vec<ProjectRow>, ProjectError> {
            Ok(self.find_all_result.clone())
        }
    }
    #[async_trait]
    impl CreateProject for MockRepo {
        async fn create(&self, _input: CreateProjectRow) -> Result<ProjectRow, ProjectError> {
            Ok(make_project_row())
        }
    }
    #[async_trait]
    impl UpdateProject for MockRepo {
        async fn update(
            &self,
            uuid: Uuid,
            _input: UpdateProjectRow,
        ) -> Result<ProjectRow, ProjectError> {
            Ok(ProjectRow {
                pk_project: uuid,
                ..make_project_row()
            })
        }
    }
    #[async_trait]
    impl DeleteProject for MockRepo {
        async fn delete(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError> {
            Ok(ProjectRow {
                pk_project: uuid,
                ..make_project_row()
            })
        }
    }
    #[async_trait]
    impl FindStageById for MockRepo {
        async fn find_stage_by_id(
            &self,
            _uuid: Uuid,
        ) -> Result<Option<ProjectStageRow>, ProjectError> {
            Ok(None)
        }
    }
    #[async_trait]
    impl CreateStage for MockRepo {
        async fn create_stage(
            &self,
            _input: CreateProjectStageRow,
        ) -> Result<ProjectStageRow, ProjectError> {
            unreachable!()
        }
    }
    #[async_trait]
    impl UpdateStage for MockRepo {
        async fn update_stage(
            &self,
            _uuid: Uuid,
            _input: UpdateProjectStageRow,
        ) -> Result<ProjectStageRow, ProjectError> {
            unreachable!()
        }
    }
    #[async_trait]
    impl FindAllocationById for MockRepo {
        async fn find_allocation_by_id(
            &self,
            _uuid: Uuid,
        ) -> Result<Option<ProjectDailyAllocationRow>, ProjectError> {
            Ok(None)
        }
    }
    #[async_trait]
    impl FindAllocationsByProjectId for MockRepo {
        async fn find_allocations_by_project_id(
            &self,
            _id: Uuid,
        ) -> Result<Vec<ProjectDailyAllocationRow>, ProjectError> {
            Ok(vec![])
        }
    }
    #[async_trait]
    impl CreateAllocation for MockRepo {
        async fn create_allocation(
            &self,
            input: CreateProjectDailyAllocationRow,
        ) -> Result<ProjectDailyAllocationRow, ProjectError> {
            Ok(ProjectDailyAllocationRow {
                pk_project_daily_allocation: Uuid::now_v7(),
                idx_project_daily_allocation: 1,
                fk_project: input.fk_project,
                fk_collaborator: input.fk_collaborator,
                dt_work_date: input.dt_work_date,
                nr_hours_worked: input.nr_hours_worked,
                nr_hourly_rate_snapshot: input.nr_hourly_rate_snapshot,
                tx_notes: input.tx_notes,
                bl_present: input.bl_present,
                ts_allocated_collaborator_created_at: now(),
                ts_allocated_collaborator_updated_at: now(),
            })
        }
    }
    #[async_trait]
    impl UpdateAllocation for MockRepo {
        async fn update_allocation(
            &self,
            _uuid: Uuid,
            _input: UpdateProjectDailyAllocationRow,
        ) -> Result<ProjectDailyAllocationRow, ProjectError> {
            unreachable!()
        }
    }
    #[async_trait]
    impl FindStagesByProjectId for MockRepo {
        async fn find_stages_by_project_id(
            &self,
            _id: Uuid,
        ) -> Result<Vec<ProjectStageRow>, ProjectError> {
            Ok(vec![])
        }
    }
    #[async_trait]
    impl FindAllocationsByCollaboratorId for MockRepo {
        async fn find_allocations_by_collaborator_id(
            &self,
            _id: Uuid,
        ) -> Result<Vec<AllocationWithProjectName>, ProjectError> {
            Ok(vec![])
        }
    }
    #[async_trait]
    impl FindCollaboratorById for MockRepo {
        async fn collaborator_exists(&self, _collaborator_id: Uuid) -> Result<bool, ProjectError> {
            Ok(self.collaborator_exists_result)
        }
    }

    #[tokio::test]
    async fn find_project_returns_row_when_exists() {
        let row = make_project_row();
        let uuid = row.pk_project;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = ProjectService::new(repo);
        let result = FindProjectUseCase::execute(&service, uuid).await.unwrap();
        assert_eq!(result.pk_project, uuid);
        assert_eq!(result.tx_name, "Test Project");
    }

    #[tokio::test]
    async fn find_project_returns_not_found_when_missing() {
        let uuid = Uuid::now_v7();
        let repo = MockRepo::new();
        let service = ProjectService::new(repo);
        let result = FindProjectUseCase::execute(&service, uuid).await;
        assert!(matches!(result, Err(ProjectError::NotFound { .. })));
    }

    #[tokio::test]
    async fn list_projects_returns_all() {
        let p1 = make_project_row();
        let p2 = make_project_row();
        let mut repo = MockRepo::new();
        repo.find_all_result = vec![p1, p2];
        let service = ProjectService::new(repo);
        let result = ListProjectsUseCase::execute(&service).await.unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn start_project_succeeds_when_planning() {
        let row = make_project_row();
        let uuid = row.pk_project;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = ProjectService::new(repo);
        let result = StartProjectUseCase::execute(&service, uuid).await.unwrap();
        assert_eq!(result.pk_project, uuid);
    }

    #[tokio::test]
    async fn start_project_fails_when_already_in_progress() {
        let mut row = make_project_row();
        row.tx_status = "in_progress".to_string();
        let uuid = row.pk_project;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = ProjectService::new(repo);
        let result = StartProjectUseCase::execute(&service, uuid).await;
        assert!(matches!(result, Err(ProjectError::AlreadyInStatus { .. })));
    }

    #[tokio::test]
    async fn pause_project_succeeds_when_in_progress() {
        let mut row = make_project_row();
        row.tx_status = "in_progress".to_string();
        let uuid = row.pk_project;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = ProjectService::new(repo);
        let result = PauseProjectUseCase::execute(&service, uuid).await.unwrap();
        assert_eq!(result.pk_project, uuid);
    }

    #[tokio::test]
    async fn pause_project_fails_when_already_paused() {
        let mut row = make_project_row();
        row.tx_status = "paused".to_string();
        let uuid = row.pk_project;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = ProjectService::new(repo);
        let result = PauseProjectUseCase::execute(&service, uuid).await;
        assert!(matches!(result, Err(ProjectError::AlreadyInStatus { .. })));
    }

    #[tokio::test]
    async fn complete_project_succeeds() {
        let mut row = make_project_row();
        row.tx_status = "in_progress".to_string(); // planning → in_progress → completed
        let uuid = row.pk_project;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = ProjectService::new(repo);
        let result = CompleteProjectUseCase::execute(&service, uuid)
            .await
            .unwrap();
        assert_eq!(result.pk_project, uuid);
    }

    #[tokio::test]
    async fn cancel_project_succeeds() {
        let row = make_project_row();
        let uuid = row.pk_project;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = ProjectService::new(repo);
        let result = CancelProjectUseCase::execute(&service, uuid).await.unwrap();
        assert_eq!(result.pk_project, uuid);
    }

    // --- State machine tests ---

    #[test]
    fn valid_transition_planning_to_in_progress() {
        assert!(valid_transition("planning", &ProjectStatus::InProgress));
    }

    #[test]
    fn valid_transition_planning_to_cancelled() {
        assert!(valid_transition("planning", &ProjectStatus::Cancelled));
    }

    #[test]
    fn invalid_transition_planning_to_paused() {
        assert!(!valid_transition("planning", &ProjectStatus::Paused));
    }

    #[test]
    fn invalid_transition_planning_to_completed() {
        assert!(!valid_transition("planning", &ProjectStatus::Completed));
    }

    #[test]
    fn invalid_transition_completed_is_terminal() {
        assert!(!valid_transition("completed", &ProjectStatus::InProgress));
        assert!(!valid_transition("completed", &ProjectStatus::Paused));
        assert!(!valid_transition("completed", &ProjectStatus::Cancelled));
    }

    #[test]
    fn invalid_transition_cancelled_is_terminal() {
        assert!(!valid_transition("cancelled", &ProjectStatus::InProgress));
        assert!(!valid_transition("cancelled", &ProjectStatus::Paused));
        assert!(!valid_transition("cancelled", &ProjectStatus::Completed));
    }

    #[test]
    fn check_transition_same_status_returns_already_in_status() {
        let uuid = Uuid::now_v7();
        let result = check_transition(uuid, "in_progress", &ProjectStatus::InProgress);
        assert!(matches!(result, Err(ProjectError::AlreadyInStatus { .. })));
    }

    #[test]
    fn check_transition_invalid_returns_invalid_transition() {
        let uuid = Uuid::now_v7();
        let result = check_transition(uuid, "completed", &ProjectStatus::InProgress);
        assert!(matches!(result, Err(ProjectError::InvalidTransition { .. })));
    }

    #[tokio::test]
    async fn start_project_fails_when_completed() {
        let mut row = make_project_row();
        row.tx_status = "completed".to_string();
        let uuid = row.pk_project;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = ProjectService::new(repo);
        let result = StartProjectUseCase::execute(&service, uuid).await;
        assert!(matches!(result, Err(ProjectError::InvalidTransition { .. })));
    }

    #[tokio::test]
    async fn pause_project_fails_when_planning() {
        let row = make_project_row(); // status = "planning"
        let uuid = row.pk_project;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = ProjectService::new(repo);
        let result = PauseProjectUseCase::execute(&service, uuid).await;
        assert!(matches!(result, Err(ProjectError::InvalidTransition { .. })));
    }

    #[tokio::test]
    async fn complete_project_fails_when_paused() {
        let mut row = make_project_row();
        row.tx_status = "paused".to_string();
        let uuid = row.pk_project;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = ProjectService::new(repo);
        let result = CompleteProjectUseCase::execute(&service, uuid).await;
        assert!(matches!(result, Err(ProjectError::InvalidTransition { .. })));
    }

    #[tokio::test]
    async fn cancel_project_from_cancelled_returns_already_in_status() {
        let mut row = make_project_row();
        row.tx_status = "cancelled".to_string();
        let uuid = row.pk_project;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = ProjectService::new(repo);
        let result = CancelProjectUseCase::execute(&service, uuid).await;
        assert!(matches!(result, Err(ProjectError::AlreadyInStatus { .. })));
    }

    #[tokio::test]
    async fn delete_project_succeeds_when_exists() {
        let row = make_project_row();
        let uuid = row.pk_project;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = ProjectService::new(repo);
        let result = DeleteProjectUseCase::execute(&service, uuid).await.unwrap();
        assert_eq!(result.pk_project, uuid);
    }

    #[tokio::test]
    async fn create_allocation_fails_when_collaborator_not_found() {
        let project_row = make_project_row();
        let project_id = project_row.pk_project;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(project_row);
        repo.collaborator_exists_result = false;
        let service = ProjectService::new(repo);
        let collaborator_id = Uuid::now_v7(); // colaborador inexistente
        let input = CreateAllocationInput {
            collaborator_id,
            work_date: chrono::NaiveDate::from_ymd_opt(2023, 11, 30).expect("valid date"),
            hours_worked: Some("8".to_string()),
            hourly_rate_snapshot: Some("10".to_string()),
            notes: Some("test".to_string()),
            present: true,
        };
        let result = CreateAllocationUseCase::execute(&service, project_id, input).await;
        match result {
            Err(ProjectError::CollaboratorNotFound { uuid }) => assert_eq!(uuid, collaborator_id),
            other => panic!("Expected CollaboratorNotFound, got: {other:?}"),
        }
    }
    #[tokio::test]
    async fn delete_project_fails_when_not_found() {
        let uuid = Uuid::now_v7();
        let repo = MockRepo::new();
        let service = ProjectService::new(repo);
        let result = DeleteProjectUseCase::execute(&service, uuid).await;
        assert!(matches!(result, Err(ProjectError::NotFound { .. })));
    }

    #[tokio::test]
    async fn update_project_returns_not_found_when_missing() {
        let uuid = Uuid::now_v7();
        let repo = MockRepo::new();
        let service = ProjectService::new(repo);
        let input = UpdateProjectInput {
            name: Some("Updated".to_string()),
            description: None,
            start_date: None,
            estimated_end_date: None,
            actual_end_date: None,
            total_area_m2: None,
            estimated_cost: None,
            actual_cost: None,
            notes: None,
            active: None,
        };
        let result = UpdateProjectUseCase::execute(&service, uuid, input).await;
        assert!(matches!(result, Err(ProjectError::NotFound { .. })));
    }

    #[test]
    fn error_not_found_message_contains_uuid() {
        let uuid = Uuid::now_v7();
        let err = ProjectError::NotFound { uuid };
        let msg = err.to_string();
        assert!(msg.contains(&uuid.to_string()));
        assert!(msg.contains("not found"));
    }

    #[test]
    fn error_already_in_status_message_contains_status() {
        let uuid = Uuid::now_v7();
        let err = ProjectError::AlreadyInStatus {
            uuid,
            status: "in_progress".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("in_progress"));
    }

    #[test]
    fn parse_decimal_valid_string_returns_bigdecimal() {
        let result = parse_decimal(Some("150.75".to_string()), "total_area_m2");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("150.75".parse::<BigDecimal>().unwrap()));
    }

    #[test]
    fn parse_decimal_invalid_string_returns_invalid_field() {
        let result = parse_decimal(Some("abc".to_string()), "total_area_m2");
        assert!(matches!(
            result,
            Err(ProjectError::InvalidField {
                field: "total_area_m2",
                ..
            })
        ));
    }

    #[test]
    fn parse_decimal_none_returns_none() {
        let result = parse_decimal(None, "estimated_cost");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }
}

