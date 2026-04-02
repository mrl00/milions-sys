use crate::domain::errors::ProjectError;
use crate::domain::models::db::project_rows::{CreateProjectRow, ProjectRow, UpdateProjectRow};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait ProjectRepository: Send + Sync {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<ProjectRow>, ProjectError>;
    async fn find_by_client_id(&self, client_id: Uuid) -> Result<Vec<ProjectRow>, ProjectError>;
    async fn find_all(&self) -> Result<Vec<ProjectRow>, ProjectError>;
    async fn create(&self, input: CreateProjectRow) -> Result<ProjectRow, ProjectError>;
    async fn update(&self, uuid: Uuid, input: UpdateProjectRow)
    -> Result<ProjectRow, ProjectError>;
    async fn delete(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError>;
}
