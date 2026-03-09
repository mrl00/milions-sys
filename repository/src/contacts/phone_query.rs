use uuid::Uuid;

use crate::contacts::models::phone::Phone;

pub struct PhoneQuery;

impl PhoneQuery {
    /// Busca um telefone pelo seu identificador (`pk_phone`) na tabela `contacts.tb_phone`.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar a query.
    /// - **uuid**: identificador UUID do telefone.
    ///
    /// Retorna `Ok(Some(Phone))` quando encontrado, `Ok(None)` quando não houver registro
    /// correspondente e `Err(sqlx::Error)` em caso de erro de banco.
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

    /// Lista todos os telefones associados a um contato específico (`fk_contact`).
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar a query.
    /// - **contact**: identificador UUID do contato.
    ///
    /// Retorna um vetor com todos os telefones do contato ou erro de banco.
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

    /// A partir de uma lista de números de telefone, retorna quais **não** existem
    /// na tabela `contacts.tb_phone`.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar a query.
    /// - **phones**: lista de números de telefone que serão verificados.
    ///
    /// Retorna um vetor apenas com os números ausentes no banco.
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
