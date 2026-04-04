// Integration tests for contact bounded context.
// Uses #[sqlx::test] to create isolated databases with migrations applied.

use contact::adapters::driven::postgres::pg_contact_repository::PgContactRepository;
use contact::adapters::driven::postgres::pg_phone_repository::PgPhoneRepository;
use contact::application::contact_service::ConcreteContactService;
use contact::domain::errors::contact_error::ContactError;
use contact::domain::ports::contact_repository::CreateContact;
use contact::domain::ports::contact_use_cases::{
    AddPhoneUseCase, FindContactUseCase, FindPhoneUseCase, ListContactsUseCase, ListPhonesUseCase,
    RegisterContactInput, RegisterContactUseCase, RemovePhoneUseCase, UpdateContactEmailUseCase,
    UpdatePhoneUseCase,
};
use contact::domain::ports::phone_repository::CreatePhone;
use sqlx::PgPool;

fn make_service(pool: PgPool) -> ConcreteContactService {
    ConcreteContactService::new(
        PgContactRepository::new(pool.clone()),
        PgPhoneRepository::new(pool),
    )
}

// --- Contact tests ---

#[sqlx::test(migrations = "../migrations")]
async fn create_and_find_contact(pool: PgPool) {
    let service = make_service(pool);

    let input = RegisterContactInput {
        email: "test@example.com".to_string(),
    };

    let created = RegisterContactUseCase::execute(&service, input)
        .await
        .expect("create contact");

    let found = FindContactUseCase::execute(&service, created.pk_contact)
        .await
        .expect("find contact");

    assert_eq!(found.pk_contact, created.pk_contact);
    assert_eq!(found.tx_email, Some("test@example.com".to_string()));
}

#[sqlx::test(migrations = "../migrations")]
async fn register_contact_with_duplicate_email_returns_error(pool: PgPool) {
    let service = make_service(pool);

    let _input = RegisterContactInput {
        email: "dup@example.com".to_string(),
    };

    let first = RegisterContactUseCase::execute(
        &service,
        RegisterContactInput {
            email: "dup@example.com".to_string(),
        },
    )
    .await
    .expect("create first contact");

    let result = RegisterContactUseCase::execute(
        &service,
        RegisterContactInput {
            email: "dup@example.com".to_string(),
        },
    )
    .await;

    assert!(matches!(result, Err(ContactError::AlreadyExists { .. })));
    let _ = first;
}

#[sqlx::test(migrations = "../migrations")]
async fn update_contact_email(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgContactRepository::new(pool);

    let created = repo
        .create(contact::domain::models::db::contact_row::CreateContactRow {
            tx_email: "old@example.com".to_string(),
        })
        .await
        .expect("create contact");

    let updated = UpdateContactEmailUseCase::execute(
        &service,
        created.pk_contact,
        "new@example.com".to_string(),
    )
    .await
    .expect("update email");

    assert_eq!(updated.tx_email, Some("new@example.com".to_string()));
}

#[sqlx::test(migrations = "../migrations")]
async fn list_contacts_returns_all(pool: PgPool) {
    let service = make_service(pool.clone());
    let repo = PgContactRepository::new(pool);

    repo.create(contact::domain::models::db::contact_row::CreateContactRow {
        tx_email: "a@example.com".to_string(),
    })
    .await
    .expect("create contact A");

    repo.create(contact::domain::models::db::contact_row::CreateContactRow {
        tx_email: "b@example.com".to_string(),
    })
    .await
    .expect("create contact B");

    let contacts = ListContactsUseCase::execute(&service)
        .await
        .expect("list contacts");

    assert_eq!(contacts.len(), 2);
}

#[sqlx::test(migrations = "../migrations")]
async fn list_contacts_returns_empty_when_none_exist(pool: PgPool) {
    let service = make_service(pool);

    let contacts = ListContactsUseCase::execute(&service)
        .await
        .expect("list contacts");

    assert!(contacts.is_empty());
}

// --- Phone tests ---

#[sqlx::test(migrations = "../migrations")]
async fn add_phone_to_contact(pool: PgPool) {
    let service = make_service(pool.clone());
    let contact_repo = PgContactRepository::new(pool);

    let contact = contact_repo
        .create(contact::domain::models::db::contact_row::CreateContactRow {
            tx_email: "phone@example.com".to_string(),
        })
        .await
        .expect("create contact");

    let phone =
        AddPhoneUseCase::execute(&service, contact.pk_contact, "+5511999999999".to_string())
            .await
            .expect("add phone");

    assert_eq!(phone.tx_phone, "+5511999999999");
    assert_eq!(phone.fk_contact, contact.pk_contact);
}

