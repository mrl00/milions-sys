use uuid::Uuid;

use crate::contacts::models::contact::ContactModel;

pub struct ContactQuery;

impl ContactQuery {
    /// Obtém um contato por `pk_contact` em `contacts.tb_contact`.
    pub async fn get_by_uuid<'a, E>(
        executor: E,
        uuid: Uuid,
    ) -> Result<Option<ContactModel>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let contact = sqlx::query_as!(
            ContactModel,
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

    /// Obtém um contato por `tx_email` em `contacts.tb_contact`.
    pub async fn get_by_email<'a, E>(
        executor: E,
        email: String,
    ) -> Result<Option<ContactModel>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let contact = sqlx::query_as!(
            ContactModel,
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

    /// Lista todos os contatos de `contacts.tb_contact` ordenados por `idx_contact`.
    pub async fn list_all(pool: &sqlx::PgPool) -> Result<Vec<ContactModel>, sqlx::Error> {
        let contacts = sqlx::query_as!(
            ContactModel,
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
