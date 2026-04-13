use async_trait::async_trait;
use uuid::Uuid;

use crate::adapters::driven::pg_location_repository::PgLocationRepository;
use crate::domain::errors::location_error::LocationError;
use crate::domain::models::db::location_row::{CreateLocationRow, LocationRow, UpdateLocationRow};
use crate::domain::ports::location_repository::LocationRepository;
use crate::domain::ports::location_use_cases::{
    CreateLocationInput, CreateLocationUseCase, DeleteLocationUseCase, FindLocationUseCase,
    ListLocationsUseCase, UpdateLocationInput, UpdateLocationUseCase,
};

pub struct LocationService<R> {
    repo: R,
}

impl<R: LocationRepository> LocationService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    fn to_create_row(input: CreateLocationInput) -> CreateLocationRow {
        CreateLocationRow {
            tx_street: crate::domain::value_objects::text::remove_accents(&input.street),
            tx_number: input.number,
            tx_city: crate::domain::value_objects::text::remove_accents(&input.city),
            tx_state: input.state,
            tx_zipcode: input.zipcode,
            tx_address_complement: crate::domain::value_objects::text::remove_accents(&input.complement),
            tx_public_space: crate::domain::value_objects::text::remove_accents(&input.public_space),
            tx_unit: input.unit,
            tx_neighborhood: crate::domain::value_objects::text::remove_accents(&input.neighborhood),
            tx_locality: crate::domain::value_objects::text::remove_accents(&input.locality),
            tx_region: input.region,
            tx_ibge: input.ibge,
            tx_gia: input.gia,
            tx_ddd: input.ddd,
            tx_siafi: input.siafi,
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

pub type ConcreteLocationService = LocationService<PgLocationRepository>;

#[async_trait]
impl<R: LocationRepository> FindLocationUseCase for LocationService<R> {
    async fn execute(&self, uuid: Uuid) -> Result<LocationRow, LocationError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(LocationError::NotFound { uuid })
    }
}

#[async_trait]
impl<R: LocationRepository> ListLocationsUseCase for LocationService<R> {
    async fn execute(&self) -> Result<Vec<LocationRow>, LocationError> {
        self.repo.find_all().await
    }
}

#[async_trait]
impl<R: LocationRepository> CreateLocationUseCase for LocationService<R> {
    async fn execute(&self, input: CreateLocationInput) -> Result<LocationRow, LocationError> {
        self.repo.create(Self::to_create_row(input)).await
    }
}

#[async_trait]
impl<R: LocationRepository> UpdateLocationUseCase for LocationService<R> {
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
impl<R: LocationRepository> DeleteLocationUseCase for LocationService<R> {
    async fn execute(&self, uuid: Uuid) -> Result<LocationRow, LocationError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(LocationError::NotFound { uuid })?;

        self.repo.delete(uuid).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ports::location_repository::{
        CreateLocation, DeleteLocation, FindAllLocations, FindLocationById, UpdateLocation,
    };
    use crate::domain::ports::location_use_cases::{
        CreateLocationUseCase, DeleteLocationUseCase, FindLocationUseCase, ListLocationsUseCase,
        UpdateLocationUseCase,
    };
    use sqlx::types::chrono::NaiveDateTime;

    enum FindByIdResult {
        Found(LocationRow),
        NotFound,
    }

    enum FindAllResult {
        Found(Vec<LocationRow>),
    }

    #[derive(Default)]
    struct MockRepo {
        find_by_id_result: Option<FindByIdResult>,
        find_all_result: Option<FindAllResult>,
    }

    impl MockRepo {
        fn new() -> Self {
            Self::default()
        }
    }

    fn now() -> NaiveDateTime {
        NaiveDateTime::default()
    }

    fn make_row() -> LocationRow {
        LocationRow {
            pk_location: Uuid::now_v7(),
            idx_location: 1,
            tx_public_space: "Rua".to_string(),
            tx_address_complement: Some("Apto 101".to_string()),
            tx_unit: "101".to_string(),
            tx_neighborhood: "Centro".to_string(),
            tx_locality: "São Paulo".to_string(),
            tx_region: "SP".to_string(),
            tx_ibge: Some("3550308".to_string()),
            tx_gia: Some("1004".to_string()),
            tx_ddd: "11".to_string(),
            tx_siafi: Some("7107".to_string()),
            tx_street: "Paulista".to_string(),
            tx_number: "1000".to_string(),
            tx_city: "São Paulo".to_string(),
            tx_state: "SP".to_string(),
            tx_zipcode: "01310100".to_string(),
            nr_hash: Some(123456789),
            ts_location_created_at: now(),
            ts_location_updated_at: now(),
        }
    }

