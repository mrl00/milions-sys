use crate::domain::errors::InfraError;
use types::cpf::CpfError;
use types::phone::PhoneError;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum CollaboratorError {
    #[error("colaborador não encontrado: {uuid}")]
    NotFound { uuid: Uuid },

    #[error("CPF '{cpf}' já cadastrado")]
    CpfAlreadyExists { cpf: String },

    #[error("colaborador já está ativo: {uuid}")]
    AlreadyActive { uuid: Uuid },

    #[error("colaborador já está inativo: {uuid}")]
    AlreadyInactive { uuid: Uuid },

    #[error(transparent)]
    InvalidCpf(#[from] CpfError),

    #[error(transparent)]
    InvalidPhone(#[from] PhoneError),

    #[error(transparent)]
    Infra { source: InfraError },
}
