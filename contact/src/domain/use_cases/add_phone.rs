use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::contact_error::ContactError;
use crate::domain::models::db::phone_row::PhoneRow;

#[async_trait]
pub trait AddPhone: Send + Sync {
    async fn execute(&self, contact_id: Uuid, phone: String) -> Result<PhoneRow, ContactError>;
}
