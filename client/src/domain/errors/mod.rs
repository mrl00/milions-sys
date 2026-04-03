use types::cep::CepError;
use types::doc::DocError;
use types::email::EmailError;
use types::{errors::infra_error::InfraError, phone::PhoneError};
use uuid::Uuid;
use viacep::domain::ports::viacep_port::ViaCepError;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("client '{name}' already exists")]
    AlreadyExists { name: String },

    #[error("client not found: {uuid}")]
    NotFound { uuid: Uuid },

    #[error("client is already active: {uuid}")]
    AlreadyActive { uuid: Uuid },

    #[error("client is already inactive: {uuid}")]
    AlreadyInactive { uuid: Uuid },

    #[error("contact not found: {uuid}")]
    ContactNotFound { uuid: Uuid },

    #[error("address not found: {uuid}")]
    LocationNotFound { uuid: Uuid },

    #[error("document already registered")]
    DocumentAlreadyExists { doc: String },

    #[error("email already registered")]
    EmailAlreadyExists { email: String },

    #[error("phone '{phone}' already exists")]
    PhoneAlreadyExists { phone: String },

    #[error(transparent)]
    InvalidDoc(#[from] DocError),

    #[error(transparent)]
    InvalidEmail(#[from] EmailError),

    #[error(transparent)]
    InvalidPhone(#[from] PhoneError),

    #[error(transparent)]
    InvalidCep(#[from] CepError),

    #[error(transparent)]
    ViaCep(#[from] ViaCepError),

    #[error(transparent)]
    Infra(#[from] InfraError),
}
