use async_trait::async_trait;
use sqlx::types::BigDecimal;
use sqlx::types::chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::errors::ProjectError;
use crate::domain::models::db::project_rows::ProjectRow;

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

#[async_trait]
pub trait UpdateProject: Send + Sync {
    async fn execute(
        &self,
        uuid: Uuid,
        input: UpdateProjectInput,
    ) -> Result<ProjectRow, ProjectError>;
}
