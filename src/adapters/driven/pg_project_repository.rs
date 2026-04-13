use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::errors::infra_error::InfraError;
use crate::domain::errors::project_error::ProjectError;
use crate::domain::models::db::project_rows::{
    AllocationWithProjectName, CreateProjectDailyAllocationRow, CreateProjectRow,
    CreateProjectStageRow, ProjectDailyAllocationRow, ProjectRow, ProjectStageRow,
    UpdateProjectDailyAllocationRow, UpdateProjectRow, UpdateProjectStageRow,
};
use crate::domain::ports::project_repository::*;

pub struct PgProjectRepository {
    pool: PgPool,
}

impl PgProjectRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn db_err(action: &'static str, e: sqlx::Error) -> ProjectError {
    InfraError::Database { action, source: e }.into()
}

#[async_trait]
impl FindProjectById for PgProjectRepository {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<ProjectRow>, ProjectError> {
        sqlx::query_as!(
            ProjectRow,
            r#"
            SELECT *
            FROM project.tb_project
            WHERE pk_project = $1
            "#,
            &uuid,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| db_err("find project by id", e))
    }
}

#[async_trait]
impl FindAllProjects for PgProjectRepository {
    async fn find_all(&self) -> Result<Vec<ProjectRow>, ProjectError> {
        sqlx::query_as!(
            ProjectRow,
            r#"
            SELECT *
            FROM project.tb_project
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| db_err("list projects", e))
    }
}

