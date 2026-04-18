use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::errors::contact_error::ContactError;
use crate::domain::errors::infra_error::InfraError;
use crate::domain::models::db::phone_row::PhoneRow;
use crate::domain::ports::repositories::phone_repository::*;

pub struct PgPhoneRepository {
    pool: PgPool,
}

impl PgPhoneRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_with_executor<'a, E>(
        executor: E,
        contact_id: Uuid,
        phone: String,
    ) -> Result<PhoneRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query_as!(
            PhoneRow,
            r#"
            INSERT INTO contacts.tb_phone (pk_phone, tx_phone, fk_contact)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &phone,
            &contact_id,
        )
        .fetch_one(executor)
        .await
    }

    pub async fn create_many_with_executor<'a, E>(
        executor: E,
        contact_id: Uuid,
        phones: Vec<String>,
    ) -> Result<Vec<PhoneRow>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let pks: Vec<Uuid> = phones.iter().map(|_| Uuid::now_v7()).collect();
        let fks: Vec<Uuid> = std::iter::repeat_n(contact_id, phones.len()).collect();

        sqlx::query_as!(
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
        .await
    }
}

fn sqlx_err(action: &'static str) -> impl FnOnce(sqlx::Error) -> ContactError {
    move |e| ContactError::Infra(InfraError::Database { action, source: e })
}

#[async_trait]
impl FindPhoneById for PgPhoneRepository {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<PhoneRow>, ContactError> {
        sqlx::query_as!(
            PhoneRow,
            r#"
            SELECT *
            FROM contacts.tb_phone
            WHERE pk_phone = $1
            "#,
            &uuid,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_err("find phone by id"))
    }
}

#[async_trait]
impl FindPhoneByContactId for PgPhoneRepository {
    async fn find_by_contact_id(&self, contact_id: Uuid) -> Result<Vec<PhoneRow>, ContactError> {
        sqlx::query_as!(
            PhoneRow,
            r#"
            SELECT *
            FROM contacts.tb_phone
            WHERE fk_contact = $1
            "#,
            &contact_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(sqlx_err("find phones by contact"))
    }
}

#[async_trait]
impl CreatePhone for PgPhoneRepository {
    async fn create(&self, contact_id: Uuid, phone: String) -> Result<PhoneRow, ContactError> {
        sqlx::query_as!(
            PhoneRow,
            r#"
            INSERT INTO contacts.tb_phone (pk_phone, tx_phone, fk_contact)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &phone,
            &contact_id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_err("create phone"))
    }
}

#[async_trait]
impl CreateManyPhones for PgPhoneRepository {
    async fn create_many(
        &self,
        contact_id: Uuid,
        phones: Vec<String>,
    ) -> Result<Vec<PhoneRow>, ContactError> {
        let pks: Vec<Uuid> = phones.iter().map(|_| Uuid::now_v7()).collect();
        let fks: Vec<Uuid> = std::iter::repeat_n(contact_id, phones.len()).collect();

        sqlx::query_as!(
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
        .fetch_all(&self.pool)
        .await
        .map_err(sqlx_err("create phones"))
    }
}

#[async_trait]
impl UpdatePhone for PgPhoneRepository {
    async fn update(&self, uuid: Uuid, phone: String) -> Result<PhoneRow, ContactError> {
        sqlx::query_as!(
            PhoneRow,
            r#"
            UPDATE contacts.tb_phone
            SET tx_phone = $1
            WHERE pk_phone = $2
            RETURNING *
            "#,
            &phone,
            &uuid,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_err("update phone"))
    }
}

#[async_trait]
impl DeletePhone for PgPhoneRepository {
    async fn delete(&self, uuid: Uuid) -> Result<PhoneRow, ContactError> {
        sqlx::query_as!(
            PhoneRow,
            r#"
            DELETE FROM contacts.tb_phone
            WHERE pk_phone = $1
            RETURNING *
            "#,
            &uuid,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_err("remove phone"))
    }
}

#[async_trait]
impl FindNonexistentPhones for PgPhoneRepository {
    async fn find_nonexistent_phones(
        &self,
        phones: Vec<String>,
    ) -> Result<Vec<String>, ContactError> {
        let r = sqlx::query_scalar!(
            r#"SELECT input.tx_phone
            FROM UNNEST($1::text[]) AS input(tx_phone)
            LEFT JOIN contacts.tb_phone p ON p.tx_phone = input.tx_phone
            WHERE p.tx_phone IS NULL"#,
            &phones as &[String],
        )
        .fetch_all(&self.pool)
        .await
        .map_err(sqlx_err("check nonexistent phones"))?
        .iter()
        .filter_map(|p| p.clone())
        .collect();

        Ok(r)
    }
}

impl FindAndCreatePhone for PgPhoneRepository {}
impl FindAndUpdatePhone for PgPhoneRepository {}
impl FindAndDeletePhone for PgPhoneRepository {}