    fn default_row(uuid: Uuid) -> LocationRow {
        LocationRow {
            pk_location: uuid,
            idx_location: 0,
            tx_public_space: "".to_string(),
            tx_address_complement: None,
            tx_unit: "".to_string(),
            tx_neighborhood: "".to_string(),
            tx_locality: "".to_string(),
            tx_region: "".to_string(),
            tx_ibge: None,
            tx_gia: None,
            tx_ddd: "".to_string(),
            tx_siafi: None,
            tx_street: "".to_string(),
            tx_number: "".to_string(),
            tx_city: "".to_string(),
            tx_state: "".to_string(),
            tx_zipcode: "".to_string(),
            nr_hash: Some(0),
            ts_location_created_at: now(),
            ts_location_updated_at: now(),
        }
    }

    #[async_trait]
    impl FindLocationById for MockRepo {
        async fn find_by_id(&self, _uuid: Uuid) -> Result<Option<LocationRow>, LocationError> {
            match &self.find_by_id_result {
                Some(FindByIdResult::Found(row)) => Ok(Some(row.clone())),
                Some(FindByIdResult::NotFound) | None => Ok(None),
            }
        }
    }

    #[async_trait]
    impl FindAllLocations for MockRepo {
        async fn find_all(&self) -> Result<Vec<LocationRow>, LocationError> {
            match &self.find_all_result {
                Some(FindAllResult::Found(rows)) => Ok(rows.clone()),
                None => Ok(vec![]),
            }
        }
    }

    #[async_trait]
    impl CreateLocation for MockRepo {
        async fn create(&self, input: CreateLocationRow) -> Result<LocationRow, LocationError> {
            Ok(LocationRow {
                pk_location: Uuid::now_v7(),
                idx_location: 0,
                tx_public_space: input.tx_public_space,
                tx_address_complement: Some(input.tx_address_complement),
                tx_unit: input.tx_unit,
                tx_neighborhood: input.tx_neighborhood,
                tx_locality: input.tx_locality,
                tx_region: input.tx_region,
                tx_ibge: input.tx_ibge,
                tx_gia: input.tx_gia,
                tx_ddd: input.tx_ddd,
                tx_siafi: input.tx_siafi,
                tx_street: input.tx_street,
                tx_number: input.tx_number,
                tx_city: input.tx_city,
                tx_state: input.tx_state,
                tx_zipcode: input.tx_zipcode,
                nr_hash: Some(0),
                ts_location_created_at: now(),
                ts_location_updated_at: now(),
            })
        }
    }

    #[async_trait]
    impl UpdateLocation for MockRepo {
        async fn update(
            &self,
            uuid: Uuid,
            _input: UpdateLocationRow,
        ) -> Result<LocationRow, LocationError> {
            Ok(default_row(uuid))
        }
    }

    #[async_trait]
    impl DeleteLocation for MockRepo {
        async fn delete(&self, uuid: Uuid) -> Result<LocationRow, LocationError> {
            Ok(default_row(uuid))
        }
    }

    #[tokio::test]
    async fn find_location_returns_row_when_exists() {
        let row = make_row();
        let uuid = row.pk_location;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(FindByIdResult::Found(row));
        let service = LocationService::new(repo);
        let result = FindLocationUseCase::execute(&service, uuid).await.unwrap();
        assert_eq!(result.pk_location, uuid);
        assert_eq!(result.tx_street, "Paulista");
    }

    #[tokio::test]
    async fn find_location_returns_not_found_when_missing() {
        let uuid = Uuid::now_v7();
        let repo = MockRepo::new();
        let service = LocationService::new(repo);
        let result = FindLocationUseCase::execute(&service, uuid).await;
        assert!(matches!(result, Err(LocationError::NotFound { .. })));
    }

