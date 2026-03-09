use uuid::Uuid;

use crate::contacts::{
    models::{
        contact::{Contact, CreateContact},
        phone::Phone,
    },
    phone_mutation::PhoneMutation,
};

pub struct ContactMutation;

impl ContactMutation {
    /// Cria um novo contato na tabela `contacts.tb_contact`.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `INSERT`.
    /// - **contact**: dados para criação (`CreateContact`), incluindo e‑mail.
    ///
    /// Gera um novo `pk_contact` (`Uuid::now_v7()`), insere o registro e retorna o contato criado.
    pub async fn create<'a, E>(executor: E, contact: CreateContact) -> Result<Contact, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
    {
        let created_contact = sqlx::query_as!(
            Contact,
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

    /// Atualiza o e‑mail (`tx_email`) de um contato existente.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `UPDATE`.
    /// - **uuid**: identificador UUID do contato a ser atualizado.
    /// - **email**: novo endereço de e‑mail.
    ///
    /// Retorna o contato atualizado ou erro de banco.
    pub async fn update_email<'a, E>(
        executor: E,
        uuid: Uuid,
        email: String,
    ) -> Result<Contact, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
    {
        let updated_contact = sqlx::query_as!(
            Contact,
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

    /// Cria e associa um telefone a um contato, delegando para `PhoneMutation::create`.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `INSERT`.
    /// - **contact_uuid**: identificador UUID do contato.
    /// - **phone**: número de telefone a ser cadastrado.
    ///
    /// Retorna o telefone criado e associado ao contato, ou erro de banco.
    pub async fn add_phone<'a, E>(
        executor: E,
        contact_uuid: Uuid,
        phone: String,
    ) -> Result<Phone, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
    {
        PhoneMutation::create(executor, contact_uuid, phone).await
    }
}
