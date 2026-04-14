use crate::domain::errors::infra_error::InfraError;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum ProjectError {
    #[error("project not found: {uuid}")]
    NotFound { uuid: Uuid },

    #[error("project is already in status '{status}'")]
    AlreadyInStatus { uuid: Uuid, status: String },

    #[error("stage not found: {uuid}")]
    StageNotFound { uuid: Uuid },

    #[error("allocation not found: {uuid}")]
    AllocationNotFound { uuid: Uuid },

    #[error("collaborator not found: {uuid}")]
    CollaboratorNotFound { uuid: Uuid },

    #[error("invalid field: {field} -- {reason}")]
    InvalidField { field: &'static str, reason: String },

    #[error(transparent)]
    Infra(#[from] InfraError),
}
