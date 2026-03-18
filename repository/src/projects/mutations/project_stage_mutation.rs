use crate::projects::models::client_projects::{
    CreateProjectStage, ProjectStage, ProjectStageStatus, UpdateProjectStage,
};

pub struct ProjectStageMutation;

impl ProjectStageMutation {
    /// Cria um projeto de cliente em `clients.tb_project`.
    pub async fn create_stage<'a, E>(
        executor: E,
        c: CreateProjectStage,
    ) -> Result<ProjectStage, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ProjectStage,
            r#"
            INSERT INTO clients.tb_project_stage (pk_project_stage, fk_project, tx_name, tx_description, nr_order, tx_status, dt_start_date, dt_end_date)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
            uuid::Uuid::now_v7(),
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
        uuid: uuid::Uuid,
        u: UpdateProjectStage,
    ) -> Result<ProjectStage, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ProjectStage,
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

    /// Marca um projeto como em andamento (`tx_status = InProgress`).
    pub async fn start<'a, E>(executor: E, uuid: uuid::Uuid) -> Result<ProjectStage, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ProjectStageMutation::update_status(executor, uuid, ProjectStageStatus::InProgress).await
    }

    /// Marca um projeto como concluído (`tx_status = Completed`).
    pub async fn done<'a, E>(executor: E, uuid: uuid::Uuid) -> Result<ProjectStage, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ProjectStageMutation::update_status(executor, uuid, ProjectStageStatus::Completed).await
    }

    /// Marca um projeto como parado (`tx_status = Skipped`).
    pub async fn skip<'a, E>(executor: E, uuid: uuid::Uuid) -> Result<ProjectStage, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ProjectStageMutation::update_status(executor, uuid, ProjectStageStatus::Skipped).await
    }

    /// Atualiza `tx_status` de um projeto.
    async fn update_status<'a, E>(
        executor: E,
        uuid: uuid::Uuid,
        status: ProjectStageStatus,
    ) -> Result<ProjectStage, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ProjectStage,
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
