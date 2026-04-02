use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::ClientError;
use crate::domain::models::db::client_row::ClientRow;

#[async_trait]
pub trait DeleteClient: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError>;
}
