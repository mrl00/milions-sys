// =============================================================================
// ENDEREÇO DO CLIENTE
// Associação entre clientes e localizações físicas (um cliente pode ter vários).
// =============================================================================
#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct ClientAddressModel {
    pub pk_client_address: uuid::Uuid,
    pub idx_client_address: i32,
    pub fk_client: uuid::Uuid,
    /// FK para locations.tb_location
    pub fk_address: uuid::Uuid,
    pub ts_client_address_created_at: sqlx::types::chrono::NaiveDateTime,
    pub ts_client_address_updated_at: sqlx::types::chrono::NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateClientAddressModel {
    pub fk_client: uuid::Uuid,
    pub fk_address: uuid::Uuid,
}
