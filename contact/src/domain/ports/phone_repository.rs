use crate::domain::models::db::phone_row::PhoneRow;
use crate::domain::errors::contact_error::ContactError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait PhoneRepository: Send + Sync {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<PhoneRow>, ContactError>;
    async fn find_by_contact_id(&self, contact_id: Uuid) -> Result<Vec<PhoneRow>, ContactError>;
    async fn create(&self, contact_id: Uuid, phone: String) -> Result<PhoneRow, ContactError>;
    async fn create_many(
        &self,
        contact_id: Uuid,
        phones: Vec<String>,
    ) -> Result<Vec<PhoneRow>, ContactError>;
    async fn update(&self, uuid: Uuid, phone: String) -> Result<PhoneRow, ContactError>;
    async fn delete(&self, uuid: Uuid) -> Result<PhoneRow, ContactError>;
    async fn find_nonexistent_phones(
        &self,
        phones: Vec<String>,
    ) -> Result<Vec<String>, ContactError>;
}
