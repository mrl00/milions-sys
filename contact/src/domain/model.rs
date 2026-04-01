use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct ContactRow {
    pub pk_contact: Uuid,
    pub idx_contact: i64,
    pub tx_email: Option<String>,
    pub ts_contact_created_at: NaiveDateTime,
    pub ts_contact_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateContactRow {
    pub tx_email: String,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct PhoneRow {
    pub pk_phone: Uuid,
    pub idx_phone: i64,
    pub tx_phone: String,
    pub fk_contact: Uuid,
    pub ts_phone_created_at: NaiveDateTime,
    pub ts_phone_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreatePhoneRow {
    pub tx_phone: String,
    pub fk_contact: Uuid,
}
