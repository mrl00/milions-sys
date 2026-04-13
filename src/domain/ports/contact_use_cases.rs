use crate::domain::errors::contact_error::ContactError;
use crate::domain::models::db::contact_row::ContactRow;
use crate::domain::models::db::phone_row::PhoneRow;
use async_trait::async_trait;
use uuid::Uuid;
// --- Contact ---

pub struct RegisterContactInput {
    pub email: String,
}

#[async_trait]
pub trait RegisterContactUseCase: Send + Sync {
    async fn execute(&self, input: RegisterContactInput) -> Result<ContactRow, ContactError>;
}

#[async_trait]
pub trait FindContactUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ContactRow, ContactError>;
}

#[async_trait]
pub trait ListContactsUseCase: Send + Sync {
    async fn execute(&self) -> Result<Vec<ContactRow>, ContactError>;
}

#[async_trait]
pub trait UpdateContactEmailUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid, email: String) -> Result<ContactRow, ContactError>;
}

// --- Phone ---

#[async_trait]
pub trait FindPhoneUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<PhoneRow, ContactError>;
}

#[async_trait]
pub trait ListPhonesUseCase: Send + Sync {
    async fn execute(&self, contact_id: Uuid) -> Result<Vec<PhoneRow>, ContactError>;
}

#[async_trait]
pub trait AddPhoneUseCase: Send + Sync {
    async fn execute(&self, contact_id: Uuid, phone: String) -> Result<PhoneRow, ContactError>;
}

#[async_trait]
pub trait AddPhonesUseCase: Send + Sync {
    async fn execute(
        &self,
        contact_id: Uuid,
        phones: Vec<String>,
    ) -> Result<Vec<PhoneRow>, ContactError>;
}

#[async_trait]
pub trait UpdatePhoneUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid, phone: String) -> Result<PhoneRow, ContactError>;
}

#[async_trait]
pub trait RemovePhoneUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<PhoneRow, ContactError>;
}
