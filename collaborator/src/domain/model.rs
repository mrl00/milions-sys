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
pub struct CollaboratorRow {
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
pub struct CreateCollaboratorRow {
    pub tx_name: String,
    pub tx_cpf: String,
    pub tx_level: CollaboratorLevel,
    pub tx_status: CollaboratorStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateCollaboratorRow {
    pub tx_name: Option<String>,
    pub tx_level: Option<String>,
    pub tx_status: Option<String>,
    pub tx_cpf: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct CollaboratorContactRow {
    pub pk_collaborator_contact: Uuid,
    pub idx_collaborator_contact: i64,
    pub fk_collaborator: Uuid,
    pub fk_contact: Uuid,
    pub ts_collaborator_contact_created_at: NaiveDateTime,
    pub ts_collaborator_contact_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct CollaboratorAddressRow {
    pub pk_collaborator_address: Uuid,
    pub idx_collaborator_address: i64,
    pub fk_collaborator: Uuid,
    pub fk_address: Uuid,
    pub ts_collaborator_address_created_at: NaiveDateTime,
    pub ts_collaborator_address_updated_at: NaiveDateTime,
}
