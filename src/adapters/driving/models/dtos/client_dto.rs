use garde::Validate;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterClientRequest {
    #[garde(length(min = 1, max = 64))]
    pub name: String,
    #[garde(pattern(r"^\d{11}$|^\d{14}$"))]
    pub document: String,
    #[garde(dive)]
    pub contact: Option<ContactDto>,
    #[garde(dive)]
    pub address: Option<AddressDto>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateClientRequest {
    #[garde(inner(length(min = 1, max = 64)))]
    pub name: Option<String>,
    #[garde(inner(pattern(r"^\d{11}$|^\d{14}$")))]
    pub document: Option<String>,
    #[garde(skip)]
    pub contact: Option<ContactDto>,
    #[garde(skip)]
    pub address: Option<AddressDto>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct ContactDto {
    #[garde(email, length(max = 256))]
    pub email: String,
    #[garde(dive, length(min = 1))]
    pub phones: Vec<PhoneEntry>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct PhoneEntry {
    #[garde(pattern(r"^\+\d{8,16}$"))]
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct AddressDto {
    #[garde(pattern(r"^\d{8}$"))]
    pub cep: String,
    #[garde(length(min = 1, max = 128))]
    pub street: String,
    #[garde(length(min = 1, max = 16))]
    pub number: String,
    #[garde(inner(length(max = 64)))]
    pub complement: Option<String>,
    #[garde(length(min = 1, max = 64))]
    pub neighborhood: String,
    #[garde(length(min = 1, max = 64))]
    pub city: String,
    #[garde(length(min = 2, max = 2), pattern(r"^[A-Z]{2}$"))]
    pub state: String,
}

#[derive(Debug, Serialize)]
pub struct ClientResponse {
    pub id: Uuid,
    pub name: String,
    pub status: String,
    pub document: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<crate::domain::models::db::client_row::ClientRow> for ClientResponse {
    fn from(row: crate::domain::models::db::client_row::ClientRow) -> Self {
        Self {
            id: row.pk_client,
            name: row.tx_name,
            status: row.tx_status,
            document: row.tx_doc,
            created_at: row.ts_client_created_at,
            updated_at: row.ts_client_updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct ClientStatusRequest {
    #[garde(pattern(r"^(active|inactive)$"))]
    pub status: String,
}

impl Default for ClientStatusRequest {
    fn default() -> Self {
        Self {
            status: "active".to_string(),
        }
    }
}
