use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

// --- Contact ---

#[derive(Debug, Deserialize)]
pub struct RegisterContactRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateContactEmailRequest {
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct ContactResponse {
    pub id: Uuid,
    pub email: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<crate::domain::models::db::contact_row::ContactRow> for ContactResponse {
    fn from(row: crate::domain::models::db::contact_row::ContactRow) -> Self {
        Self {
            id: row.pk_contact,
            email: row.tx_email,
            created_at: row.ts_contact_created_at,
            updated_at: row.ts_contact_updated_at,
        }
    }
}

// --- Phone ---

#[derive(Debug, Deserialize)]
pub struct AddPhoneRequest {
    pub phone: String,
}

#[derive(Debug, Deserialize)]
pub struct AddPhonesRequest {
    pub phones: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePhoneRequest {
    pub phone: String,
}

#[derive(Debug, Serialize)]
pub struct PhoneResponse {
    pub id: Uuid,
    pub phone: String,
    pub contact_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<crate::domain::models::db::phone_row::PhoneRow> for PhoneResponse {
    fn from(row: crate::domain::models::db::phone_row::PhoneRow) -> Self {
        Self {
            id: row.pk_phone,
            phone: row.tx_phone,
            contact_id: row.fk_contact,
            created_at: row.ts_phone_created_at,
            updated_at: row.ts_phone_updated_at,
        }
    }
}
