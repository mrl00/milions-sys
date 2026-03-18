use crate::projects::models::client_projects::{Project, ProjectStatus, UpdateProject};

pub struct ProjectMutation;

impl ProjectMutation {
    /// Cria um projeto de cliente em `clients.tb_project`.
    pub async fn create<'a, E>(
        executor: E,
        client_uuid: uuid::Uuid,
        location_uuid: uuid::Uuid,
    ) -> Result<Project, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            Project,
            r#"
            INSERT INTO clients.tb_project (pk_project, fk_client, fk_address)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            uuid::Uuid::now_v7(),
            &client_uuid,
            &location_uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(r)
    }

    pub async fn update<'a, E>(
        executor: E,
        uuid: uuid::Uuid,
        u: UpdateProject,
    ) -> Result<Project, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            Project,
            r#"
            UPDATE clients.tb_project
            SET tx_name = $1, tx_status = $2
            WHERE pk_project = $3
            RETURNING *
            "#,
            u.tx_name,
            u.tx_status
                .map_or_else(|| "inactive".to_string(), |s| s.to_string()),
            uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(r)
    }

    /// Marca um projeto como inativo (`tx_status = Inactive`).
    pub async fn pause<'a, E>(executor: E, uuid: uuid::Uuid) -> Result<Project, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ProjectMutation::update_status(executor, uuid, ProjectStatus::Paused).await
    }

    /// Marca um projeto como parado (`tx_status = Stopped`).
    pub async fn cancel<'a, E>(executor: E, uuid: uuid::Uuid) -> Result<Project, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ProjectMutation::update_status(executor, uuid, ProjectStatus::Cancelled).await
    }

    /// Marca um projeto como em andamento (`tx_status = InProgress`).
    pub async fn start<'a, E>(executor: E, uuid: uuid::Uuid) -> Result<Project, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ProjectMutation::update_status(executor, uuid, ProjectStatus::InProgress).await
    }

    /// Marca um projeto como concluído (`tx_status = Done`).
    pub async fn done<'a, E>(executor: E, uuid: uuid::Uuid) -> Result<Project, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ProjectMutation::update_status(executor, uuid, ProjectStatus::Completed).await
    }

    /// Atualiza `tx_status` de um projeto.
    async fn update_status<'a, E>(
        executor: E,
        uuid: uuid::Uuid,
        status: ProjectStatus,
    ) -> Result<Project, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            Project,
            r#"
            UPDATE clients.tb_project
            SET tx_status = $1
            WHERE pk_project = $2
            RETURNING *
            "#,
            &status.to_string(),
            &uuid,
        )
        .fetch_one(executor)
        .await?;
        Ok(r)
    }
}
