use crate::domain::errors::contact_error::ContactError;
use crate::domain::models::db::contact_row::{ContactRow, CreateContactRow};
use crate::domain::models::db::phone_row::PhoneRow;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait FindContactById: Send + Sync {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<ContactRow>, ContactError>;
}

#[async_trait]
pub trait FindContactByEmail: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<ContactRow>, ContactError>;
}

#[async_trait]
pub trait FindAllContacts: Send + Sync {
    async fn find_all(&self) -> Result<Vec<ContactRow>, ContactError>;
}

#[async_trait]
pub trait FindAllContactPhones: Send + Sync {
    async fn find_all_phones(&self, contact_id: Uuid) -> Result<Vec<PhoneRow>, ContactError>;
}

#[async_trait]
pub trait CreateContact: Send + Sync {
    async fn create(&self, input: CreateContactRow) -> Result<ContactRow, ContactError>;
}

#[async_trait]
pub trait UpdateContactEmail: Send + Sync {
    async fn update_email(&self, uuid: Uuid, email: String) -> Result<ContactRow, ContactError>;
}

pub trait ContactRepository:
    FindContactById
    + FindContactByEmail
    + FindAllContacts
    + FindAllContactPhones
    + CreateContact
    + UpdateContactEmail
    + Send
    + Sync
{
}
impl<T> ContactRepository for T where
    T: FindContactById
        + FindContactByEmail
        + FindAllContacts
        + FindAllContactPhones
        + CreateContact
        + UpdateContactEmail
        + Send
        + Sync
{
}
