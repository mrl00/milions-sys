pub mod adapters;
pub mod application;
pub mod domain;

use sqlx::PgPool;

pub fn build(pool: PgPool) -> application::location_service::ConcreteLocationService {
    application::location_service::ConcreteLocationService::new(
        adapters::driven::postgres::pg_location_repository::PgLocationRepository::new(pool),
    )
}
