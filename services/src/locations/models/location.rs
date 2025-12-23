use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

use crate::contacts::models::{address::Address, phone::Phone};

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Location {
    pub pk_location: Uuid,
    pub idx_location: i64,
    pub tx_street: String,
    pub tx_number: String,
    pub tx_city: String,
    pub tx_state: String,
    pub tx_zipcode: String,
    pub ts_location_created_at: NaiveDateTime,
    pub ts_location_updated_at: NaiveDateTime,
}

pub struct LocationResult {
    pub pk_location: Uuid,
    pub idx_location: i64,
    pub tx_street: String,
    pub tx_number: String,
    pub tx_city: String,
    pub tx_state: String,
    pub tx_zipcode: String,
    pub ts_location_created_at: NaiveDateTime,
    pub ts_location_updated_at: NaiveDateTime,

    pub phones: Vec<Phone>,
    pub addresses: Vec<Address>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateLocation {
    pub tx_street: String,
    pub tx_number: String,
    pub tx_city: String,
    pub tx_state: String,
    pub tx_zipcode: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateLocation {
    pub tx_street: Option<String>,
    pub tx_number: Option<String>,
    pub tx_city: Option<String>,
    pub tx_state: Option<String>,
    pub tx_zipcode: Option<String>,
}
