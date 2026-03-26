use domain::models::db::client_projects_row::{
    CreateProjectDailyAllocationRow, ProjectDailyAllocationRow, UpdateProjectDailyAllocationRow,
};

pub struct ProjectDailyAllocationMutation;

impl ProjectDailyAllocationMutation {
    /// Cria uma alocação diária em `clients.tb_project_daily_allocation`.
    pub async fn create<'a, E>(
        executor: E,
        c: CreateProjectDailyAllocationRow,
    ) -> Result<ProjectDailyAllocationRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ProjectDailyAllocationRow,
            r#"
            INSERT INTO clients.tb_project_daily_allocation (
                pk_project_daily_allocation,
                fk_project,
                fk_collaborator,
                dt_work_date,
                nr_hours_worked,
                nr_hourly_rate_snapshot,
                tx_notes,
                bl_present
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
            c.fk_collaborator,
            c.dt_work_date,
            c.nr_hours_worked,
            c.nr_hourly_rate_snapshot,
            c.tx_notes,
            c.bl_present,
        )
        .fetch_one(executor)
        .await?;

        Ok(r)
    }

    /// Atualiza uma alocação diária em `clients.tb_project_daily_allocation`.
    pub async fn update<'a, E>(
        executor: E,
        uuid: uuid::Uuid,
        u: UpdateProjectDailyAllocationRow,
    ) -> Result<ProjectDailyAllocationRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ProjectDailyAllocationRow,
            r#"
            UPDATE clients.tb_project_daily_allocation
            SET
            nr_hours_worked = $1,
            nr_hourly_rate_snapshot = $2,
            tx_notes = $3,
            bl_present = $4
            WHERE pk_project_daily_allocation = $5
            RETURNING *
            "#,
            u.nr_hours_worked,
            u.nr_hourly_rate_snapshot,
            u.tx_notes,
            u.bl_present,
            uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(r)
    }
}
