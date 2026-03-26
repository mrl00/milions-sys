use domain::models::db::client_projects::{
    CreateProjectServiceRow, ProjectServiceRow, ProjectServiceStatus, UpdateProjectServiceRow,
};

pub struct ProjectServiceMutation;

impl ProjectServiceMutation {
    /// Cria um tipo de projeto em `clients.tb_service_type`.
    pub async fn create<'a, E>(
        executor: E,
        c: CreateProjectServiceRow,
    ) -> Result<ProjectServiceRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ProjectServiceRow,
            r#"
            INSERT INTO clients.tb_project_service (
                pk_project_service,
                fk_project,
                fk_project_stage,
                fk_service_type,
                tx_description,
                nr_quantity,
                nr_unit_price,
                tx_status
            )
            VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6,
                $7,
                $8
            )
            RETURNING *
            "#,
            uuid::Uuid::now_v7(),
            c.fk_project,
            c.fk_project_stage,
            c.fk_service_type,
            c.tx_description,
            &c.nr_quantity,
            &c.nr_unit_price,
            c.tx_status.to_string(),
        )
        .fetch_one(executor)
        .await?;

        Ok(r)
    }

    pub async fn update<'a, E>(
        executor: E,
        uuid: uuid::Uuid,
        u: UpdateProjectServiceRow,
    ) -> Result<ProjectServiceRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ProjectServiceRow,
            r#"
            UPDATE clients.tb_project_service
            SET 
            fk_project_stage = $1, 
            tx_description = $2,
            nr_quantity = $3, 
            nr_unit_price = $4, 
            tx_status = $5
            WHERE pk_project_service = $6
            RETURNING *
            "#,
            u.fk_project_stage,
            u.tx_description,
            u.nr_quantity,
            u.nr_unit_price,
            u.tx_status
                .map_or_else(|| "pending".to_string(), |s| s.to_string()),
            uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(r)
    }

    /// Marca um serviço como pendente (`tx_status = Pending`).
    pub async fn pending<'a, E>(
        executor: E,
        uuid: uuid::Uuid,
    ) -> Result<ProjectServiceRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ProjectServiceMutation::update_status(executor, uuid, ProjectServiceStatus::Pending).await
    }

    /// Marca um serviço como em andamento (`tx_status = InProgress`).
    pub async fn in_progress<'a, E>(
        executor: E,
        uuid: uuid::Uuid,
    ) -> Result<ProjectServiceRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ProjectServiceMutation::update_status(executor, uuid, ProjectServiceStatus::InProgress)
            .await
    }

    /// Marca um serviço como concluído (`tx_status = Completed`).
    pub async fn completed<'a, E>(
        executor: E,
        uuid: uuid::Uuid,
    ) -> Result<ProjectServiceRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ProjectServiceMutation::update_status(executor, uuid, ProjectServiceStatus::Completed).await
    }

    async fn update_status<'a, E>(
        executor: E,
        uuid: uuid::Uuid,
        status: ProjectServiceStatus,
    ) -> Result<ProjectServiceRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ProjectServiceRow,
            r#"
            UPDATE clients.tb_project_service
            SET tx_status = $1
            WHERE pk_project_service = $2
            RETURNING *
            "#,
            status.to_string(),
            uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(r)
    }
}
