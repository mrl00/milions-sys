#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct ClientAddress {
    pub pk_client_address: uuid::Uuid,
    pub idx_client_address: i64,
    pub fk_client: uuid::Uuid,
    pub fk_address: uuid::Uuid,
    pub ts_client_address_created_at: sqlx::types::chrono::NaiveDateTime,
    pub ts_client_address_updated_at: sqlx::types::chrono::NaiveDateTime,
}
