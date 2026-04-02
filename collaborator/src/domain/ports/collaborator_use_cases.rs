use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::collaborator_error::CollaboratorError;
use crate::domain::models::db::collaborator_row::CollaboratorRow;

pub struct RegisterCollaboratorInput {
    pub name: String,
    pub cpf: String,
}

pub struct UpdateCollaboratorInput {
    pub name: Option<String>,
    pub cpf: Option<String>,
    pub level: Option<String>,
}

#[async_trait]
pub trait RegisterCollaborator: Send + Sync {
    async fn execute(
        &self,
        input: RegisterCollaboratorInput,
    ) -> Result<CollaboratorRow, CollaboratorError>;
}

#[async_trait]
pub trait FindCollaborator: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<CollaboratorRow, CollaboratorError>;
}

#[async_trait]
pub trait FindCollaboratorByCpf: Send + Sync {
    async fn execute(&self, cpf: &str) -> Result<Option<CollaboratorRow>, CollaboratorError>;
}

#[async_trait]
pub trait ListCollaborators: Send + Sync {
    async fn execute(&self) -> Result<Vec<CollaboratorRow>, CollaboratorError>;
}

#[async_trait]
pub trait UpdateCollaborator: Send + Sync {
    async fn execute(
        &self,
        uuid: Uuid,
        input: UpdateCollaboratorInput,
    ) -> Result<CollaboratorRow, CollaboratorError>;
}

#[async_trait]
pub trait ActivateCollaborator: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<CollaboratorRow, CollaboratorError>;
}

#[async_trait]
pub trait DeactivateCollaborator: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<CollaboratorRow, CollaboratorError>;
}

#[async_trait]
pub trait DeleteCollaborator: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<CollaboratorRow, CollaboratorError>;
}
