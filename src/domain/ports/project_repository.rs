use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::errors::project_error::ProjectError;
use crate::domain::models::db::project_rows::{
    AllocationWithProjectName, CreateProjectDailyAllocationRow, CreateProjectRow,
    CreateProjectStageRow, ProjectDailyAllocationRow, ProjectRow, ProjectStageRow,
    UpdateProjectDailyAllocationRow, UpdateProjectRow, UpdateProjectStageRow,
};

#[async_trait]
pub trait FindProjectById: Send + Sync {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<ProjectRow>, ProjectError>;
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

#[async_trait]
pub trait FindStagesByProjectId: Send + Sync {
    async fn find_stages_by_project_id(
        &self,
        project_id: Uuid,
    ) -> Result<Vec<ProjectStageRow>, ProjectError>;
}

#[async_trait]
pub trait FindAllocationsByCollaboratorId: Send + Sync {
    async fn find_allocations_by_collaborator_id(
        &self,
        collaborator_id: Uuid,
    ) -> Result<Vec<AllocationWithProjectName>, ProjectError>;
}

pub trait ProjectRepository:
    FindProjectById
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
    + Sync
{
}
impl<T> ProjectRepository for T where
    T: FindProjectById
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
        + Sync
{
}
