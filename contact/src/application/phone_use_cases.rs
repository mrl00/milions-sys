use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::adapters::driven::postgres::pg_phone_repository::PgPhoneRepository;
use crate::domain::errors::contact_error::ContactError;
use crate::domain::models::db::phone_row::PhoneRow;
use crate::domain::ports::phone_repository::*;
use crate::domain::use_cases::add_phone::AddPhone;
use crate::domain::use_cases::add_phones::AddPhones;
use crate::domain::use_cases::find_phone::FindPhone;
use crate::domain::use_cases::list_phones::ListPhones;
use crate::domain::use_cases::remove_phone::RemovePhone;
use crate::domain::use_cases::update_phone::UpdatePhone as UpdatePhoneTrait;
use types::phone::Phone;

pub struct PhoneUseCases {
    repo: PgPhoneRepository,
}

impl PhoneUseCases {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: PgPhoneRepository::new(pool),
        }
    }
}

fn validate_phone(phone: &str) -> Result<Phone, ContactError> {
    Phone::try_from(phone.to_string()).map_err(ContactError::InvalidPhone)
}

#[async_trait]
impl FindPhone for PhoneUseCases {
    async fn execute(&self, uuid: Uuid) -> Result<PhoneRow, ContactError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ContactError::PhoneNotFound { uuid })
    }
}

#[async_trait]
impl ListPhones for PhoneUseCases {
    async fn execute(&self, contact_id: Uuid) -> Result<Vec<PhoneRow>, ContactError> {
        self.repo.find_by_contact_id(contact_id).await
    }
}

#[async_trait]
impl AddPhone for PhoneUseCases {
    async fn execute(&self, contact_id: Uuid, phone: String) -> Result<PhoneRow, ContactError> {
        validate_phone(&phone)?;

        let existing = self.repo.find_by_contact_id(contact_id).await?;

        if existing.iter().any(|p| p.tx_phone == phone) {
            return Err(ContactError::PhoneAlreadyExists { phone });
        }

        self.repo.create(contact_id, phone).await
    }
}

#[async_trait]
impl AddPhones for PhoneUseCases {
    async fn execute(
        &self,
        contact_id: Uuid,
        phones: Vec<String>,
    ) -> Result<Vec<PhoneRow>, ContactError> {
        for phone in &phones {
            validate_phone(phone)?;
        }

        let nonexistent = self.repo.find_nonexistent_phones(phones.clone()).await?;

        if nonexistent.len() != phones.len() {
            if let Some(phone) = phones.iter().find(|p| !nonexistent.contains(p)) {
                return Err(ContactError::PhoneAlreadyExists {
                    phone: phone.clone(),
                });
            }
        }

        self.repo.create_many(contact_id, phones).await
    }
}

#[async_trait]
impl UpdatePhoneTrait for PhoneUseCases {
    async fn execute(&self, uuid: Uuid, phone: String) -> Result<PhoneRow, ContactError> {
        validate_phone(&phone)?;

        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ContactError::PhoneNotFound { uuid })?;

        self.repo.update(uuid, phone).await
    }
}

#[async_trait]
impl RemovePhone for PhoneUseCases {
    async fn execute(&self, uuid: Uuid) -> Result<PhoneRow, ContactError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ContactError::PhoneNotFound { uuid })?;

        self.repo.delete(uuid).await
    }
}
