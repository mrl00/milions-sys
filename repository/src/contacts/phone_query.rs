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

    pub async fn find_nonexistent_phones<'a, E>(
        executor: E,
        phones: Vec<String>,
    ) -> Result<Vec<String>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
    {
        let r = sqlx::query_scalar!(
            r#"SELECT input.tx_phone
            FROM UNNEST($1::text[]) AS input(tx_phone)
            LEFT JOIN contacts.tb_phone p ON p.tx_phone = input.tx_phone
            WHERE p.tx_phone IS NULL"#,
            &phones as &[String],
        )
        .fetch_all(executor)
        .await?
        .iter()
        .filter_map(|p| p.clone())
        .collect();

        Ok(r)
    }
}
