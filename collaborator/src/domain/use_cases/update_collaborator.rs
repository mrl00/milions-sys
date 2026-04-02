use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::collaborator_error::CollaboratorError;
use crate::domain::models::db::collaborator_row::CollaboratorRow;

pub struct UpdateCollaboratorInput {
    pub name: Option<String>,
    pub cpf: Option<String>,
    pub level: Option<String>,
}

#[async_trait]
pub trait UpdateCollaborator: Send + Sync {
    async fn execute(
        &self,
        uuid: Uuid,
        input: UpdateCollaboratorInput,
    ) -> Result<CollaboratorRow, CollaboratorError>;
}
