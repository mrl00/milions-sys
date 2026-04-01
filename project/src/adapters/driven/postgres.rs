use crate::domain::model::{
    CreateProjectStageRow, ProjectRow, ProjectStageRow, ProjectStageStatus, ProjectStatus,
    UpdateProjectRow, UpdateProjectStageRow,
};
use sqlx::PgPool;
use uuid::Uuid;

pub struct PgProjectRepository {
    pool: PgPool,
}

impl PgProjectRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create<'a, E>(
        executor: E,
        client_uuid: Uuid,
        location_uuid: Uuid,
    ) -> Result<ProjectRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ProjectRow,
            r#"
            INSERT INTO clients.tb_project (pk_project, fk_client, fk_address)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &client_uuid,
            &location_uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(r)
    }

    pub async fn update<'a, E>(
        executor: E,
        uuid: Uuid,
        u: UpdateProjectRow,
    ) -> Result<ProjectRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ProjectRow,
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

    pub async fn pause<'a, E>(executor: E, uuid: Uuid) -> Result<ProjectRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        Self::update_status(executor, uuid, ProjectStatus::Paused).await
    }

    pub async fn cancel<'a, E>(executor: E, uuid: Uuid) -> Result<ProjectRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        Self::update_status(executor, uuid, ProjectStatus::Cancelled).await
    }

    pub async fn start<'a, E>(executor: E, uuid: Uuid) -> Result<ProjectRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        Self::update_status(executor, uuid, ProjectStatus::InProgress).await
    }

    pub async fn done<'a, E>(executor: E, uuid: Uuid) -> Result<ProjectRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        Self::update_status(executor, uuid, ProjectStatus::Completed).await
    }

    async fn update_status<'a, E>(
        executor: E,
        uuid: Uuid,
        status: ProjectStatus,
    ) -> Result<ProjectRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ProjectRow,
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

pub struct ProjectStageRepositoryImpl {
    pool: PgPool,
}

impl ProjectStageRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_stage<'a, E>(
        executor: E,
        c: CreateProjectStageRow,
    ) -> Result<ProjectStageRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ProjectStageRow,
            r#"
            INSERT INTO clients.tb_project_stage (pk_project_stage, fk_project, tx_name, tx_description, nr_order, tx_status, dt_start_date, dt_end_date)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &c.fk_project,
            &c.tx_name,
            c.tx_description,
            &c.nr_order,
            &c.tx_status.to_string(),
            c.dt_start_date,
            c.dt_end_date,
        )
        .fetch_one(executor)
        .await?;

        Ok(r)
    }

    pub async fn update<'a, E>(
        executor: E,
        uuid: Uuid,
        u: UpdateProjectStageRow,
    ) -> Result<ProjectStageRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ProjectStageRow,
            r#"
            UPDATE clients.tb_project_stage
            SET tx_name = $1, tx_description = $2, nr_order = $3, tx_status = $4, dt_start_date = $5, dt_end_date = $6
            WHERE pk_project_stage = $7
            RETURNING *
            "#,
            u.tx_name,
            u.tx_description,
            u.nr_order,
            u.tx_status.map_or_else(|| "pending".to_string(), |s| s.to_string()),
            u.dt_start_date,
            u.dt_end_date,
            uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(r)
    }

    pub async fn start<'a, E>(executor: E, uuid: Uuid) -> Result<ProjectStageRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        Self::update_status(executor, uuid, ProjectStageStatus::InProgress).await
    }

    pub async fn done<'a, E>(executor: E, uuid: Uuid) -> Result<ProjectStageRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        Self::update_status(executor, uuid, ProjectStageStatus::Completed).await
    }

    pub async fn skip<'a, E>(executor: E, uuid: Uuid) -> Result<ProjectStageRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        Self::update_status(executor, uuid, ProjectStageStatus::Skipped).await
    }

    async fn update_status<'a, E>(
        executor: E,
        uuid: Uuid,
        status: ProjectStageStatus,
    ) -> Result<ProjectStageRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ProjectStageRow,
            r#"
            UPDATE clients.tb_project_stage
            SET tx_status = $1
            WHERE pk_project_stage = $2
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
