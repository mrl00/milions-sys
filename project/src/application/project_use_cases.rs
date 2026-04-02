use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::adapters::driven::postgres::PgProjectRepository;
use crate::domain::errors::ProjectError;
use crate::domain::models::db::project_rows::{
    CreateProjectRow, ProjectRow, ProjectStatus, UpdateProjectRow,
};
use crate::domain::ports::project_repository::*;
use crate::domain::use_cases::cancel_project::CancelProject;
use crate::domain::use_cases::complete_project::CompleteProject;
use crate::domain::use_cases::create_project::{
    CreateProject as CreateProjectTrait, CreateProjectInput,
};
use crate::domain::use_cases::delete_project::DeleteProject as DeleteProjectTrait;
use crate::domain::use_cases::find_project::FindProject;
use crate::domain::use_cases::list_projects::ListProjects;
use crate::domain::use_cases::list_projects_by_client::ListProjectsByClient;
use crate::domain::use_cases::pause_project::PauseProject;
use crate::domain::use_cases::start_project::StartProject;
use crate::domain::use_cases::update_project::{
    UpdateProject as UpdateProjectTrait, UpdateProjectInput,
};

pub struct ProjectUseCases {
    repo: PgProjectRepository,
}

impl ProjectUseCases {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: PgProjectRepository::new(pool),
        }
    }
}

#[async_trait]
impl FindProject for ProjectUseCases {
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ProjectError::NotFound { uuid })
    }
}

#[async_trait]
impl ListProjects for ProjectUseCases {
    async fn execute(&self) -> Result<Vec<ProjectRow>, ProjectError> {
        self.repo.find_all().await
    }
}

#[async_trait]
impl ListProjectsByClient for ProjectUseCases {
    async fn execute(&self, client_id: Uuid) -> Result<Vec<ProjectRow>, ProjectError> {
        self.repo.find_by_client_id(client_id).await
    }
}

#[async_trait]
impl CreateProjectTrait for ProjectUseCases {
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
impl UpdateProjectTrait for ProjectUseCases {
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
impl StartProject for ProjectUseCases {
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
impl PauseProject for ProjectUseCases {
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
impl CompleteProject for ProjectUseCases {
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
impl CancelProject for ProjectUseCases {
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
impl DeleteProjectTrait for ProjectUseCases {
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ProjectError::NotFound { uuid })?;

        self.repo.delete(uuid).await
    }
}
