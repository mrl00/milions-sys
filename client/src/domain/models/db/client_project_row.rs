use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct ClientProjectRow {
    pub pk_client_project: Uuid,
    pub idx_client_project: i32,
    pub fk_client: Uuid,
    pub fk_project: Uuid,
    pub ts_client_project_created_at: NaiveDateTime,
    pub ts_client_project_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateClientProjectRow {
    pub fk_client: Uuid,
    pub fk_project: Uuid,
}
