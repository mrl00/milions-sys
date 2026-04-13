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
pub trait RegisterCollaboratorUseCase: Send + Sync {
    async fn execute(
        &self,
        input: RegisterCollaboratorInput,
    ) -> Result<CollaboratorRow, CollaboratorError>;
}

#[async_trait]
pub trait FindCollaboratorUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<CollaboratorRow, CollaboratorError>;
}

#[allow(dead_code)]
#[async_trait]
pub trait FindCollaboratorByDocumentUseCase: Send + Sync {
    async fn execute(&self, cpf: &str) -> Result<Option<CollaboratorRow>, CollaboratorError>;
}

#[async_trait]
pub trait ListCollaboratorsUseCase: Send + Sync {
    async fn execute(&self) -> Result<Vec<CollaboratorRow>, CollaboratorError>;
}

#[async_trait]
pub trait UpdateCollaboratorUseCase: Send + Sync {
    async fn execute(
        &self,
        uuid: Uuid,
        input: UpdateCollaboratorInput,
    ) -> Result<CollaboratorRow, CollaboratorError>;
}

#[async_trait]
pub trait ActivateCollaboratorUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<CollaboratorRow, CollaboratorError>;
}

#[async_trait]
pub trait DeactivateCollaboratorUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<CollaboratorRow, CollaboratorError>;
}

#[async_trait]
pub trait DeleteCollaboratorUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<CollaboratorRow, CollaboratorError>;
}
