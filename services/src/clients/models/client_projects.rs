use std::fmt;

use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectStatus {
    InProgress,
    Stopped,
    Done,
    Active,
    Inactive,
}

impl fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProjectStatus::InProgress => write!(f, "in_progress"),
            ProjectStatus::Stopped => write!(f, "stopped"),
            ProjectStatus::Done => write!(f, "done"),
            ProjectStatus::Active => write!(f, "active"),
            ProjectStatus::Inactive => write!(f, "inactive"),
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct ClientProject {
    pub pk_project: uuid::Uuid,
    pub idx_project: i64,
    pub tx_name: String,
    pub tx_status: String,
    pub fk_address: uuid::Uuid,
    pub fk_client: uuid::Uuid,
    pub ts_project_created_at: NaiveDateTime,
    pub ts_project_updated_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct CreateClientProject {
    pub tx_name: String,
    pub tx_status: ProjectStatus,
    pub fk_address: uuid::Uuid,
    pub fk_client: uuid::Uuid,
}
