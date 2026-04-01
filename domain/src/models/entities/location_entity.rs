use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

use crate::models::db::location_row::LocationRow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocationEntity {
    pub pk_location: Uuid,
    pub idx_location: i64,
    pub tx_public_space: String, //logradouro
    pub tx_address_complement: Option<String>,
    pub tx_unit: String,
    pub tx_neighborhood: String,
    pub tx_locality: String,
    pub tx_region: String,
    pub tx_ibge: Option<String>,
    pub tx_gia: Option<String>,
    pub tx_ddd: String,
    pub tx_siafi: Option<String>,
    pub tx_street: String,
    pub tx_number: String,
    pub tx_city: String,
    pub tx_state: String,
    pub tx_zipcode: String,
    pub nr_hash: i64,
    pub ts_location_created_at: NaiveDateTime,
    pub ts_location_updated_at: NaiveDateTime,
}

impl From<LocationRow> for LocationEntity {
    fn from(row: LocationRow) -> Self {
        Self {
            pk_location: row.pk_location,
            idx_location: row.idx_location,
            tx_public_space: row.tx_public_space,
            tx_address_complement: row.tx_address_complement,
            tx_unit: row.tx_unit,
            tx_neighborhood: row.tx_neighborhood,
            tx_locality: row.tx_locality,
            tx_region: row.tx_region,
            tx_ibge: row.tx_ibge,
            tx_gia: row.tx_gia,
            tx_ddd: row.tx_ddd,
            tx_siafi: row.tx_siafi,
            tx_street: row.tx_street,
            tx_number: row.tx_number,
            tx_city: row.tx_city,
            tx_state: row.tx_state,
            tx_zipcode: row.tx_zipcode,
            nr_hash: row.nr_hash,
            ts_location_created_at: row.ts_location_created_at,
            ts_location_updated_at: row.ts_location_updated_at,
        }
    }
}
