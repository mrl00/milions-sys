use async_trait::async_trait;
use uuid::Uuid;

use crate::adapters::driven::postgres::pg_contact_repository::PgContactRepository;
use crate::adapters::driven::postgres::pg_phone_repository::PgPhoneRepository;
use crate::domain::errors::contact_error::ContactError;
use crate::domain::models::db::contact_row::{ContactRow, CreateContactRow};
use crate::domain::models::db::phone_row::PhoneRow;
use crate::domain::ports::contact_repository::{
    CreateContact, FindAllContacts, FindContactByEmail, FindContactById, UpdateContactEmail,
};
use crate::domain::ports::contact_use_cases::{
    AddPhone, AddPhones, FindContact, FindPhone, ListContacts, ListPhones, RegisterContact,
    RegisterContactInput, RemovePhone, UpdateContactEmail as UpdateContactEmailTrait,
    UpdatePhone as UpdatePhoneTrait,
};
use crate::domain::ports::phone_repository::{
    CreateManyPhones, CreatePhone, DeletePhone, FindNonexistentPhones, FindPhoneByContactId,
    FindPhoneById, UpdatePhone,
};
use types::phone::Phone;

pub type ConcreteContactService = ContactService<PgContactRepository, PgPhoneRepository>;

pub struct ContactService<C, P> {
    contact_repo: C,
    phone_repo: P,
}

impl<C, P> ContactService<C, P>
where
    C: FindContactById + FindContactByEmail + FindAllContacts + CreateContact + UpdateContactEmail,
    P: FindPhoneById
        + FindPhoneByContactId
        + CreatePhone
        + CreateManyPhones
        + UpdatePhone
        + DeletePhone
        + FindNonexistentPhones,
{
    pub fn new(contact_repo: C, phone_repo: P) -> Self {
        Self {
            contact_repo,
            phone_repo,
        }
    }
}

// --- Contact ---

