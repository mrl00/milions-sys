use async_trait::async_trait;

use crate::domain::errors::contact_error::ContactError;
use crate::domain::models::db::contact_row::ContactRow;

pub struct RegisterContactInput {
    pub email: String,
}

#[async_trait]
pub trait RegisterContact: Send + Sync {
    async fn execute(&self, input: RegisterContactInput) -> Result<ContactRow, ContactError>;
}
