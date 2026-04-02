use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::LocationError;
use crate::domain::models::db::location_row::LocationRow;

pub struct UpdateLocationInput {
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

#[async_trait]
pub trait UpdateLocation: Send + Sync {
    async fn execute(
        &self,
        uuid: Uuid,
        input: UpdateLocationInput,
    ) -> Result<LocationRow, LocationError>;
}
