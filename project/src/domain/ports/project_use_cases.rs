use async_trait::async_trait;
use sqlx::types::BigDecimal;
use sqlx::types::chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::errors::ProjectError;
use crate::domain::models::db::project_rows::{ProjectRow, ProjectStageRow};

pub struct CreateProjectInput {
    pub name: String,
    pub description: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub estimated_end_date: Option<NaiveDate>,
    pub total_area_m2: Option<BigDecimal>,
    pub estimated_cost: Option<BigDecimal>,
    pub notes: Option<String>,
    pub client_id: Uuid,
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

#[async_trait]
pub trait FindProject: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError>;
}

#[async_trait]
pub trait ListProjects: Send + Sync {
    async fn execute(&self) -> Result<Vec<ProjectRow>, ProjectError>;
}

#[async_trait]
pub trait ListProjectsByClient: Send + Sync {
    async fn execute(&self, client_id: Uuid) -> Result<Vec<ProjectRow>, ProjectError>;
}

#[async_trait]
pub trait CreateProject: Send + Sync {
    async fn execute(&self, input: CreateProjectInput) -> Result<ProjectRow, ProjectError>;
}

#[async_trait]
pub trait UpdateProject: Send + Sync {
    async fn execute(
        &self,
        uuid: Uuid,
        input: UpdateProjectInput,
    ) -> Result<ProjectRow, ProjectError>;
}

#[async_trait]
pub trait StartProject: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError>;
}

#[async_trait]
pub trait PauseProject: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError>;
}

#[async_trait]
pub trait CompleteProject: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError>;
}

#[async_trait]
pub trait CancelProject: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError>;
}

#[async_trait]
pub trait DeleteProject: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ProjectRow, ProjectError>;
}

#[async_trait]
pub trait CreateStage: Send + Sync {
    async fn execute(
        &self,
        project_id: Uuid,
        input: CreateStageInput,
    ) -> Result<ProjectStageRow, ProjectError>;
}

#[async_trait]
pub trait UpdateStage: Send + Sync {
    async fn execute(
        &self,
        project_id: Uuid,
        stage_id: Uuid,
        input: UpdateStageInput,
    ) -> Result<ProjectStageRow, ProjectError>;
}
