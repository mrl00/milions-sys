use uuid::Uuid;

use crate::collaborators::models::collaborator::Collaborator;

pub struct CollaboratorQuery;

impl CollaboratorQuery {
    /// Busca um colaborador pelo seu identificador (`pk_collaborator`)
    /// na tabela `collaborators.tb_collaborator`.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar a query.
    /// - **uuid**: identificador UUID do colaborador.
    ///
    /// Retorna `Ok(Some(Collaborator))` quando encontrado, `Ok(None)` quando não houver registro
    /// correspondente e `Err(sqlx::Error)` em caso de erro de banco.
    pub async fn find_by_uuid<'a, E>(
        executor: E,
        uuid: Uuid,
    ) -> Result<Option<Collaborator>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
    {
        let collaborator = sqlx::query_as!(
            Collaborator,
            r#"
            SELECT *
            FROM collaborators.tb_collaborator
            WHERE pk_collaborator = $1
            "#,
            &uuid
        )
        .fetch_optional(executor)
        .await?;

        Ok(collaborator)
    }

    /// Busca um colaborador pelo CPF (`tx_cpf`) na tabela `collaborators.tb_collaborator`.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar a query.
    /// - **cpf**: CPF do colaborador em formato texto.
    ///
    /// Retorna `Ok(Some(Collaborator))` quando encontrado, `Ok(None)` quando não houver registro
    /// correspondente e `Err(sqlx::Error)` em caso de erro de banco.
    pub async fn find_by_cpf<'a, E>(
        executor: E,
        cpf: String,
    ) -> Result<Option<Collaborator>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
    {
        let collaborator = sqlx::query_as!(
            Collaborator,
            r#"
            SELECT *
            FROM collaborators.tb_collaborator
            WHERE tx_cpf = $1
            "#,
            &cpf
        )
        .fetch_optional(executor)
        .await?;

        Ok(collaborator)
    }
}
