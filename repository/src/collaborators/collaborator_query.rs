use uuid::Uuid;

use crate::collaborators::models::collaborator::Collaborator;

pub struct CollaboratorQuery;

impl CollaboratorQuery {
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
