use async_trait::async_trait;

use crate::domain::errors::collaborator_error::CollaboratorError;
use crate::domain::models::db::collaborator_row::CollaboratorRow;

#[async_trait]
pub trait ListCollaborators: Send + Sync {
    async fn execute(&self) -> Result<Vec<CollaboratorRow>, CollaboratorError>;
}
