use async_trait::async_trait;

use crate::domain::errors::ClientError;
use crate::domain::models::db::client_row::ClientRow;

pub struct RegisterClientInput {
    pub name: String,
    pub doc: String,
    pub email: String,
    pub phones: Vec<String>,
    pub cep: String,
    pub number: String,
    pub complement: String,
}

#[async_trait]
pub trait RegisterClient: Send + Sync {
    async fn execute(&self, input: RegisterClientInput) -> Result<ClientRow, ClientError>;
}
