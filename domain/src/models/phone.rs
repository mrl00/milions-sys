use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct PhoneModel {
    pub pk_phone: Uuid,
    pub idx_phone: i64,
    pub tx_phone: String,
    pub fk_contact: Uuid,
    pub ts_phone_created_at: NaiveDateTime,
    pub ts_phone_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreatePhoneModel {
    pub tx_phone: String,
    pub fk_contact: Uuid,
}
