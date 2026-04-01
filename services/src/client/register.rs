use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use domain::{
    errors::ClientError,
    errors::infra_error::InfraError,
    models::db::{
        client_row::{ClientRow, ClientStatus, CreateClientRow},
        contact_row::CreateContactRow,
        location_row::{CreateLocationRow, LocationRow},
        viacep::ViaCepAddressModel,
    },
    ports::viacep_port::ViaCepPort,
    types::{cep::Cep, doc::Doc, email::Email, phone::Phone},
};
use repository::{
    clients::{
        commands::{
            client_address_mutation::ClientAddressMutation,
            client_contact_mutation::ClientContactMutation, client_mutations::ClientMutation,
        },
        queries::client_query::ClientQuery,
    },
    contacts::{
        commands::{contact_mutation::ContactMutation, phone_mutation::PhoneMutation},
        queries::{contact_query::ContactQuery, phone_query::PhoneQuery},
    },
    locations::{
        commands::location_mutation::LocationMutation, queries::location_query::LocationQuery,
    },
};
use sqlx::PgPool;
use uuid::Uuid;

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

pub struct RegisterClient;

fn sqlx_to_client(action: &'static str) -> impl FnOnce(sqlx::Error) -> ClientError {
    move |e| ClientError::Infra(InfraError::Database { action, source: e })
}

impl RegisterClient {
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
        if ClientQuery::find_by_document(pool, doc.to_string())
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

    async fn ensure_email_available(pool: &PgPool, email: &Email) -> Result<(), ClientError> {
        if ContactQuery::get_by_email(pool, email.as_ref().to_string())
            .await
            .map_err(sqlx_to_client("buscar contato por email"))?
            .is_some()
        {
            return Err(ClientError::EmailAlreadyExists {
                email: email.as_ref().to_string(),
            });
        }
        Ok(())
    }

    async fn ensure_phones_available(pool: &PgPool, phones: &[Phone]) -> Result<(), ClientError> {
        let phone_strings: Vec<String> = phones.iter().map(|p| p.as_ref().to_string()).collect();
        let nonexistent = PhoneQuery::find_nonexistent_phones(pool, phone_strings.clone())
            .await
            .map_err(sqlx_to_client("verificar telefones existentes"))?;

        // nonexistent = phones that DON'T exist in DB
        // if all are nonexistent, we're good
        if nonexistent.len() == phone_strings.len() {
            return Ok(());
        }

        let existing = phone_strings.iter().find(|p| !nonexistent.contains(p));

        match existing {
            Some(phone) => Err(ClientError::PhoneAlreadyExists {
                phone: phone.clone(),
            }),
            None => Ok(()),
        }
    }

    async fn fetch_address(
        viacep: &dyn ViaCepPort,
        cep: &Cep,
    ) -> Result<ViaCepAddressModel, ClientError> {
        Ok(viacep.fetch_address(cep.as_ref()).await?)
    }

    async fn find_or_create_location(
        pool: &PgPool,
        viacep_address: &ViaCepAddressModel,
        input: &ValidatedInput,
    ) -> Result<LocationRow, ClientError> {
        let nr_hash = compute_location_hash(viacep_address);

        if let Some(existing) = LocationQuery::find_by_hash(pool, nr_hash)
            .await
            .map_err(sqlx_to_client("buscar endereço por hash"))?
        {
            return Ok(existing);
        }

        let create = CreateLocationRow {
            tx_public_space: viacep_address.logradouro.clone(),
            tx_address_complement: viacep_address.complemento.clone(),
            tx_unit: viacep_address.unidade.clone(),
            tx_neighborhood: viacep_address.bairro.clone(),
            tx_locality: viacep_address.localidade.clone(),
            tx_region: viacep_address.regiao.clone(),
            tx_ibge: Some(viacep_address.ibge.clone()),
            tx_gia: Some(viacep_address.gia.clone()),
            tx_ddd: viacep_address.ddd.clone(),
            tx_siafi: Some(viacep_address.siafi.clone()),
            tx_street: viacep_address.logradouro.clone(),
            tx_number: input.number.clone(),
            tx_city: viacep_address.localidade.clone(),
            tx_state: viacep_address.uf.clone(),
            tx_zipcode: viacep_address.cep.clone(),
            nr_hash,
        };

        LocationMutation::create(pool, create)
            .await
            .map_err(sqlx_to_client("criar localização"))
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

        let contact = ContactMutation::create(
            &mut *tx,
            CreateContactRow {
                tx_email: input.email.as_ref().to_string(),
            },
        )
        .await
        .map_err(sqlx_to_client("criar contato"))?;

        let phone_strings: Vec<String> = input
            .phones
            .iter()
            .map(|p| p.as_ref().to_string())
            .collect();
        PhoneMutation::create_many(&mut *tx, contact.pk_contact, phone_strings)
            .await
            .map_err(sqlx_to_client("criar telefones"))?;

        let client = ClientMutation::create(
            &mut *tx,
            CreateClientRow {
                tx_name: input.name.clone(),
                tx_status: ClientStatus::Active,
                tx_doc: input.doc.to_string(),
            },
        )
        .await
        .map_err(sqlx_to_client("criar cliente"))?;

        ClientContactMutation::create_contact(&mut *tx, client.pk_client, contact.pk_contact)
            .await
            .map_err(sqlx_to_client("vincular cliente-contato"))?;

        ClientAddressMutation::create(&mut *tx, client.pk_client, location_uuid)
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
