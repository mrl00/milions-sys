use async_trait::async_trait;

use crate::domain::errors::collaborator_error::CollaboratorError;
use crate::domain::models::db::collaborator_row::CollaboratorRow;

pub struct RegisterCollaboratorInput {
    pub name: String,
    pub cpf: String,
}

#[async_trait]
pub trait RegisterCollaborator: Send + Sync {
    async fn execute(
        &self,
        input: RegisterCollaboratorInput,
    ) -> Result<CollaboratorRow, CollaboratorError>;
}
