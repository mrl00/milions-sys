use garde::Validate;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateLocationRequest {
    #[garde(length(min = 1, max = 128))]
    pub street: String,

    #[garde(length(min = 1, max = 128))]
    pub number: String,

    #[garde(length(min = 1, max = 128))]
    pub city: String,

    #[garde(length(min = 2, max = 2))]
    pub state: String,

    #[garde(pattern(r"^[0-9]{5}-?[0-9]{3}$"))]
    pub zipcode: String,

    #[garde(length(min = 0, max = 256))]
    pub complement: String,

    #[garde(length(min = 1, max = 128))]
    pub public_space: String,

    #[garde(length(min = 1, max = 64))]
    pub unit: String,

    #[garde(length(min = 1, max = 128))]
    pub neighborhood: String,

    #[garde(length(min = 1, max = 128))]
    pub locality: String,

    #[garde(length(min = 1, max = 64))]
    pub region: String,

    #[garde(skip)]
    pub ibge: Option<String>,

    #[garde(skip)]
    pub gia: Option<String>,

    #[garde(length(min = 1, max = 3))]
    pub ddd: String,

    #[garde(skip)]
    pub siafi: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateLocationRequest {
    #[garde(inner(length(min = 1, max = 128)))]
    pub street: Option<String>,

    #[garde(inner(length(min = 1, max = 128)))]
    pub number: Option<String>,

    #[garde(inner(length(min = 1, max = 128)))]
    pub city: Option<String>,

    #[garde(inner(length(min = 2, max = 2)))]
    pub state: Option<String>,

    #[garde(inner(pattern(r"^[0-9]{5}-?[0-9]{3}$")))]
    pub zipcode: Option<String>,

    #[garde(inner(length(min = 0, max = 256)))]
    pub complement: Option<String>,

    #[garde(inner(length(min = 1, max = 128)))]
    pub public_space: Option<String>,

    #[garde(inner(length(min = 1, max = 64)))]
    pub unit: Option<String>,

    #[garde(inner(length(min = 1, max = 128)))]
    pub neighborhood: Option<String>,

    #[garde(inner(length(min = 1, max = 128)))]
    pub locality: Option<String>,

    #[garde(inner(length(min = 1, max = 64)))]
    pub region: Option<String>,

    #[garde(skip)]
    pub ibge: Option<String>,

    #[garde(skip)]
    pub gia: Option<String>,

    #[garde(inner(length(min = 1, max = 3)))]
    pub ddd: Option<String>,

    #[garde(skip)]
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
