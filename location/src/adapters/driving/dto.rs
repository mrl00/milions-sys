use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateLocationRequest {
    pub street: String,
    pub number: String,
    pub city: String,
    pub state: String,
    pub zipcode: String,
    pub complement: String,
    pub public_space: String,
    pub unit: String,
    pub neighborhood: String,
    pub locality: String,
    pub region: String,
    pub ibge: Option<String>,
    pub gia: Option<String>,
    pub ddd: String,
    pub siafi: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLocationRequest {
    pub street: Option<String>,
    pub number: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zipcode: Option<String>,
    pub complement: Option<String>,
    pub public_space: Option<String>,
    pub unit: Option<String>,
    pub neighborhood: Option<String>,
    pub locality: Option<String>,
    pub region: Option<String>,
    pub ibge: Option<String>,
    pub gia: Option<String>,
    pub ddd: Option<String>,
    pub siafi: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LocationResponse {
    pub id: Uuid,
    pub public_space: String,
    pub address_complement: Option<String>,
    pub unit: String,
    pub neighborhood: String,
    pub locality: String,
    pub region: String,
    pub ibge: Option<String>,
    pub gia: Option<String>,
    pub ddd: String,
    pub siafi: Option<String>,
    pub street: String,
    pub number: String,
    pub city: String,
    pub state: String,
    pub zipcode: String,
    pub hash: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<crate::domain::models::db::location_row::LocationRow> for LocationResponse {
    fn from(row: crate::domain::models::db::location_row::LocationRow) -> Self {
        Self {
            id: row.pk_location,
            public_space: row.tx_public_space,
            address_complement: row.tx_address_complement,
            unit: row.tx_unit,
            neighborhood: row.tx_neighborhood,
            locality: row.tx_locality,
            region: row.tx_region,
            ibge: row.tx_ibge,
            gia: row.tx_gia,
            ddd: row.tx_ddd,
            siafi: row.tx_siafi,
            street: row.tx_street,
            number: row.tx_number,
            city: row.tx_city,
            state: row.tx_state,
            zipcode: row.tx_zipcode,
            hash: row.nr_hash,
            created_at: row.ts_location_created_at,
            updated_at: row.ts_location_updated_at,
        }
    }
}
