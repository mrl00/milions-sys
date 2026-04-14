use crate::domain::errors::infra_error::InfraError;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum LocationError {
    #[error("location already exists: hash '{hash}'")]
    AlreadyExists { hash: i64 },

    #[error("location not found: {uuid}")]
    NotFound { uuid: Uuid },

    #[error("invalid field: {field} -- {reason}")]
    InvalidField { field: &'static str, reason: String },

    #[error(transparent)]
    Infra { source: InfraError },
}
