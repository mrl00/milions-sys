use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::client_error::ClientError;
use crate::domain::models::db::client_project_row::{ClientProjectRow, CreateClientProjectRow};
use crate::domain::models::db::client_row::{ClientRow, CreateClientRow, UpdateClientRow};

#[async_trait]
pub trait FindById: Send + Sync {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<ClientRow>, ClientError>;
}

#[async_trait]
pub trait FindByDocument: Send + Sync {
    async fn find_by_document(&self, doc: &str) -> Result<Option<ClientRow>, ClientError>;
}

#[async_trait]
pub trait FindAll: Send + Sync {
    async fn find_all(&self) -> Result<Vec<ClientRow>, ClientError>;
}

#[async_trait]
pub trait CreateClient: Send + Sync {
    async fn create(&self, input: CreateClientRow) -> Result<ClientRow, ClientError>;
}

#[async_trait]
pub trait UpdateClient: Send + Sync {
    async fn update(&self, uuid: Uuid, input: UpdateClientRow) -> Result<ClientRow, ClientError>;
}

#[async_trait]
pub trait DeleteClient: Send + Sync {
    async fn delete(&self, uuid: Uuid) -> Result<ClientRow, ClientError>;
}

#[async_trait]
pub trait CreateClientWithTx: Send + Sync {
    async fn create_with_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        input: CreateClientRow,
    ) -> Result<ClientRow, ClientError>;
}

#[async_trait]
pub trait CreateClientProject: Send + Sync {
    async fn create_client_project(
        &self,
        input: CreateClientProjectRow,
    ) -> Result<ClientProjectRow, ClientError>;
}

#[async_trait]
pub trait FindProjectsByClientId: Send + Sync {
    async fn find_projects_by_client_id(
        &self,
        client_id: Uuid,
    ) -> Result<Vec<ClientProjectRow>, ClientError>;
}

pub trait ClientRepository:
    FindById + FindByDocument + FindAll + CreateClient + UpdateClient + DeleteClient + Send + Sync
{
}
impl<T> ClientRepository for T where
    T: FindById
        + FindByDocument
        + FindAll
        + CreateClient
        + UpdateClient
        + DeleteClient
        + Send
        + Sync
{
}
