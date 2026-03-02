use uuid::Uuid;

use crate::contacts::models::contact::Contact;

pub struct ContactQuery;

impl ContactQuery {
    pub async fn get_by_uuid(pool: &sqlx::PgPool, uuid: Uuid) -> Result<Contact, sqlx::Error> {
        let contact = sqlx::query_as!(
            Contact,
            r#"
            SELECT *
            FROM contacts.tb_contact
            WHERE pk_contact = $1
            "#,
            &uuid,
        )
        .fetch_one(pool)
        .await?;

        Ok(contact)
    }

    pub async fn get_by_email(
        pool: &sqlx::PgPool,
        email: String,
    ) -> Result<Option<Contact>, sqlx::Error> {
        let contact = sqlx::query_as!(
            Contact,
            r#"
            SELECT *
            FROM contacts.tb_contact
            WHERE tx_email = $1
            "#,
            &email,
        )
        .fetch_optional(pool)
        .await?;

        Ok(contact)
    }

    pub async fn list_all(pool: &sqlx::PgPool) -> Result<Vec<Contact>, sqlx::Error> {
        let contacts = sqlx::query_as!(
            Contact,
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