    #[tokio::test]
    async fn list_locations_returns_all_rows() {
        let row1 = make_row();
        let row2 = make_row();
        let mut repo = MockRepo::new();
        repo.find_all_result = Some(FindAllResult::Found(vec![row1, row2]));
        let service = LocationService::new(repo);
        let result = ListLocationsUseCase::execute(&service).await.unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn list_locations_returns_empty_when_none() {
        let repo = MockRepo::new();
        let service = LocationService::new(repo);
        let result = ListLocationsUseCase::execute(&service).await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn create_location_delegates_to_repo() {
        let input = CreateLocationInput {
            street: "Paulista".to_string(),
            number: "1000".to_string(),
            city: "São Paulo".to_string(),
            state: "SP".to_string(),
            zipcode: "01310100".to_string(),
            complement: "Apto 101".to_string(),
            public_space: "Rua".to_string(),
            unit: "101".to_string(),
            neighborhood: "Centro".to_string(),
            locality: "São Paulo".to_string(),
            region: "SP".to_string(),
            ibge: Some("3550308".to_string()),
            gia: Some("1004".to_string()),
            ddd: "11".to_string(),
            siafi: Some("7107".to_string()),
        };
        let repo = MockRepo::new();
        let service = LocationService::new(repo);
        let result = CreateLocationUseCase::execute(&service, input)
            .await
            .unwrap();
        assert_eq!(result.tx_street, "Paulista");
    }

    #[tokio::test]
    async fn update_location_returns_not_found_when_missing() {
        let uuid = Uuid::now_v7();
        let repo = MockRepo::new();
        let service = LocationService::new(repo);
        let input = UpdateLocationInput {
            street: Some("Updated".to_string()),
            number: None,
            city: None,
            state: None,
            zipcode: None,
            complement: None,
            public_space: None,
            unit: None,
            neighborhood: None,
            locality: None,
            region: None,
            ibge: None,
            gia: None,
            ddd: None,
            siafi: None,
        };
        let result = UpdateLocationUseCase::execute(&service, uuid, input).await;
        assert!(matches!(result, Err(LocationError::NotFound { .. })));
    }

    #[tokio::test]
    async fn update_location_delegates_to_repo_when_exists() {
        let row = make_row();
        let uuid = row.pk_location;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(FindByIdResult::Found(row));
        let input = UpdateLocationInput {
            street: Some("Updated Street".to_string()),
            number: Some("2000".to_string()),
            city: None,
            state: None,
            zipcode: None,
            complement: None,
            public_space: None,
            unit: None,
            neighborhood: None,
            locality: None,
            region: None,
            ibge: None,
            gia: None,
            ddd: None,
            siafi: None,
        };
        let service = LocationService::new(repo);
        let result = UpdateLocationUseCase::execute(&service, uuid, input)
            .await
            .unwrap();
        assert_eq!(result.pk_location, uuid);
    }

    #[tokio::test]
    async fn delete_location_returns_not_found_when_missing() {
        let uuid = Uuid::now_v7();
        let repo = MockRepo::new();
        let service = LocationService::new(repo);
        let result = DeleteLocationUseCase::execute(&service, uuid).await;
        assert!(matches!(result, Err(LocationError::NotFound { .. })));
    }

    #[tokio::test]
    async fn delete_location_delegates_to_repo_when_exists() {
        let row = make_row();
        let uuid = row.pk_location;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(FindByIdResult::Found(row));
        let service = LocationService::new(repo);
        let result = DeleteLocationUseCase::execute(&service, uuid)
            .await
            .unwrap();
        assert_eq!(result.pk_location, uuid);
    }

    #[test]
    fn error_not_found_message_contains_uuid() {
        let uuid = Uuid::now_v7();
        let err = LocationError::NotFound { uuid };
        let msg = err.to_string();
        assert!(msg.contains(&uuid.to_string()));
        assert!(msg.contains("location not found"));
    }

    #[test]
    fn error_already_exists_message_contains_hash() {
        let err = LocationError::AlreadyExists { hash: 12345 };
        let msg = err.to_string();
        assert!(msg.contains("12345"));
        assert!(msg.contains("location already exists"));
    }

    #[test]
    fn error_invalid_field_message_contains_field_and_reason() {
        let err = LocationError::InvalidField {
            field: "zipcode",
            reason: "must be 8 digits".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("zipcode"));
        assert!(msg.contains("must be 8 digits"));
        assert!(msg.contains("invalid field"));
    }

    #[test]
    fn location_row_clone_and_eq() {
        let row = make_row();
        let cloned = row.clone();
        assert_eq!(row, cloned);
    }

    #[test]
    fn location_row_debug_does_not_panic() {
        let row = make_row();
        let debug = format!("{:?}", row);
        assert!(debug.contains("LocationRow"));
        assert!(debug.contains("tx_street"));
    }
}
