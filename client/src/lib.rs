pub mod adapters;
pub mod application;
pub mod domain;

use sqlx::PgPool;

pub fn build(pool: PgPool) -> application::client_service::ConcreteClientService {
    application::client_service::ConcreteClientService::new(
        adapters::driven::postgres::pg_client_repository::PgClientRepository::new(pool.clone()),
        pool,
    )
}
