use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

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

#[derive(Debug, Deserialize)]
pub struct RegisterCollaboratorRequest {
    pub name: String,
    pub cpf: String,
    pub level: String,
    pub contact: ContactDto,
    pub address: AddressDto,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCollaboratorRequest {
    pub name: Option<String>,
    pub cpf: Option<String>,
    pub level: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CollaboratorResponse {
    pub id: Uuid,
    pub name: String,
    pub cpf: String,
    pub level: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<crate::domain::models::db::collaborator_row::CollaboratorRow> for CollaboratorResponse {
    fn from(row: crate::domain::models::db::collaborator_row::CollaboratorRow) -> Self {
        Self {
            id: row.pk_collaborator,
            name: row.tx_name,
            cpf: row.tx_cpf,
            level: row.tx_level,
            status: row.tx_status,
            created_at: row.ts_collaborator_created_at,
            updated_at: row.ts_collaborator_updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct StatusRequest {
    pub status: String,
}
