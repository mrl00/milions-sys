use crate::domain::errors::CollaboratorError;
use domain::types::cpf::Cpf;
use uuid::Uuid;

use crate::domain::model::{
    CollaboratorRow, CollaboratorStatus, CreateCollaboratorRow, UpdateCollaboratorRow,
};
use crate::domain::ports::CollaboratorRepository;

pub struct CollaboratorService;

impl CollaboratorService {
    pub async fn find_by_id(
        repo: &dyn CollaboratorRepository,
        uuid: Uuid,
    ) -> Result<CollaboratorRow, CollaboratorError> {
        repo.find_by_id(uuid)
            .await?
            .ok_or(CollaboratorError::NotFound { uuid })
    }

    pub async fn find_by_cpf(
        repo: &dyn CollaboratorRepository,
        cpf: &str,
    ) -> Result<Option<CollaboratorRow>, CollaboratorError> {
        repo.find_by_cpf(cpf).await
    }

    pub async fn find_all(
        repo: &dyn CollaboratorRepository,
    ) -> Result<Vec<CollaboratorRow>, CollaboratorError> {
        repo.find_all().await
    }

    pub async fn create(
        repo: &dyn CollaboratorRepository,
        name: String,
        cpf: String,
    ) -> Result<CollaboratorRow, CollaboratorError> {
        let _validated_cpf: Cpf = cpf.clone().try_into()?;

        if repo.find_by_cpf(&cpf).await?.is_some() {
            return Err(CollaboratorError::CpfAlreadyExists { cpf });
        }

        let input = CreateCollaboratorRow {
            tx_name: name,
            tx_cpf: cpf,
            tx_level: crate::domain::model::CollaboratorLevel::P0,
            tx_status: CollaboratorStatus::Active,
        };

        repo.create(input).await
    }

    pub async fn update(
        repo: &dyn CollaboratorRepository,
        uuid: Uuid,
        input: UpdateCollaboratorRow,
    ) -> Result<CollaboratorRow, CollaboratorError> {
        Self::find_by_id(repo, uuid).await?;

        if let Some(ref cpf) = input.tx_cpf {
            let _validated: Cpf = cpf.clone().try_into()?;
        }

        repo.update(uuid, input).await
    }

    pub async fn activate(
        repo: &dyn CollaboratorRepository,
        uuid: Uuid,
    ) -> Result<CollaboratorRow, CollaboratorError> {
        let current = Self::find_by_id(repo, uuid).await?;
        if current.tx_status == CollaboratorStatus::Active.to_string() {
            return Err(CollaboratorError::AlreadyActive { uuid });
        }

        repo.update(
            uuid,
            UpdateCollaboratorRow {
                tx_name: None,
                tx_level: None,
                tx_status: Some(CollaboratorStatus::Active.to_string()),
                tx_cpf: None,
            },
        )
        .await
    }

    pub async fn deactivate(
        repo: &dyn CollaboratorRepository,
        uuid: Uuid,
    ) -> Result<CollaboratorRow, CollaboratorError> {
        let current = Self::find_by_id(repo, uuid).await?;
        if current.tx_status == CollaboratorStatus::Inactive.to_string() {
            return Err(CollaboratorError::AlreadyInactive { uuid });
        }

        repo.update(
            uuid,
            UpdateCollaboratorRow {
                tx_name: None,
                tx_level: None,
                tx_status: Some(CollaboratorStatus::Inactive.to_string()),
                tx_cpf: None,
            },
        )
        .await
    }

    pub async fn delete(
        repo: &dyn CollaboratorRepository,
        uuid: Uuid,
    ) -> Result<CollaboratorRow, CollaboratorError> {
        Self::find_by_id(repo, uuid).await?;
        repo.delete(uuid).await
    }
}
