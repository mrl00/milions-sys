use crate::domain::errors::LocationError;
use uuid::Uuid;
use crate::domain::models::db::location_row::{CreateLocationRow, LocationRow, UpdateLocationRow};
use crate::domain::ports::location_repository::LocationRepository;

pub struct LocationService;

impl LocationService {
    pub async fn find_by_id(
        repo: &dyn LocationRepository,
        uuid: Uuid,
    ) -> Result<LocationRow, LocationError> {
        repo.find_by_id(uuid)
            .await?
            .ok_or(LocationError::NotFound { uuid })
    }

    pub async fn find_by_hash(
        repo: &dyn LocationRepository,
        hash: i64,
    ) -> Result<Option<LocationRow>, LocationError> {
        repo.find_by_hash(hash).await
    }

    pub async fn find_all(
        repo: &dyn LocationRepository,
    ) -> Result<Vec<LocationRow>, LocationError> {
        repo.find_all().await
    }

    pub async fn create(
        repo: &dyn LocationRepository,
        input: CreateLocationRow,
    ) -> Result<LocationRow, LocationError> {
        if let Some(existing) = repo.find_by_hash(input.nr_hash).await? {
            return Ok(existing);
        }

        repo.create(input).await
    }

    pub async fn find_or_create(
        repo: &dyn LocationRepository,
        input: CreateLocationRow,
    ) -> Result<LocationRow, LocationError> {
        Self::create(repo, input).await
    }

    pub async fn update(
        repo: &dyn LocationRepository,
        uuid: Uuid,
        input: UpdateLocationRow,
    ) -> Result<LocationRow, LocationError> {
        Self::find_by_id(repo, uuid).await?;
        repo.update(uuid, input).await
    }

    pub async fn delete(
        repo: &dyn LocationRepository,
        uuid: Uuid,
    ) -> Result<LocationRow, LocationError> {
        Self::find_by_id(repo, uuid).await?;
        repo.delete(uuid).await
    }
}
