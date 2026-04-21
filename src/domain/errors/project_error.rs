use crate::domain::errors::infra_error::InfraError;
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum ProjectError {
    #[error("project not found: {uuid}")]
    NotFound { uuid: Uuid },

    #[error("project is already in status '{status}'")]
    AlreadyInStatus { uuid: Uuid, status: String },

    #[error("invalid status transition: '{from}' → '{to}'")]
    InvalidTransition { from: String, to: String },

    #[error("stage not found: {uuid}")]
    StageNotFound { uuid: Uuid },

    #[error("allocation not found: {uuid}")]
    AllocationNotFound { uuid: Uuid },

    #[error(
        "collaborator ({collaborator_id}) is already allocated at ({project_id}) on {work_date}"
    )]
    AllocationConflict {
        project_id: Uuid,
        collaborator_id: Uuid,
        work_date: NaiveDate,
    },

    #[error("collaborator not found: {uuid}")]
    CollaboratorNotFound { uuid: Uuid },

    #[error("invalid field '{field}': {reason}")]
    InvalidField { field: &'static str, reason: String },

    #[error(transparent)]
    Infra(#[from] InfraError),
}
