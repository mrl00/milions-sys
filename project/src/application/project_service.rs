use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::adapters::driven::postgres::PgProjectRepository;
use crate::domain::errors::ProjectError;
use crate::domain::models::db::project_rows::{
    CreateProjectDailyAllocationRow, CreateProjectRow, CreateProjectStageRow,
    ProjectDailyAllocationRow, ProjectRow, ProjectStageRow, ProjectStageStatus, ProjectStatus,
    UpdateProjectRow,
};
use crate::domain::ports::project_repository::{
    CreateAllocation as _, CreateProject as _, CreateStage as _, DeleteProject as _,
    FindAllProjects as _, FindAllocationById as _, FindAllocationsByProjectId as _,
    FindProjectByClientId as _, FindProjectById as _, FindStageById as _, UpdateAllocation as _,
    UpdateProject as _, UpdateStage as _,
};
use crate::domain::ports::project_use_cases::{
    CancelProject, CompleteProject, CreateAllocation as CreateAllocationTrait,
    CreateAllocationInput, CreateProject as CreateProjectTrait, CreateProjectInput,
    CreateStage as CreateStageTrait, CreateStageInput, DeleteProject as DeleteProjectTrait,
    FindProject, ListAllocations, ListProjects, ListProjectsByClient, PauseProject, StartProject,
    UpdateAllocation as UpdateAllocationTrait, UpdateAllocationInput,
    UpdateProject as UpdateProjectTrait, UpdateProjectInput, UpdateStage as UpdateStageTrait,
    UpdateStageInput,
};

pub struct ProjectService {
    repo: PgProjectRepository,
}

impl ProjectService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: PgProjectRepository::new(pool),
        }
    }
}

#[async_trait]
impl FindProject for ProjectService {
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ProjectError::NotFound { uuid })
    }
}

#[async_trait]
impl ListProjects for ProjectService {
    async fn execute(&self) -> Result<Vec<ProjectRow>, ProjectError> {
        self.repo.find_all().await
    }
}

#[async_trait]
impl ListProjectsByClient for ProjectService {
    async fn execute(&self, client_id: Uuid) -> Result<Vec<ProjectRow>, ProjectError> {
        self.repo.find_by_client_id(client_id).await
    }
}

#[async_trait]
impl CreateProjectTrait for ProjectService {
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
impl UpdateProjectTrait for ProjectService {
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
impl StartProject for ProjectService {
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
impl PauseProject for ProjectService {
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
impl CompleteProject for ProjectService {
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
impl CancelProject for ProjectService {
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
impl DeleteProjectTrait for ProjectService {
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ProjectError::NotFound { uuid })?;

        self.repo.delete(uuid).await
    }
}

#[async_trait]
impl CreateStageTrait for ProjectService {
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
impl UpdateStageTrait for ProjectService {
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
impl CreateAllocationTrait for ProjectService {
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
impl ListAllocations for ProjectService {
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
impl UpdateAllocationTrait for ProjectService {
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
