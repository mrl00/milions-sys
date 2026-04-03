use types::{errors::infra_error::InfraError, phone::PhoneError};
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum ContactError {
    #[error("contact with email '{email}' already exists")]
    AlreadyExists { email: String },

    #[error("contact not found: {uuid}")]
    NotFound { uuid: Uuid },

    #[error("phone '{phone}' already exists")]
    PhoneAlreadyExists { phone: String },

    #[error("phone not found: {uuid}")]
    PhoneNotFound { uuid: Uuid },

    #[error(transparent)]
    InvalidPhone(#[from] PhoneError),

    #[error(transparent)]
    Infra { source: InfraError },
}
