use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::collaborator_error::CollaboratorError;
use crate::domain::models::db::collaborator_row::{
    CollaboratorRow, CreateCollaboratorRow, UpdateCollaboratorRow,
};

#[async_trait]
pub trait FindCollaboratorById: Send + Sync {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<CollaboratorRow>, CollaboratorError>;
}

#[async_trait]
pub trait FindCollaboratorByDocument: Send + Sync {
    async fn find_by_document(
        &self,
        doc: &str,
    ) -> Result<Option<CollaboratorRow>, CollaboratorError>;
}

#[async_trait]
pub trait FindAllCollaborators: Send + Sync {
    async fn find_all(&self) -> Result<Vec<CollaboratorRow>, CollaboratorError>;
}

#[async_trait]
pub trait CreateCollaborator: Send + Sync {
    async fn create(
        &self,
        input: CreateCollaboratorRow,
    ) -> Result<CollaboratorRow, CollaboratorError>;
}

#[async_trait]
pub trait UpdateCollaborator: Send + Sync {
    async fn update(
        &self,
        uuid: Uuid,
        input: UpdateCollaboratorRow,
    ) -> Result<CollaboratorRow, CollaboratorError>;
}

#[async_trait]
pub trait DeleteCollaborator: Send + Sync {
    async fn delete(&self, uuid: Uuid) -> Result<CollaboratorRow, CollaboratorError>;
}

pub trait CollaboratorRepository:
    FindCollaboratorById
    + FindCollaboratorByDocument
    + FindAllCollaborators
    + CreateCollaborator
    + UpdateCollaborator
    + DeleteCollaborator
    + Send
    + Sync
{
}
impl<T> CollaboratorRepository for T where
    T: FindCollaboratorById
        + FindCollaboratorByDocument
        + FindAllCollaborators
        + CreateCollaborator
        + UpdateCollaborator
        + DeleteCollaborator
        + Send
        + Sync
{
}
