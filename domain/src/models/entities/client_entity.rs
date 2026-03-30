use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientEntity {
    pub pk_client: Uuid,
    pub tx_name: String,
    pub tx_status: String,
    pub tx_doc: String,
    pub ts_client_created_at: NaiveDateTime,
    pub ts_client_updated_at: NaiveDateTime,
}
