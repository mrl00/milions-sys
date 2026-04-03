use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::errors::contact_error::ContactError;
use crate::domain::models::db::contact_row::{ContactRow, CreateContactRow};
use crate::domain::ports::contact_repository::*;
use types::errors::infra_error::InfraError;

pub struct PgContactRepository {
    pool: PgPool,
}

impl PgContactRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_with_executor<'a, E>(
        executor: E,
        input: CreateContactRow,
    ) -> Result<ContactRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query_as!(
            ContactRow,
            r#"
            INSERT INTO contacts.tb_contact (pk_contact, tx_email)
            VALUES ($1, $2)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &input.tx_email,
        )
        .fetch_one(executor)
        .await
    }
}

fn sqlx_err(action: &'static str) -> impl FnOnce(sqlx::Error) -> ContactError {
    move |e| ContactError::Infra {
        source: InfraError::Database { action, source: e },
    }
}

#[async_trait]
impl FindContactById for PgContactRepository {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<ContactRow>, ContactError> {
        sqlx::query_as!(
            ContactRow,
            r#"
            SELECT *
            FROM contacts.tb_contact
            WHERE pk_contact = $1
            "#,
            &uuid,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_err("find contact by id"))
    }
}

#[async_trait]
impl FindContactByEmail for PgContactRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<ContactRow>, ContactError> {
        sqlx::query_as!(
            ContactRow,
            r#"
            SELECT *
            FROM contacts.tb_contact
            WHERE tx_email = $1
            "#,
            &email,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_err("find contact by email"))
    }
}

#[async_trait]
impl FindAllContacts for PgContactRepository {
    async fn find_all(&self) -> Result<Vec<ContactRow>, ContactError> {
        sqlx::query_as!(
            ContactRow,
            r#"
            SELECT *
            FROM contacts.tb_contact
            ORDER BY idx_contact
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(sqlx_err("list contacts"))
    }
}

#[async_trait]
impl CreateContact for PgContactRepository {
    async fn create(&self, contact: CreateContactRow) -> Result<ContactRow, ContactError> {
        sqlx::query_as!(
            ContactRow,
            r#"
            INSERT INTO contacts.tb_contact (pk_contact, tx_email)
            VALUES ($1, $2)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &contact.tx_email,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_err("create contact"))
    }
}

#[async_trait]
impl UpdateContactEmail for PgContactRepository {
    async fn update_email(&self, uuid: Uuid, email: String) -> Result<ContactRow, ContactError> {
        sqlx::query_as!(
            ContactRow,
            r#"
            UPDATE contacts.tb_contact
            SET tx_email = $1
            WHERE pk_contact = $2
            RETURNING *
            "#,
            &email,
            &uuid,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_err("update contact email"))
    }
}

impl FindAndCreateContact for PgContactRepository {}
impl FindAndUpdateContact for PgContactRepository {}
