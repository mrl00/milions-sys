use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Collaborator {
    pub pk_collaborator: Uuid,
    pub idx_collaborator: i64,
    pub tx_name: String,
    pub tx_level: String,
    pub ts_collaborator_created_at: NaiveDateTime,
    pub ts_collaborator_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateCollaborator {
    pub tx_name: String,
    pub tx_level: String,
}
