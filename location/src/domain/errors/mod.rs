mod infra_error;

pub use infra_error::InfraError;

use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum LocationError {
    #[error("localização já existe: hash '{hash}'")]
    AlreadyExists { hash: i64 },

    #[error("localização não encontrada: {uuid}")]
    NotFound { uuid: Uuid },

    #[error("campo inválido: {field} — {reason}")]
    InvalidField { field: &'static str, reason: String },

    #[error(transparent)]
    Infra { source: InfraError },
}
