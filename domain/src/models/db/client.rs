use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

// =============================================================================
// CLIENTE
// Pessoa física ou jurídica contratante da obra.
// =============================================================================
#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct ClientRow {
    /// Identificador único do cliente (UUID gerado pela aplicação)
    pub pk_client: Uuid,
    /// Índice sequencial auto-incrementado
    pub idx_client: i32,
    /// Nome do cliente
    pub tx_name: String,
    /// Status do cliente (ex: Active, Inactive)
    pub tx_status: String,
    pub tx_doc: String,
    pub ts_client_created_at: NaiveDateTime,
    pub ts_client_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateClientRow {
    pub tx_name: String,
    pub tx_status: ClientStatus,
    pub tx_doc: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateClientRow {
    pub tx_name: Option<String>,
    pub tx_status: Option<ClientStatus>,
    pub tx_doc: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientStatus {
    Active,
    Inactive,
}

impl std::fmt::Display for ClientStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientStatus::Active => write!(f, "active"),
            ClientStatus::Inactive => write!(f, "inactive"),
        }
    }
}
