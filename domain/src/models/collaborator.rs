use std::fmt;

use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollaboratorLevel {
    P0 = 0,
    P1 = 1,
    P2 = 2,
    P3 = 3,
}

impl fmt::Display for CollaboratorLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollaboratorLevel::P0 => write!(f, "P0"),
            CollaboratorLevel::P1 => write!(f, "P1"),
            CollaboratorLevel::P2 => write!(f, "P2"),
            CollaboratorLevel::P3 => write!(f, "P3"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollaboratorStatus {
    Active,
    Inactive,
}

impl fmt::Display for CollaboratorStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CollaboratorStatus::Active => write!(f, "active"),
            CollaboratorStatus::Inactive => write!(f, "inactive"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct CollaboratorModel {
    pub pk_collaborator: Uuid,
    pub idx_collaborator: i64,
    pub tx_name: String,
    pub tx_cpf: String,
    pub tx_level: String,
    pub tx_status: String,
    pub ts_collaborator_created_at: NaiveDateTime,
    pub ts_collaborator_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateCollaboratorModel {
    pub tx_name: String,
    pub tx_cpf: String,
    pub tx_level: CollaboratorLevel,
    pub tx_status: CollaboratorStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateCollaboratorModel {
    pub tx_name: Option<String>,
    pub tx_level: Option<String>,
    pub tx_status: Option<String>,
    pub tx_cpf: Option<String>,
}
