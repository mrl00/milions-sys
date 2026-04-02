use crate::domain::errors::collaborator_error::CollaboratorError;
use crate::domain::models::db::collaborator_row::{
    CollaboratorRow, CreateCollaboratorRow, UpdateCollaboratorRow,
};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait CollaboratorRepository: Send + Sync {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<CollaboratorRow>, CollaboratorError>;
    async fn find_by_cpf(&self, cpf: &str) -> Result<Option<CollaboratorRow>, CollaboratorError>;
    async fn find_all(&self) -> Result<Vec<CollaboratorRow>, CollaboratorError>;
    async fn create(
        &self,
        input: CreateCollaboratorRow,
    ) -> Result<CollaboratorRow, CollaboratorError>;
    async fn update(
        &self,
        uuid: Uuid,
        input: UpdateCollaboratorRow,
    ) -> Result<CollaboratorRow, CollaboratorError>;
    async fn delete(&self, uuid: Uuid) -> Result<CollaboratorRow, CollaboratorError>;
}
