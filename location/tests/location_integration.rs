// Integration tests for location bounded context.
// Uses #[sqlx::test] to create isolated databases with migrations applied.

use location::adapters::driven::postgres::pg_location_repository::PgLocationRepository;
use location::application::location_service::ConcreteLocationService;
use location::domain::ports::location_repository::CreateLocation;
use location::domain::ports::location_use_cases::{
    CreateLocationInput, CreateLocationUseCase, DeleteLocationUseCase, FindLocationUseCase,
    ListLocationsUseCase, UpdateLocationInput, UpdateLocationUseCase,
};
use sqlx::PgPool;

fn make_service(pool: PgPool) -> ConcreteLocationService {
    ConcreteLocationService::new(PgLocationRepository::new(pool))
}

fn make_location_row(
    street: &str,
    number: &str,
    city: &str,
    state: &str,
    zipcode: &str,
) -> location::domain::models::db::location_row::CreateLocationRow {
    location::domain::models::db::location_row::CreateLocationRow {
        tx_street: street.to_string(),
        tx_number: number.to_string(),
        tx_city: city.to_string(),
        tx_state: state.to_string(),
        tx_zipcode: zipcode.to_string(),
        tx_public_space: "Rua".to_string(),
        tx_address_complement: "".to_string(),
        tx_unit: "".to_string(),
        tx_neighborhood: "Neighborhood".to_string(),
        tx_locality: city.to_string(),
        tx_region: state.to_string(),
        tx_ibge: None,
        tx_gia: None,
        tx_ddd: "11".to_string(),
        tx_siafi: None,
    }
}

#[sqlx::test(migrations = "../migrations")]
async fn create_and_find_location(pool: PgPool) {
    let service = make_service(pool);

    let input = CreateLocationInput {
        street: "Praça da Sé".to_string(),
        number: "100".to_string(),
        city: "São Paulo".to_string(),
        state: "SP".to_string(),
        zipcode: "01001000".to_string(),
        complement: "Lado ímpar".to_string(),
        public_space: "Praça".to_string(),
        unit: "".to_string(),
        neighborhood: "Sé".to_string(),
        locality: "São Paulo".to_string(),
        region: "Sudeste".to_string(),
        ibge: Some("3550308".to_string()),
        gia: Some("1004".to_string()),
        ddd: "11".to_string(),
        siafi: Some("7107".to_string()),
    };

    let created = CreateLocationUseCase::execute(&service, input)
        .await
        .expect("create location");

    let found = FindLocationUseCase::execute(&service, created.pk_location)
        .await
        .expect("find location");

    assert_eq!(found.pk_location, created.pk_location);
    assert_eq!(found.tx_street, "Praca da Se");
    assert_eq!(found.tx_city, "Sao Paulo");
    assert_eq!(found.tx_state, "SP");
    assert_eq!(found.tx_zipcode, "01001000");
    assert_eq!(found.tx_neighborhood, "Se");
}

#[sqlx::test(migrations = "../migrations")]
async fn create_location_with_same_address_returns_existing(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgLocationRepository::new(pool);

    let input = CreateLocationInput {
        street: "Rua Augusta".to_string(),
        number: "500".to_string(),
        city: "São Paulo".to_string(),
        state: "SP".to_string(),
        zipcode: "01305000".to_string(),
        complement: "".to_string(),
        public_space: "Rua".to_string(),
        unit: "".to_string(),
        neighborhood: "Consolação".to_string(),
        locality: "São Paulo".to_string(),
        region: "Sudeste".to_string(),
        ibge: None,
        gia: None,
        ddd: "11".to_string(),
        siafi: None,
    };

    let first = CreateLocationUseCase::execute(&service, input.clone())
        .await
        .expect("create first location");

    // Second insert with same address should fail due to unique constraint on nr_hash
    let result = CreateLocationUseCase::execute(&service, input).await;
    assert!(result.is_err());

    // But the first location still exists and can be found
    let found = FindLocationUseCase::execute(&service, first.pk_location)
        .await
        .expect("find first location");
    assert_eq!(found.pk_location, first.pk_location);
}

#[sqlx::test(migrations = "../migrations")]
async fn list_locations_returns_all(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgLocationRepository::new(pool);

    repo.create(make_location_row(
        "Street A", "1", "City A", "SP", "00000001",
    ))
    .await
    .expect("create location A");

    repo.create(make_location_row(
        "Street B", "2", "City B", "RJ", "00000002",
    ))
    .await
    .expect("create location B");

    let locations = ListLocationsUseCase::execute(&service)
        .await
        .expect("list locations");

    assert_eq!(locations.len(), 2);
}

#[sqlx::test(migrations = "../migrations")]
async fn update_location_changes_fields(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgLocationRepository::new(pool);

    let created = repo
        .create(make_location_row(
            "Old Street",
            "100",
            "Old City",
            "SP",
            "00000000",
        ))
        .await
        .expect("create location");

    let updated = UpdateLocationUseCase::execute(
        &service,
        created.pk_location,
        UpdateLocationInput {
            street: Some("New Street".to_string()),
            number: Some("200".to_string()),
            city: Some("New City".to_string()),
            state: Some("RJ".to_string()),
            zipcode: Some("11111111".to_string()),
            complement: Some("New Complement".to_string()),
            public_space: None,
            unit: None,
            neighborhood: Some("New Neighborhood".to_string()),
            locality: None,
            region: None,
            ibge: None,
            gia: None,
            ddd: None,
            siafi: None,
        },
    )
    .await
    .expect("update location");

    assert_eq!(updated.tx_street, "New Street");
    assert_eq!(updated.tx_number, "200");
    assert_eq!(updated.tx_city, "New City");
    assert_eq!(updated.tx_state, "RJ");
    assert_eq!(updated.tx_zipcode, "11111111");
    assert_eq!(
        updated.tx_address_complement,
        Some("New Complement".to_string())
    );
    assert_eq!(updated.tx_neighborhood, "New Neighborhood");
    assert_eq!(updated.tx_public_space, "Rua");
}

#[sqlx::test(migrations = "../migrations")]
async fn delete_location_removes_row(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgLocationRepository::new(pool);

    let created = repo
        .create(make_location_row(
            "To Delete",
            "1",
            "City",
            "SP",
            "00000000",
        ))
        .await
        .expect("create location");

    DeleteLocationUseCase::execute(&service, created.pk_location)
        .await
        .expect("delete location");

    let found = FindLocationUseCase::execute(&service, created.pk_location).await;
    assert!(found.is_err());
}

#[sqlx::test(migrations = "../migrations")]
async fn find_location_returns_not_found_for_missing(pool: PgPool) {
    let service = make_service(pool);

    let uuid = uuid::Uuid::now_v7();
    let result: Result<_, _> = FindLocationUseCase::execute(&service, uuid).await;

    assert!(result.is_err());
}
