use async_trait::async_trait;
use uuid::Uuid;

use crate::adapters::driven::postgres::pg_collaborator_repository::PgCollaboratorRepository;
use crate::domain::errors::collaborator_error::CollaboratorError;
use crate::domain::models::db::collaborator_row::{
    CollaboratorLevel, CollaboratorRow, CollaboratorStatus, CreateCollaboratorRow,
    UpdateCollaboratorRow,
};
use crate::domain::ports::collaborator_repository::{
    CollaboratorRepository, CreateCollaborator, DeleteCollaborator, FindAllCollaborators,
    FindCollaboratorByDocument, FindCollaboratorById, UpdateCollaborator,
};
use crate::domain::ports::collaborator_use_cases::{
    ActivateCollaboratorUseCase, DeactivateCollaboratorUseCase, DeleteCollaboratorUseCase,
    FindCollaboratorByDocumentUseCase, FindCollaboratorUseCase, ListCollaboratorsUseCase,
    RegisterCollaboratorInput, RegisterCollaboratorUseCase, UpdateCollaboratorInput,
    UpdateCollaboratorUseCase,
};
use types::cpf::Cpf;

pub struct CollaboratorService<R> {
    repo: R,
}

impl<R> CollaboratorService<R>
where
    R: FindCollaboratorById
        + FindCollaboratorByDocument
        + FindAllCollaborators
        + CreateCollaborator
        + UpdateCollaborator
        + DeleteCollaborator,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

pub type ConcreteCollaboratorService = CollaboratorService<PgCollaboratorRepository>;

#[async_trait]
impl<R: CollaboratorRepository> FindCollaboratorUseCase for CollaboratorService<R> {
    async fn execute(&self, uuid: Uuid) -> Result<CollaboratorRow, CollaboratorError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(CollaboratorError::NotFound { uuid })
    }
}

#[async_trait]
impl<R: CollaboratorRepository> FindCollaboratorByDocumentUseCase for CollaboratorService<R> {
    async fn execute(&self, cpf: &str) -> Result<Option<CollaboratorRow>, CollaboratorError> {
        self.repo.find_by_document(cpf).await
    }
}

#[async_trait]
impl<R: CollaboratorRepository> ListCollaboratorsUseCase for CollaboratorService<R> {
    async fn execute(&self) -> Result<Vec<CollaboratorRow>, CollaboratorError> {
        self.repo.find_all().await
    }
}

