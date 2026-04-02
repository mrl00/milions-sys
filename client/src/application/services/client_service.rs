use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::adapters::driven::postgres::pg_client_repository::PgClientRepository;
use crate::domain::errors::ClientError;
use crate::domain::models::db::client_row::{ClientRow, ClientStatus};
use crate::domain::ports::client_repository::*;
use crate::domain::use_cases::activate_client::ActivateClient;
use crate::domain::use_cases::deactivate_client::DeactivateClient;
use crate::domain::use_cases::delete_client::DeleteClient as DeleteClientTrait;
use crate::domain::use_cases::find_client::FindClientById;
use crate::domain::use_cases::find_client_by_document::FindClientByDocument;
use crate::domain::use_cases::list_clients::ListClients;
use crate::domain::use_cases::update_client::{UpdateClient as UpdateClientTrait, UpdateClientInput};
use types::doc::Doc;

pub struct ClientUseCases {
    repo: PgClientRepository,
}

impl ClientUseCases {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: PgClientRepository::new(pool),
        }
    }
}

#[async_trait]
impl FindClientById for ClientUseCases {
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ClientError::NotFound { uuid })
    }
}

#[async_trait]
impl FindClientByDocument for ClientUseCases {
    async fn execute(&self, doc: &str) -> Result<Option<ClientRow>, ClientError> {
        self.repo.find_by_document(doc).await
    }
}

#[async_trait]
impl ListClients for ClientUseCases {
    async fn execute(&self) -> Result<Vec<ClientRow>, ClientError> {
        self.repo.find_all().await
    }
}

#[async_trait]
impl UpdateClientTrait for ClientUseCases {
    async fn execute(
        &self,
        uuid: Uuid,
        input: UpdateClientInput,
    ) -> Result<ClientRow, ClientError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ClientError::NotFound { uuid })?;

        if let Some(ref doc) = input.doc {
            let _validated: Doc = doc.clone().try_into()?;
        }

        self.repo
            .update(
                uuid,
                crate::domain::models::db::client_row::UpdateClientRow {
                    tx_name: input.name,
                    tx_status: None,
                    tx_doc: input.doc,
                },
            )
            .await
    }
}

#[async_trait]
impl ActivateClient for ClientUseCases {
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError> {
        let current = self
            .repo
            .find_by_id(uuid)
            .await?
            .ok_or(ClientError::NotFound { uuid })?;

        if current.tx_status == ClientStatus::Active.to_string() {
            return Err(ClientError::AlreadyActive { uuid });
        }

        self.repo
            .update(
                uuid,
                crate::domain::models::db::client_row::UpdateClientRow {
                    tx_name: None,
                    tx_status: Some(ClientStatus::Active),
                    tx_doc: None,
                },
            )
            .await
    }
}

#[async_trait]
impl DeactivateClient for ClientUseCases {
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError> {
        let current = self
            .repo
            .find_by_id(uuid)
            .await?
            .ok_or(ClientError::NotFound { uuid })?;

        if current.tx_status == ClientStatus::Inactive.to_string() {
            return Err(ClientError::AlreadyInactive { uuid });
        }

        self.repo
            .update(
                uuid,
                crate::domain::models::db::client_row::UpdateClientRow {
                    tx_name: None,
                    tx_status: Some(ClientStatus::Inactive),
                    tx_doc: None,
                },
            )
            .await
    }
}

#[async_trait]
impl DeleteClientTrait for ClientUseCases {
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ClientError::NotFound { uuid })?;

        self.repo.delete(uuid).await
    }
}
