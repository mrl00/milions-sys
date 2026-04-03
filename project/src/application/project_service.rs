use async_trait::async_trait;
use sqlx::types::BigDecimal;
use uuid::Uuid;

use crate::adapters::driven::postgres::PgProjectRepository;
use crate::domain::errors::ProjectError;
use crate::domain::models::db::project_rows::{
    CreateProjectDailyAllocationRow, CreateProjectRow, CreateProjectStageRow,
    ProjectDailyAllocationRow, ProjectRow, ProjectStageRow, ProjectStageStatus, ProjectStatus,
    UpdateProjectRow,
};
use crate::domain::ports::project_repository::{
    CreateAllocation, CreateProject, CreateStage, DeleteProject, FindAllProjects,
    FindAllocationById, FindAllocationsByCollaboratorId, FindAllocationsByProjectId,
    FindProjectByClientId, FindProjectById, FindStageById, FindStagesByProjectId, UpdateAllocation,
    UpdateProject, UpdateStage,
};
use crate::domain::ports::project_use_cases::{
    CancelProject, CompleteProject, CreateAllocation as CreateAllocationTrait,
    CreateAllocationInput, CreateProject as CreateProjectTrait, CreateProjectInput,
    CreateStage as CreateStageTrait, CreateStageInput, DeleteProject as DeleteProjectTrait,
    FindProject, GetCostReport, GetHistoryReport, GetProgressReport, ListAllocations, ListProjects,
    ListProjectsByClient, PauseProject, StartProject, UpdateAllocation as UpdateAllocationTrait,
    UpdateAllocationInput, UpdateProject as UpdateProjectTrait, UpdateProjectInput,
    UpdateStage as UpdateStageTrait, UpdateStageInput,
};

pub struct ProjectService<R> {
    repo: R,
}

impl<R> ProjectService<R>
where
    R: FindProjectById
        + FindProjectByClientId
        + FindAllProjects
        + CreateProject
        + UpdateProject
        + DeleteProject
        + FindStageById
        + CreateStage
        + UpdateStage
        + FindAllocationById
        + FindAllocationsByProjectId
        + CreateAllocation
        + UpdateAllocation
        + FindStagesByProjectId
        + FindAllocationsByCollaboratorId,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

pub type ConcreteProjectService = ProjectService<PgProjectRepository>;

#[async_trait]
impl<R> FindProject for ProjectService<R>
where
    R: FindProjectById
        + FindProjectByClientId
        + FindAllProjects
        + CreateProject
        + UpdateProject
        + DeleteProject
        + FindStageById
        + CreateStage
        + UpdateStage
        + FindAllocationById
        + FindAllocationsByProjectId
        + CreateAllocation
        + UpdateAllocation
        + FindStagesByProjectId
        + FindAllocationsByCollaboratorId
        + Send
        + Sync,
{
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ProjectError::NotFound { uuid })
    }
}

#[async_trait]
impl<R> ListProjects for ProjectService<R>
where
    R: FindProjectById
        + FindProjectByClientId
        + FindAllProjects
        + CreateProject
        + UpdateProject
        + DeleteProject
        + FindStageById
        + CreateStage
        + UpdateStage
        + FindAllocationById
        + FindAllocationsByProjectId
        + CreateAllocation
        + UpdateAllocation
        + FindStagesByProjectId
        + FindAllocationsByCollaboratorId
        + Send
        + Sync,
{
    async fn execute(&self) -> Result<Vec<ProjectRow>, ProjectError> {
        self.repo.find_all().await
    }
}

#[async_trait]
impl<R> ListProjectsByClient for ProjectService<R>
where
    R: FindProjectById
        + FindProjectByClientId
        + FindAllProjects
        + CreateProject
        + UpdateProject
        + DeleteProject
        + FindStageById
        + CreateStage
        + UpdateStage
        + FindAllocationById
        + FindAllocationsByProjectId
        + CreateAllocation
        + UpdateAllocation
        + FindStagesByProjectId
        + FindAllocationsByCollaboratorId
        + Send
        + Sync,
{
    async fn execute(&self, client_id: Uuid) -> Result<Vec<ProjectRow>, ProjectError> {
        self.repo.find_by_client_id(client_id).await
    }
}

