use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::adapters::driven::postgres::pg_client_repository::PgClientRepository;
use crate::domain::errors::ClientError;
use crate::domain::models::db::client_row::{ClientRow, ClientStatus, UpdateClientRow};
use crate::domain::ports::client_repository::{
    CreateClient, CreateClientWithTx, DeleteClient, FindAll, FindByDocument, FindById,
    UpdateClient as UpdateClientRepo,
};
use crate::domain::ports::client_use_cases::{
    ActivateClient, DeactivateClient, DeleteClient as DeleteClientTrait, FindClientByDocument,
    FindClientById, ListClients, RegisterClient, RegisterClientInput, UpdateClient,
    UpdateClientInput,
};
use contact::adapters::driven::postgres::pg_contact_repository::PgContactRepository;
use contact::adapters::driven::postgres::pg_phone_repository::PgPhoneRepository;
use contact::domain::models::db::contact_row::CreateContactRow as ContactCreateRow;
use location::adapters::driven::postgres::pg_location_repository::PgLocationRepository;
use location::domain::models::db::location_row::CreateLocationRow;
use types::doc::Doc;
use types::phone::Phone;

pub struct ClientService<R> {
    repo: R,
}

impl<R> ClientService<R>
where
    R: FindById + FindByDocument + FindAll + CreateClient + UpdateClientRepo + DeleteClient,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

pub struct ConcreteClientService {
    repo: PgClientRepository,
    pool: PgPool,
}

impl ConcreteClientService {
    pub fn new(repo: PgClientRepository, pool: PgPool) -> Self {
        Self { repo, pool }
    }
}

fn compute_location_hash(
    street: &str,
    number: &str,
    city: &str,
    state: &str,
    zipcode: &str,
) -> i64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    street.hash(&mut hasher);
    number.hash(&mut hasher);
    city.hash(&mut hasher);
    state.hash(&mut hasher);
    zipcode.hash(&mut hasher);
    hasher.finish() as i64
}

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

        let mut tx = self.pool.begin().await.map_err(|e| {
            ClientError::Infra(types::errors::infra_error::InfraError::BeginTransaction {
                source: e,
            })
        })?;

        let location_hash = compute_location_hash(
            &input.street,
            &input.number,
            &input.city,
            &input.state,
            &input.cep,
        );

        let location_row =
            match PgLocationRepository::find_by_hash_with_executor(&mut *tx, location_hash)
                .await
                .map_err(|e| {
                    ClientError::Infra(types::errors::infra_error::InfraError::Database {
                        action: "find location",
                        source: e,
                    })
                })? {
                Some(existing) => existing,
                None => PgLocationRepository::create_with_executor(
                    &mut *tx,
                    CreateLocationRow {
                        tx_street: input.street.clone(),
                        tx_number: input.number.clone(),
                        tx_city: input.city.clone(),
                        tx_state: input.state.clone(),
                        tx_zipcode: input.cep.clone(),
                        tx_public_space: "".to_string(),
                        tx_address_complement: input.complement.clone(),
                        tx_unit: "".to_string(),
                        tx_neighborhood: "".to_string(),
                        tx_locality: input.city.clone(),
                        tx_region: input.state.clone(),
                        tx_ibge: None,
                        tx_gia: None,
                        tx_ddd: "".to_string(),
                        tx_siafi: None,
                        nr_hash: location_hash,
                    },
                )
                .await
                .map_err(|e| {
                    ClientError::Infra(types::errors::infra_error::InfraError::Database {
                        action: "create location",
                        source: e,
                    })
                })?,
            };

        let contact_row = PgContactRepository::create_with_executor(
            &mut *tx,
            ContactCreateRow {
                tx_email: input.email.clone(),
            },
        )
        .await
        .map_err(|e| {
            ClientError::Infra(types::errors::infra_error::InfraError::Database {
                action: "create contact",
                source: e,
            })
        })?;

        if !input.phones.is_empty() {
            PgPhoneRepository::create_many_with_executor(
                &mut *tx,
                contact_row.pk_contact,
                input.phones.clone(),
            )
            .await
            .map_err(|e| {
                ClientError::Infra(types::errors::infra_error::InfraError::Database {
                    action: "create phones",
                    source: e,
                })
            })?;
        }

        let client_row = PgClientRepository::create_with_tx(
            &self.repo,
            &mut tx,
            crate::domain::models::db::client_row::CreateClientRow {
                tx_name: input.name,
                tx_status: ClientStatus::Active,
                tx_doc: input.doc,
            },
        )
        .await?;

        PgClientRepository::create_contact(&mut *tx, client_row.pk_client, contact_row.pk_contact)
            .await
            .map_err(|e| {
                ClientError::Infra(types::errors::infra_error::InfraError::Database {
                    action: "create client-contact link",
                    source: e,
                })
            })?;

        PgClientRepository::create_address(
            &mut *tx,
            client_row.pk_client,
            location_row.pk_location,
        )
        .await
        .map_err(|e| {
            ClientError::Infra(types::errors::infra_error::InfraError::Database {
                action: "create client-address link",
                source: e,
            })
        })?;

        tx.commit().await.map_err(|e| {
            ClientError::Infra(types::errors::infra_error::InfraError::CommitTransaction {
                source: e,
            })
        })?;

        Ok(client_row)
    }
}

