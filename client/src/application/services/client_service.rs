use crate::domain::errors::ClientError;
use domain::types::doc::Doc;
use uuid::Uuid;

use crate::domain::model::{ClientRow, ClientStatus, CreateClientRow, UpdateClientRow};
use crate::domain::ports::ClientRepository;

pub struct ClientService;

impl ClientService {
    pub async fn find_by_id(
        repo: &dyn ClientRepository,
        uuid: Uuid,
    ) -> Result<ClientRow, ClientError> {
        repo.find_by_id(uuid)
            .await?
            .ok_or(ClientError::NotFound { uuid })
    }

    pub async fn find_by_document(
        repo: &dyn ClientRepository,
        doc: &str,
    ) -> Result<Option<ClientRow>, ClientError> {
        repo.find_by_document(doc).await
    }

    pub async fn find_all(repo: &dyn ClientRepository) -> Result<Vec<ClientRow>, ClientError> {
        repo.find_all().await
    }

    pub async fn create(
        repo: &dyn ClientRepository,
        name: String,
        doc: String,
    ) -> Result<ClientRow, ClientError> {
        let _validated_doc: Doc = doc.clone().try_into()?;

        if repo.find_by_document(&doc).await?.is_some() {
            return Err(ClientError::DocumentAlreadyExists { doc });
        }

        let input = CreateClientRow {
            tx_name: name,
            tx_status: ClientStatus::Active,
            tx_doc: doc,
        };

        repo.create(input).await
    }

    pub async fn update(
        repo: &dyn ClientRepository,
        uuid: Uuid,
        input: UpdateClientRow,
    ) -> Result<ClientRow, ClientError> {
        Self::find_by_id(repo, uuid).await?;

        if let Some(ref doc) = input.tx_doc {
            let _validated: Doc = doc.clone().try_into()?;
        }

        repo.update(uuid, input).await
    }

    pub async fn activate(
        repo: &dyn ClientRepository,
        uuid: Uuid,
    ) -> Result<ClientRow, ClientError> {
        let current = Self::find_by_id(repo, uuid).await?;
        if current.tx_status == ClientStatus::Active.to_string() {
            return Err(ClientError::AlreadyActive { uuid });
        }

        repo.update(
            uuid,
            UpdateClientRow {
                tx_name: None,
                tx_status: Some(ClientStatus::Active),
                tx_doc: None,
            },
        )
        .await
    }

    pub async fn deactivate(
        repo: &dyn ClientRepository,
        uuid: Uuid,
    ) -> Result<ClientRow, ClientError> {
        let current = Self::find_by_id(repo, uuid).await?;
        if current.tx_status == ClientStatus::Inactive.to_string() {
            return Err(ClientError::AlreadyInactive { uuid });
        }

        repo.update(
            uuid,
            UpdateClientRow {
                tx_name: None,
                tx_status: Some(ClientStatus::Inactive),
                tx_doc: None,
            },
        )
        .await
    }

    pub async fn delete(repo: &dyn ClientRepository, uuid: Uuid) -> Result<ClientRow, ClientError> {
        Self::find_by_id(repo, uuid).await?;
        repo.delete(uuid).await
    }
}
