use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::adapters::driven::postgres::pg_contact_repository::PgContactRepository;
use crate::adapters::driven::postgres::pg_phone_repository::PgPhoneRepository;
use crate::domain::errors::contact_error::ContactError;
use crate::domain::models::db::contact_row::{ContactRow, CreateContactRow};
use crate::domain::models::db::phone_row::PhoneRow;
use crate::domain::ports::contact_repository::{
    CreateContact as _, FindAllContacts as _, FindContactByEmail as _, FindContactById as _,
    UpdateContactEmail as _,
};
use crate::domain::ports::contact_use_cases::{
    AddPhone, AddPhones, FindContact, FindPhone, ListContacts, ListPhones, RegisterContact,
    RegisterContactInput, RemovePhone, UpdateContactEmail as UpdateContactEmailTrait,
    UpdatePhone as UpdatePhoneTrait,
};
use crate::domain::ports::phone_repository::{
    CreateManyPhones as _, CreatePhone as _, DeletePhone as _, FindNonexistentPhones as _,
    FindPhoneByContactId as _, FindPhoneById as _, UpdatePhone as _,
};
use types::phone::Phone;

pub struct ContactService {
    contact_repo: PgContactRepository,
    phone_repo: PgPhoneRepository,
}

impl ContactService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            contact_repo: PgContactRepository::new(pool.clone()),
            phone_repo: PgPhoneRepository::new(pool),
        }
    }
}

// --- Contact ---

#[async_trait]
impl RegisterContact for ContactService {
    async fn execute(&self, input: RegisterContactInput) -> Result<ContactRow, ContactError> {
        if self
            .contact_repo
            .find_by_email(&input.email)
            .await?
            .is_some()
        {
            return Err(ContactError::AlreadyExists { email: input.email });
        }

        self.contact_repo
            .create(CreateContactRow {
                tx_email: input.email,
            })
            .await
    }
}

#[async_trait]
impl FindContact for ContactService {
    async fn execute(&self, uuid: Uuid) -> Result<ContactRow, ContactError> {
        self.contact_repo
            .find_by_id(uuid)
            .await?
            .ok_or(ContactError::NotFound { uuid })
    }
}

#[async_trait]
impl ListContacts for ContactService {
    async fn execute(&self) -> Result<Vec<ContactRow>, ContactError> {
        self.contact_repo.find_all().await
    }
}

#[async_trait]
impl UpdateContactEmailTrait for ContactService {
    async fn execute(&self, uuid: Uuid, email: String) -> Result<ContactRow, ContactError> {
        self.contact_repo
            .find_by_id(uuid)
            .await?
            .ok_or(ContactError::NotFound { uuid })?;

        if let Some(existing) = self.contact_repo.find_by_email(&email).await? {
            if existing.pk_contact != uuid {
                return Err(ContactError::AlreadyExists { email });
            }
        }

        self.contact_repo.update_email(uuid, email).await
    }
}

// --- Phone ---

fn validate_phone(phone: &str) -> Result<Phone, ContactError> {
    Phone::try_from(phone.to_string()).map_err(ContactError::InvalidPhone)
}

#[async_trait]
impl FindPhone for ContactService {
    async fn execute(&self, uuid: Uuid) -> Result<PhoneRow, ContactError> {
        self.phone_repo
            .find_by_id(uuid)
            .await?
            .ok_or(ContactError::PhoneNotFound { uuid })
    }
}

#[async_trait]
impl ListPhones for ContactService {
    async fn execute(&self, contact_id: Uuid) -> Result<Vec<PhoneRow>, ContactError> {
        self.phone_repo.find_by_contact_id(contact_id).await
    }
}

#[async_trait]
impl AddPhone for ContactService {
    async fn execute(&self, contact_id: Uuid, phone: String) -> Result<PhoneRow, ContactError> {
        validate_phone(&phone)?;

        let existing = self.phone_repo.find_by_contact_id(contact_id).await?;

        if existing.iter().any(|p| p.tx_phone == phone) {
            return Err(ContactError::PhoneAlreadyExists { phone });
        }

        self.phone_repo.create(contact_id, phone).await
    }
}

#[async_trait]
impl AddPhones for ContactService {
    async fn execute(
        &self,
        contact_id: Uuid,
        phones: Vec<String>,
    ) -> Result<Vec<PhoneRow>, ContactError> {
        for phone in &phones {
            validate_phone(phone)?;
        }

        let nonexistent = self
            .phone_repo
            .find_nonexistent_phones(phones.clone())
            .await?;

        if nonexistent.len() != phones.len() {
            if let Some(phone) = phones.iter().find(|p| !nonexistent.contains(p)) {
                return Err(ContactError::PhoneAlreadyExists {
                    phone: phone.clone(),
                });
            }
        }

        self.phone_repo.create_many(contact_id, phones).await
    }
}

#[async_trait]
impl UpdatePhoneTrait for ContactService {
    async fn execute(&self, uuid: Uuid, phone: String) -> Result<PhoneRow, ContactError> {
        validate_phone(&phone)?;

        self.phone_repo
            .find_by_id(uuid)
            .await?
            .ok_or(ContactError::PhoneNotFound { uuid })?;

        self.phone_repo.update(uuid, phone).await
    }
}

#[async_trait]
impl RemovePhone for ContactService {
    async fn execute(&self, uuid: Uuid) -> Result<PhoneRow, ContactError> {
        self.phone_repo
            .find_by_id(uuid)
            .await?
            .ok_or(ContactError::PhoneNotFound { uuid })?;

        self.phone_repo.delete(uuid).await
    }
}