#[async_trait]
impl FindClientById for ConcreteClientService {
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ClientError::NotFound { uuid })
    }
}

#[async_trait]
impl FindClientByDocument for ConcreteClientService {
    async fn execute(&self, doc: &str) -> Result<Option<ClientRow>, ClientError> {
        self.repo.find_by_document(doc).await
    }
}

#[async_trait]
impl ListClients for ConcreteClientService {
    async fn execute(&self) -> Result<Vec<ClientRow>, ClientError> {
        self.repo.find_all().await
    }
}

#[async_trait]
impl UpdateClient for ConcreteClientService {
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
impl ActivateClient for ConcreteClientService {
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
impl DeactivateClient for ConcreteClientService {
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
impl DeleteClientTrait for ConcreteClientService {
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ClientError::NotFound { uuid })?;

        self.repo.delete(uuid).await
    }
}

#[async_trait]
impl<R> FindClientById for ClientService<R>
where
    R: FindById
        + FindByDocument
        + FindAll
        + CreateClient
        + UpdateClientRepo
        + DeleteClient
        + Send
        + Sync,
{
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ClientError::NotFound { uuid })
    }
}

#[async_trait]
impl<R> FindClientByDocument for ClientService<R>
where
    R: FindById
        + FindByDocument
        + FindAll
        + CreateClient
        + UpdateClientRepo
        + DeleteClient
        + Send
        + Sync,
{
    async fn execute(&self, doc: &str) -> Result<Option<ClientRow>, ClientError> {
        self.repo.find_by_document(doc).await
    }
}

#[async_trait]
impl<R> ListClients for ClientService<R>
where
    R: FindById
        + FindByDocument
        + FindAll
        + CreateClient
        + UpdateClientRepo
        + DeleteClient
        + Send
        + Sync,
{
    async fn execute(&self) -> Result<Vec<ClientRow>, ClientError> {
        self.repo.find_all().await
    }
}

