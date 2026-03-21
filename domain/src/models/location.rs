use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct LocationModel {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateLocationModel {
    pub tx_public_space: String,
    pub tx_address_complement: String,
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateLocationModel {
    pub tx_public_space: Option<String>,
    pub tx_address_complement: Option<String>,
    pub tx_unit: Option<String>,
    pub tx_neighborhood: Option<String>,
    pub tx_locality: Option<String>,
    pub tx_region: Option<String>,
    pub tx_ibge: Option<String>,
    pub tx_gia: Option<String>,
    pub tx_ddd: Option<String>,
    pub tx_siafi: Option<String>,
    pub tx_street: Option<String>,
    pub tx_number: Option<String>,
    pub tx_city: Option<String>,
    pub tx_state: Option<String>,
    pub tx_zipcode: Option<String>,
}
