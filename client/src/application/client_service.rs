use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::adapters::driven::postgres::pg_client_repository::PgClientRepository;
use crate::domain::errors::ClientError;
use crate::domain::models::db::client_row::{ClientRow, ClientStatus, UpdateClientRow};
use crate::domain::ports::client_repository::{
    CreateClient as _, DeleteClient as _, FindAll as _, FindById as _,
    FindByDocument as _, UpdateClient as _,
};
use crate::domain::ports::client_use_cases::{
    ActivateClient, DeactivateClient, DeleteClient as DeleteClientTrait, FindClientById,
    FindClientByDocument, ListClients, RegisterClient, RegisterClientInput, UpdateClient,
    UpdateClientInput,
};
use types::doc::Doc;
use types::phone::Phone;

pub struct ClientService {
    repo: PgClientRepository,
}

impl ClientService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: PgClientRepository::new(pool),
        }
    }
}

#[async_trait]
impl RegisterClient for ClientService {
    async fn execute(
        &self,
        input: RegisterClientInput,
    ) -> Result<ClientRow, ClientError> {
        let _doc: Doc = input.doc.clone().try_into()?;
        let _cep: types::cep::Cep = input.cep.clone().try_into()?;
        let _email: types::email::Email = input.email.clone().try_into()?;
        let _phones: Vec<Phone> = input
            .phones
            .iter()
            .map(|p| p.clone().try_into())
            .collect::<Result<Vec<_>, _>>()?;

        if self.repo.find_by_document(&input.doc).await?.is_some() {
            return Err(ClientError::DocumentAlreadyExists { doc: input.doc });
        }

        self.repo
            .create(crate::domain::models::db::client_row::CreateClientRow {
                tx_name: input.name,
                tx_status: ClientStatus::Active,
                tx_doc: input.doc,
            })
            .await
    }
}

#[async_trait]
impl FindClientById for ClientService {
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ClientError::NotFound { uuid })
    }
}

#[async_trait]
impl FindClientByDocument for ClientService {
    async fn execute(&self, doc: &str) -> Result<Option<ClientRow>, ClientError> {
        self.repo.find_by_document(doc).await
    }
}

#[async_trait]
impl ListClients for ClientService {
    async fn execute(&self) -> Result<Vec<ClientRow>, ClientError> {
        self.repo.find_all().await
    }
}

#[async_trait]
impl UpdateClient for ClientService {
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
                UpdateClientRow {
                    tx_name: input.name,
                    tx_status: None,
                    tx_doc: input.doc,
                },
            )
            .await
    }
}

#[async_trait]
impl ActivateClient for ClientService {
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
                UpdateClientRow {
                    tx_name: None,
                    tx_status: Some(ClientStatus::Active),
                    tx_doc: None,
                },
            )
            .await
    }
}

#[async_trait]
impl DeactivateClient for ClientService {
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
                UpdateClientRow {
                    tx_name: None,
                    tx_status: Some(ClientStatus::Inactive),
                    tx_doc: None,
                },
            )
            .await
    }
}

#[async_trait]
impl DeleteClientTrait for ClientService {
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ClientError::NotFound { uuid })?;

        self.repo.delete(uuid).await
    }
}
