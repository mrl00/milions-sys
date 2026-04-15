use crate::domain::models::db;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{NaiveDate, NaiveDateTime};
use uuid::Uuid;
// --- Project ---

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub address_id: Uuid,
    pub start_date: Option<NaiveDate>,
    pub estimated_end_date: Option<NaiveDate>,
    pub total_area_m2: Option<String>,
    pub estimated_cost: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub estimated_end_date: Option<NaiveDate>,
    pub actual_end_date: Option<NaiveDate>,
    pub total_area_m2: Option<String>,
    pub estimated_cost: Option<String>,
    pub actual_cost: Option<String>,
    pub notes: Option<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ProjectResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub start_date: Option<NaiveDate>,
    pub estimated_end_date: Option<NaiveDate>,
    pub actual_end_date: Option<NaiveDate>,
    pub total_area_m2: Option<String>,
    pub estimated_cost: Option<String>,
    pub actual_cost: Option<String>,
    pub notes: Option<String>,
    pub active: bool,
    pub address_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<db::project_rows::ProjectRow> for ProjectResponse {
    fn from(row: db::project_rows::ProjectRow) -> Self {
        Self {
            id: row.pk_project,
            name: row.tx_name,
            description: row.tx_description,
            status: row.tx_status,
            start_date: row.dt_start_date,
            estimated_end_date: row.dt_estimated_end_date,
            actual_end_date: row.dt_actual_end_date,
            total_area_m2: row.nr_total_area_m2.map(|v| v.to_string()),
            estimated_cost: row.nr_estimated_cost.map(|v| v.to_string()),
            actual_cost: row.nr_actual_cost.map(|v| v.to_string()),
            notes: row.tx_notes,
            active: row.bl_active,
            address_id: row.fk_address,
            created_at: row.ts_project_created_at,
            updated_at: row.ts_project_updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ProjectStatusRequest {
    pub status: String,
}

// --- Stage ---

#[derive(Debug, Deserialize)]
pub struct CreateStageRequest {
    pub name: String,
    pub description: Option<String>,
    pub order: i32,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStageRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub order: Option<i32>,
    pub status: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize)]
pub struct StageResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub order: i32,
    pub status: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<db::project_rows::ProjectStageRow> for StageResponse {
    fn from(row: db::project_rows::ProjectStageRow) -> Self {
        Self {
            id: row.pk_project_stage,
            project_id: row.fk_project,
            name: row.tx_name,
            description: row.tx_description,
            order: row.nr_order,
            status: row.tx_status,
            start_date: row.dt_start_date,
            end_date: row.dt_end_date,
            created_at: row.ts_created_at_project_stage,
            updated_at: row.ts_updated_at_project_stage,
        }
    }
}

// --- Allocation ---

#[derive(Debug, Deserialize)]
pub struct CreateAllocationRequest {
    pub collaborator_id: Uuid,
    pub work_date: NaiveDate,
    pub hours_worked: Option<String>,
    pub hourly_rate_snapshot: Option<String>,
    pub present: bool,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAllocationRequest {
    pub hours_worked: Option<String>,
    pub hourly_rate_snapshot: Option<String>,
    pub present: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AllocationResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub collaborator_id: Uuid,
    pub work_date: NaiveDate,
    pub hours_worked: Option<String>,
    pub hourly_rate_snapshot: Option<String>,
    pub notes: Option<String>,
    pub present: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<db::project_rows::ProjectDailyAllocationRow> for AllocationResponse {
    fn from(row: db::project_rows::ProjectDailyAllocationRow) -> Self {
        Self {
            id: row.pk_project_daily_allocation,
            project_id: row.fk_project,
            collaborator_id: row.fk_collaborator,
            work_date: row.dt_work_date,
            hours_worked: row.nr_hours_worked.map(|v| v.to_string()),
            hourly_rate_snapshot: row.nr_hourly_rate_snapshot.map(|v| v.to_string()),
            notes: row.tx_notes,
            present: row.bl_present,
            created_at: row.ts_allocated_collaborator_created_at,
            updated_at: row.ts_allocated_collaborator_updated_at,
        }
    }
}

// --- Reports ---

#[derive(Debug, Serialize)]
pub struct CostReportResponse {
    pub project_id: Uuid,
    pub project_name: String,
    pub estimated_cost: Option<String>,
    pub actual_cost: String,
    pub variance: String,
    pub variance_pct: String,
}

#[derive(Debug, Serialize)]
pub struct StageProgress {
    pub stage_id: Uuid,
    pub name: String,
    pub order: i32,
    pub status: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize)]
pub struct ProgressReportResponse {
    pub project_id: Uuid,
    pub project_name: String,
    pub stages: Vec<StageProgress>,
    pub total_stages: i32,
    pub completed_stages: i32,
    pub progress_pct: String,
}

#[derive(Debug, Serialize)]
pub struct AllocationHistory {
    pub allocation_id: Uuid,
    pub project_id: Uuid,
    pub project_name: String,
    pub work_date: NaiveDate,
    pub hours_worked: Option<String>,
    pub hourly_rate_snapshot: Option<String>,
    pub present: bool,
}

#[derive(Debug, Serialize)]
pub struct HistoryReportResponse {
    pub collaborator_id: Uuid,
    pub collaborator_name: String,
    pub allocations: Vec<AllocationHistory>,
    pub total_days: i32,
    pub total_hours: String,
}
