use async_trait::async_trait;
use uuid::Uuid;

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
pub trait FindLocation: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<LocationRow, LocationError>;
}

#[async_trait]
pub trait ListLocations: Send + Sync {
    async fn execute(&self) -> Result<Vec<LocationRow>, LocationError>;
}

#[async_trait]
pub trait CreateLocation: Send + Sync {
    async fn execute(&self, input: CreateLocationInput) -> Result<LocationRow, LocationError>;
}

#[async_trait]
pub trait UpdateLocation: Send + Sync {
    async fn execute(
        &self,
        uuid: Uuid,
        input: UpdateLocationInput,
    ) -> Result<LocationRow, LocationError>;
}

#[async_trait]
pub trait DeleteLocation: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<LocationRow, LocationError>;
}
