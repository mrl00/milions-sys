use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::LocationError;
use crate::domain::models::db::location_row::{CreateLocationRow, LocationRow, UpdateLocationRow};

#[async_trait]
pub trait FindLocationById: Send + Sync {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<LocationRow>, LocationError>;
}

#[async_trait]
pub trait FindLocationByHash: Send + Sync {
    async fn find_by_hash(&self, hash: i64) -> Result<Option<LocationRow>, LocationError>;
}

#[async_trait]
pub trait FindAllLocations: Send + Sync {
    async fn find_all(&self) -> Result<Vec<LocationRow>, LocationError>;
}

#[async_trait]
pub trait CreateLocation: Send + Sync {
    async fn create(&self, input: CreateLocationRow) -> Result<LocationRow, LocationError>;
}

#[async_trait]
pub trait UpdateLocation: Send + Sync {
    async fn update(
        &self,
        uuid: Uuid,
        input: UpdateLocationRow,
    ) -> Result<LocationRow, LocationError>;
}

#[async_trait]
pub trait DeleteLocation: Send + Sync {
    async fn delete(&self, uuid: Uuid) -> Result<LocationRow, LocationError>;
}

pub trait FindOrCreateLocation: FindLocationByHash + CreateLocation {}
pub trait FindAndUpdateLocation: FindLocationById + UpdateLocation {}
pub trait FindAndDeleteLocation: FindLocationById + DeleteLocation {}
