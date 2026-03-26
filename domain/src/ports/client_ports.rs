use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    errors::ClientError,
    models::db::client::{ClientRow, CreateClientRow, UpdateClientRow},
};

// domain/src/clients/ports.rs
#[async_trait]
pub trait ClientQuery: Send + Sync {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<ClientRow>, ClientError>;
    async fn find_by_document(&self, doc: &str) -> Result<Option<ClientRow>, ClientError>;
    async fn find_all(&self) -> Result<Vec<ClientRow>, ClientError>;
}

#[async_trait]
pub trait ClientMutation: Send + Sync {
    async fn create(&self, input: CreateClientRow) -> Result<ClientRow, ClientError>;

    async fn update(&self, uuid: Uuid, input: UpdateClientRow) -> Result<ClientRow, ClientError>;

    async fn delete(&self, uuid: Uuid) -> Result<ClientRow, ClientError>;

    // operações que precisam de transaction recebem a tx explicitamente
    async fn create_with_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        input: CreateClientRow,
    ) -> Result<ClientRow, ClientError>;
}
