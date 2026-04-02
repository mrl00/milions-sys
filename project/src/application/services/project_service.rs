use crate::domain::errors::ProjectError;
use crate::domain::models::db::project_rows::{
    CreateProjectRow, ProjectRow, ProjectStatus, UpdateProjectRow,
};
use crate::domain::ports::project_repository::ProjectRepository;
use uuid::Uuid;

pub struct ProjectService;

impl ProjectService {
    pub async fn find_by_id(
        repo: &dyn ProjectRepository,
        uuid: Uuid,
    ) -> Result<ProjectRow, ProjectError> {
        repo.find_by_id(uuid)
            .await?
            .ok_or(ProjectError::NotFound { uuid })
    }

    pub async fn find_by_client_id(
        repo: &dyn ProjectRepository,
        client_id: Uuid,
    ) -> Result<Vec<ProjectRow>, ProjectError> {
        repo.find_by_client_id(client_id).await
    }

    pub async fn find_all(repo: &dyn ProjectRepository) -> Result<Vec<ProjectRow>, ProjectError> {
        repo.find_all().await
    }

    pub async fn create(
        repo: &dyn ProjectRepository,
        input: CreateProjectRow,
    ) -> Result<ProjectRow, ProjectError> {
        repo.create(input).await
    }

    pub async fn update(
        repo: &dyn ProjectRepository,
        uuid: Uuid,
        input: UpdateProjectRow,
    ) -> Result<ProjectRow, ProjectError> {
        Self::find_by_id(repo, uuid).await?;
        repo.update(uuid, input).await
    }

    pub async fn start(
        repo: &dyn ProjectRepository,
        uuid: Uuid,
    ) -> Result<ProjectRow, ProjectError> {
        let current = Self::find_by_id(repo, uuid).await?;
        if current.tx_status == ProjectStatus::InProgress.to_string() {
            return Err(ProjectError::AlreadyInStatus {
                uuid,
                status: ProjectStatus::InProgress.to_string(),
            });
        }

        repo.update(
            uuid,
            UpdateProjectRow {
                tx_name: None,
                tx_description: None,
                tx_status: Some(ProjectStatus::InProgress),
                dt_start_date: None,
                dt_estimated_end_date: None,
                dt_actual_end_date: None,
                nr_total_area_m2: None,
                nr_estimated_cost: None,
                nr_actual_cost: None,
                tx_notes: None,
                bl_active: None,
            },
        )
        .await
    }

    pub async fn pause(
        repo: &dyn ProjectRepository,
        uuid: Uuid,
    ) -> Result<ProjectRow, ProjectError> {
        let current = Self::find_by_id(repo, uuid).await?;
        if current.tx_status == ProjectStatus::Paused.to_string() {
            return Err(ProjectError::AlreadyInStatus {
                uuid,
                status: ProjectStatus::Paused.to_string(),
            });
        }

        repo.update(
            uuid,
            UpdateProjectRow {
                tx_name: None,
                tx_description: None,
                tx_status: Some(ProjectStatus::Paused),
                dt_start_date: None,
                dt_estimated_end_date: None,
                dt_actual_end_date: None,
                nr_total_area_m2: None,
                nr_estimated_cost: None,
                nr_actual_cost: None,
                tx_notes: None,
                bl_active: None,
            },
        )
        .await
    }

    pub async fn complete(
        repo: &dyn ProjectRepository,
        uuid: Uuid,
    ) -> Result<ProjectRow, ProjectError> {
        let current = Self::find_by_id(repo, uuid).await?;
        if current.tx_status == ProjectStatus::Completed.to_string() {
            return Err(ProjectError::AlreadyInStatus {
                uuid,
                status: ProjectStatus::Completed.to_string(),
            });
        }

        repo.update(
            uuid,
            UpdateProjectRow {
                tx_name: None,
                tx_description: None,
                tx_status: Some(ProjectStatus::Completed),
                dt_start_date: None,
                dt_estimated_end_date: None,
                dt_actual_end_date: None,
                nr_total_area_m2: None,
                nr_estimated_cost: None,
                nr_actual_cost: None,
                tx_notes: None,
                bl_active: None,
            },
        )
        .await
    }

    pub async fn cancel(
        repo: &dyn ProjectRepository,
        uuid: Uuid,
    ) -> Result<ProjectRow, ProjectError> {
        let current = Self::find_by_id(repo, uuid).await?;
        if current.tx_status == ProjectStatus::Cancelled.to_string() {
            return Err(ProjectError::AlreadyInStatus {
                uuid,
                status: ProjectStatus::Cancelled.to_string(),
            });
        }

        repo.update(
            uuid,
            UpdateProjectRow {
                tx_name: None,
                tx_description: None,
                tx_status: Some(ProjectStatus::Cancelled),
                dt_start_date: None,
                dt_estimated_end_date: None,
                dt_actual_end_date: None,
                nr_total_area_m2: None,
                nr_estimated_cost: None,
                nr_actual_cost: None,
                tx_notes: None,
                bl_active: None,
            },
        )
        .await
    }

    pub async fn delete(
        repo: &dyn ProjectRepository,
        uuid: Uuid,
    ) -> Result<ProjectRow, ProjectError> {
        Self::find_by_id(repo, uuid).await?;
        repo.delete(uuid).await
    }
}
