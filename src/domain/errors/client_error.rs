use crate::domain::errors::contact_error::ContactError;
use crate::domain::errors::infra_error::InfraError;
use crate::domain::errors::location_error::LocationError;
use crate::domain::value_objects::doc::DocError;
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

    #[error("document already registered")]
    DocumentAlreadyExists { doc: String },

    #[error("project {project_uuid} already associated with client {client_uuid}")]
    ProjectAlreadyAssociated {
        client_uuid: Uuid,
        project_uuid: Uuid,
    },

    #[error("project {project_uuid} not associated with client {client_uuid}")]
    ProjectNotAssociated {
        client_uuid: Uuid,
        project_uuid: Uuid,
    },

    #[error("client contact not found for client: {client_uuid}")]
    ContactNotFound { client_uuid: Uuid },

    #[error("client location not found for client: {client_uuid}")]
    LocationNotFound { client_uuid: Uuid },

    #[error("phone '{phone}' not found for contact: {contact_uuid}")]
    PhoneNotFound { phone: String, contact_uuid: Uuid },

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
