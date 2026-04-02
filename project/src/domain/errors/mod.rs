use types::errors::infra_error::InfraError;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum ProjectError {
    #[error("projeto não encontrado: {uuid}")]
    NotFound { uuid: Uuid },

    #[error("projeto já está com status '{status}'")]
    AlreadyInStatus { uuid: Uuid, status: String },

    #[error("etapa não encontrada: {uuid}")]
    StageNotFound { uuid: Uuid },

    #[error("campo inválido: {field} — {reason}")]
    InvalidField { field: &'static str, reason: String },

    #[error(transparent)]
    Infra { source: InfraError },
}
