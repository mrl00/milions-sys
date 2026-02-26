use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Client {
    pub pk_client: Uuid,
    pub idx_client: i64,
    pub tx_name: String,
    pub tx_email: String,
    pub ts_client_created_at: NaiveDateTime,
    pub ts_client_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateClient {
    pub tx_name: String,
    pub tx_email: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateClient {
    pub tx_name: Option<String>,
    pub tx_email: Option<String>,
}
