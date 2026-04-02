use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::contact_error::ContactError;
use crate::domain::models::db::phone_row::PhoneRow;

#[async_trait]
pub trait FindPhoneById: Send + Sync {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<PhoneRow>, ContactError>;
}

#[async_trait]
pub trait FindPhoneByContactId: Send + Sync {
    async fn find_by_contact_id(&self, contact_id: Uuid) -> Result<Vec<PhoneRow>, ContactError>;
}

#[async_trait]
pub trait CreatePhone: Send + Sync {
    async fn create(&self, contact_id: Uuid, phone: String) -> Result<PhoneRow, ContactError>;
}

#[async_trait]
pub trait CreateManyPhones: Send + Sync {
    async fn create_many(
        &self,
        contact_id: Uuid,
        phones: Vec<String>,
    ) -> Result<Vec<PhoneRow>, ContactError>;
}

#[async_trait]
pub trait UpdatePhone: Send + Sync {
    async fn update(&self, uuid: Uuid, phone: String) -> Result<PhoneRow, ContactError>;
}

#[async_trait]
pub trait DeletePhone: Send + Sync {
    async fn delete(&self, uuid: Uuid) -> Result<PhoneRow, ContactError>;
}

#[async_trait]
pub trait FindNonexistentPhones: Send + Sync {
    async fn find_nonexistent_phones(
        &self,
        phones: Vec<String>,
    ) -> Result<Vec<String>, ContactError>;
}

pub trait FindAndCreatePhone: FindPhoneByContactId + CreatePhone {}
pub trait FindAndUpdatePhone: FindPhoneById + UpdatePhone {}
pub trait FindAndDeletePhone: FindPhoneById + DeletePhone {}
