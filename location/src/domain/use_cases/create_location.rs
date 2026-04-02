use async_trait::async_trait;

use crate::domain::errors::LocationError;
use crate::domain::models::db::location_row::LocationRow;

pub struct CreateLocationInput {
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
    pub hash: i64,
}

#[async_trait]
pub trait CreateLocation: Send + Sync {
    async fn execute(&self, input: CreateLocationInput) -> Result<LocationRow, LocationError>;
}
