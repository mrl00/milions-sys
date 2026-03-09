#[derive(Debug, sqlx::FromRow)]
pub struct ClientProjectCollaborator {
    pub pk_allocated_collaborator: uuid::Uuid,
    pub idx_allocated_collaborator: i64,
    pub fk_project: uuid::Uuid,
    pub fk_collaborator: uuid::Uuid,
    pub ts_allocated_collaborator_created_at: sqlx::types::chrono::NaiveDateTime,
    pub ts_allocated_collaborator_updated_at: sqlx::types::chrono::NaiveDateTime,
}
