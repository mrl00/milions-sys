use crate::domain::models::db::contact_row::{ContactRow, CreateContactRow};
use sqlx::PgPool;
use uuid::Uuid;

pub struct PgContactRepository {
    pool: PgPool,
}

impl PgContactRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create<'a, E>(
        executor: E,
        contact: CreateContactRow,
    ) -> Result<ContactRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let created_contact = sqlx::query_as!(
            ContactRow,
            r#"
            INSERT INTO contacts.tb_contact (pk_contact, tx_email)
            VALUES ($1, $2)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &contact.tx_email,
        )
        .fetch_one(executor)
        .await?;

        Ok(created_contact)
    }

    pub async fn update_email<'a, E>(
        executor: E,
        uuid: Uuid,
        email: String,
    ) -> Result<ContactRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let updated_contact = sqlx::query_as!(
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
        .fetch_one(executor)
        .await?;

        Ok(updated_contact)
    }

    pub async fn get_by_uuid<'a, E>(
        executor: E,
        uuid: Uuid,
    ) -> Result<Option<ContactRow>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let contact = sqlx::query_as!(
            ContactRow,
            r#"
            SELECT *
            FROM contacts.tb_contact
            WHERE pk_contact = $1
            "#,
            &uuid,
        )
        .fetch_optional(executor)
        .await?;

        Ok(contact)
    }

    pub async fn get_by_email<'a, E>(
        executor: E,
        email: String,
    ) -> Result<Option<ContactRow>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let contact = sqlx::query_as!(
            ContactRow,
            r#"
            SELECT *
            FROM contacts.tb_contact
            WHERE tx_email = $1
            "#,
            &email,
        )
        .fetch_optional(executor)
        .await?;

        Ok(contact)
    }

    pub async fn list_all(pool: &sqlx::PgPool) -> Result<Vec<ContactRow>, sqlx::Error> {
        let contacts = sqlx::query_as!(
            ContactRow,
            r#"
            SELECT *
            FROM contacts.tb_contact
            ORDER BY idx_contact
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(contacts)
    }
}
