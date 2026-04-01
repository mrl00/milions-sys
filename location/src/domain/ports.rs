use async_trait::async_trait;
use uuid::Uuid;

use super::errors::LocationError;
use super::model::{CreateLocationRow, LocationRow, UpdateLocationRow};

#[async_trait]
pub trait LocationRepository: Send + Sync {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<LocationRow>, LocationError>;
    async fn find_by_hash(&self, hash: i64) -> Result<Option<LocationRow>, LocationError>;
    async fn find_all(&self) -> Result<Vec<LocationRow>, LocationError>;
    async fn create(&self, input: CreateLocationRow) -> Result<LocationRow, LocationError>;
    async fn update(
        &self,
        uuid: Uuid,
        input: UpdateLocationRow,
    ) -> Result<LocationRow, LocationError>;
    async fn delete(&self, uuid: Uuid) -> Result<LocationRow, LocationError>;
}
