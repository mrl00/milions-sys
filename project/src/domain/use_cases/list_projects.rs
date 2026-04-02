use async_trait::async_trait;

use crate::domain::errors::ProjectError;
use crate::domain::models::db::project_rows::ProjectRow;

#[async_trait]
pub trait ListProjects: Send + Sync {
    async fn execute(&self) -> Result<Vec<ProjectRow>, ProjectError>;
}
