use async_trait::async_trait;

use crate::domain::errors::ClientError;
use crate::domain::models::db::client_row::ClientRow;

#[async_trait]
pub trait ListClients: Send + Sync {
    async fn execute(&self) -> Result<Vec<ClientRow>, ClientError>;
}
