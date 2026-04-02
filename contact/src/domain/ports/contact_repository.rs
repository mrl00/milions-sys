use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::contact_error::ContactError;
use crate::domain::models::db::contact_row::{ContactRow, CreateContactRow};

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
pub trait CreateContact: Send + Sync {
    async fn create(&self, input: CreateContactRow) -> Result<ContactRow, ContactError>;
}

#[async_trait]
pub trait UpdateContactEmail: Send + Sync {
    async fn update_email(&self, uuid: Uuid, email: String) -> Result<ContactRow, ContactError>;
}

pub trait FindAndCreateContact: FindContactByEmail + CreateContact {}
pub trait FindAndUpdateContact: FindContactById + FindContactByEmail + UpdateContactEmail {}
