use sqlx::types::BigDecimal;

use domain::models::db::client_projects::{
    CreateProjectServiceTypeRow, ProjectServiceTypeRow, UpdateProjectServiceTypeRow,
};

pub struct ProjectTypeMutation;

impl ProjectTypeMutation {
    /// Cria um tipo de projeto em `clients.tb_service_type`.
    pub async fn create<'a, E>(
        executor: E,
        c: CreateProjectServiceTypeRow,
    ) -> Result<ProjectServiceTypeRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ProjectServiceTypeRow,
            r#"
            INSERT INTO clients.tb_service_type (pk_service_type, tx_name, tx_description, tx_unit, nr_default_unit_price)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
            uuid::Uuid::now_v7(),
            &c.tx_name,
            c.tx_description,
            c.tx_unit.to_string(),
            &c.nr_default_unit_price.map_or_else(|| BigDecimal::from(0), |d| d),
        )
        .fetch_one(executor)
        .await?;

        Ok(r)
    }

    pub async fn update<'a, E>(
        executor: E,
        uuid: uuid::Uuid,
        u: UpdateProjectServiceTypeRow,
    ) -> Result<ProjectServiceTypeRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ProjectServiceTypeRow,
            r#"
            UPDATE clients.tb_service_type
            SET tx_name = $1, tx_description = $2, tx_unit = $3, nr_default_unit_price = $4
            WHERE pk_service_type = $5
            RETURNING *
            "#,
            u.tx_name,
            u.tx_description,
            u.tx_unit
                .map_or_else(|| "m2".to_string(), |s| s.to_string()),
            u.nr_default_unit_price,
            uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(r)
    }
}
