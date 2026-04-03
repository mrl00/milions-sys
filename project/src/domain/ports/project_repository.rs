use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::ProjectError;
use crate::domain::models::db::project_rows::{
    CreateProjectDailyAllocationRow, CreateProjectRow, CreateProjectStageRow,
    ProjectDailyAllocationRow, ProjectRow, ProjectStageRow, UpdateProjectDailyAllocationRow,
    UpdateProjectRow, UpdateProjectStageRow,
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
    async fn update(&self, uuid: Uuid, input: UpdateProjectRow)
    -> Result<ProjectRow, ProjectError>;
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
    async fn create_stage(
        &self,
        input: CreateProjectStageRow,
    ) -> Result<ProjectStageRow, ProjectError>;
}

#[async_trait]
pub trait UpdateStage: Send + Sync {
    async fn update_stage(
        &self,
        uuid: Uuid,
        input: UpdateProjectStageRow,
    ) -> Result<ProjectStageRow, ProjectError>;
}

#[async_trait]
pub trait FindAllocationById: Send + Sync {
    async fn find_allocation_by_id(
        &self,
        uuid: Uuid,
    ) -> Result<Option<ProjectDailyAllocationRow>, ProjectError>;
}

#[async_trait]
pub trait FindAllocationsByProjectId: Send + Sync {
    async fn find_allocations_by_project_id(
        &self,
        project_id: Uuid,
    ) -> Result<Vec<ProjectDailyAllocationRow>, ProjectError>;
}

#[async_trait]
pub trait CreateAllocation: Send + Sync {
    async fn create_allocation(
        &self,
        input: CreateProjectDailyAllocationRow,
    ) -> Result<ProjectDailyAllocationRow, ProjectError>;
}

#[async_trait]
pub trait UpdateAllocation: Send + Sync {
    async fn update_allocation(
        &self,
        uuid: Uuid,
        input: UpdateProjectDailyAllocationRow,
    ) -> Result<ProjectDailyAllocationRow, ProjectError>;
}

pub trait FindAndCreateProject: FindProjectByClientId + CreateProject {}
pub trait FindAndUpdateProject: FindProjectById + UpdateProject {}
pub trait FindAndDeleteProject: FindProjectById + DeleteProject {}
pub trait FindAndCreateStage: FindStageById + CreateStage {}
pub trait FindAndUpdateStage: FindStageById + UpdateStage {}
pub trait FindAndCreateAllocation: FindAllocationById + CreateAllocation {}
pub trait FindAndUpdateAllocation: FindAllocationById + UpdateAllocation {}
