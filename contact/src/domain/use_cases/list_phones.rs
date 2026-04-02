use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::contact_error::ContactError;
use crate::domain::models::db::phone_row::PhoneRow;

#[async_trait]
pub trait ListPhones: Send + Sync {
    async fn execute(&self, contact_id: Uuid) -> Result<Vec<PhoneRow>, ContactError>;
}
