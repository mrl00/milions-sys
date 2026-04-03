use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::ClientError;
use crate::domain::models::db::client_row::ClientRow;

pub struct RegisterClientInput {
    pub name: String,
    pub doc: String,
    pub email: String,
    pub phones: Vec<String>,
    pub cep: String,
    pub street: String,
    pub number: String,
    pub complement: String,
    pub neighborhood: String,
    pub city: String,
    pub state: String,
}

pub struct UpdateClientInput {
    pub name: Option<String>,
    pub doc: Option<String>,
}

#[async_trait]
pub trait RegisterClient: Send + Sync {
    async fn execute(&self, input: RegisterClientInput) -> Result<ClientRow, ClientError>;
}

#[async_trait]
pub trait FindClientById: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError>;
}

#[async_trait]
pub trait FindClientByDocument: Send + Sync {
    async fn execute(&self, doc: &str) -> Result<Option<ClientRow>, ClientError>;
}

#[async_trait]
pub trait ListClients: Send + Sync {
    async fn execute(&self) -> Result<Vec<ClientRow>, ClientError>;
}

#[async_trait]
pub trait UpdateClient: Send + Sync {
    async fn execute(&self, uuid: Uuid, input: UpdateClientInput)
    -> Result<ClientRow, ClientError>;
}

#[async_trait]
pub trait ActivateClient: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError>;
}

#[async_trait]
pub trait DeactivateClient: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError>;
}

#[async_trait]
pub trait DeleteClient: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError>;
}
