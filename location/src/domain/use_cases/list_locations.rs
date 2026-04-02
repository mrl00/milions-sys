use async_trait::async_trait;

use crate::domain::errors::LocationError;
use crate::domain::models::db::location_row::LocationRow;

#[async_trait]
pub trait ListLocations: Send + Sync {
    async fn execute(&self) -> Result<Vec<LocationRow>, LocationError>;
}
