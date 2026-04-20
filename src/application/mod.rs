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

pub fn pg_contact_serv_build(
    pool: sqlx::PgPool,
) -> crate::application::contact_service::PgContactService {
    crate::application::contact_service::PgContactService::new(
        crate::adapters::driven::pg_contact_repository::PgContactRepository::new(pool.clone()),
        crate::adapters::driven::pg_phone_repository::PgPhoneRepository::new(pool.clone()),
    )
}

pub fn pg_collaborator_serv_build(
    pool: sqlx::PgPool,
) -> crate::application::collaborator_service::PgCollaboratorService {
    crate::application::collaborator_service::PgCollaboratorService::new(
        crate::adapters::driven::pg_collaborator_repository::PgCollaboratorRepository::new(pool),
    )
}

pub fn pg_project_serv_build(
    pool: sqlx::PgPool,
) -> crate::application::project_service::PgProjectService {
    crate::application::project_service::PgProjectService::new(
        crate::adapters::driven::pg_project_repository::PgProjectRepository::new(pool),
    )
}
