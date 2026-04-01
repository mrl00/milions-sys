use crate::domain::errors::ClientError;
use crate::domain::models::db::client_row::{ClientRow, CreateClientRow, UpdateClientRow};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait ClientRepository: Send + Sync {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<ClientRow>, ClientError>;
    async fn find_by_document(&self, doc: &str) -> Result<Option<ClientRow>, ClientError>;
    async fn find_all(&self) -> Result<Vec<ClientRow>, ClientError>;
    async fn create(&self, input: CreateClientRow) -> Result<ClientRow, ClientError>;
    async fn update(&self, uuid: Uuid, input: UpdateClientRow) -> Result<ClientRow, ClientError>;
    async fn delete(&self, uuid: Uuid) -> Result<ClientRow, ClientError>;
    async fn create_with_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        input: CreateClientRow,
    ) -> Result<ClientRow, ClientError>;
}
