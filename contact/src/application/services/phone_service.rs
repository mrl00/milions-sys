use crate::domain::models::db::phone_row::PhoneRow;
use crate::domain::errors::contact_error::ContactError;
use crate::domain::ports::PhoneRepository;
use types::phone::Phone;
use uuid::Uuid;

pub struct PhoneService;

impl PhoneService {
    pub async fn find_by_id(
        repo: &dyn PhoneRepository,
        uuid: Uuid,
    ) -> Result<PhoneRow, ContactError> {
        repo.find_by_id(uuid)
            .await?
            .ok_or(ContactError::PhoneNotFound { uuid })
    }

    pub async fn find_by_contact_id(
        repo: &dyn PhoneRepository,
        contact_id: Uuid,
    ) -> Result<Vec<PhoneRow>, ContactError> {
        repo.find_by_contact_id(contact_id).await
    }

    pub async fn create(
        repo: &dyn PhoneRepository,
        contact_id: Uuid,
        phone: String,
    ) -> Result<PhoneRow, ContactError> {
        Self::validate_phone(&phone)?;

        let existing = repo.find_by_contact_id(contact_id).await?;

        if existing.iter().any(|p| p.tx_phone == phone) {
            return Err(ContactError::PhoneAlreadyExists { phone });
        }

        repo.create(contact_id, phone).await
    }

    pub async fn create_many(
        repo: &dyn PhoneRepository,
        contact_id: Uuid,
        phones: Vec<String>,
    ) -> Result<Vec<PhoneRow>, ContactError> {
        for phone in &phones {
            Self::validate_phone(phone)?;
        }

        let nonexistent = repo.find_nonexistent_phones(phones.clone()).await?;

        if nonexistent.len() != phones.len() {
            let existing = phones.iter().find(|p| !nonexistent.contains(p));
            if let Some(phone) = existing {
                return Err(ContactError::PhoneAlreadyExists {
                    phone: phone.clone(),
                });
            }
        }

        repo.create_many(contact_id, phones).await
    }

    pub async fn update(
        repo: &dyn PhoneRepository,
        uuid: Uuid,
        phone: String,
    ) -> Result<PhoneRow, ContactError> {
        Self::validate_phone(&phone)?;
        Self::find_by_id(repo, uuid).await?;
        repo.update(uuid, phone).await
    }

    pub async fn delete(repo: &dyn PhoneRepository, uuid: Uuid) -> Result<PhoneRow, ContactError> {
        Self::find_by_id(repo, uuid).await?;
        repo.delete(uuid).await
    }

    fn validate_phone(phone: &str) -> Result<Phone, ContactError> {
        let p = Phone::try_from(phone.to_string())?;
        Ok(p)
    }
}
