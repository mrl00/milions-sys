use async_trait::async_trait;

use crate::domain::errors::contact_error::ContactError;
use crate::domain::models::db::contact_row::ContactRow;

#[async_trait]
pub trait ListContacts: Send + Sync {
    async fn execute(&self) -> Result<Vec<ContactRow>, ContactError>;
}
