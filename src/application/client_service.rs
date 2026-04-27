use async_trait::async_trait;
use uuid::Uuid;

use crate::adapters::driven::pg_client_repository::PgClientRepository;
use crate::application::contact_service::PgContactService;
use crate::application::location_service::PgLocationService;
use crate::domain::errors::client_error::ClientError;
use crate::domain::models::db::client_row::{
    ClientRow, ClientStatus, CreateClientRow, UpdateClientRow,
};
use crate::domain::models::db::location_row::LocationRow;
use crate::domain::models::db::phone_row::PhoneRow;
use crate::domain::ports::repositories::client_repository::ClientRepository;
use crate::domain::ports::use_cases::client_use_cases::{
    ActivateClientUseCase, AddClientPhonesUseCase, DeactivateClientUseCase, DeleteClientUseCase,
    FindClientByDocumentUseCase, FindClientByIdUseCase, ListClientsUseCase, RegisterClientInput,
    RegisterClientLocationInput, RegisterClientUseCase, UpdateClientEmailUseCase,
    UpdateClientInput, UpdateClientLocationUseCase, UpdateClientPhoneUseCase, UpdateClientUseCase,
};
use crate::domain::ports::use_cases::contact_use_cases::{
    AddPhonesUseCase, RegisterContactUseCase, UpdateContactEmailUseCase, UpdatePhoneUseCase,
};
use crate::domain::ports::use_cases::location_use_cases::{
    CreateLocationUseCase, UpdateLocationUseCase,
};
use crate::domain::value_objects::doc::Doc;

// =============================================================================
// ClientService<R, L, C> — Generic service parametrizado por repositório,
// location service e contact service.
// =============================================================================

pub struct ClientService<R, L, C> {
    repo: R,
    location_service: L,
    contact_service: C,
}

impl<R, L, C> ClientService<R, L, C>
where
    R: ClientRepository,
    L: Send + Sync,
    C: Send + Sync,
{
    pub fn new(repo: R, location_service: L, contact_service: C) -> Self {
        Self {
            repo,
            location_service,
            contact_service,
        }
    }
}

/// Type alias para produção — evita propagar os 3 type params nas rotas e no startup.
pub type PgClientService = ClientService<PgClientRepository, PgLocationService, PgContactService>;

// =============================================================================
// CRUD — dependem apenas de R: ClientRepository
// =============================================================================

#[async_trait]
impl<R, L, C> FindClientByIdUseCase for ClientService<R, L, C>
where
    R: ClientRepository,
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
impl<R, L, C> FindClientByDocumentUseCase for ClientService<R, L, C>
where
    R: ClientRepository,
    L: Send + Sync,
    C: Send + Sync,
{
    async fn execute(&self, doc: &str) -> Result<Option<ClientRow>, ClientError> {
        self.repo.find_by_document(doc).await
    }
}

#[async_trait]
impl<R, L, C> ListClientsUseCase for ClientService<R, L, C>
where
    R: ClientRepository,
    L: Send + Sync,
    C: Send + Sync,
{
    async fn execute(&self) -> Result<Vec<ClientRow>, ClientError> {
        self.repo.find_all().await
    }
}

#[async_trait]
impl<R, L, C> UpdateClientUseCase for ClientService<R, L, C>
where
    R: ClientRepository,
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
            if self.repo.find_by_document(doc).await?.is_some() {
                return Err(ClientError::DocumentAlreadyExists { doc: doc.clone() });
            }
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
impl<R, L, C> ActivateClientUseCase for ClientService<R, L, C>
where
    R: ClientRepository,
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
impl<R, L, C> DeactivateClientUseCase for ClientService<R, L, C>
where
    R: ClientRepository,
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
impl<R, L, C> DeleteClientUseCase for ClientService<R, L, C>
where
    R: ClientRepository,
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

// =============================================================================
// Use cases compostos — dependem de R + L + C
// =============================================================================

