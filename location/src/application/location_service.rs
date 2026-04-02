use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::adapters::driven::postgres::pg_location_repository::PgLocationRepository;
use crate::domain::errors::LocationError;
use crate::domain::models::db::location_row::{CreateLocationRow, LocationRow, UpdateLocationRow};
use crate::domain::ports::location_repository::{
    CreateLocation as _, DeleteLocation as _, FindAllLocations as _, FindLocationByHash as _,
    FindLocationById as _, UpdateLocation as _,
};
use crate::domain::ports::location_use_cases::{
    CreateLocation as CreateLocationTrait, CreateLocationInput,
    DeleteLocation as DeleteLocationTrait, FindLocation, FindOrCreateLocation as FindOrCreateLocationTrait,
    ListLocations, UpdateLocation as UpdateLocationTrait, UpdateLocationInput,
};

pub struct LocationService {
    repo: PgLocationRepository,
}

impl LocationService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: PgLocationRepository::new(pool),
        }
    }

    fn to_create_row(input: CreateLocationInput) -> CreateLocationRow {
        CreateLocationRow {
            tx_street: input.street,
            tx_number: input.number,
            tx_city: input.city,
            tx_state: input.state,
            tx_zipcode: input.zipcode,
            tx_address_complement: input.complement,
            tx_public_space: input.public_space,
            tx_unit: input.unit,
            tx_neighborhood: input.neighborhood,
            tx_locality: input.locality,
            tx_region: input.region,
            tx_ibge: input.ibge,
            tx_gia: input.gia,
            tx_ddd: input.ddd,
            tx_siafi: input.siafi,
            nr_hash: input.hash,
        }
    }

    fn to_update_row(input: UpdateLocationInput) -> UpdateLocationRow {
        UpdateLocationRow {
            tx_street: input.street,
            tx_number: input.number,
            tx_city: input.city,
            tx_state: input.state,
            tx_zipcode: input.zipcode,
            tx_address_complement: input.complement,
            tx_public_space: input.public_space,
            tx_unit: input.unit,
            tx_neighborhood: input.neighborhood,
            tx_locality: input.locality,
            tx_region: input.region,
            tx_ibge: input.ibge,
            tx_gia: input.gia,
            tx_ddd: input.ddd,
            tx_siafi: input.siafi,
        }
    }
}

#[async_trait]
impl FindLocation for LocationService {
    async fn execute(&self, uuid: Uuid) -> Result<LocationRow, LocationError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(LocationError::NotFound { uuid })
    }
}

#[async_trait]
impl ListLocations for LocationService {
    async fn execute(&self) -> Result<Vec<LocationRow>, LocationError> {
        self.repo.find_all().await
    }
}

#[async_trait]
impl CreateLocationTrait for LocationService {
    async fn execute(&self, input: CreateLocationInput) -> Result<LocationRow, LocationError> {
        self.repo.create(Self::to_create_row(input)).await
    }
}

#[async_trait]
impl FindOrCreateLocationTrait for LocationService {
    async fn execute(&self, input: CreateLocationInput) -> Result<LocationRow, LocationError> {
        if let Some(existing) = self.repo.find_by_hash(input.hash).await? {
            return Ok(existing);
        }

        self.repo.create(Self::to_create_row(input)).await
    }
}

#[async_trait]
impl UpdateLocationTrait for LocationService {
    async fn execute(
        &self,
        uuid: Uuid,
        input: UpdateLocationInput,
    ) -> Result<LocationRow, LocationError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(LocationError::NotFound { uuid })?;

        self.repo.update(uuid, Self::to_update_row(input)).await
    }
}

#[async_trait]
impl DeleteLocationTrait for LocationService {
    async fn execute(&self, uuid: Uuid) -> Result<LocationRow, LocationError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(LocationError::NotFound { uuid })?;

        self.repo.delete(uuid).await
    }
}
