use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::ProjectError;
use crate::domain::models::db::project_rows::ProjectRow;

#[async_trait]
pub trait ListProjectsByClient: Send + Sync {
    async fn execute(&self, client_id: Uuid) -> Result<Vec<ProjectRow>, ProjectError>;
}
