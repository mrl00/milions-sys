use domain::models::db::{
    contact_row::{ContactRow, CreateContactRow},
    phone_row::PhoneRow,
};
use uuid::Uuid;

use crate::contacts::commands::phone_mutation::PhoneMutation;

pub struct ContactMutation;

impl ContactMutation {
    /// Cria um contato em `contacts.tb_contact`.
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

    /// Atualiza `tx_email` de um contato em `contacts.tb_contact`.
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

    /// Cria e associa um telefone a um contato (via `PhoneMutation::create`).
    pub async fn add_phone<'a, E>(
        executor: E,
        contact_uuid: Uuid,
        phone: String,
    ) -> Result<PhoneRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        PhoneMutation::create(executor, contact_uuid, phone).await
    }
}