#[async_trait]
impl<R> CreateProjectTrait for ProjectService<R>
where
    R: FindProjectById
        + FindProjectByClientId
        + FindAllProjects
        + CreateProject
        + UpdateProject
        + DeleteProject
        + FindStageById
        + CreateStage
        + UpdateStage
        + FindAllocationById
        + FindAllocationsByProjectId
        + CreateAllocation
        + UpdateAllocation
        + FindStagesByProjectId
        + FindAllocationsByCollaboratorId
        + Send
        + Sync,
{
    async fn execute(&self, input: CreateProjectInput) -> Result<ProjectRow, ProjectError> {
        let row = CreateProjectRow {
            tx_name: input.name,
            tx_description: input.description,
            tx_status: ProjectStatus::Planning,
            dt_start_date: input.start_date,
            dt_estimated_end_date: input.estimated_end_date,
            nr_total_area_m2: input.total_area_m2,
            nr_estimated_cost: input.estimated_cost,
            tx_notes: input.notes,
            fk_client: input.client_id,
            fk_address: input.address_id,
        };

        self.repo.create(row).await
    }
}

#[async_trait]
impl<R> UpdateProjectTrait for ProjectService<R>
where
    R: FindProjectById
        + FindProjectByClientId
        + FindAllProjects
        + CreateProject
        + UpdateProject
        + DeleteProject
        + FindStageById
        + CreateStage
        + UpdateStage
        + FindAllocationById
        + FindAllocationsByProjectId
        + CreateAllocation
        + UpdateAllocation
        + FindStagesByProjectId
        + FindAllocationsByCollaboratorId
        + Send
        + Sync,
{
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
                    nr_total_area_m2: input.total_area_m2,
                    nr_estimated_cost: input.estimated_cost,
                    nr_actual_cost: input.actual_cost,
                    tx_notes: input.notes,
                    bl_active: input.active,
                },
            )
            .await
    }
}

