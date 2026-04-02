use async_trait::async_trait;

use crate::domain::errors::LocationError;
use crate::domain::models::db::location_row::LocationRow;

pub use super::create_location::CreateLocationInput;

#[async_trait]
pub trait FindOrCreateLocation: Send + Sync {
    async fn execute(&self, input: CreateLocationInput) -> Result<LocationRow, LocationError>;
}
