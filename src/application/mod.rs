pub mod client_service;
pub mod collaborator_service;
pub mod contact_service;
pub mod location_service;
pub mod project_service;

pub fn pg_location_serv_build(
    pool: sqlx::PgPool,
) -> crate::application::location_service::PgLocationService {
    crate::application::location_service::PgLocationService::new(
        crate::adapters::driven::pg_location_repository::PgLocationRepository::new(pool),
    )
}
