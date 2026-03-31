use crate::errors::infra_error::InfraError;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum ContactError {
    #[error("contato com email '{email}' já existe")]
    AlreadyExists { email: String },

    #[error("contato não encontrado: {uuid}")]
    NotFound { uuid: Uuid },

    #[error("telefone '{phone}' já existe")]
    PhoneAlreadyExists { phone: String },

    #[error("telefone não encontrado: {uuid}")]
    PhoneNotFound { uuid: Uuid },

    #[error(transparent)]
    Infra { source: InfraError },
}
