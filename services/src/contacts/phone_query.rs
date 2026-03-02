use uuid::Uuid;

use crate::contacts::models::phone::Phone;

pub struct PhoneQuery;

impl PhoneQuery {
    pub async fn find_by_uuid<'a, E>(executor: E, uuid: Uuid) -> Result<Option<Phone>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
    {
        let phone = sqlx::query_as!(
            Phone,
            r#"
            SELECT *
            FROM contacts.tb_phone
            WHERE pk_phone = $1
            "#,
            &uuid,
        )
        .fetch_optional(executor)
        .await?;

        Ok(phone)
    }

    pub async fn get_by_contact<'a, E>(
        executor: E,
        contact: Uuid,
    ) -> Result<Vec<Phone>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
    {
        let phones = sqlx::query_as!(
            Phone,
            r#"
            SELECT *
            FROM contacts.tb_phone
            WHERE fk_contact = $1
            "#,
            &contact,
        )
        .fetch_all(executor)
        .await?;

        Ok(phones)
    }

    pub async fn check_by_phone_number<'a, E>(
        executor: E,
        phone: String,
    ) -> Result<bool, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
    {
        let exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM contacts.tb_phone
                WHERE tx_phone = $1
            )
            "#,
            &phone,
        )
        .fetch_one(executor)
        .await?;

        Ok(exists.unwrap_or(false))
    }
}
