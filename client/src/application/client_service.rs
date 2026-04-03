use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::adapters::driven::postgres::pg_client_repository::PgClientRepository;
use crate::domain::errors::ClientError;
use crate::domain::models::db::client_row::{ClientRow, ClientStatus, UpdateClientRow};
use crate::domain::ports::client_repository::{
    CreateClient, CreateClientWithTx as _, DeleteClient, FindAll, FindByDocument, FindById,
    UpdateClient as UpdateClientRepo,
};
use crate::domain::ports::client_use_cases::{
    ActivateClient, DeactivateClient, DeleteClient as DeleteClientTrait, FindClientByDocument,
    FindClientById, ListClients, RegisterClient, RegisterClientInput, UpdateClient,
    UpdateClientInput,
};
use contact::domain::ports::contact_use_cases::{
    AddPhones as AddPhonesTrait, RegisterContact as RegisterContactTrait,
};
use location::domain::ports::location_use_cases::FindOrCreateLocation as FindOrCreateLocationTrait;
use types::doc::Doc;
use types::phone::Phone;

pub struct ClientService<R, L, C> {
    repo: R,
    location_service: Arc<L>,
    contact_service: Arc<C>,
    pool: Option<PgPool>,
}

impl<R, L, C> ClientService<R, L, C>
where
    R: FindById + FindByDocument + FindAll + CreateClient + UpdateClientRepo + DeleteClient,
{
    pub fn new(
        repo: R,
        location_service: Arc<L>,
        contact_service: Arc<C>,
        pool: Option<PgPool>,
    ) -> Self {
        Self {
            repo,
            location_service,
            contact_service,
            pool,
        }
    }
}

pub type ConcreteClientService = ClientService<
    PgClientRepository,
    location::application::location_service::ConcreteLocationService,
    contact::application::contact_service::ConcreteContactService,
>;

#[async_trait]
impl RegisterClient for ConcreteClientService {
    async fn execute(&self, input: RegisterClientInput) -> Result<ClientRow, ClientError> {
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

        let mut tx = self.pool.as_ref().unwrap().begin().await.map_err(|e| {
            ClientError::Infra(types::errors::infra_error::InfraError::Database {
                action: "iniciar transação",
                source: e,
            })
        })?;

        let location = FindOrCreateLocationTrait::execute(
            self.location_service.as_ref(),
            location::domain::ports::location_use_cases::CreateLocationInput {
                street: input.number.clone(),
                number: input.number.clone(),
                city: String::new(),
                state: String::new(),
                zipcode: input.cep.clone(),
                complement: input.complement.clone(),
                public_space: String::new(),
                unit: String::new(),
                neighborhood: String::new(),
                locality: String::new(),
                region: String::new(),
                ibge: None,
                gia: None,
                ddd: String::new(),
                siafi: None,
                hash: 0,
            },
        )
        .await
        .map_err(|e| match e {
            location::domain::errors::LocationError::AlreadyExists { .. } => {
                ClientError::AlreadyExists {
                    name: "endereço".to_string(),
                }
            }
            _ => ClientError::Infra(types::errors::infra_error::InfraError::Database {
                action: "criar localização",
                source: sqlx::Error::Protocol("location error".to_string()),
            }),
        })?;

        let contact = RegisterContactTrait::execute(
            self.contact_service.as_ref(),
            contact::domain::ports::contact_use_cases::RegisterContactInput {
                email: input.email.clone(),
            },
        )
        .await
        .map_err(|e| match e {
            contact::domain::errors::contact_error::ContactError::AlreadyExists { .. } => {
                ClientError::AlreadyExists {
                    name: "contato".to_string(),
                }
            }
            _ => ClientError::Infra(types::errors::infra_error::InfraError::Database {
                action: "criar contato",
                source: sqlx::Error::Protocol("contact error".to_string()),
            }),
        })?;

        if !input.phones.is_empty() {
            AddPhonesTrait::execute(
                self.contact_service.as_ref(),
                contact.pk_contact,
                input.phones.clone(),
            )
            .await
            .map_err(|e| {
                ClientError::Infra(types::errors::infra_error::InfraError::Database {
                    action: "adicionar telefones",
                    source: sqlx::Error::Protocol("phone error".to_string()),
                })
            })?;
        }

        let client = self
            .repo
            .create_with_tx(
                &mut tx,
                crate::domain::models::db::client_row::CreateClientRow {
                    tx_name: input.name.clone(),
                    tx_status: ClientStatus::Active,
                    tx_doc: input.doc.clone(),
                },
            )
            .await?;

        PgClientRepository::create_contact(&mut *tx, client.pk_client, contact.pk_contact)
            .await
            .map_err(|e| {
                ClientError::Infra(types::errors::infra_error::InfraError::Database {
                    action: "vincular contato ao cliente",
                    source: e,
                })
            })?;

        PgClientRepository::create_address(&mut *tx, client.pk_client, location.pk_location)
            .await
            .map_err(|e| {
                ClientError::Infra(types::errors::infra_error::InfraError::Database {
                    action: "vincular endereço ao cliente",
                    source: e,
                })
            })?;

        tx.commit().await.map_err(|e| {
            ClientError::Infra(types::errors::infra_error::InfraError::Database {
                action: "commit transação",
                source: e,
            })
        })?;

        Ok(client)
    }
}

