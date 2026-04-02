use async_trait::async_trait;

use crate::domain::errors::collaborator_error::CollaboratorError;
use crate::domain::models::db::collaborator_row::CollaboratorRow;

#[async_trait]
pub trait FindCollaboratorByCpf: Send + Sync {
    async fn execute(&self, cpf: &str) -> Result<Option<CollaboratorRow>, CollaboratorError>;
}
