use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::LocationError;
use crate::domain::models::db::location_row::LocationRow;

#[async_trait]
pub trait FindLocation: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<LocationRow, LocationError>;
}
