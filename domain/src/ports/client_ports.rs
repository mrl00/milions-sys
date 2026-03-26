use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    errors::ClientError,
    models::client::{ClientModel, CreateClientModel, UpdateClientModel},
};

// domain/src/clients/ports.rs
#[async_trait]
pub trait ClientQuery: Send + Sync {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<ClientModel>, ClientError>;
    async fn find_by_document(&self, doc: &str) -> Result<Option<ClientModel>, ClientError>;
    async fn find_all(&self) -> Result<Vec<ClientModel>, ClientError>;
}

#[async_trait]
pub trait ClientMutation: Send + Sync {
    async fn create(&self, input: CreateClientModel) -> Result<ClientModel, ClientError>;

    async fn update(
        &self,
        uuid: Uuid,
        input: UpdateClientModel,
    ) -> Result<ClientModel, ClientError>;

    async fn delete(&self, uuid: Uuid) -> Result<ClientModel, ClientError>;

    // operações que precisam de transaction recebem a tx explicitamente
    async fn create_with_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        input: CreateClientModel,
    ) -> Result<ClientModel, ClientError>;
}
