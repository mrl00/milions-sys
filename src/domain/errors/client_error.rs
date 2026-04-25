use crate::domain::errors::contact_error::ContactError;
use crate::domain::errors::infra_error::InfraError;
use crate::domain::errors::location_error::LocationError;
use crate::domain::value_objects::doc::DocError;
use uuid::Uuid;
use viacep::domain::ports::viacep_port::ViaCepError;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("not implemented")]
    NotImplemented,

    #[error("client '{name}' already exists")]
    AlreadyExists { name: String },

    #[error("client not found: {uuid}")]
    NotFound { uuid: Uuid },

    #[error("client is already active: {uuid}")]
    AlreadyActive { uuid: Uuid },

    #[error("client is already inactive: {uuid}")]
    AlreadyInactive { uuid: Uuid },

    #[error("document already registered")]
    DocumentAlreadyExists { doc: String },

    #[error(transparent)]
    InvalidDocument(#[from] DocError),

    #[error(transparent)]
    Location(#[from] LocationError),

    #[error(transparent)]
    Contact(#[from] ContactError),

    #[error(transparent)]
    ViaCep(#[from] ViaCepError),

    #[error(transparent)]
    Infra(#[from] InfraError),
}
