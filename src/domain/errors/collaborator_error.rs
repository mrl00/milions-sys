use crate::domain::errors::infra_error::InfraError;
use crate::domain::value_objects::cpf::CpfError;
use crate::domain::value_objects::phone::PhoneError;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum CollaboratorError {
    #[error("collaborator not found: {uuid}")]
    NotFound { uuid: Uuid },

    #[error("CPF '{cpf}' already registered")]
    CpfAlreadyExists { cpf: String },

    #[error("collaborator is already active: {uuid}")]
    AlreadyActive { uuid: Uuid },

    #[error("collaborator is already inactive: {uuid}")]
    AlreadyInactive { uuid: Uuid },

    #[error(transparent)]
    InvalidCpf(#[from] CpfError),

    #[error(transparent)]
    InvalidPhone(#[from] PhoneError),

    #[error(transparent)]
    Infra { source: InfraError },
}
