use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::adapters::driven::postgres::pg_client_repository::PgClientRepository;
use crate::domain::errors::{ClientError, InfraError};

use crate::domain::models::db::client_row::{ClientRow, ClientStatus, CreateClientRow};
use location::domain::model::LocationRow;
use sqlx::PgPool;
use types::cep::Cep;
use types::doc::Doc;
use types::email::Email;
use types::phone::Phone;
use uuid::Uuid;
use viacep::domain::models::viacep_model::ViaCepAddressModel;
use viacep::domain::ports::viacep_port::ViaCepPort;

pub struct RegisterClientInput {
    pub name: String,
    pub doc: String,
    pub email: String,
    pub phones: Vec<String>,
    pub cep: String,
    pub number: String,
    pub complement: String,
}

struct ValidatedInput {
    doc: Doc,
    email: Email,
    phones: Vec<Phone>,
    cep: Cep,
    name: String,
    number: String,
    complement: String,
}

pub struct RegisterClientService;

fn sqlx_to_client(action: &'static str) -> impl FnOnce(sqlx::Error) -> ClientError {
    move |e| ClientError::Infra(InfraError::Database { action, source: e })
}

impl RegisterClientService {
    pub async fn execute(
        pool: &PgPool,
        viacep: &dyn ViaCepPort,
        input: RegisterClientInput,
    ) -> Result<ClientRow, ClientError> {
        let input = Self::validate_input(input)?;
        Self::ensure_document_available(pool, &input.doc).await?;
        Self::ensure_email_available(pool, &input.email).await?;
        Self::ensure_phones_available(pool, &input.phones).await?;

        let viacep_address = Self::fetch_address(viacep, &input.cep).await?;
        let location = Self::find_or_create_location(pool, &viacep_address, &input).await?;

        Self::persist_client(pool, &input, location.pk_location).await
    }

    async fn ensure_document_available(pool: &PgPool, doc: &Doc) -> Result<(), ClientError> {
        if PgClientRepository::find_by_document(pool, doc.to_string())
            .await
            .map_err(sqlx_to_client("buscar cliente por documento"))?
            .is_some()
        {
            return Err(ClientError::DocumentAlreadyExists {
                doc: doc.to_string(),
            });
        }
        Ok(())
    }

    fn validate_input(input: RegisterClientInput) -> Result<ValidatedInput, ClientError> {
        let doc: Doc = input.doc.try_into()?;
        let email: Email = input.email.try_into()?;
        let phones: Vec<Phone> = input
            .phones
            .into_iter()
            .map(|p| p.try_into())
            .collect::<Result<Vec<_>, _>>()?;
        let cep: Cep = input.cep.try_into()?;

        Ok(ValidatedInput {
            doc,
            email,
            phones,
            cep,
            name: input.name,
            number: input.number,
            complement: input.complement,
        })
    }

    async fn ensure_email_available(_pool: &PgPool, _email: &Email) -> Result<(), ClientError> {
        // TODO: Implement contact query in client crate or use contact crate
        Ok(())
    }

    async fn ensure_phones_available(_pool: &PgPool, _phones: &[Phone]) -> Result<(), ClientError> {
        // TODO: Implement phone query in client crate or use contact crate
        Ok(())
    }

    async fn fetch_address(
        viacep: &dyn ViaCepPort,
        cep: &Cep,
    ) -> Result<ViaCepAddressModel, ClientError> {
        Ok(viacep.fetch_address(cep.as_ref()).await?)
    }

    async fn find_or_create_location(
        _pool: &PgPool,
        _viacep_address: &ViaCepAddressModel,
        _input: &ValidatedInput,
    ) -> Result<LocationRow, ClientError> {
        // TODO: Implement location logic in client crate or use location crate
        todo!("Implement location logic")
    }

    async fn persist_client(
        pool: &PgPool,
        input: &ValidatedInput,
        location_uuid: Uuid,
    ) -> Result<ClientRow, ClientError> {
        let mut tx = pool
            .begin()
            .await
            .map_err(|e| ClientError::Infra(InfraError::BeginTransaction { source: e }))?;

        // TODO: Implement contact creation in client crate or use contact crate
        // For now, create a placeholder contact UUID
        let contact_uuid = Uuid::now_v7();

        let client = PgClientRepository::create(
            &mut *tx,
            CreateClientRow {
                tx_name: input.name.clone(),
                tx_status: ClientStatus::Active,
                tx_doc: input.doc.to_string(),
            },
        )
        .await
        .map_err(sqlx_to_client("criar cliente"))?;

        PgClientRepository::create_contact(&mut *tx, client.pk_client, contact_uuid)
            .await
            .map_err(sqlx_to_client("vincular cliente-contato"))?;

        PgClientRepository::create_address(&mut *tx, client.pk_client, location_uuid)
            .await
            .map_err(sqlx_to_client("vincular cliente-endereço"))?;

        tx.commit()
            .await
            .map_err(|e| ClientError::Infra(InfraError::CommitTransaction { source: e }))?;

        Ok(client)
    }
}

fn compute_location_hash(addr: &ViaCepAddressModel) -> i64 {
    let mut hasher = DefaultHasher::new();
    addr.logradouro.hash(&mut hasher);
    addr.bairro.hash(&mut hasher);
    addr.localidade.hash(&mut hasher);
    addr.uf.hash(&mut hasher);
    addr.cep.hash(&mut hasher);
    hasher.finish() as i64
}