#[async_trait]
impl<R: CollaboratorRepository> RegisterCollaboratorUseCase for CollaboratorService<R> {
    async fn execute(
        &self,
        input: RegisterCollaboratorInput,
    ) -> Result<CollaboratorRow, CollaboratorError> {
        let _validated_cpf: Cpf = input.cpf.clone().try_into()?;

        if self.repo.find_by_document(&input.cpf).await?.is_some() {
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
impl<R: CollaboratorRepository> UpdateCollaboratorUseCase for CollaboratorService<R> {
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
impl<R: CollaboratorRepository> ActivateCollaboratorUseCase for CollaboratorService<R> {
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
impl<R: CollaboratorRepository> DeactivateCollaboratorUseCase for CollaboratorService<R> {
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
impl<R: CollaboratorRepository> DeleteCollaboratorUseCase for CollaboratorService<R> {
    async fn execute(&self, uuid: Uuid) -> Result<CollaboratorRow, CollaboratorError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(CollaboratorError::NotFound { uuid })?;

        self.repo.delete(uuid).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::errors::collaborator_error::CollaboratorError;
    use crate::domain::models::db::collaborator_row::{
        CollaboratorRow, CreateCollaboratorRow, UpdateCollaboratorRow,
    };
    use crate::domain::ports::collaborator_repository::{
        CreateCollaborator, DeleteCollaborator, FindAllCollaborators, FindCollaboratorByDocument,
        FindCollaboratorById, UpdateCollaborator,
    };
    use crate::domain::ports::collaborator_use_cases::{
        ActivateCollaboratorUseCase, DeactivateCollaboratorUseCase, DeleteCollaboratorUseCase,
        FindCollaboratorByDocumentUseCase, FindCollaboratorUseCase, ListCollaboratorsUseCase,
        RegisterCollaboratorInput, RegisterCollaboratorUseCase, UpdateCollaboratorInput,
        UpdateCollaboratorUseCase,
    };

    #[derive(Default)]
    struct MockRepo {
        find_by_id_result: Option<CollaboratorRow>,
        find_by_document_result: Option<CollaboratorRow>,
        find_all_result: Vec<CollaboratorRow>,
    }

    impl MockRepo {
        fn new() -> Self {
            Self::default()
        }
    }

    fn now() -> chrono::NaiveDateTime {
        chrono::NaiveDateTime::default()
    }

    fn make_row() -> CollaboratorRow {
        CollaboratorRow {
            pk_collaborator: Uuid::now_v7(),
            idx_collaborator: 1,
            tx_name: "John Doe".to_string(),
            tx_cpf: "12345678909".to_string(),
            tx_level: "P0".to_string(),
            tx_status: "active".to_string(),
            ts_collaborator_created_at: now(),
            ts_collaborator_updated_at: now(),
        }
    }

    #[async_trait]
    impl FindCollaboratorById for MockRepo {
        async fn find_by_id(
            &self,
            _uuid: Uuid,
        ) -> Result<Option<CollaboratorRow>, CollaboratorError> {
            Ok(self.find_by_id_result.clone())
        }
    }

    #[async_trait]
    impl FindCollaboratorByDocument for MockRepo {
        async fn find_by_document(
            &self,
            _cpf: &str,
        ) -> Result<Option<CollaboratorRow>, CollaboratorError> {
            Ok(self.find_by_document_result.clone())
        }
    }

    #[async_trait]
    impl FindAllCollaborators for MockRepo {
        async fn find_all(&self) -> Result<Vec<CollaboratorRow>, CollaboratorError> {
            Ok(self.find_all_result.clone())
        }
    }

    #[async_trait]
    impl CreateCollaborator for MockRepo {
        async fn create(
            &self,
            input: CreateCollaboratorRow,
        ) -> Result<CollaboratorRow, CollaboratorError> {
            Ok(CollaboratorRow {
                pk_collaborator: Uuid::now_v7(),
                idx_collaborator: 0,
                tx_name: input.tx_name,
                tx_cpf: input.tx_cpf,
                tx_level: input.tx_level.to_string(),
                tx_status: input.tx_status.to_string(),
                ts_collaborator_created_at: now(),
                ts_collaborator_updated_at: now(),
            })
        }
    }

    #[async_trait]
    impl UpdateCollaborator for MockRepo {
        async fn update(
            &self,
            uuid: Uuid,
            _input: UpdateCollaboratorRow,
        ) -> Result<CollaboratorRow, CollaboratorError> {
            Ok(CollaboratorRow {
                pk_collaborator: uuid,
                idx_collaborator: 0,
                tx_name: "".to_string(),
                tx_cpf: "".to_string(),
                tx_level: "".to_string(),
                tx_status: "".to_string(),
                ts_collaborator_created_at: now(),
                ts_collaborator_updated_at: now(),
            })
        }
    }

    #[async_trait]
    impl DeleteCollaborator for MockRepo {
        async fn delete(&self, uuid: Uuid) -> Result<CollaboratorRow, CollaboratorError> {
            Ok(CollaboratorRow {
                pk_collaborator: uuid,
                idx_collaborator: 0,
                tx_name: "".to_string(),
                tx_cpf: "".to_string(),
                tx_level: "".to_string(),
                tx_status: "".to_string(),
                ts_collaborator_created_at: now(),
                ts_collaborator_updated_at: now(),
            })
        }
    }

    #[tokio::test]
    async fn find_collaborator_returns_row_when_exists() {
        let row = make_row();
        let uuid = row.pk_collaborator;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = CollaboratorService::new(repo);
        let result = FindCollaboratorUseCase::execute(&service, uuid)
            .await
            .unwrap();
        assert_eq!(result.pk_collaborator, uuid);
        assert_eq!(result.tx_name, "John Doe");
    }

    #[tokio::test]
    async fn find_collaborator_returns_not_found_when_missing() {
        let uuid = Uuid::now_v7();
        let repo = MockRepo::new();
        let service = CollaboratorService::new(repo);
        let result = FindCollaboratorUseCase::execute(&service, uuid).await;
        assert!(matches!(result, Err(CollaboratorError::NotFound { .. })));
    }

    #[tokio::test]
    async fn find_collaborator_by_cpf_returns_row() {
        let row = make_row();
        let mut repo = MockRepo::new();
        repo.find_by_document_result = Some(row.clone());
        let service = CollaboratorService::new(repo);
        let result = FindCollaboratorByDocumentUseCase::execute(&service, "12345678909")
            .await
            .unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().tx_cpf, "12345678909");
    }

    #[tokio::test]
    async fn list_collaborators_returns_all() {
        let r1 = make_row();
        let r2 = make_row();
        let mut repo = MockRepo::new();
        repo.find_all_result = vec![r1, r2];
        let service = CollaboratorService::new(repo);
        let result = ListCollaboratorsUseCase::execute(&service).await.unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn list_collaborators_returns_empty() {
        let repo = MockRepo::new();
        let service = CollaboratorService::new(repo);
        let result = ListCollaboratorsUseCase::execute(&service).await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn register_collaborator_fails_when_cpf_exists() {
        let mut repo = MockRepo::new();
        repo.find_by_document_result = Some(make_row());
        let service = CollaboratorService::new(repo);
        let input = RegisterCollaboratorInput {
            name: "Jane".to_string(),
            cpf: "12345678909".to_string(),
        };
        let result = RegisterCollaboratorUseCase::execute(&service, input).await;
        assert!(matches!(
            result,
            Err(CollaboratorError::CpfAlreadyExists { .. })
        ));
    }

    #[tokio::test]
    async fn update_collaborator_returns_not_found_when_missing() {
        let uuid = Uuid::now_v7();
        let repo = MockRepo::new();
        let service = CollaboratorService::new(repo);
        let input = UpdateCollaboratorInput {
            name: Some("Updated".to_string()),
            cpf: None,
            level: None,
        };
        let result = UpdateCollaboratorUseCase::execute(&service, uuid, input).await;
        assert!(matches!(result, Err(CollaboratorError::NotFound { .. })));
    }

    #[tokio::test]
    async fn activate_collaborator_succeeds_when_inactive() {
        let mut row = make_row();
        row.tx_status = "inactive".to_string();
        let uuid = row.pk_collaborator;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = CollaboratorService::new(repo);
        let result = ActivateCollaboratorUseCase::execute(&service, uuid)
            .await
            .unwrap();
        assert_eq!(result.pk_collaborator, uuid);
    }

    #[tokio::test]
    async fn activate_collaborator_fails_when_already_active() {
        let row = make_row();
        let uuid = row.pk_collaborator;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = CollaboratorService::new(repo);
        let result = ActivateCollaboratorUseCase::execute(&service, uuid).await;
        assert!(matches!(
            result,
            Err(CollaboratorError::AlreadyActive { .. })
        ));
    }

    #[tokio::test]
    async fn deactivate_collaborator_succeeds_when_active() {
        let row = make_row();
        let uuid = row.pk_collaborator;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = CollaboratorService::new(repo);
        let result = DeactivateCollaboratorUseCase::execute(&service, uuid)
            .await
            .unwrap();
        assert_eq!(result.pk_collaborator, uuid);
    }

    #[tokio::test]
    async fn deactivate_collaborator_fails_when_already_inactive() {
        let mut row = make_row();
        row.tx_status = "inactive".to_string();
        let uuid = row.pk_collaborator;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = CollaboratorService::new(repo);
        let result = DeactivateCollaboratorUseCase::execute(&service, uuid).await;
        assert!(matches!(
            result,
            Err(CollaboratorError::AlreadyInactive { .. })
        ));
    }

    #[tokio::test]
    async fn delete_collaborator_succeeds_when_exists() {
        let row = make_row();
        let uuid = row.pk_collaborator;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = CollaboratorService::new(repo);
        let result = DeleteCollaboratorUseCase::execute(&service, uuid)
            .await
            .unwrap();
        assert_eq!(result.pk_collaborator, uuid);
    }

    #[tokio::test]
    async fn delete_collaborator_fails_when_not_found() {
        let uuid = Uuid::now_v7();
        let repo = MockRepo::new();
        let service = CollaboratorService::new(repo);
        let result = DeleteCollaboratorUseCase::execute(&service, uuid).await;
        assert!(matches!(result, Err(CollaboratorError::NotFound { .. })));
    }

    #[test]
    fn error_not_found_message_contains_uuid() {
        let uuid = Uuid::now_v7();
        let err = CollaboratorError::NotFound { uuid };
        let msg = err.to_string();
        assert!(msg.contains(&uuid.to_string()));
        assert!(msg.contains("not found"));
    }

    #[test]
    fn error_cpf_already_exists_message_contains_cpf() {
        let err = CollaboratorError::CpfAlreadyExists {
            cpf: "12345678909".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("12345678909"));
        assert!(msg.contains("already registered"));
    }

    #[test]
    fn error_already_active_message_contains_uuid() {
        let uuid = Uuid::now_v7();
        let err = CollaboratorError::AlreadyActive { uuid };
        let msg = err.to_string();
        assert!(msg.contains(&uuid.to_string()));
        assert!(msg.contains("already active"));
    }

    #[test]
    fn error_already_inactive_message_contains_uuid() {
        let uuid = Uuid::now_v7();
        let err = CollaboratorError::AlreadyInactive { uuid };
        let msg = err.to_string();
        assert!(msg.contains(&uuid.to_string()));
        assert!(msg.contains("already inactive"));
    }
}
