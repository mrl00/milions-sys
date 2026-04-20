use garde::Validate;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct ContactDto {
    #[garde(email, length(max = 256))]
    pub email: String,
    #[garde(dive, length(min = 1))]
    pub phones: Vec<PhoneEntry>,
}

/// Wrapper necessário para aplicar `garde` a cada item de `phones`.
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
    #[garde(length(max = 64), inner(length(max = 64)))]
    pub complement: Option<String>,
    #[garde(length(min = 1, max = 64))]
    pub neighborhood: String,
    #[garde(length(min = 1, max = 64))]
    pub city: String,
    /// Sigla do estado (ex: DF, SP)
    #[garde(length(min = 2, max = 2), pattern(r"^[A-Z]{2}$"))]
    pub state: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterCollaboratorRequest {
    #[garde(length(min = 1, max = 64))]
    pub name: String,
    /// CPF sem formatação, apenas dígitos (ex: 00000000000)
    #[garde(pattern(r"^\d{11}$"))]
    pub cpf: String,
    #[garde(pattern(r"^(painter|helper|supervisor|generalist)$"))]
    pub level: String,
    #[garde(dive)]
    pub contact: ContactDto,
    #[garde(dive)]
    pub address: AddressDto,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCollaboratorRequest {
    #[garde(length(min = 1, max = 64), inner(length(min = 1, max = 64)))]
    pub name: Option<String>,
    #[garde(pattern(r"^\d{11}$"), inner(pattern(r"^\d{11}$")))]
    pub cpf: Option<String>,
    #[garde(
        pattern(r"^(painter|helper|supervisor|generalist)$"),
        inner(pattern(r"^(painter|helper|supervisor|generalist)$"))
    )]
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

#[derive(Debug, Deserialize, Validate)]
pub struct StatusRequest {
    #[garde(pattern(r"^(active|inactive)$"))]
    pub status: String,
}