#[async_trait]
impl<R, L, C> RegisterClientUseCase for ClientService<R, L, C>
where
    R: ClientRepository,
    L: CreateLocationUseCase,
    C: RegisterContactUseCase + AddPhonesUseCase,
{
    async fn execute(&self, input: RegisterClientInput) -> Result<ClientRow, ClientError> {
        if self
            .repo
            .find_by_document(&input.doc)
            .await?
            .is_some()
        {
            return Err(ClientError::DocumentAlreadyExists { doc: input.doc });
        };

        let location_uuid = if let Some(location) = &input.location {
            let pk_location =
                CreateLocationUseCase::execute(&self.location_service, location.clone().into())
                    .await?
                    .pk_location;
            Some(pk_location)
        } else {
            None
        };

        let contact_uuid = if let Some(contact) = &input.contact {
            let pk_contact =
                RegisterContactUseCase::execute(&self.contact_service, contact.clone().into())
                    .await?
                    .pk_contact;

            if !contact.phones.is_empty() {
                AddPhonesUseCase::execute(
                    &self.contact_service,
                    pk_contact,
                    contact.phones.clone(),
                )
                .await?;
            }
            Some(pk_contact)
        } else {
            None
        };

        let created_client = self
            .repo
            .create(CreateClientRow::from(input.clone()))
            .await?;

        if let Some(location_uuid) = location_uuid {
            self.repo
                .link_created_location_to_client(location_uuid, created_client.pk_client)
                .await?;
        }

        if let Some(contact_uuid) = contact_uuid {
            self.repo
                .link_created_contact_to_client(contact_uuid, created_client.pk_client)
                .await?;
        }

        Ok(created_client)
    }
}

#[async_trait]
impl<R, L, C> UpdateClientEmailUseCase for ClientService<R, L, C>
where
    R: ClientRepository,
    L: Send + Sync,
    C: UpdateContactEmailUseCase,
{
    async fn execute(&self, client_uuid: Uuid, email: String) -> Result<ClientRow, ClientError> {
        let client = self
            .repo
            .find_by_id(client_uuid)
            .await?
            .ok_or(ClientError::NotFound { uuid: client_uuid })?;

        let client_contact = self
            .repo
            .find_contact_by_client_id(client_uuid)
            .await?
            .ok_or(ClientError::ContactNotFound { client_uuid })?;

        UpdateContactEmailUseCase::execute(&self.contact_service, client_contact.fk_contact, email)
            .await?;

        Ok(client)
    }
}

#[async_trait]
impl<R, L, C> UpdateClientPhoneUseCase for ClientService<R, L, C>
where
    R: ClientRepository,
    L: Send + Sync,
    C: UpdatePhoneUseCase,
{
    async fn execute(
        &self,
        uuid: Uuid,
        phone: String,
        new_phone: String,
    ) -> Result<PhoneRow, ClientError> {
        let _ = self
            .repo
            .find_by_id(uuid)
            .await?
            .ok_or(ClientError::NotFound { uuid })?;

        let client_contact = self
            .repo
            .find_contact_by_client_id(uuid)
            .await?
            .ok_or(ClientError::ContactNotFound { client_uuid: uuid })?;

        UpdatePhoneUseCase::execute(
            &self.contact_service,
            client_contact.fk_contact,
            phone,
            new_phone,
        )
        .await
        .map_err(ClientError::from)
    }
}

#[async_trait]
impl<R, L, C> AddClientPhonesUseCase for ClientService<R, L, C>
where
    R: ClientRepository,
    L: Send + Sync,
    C: AddPhonesUseCase,
{
    async fn execute(&self, uuid: Uuid, phones: Vec<String>) -> Result<ClientRow, ClientError> {
        let client = self
            .repo
            .find_by_id(uuid)
            .await?
            .ok_or(ClientError::NotFound { uuid })?;

        let client_contact = self
            .repo
            .find_contact_by_client_id(uuid)
            .await?
            .ok_or(ClientError::ContactNotFound { client_uuid: uuid })?;

        AddPhonesUseCase::execute(&self.contact_service, client_contact.fk_contact, phones).await?;

        Ok(client)
    }
}

