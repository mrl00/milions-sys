use crate::domain::errors::ContactError;
use uuid::Uuid;

use crate::domain::model::{ContactRow, CreateContactRow};
use crate::domain::ports::ContactRepository;

pub struct ContactService;

impl ContactService {
    pub async fn find_by_id(
        repo: &dyn ContactRepository,
        uuid: Uuid,
    ) -> Result<ContactRow, ContactError> {
        repo.find_by_id(uuid)
            .await?
            .ok_or(ContactError::NotFound { uuid })
    }

    pub async fn find_by_email(
        repo: &dyn ContactRepository,
        email: &str,
    ) -> Result<Option<ContactRow>, ContactError> {
        repo.find_by_email(email).await
    }

    pub async fn find_all(repo: &dyn ContactRepository) -> Result<Vec<ContactRow>, ContactError> {
        repo.find_all().await
    }

    pub async fn create(
        repo: &dyn ContactRepository,
        input: CreateContactRow,
    ) -> Result<ContactRow, ContactError> {
        if repo.find_by_email(&input.tx_email).await?.is_some() {
            return Err(ContactError::AlreadyExists {
                email: input.tx_email,
            });
        }

        repo.create(input).await
    }

    pub async fn update_email(
        repo: &dyn ContactRepository,
        uuid: Uuid,
        email: String,
    ) -> Result<ContactRow, ContactError> {
        Self::find_by_id(repo, uuid).await?;

        match repo.find_by_email(&email).await? {
            Some(existing) => {
                if existing.pk_contact != uuid {
                    return Err(ContactError::AlreadyExists { email });
                }
            }
            _ => (),
        }

        repo.update_email(uuid, email).await
    }
}
