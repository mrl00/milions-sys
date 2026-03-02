use uuid::Uuid;

use crate::collaborators::models::collaborator::Collaborator;

pub struct CollaboratorQuery;

impl CollaboratorQuery {
    pub async fn get_by_uuid<'a, E>(
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
            &uuid,
        )
        .fetch_optional(executor)
        .await?;

        Ok(collaborator)
    }
    pub async fn get_by_name<'a, E>(
        executor: E,
        name: String,
    ) -> Result<Option<Collaborator>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
    {
        let collaborator = sqlx::query_as!(
            Collaborator,
            r#"
            SELECT *
            FROM collaborators.tb_collaborator
            WHERE tx_name = $1
            "#,
            &name,
        )
        .fetch_optional(executor)
        .await?;

        Ok(collaborator)
    }
}
