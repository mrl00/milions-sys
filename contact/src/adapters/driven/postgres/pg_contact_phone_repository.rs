use crate::domain::models::db::phone_row::PhoneRow;
use sqlx::PgPool;
use uuid::Uuid;

pub struct PgPhoneRepository {
    pool: PgPool,
}

impl PgPhoneRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create<'a, E>(
        executor: E,
        contact_uuid: Uuid,
        phone: String,
    ) -> Result<PhoneRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let created_phone = sqlx::query_as!(
            PhoneRow,
            r#"
            INSERT INTO contacts.tb_phone (pk_phone, tx_phone, fk_contact)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &phone,
            &contact_uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(created_phone)
    }

    pub async fn create_many<'a, E>(
        executor: E,
        contact_uuid: Uuid,
        phones: Vec<String>,
    ) -> Result<Vec<PhoneRow>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let pks: Vec<Uuid> = phones.iter().map(|_| Uuid::now_v7()).collect();
        let fks: Vec<Uuid> = std::iter::repeat_n(contact_uuid, phones.len()).collect();

        let r = sqlx::query_as!(
            PhoneRow,
            r#"
            INSERT INTO contacts.tb_phone (pk_phone, fk_contact, tx_phone)
            SELECT * FROM UNNEST(
            $1::uuid[],
            $2::uuid[],
            $3::text[])
            RETURNING *
            "#,
            &pks as &[Uuid],
            &fks as &[Uuid],
            &phones as &[String],
        )
        .fetch_all(executor)
        .await?;

        Ok(r)
    }

    pub async fn update<'a, E>(
        executor: E,
        contact_uuid: Uuid,
        phone: String,
    ) -> Result<PhoneRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let updated_phone = sqlx::query_as!(
            PhoneRow,
            r#"
                    UPDATE contacts.tb_phone
                    SET tx_phone = $1
                    WHERE pk_phone = $2
                    RETURNING *
                    "#,
            &phone,
            &contact_uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(updated_phone)
    }

    pub async fn delete<'a, E>(executor: E, uuid: Uuid) -> Result<PhoneRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let deleted_phone = sqlx::query_as!(
            PhoneRow,
            r#"
            DELETE FROM contacts.tb_phone
            WHERE pk_phone = $1
            RETURNING *
            "#,
            &uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(deleted_phone)
    }

    pub async fn find_by_uuid<'a, E>(
        executor: E,
        uuid: Uuid,
    ) -> Result<Option<PhoneRow>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let phone = sqlx::query_as!(
            PhoneRow,
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
    ) -> Result<Vec<PhoneRow>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let phones = sqlx::query_as!(
            PhoneRow,
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
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
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