#[async_trait]
impl<R> UpdateClient for ClientService<R>
where
    R: FindById
        + FindByDocument
        + FindAll
        + CreateClient
        + UpdateClientRepo
        + DeleteClient
        + Send
        + Sync,
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
impl<R> ActivateClient for ClientService<R>
where
    R: FindById
        + FindByDocument
        + FindAll
        + CreateClient
        + UpdateClientRepo
        + DeleteClient
        + Send
        + Sync,
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
impl<R> DeactivateClient for ClientService<R>
where
    R: FindById
        + FindByDocument
        + FindAll
        + CreateClient
        + UpdateClientRepo
        + DeleteClient
        + Send
        + Sync,
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
impl<R> DeleteClientTrait for ClientService<R>
where
    R: FindById
        + FindByDocument
        + FindAll
        + CreateClient
        + UpdateClientRepo
        + DeleteClient
        + Send
        + Sync,
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
        FindClientById, ListClients, UpdateClient, UpdateClientInput,
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

    #[tokio::test]
    async fn find_client_returns_row_when_exists() {
        let row = make_row();
        let uuid = row.pk_client;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = ClientService::new(repo);
        let result = FindClientById::execute(&service, uuid).await.unwrap();
        assert_eq!(result.pk_client, uuid);
        assert_eq!(result.tx_name, "Test Client");
    }

    #[tokio::test]
    async fn find_client_returns_not_found_when_missing() {
        let uuid = Uuid::now_v7();
        let repo = MockRepo::new();
        let service = ClientService::new(repo);
        let result = FindClientById::execute(&service, uuid).await;
        assert!(matches!(result, Err(ClientError::NotFound { .. })));
    }

    #[tokio::test]
    async fn find_client_by_document_returns_row() {
        let row = make_row();
        let mut repo = MockRepo::new();
        repo.find_by_document_result = Some(row.clone());
        let service = ClientService::new(repo);
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
        let service = ClientService::new(repo);
        let result = ListClients::execute(&service).await.unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn list_clients_returns_empty() {
        let repo = MockRepo::new();
        let service = ClientService::new(repo);
        let result = ListClients::execute(&service).await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn update_client_returns_not_found_when_missing() {
        let uuid = Uuid::now_v7();
        let repo = MockRepo::new();
        let service = ClientService::new(repo);
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
        let service = ClientService::new(repo);
        let result = ActivateClient::execute(&service, uuid).await.unwrap();
        assert_eq!(result.pk_client, uuid);
    }

    #[tokio::test]
    async fn activate_client_fails_when_already_active() {
        let row = make_row();
        let uuid = row.pk_client;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = ClientService::new(repo);
        let result = ActivateClient::execute(&service, uuid).await;
        assert!(matches!(result, Err(ClientError::AlreadyActive { .. })));
    }

    #[tokio::test]
    async fn deactivate_client_succeeds_when_active() {
        let row = make_row();
        let uuid = row.pk_client;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = ClientService::new(repo);
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
        let service = ClientService::new(repo);
        let result = DeactivateClient::execute(&service, uuid).await;
        assert!(matches!(result, Err(ClientError::AlreadyInactive { .. })));
    }

    #[tokio::test]
    async fn delete_client_succeeds_when_exists() {
        let row = make_row();
        let uuid = row.pk_client;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = ClientService::new(repo);
        let result = DeleteClientTrait::execute(&service, uuid).await.unwrap();
        assert_eq!(result.pk_client, uuid);
    }

    #[tokio::test]
    async fn delete_client_fails_when_not_found() {
        let uuid = Uuid::now_v7();
        let repo = MockRepo::new();
        let service = ClientService::new(repo);
        let result = DeleteClientTrait::execute(&service, uuid).await;
        assert!(matches!(result, Err(ClientError::NotFound { .. })));
    }

    #[test]
    fn error_not_found_message_contains_uuid() {
        let uuid = Uuid::now_v7();
        let err = ClientError::NotFound { uuid };
        let msg = err.to_string();
        assert!(msg.contains(&uuid.to_string()));
        assert!(msg.contains("not found"));
    }

    #[test]
    fn error_document_already_exists_message_contains_doc() {
        let err = ClientError::DocumentAlreadyExists {
            doc: "12345678909".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("document already registered"));
    }

    #[test]
    fn error_already_active_message_contains_uuid() {
        let uuid = Uuid::now_v7();
        let err = ClientError::AlreadyActive { uuid };
        let msg = err.to_string();
        assert!(msg.contains(&uuid.to_string()));
        assert!(msg.contains("already active"));
    }

    #[test]
    fn error_already_inactive_message_contains_uuid() {
        let uuid = Uuid::now_v7();
        let err = ClientError::AlreadyInactive { uuid };
        let msg = err.to_string();
        assert!(msg.contains(&uuid.to_string()));
        assert!(msg.contains("already inactive"));
    }
}