#[async_trait]
impl<R, L, C> FindClientById for ClientService<R, L, C>
where
    R: FindById
        + FindByDocument
        + FindAll
        + CreateClient
        + UpdateClientRepo
        + DeleteClient
        + Send
        + Sync,
    L: Send + Sync,
    C: Send + Sync,
{
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ClientError::NotFound { uuid })
    }
}

#[async_trait]
impl<R, L, C> FindClientByDocument for ClientService<R, L, C>
where
    R: FindById
        + FindByDocument
        + FindAll
        + CreateClient
        + UpdateClientRepo
        + DeleteClient
        + Send
        + Sync,
    L: Send + Sync,
    C: Send + Sync,
{
    async fn execute(&self, doc: &str) -> Result<Option<ClientRow>, ClientError> {
        self.repo.find_by_document(doc).await
    }
}

#[async_trait]
impl<R, L, C> ListClients for ClientService<R, L, C>
where
    R: FindById
        + FindByDocument
        + FindAll
        + CreateClient
        + UpdateClientRepo
        + DeleteClient
        + Send
        + Sync,
    L: Send + Sync,
    C: Send + Sync,
{
    async fn execute(&self) -> Result<Vec<ClientRow>, ClientError> {
        self.repo.find_all().await
    }
}

#[async_trait]
impl<R, L, C> UpdateClient for ClientService<R, L, C>
where
    R: FindById
        + FindByDocument
        + FindAll
        + CreateClient
        + UpdateClientRepo
        + DeleteClient
        + Send
        + Sync,
    L: Send + Sync,
    C: Send + Sync,
{
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
impl<R, L, C> ActivateClient for ClientService<R, L, C>
where
    R: FindById
        + FindByDocument
        + FindAll
        + CreateClient
        + UpdateClientRepo
        + DeleteClient
        + Send
        + Sync,
    L: Send + Sync,
    C: Send + Sync,
{
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
impl<R, L, C> DeactivateClient for ClientService<R, L, C>
where
    R: FindById
        + FindByDocument
        + FindAll
        + CreateClient
        + UpdateClientRepo
        + DeleteClient
        + Send
        + Sync,
    L: Send + Sync,
    C: Send + Sync,
{
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
impl<R, L, C> DeleteClientTrait for ClientService<R, L, C>
where
    R: FindById
        + FindByDocument
        + FindAll
        + CreateClient
        + UpdateClientRepo
        + DeleteClient
        + Send
        + Sync,
    L: Send + Sync,
    C: Send + Sync,
{
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ClientError::NotFound { uuid })?;

        self.repo.delete(uuid).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::errors::ClientError;
    use crate::domain::models::db::client_row::{ClientRow, CreateClientRow, UpdateClientRow};
    use crate::domain::ports::client_repository::{
        CreateClient, DeleteClient, FindAll, FindByDocument, FindById,
        UpdateClient as UpdateClientRepo,
    };
    use crate::domain::ports::client_use_cases::{
        ActivateClient, DeactivateClient, DeleteClient as DeleteClientTrait, FindClientByDocument,
        FindClientById, ListClients, RegisterClient, RegisterClientInput, UpdateClient,
        UpdateClientInput,
    };

    #[derive(Default)]
    struct MockRepo {
        find_by_id_result: Option<ClientRow>,
        find_by_document_result: Option<ClientRow>,
        find_all_result: Vec<ClientRow>,
    }

    impl MockRepo {
        fn new() -> Self {
            Self::default()
        }
    }

    use sqlx::types::chrono::NaiveDateTime;

    fn now() -> NaiveDateTime {
        NaiveDateTime::default()
    }

    fn make_row() -> ClientRow {
        ClientRow {
            pk_client: Uuid::now_v7(),
            idx_client: 1,
            tx_name: "Test Client".to_string(),
            tx_status: "active".to_string(),
            tx_doc: "12345678909".to_string(),
            ts_client_created_at: now(),
            ts_client_updated_at: now(),
        }
    }

    #[async_trait]
    impl FindById for MockRepo {
        async fn find_by_id(&self, _uuid: Uuid) -> Result<Option<ClientRow>, ClientError> {
            Ok(self.find_by_id_result.clone())
        }
    }

    #[async_trait]
    impl FindByDocument for MockRepo {
        async fn find_by_document(&self, _doc: &str) -> Result<Option<ClientRow>, ClientError> {
            Ok(self.find_by_document_result.clone())
        }
    }

    #[async_trait]
    impl FindAll for MockRepo {
        async fn find_all(&self) -> Result<Vec<ClientRow>, ClientError> {
            Ok(self.find_all_result.clone())
        }
    }

    #[async_trait]
    impl CreateClient for MockRepo {
        async fn create(&self, input: CreateClientRow) -> Result<ClientRow, ClientError> {
            Ok(ClientRow {
                pk_client: Uuid::now_v7(),
                idx_client: 0,
                tx_name: input.tx_name,
                tx_status: input.tx_status.to_string(),
                tx_doc: input.tx_doc,
                ts_client_created_at: now(),
                ts_client_updated_at: now(),
            })
        }
    }

    #[async_trait]
    impl UpdateClientRepo for MockRepo {
        async fn update(
            &self,
            uuid: Uuid,
            _input: UpdateClientRow,
        ) -> Result<ClientRow, ClientError> {
            Ok(ClientRow {
                pk_client: uuid,
                idx_client: 0,
                tx_name: "".to_string(),
                tx_status: "".to_string(),
                tx_doc: "".to_string(),
                ts_client_created_at: now(),
                ts_client_updated_at: now(),
            })
        }
    }

    #[async_trait]
    impl DeleteClient for MockRepo {
        async fn delete(&self, uuid: Uuid) -> Result<ClientRow, ClientError> {
            Ok(ClientRow {
                pk_client: uuid,
                idx_client: 0,
                tx_name: "".to_string(),
                tx_status: "".to_string(),
                tx_doc: "".to_string(),
                ts_client_created_at: now(),
                ts_client_updated_at: now(),
            })
        }
    }

    struct MockLocationService;
    struct MockContactService;

    fn make_service_with_repo(
        repo: MockRepo,
    ) -> ClientService<MockRepo, MockLocationService, MockContactService> {
        ClientService::new(
            repo,
            Arc::new(MockLocationService),
            Arc::new(MockContactService),
            None,
        )
    }

    fn make_service() -> ClientService<MockRepo, MockLocationService, MockContactService> {
        make_service_with_repo(MockRepo::new())
    }

    #[tokio::test]
    async fn find_client_returns_row_when_exists() {
        let row = make_row();
        let uuid = row.pk_client;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = make_service_with_repo(repo);
        let result = FindClientById::execute(&service, uuid).await.unwrap();
        assert_eq!(result.pk_client, uuid);
        assert_eq!(result.tx_name, "Test Client");
    }

    #[tokio::test]
    async fn find_client_returns_not_found_when_missing() {
        let uuid = Uuid::now_v7();
        let service = make_service();
        let result = FindClientById::execute(&service, uuid).await;
        assert!(matches!(result, Err(ClientError::NotFound { .. })));
    }

    #[tokio::test]
    async fn find_client_by_document_returns_row() {
        let row = make_row();
        let mut repo = MockRepo::new();
        repo.find_by_document_result = Some(row.clone());
        let service = make_service_with_repo(repo);
        let result = FindClientByDocument::execute(&service, "12345678909")
            .await
            .unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().tx_doc, "12345678909");
    }

    #[tokio::test]
    async fn list_clients_returns_all() {
        let r1 = make_row();
        let r2 = make_row();
        let mut repo = MockRepo::new();
        repo.find_all_result = vec![r1, r2];
        let service = make_service_with_repo(repo);
        let result = ListClients::execute(&service).await.unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn list_clients_returns_empty() {
        let service = make_service();
        let result = ListClients::execute(&service).await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn update_client_returns_not_found_when_missing() {
        let uuid = Uuid::now_v7();
        let service = make_service();
        let input = UpdateClientInput {
            name: Some("Updated".to_string()),
            doc: None,
        };
        let result = UpdateClient::execute(&service, uuid, input).await;
        assert!(matches!(result, Err(ClientError::NotFound { .. })));
    }

    #[tokio::test]
    async fn activate_client_succeeds_when_inactive() {
        let mut row = make_row();
        row.tx_status = "inactive".to_string();
        let uuid = row.pk_client;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = make_service_with_repo(repo);
        let result = ActivateClient::execute(&service, uuid).await.unwrap();
        assert_eq!(result.pk_client, uuid);
    }

    #[tokio::test]
    async fn activate_client_fails_when_already_active() {
        let row = make_row();
        let uuid = row.pk_client;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = make_service_with_repo(repo);
        let result = ActivateClient::execute(&service, uuid).await;
        assert!(matches!(result, Err(ClientError::AlreadyActive { .. })));
    }

    #[tokio::test]
    async fn deactivate_client_succeeds_when_active() {
        let row = make_row();
        let uuid = row.pk_client;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = make_service_with_repo(repo);
        let result = DeactivateClient::execute(&service, uuid).await.unwrap();
        assert_eq!(result.pk_client, uuid);
    }

    #[tokio::test]
    async fn deactivate_client_fails_when_already_inactive() {
        let mut row = make_row();
        row.tx_status = "inactive".to_string();
        let uuid = row.pk_client;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = make_service_with_repo(repo);
        let result = DeactivateClient::execute(&service, uuid).await;
        assert!(matches!(result, Err(ClientError::AlreadyInactive { .. })));
    }

    #[tokio::test]
    async fn delete_client_succeeds_when_exists() {
        let row = make_row();
        let uuid = row.pk_client;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = make_service_with_repo(repo);
        let result = DeleteClientTrait::execute(&service, uuid).await.unwrap();
        assert_eq!(result.pk_client, uuid);
    }

    #[tokio::test]
    async fn delete_client_fails_when_not_found() {
        let uuid = Uuid::now_v7();
        let service = make_service();
        let result = DeleteClientTrait::execute(&service, uuid).await;
        assert!(matches!(result, Err(ClientError::NotFound { .. })));
    }

    #[test]
    fn error_not_found_message_contains_uuid() {
        let uuid = Uuid::now_v7();
        let err = ClientError::NotFound { uuid };
        let msg = err.to_string();
        assert!(msg.contains(&uuid.to_string()));
        assert!(msg.contains("não encontrado"));
    }

    #[test]
    fn error_document_already_exists_message_contains_doc() {
        let err = ClientError::DocumentAlreadyExists {
            doc: "12345678909".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("documento já cadastrado"));
    }

    #[test]
    fn error_already_active_message_contains_uuid() {
        let uuid = Uuid::now_v7();
        let err = ClientError::AlreadyActive { uuid };
        let msg = err.to_string();
        assert!(msg.contains(&uuid.to_string()));
        assert!(msg.contains("já está ativo"));
    }

    #[test]
    fn error_already_inactive_message_contains_uuid() {
        let uuid = Uuid::now_v7();
        let err = ClientError::AlreadyInactive { uuid };
        let msg = err.to_string();
        assert!(msg.contains(&uuid.to_string()));
        assert!(msg.contains("já está inativo"));
    }
}
