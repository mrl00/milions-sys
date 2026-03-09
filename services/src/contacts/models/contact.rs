use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Contact {
    pub pk_contact: Uuid,
    pub idx_contact: i64,
    pub tx_email: Option<String>,
    pub ts_contact_created_at: NaiveDateTime,
    pub ts_contact_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateContact {
    pub tx_email: String,
}
