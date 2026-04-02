use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::collaborator_error::CollaboratorError;
use crate::domain::models::db::collaborator_row::CollaboratorRow;

#[async_trait]
pub trait FindCollaborator: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<CollaboratorRow, CollaboratorError>;
}