#[sqlx::test(migrations = "../migrations")]
async fn add_duplicate_phone_returns_error(pool: PgPool) {
    let service = make_service(pool.clone());
    let contact_repo = PgContactRepository::new(pool);

    let contact = contact_repo
        .create(contact::domain::models::db::contact_row::CreateContactRow {
            tx_email: "dupphone@example.com".to_string(),
        })
        .await
        .expect("create contact");

    AddPhoneUseCase::execute(&service, contact.pk_contact, "+5511999999999".to_string())
        .await
        .expect("add first phone");

    let result =
        AddPhoneUseCase::execute(&service, contact.pk_contact, "+5511999999999".to_string()).await;

    assert!(matches!(
        result,
        Err(ContactError::PhoneAlreadyExists { .. })
    ));
}

#[sqlx::test(migrations = "../migrations")]
async fn list_phones_for_contact(pool: PgPool) {
    let service = make_service(pool.clone());
    let contact_repo = PgContactRepository::new(pool.clone());
    let phone_repo = PgPhoneRepository::new(pool);

    let contact = contact_repo
        .create(contact::domain::models::db::contact_row::CreateContactRow {
            tx_email: "listphones@example.com".to_string(),
        })
        .await
        .expect("create contact");

    phone_repo
        .create(contact.pk_contact, "+5511999999999".to_string())
        .await
        .expect("add phone 1");

    phone_repo
        .create(contact.pk_contact, "+5511888888888".to_string())
        .await
        .expect("add phone 2");

    let phones = ListPhonesUseCase::execute(&service, contact.pk_contact)
        .await
        .expect("list phones");

    assert_eq!(phones.len(), 2);
}

#[sqlx::test(migrations = "../migrations")]
async fn list_phones_returns_empty_when_none_exist(pool: PgPool) {
    let service = make_service(pool.clone());
    let contact_repo = PgContactRepository::new(pool);

    let contact = contact_repo
        .create(contact::domain::models::db::contact_row::CreateContactRow {
            tx_email: "nophones@example.com".to_string(),
        })
        .await
        .expect("create contact");

    let phones = ListPhonesUseCase::execute(&service, contact.pk_contact)
        .await
        .expect("list phones");

    assert!(phones.is_empty());
}

#[sqlx::test(migrations = "../migrations")]
async fn update_phone(pool: PgPool) {
    let service = make_service(pool.clone());
    let contact_repo = PgContactRepository::new(pool.clone());
    let phone_repo = PgPhoneRepository::new(pool);

    let contact = contact_repo
        .create(contact::domain::models::db::contact_row::CreateContactRow {
            tx_email: "updatephone@example.com".to_string(),
        })
        .await
        .expect("create contact");

    let phone = phone_repo
        .create(contact.pk_contact, "+5511999999999".to_string())
        .await
        .expect("add phone");

    let updated =
        UpdatePhoneUseCase::execute(&service, phone.pk_phone, "+5511888888888".to_string())
            .await
            .expect("update phone");

    assert_eq!(updated.tx_phone, "+5511888888888");
}

#[sqlx::test(migrations = "../migrations")]
async fn remove_phone(pool: PgPool) {
    let service = make_service(pool.clone());
    let contact_repo = PgContactRepository::new(pool.clone());
    let phone_repo = PgPhoneRepository::new(pool);

    let contact = contact_repo
        .create(contact::domain::models::db::contact_row::CreateContactRow {
            tx_email: "removephone@example.com".to_string(),
        })
        .await
        .expect("create contact");

    let phone = phone_repo
        .create(contact.pk_contact, "+5511999999999".to_string())
        .await
        .expect("add phone");

    RemovePhoneUseCase::execute(&service, phone.pk_phone)
        .await
        .expect("remove phone");

    let result = FindPhoneUseCase::execute(&service, phone.pk_phone).await;
    assert!(result.is_err());
}

#[sqlx::test(migrations = "../migrations")]
async fn find_contact_returns_not_found_for_missing(pool: PgPool) {
    let service = make_service(pool);

    let uuid = uuid::Uuid::now_v7();
    let result: Result<_, _> = FindContactUseCase::execute(&service, uuid).await;

    assert!(result.is_err());
}

#[sqlx::test(migrations = "../migrations")]
async fn find_phone_returns_not_found_for_missing(pool: PgPool) {
    let service = make_service(pool);

    let uuid = uuid::Uuid::now_v7();
    let result: Result<_, _> = FindPhoneUseCase::execute(&service, uuid).await;

    assert!(result.is_err());
}
