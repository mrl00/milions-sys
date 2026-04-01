use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::ContactError;
use crate::domain::model::{ContactRow, CreateContactRow};

#[async_trait]
pub trait ContactRepository: Send + Sync {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<ContactRow>, ContactError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<ContactRow>, ContactError>;
    async fn find_all(&self) -> Result<Vec<ContactRow>, ContactError>;
    async fn create(&self, input: CreateContactRow) -> Result<ContactRow, ContactError>;
    async fn update_email(&self, uuid: Uuid, email: String) -> Result<ContactRow, ContactError>;
}
