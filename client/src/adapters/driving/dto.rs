use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct RegisterClientRequest {
    pub name: String,
    pub document: String,
    pub contact: ContactDto,
    pub address: AddressDto,
}

#[derive(Debug, Deserialize)]
pub struct UpdateClientRequest {
    pub name: Option<String>,
    pub document: Option<String>,
    pub contact: Option<ContactDto>,
    pub address: Option<AddressDto>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContactDto {
    pub email: String,
    pub phones: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AddressDto {
    pub cep: String,
    pub street: String,
    pub number: String,
    pub complement: Option<String>,
    pub neighborhood: String,
    pub city: String,
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

#[derive(Debug, Deserialize)]
pub struct StatusRequest {
    pub status: String,
}
