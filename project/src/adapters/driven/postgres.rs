use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::errors::ProjectError;
use crate::domain::models::db::project_rows::{CreateProjectRow, ProjectRow, UpdateProjectRow};
use crate::domain::ports::project_repository::*;
use types::errors::infra_error::InfraError;

pub struct PgProjectRepository {
    pool: PgPool,
}

impl PgProjectRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn sqlx_err(action: &'static str) -> impl FnOnce(sqlx::Error) -> ProjectError {
    move |e| ProjectError::Infra {
        source: InfraError::Database { action, source: e },
    }
}

#[async_trait]
impl FindProjectById for PgProjectRepository {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<ProjectRow>, ProjectError> {
        sqlx::query_as!(
            ProjectRow,
            r#"
            SELECT *
            FROM clients.tb_project
            WHERE pk_project = $1
            "#,
            &uuid,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_err("buscar projeto por id"))
    }
}

#[async_trait]
impl FindProjectByClientId for PgProjectRepository {
    async fn find_by_client_id(&self, client_id: Uuid) -> Result<Vec<ProjectRow>, ProjectError> {
        sqlx::query_as!(
            ProjectRow,
            r#"
            SELECT *
            FROM clients.tb_project
            WHERE fk_client = $1
            "#,
            &client_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(sqlx_err("buscar projetos por cliente"))
    }
}

#[async_trait]
impl FindAllProjects for PgProjectRepository {
    async fn find_all(&self) -> Result<Vec<ProjectRow>, ProjectError> {
        sqlx::query_as!(
            ProjectRow,
            r#"
            SELECT *
            FROM clients.tb_project
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(sqlx_err("listar projetos"))
    }
}

#[async_trait]
impl CreateProject for PgProjectRepository {
    async fn create(&self, input: CreateProjectRow) -> Result<ProjectRow, ProjectError> {
        sqlx::query_as!(
            ProjectRow,
            r#"
            INSERT INTO clients.tb_project (
                pk_project, tx_name, tx_description, tx_status,
                dt_start_date, dt_estimated_end_date,
                nr_total_area_m2, nr_estimated_cost, tx_notes,
                fk_client, fk_address
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &input.tx_name,
            input.tx_description,
            &input.tx_status.to_string(),
            input.dt_start_date,
            input.dt_estimated_end_date,
            input.nr_total_area_m2,
            input.nr_estimated_cost,
            input.tx_notes,
            &input.fk_client,
            &input.fk_address,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_err("criar projeto"))
    }
}

#[async_trait]
impl UpdateProject for PgProjectRepository {
    async fn update(&self, uuid: Uuid, u: UpdateProjectRow) -> Result<ProjectRow, ProjectError> {
        sqlx::query_as!(
            ProjectRow,
            r#"
            UPDATE clients.tb_project
            SET tx_name = COALESCE($1, tx_name),
                tx_description = COALESCE($2, tx_description),
                tx_status = COALESCE($3, tx_status),
                dt_start_date = COALESCE($4, dt_start_date),
                dt_estimated_end_date = COALESCE($5, dt_estimated_end_date),
                dt_actual_end_date = COALESCE($6, dt_actual_end_date),
                nr_total_area_m2 = COALESCE($7, nr_total_area_m2),
                nr_estimated_cost = COALESCE($8, nr_estimated_cost),
                nr_actual_cost = COALESCE($9, nr_actual_cost),
                tx_notes = COALESCE($10, tx_notes),
                bl_active = COALESCE($11, bl_active)
            WHERE pk_project = $12
            RETURNING *
            "#,
            u.tx_name,
            u.tx_description,
            u.tx_status.map(|s| s.to_string()),
            u.dt_start_date,
            u.dt_estimated_end_date,
            u.dt_actual_end_date,
            u.nr_total_area_m2,
            u.nr_estimated_cost,
            u.nr_actual_cost,
            u.tx_notes,
            u.bl_active,
            &uuid,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_err("atualizar projeto"))
    }
}

#[async_trait]
impl DeleteProject for PgProjectRepository {
    async fn delete(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError> {
        sqlx::query_as!(
            ProjectRow,
            r#"
            DELETE FROM clients.tb_project
            WHERE pk_project = $1
            RETURNING *
            "#,
            &uuid,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_err("remover projeto"))
    }
}

impl FindAndCreateProject for PgProjectRepository {}
impl FindAndUpdateProject for PgProjectRepository {}
impl FindAndDeleteProject for PgProjectRepository {}
