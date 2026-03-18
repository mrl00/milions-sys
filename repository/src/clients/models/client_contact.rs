// =============================================================================
// CONTATO DO CLIENTE
// Associação N:N entre clientes e contatos (modelada como 1:1 via UNIQUE).
// =============================================================================
#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct ClientContact {
    pub pk_client_contact: uuid::Uuid,
    pub idx_client_contact: i64,
    pub fk_client: uuid::Uuid,
    pub fk_contact: uuid::Uuid,
    pub ts_client_contact_created_at: sqlx::types::chrono::NaiveDateTime,
    pub ts_client_contact_updated_at: sqlx::types::chrono::NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateClientContact {
    pub fk_client: uuid::Uuid,
    pub fk_contact: uuid::Uuid,
}
