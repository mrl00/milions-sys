pub mod adapters;
pub mod application;
pub mod domain;

use sqlx::PgPool;

pub fn build(pool: PgPool) -> application::contact_service::ConcreteContactService {
    application::contact_service::ConcreteContactService::new(
        adapters::driven::postgres::pg_contact_repository::PgContactRepository::new(pool.clone()),
        adapters::driven::postgres::pg_phone_repository::PgPhoneRepository::new(pool),
    )
}