#[async_trait]
impl<R> StartProject for ProjectService<R>
where
    R: FindProjectById
        + FindProjectByClientId
        + FindAllProjects
        + CreateProject
        + UpdateProject
        + DeleteProject
        + FindStageById
        + CreateStage
        + UpdateStage
        + FindAllocationById
        + FindAllocationsByProjectId
        + CreateAllocation
        + UpdateAllocation
        + FindStagesByProjectId
        + FindAllocationsByCollaboratorId
        + Send
        + Sync,
{
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError> {
        let current = self
            .repo
            .find_by_id(uuid)
            .await?
            .ok_or(ProjectError::NotFound { uuid })?;

        if current.tx_status == ProjectStatus::InProgress.to_string() {
            return Err(ProjectError::AlreadyInStatus {
                uuid,
                status: ProjectStatus::InProgress.to_string(),
            });
        }

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
impl<R> PauseProject for ProjectService<R>
where
    R: FindProjectById
        + FindProjectByClientId
        + FindAllProjects
        + CreateProject
        + UpdateProject
        + DeleteProject
        + FindStageById
        + CreateStage
        + UpdateStage
        + FindAllocationById
        + FindAllocationsByProjectId
        + CreateAllocation
        + UpdateAllocation
        + FindStagesByProjectId
        + FindAllocationsByCollaboratorId
        + Send
        + Sync,
{
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError> {
        let current = self
            .repo
            .find_by_id(uuid)
            .await?
            .ok_or(ProjectError::NotFound { uuid })?;

        if current.tx_status == ProjectStatus::Paused.to_string() {
            return Err(ProjectError::AlreadyInStatus {
                uuid,
                status: ProjectStatus::Paused.to_string(),
            });
        }

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
impl<R> CompleteProject for ProjectService<R>
where
    R: FindProjectById
        + FindProjectByClientId
        + FindAllProjects
        + CreateProject
        + UpdateProject
        + DeleteProject
        + FindStageById
        + CreateStage
        + UpdateStage
        + FindAllocationById
        + FindAllocationsByProjectId
        + CreateAllocation
        + UpdateAllocation
        + FindStagesByProjectId
        + FindAllocationsByCollaboratorId
        + Send
        + Sync,
{
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError> {
        let current = self
            .repo
            .find_by_id(uuid)
            .await?
            .ok_or(ProjectError::NotFound { uuid })?;

        if current.tx_status == ProjectStatus::Completed.to_string() {
            return Err(ProjectError::AlreadyInStatus {
                uuid,
                status: ProjectStatus::Completed.to_string(),
            });
        }

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
impl<R> CancelProject for ProjectService<R>
where
    R: FindProjectById
        + FindProjectByClientId
        + FindAllProjects
        + CreateProject
        + UpdateProject
        + DeleteProject
        + FindStageById
        + CreateStage
        + UpdateStage
        + FindAllocationById
        + FindAllocationsByProjectId
        + CreateAllocation
        + UpdateAllocation
        + FindStagesByProjectId
        + FindAllocationsByCollaboratorId
        + Send
        + Sync,
{
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError> {
        let current = self
            .repo
            .find_by_id(uuid)
            .await?
            .ok_or(ProjectError::NotFound { uuid })?;

        if current.tx_status == ProjectStatus::Cancelled.to_string() {
            return Err(ProjectError::AlreadyInStatus {
                uuid,
                status: ProjectStatus::Cancelled.to_string(),
            });
        }

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
impl<R> DeleteProjectTrait for ProjectService<R>
where
    R: FindProjectById
        + FindProjectByClientId
        + FindAllProjects
        + CreateProject
        + UpdateProject
        + DeleteProject
        + FindStageById
        + CreateStage
        + UpdateStage
        + FindAllocationById
        + FindAllocationsByProjectId
        + CreateAllocation
        + UpdateAllocation
        + FindStagesByProjectId
        + FindAllocationsByCollaboratorId
        + Send
        + Sync,
{
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ProjectError::NotFound { uuid })?;

        self.repo.delete(uuid).await
    }
}

#[async_trait]
impl<R> CreateStageTrait for ProjectService<R>
where
    R: FindProjectById
        + FindProjectByClientId
        + FindAllProjects
        + CreateProject
        + UpdateProject
        + DeleteProject
        + FindStageById
        + CreateStage
        + UpdateStage
        + FindAllocationById
        + FindAllocationsByProjectId
        + CreateAllocation
        + UpdateAllocation
        + FindStagesByProjectId
        + FindAllocationsByCollaboratorId
        + Send
        + Sync,
{
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
impl<R> UpdateStageTrait for ProjectService<R>
where
    R: FindProjectById
        + FindProjectByClientId
        + FindAllProjects
        + CreateProject
        + UpdateProject
        + DeleteProject
        + FindStageById
        + CreateStage
        + UpdateStage
        + FindAllocationById
        + FindAllocationsByProjectId
        + CreateAllocation
        + UpdateAllocation
        + FindStagesByProjectId
        + FindAllocationsByCollaboratorId
        + Send
        + Sync,
{
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
impl<R> CreateAllocationTrait for ProjectService<R>
where
    R: FindProjectById
        + FindProjectByClientId
        + FindAllProjects
        + CreateProject
        + UpdateProject
        + DeleteProject
        + FindStageById
        + CreateStage
        + UpdateStage
        + FindAllocationById
        + FindAllocationsByProjectId
        + CreateAllocation
        + UpdateAllocation
        + FindStagesByProjectId
        + FindAllocationsByCollaboratorId
        + Send
        + Sync,
{
    async fn execute(
        &self,
        project_id: Uuid,
        input: CreateAllocationInput,
    ) -> Result<ProjectDailyAllocationRow, ProjectError> {
        self.repo
            .find_by_id(project_id)
            .await?
            .ok_or(ProjectError::NotFound { uuid: project_id })?;

        let row = CreateProjectDailyAllocationRow {
            fk_project: project_id,
            fk_collaborator: input.collaborator_id,
            dt_work_date: input.work_date,
            nr_hours_worked: input.hours_worked,
            nr_hourly_rate_snapshot: input.hourly_rate_snapshot,
            tx_notes: input.notes,
            bl_present: input.present,
        };

        self.repo.create_allocation(row).await
    }
}

#[async_trait]
impl<R> ListAllocations for ProjectService<R>
where
    R: FindProjectById
        + FindProjectByClientId
        + FindAllProjects
        + CreateProject
        + UpdateProject
        + DeleteProject
        + FindStageById
        + CreateStage
        + UpdateStage
        + FindAllocationById
        + FindAllocationsByProjectId
        + CreateAllocation
        + UpdateAllocation
        + FindStagesByProjectId
        + FindAllocationsByCollaboratorId
        + Send
        + Sync,
{
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
impl<R> UpdateAllocationTrait for ProjectService<R>
where
    R: FindProjectById
        + FindProjectByClientId
        + FindAllProjects
        + CreateProject
        + UpdateProject
        + DeleteProject
        + FindStageById
        + CreateStage
        + UpdateStage
        + FindAllocationById
        + FindAllocationsByProjectId
        + CreateAllocation
        + UpdateAllocation
        + FindStagesByProjectId
        + FindAllocationsByCollaboratorId
        + Send
        + Sync,
{
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
                crate::domain::models::db::project_rows::UpdateProjectDailyAllocationRow {
                    nr_hours_worked: input.hours_worked,
                    nr_hourly_rate_snapshot: input.hourly_rate_snapshot,
                    tx_notes: input.notes,
                    bl_present: input.present,
                },
            )
            .await
    }
}

#[async_trait]
impl<R> GetCostReport for ProjectService<R>
where
    R: FindProjectById
        + FindProjectByClientId
        + FindAllProjects
        + CreateProject
        + UpdateProject
        + DeleteProject
        + FindStageById
        + CreateStage
        + UpdateStage
        + FindAllocationById
        + FindAllocationsByProjectId
        + CreateAllocation
        + UpdateAllocation
        + FindStagesByProjectId
        + FindAllocationsByCollaboratorId
        + Send
        + Sync,
{
    async fn execute(
        &self,
        project_id: Uuid,
    ) -> Result<crate::domain::ports::project_use_cases::CostReportData, ProjectError> {
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

        Ok(crate::domain::ports::project_use_cases::CostReportData {
            project_id: project.pk_project,
            project_name: project.tx_name,
            estimated_cost: project.nr_estimated_cost,
            actual_cost,
            variance,
            variance_pct,
        })
    }
}

#[async_trait]
impl<R> GetProgressReport for ProjectService<R>
where
    R: FindProjectById
        + FindProjectByClientId
        + FindAllProjects
        + CreateProject
        + UpdateProject
        + DeleteProject
        + FindStageById
        + CreateStage
        + UpdateStage
        + FindAllocationById
        + FindAllocationsByProjectId
        + CreateAllocation
        + UpdateAllocation
        + FindStagesByProjectId
        + FindAllocationsByCollaboratorId
        + Send
        + Sync,
{
    async fn execute(
        &self,
        project_id: Uuid,
    ) -> Result<crate::domain::ports::project_use_cases::ProgressReportData, ProjectError> {
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
            crate::domain::ports::project_use_cases::ProgressReportData {
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
impl<R> GetHistoryReport for ProjectService<R>
where
    R: FindProjectById
        + FindProjectByClientId
        + FindAllProjects
        + CreateProject
        + UpdateProject
        + DeleteProject
        + FindStageById
        + CreateStage
        + UpdateStage
        + FindAllocationById
        + FindAllocationsByProjectId
        + CreateAllocation
        + UpdateAllocation
        + FindStagesByProjectId
        + FindAllocationsByCollaboratorId
        + Send
        + Sync,
{
    async fn execute(
        &self,
        collaborator_id: Uuid,
    ) -> Result<crate::domain::ports::project_use_cases::HistoryReportData, ProjectError> {
        let allocations = self
            .repo
            .find_allocations_by_collaborator_id(collaborator_id)
            .await?;

        let total_days = allocations.len() as i32;
        let total_hours: BigDecimal = allocations
            .iter()
            .filter_map(|a| a.nr_hours_worked.as_ref())
            .fold(BigDecimal::from(0), |acc, h| acc + h);

        let history_entries: Vec<crate::domain::ports::project_use_cases::AllocationHistoryEntry> =
            allocations
                .into_iter()
                .map(
                    |a| crate::domain::ports::project_use_cases::AllocationHistoryEntry {
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

        Ok(crate::domain::ports::project_use_cases::HistoryReportData {
            collaborator_id,
            collaborator_name: String::new(),
            allocations: history_entries,
            total_days,
            total_hours,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::errors::ProjectError;
    use crate::domain::models::db::project_rows::{
        AllocationWithProjectName, CreateProjectDailyAllocationRow, CreateProjectRow,
        CreateProjectStageRow, ProjectDailyAllocationRow, ProjectRow, ProjectStageRow,
        UpdateProjectDailyAllocationRow, UpdateProjectRow, UpdateProjectStageRow,
    };
    use crate::domain::ports::project_repository::{
        CreateAllocation, CreateProject, CreateStage, DeleteProject, FindAllProjects,
        FindAllocationById, FindAllocationsByCollaboratorId, FindAllocationsByProjectId,
        FindProjectByClientId, FindProjectById, FindStageById, FindStagesByProjectId,
        UpdateAllocation, UpdateProject, UpdateStage,
    };
    use crate::domain::ports::project_use_cases::{
        CancelProject, CompleteProject, DeleteProject as DeleteProjectTrait, FindProject,
        ListProjects, PauseProject, StartProject, UpdateProject as UpdateProjectTrait,
    };

    #[derive(Default)]
    struct MockRepo {
        find_by_id_result: Option<ProjectRow>,
        find_all_result: Vec<ProjectRow>,
    }

    impl MockRepo {
        fn new() -> Self {
            Self::default()
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
            fk_client: Uuid::now_v7(),
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
    impl FindProjectByClientId for MockRepo {
        async fn find_by_client_id(&self, _id: Uuid) -> Result<Vec<ProjectRow>, ProjectError> {
            Ok(vec![])
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
            _input: CreateProjectDailyAllocationRow,
        ) -> Result<ProjectDailyAllocationRow, ProjectError> {
            unreachable!()
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

    #[tokio::test]
    async fn find_project_returns_row_when_exists() {
        let row = make_project_row();
        let uuid = row.pk_project;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = ProjectService::new(repo);
        let result = FindProject::execute(&service, uuid).await.unwrap();
        assert_eq!(result.pk_project, uuid);
        assert_eq!(result.tx_name, "Test Project");
    }

    #[tokio::test]
    async fn find_project_returns_not_found_when_missing() {
        let uuid = Uuid::now_v7();
        let repo = MockRepo::new();
        let service = ProjectService::new(repo);
        let result = FindProject::execute(&service, uuid).await;
        assert!(matches!(result, Err(ProjectError::NotFound { .. })));
    }

    #[tokio::test]
    async fn list_projects_returns_all() {
        let p1 = make_project_row();
        let p2 = make_project_row();
        let mut repo = MockRepo::new();
        repo.find_all_result = vec![p1, p2];
        let service = ProjectService::new(repo);
        let result = ListProjects::execute(&service).await.unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn start_project_succeeds_when_planning() {
        let row = make_project_row();
        let uuid = row.pk_project;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = ProjectService::new(repo);
        let result = StartProject::execute(&service, uuid).await.unwrap();
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
        let result = StartProject::execute(&service, uuid).await;
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
        let result = PauseProject::execute(&service, uuid).await.unwrap();
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
        let result = PauseProject::execute(&service, uuid).await;
        assert!(matches!(result, Err(ProjectError::AlreadyInStatus { .. })));
    }

    #[tokio::test]
    async fn complete_project_succeeds() {
        let row = make_project_row();
        let uuid = row.pk_project;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = ProjectService::new(repo);
        let result = CompleteProject::execute(&service, uuid).await.unwrap();
        assert_eq!(result.pk_project, uuid);
    }

    #[tokio::test]
    async fn cancel_project_succeeds() {
        let row = make_project_row();
        let uuid = row.pk_project;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = ProjectService::new(repo);
        let result = CancelProject::execute(&service, uuid).await.unwrap();
        assert_eq!(result.pk_project, uuid);
    }

    #[tokio::test]
    async fn delete_project_succeeds_when_exists() {
        let row = make_project_row();
        let uuid = row.pk_project;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = ProjectService::new(repo);
        let result = DeleteProjectTrait::execute(&service, uuid).await.unwrap();
        assert_eq!(result.pk_project, uuid);
    }

    #[tokio::test]
    async fn delete_project_fails_when_not_found() {
        let uuid = Uuid::now_v7();
        let repo = MockRepo::new();
        let service = ProjectService::new(repo);
        let result = DeleteProjectTrait::execute(&service, uuid).await;
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
        let result = UpdateProjectTrait::execute(&service, uuid, input).await;
        assert!(matches!(result, Err(ProjectError::NotFound { .. })));
    }

    #[test]
    fn error_not_found_message_contains_uuid() {
        let uuid = Uuid::now_v7();
        let err = ProjectError::NotFound { uuid };
        let msg = err.to_string();
        assert!(msg.contains(&uuid.to_string()));
        assert!(msg.contains("não encontrado"));
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
}
