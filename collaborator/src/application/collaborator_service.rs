use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::adapters::driven::postgres::pg_collaborator_repository::PgCollaboratorRepository;
use crate::domain::errors::collaborator_error::CollaboratorError;
use crate::domain::models::db::collaborator_row::{
    CollaboratorLevel, CollaboratorRow, CollaboratorStatus, CreateCollaboratorRow,
    UpdateCollaboratorRow,
};
use crate::domain::ports::collaborator_repository::CreateCollaborator as _;
use crate::domain::ports::collaborator_repository::DeleteCollaborator as _;
use crate::domain::ports::collaborator_repository::FindAllCollaborators as _;
use crate::domain::ports::collaborator_repository::FindCollaboratorByCpf as _;
use crate::domain::ports::collaborator_repository::FindCollaboratorById as _;
use crate::domain::ports::collaborator_repository::UpdateCollaborator as _;
use crate::domain::ports::collaborator_use_cases::{
    ActivateCollaborator, DeactivateCollaborator, DeleteCollaborator as DeleteCollaboratorTrait,
    FindCollaborator, FindCollaboratorByCpf, ListCollaborators,
    RegisterCollaborator as RegisterCollaboratorTrait, RegisterCollaboratorInput,
    UpdateCollaborator as UpdateCollaboratorTrait, UpdateCollaboratorInput,
};
use types::cpf::Cpf;

pub struct CollaboratorService {
    repo: PgCollaboratorRepository,
}

impl CollaboratorService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: PgCollaboratorRepository::new(pool),
        }
    }
}

#[async_trait]
impl FindCollaborator for CollaboratorService {
    async fn execute(&self, uuid: Uuid) -> Result<CollaboratorRow, CollaboratorError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(CollaboratorError::NotFound { uuid })
    }
}

#[async_trait]
impl FindCollaboratorByCpf for CollaboratorService {
    async fn execute(&self, cpf: &str) -> Result<Option<CollaboratorRow>, CollaboratorError> {
        self.repo.find_by_cpf(cpf).await
    }
}

#[async_trait]
impl ListCollaborators for CollaboratorService {
    async fn execute(&self) -> Result<Vec<CollaboratorRow>, CollaboratorError> {
        self.repo.find_all().await
    }
}

#[async_trait]
impl RegisterCollaboratorTrait for CollaboratorService {
    async fn execute(
        &self,
        input: RegisterCollaboratorInput,
    ) -> Result<CollaboratorRow, CollaboratorError> {
        let _validated_cpf: Cpf = input.cpf.clone().try_into()?;

        if self.repo.find_by_cpf(&input.cpf).await?.is_some() {
            return Err(CollaboratorError::CpfAlreadyExists { cpf: input.cpf });
        }

        self.repo
            .create(CreateCollaboratorRow {
                tx_name: input.name,
                tx_cpf: input.cpf,
                tx_level: CollaboratorLevel::P0,
                tx_status: CollaboratorStatus::Active,
            })
            .await
    }
}

#[async_trait]
impl UpdateCollaboratorTrait for CollaboratorService {
    async fn execute(
        &self,
        uuid: Uuid,
        input: UpdateCollaboratorInput,
    ) -> Result<CollaboratorRow, CollaboratorError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(CollaboratorError::NotFound { uuid })?;

        if let Some(ref cpf) = input.cpf {
            let _validated: Cpf = cpf.clone().try_into()?;
        }

        self.repo
            .update(
                uuid,
                UpdateCollaboratorRow {
                    tx_name: input.name,
                    tx_level: input.level,
                    tx_status: None,
                    tx_cpf: input.cpf,
                },
            )
            .await
    }
}

#[async_trait]
impl ActivateCollaborator for CollaboratorService {
    async fn execute(&self, uuid: Uuid) -> Result<CollaboratorRow, CollaboratorError> {
        let current = self
            .repo
            .find_by_id(uuid)
            .await?
            .ok_or(CollaboratorError::NotFound { uuid })?;

        if current.tx_status == CollaboratorStatus::Active.to_string() {
            return Err(CollaboratorError::AlreadyActive { uuid });
        }

        self.repo
            .update(
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
}

#[async_trait]
impl DeactivateCollaborator for CollaboratorService {
    async fn execute(&self, uuid: Uuid) -> Result<CollaboratorRow, CollaboratorError> {
        let current = self
            .repo
            .find_by_id(uuid)
            .await?
            .ok_or(CollaboratorError::NotFound { uuid })?;

        if current.tx_status == CollaboratorStatus::Inactive.to_string() {
            return Err(CollaboratorError::AlreadyInactive { uuid });
        }

        self.repo
            .update(
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
}

#[async_trait]
impl DeleteCollaboratorTrait for CollaboratorService {
    async fn execute(&self, uuid: Uuid) -> Result<CollaboratorRow, CollaboratorError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(CollaboratorError::NotFound { uuid })?;

        self.repo.delete(uuid).await
    }
}