#[async_trait]
impl CreateProject for PgProjectRepository {
    async fn create(&self, input: CreateProjectRow) -> Result<ProjectRow, ProjectError> {
        sqlx::query_as!(
            ProjectRow,
            r#"
            INSERT INTO project.tb_project (
                pk_project, tx_name, tx_description, tx_status,
                dt_start_date, dt_estimated_end_date,
                nr_total_area_m2, nr_estimated_cost, tx_notes,
                fk_address
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
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
            &input.fk_address,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| db_err("create project", e))
    }
}

#[async_trait]
impl UpdateProject for PgProjectRepository {
    async fn update(&self, uuid: Uuid, u: UpdateProjectRow) -> Result<ProjectRow, ProjectError> {
        sqlx::query_as!(
            ProjectRow,
            r#"
            UPDATE project.tb_project
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
        .map_err(|e| db_err("update project", e))
    }
}

#[async_trait]
impl DeleteProject for PgProjectRepository {
    async fn delete(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError> {
        sqlx::query_as!(
            ProjectRow,
            r#"
            DELETE FROM project.tb_project
            WHERE pk_project = $1
            RETURNING *
            "#,
            &uuid,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| db_err("remove project", e))
    }
}

#[async_trait]
impl FindStageById for PgProjectRepository {
    async fn find_stage_by_id(&self, uuid: Uuid) -> Result<Option<ProjectStageRow>, ProjectError> {
        sqlx::query_as!(
            ProjectStageRow,
            r#"
            SELECT *
            FROM project.tb_project_stage
            WHERE pk_project_stage = $1
            "#,
            &uuid,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| db_err("find stage by id", e))
    }
}

#[async_trait]
impl CreateStage for PgProjectRepository {
    async fn create_stage(
        &self,
        input: CreateProjectStageRow,
    ) -> Result<ProjectStageRow, ProjectError> {
        sqlx::query_as!(
            ProjectStageRow,
            r#"
            INSERT INTO project.tb_project_stage (
                pk_project_stage, fk_project, tx_name, tx_description,
                nr_order, tx_status, dt_start_date, dt_end_date
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &input.fk_project,
            &input.tx_name,
            input.tx_description,
            input.nr_order,
            &input.tx_status.to_string(),
            input.dt_start_date,
            input.dt_end_date,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| db_err("create stage", e))
    }
}

#[async_trait]
impl UpdateStage for PgProjectRepository {
    async fn update_stage(
        &self,
        uuid: Uuid,
        u: UpdateProjectStageRow,
    ) -> Result<ProjectStageRow, ProjectError> {
        sqlx::query_as!(
            ProjectStageRow,
            r#"
            UPDATE project.tb_project_stage
            SET tx_name = COALESCE($1, tx_name),
                tx_description = COALESCE($2, tx_description),
                nr_order = COALESCE($3, nr_order),
                tx_status = COALESCE($4, tx_status),
                dt_start_date = COALESCE($5, dt_start_date),
                dt_end_date = COALESCE($6, dt_end_date)
            WHERE pk_project_stage = $7
            RETURNING *
            "#,
            u.tx_name,
            u.tx_description,
            u.nr_order,
            u.tx_status.map(|s| s.to_string()),
            u.dt_start_date,
            u.dt_end_date,
            &uuid,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| db_err("update stage", e))
    }
}

#[async_trait]
impl FindAllocationById for PgProjectRepository {
    async fn find_allocation_by_id(
        &self,
        uuid: Uuid,
    ) -> Result<Option<ProjectDailyAllocationRow>, ProjectError> {
        sqlx::query_as!(
            ProjectDailyAllocationRow,
            r#"
            SELECT *
            FROM project.tb_project_daily_allocation
            WHERE pk_project_daily_allocation = $1
            "#,
            &uuid,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| db_err("find allocation by id", e))
    }
}

#[async_trait]
impl FindAllocationsByProjectId for PgProjectRepository {
    async fn find_allocations_by_project_id(
        &self,
        project_id: Uuid,
    ) -> Result<Vec<ProjectDailyAllocationRow>, ProjectError> {
        sqlx::query_as!(
            ProjectDailyAllocationRow,
            r#"
            SELECT *
            FROM project.tb_project_daily_allocation
            WHERE fk_project = $1
            ORDER BY dt_work_date DESC
            "#,
            &project_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| db_err("list allocations by project", e))
    }
}

#[async_trait]
impl CreateAllocation for PgProjectRepository {
    async fn create_allocation(
        &self,
        input: CreateProjectDailyAllocationRow,
    ) -> Result<ProjectDailyAllocationRow, ProjectError> {
        sqlx::query_as!(
            ProjectDailyAllocationRow,
            r#"
            INSERT INTO project.tb_project_daily_allocation (
                pk_project_daily_allocation, fk_project, fk_collaborator,
                dt_work_date, nr_hours_worked, nr_hourly_rate_snapshot,
                tx_notes, bl_present
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &input.fk_project,
            &input.fk_collaborator,
            input.dt_work_date,
            input.nr_hours_worked,
            input.nr_hourly_rate_snapshot,
            input.tx_notes,
            input.bl_present,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| db_err("create allocation", e))
    }
}

#[async_trait]
impl UpdateAllocation for PgProjectRepository {
    async fn update_allocation(
        &self,
        uuid: Uuid,
        u: UpdateProjectDailyAllocationRow,
    ) -> Result<ProjectDailyAllocationRow, ProjectError> {
        sqlx::query_as!(
            ProjectDailyAllocationRow,
            r#"
            UPDATE project.tb_project_daily_allocation
            SET nr_hours_worked = COALESCE($1, nr_hours_worked),
                nr_hourly_rate_snapshot = COALESCE($2, nr_hourly_rate_snapshot),
                tx_notes = COALESCE($3, tx_notes),
                bl_present = COALESCE($4, bl_present)
            WHERE pk_project_daily_allocation = $5
            RETURNING *
            "#,
            u.nr_hours_worked,
            u.nr_hourly_rate_snapshot,
            u.tx_notes,
            u.bl_present,
            &uuid,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| db_err("update allocation", e))
    }
}

#[async_trait]
impl FindStagesByProjectId for PgProjectRepository {
    async fn find_stages_by_project_id(
        &self,
        project_id: Uuid,
    ) -> Result<Vec<ProjectStageRow>, ProjectError> {
        sqlx::query_as!(
            ProjectStageRow,
            r#"
            SELECT *
            FROM project.tb_project_stage
            WHERE fk_project = $1
            ORDER BY nr_order ASC
            "#,
            &project_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| db_err("list stages by project", e))
    }
}

#[async_trait]
impl FindAllocationsByCollaboratorId for PgProjectRepository {
    async fn find_allocations_by_collaborator_id(
        &self,
        collaborator_id: Uuid,
    ) -> Result<Vec<AllocationWithProjectName>, ProjectError> {
        sqlx::query_as!(
            AllocationWithProjectName,
            r#"
            SELECT
                a.pk_project_daily_allocation,
                a.fk_project,
                a.fk_collaborator,
                a.dt_work_date,
                a.nr_hours_worked,
                a.nr_hourly_rate_snapshot,
                a.tx_notes,
                a.bl_present,
                a.ts_allocated_collaborator_created_at,
                a.ts_allocated_collaborator_updated_at,
                p.tx_name as project_name
            FROM project.tb_project_daily_allocation a
            JOIN project.tb_project p ON a.fk_project = p.pk_project
            WHERE a.fk_collaborator = $1
            ORDER BY a.dt_work_date DESC
            "#,
            &collaborator_id,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| db_err("list allocations by collaborator", e))
    }
}
