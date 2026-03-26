use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct CollaboratorAddressRow {
    pub pk_collaborator_address: Uuid,
    pub idx_collaborator_address: i64,
    pub fk_collaborator: Uuid,
    pub fk_address: Uuid,
    pub ts_collaborator_address_created_at: sqlx::types::chrono::NaiveDateTime,
    pub ts_collaborator_address_updated_at: sqlx::types::chrono::NaiveDateTime,
}