#[async_trait]
impl<R, L, C> UpdateClientLocationUseCase for ClientService<R, L, C>
where
    R: ClientRepository,
    L: UpdateLocationUseCase,
    C: Send + Sync,
{
    async fn execute(
        &self,
        uuid: Uuid,
        input: RegisterClientLocationInput,
    ) -> Result<LocationRow, ClientError> {
        let _ = self
            .repo
            .find_by_id(uuid)
            .await?
            .ok_or(ClientError::NotFound { uuid })?;

        let client_address = self
            .repo
            .find_location_by_client_id(uuid)
            .await?
            .ok_or(ClientError::LocationNotFound { client_uuid: uuid })?;

        let updated_client_address = UpdateLocationUseCase::execute(
            &self.location_service,
            client_address.fk_address,
            input.into(),
        )
        .await?;

        Ok(updated_client_address)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::db::client_address_row::ClientAddressRow;
    use crate::domain::models::db::client_contact_row::ClientContactRow;
    use crate::domain::models::db::client_row::{ClientRow, CreateClientRow, UpdateClientRow};
    use crate::domain::ports::repositories::client_repository::{
        CreateClient, DeleteClient, FindAll, FindByDocument, FindById,
        FindContactByClientId, FindLocationByClientId, LinkCreatedContactToClient,
        LinkCreatedLocationToClient, UpdateClient as UpdateClientRepo,
    };
    use crate::domain::ports::use_cases::client_use_cases::{
        ActivateClientUseCase, DeactivateClientUseCase, DeleteClientUseCase,
        FindClientByDocumentUseCase, FindClientByIdUseCase, ListClientsUseCase, UpdateClientInput,
        UpdateClientUseCase,
    };

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

    // --- Mock repo ---

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

    #[async_trait]
    impl LinkCreatedLocationToClient for MockRepo {
        async fn link_created_location_to_client(
            &self,
            location_id: Uuid,
            client_id: Uuid,
        ) -> Result<ClientAddressRow, ClientError> {
            Ok(ClientAddressRow {
                pk_client_address: Uuid::now_v7(),
                idx_client_address: 0,
                fk_client: client_id,
                fk_address: location_id,
                ts_client_address_created_at: now(),
                ts_client_address_updated_at: now(),
            })
        }
    }

    #[async_trait]
    impl LinkCreatedContactToClient for MockRepo {
        async fn link_created_contact_to_client(
            &self,
            contact_id: Uuid,
            client_id: Uuid,
        ) -> Result<ClientContactRow, ClientError> {
            Ok(ClientContactRow {
                pk_client_contact: Uuid::now_v7(),
                idx_client_contact: 0,
                fk_client: client_id,
                fk_contact: contact_id,
                ts_client_contact_created_at: now(),
                ts_client_contact_updated_at: now(),
            })
        }
    }

    #[async_trait]
    impl FindContactByClientId for MockRepo {
        async fn find_contact_by_client_id(
            &self,
            client_id: Uuid,
        ) -> Result<Option<ClientContactRow>, ClientError> {
            Ok(Some(ClientContactRow {
                pk_client_contact: Uuid::now_v7(),
                idx_client_contact: 0,
                fk_client: client_id,
                fk_contact: Uuid::now_v7(),
                ts_client_contact_created_at: now(),
                ts_client_contact_updated_at: now(),
            }))
        }
    }

    #[async_trait]
    impl FindLocationByClientId for MockRepo {
        async fn find_location_by_client_id(
            &self,
            client_id: Uuid,
        ) -> Result<Option<ClientAddressRow>, ClientError> {
            Ok(Some(ClientAddressRow {
                pk_client_address: Uuid::now_v7(),
                idx_client_address: 0,
                fk_client: client_id,
                fk_address: Uuid::now_v7(),
                ts_client_address_created_at: now(),
                ts_client_address_updated_at: now(),
            }))
        }
    }

    // --- Stub para L e C nos testes que não precisam deles ---

    struct NoOp;

    // --- Helper para construir service de teste ---

    fn make_service(repo: MockRepo) -> ClientService<MockRepo, NoOp, NoOp> {
        ClientService::new(repo, NoOp, NoOp)
    }

    // --- Tests ---

    #[tokio::test]
    async fn find_client_returns_row_when_exists() {
        let row = make_row();
        let uuid = row.pk_client;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = make_service(repo);
        let result = FindClientByIdUseCase::execute(&service, uuid)
            .await
            .unwrap();
        assert_eq!(result.pk_client, uuid);
        assert_eq!(result.tx_name, "Test Client");
    }

    #[tokio::test]
    async fn find_client_returns_not_found_when_missing() {
        let uuid = Uuid::now_v7();
        let repo = MockRepo::new();
        let service = make_service(repo);
        let result = FindClientByIdUseCase::execute(&service, uuid).await;
        assert!(matches!(result, Err(ClientError::NotFound { .. })));
    }

    #[tokio::test]
    async fn find_client_by_document_returns_row() {
        let row = make_row();
        let mut repo = MockRepo::new();
        repo.find_by_document_result = Some(row.clone());
        let service = make_service(repo);
        let result = FindClientByDocumentUseCase::execute(&service, "12345678909")
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
        let service = make_service(repo);
        let result = ListClientsUseCase::execute(&service).await.unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn list_clients_returns_empty() {
        let repo = MockRepo::new();
        let service = make_service(repo);
        let result = ListClientsUseCase::execute(&service).await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn update_client_returns_not_found_when_missing() {
        let uuid = Uuid::now_v7();
        let repo = MockRepo::new();
        let service = make_service(repo);
        let input = UpdateClientInput {
            name: Some("Updated".to_string()),
            doc: None,
        };
        let result = UpdateClientUseCase::execute(&service, uuid, input).await;
        assert!(matches!(result, Err(ClientError::NotFound { .. })));
    }

    #[tokio::test]
    async fn activate_client_succeeds_when_inactive() {
        let mut row = make_row();
        row.tx_status = "inactive".to_string();
        let uuid = row.pk_client;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = make_service(repo);
        let result = ActivateClientUseCase::execute(&service, uuid)
            .await
            .unwrap();
        assert_eq!(result.pk_client, uuid);
    }

    #[tokio::test]
    async fn activate_client_fails_when_already_active() {
        let row = make_row();
        let uuid = row.pk_client;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = make_service(repo);
        let result = ActivateClientUseCase::execute(&service, uuid).await;
        assert!(matches!(result, Err(ClientError::AlreadyActive { .. })));
    }

    #[tokio::test]
    async fn deactivate_client_succeeds_when_active() {
        let row = make_row();
        let uuid = row.pk_client;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = make_service(repo);
        let result = DeactivateClientUseCase::execute(&service, uuid)
            .await
            .unwrap();
        assert_eq!(result.pk_client, uuid);
    }

    #[tokio::test]
    async fn deactivate_client_fails_when_already_inactive() {
        let mut row = make_row();
        row.tx_status = "inactive".to_string();
        let uuid = row.pk_client;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = make_service(repo);
        let result = DeactivateClientUseCase::execute(&service, uuid).await;
        assert!(matches!(result, Err(ClientError::AlreadyInactive { .. })));
    }

    #[tokio::test]
    async fn delete_client_succeeds_when_exists() {
        let row = make_row();
        let uuid = row.pk_client;
        let mut repo = MockRepo::new();
        repo.find_by_id_result = Some(row);
        let service = make_service(repo);
        let result = DeleteClientUseCase::execute(&service, uuid).await.unwrap();
        assert_eq!(result.pk_client, uuid);
    }

    #[tokio::test]
    async fn delete_client_fails_when_not_found() {
        let uuid = Uuid::now_v7();
        let repo = MockRepo::new();
        let service = make_service(repo);
        let result = DeleteClientUseCase::execute(&service, uuid).await;
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
