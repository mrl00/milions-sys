pub mod adapters;
pub mod application;
pub mod domain;

use sqlx::PgPool;

pub fn build(pool: PgPool) -> application::project_service::ConcreteProjectService {
    application::project_service::ConcreteProjectService::new(
        adapters::driven::postgres::PgProjectRepository::new(pool),
    )
}
