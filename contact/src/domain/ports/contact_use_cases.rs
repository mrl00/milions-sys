use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::contact_error::ContactError;
use crate::domain::models::db::contact_row::ContactRow;
use crate::domain::models::db::phone_row::PhoneRow;

// --- Contact ---

pub struct RegisterContactInput {
    pub email: String,
}

#[async_trait]
pub trait RegisterContact: Send + Sync {
    async fn execute(&self, input: RegisterContactInput) -> Result<ContactRow, ContactError>;
}

#[async_trait]
pub trait FindContact: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ContactRow, ContactError>;
}

#[async_trait]
pub trait ListContacts: Send + Sync {
    async fn execute(&self) -> Result<Vec<ContactRow>, ContactError>;
}

#[async_trait]
pub trait UpdateContactEmail: Send + Sync {
    async fn execute(&self, uuid: Uuid, email: String) -> Result<ContactRow, ContactError>;
}

// --- Phone ---

#[async_trait]
pub trait FindPhone: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<PhoneRow, ContactError>;
}

#[async_trait]
pub trait ListPhones: Send + Sync {
    async fn execute(&self, contact_id: Uuid) -> Result<Vec<PhoneRow>, ContactError>;
}

#[async_trait]
pub trait AddPhone: Send + Sync {
    async fn execute(&self, contact_id: Uuid, phone: String) -> Result<PhoneRow, ContactError>;
}

#[async_trait]
pub trait AddPhones: Send + Sync {
    async fn execute(
        &self,
        contact_id: Uuid,
        phones: Vec<String>,
    ) -> Result<Vec<PhoneRow>, ContactError>;
}

#[async_trait]
pub trait UpdatePhone: Send + Sync {
    async fn execute(&self, uuid: Uuid, phone: String) -> Result<PhoneRow, ContactError>;
}

#[async_trait]
pub trait RemovePhone: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<PhoneRow, ContactError>;
}
