use uuid::Uuid;

use crate::contacts::models::contact::Contact;

pub struct ContactQuery;

impl ContactQuery {
    /// Busca um contato pelo seu identificador (`pk_contact`) na tabela `contacts.tb_contact`.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar a query.
    /// - **uuid**: identificador UUID do contato.
    ///
    /// Retorna `Ok(Some(Contact))` quando encontrado, `Ok(None)` quando não houver registro
    /// correspondente e `Err(sqlx::Error)` em caso de erro de banco.
    pub async fn get_by_uuid<'a, E>(executor: E, uuid: Uuid) -> Result<Option<Contact>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let contact = sqlx::query_as!(
            Contact,
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

    /// Busca um contato pelo e‑mail (`tx_email`) na tabela `contacts.tb_contact`.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar a query.
    /// - **email**: endereço de e‑mail a ser procurado.
    ///
    /// Retorna `Ok(Some(Contact))` quando encontrado, `Ok(None)` quando não houver registro
    /// correspondente e `Err(sqlx::Error)` em caso de erro de banco.
    pub async fn get_by_email<'a, E>(
        executor: E,
        email: String,
    ) -> Result<Option<Contact>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let contact = sqlx::query_as!(
            Contact,
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

    /// Lista todos os contatos cadastrados em `contacts.tb_contact`, ordenados por `idx_contact`.
    ///
    /// - **pool**: pool de conexões Postgres usado para executar a consulta.
    ///
    /// Retorna um vetor com todos os contatos ou erro de banco.
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
