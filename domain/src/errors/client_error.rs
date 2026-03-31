use crate::errors::infra_error::InfraError;
use sqlx::types::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("cliente '{name}' já existe")]
    AlreadyExists { name: String },

    #[error("cliente não encontrado: {uuid}")]
    NotFound { uuid: Uuid },

    #[error("contato não encontrado: {uuid}")]
    ContactNotFound { uuid: Uuid },

    #[error("endereço não encontrado: {uuid}")]
    LocationNotFound { uuid: Uuid },

    #[error(transparent)]
    Infra { source: InfraError },
}
