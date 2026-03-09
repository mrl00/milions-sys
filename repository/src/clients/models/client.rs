use std::fmt;

use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientStatus {
    Active,
    Inactive,
}

impl fmt::Display for ClientStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientStatus::Active => write!(f, "active"),
            ClientStatus::Inactive => write!(f, "inactive"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Client {
    pub pk_client: Uuid,
    pub idx_client: i64,
    pub tx_name: String,
    pub tx_status: String,
    pub ts_client_created_at: NaiveDateTime,
    pub ts_client_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateClient {
    pub tx_name: String,
    pub tx_email: String,
    pub tx_status: ClientStatus,
}
