use async_trait::async_trait;

use crate::domain::errors::ClientError;
use crate::domain::models::db::client_row::ClientRow;

#[async_trait]
pub trait FindClientByDocument: Send + Sync {
    async fn execute(&self, doc: &str) -> Result<Option<ClientRow>, ClientError>;
}
