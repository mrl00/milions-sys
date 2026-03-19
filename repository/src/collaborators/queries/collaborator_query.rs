use uuid::Uuid;

use crate::collaborators::models::collaborator::CollaboratorModel;

pub struct CollaboratorQuery;

impl CollaboratorQuery {
    /// Obtém um colaborador por `pk_collaborator` em `collaborators.tb_collaborator`.
    pub async fn find_by_uuid<'a, E>(
        executor: E,
        uuid: Uuid,
    ) -> Result<Option<CollaboratorModel>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let collaborator = sqlx::query_as!(
            CollaboratorModel,
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

    /// Obtém um colaborador por `tx_cpf` em `collaborators.tb_collaborator`.
    pub async fn find_by_cpf<'a, E>(
        executor: E,
        cpf: String,
    ) -> Result<Option<CollaboratorModel>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let collaborator = sqlx::query_as!(
            CollaboratorModel,
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
