use async_trait::async_trait;
use sqlx::types::BigDecimal;
use sqlx::types::chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::errors::ProjectError;
use crate::domain::models::db::project_rows::ProjectRow;

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

#[async_trait]
pub trait CreateProject: Send + Sync {
    async fn execute(&self, input: CreateProjectInput) -> Result<ProjectRow, ProjectError>;
}
