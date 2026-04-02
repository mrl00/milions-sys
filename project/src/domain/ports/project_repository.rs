use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::ProjectError;
use crate::domain::models::db::project_rows::{
    CreateProjectRow, CreateProjectStageRow, ProjectRow, ProjectStageRow, UpdateProjectRow,
    UpdateProjectStageRow,
};

#[async_trait]
pub trait FindProjectById: Send + Sync {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<ProjectRow>, ProjectError>;
}

#[async_trait]
pub trait FindProjectByClientId: Send + Sync {
    async fn find_by_client_id(&self, client_id: Uuid) -> Result<Vec<ProjectRow>, ProjectError>;
}

#[async_trait]
pub trait FindAllProjects: Send + Sync {
    async fn find_all(&self) -> Result<Vec<ProjectRow>, ProjectError>;
}

#[async_trait]
pub trait CreateProject: Send + Sync {
    async fn create(&self, input: CreateProjectRow) -> Result<ProjectRow, ProjectError>;
}

#[async_trait]
pub trait UpdateProject: Send + Sync {
    async fn update(
        &self,
        uuid: Uuid,
        input: UpdateProjectRow,
    ) -> Result<ProjectRow, ProjectError>;
}

#[async_trait]
pub trait DeleteProject: Send + Sync {
    async fn delete(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError>;
}

#[async_trait]
pub trait FindStageById: Send + Sync {
    async fn find_stage_by_id(&self, uuid: Uuid) -> Result<Option<ProjectStageRow>, ProjectError>;
}

#[async_trait]
pub trait CreateStage: Send + Sync {
    async fn create_stage(&self, input: CreateProjectStageRow) -> Result<ProjectStageRow, ProjectError>;
}

#[async_trait]
pub trait UpdateStage: Send + Sync {
    async fn update_stage(
        &self,
        uuid: Uuid,
        input: UpdateProjectStageRow,
    ) -> Result<ProjectStageRow, ProjectError>;
}

pub trait FindAndCreateProject: FindProjectByClientId + CreateProject {}
pub trait FindAndUpdateProject: FindProjectById + UpdateProject {}
pub trait FindAndDeleteProject: FindProjectById + DeleteProject {}
pub trait FindAndCreateStage: FindStageById + CreateStage {}
pub trait FindAndUpdateStage: FindStageById + UpdateStage {}
