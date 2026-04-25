use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::client_error::ClientError;
use crate::domain::models::db::client_row::{ClientRow, ClientStatus, CreateClientRow};
use crate::domain::ports::use_cases::contact_use_cases::RegisterContactInput;
use crate::domain::ports::use_cases::location_use_cases::CreateLocationInput;

#[derive(Debug, Clone)]
pub struct RegisterClientLocationInput {
    pub street: String,
    pub number: String,
    pub city: String,
    pub state: String,
    pub zipcode: String,
    pub complement: String,
    pub public_space: String,
    pub unit: String,
    pub neighborhood: String,
    pub locality: String,
    pub region: String,
    pub ibge: Option<String>,
    pub gia: Option<String>,
    pub ddd: String,
    pub siafi: Option<String>,
}

impl From<RegisterClientLocationInput> for CreateLocationInput {
    fn from(input: RegisterClientLocationInput) -> Self {
        Self {
            street: input.street,
            number: input.number,
            city: input.city,
            state: input.state,
            zipcode: input.zipcode,
            complement: input.complement,
            public_space: input.public_space,
            unit: input.unit,
            neighborhood: input.neighborhood,
            locality: input.locality,
            region: input.region,
            ibge: input.ibge,
            gia: input.gia,
            ddd: input.ddd,
            siafi: input.siafi,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RegisterClientContactInput {
    pub email: String,
    pub phones: Vec<String>,
}

impl From<RegisterClientContactInput> for RegisterContactInput {
    fn from(input: RegisterClientContactInput) -> Self {
        Self { email: input.email }
    }
}

#[derive(Debug, Clone)]
pub struct RegisterClientInput {
    pub name: String,
    pub doc: String,
    pub status: ClientStatus,
    pub location: Option<RegisterClientLocationInput>,
    pub contact: Option<RegisterClientContactInput>,
}

impl From<RegisterClientInput> for CreateClientRow {
    fn from(input: RegisterClientInput) -> Self {
        Self {
            tx_name: input.name,
            tx_status: input.status,
            tx_doc: input.doc,
        }
    }
}

pub struct UpdateClientInput {
    pub name: Option<String>,
    pub doc: Option<String>,
}

#[async_trait]
pub trait FindClientByIdUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError>;
}

#[async_trait]
pub trait FindClientByDocumentUseCase: Send + Sync {
    async fn execute(&self, doc: &str) -> Result<Option<ClientRow>, ClientError>;
}

#[async_trait]
pub trait ListClientsUseCase: Send + Sync {
    async fn execute(&self) -> Result<Vec<ClientRow>, ClientError>;
}

// Mutations
#[async_trait]
pub trait RegisterClientUseCase: Send + Sync {
    async fn execute(&self, input: RegisterClientInput) -> Result<ClientRow, ClientError>;
}

#[async_trait]
pub trait UpdateClientUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid, input: UpdateClientInput)
    -> Result<ClientRow, ClientError>;
}

// --- Contact ---
#[async_trait]
pub trait UpdateClientEmailUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid, email: String) -> Result<ClientRow, ClientError>;
}

#[async_trait]
pub trait UpdateClientPhoneUseCase: Send + Sync {
    async fn execute(
        &self,
        uuid: Uuid,
        phone: String,
        new_phone: String,
    ) -> Result<ClientRow, ClientError>;
}

#[async_trait]
pub trait AddClientPhoneUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid, phone: String) -> Result<ClientRow, ClientError>;
}

// --- Update Location ---
#[async_trait]
pub trait UpdateClientLocationUseCase: Send + Sync {
    async fn execute(
        &self,
        uuid: Uuid,
        input: RegisterClientLocationInput,
    ) -> Result<ClientRow, ClientError>;
}

#[async_trait]
pub trait ActivateClientUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError>;
}

#[async_trait]
pub trait DeactivateClientUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError>;
}

#[async_trait]
pub trait DeleteClientUseCase: Send + Sync {
    async fn execute(&self, uuid: Uuid) -> Result<ClientRow, ClientError>;
}
