use uuid::Uuid;

use crate::clients::models::projects;

pub struct ProjectMutation;

impl ProjectMutation {
    pub async fn create<'a, E>(
        executor: E,
        project: projects::CreateProject,
    ) -> Result<projects::Project, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
    {
        let created_project = sqlx::query_as!(
            projects::Project,
            r#"
            INSERT INTO clients.tb_project (pk_project, tx_name, tx_status, fk_address, fk_client)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &project.tx_name,
            &project.tx_status.to_string(),
            &project.fk_address,
            &project.fk_client,
        )
        .fetch_one(executor)
        .await?;

        Ok(created_project)
    }

    pub async fn update_status(
        pool: &sqlx::PgPool,
        uuid: Uuid,
        status: projects::ProjectStatus,
    ) -> Result<projects::Project, sqlx::Error> {
        let updated_project = sqlx::query_as!(
            projects::Project,
            r#"
            UPDATE clients.tb_project
            SET tx_status = $1
            WHERE pk_project = $2
            RETURNING *
            "#,
            &status.to_string(),
            &uuid,
        )
        .fetch_one(pool)
        .await?;

        Ok(updated_project)
    }
}
