use async_trait::async_trait;
use sqlx::types::BigDecimal;
use sqlx::types::chrono::NaiveDate;
use uuid::Uuid;
use crate::domain::errors::project_error::ProjectError;
use crate::domain::models::db::project_rows::{
    ProjectDailyAllocationRow, ProjectRow, ProjectStageRow,
};

pub struct CreateProjectInput {
    pub name: String,
    pub description: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub estimated_end_date: Option<NaiveDate>,
    pub total_area_m2: Option<BigDecimal>,
    pub estimated_cost: Option<BigDecimal>,
    pub notes: Option<String>,
    pub address_id: Uuid,
}

pub struct UpdateProjectInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub estimated_end_date: Option<NaiveDate>,
    pub actual_end_date: Option<NaiveDate>,
    pub total_area_m2: Option<BigDecimal>,
    pub estimated_cost: Option<BigDecimal>,
    pub actual_cost: Option<BigDecimal>,
    pub notes: Option<String>,
    pub active: Option<bool>,
}

pub struct CreateStageInput {
    pub name: String,
    pub description: Option<String>,
    pub order: i32,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

pub struct UpdateStageInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub order: Option<i32>,
    pub status: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

pub struct CreateAllocationInput {
    pub collaborator_id: Uuid,
    pub work_date: NaiveDate,
    pub hours_worked: Option<BigDecimal>,
    pub hourly_rate_snapshot: Option<BigDecimal>,
    pub notes: Option<String>,
    pub present: bool,
}

pub struct UpdateAllocationInput {
    pub hours_worked: Option<BigDecimal>,
    pub hourly_rate_snapshot: Option<BigDecimal>,
    pub notes: Option<String>,
    pub present: Option<bool>,
}

#[async_trait]
pub trait FindProjectUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError>;
}

#[async_trait]
pub trait ListProjectsUseCase: Send + Sync {
    async fn execute(&self) -> Result<Vec<ProjectRow>, ProjectError>;
}

#[async_trait]
pub trait CreateProjectUseCase: Send + Sync {
    async fn execute(&self, input: CreateProjectInput) -> Result<ProjectRow, ProjectError>;
}

#[async_trait]
pub trait UpdateProjectUseCase: Send + Sync {
    async fn execute(
        &self,
        uuid: Uuid,
        input: UpdateProjectInput,
    ) -> Result<ProjectRow, ProjectError>;
}

#[async_trait]
pub trait StartProjectUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError>;
}

#[async_trait]
pub trait PauseProjectUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError>;
}

#[async_trait]
pub trait CompleteProjectUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError>;
}

#[async_trait]
pub trait CancelProjectUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError>;
}

#[async_trait]
pub trait DeleteProjectUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError>;
}

#[async_trait]
pub trait CreateStageUseCase: Send + Sync {
    async fn execute(
        &self,
        project_id: Uuid,
        input: CreateStageInput,
    ) -> Result<ProjectStageRow, ProjectError>;
}

#[async_trait]
pub trait UpdateStageUseCase: Send + Sync {
    async fn execute(
        &self,
        project_id: Uuid,
        stage_id: Uuid,
        input: UpdateStageInput,
    ) -> Result<ProjectStageRow, ProjectError>;
}

#[async_trait]
pub trait CreateAllocationUseCase: Send + Sync {
    async fn execute(
        &self,
        project_id: Uuid,
        input: CreateAllocationInput,
    ) -> Result<ProjectDailyAllocationRow, ProjectError>;
}

#[async_trait]
pub trait ListAllocationsUseCase: Send + Sync {
    async fn execute(
        &self,
        project_id: Uuid,
    ) -> Result<Vec<ProjectDailyAllocationRow>, ProjectError>;
}

#[async_trait]
pub trait UpdateAllocationUseCase: Send + Sync {
    async fn execute(
        &self,
        project_id: Uuid,
        allocation_id: Uuid,
        input: UpdateAllocationInput,
    ) -> Result<ProjectDailyAllocationRow, ProjectError>;
}

pub struct CostReportData {
    pub project_id: Uuid,
    pub project_name: String,
    pub estimated_cost: Option<BigDecimal>,
    pub actual_cost: BigDecimal,
    pub variance: BigDecimal,
    pub variance_pct: Option<BigDecimal>,
}

pub struct ProgressReportData {
    pub project_id: Uuid,
    pub project_name: String,
    pub stages: Vec<ProjectStageRow>,
    pub total_stages: i32,
    pub completed_stages: i32,
    pub progress_pct: BigDecimal,
}

pub struct AllocationHistoryEntry {
    pub allocation_id: Uuid,
    pub project_id: Uuid,
    pub project_name: String,
    pub work_date: NaiveDate,
    pub hours_worked: Option<BigDecimal>,
    pub hourly_rate_snapshot: Option<BigDecimal>,
    pub present: bool,
}

pub struct HistoryReportData {
    pub collaborator_id: Uuid,
    pub collaborator_name: String,
    pub allocations: Vec<AllocationHistoryEntry>,
    pub total_days: i32,
    pub total_hours: BigDecimal,
}

#[async_trait]
pub trait GetCostReportUseCase: Send + Sync {
    async fn execute(&self, project_id: Uuid) -> Result<CostReportData, ProjectError>;
}

#[async_trait]
pub trait GetProgressReportUseCase: Send + Sync {
    async fn execute(&self, project_id: Uuid) -> Result<ProgressReportData, ProjectError>;
}

#[async_trait]
pub trait GetHistoryReportUseCase: Send + Sync {
    async fn execute(&self, collaborator_id: Uuid) -> Result<HistoryReportData, ProjectError>;
}