#[async_trait]
impl<C, P> RegisterContact for ContactService<C, P>
where
    C: FindContactById
        + FindContactByEmail
        + FindAllContacts
        + CreateContact
        + UpdateContactEmail
        + Send
        + Sync,
    P: FindPhoneById
        + FindPhoneByContactId
        + CreatePhone
        + CreateManyPhones
        + UpdatePhone
        + DeletePhone
        + FindNonexistentPhones
        + Send
        + Sync,
{
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
impl<C, P> FindContact for ContactService<C, P>
where
    C: FindContactById
        + FindContactByEmail
        + FindAllContacts
        + CreateContact
        + UpdateContactEmail
        + Send
        + Sync,
    P: FindPhoneById
        + FindPhoneByContactId
        + CreatePhone
        + CreateManyPhones
        + UpdatePhone
        + DeletePhone
        + FindNonexistentPhones
        + Send
        + Sync,
{
    async fn execute(&self, uuid: Uuid) -> Result<ContactRow, ContactError> {
        self.contact_repo
            .find_by_id(uuid)
            .await?
            .ok_or(ContactError::NotFound { uuid })
    }
}

#[async_trait]
impl<C, P> ListContacts for ContactService<C, P>
where
    C: FindContactById
        + FindContactByEmail
        + FindAllContacts
        + CreateContact
        + UpdateContactEmail
        + Send
        + Sync,
    P: FindPhoneById
        + FindPhoneByContactId
        + CreatePhone
        + CreateManyPhones
        + UpdatePhone
        + DeletePhone
        + FindNonexistentPhones
        + Send
        + Sync,
{
    async fn execute(&self) -> Result<Vec<ContactRow>, ContactError> {
        self.contact_repo.find_all().await
    }
}

#[async_trait]
impl<C, P> UpdateContactEmailTrait for ContactService<C, P>
where
    C: FindContactById
        + FindContactByEmail
        + FindAllContacts
        + CreateContact
        + UpdateContactEmail
        + Send
        + Sync,
    P: FindPhoneById
        + FindPhoneByContactId
        + CreatePhone
        + CreateManyPhones
        + UpdatePhone
        + DeletePhone
        + FindNonexistentPhones
        + Send
        + Sync,
{
    async fn execute(&self, uuid: Uuid, email: String) -> Result<ContactRow, ContactError> {
        self.contact_repo
            .find_by_id(uuid)
            .await?
            .ok_or(ContactError::NotFound { uuid })?;

        if let Some(existing) = self.contact_repo.find_by_email(&email).await?
            && existing.pk_contact != uuid
        {
            return Err(ContactError::AlreadyExists { email });
        }

        self.contact_repo.update_email(uuid, email).await
    }
}

// --- Phone ---

fn validate_phone(phone: &str) -> Result<Phone, ContactError> {
    Phone::try_from(phone.to_string()).map_err(ContactError::InvalidPhone)
}

#[async_trait]
impl<C, P> FindPhone for ContactService<C, P>
where
    C: FindContactById
        + FindContactByEmail
        + FindAllContacts
        + CreateContact
        + UpdateContactEmail
        + Send
        + Sync,
    P: FindPhoneById
        + FindPhoneByContactId
        + CreatePhone
        + CreateManyPhones
        + UpdatePhone
        + DeletePhone
        + FindNonexistentPhones
        + Send
        + Sync,
{
    async fn execute(&self, uuid: Uuid) -> Result<PhoneRow, ContactError> {
        self.phone_repo
            .find_by_id(uuid)
            .await?
            .ok_or(ContactError::PhoneNotFound { uuid })
    }
}

#[async_trait]
impl<C, P> ListPhones for ContactService<C, P>
where
    C: FindContactById
        + FindContactByEmail
        + FindAllContacts
        + CreateContact
        + UpdateContactEmail
        + Send
        + Sync,
    P: FindPhoneById
        + FindPhoneByContactId
        + CreatePhone
        + CreateManyPhones
        + UpdatePhone
        + DeletePhone
        + FindNonexistentPhones
        + Send
        + Sync,
{
    async fn execute(&self, contact_id: Uuid) -> Result<Vec<PhoneRow>, ContactError> {
        self.phone_repo.find_by_contact_id(contact_id).await
    }
}

#[async_trait]
impl<C, P> AddPhone for ContactService<C, P>
where
    C: FindContactById
        + FindContactByEmail
        + FindAllContacts
        + CreateContact
        + UpdateContactEmail
        + Send
        + Sync,
    P: FindPhoneById
        + FindPhoneByContactId
        + CreatePhone
        + CreateManyPhones
        + UpdatePhone
        + DeletePhone
        + FindNonexistentPhones
        + Send
        + Sync,
{
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
impl<C, P> AddPhones for ContactService<C, P>
where
    C: FindContactById
        + FindContactByEmail
        + FindAllContacts
        + CreateContact
        + UpdateContactEmail
        + Send
        + Sync,
    P: FindPhoneById
        + FindPhoneByContactId
        + CreatePhone
        + CreateManyPhones
        + UpdatePhone
        + DeletePhone
        + FindNonexistentPhones
        + Send
        + Sync,
{
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

        if nonexistent.len() != phones.len()
            && let Some(phone) = phones.iter().find(|p| !nonexistent.contains(p))
        {
            return Err(ContactError::PhoneAlreadyExists {
                phone: phone.clone(),
            });
        }

        self.phone_repo.create_many(contact_id, phones).await
    }
}

#[async_trait]
impl<C, P> UpdatePhoneTrait for ContactService<C, P>
where
    C: FindContactById
        + FindContactByEmail
        + FindAllContacts
        + CreateContact
        + UpdateContactEmail
        + Send
        + Sync,
    P: FindPhoneById
        + FindPhoneByContactId
        + CreatePhone
        + CreateManyPhones
        + UpdatePhone
        + DeletePhone
        + FindNonexistentPhones
        + Send
        + Sync,
{
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
impl<C, P> RemovePhone for ContactService<C, P>
where
    C: FindContactById
        + FindContactByEmail
        + FindAllContacts
        + CreateContact
        + UpdateContactEmail
        + Send
        + Sync,
    P: FindPhoneById
        + FindPhoneByContactId
        + CreatePhone
        + CreateManyPhones
        + UpdatePhone
        + DeletePhone
        + FindNonexistentPhones
        + Send
        + Sync,
{
    async fn execute(&self, uuid: Uuid) -> Result<PhoneRow, ContactError> {
        self.phone_repo
            .find_by_id(uuid)
            .await?
            .ok_or(ContactError::PhoneNotFound { uuid })?;

        self.phone_repo.delete(uuid).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::errors::contact_error::ContactError;
    use crate::domain::models::db::contact_row::{ContactRow, CreateContactRow};
    use crate::domain::models::db::phone_row::PhoneRow;
    use crate::domain::ports::contact_repository::{
        CreateContact, FindAllContacts, FindContactByEmail, FindContactById, UpdateContactEmail,
    };
    use crate::domain::ports::contact_use_cases::{
        AddPhone, AddPhones, FindContact, FindPhone, ListContacts, ListPhones, RegisterContact,
        RegisterContactInput, RemovePhone, UpdateContactEmail as UpdateContactEmailTrait,
        UpdatePhone as UpdatePhoneTrait,
    };
    use crate::domain::ports::phone_repository::{
        CreateManyPhones, CreatePhone, DeletePhone, FindNonexistentPhones, FindPhoneByContactId,
        FindPhoneById, UpdatePhone,
    };

    #[derive(Default)]
    struct MockContactRepo {
        find_by_id_result: Option<ContactRow>,
        find_by_email_result: Option<ContactRow>,
        find_all_result: Vec<ContactRow>,
    }

    impl MockContactRepo {
        fn new() -> Self {
            Self::default()
        }
    }

    #[derive(Default)]
    struct MockPhoneRepo {
        find_by_id_result: Option<PhoneRow>,
        find_by_contact_id_result: Vec<PhoneRow>,
        find_nonexistent_result: Vec<String>,
    }

    impl MockPhoneRepo {
        fn new() -> Self {
            Self::default()
        }
    }

    fn make_contact() -> ContactRow {
        ContactRow {
            pk_contact: Uuid::now_v7(),
            idx_contact: 1,
            tx_email: Some("test@example.com".to_string()),
            ts_contact_created_at: chrono::NaiveDateTime::default(),
            ts_contact_updated_at: chrono::NaiveDateTime::default(),
        }
    }

    fn make_phone() -> PhoneRow {
        PhoneRow {
            pk_phone: Uuid::now_v7(),
            idx_phone: 1,
            fk_contact: Uuid::now_v7(),
            ts_phone_created_at: chrono::NaiveDateTime::default(),
            ts_phone_updated_at: chrono::NaiveDateTime::default(),
            tx_phone: "+5511999999999".to_string(),
        }
    }

    #[async_trait]
    impl FindContactById for MockContactRepo {
        async fn find_by_id(&self, _uuid: Uuid) -> Result<Option<ContactRow>, ContactError> {
            Ok(self.find_by_id_result.clone())
        }
    }

    #[async_trait]
    impl FindContactByEmail for MockContactRepo {
        async fn find_by_email(&self, _email: &str) -> Result<Option<ContactRow>, ContactError> {
            Ok(self.find_by_email_result.clone())
        }
    }

    #[async_trait]
    impl FindAllContacts for MockContactRepo {
        async fn find_all(&self) -> Result<Vec<ContactRow>, ContactError> {
            Ok(self.find_all_result.clone())
        }
    }

    #[async_trait]
    impl CreateContact for MockContactRepo {
        async fn create(&self, input: CreateContactRow) -> Result<ContactRow, ContactError> {
            Ok(ContactRow {
                pk_contact: Uuid::now_v7(),
                idx_contact: 0,
                tx_email: Some(input.tx_email),
                ts_contact_created_at: chrono::NaiveDateTime::default(),
                ts_contact_updated_at: chrono::NaiveDateTime::default(),
            })
        }
    }

    #[async_trait]
    impl UpdateContactEmail for MockContactRepo {
        async fn update_email(
            &self,
            uuid: Uuid,
            email: String,
        ) -> Result<ContactRow, ContactError> {
            Ok(ContactRow {
                pk_contact: uuid,
                idx_contact: 0,
                tx_email: Some(email),
                ts_contact_created_at: chrono::NaiveDateTime::default(),
                ts_contact_updated_at: chrono::NaiveDateTime::default(),
            })
        }
    }

    #[async_trait]
    impl FindPhoneById for MockPhoneRepo {
        async fn find_by_id(&self, _uuid: Uuid) -> Result<Option<PhoneRow>, ContactError> {
            Ok(self.find_by_id_result.clone())
        }
    }

    #[async_trait]
    impl FindPhoneByContactId for MockPhoneRepo {
        async fn find_by_contact_id(
            &self,
            _contact_id: Uuid,
        ) -> Result<Vec<PhoneRow>, ContactError> {
            Ok(self.find_by_contact_id_result.clone())
        }
    }

    #[async_trait]
    impl CreatePhone for MockPhoneRepo {
        async fn create(&self, contact_id: Uuid, phone: String) -> Result<PhoneRow, ContactError> {
            Ok(PhoneRow {
                pk_phone: Uuid::now_v7(),
                idx_phone: 0,
                fk_contact: contact_id,
                ts_phone_created_at: chrono::NaiveDateTime::default(),
                ts_phone_updated_at: chrono::NaiveDateTime::default(),
                tx_phone: phone,
            })
        }
    }

    #[async_trait]
    impl CreateManyPhones for MockPhoneRepo {
        async fn create_many(
            &self,
            contact_id: Uuid,
            phones: Vec<String>,
        ) -> Result<Vec<PhoneRow>, ContactError> {
            Ok(phones
                .into_iter()
                .map(|p| PhoneRow {
                    pk_phone: Uuid::now_v7(),
                    idx_phone: 0,
                    fk_contact: contact_id,
                    ts_phone_created_at: chrono::NaiveDateTime::default(),
                    ts_phone_updated_at: chrono::NaiveDateTime::default(),
                    tx_phone: p,
                })
                .collect())
        }
    }

    #[async_trait]
    impl UpdatePhone for MockPhoneRepo {
        async fn update(&self, uuid: Uuid, phone: String) -> Result<PhoneRow, ContactError> {
            Ok(PhoneRow {
                pk_phone: uuid,
                idx_phone: 0,
                fk_contact: Uuid::now_v7(),
                ts_phone_created_at: chrono::NaiveDateTime::default(),
                ts_phone_updated_at: chrono::NaiveDateTime::default(),
                tx_phone: phone,
            })
        }
    }

    #[async_trait]
    impl DeletePhone for MockPhoneRepo {
        async fn delete(&self, uuid: Uuid) -> Result<PhoneRow, ContactError> {
            Ok(PhoneRow {
                pk_phone: uuid,
                idx_phone: 0,
                fk_contact: Uuid::now_v7(),
                ts_phone_created_at: chrono::NaiveDateTime::default(),
                ts_phone_updated_at: chrono::NaiveDateTime::default(),
                tx_phone: "".to_string(),
            })
        }
    }

    #[async_trait]
    impl FindNonexistentPhones for MockPhoneRepo {
        async fn find_nonexistent_phones(
            &self,
            _phones: Vec<String>,
        ) -> Result<Vec<String>, ContactError> {
            Ok(self.find_nonexistent_result.clone())
        }
    }

    #[tokio::test]
    async fn register_contact_succeeds_when_email_is_new() {
        let repo = MockContactRepo::new();
        let phone_repo = MockPhoneRepo::new();
        let service = ContactService::new(repo, phone_repo);
        let input = RegisterContactInput {
            email: "new@example.com".to_string(),
        };
        let result = RegisterContact::execute(&service, input).await.unwrap();
        assert_eq!(result.tx_email, Some("new@example.com".to_string()));
    }

    #[tokio::test]
    async fn register_contact_fails_when_email_exists() {
        let mut repo = MockContactRepo::new();
        repo.find_by_email_result = Some(make_contact());
        let phone_repo = MockPhoneRepo::new();
        let service = ContactService::new(repo, phone_repo);
        let input = RegisterContactInput {
            email: "test@example.com".to_string(),
        };
        let result = RegisterContact::execute(&service, input).await;
        assert!(matches!(result, Err(ContactError::AlreadyExists { .. })));
    }

    #[tokio::test]
    async fn find_contact_returns_row_when_exists() {
        let contact = make_contact();
        let uuid = contact.pk_contact;
        let mut repo = MockContactRepo::new();
        repo.find_by_id_result = Some(contact);
        let phone_repo = MockPhoneRepo::new();
        let service = ContactService::new(repo, phone_repo);
        let result = FindContact::execute(&service, uuid).await.unwrap();
        assert_eq!(result.pk_contact, uuid);
    }

    #[tokio::test]
    async fn find_contact_returns_not_found_when_missing() {
        let uuid = Uuid::now_v7();
        let repo = MockContactRepo::new();
        let phone_repo = MockPhoneRepo::new();
        let service = ContactService::new(repo, phone_repo);
        let result = FindContact::execute(&service, uuid).await;
        assert!(matches!(result, Err(ContactError::NotFound { .. })));
    }

    #[tokio::test]
    async fn list_contacts_returns_all() {
        let c1 = make_contact();
        let c2 = make_contact();
        let mut repo = MockContactRepo::new();
        repo.find_all_result = vec![c1, c2];
        let phone_repo = MockPhoneRepo::new();
        let service = ContactService::new(repo, phone_repo);
        let result = ListContacts::execute(&service).await.unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn update_contact_email_succeeds_when_unique() {
        let contact = make_contact();
        let uuid = contact.pk_contact;
        let mut repo = MockContactRepo::new();
        repo.find_by_id_result = Some(contact);
        let phone_repo = MockPhoneRepo::new();
        let service = ContactService::new(repo, phone_repo);
        let result =
            UpdateContactEmailTrait::execute(&service, uuid, "updated@example.com".to_string())
                .await
                .unwrap();
        assert_eq!(result.tx_email, Some("updated@example.com".to_string()));
    }

    #[tokio::test]
    async fn update_contact_email_fails_when_duplicate() {
        let contact = make_contact();
        let uuid = contact.pk_contact;
        let mut repo = MockContactRepo::new();
        repo.find_by_id_result = Some(contact.clone());
        let mut other = make_contact();
        other.tx_email = Some("other@example.com".to_string());
        repo.find_by_email_result = Some(other);
        let phone_repo = MockPhoneRepo::new();
        let service = ContactService::new(repo, phone_repo);
        let result =
            UpdateContactEmailTrait::execute(&service, uuid, "other@example.com".to_string()).await;
        assert!(matches!(result, Err(ContactError::AlreadyExists { .. })));
    }

    #[tokio::test]
    async fn find_phone_returns_row_when_exists() {
        let phone = make_phone();
        let uuid = phone.pk_phone;
        let mut phone_repo = MockPhoneRepo::new();
        phone_repo.find_by_id_result = Some(phone);
        let repo = MockContactRepo::new();
        let service = ContactService::new(repo, phone_repo);
        let result = FindPhone::execute(&service, uuid).await.unwrap();
        assert_eq!(result.pk_phone, uuid);
    }

    #[tokio::test]
    async fn find_phone_returns_not_found_when_missing() {
        let uuid = Uuid::now_v7();
        let repo = MockContactRepo::new();
        let phone_repo = MockPhoneRepo::new();
        let service = ContactService::new(repo, phone_repo);
        let result = FindPhone::execute(&service, uuid).await;
        assert!(matches!(result, Err(ContactError::PhoneNotFound { .. })));
    }

    #[tokio::test]
    async fn list_phones_returns_all() {
        let p1 = make_phone();
        let p2 = make_phone();
        let mut phone_repo = MockPhoneRepo::new();
        phone_repo.find_by_contact_id_result = vec![p1, p2];
        let repo = MockContactRepo::new();
        let service = ContactService::new(repo, phone_repo);
        let result = ListPhones::execute(&service, Uuid::now_v7()).await.unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn add_phone_succeeds_when_new() {
        let contact_id = Uuid::now_v7();
        let repo = MockContactRepo::new();
        let phone_repo = MockPhoneRepo::new();
        let service = ContactService::new(repo, phone_repo);
        let result = AddPhone::execute(&service, contact_id, "+5511999999999".to_string())
            .await
            .unwrap();
        assert_eq!(result.tx_phone, "+5511999999999");
    }

    #[tokio::test]
    async fn add_phone_fails_when_duplicate() {
        let contact_id = Uuid::now_v7();
        let mut phone_repo = MockPhoneRepo::new();
        let mut existing = make_phone();
        existing.tx_phone = "+5511999999999".to_string();
        phone_repo.find_by_contact_id_result = vec![existing];
        let repo = MockContactRepo::new();
        let service = ContactService::new(repo, phone_repo);
        let result = AddPhone::execute(&service, contact_id, "+5511999999999".to_string()).await;
        assert!(matches!(
            result,
            Err(ContactError::PhoneAlreadyExists { .. })
        ));
    }

    #[tokio::test]
    async fn add_phone_fails_when_invalid() {
        let contact_id = Uuid::now_v7();
        let repo = MockContactRepo::new();
        let phone_repo = MockPhoneRepo::new();
        let service = ContactService::new(repo, phone_repo);
        let result = AddPhone::execute(&service, contact_id, "invalid".to_string()).await;
        assert!(matches!(result, Err(ContactError::InvalidPhone(_))));
    }

    #[tokio::test]
    async fn add_phones_succeeds_when_all_new() {
        let contact_id = Uuid::now_v7();
        let mut phone_repo = MockPhoneRepo::new();
        phone_repo.find_nonexistent_result =
            vec!["+5511999999999".to_string(), "+5511888888888".to_string()];
        let repo = MockContactRepo::new();
        let service = ContactService::new(repo, phone_repo);
        let result = AddPhones::execute(
            &service,
            contact_id,
            vec!["+5511999999999".to_string(), "+5511888888888".to_string()],
        )
        .await
        .unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn update_phone_succeeds_when_exists() {
        let phone = make_phone();
        let uuid = phone.pk_phone;
        let mut phone_repo = MockPhoneRepo::new();
        phone_repo.find_by_id_result = Some(phone);
        let repo = MockContactRepo::new();
        let service = ContactService::new(repo, phone_repo);
        let result = UpdatePhoneTrait::execute(&service, uuid, "+5511888888888".to_string())
            .await
            .unwrap();
        assert_eq!(result.tx_phone, "+5511888888888");
    }

    #[tokio::test]
    async fn update_phone_fails_when_not_found() {
        let uuid = Uuid::now_v7();
        let repo = MockContactRepo::new();
        let phone_repo = MockPhoneRepo::new();
        let service = ContactService::new(repo, phone_repo);
        let result = UpdatePhoneTrait::execute(&service, uuid, "+5511999999999".to_string()).await;
        assert!(matches!(result, Err(ContactError::PhoneNotFound { .. })));
    }

    #[tokio::test]
    async fn remove_phone_succeeds_when_exists() {
        let phone = make_phone();
        let uuid = phone.pk_phone;
        let mut phone_repo = MockPhoneRepo::new();
        phone_repo.find_by_id_result = Some(phone);
        let repo = MockContactRepo::new();
        let service = ContactService::new(repo, phone_repo);
        let result = RemovePhone::execute(&service, uuid).await.unwrap();
        assert_eq!(result.pk_phone, uuid);
    }

    #[tokio::test]
    async fn remove_phone_fails_when_not_found() {
        let uuid = Uuid::now_v7();
        let repo = MockContactRepo::new();
        let phone_repo = MockPhoneRepo::new();
        let service = ContactService::new(repo, phone_repo);
        let result = RemovePhone::execute(&service, uuid).await;
        assert!(matches!(result, Err(ContactError::PhoneNotFound { .. })));
    }

    #[test]
    fn error_not_found_message_contains_uuid() {
        let uuid = Uuid::now_v7();
        let err = ContactError::NotFound { uuid };
        let msg = err.to_string();
        assert!(msg.contains(&uuid.to_string()));
        assert!(msg.contains("não encontrado"));
    }

    #[test]
    fn error_already_exists_message_contains_email() {
        let err = ContactError::AlreadyExists {
            email: "test@example.com".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("test@example.com"));
        assert!(msg.contains("já existe"));
    }

    #[test]
    fn error_phone_not_found_message_contains_uuid() {
        let uuid = Uuid::now_v7();
        let err = ContactError::PhoneNotFound { uuid };
        let msg = err.to_string();
        assert!(msg.contains(&uuid.to_string()));
        assert!(msg.contains("não encontrado"));
    }
}
