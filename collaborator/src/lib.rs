pub mod adapters;
pub mod application;
pub mod domain;

use sqlx::PgPool;

pub fn build(pool: PgPool) -> application::collaborator_service::ConcreteCollaboratorService {
    application::collaborator_service::ConcreteCollaboratorService::new(
        adapters::driven::postgres::pg_collaborator_repository::PgCollaboratorRepository::new(pool),
    )
}
