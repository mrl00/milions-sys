use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::contact_error::ContactError;
use crate::domain::models::db::contact_row::ContactRow;

#[async_trait]
pub trait FindContact: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ContactRow, ContactError>;
}
