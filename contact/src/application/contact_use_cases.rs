use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::adapters::driven::postgres::pg_contact_repository::PgContactRepository;
use crate::domain::errors::contact_error::ContactError;
use crate::domain::models::db::contact_row::{ContactRow, CreateContactRow};
use crate::domain::ports::contact_repository::*;
use crate::domain::use_cases::find_contact::FindContact;
use crate::domain::use_cases::list_contacts::ListContacts;
use crate::domain::use_cases::register_contact::{RegisterContact, RegisterContactInput};
use crate::domain::use_cases::update_contact_email::UpdateContactEmail as UpdateContactEmailTrait;

pub struct ContactUseCases {
    repo: PgContactRepository,
}

impl ContactUseCases {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: PgContactRepository::new(pool),
        }
    }
}

#[async_trait]
impl RegisterContact for ContactUseCases {
    async fn execute(&self, input: RegisterContactInput) -> Result<ContactRow, ContactError> {
        if self.repo.find_by_email(&input.email).await?.is_some() {
            return Err(ContactError::AlreadyExists {
                email: input.email,
            });
        }

        self.repo
            .create(CreateContactRow {
                tx_email: input.email,
            })
            .await
    }
}

#[async_trait]
impl FindContact for ContactUseCases {
    async fn execute(&self, uuid: Uuid) -> Result<ContactRow, ContactError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ContactError::NotFound { uuid })
    }
}

#[async_trait]
impl ListContacts for ContactUseCases {
    async fn execute(&self) -> Result<Vec<ContactRow>, ContactError> {
        self.repo.find_all().await
    }
}

#[async_trait]
impl UpdateContactEmailTrait for ContactUseCases {
    async fn execute(&self, uuid: Uuid, email: String) -> Result<ContactRow, ContactError> {
        self.repo
            .find_by_id(uuid)
            .await?
            .ok_or(ContactError::NotFound { uuid })?;

        if let Some(existing) = self.repo.find_by_email(&email).await? {
            if existing.pk_contact != uuid {
                return Err(ContactError::AlreadyExists { email });
            }
        }

        self.repo.update_email(uuid, email).await
    }
}
