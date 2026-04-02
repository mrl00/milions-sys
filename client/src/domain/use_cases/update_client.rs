use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::ClientError;
use crate::domain::models::db::client_row::ClientRow;

pub struct UpdateClientInput {
    pub name: Option<String>,
    pub doc: Option<String>,
}

#[async_trait]
pub trait UpdateClient: Send + Sync {
    async fn execute(&self, uuid: Uuid, input: UpdateClientInput) -> Result<ClientRow, ClientError>;
}
