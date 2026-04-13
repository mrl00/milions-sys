use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::client_errors::ClientError;
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
pub trait RegisterClientUseCase: Send + Sync {
    async fn execute(&self, input: RegisterClientInput) -> Result<ClientRow, ClientError>;
}

#[async_trait]
pub trait FindClientByIdUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError>;
}

#[async_trait]
pub trait FindClientByDocumentUseCase: Send + Sync {
    async fn execute(&self, doc: &str) -> Result<Option<ClientRow>, ClientError>;
}

#[async_trait]
pub trait ListClientsUseCase: Send + Sync {
    async fn execute(&self) -> Result<Vec<ClientRow>, ClientError>;
}

#[async_trait]
pub trait UpdateClientUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid, input: UpdateClientInput)
    -> Result<ClientRow, ClientError>;
}

#[async_trait]
pub trait ActivateClientUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError>;
}

#[async_trait]
pub trait DeactivateClientUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError>;
}

#[async_trait]
pub trait DeleteClientUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError>;
}
