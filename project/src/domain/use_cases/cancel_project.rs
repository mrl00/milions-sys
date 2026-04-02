use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::ProjectError;
use crate::domain::models::db::project_rows::ProjectRow;

#[async_trait]
pub trait CancelProject: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError>;
}
