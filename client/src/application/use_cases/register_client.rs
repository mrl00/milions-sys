use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::adapters::driven::postgres::pg_client_repository::PgClientRepository;
use crate::domain::errors::ClientError;
use crate::domain::models::db::client_row::{ClientRow, ClientStatus};
use crate::domain::ports::client_repository::*;
use crate::domain::use_cases::register_client::{RegisterClient, RegisterClientInput};
use location::domain::models::db::location_row::LocationRow;
use types::cep::Cep;
use types::doc::Doc;
use types::email::Email;
use types::errors::infra_error::InfraError;
use types::phone::Phone;
use viacep::domain::models::viacep_model::ViaCepAddressModel;
use viacep::domain::ports::viacep_port::ViaCepPort;

struct ValidatedInput {
    doc: Doc,
    email: Email,
    phones: Vec<Phone>,
    cep: Cep,
    name: String,
    number: String,
    complement: String,
}

fn sqlx_err(action: &'static str) -> impl FnOnce(sqlx::Error) -> ClientError {
    move |e| ClientError::Infra(InfraError::Database { action, source: e })
}

pub struct RegisterClientUseCase {
    pool: PgPool,
    viacep: Box<dyn ViaCepPort>,
}

impl RegisterClientUseCase {
    pub fn new(pool: PgPool, viacep: Box<dyn ViaCepPort>) -> Self {
        Self { pool, viacep }
    }
}

#[async_trait]
impl RegisterClient for RegisterClientUseCase {
    async fn execute(&self, input: RegisterClientInput) -> Result<ClientRow, ClientError> {
        let repo = PgClientRepository::new(self.pool.clone());

        let input = validate_input(input)?;
        ensure_document_available(&repo, &input.doc).await?;
        ensure_email_available(&input.email).await?;
        ensure_phones_available(&input.phones).await?;

        let viacep_address = fetch_address(self.viacep.as_ref(), &input.cep).await?;
        let location = find_or_create_location(&viacep_address, &input).await?;

        persist_client(&self.pool, &input, location.pk_location).await
    }
}

async fn ensure_document_available(
    repo: &dyn FindByDocument,
    doc: &Doc,
) -> Result<(), ClientError> {
    if repo.find_by_document(&doc.to_string()).await?.is_some() {
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

async fn ensure_email_available(_email: &Email) -> Result<(), ClientError> {
    Ok(())
}

async fn ensure_phones_available(_phones: &[Phone]) -> Result<(), ClientError> {
    Ok(())
}

async fn fetch_address(
    viacep: &dyn ViaCepPort,
    cep: &Cep,
) -> Result<ViaCepAddressModel, ClientError> {
    Ok(viacep.fetch_address(cep.as_ref()).await?)
}

async fn find_or_create_location(
    _viacep_address: &ViaCepAddressModel,
    _input: &ValidatedInput,
) -> Result<LocationRow, ClientError> {
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

    let contact_uuid = Uuid::now_v7();

    let client = sqlx::query_as!(
        ClientRow,
        r#"
        INSERT INTO clients.tb_client (pk_client, tx_name, tx_status, tx_doc)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
        Uuid::now_v7(),
        &input.name,
        &ClientStatus::Active.to_string(),
        &input.doc.to_string(),
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(sqlx_err("criar cliente"))?;

    PgClientRepository::create_contact(&mut *tx, client.pk_client, contact_uuid)
        .await
        .map_err(sqlx_err("vincular cliente-contato"))?;

    PgClientRepository::create_address(&mut *tx, client.pk_client, location_uuid)
        .await
        .map_err(sqlx_err("vincular cliente-endereco"))?;

    tx.commit()
        .await
        .map_err(|e| ClientError::Infra(InfraError::CommitTransaction { source: e }))?;

    Ok(client)
}
