use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    contacts::{
        contact_query::{ContactQuery, PhoneQuery},
        models::{
            address::Address,
            contact::{Contact, ContactNested, CreateContact, UpdateContact},
            phone::{CreatePhone, Phone, UpdatePhone},
        },
    },
    locations::{location_mutation::LocationMutation, models::location::CreateLocation},
};

pub struct ContactMutation;

impl ContactMutation {
    pub async fn create(pool: &PgPool, c: CreateContact) -> Result<ContactNested, sqlx::Error> {
        let contact = sqlx::query_as!(
            Contact,
            r#"
            INSERT INTO contacts.tb_contact (pk_contact, tx_email)
            VALUES ($1, $2)
            RETURNING *
            "#,
            Uuid::new_v4(),
            &c.tx_email,
        )
        .fetch_one(pool)
        .await?;

        let phones: Vec<Phone> =
            ContactMutation::add_phones(pool, contact.pk_contact, c.phones).await?;

        let addresses: Vec<Address> =
            ContactMutation::add_addresses(pool, contact.pk_contact, c.addresses).await?;

        let mut nested_contact = ContactNested::from(contact);
        nested_contact.phones = phones;
        nested_contact.addresses = addresses;

        Ok(nested_contact)
    }

    pub async fn update(
        pool: &PgPool,
        uuid: Uuid,
        c: UpdateContact,
    ) -> Result<Contact, sqlx::Error> {
        let contact = ContactQuery::get_by_uuid(pool, uuid).await?;

        match contact {
            Some(_) => {
                let contact = sqlx::query_as!(
                    Contact,
                    r#"
                    UPDATE contacts.tb_contact
                    SET tx_email = $1
                    WHERE pk_contact = $2
                    RETURNING *
                    "#,
                    &c.tx_email,
                    uuid,
                )
                .fetch_one(pool)
                .await?;

                Ok(contact)
            }
            None => Err(sqlx::Error::RowNotFound),
        }
    }

    async fn add_phones(
        pool: &PgPool,
        uuid: Uuid,
        phones: Vec<CreatePhone>,
    ) -> Result<Vec<Phone>, sqlx::Error> {
        let mut created_phones = vec![];
        for phone in phones {
            let p = PhoneMutation::create(
                pool,
                CreatePhone {
                    pk_phone: phone.pk_phone,
                    tx_phone: phone.tx_phone,
                    fk_contact: uuid,
                },
            )
            .await?;

            created_phones.push(p);
        }
        Ok(created_phones)
    }

    async fn add_addresses(
        pool: &PgPool,
        uuid: Uuid,
        locations: Vec<CreateLocation>,
    ) -> Result<Vec<Address>, sqlx::Error> {
        let mut created_addresses = vec![];

        for location in locations {
            let mut created_location = LocationMutation::create(pool, location).await?;

            let created_address = sqlx::query_as!(
                Address,
                r#"
                INSERT INTO contacts.tb_address (pk_address, fk_contact)
                VALUES ($1, $2)
                RETURNING *
                "#,
                created_location.pk_location,
                uuid,
            )
            .fetch_one(pool)
            .await?;

            created_addresses.push(created_address);
        }

        Ok(created_addresses)
    }

    pub async fn delete(pool: &PgPool, uuid: Uuid) -> Result<Contact, sqlx::Error> {
        let contact = ContactQuery::get_by_uuid(pool, uuid).await?;

        match contact {
            Some(_) => {
                let contact = sqlx::query_as!(
                    Contact,
                    r#"
                    DELETE FROM contacts.tb_contact
                    WHERE pk_contact = $1
                    RETURNING *
                    "#,
                    uuid,
                )
                .fetch_one(pool)
                .await?;

                Ok(contact)
            }
            None => Err(sqlx::Error::RowNotFound),
        }
    }
}

pub struct PhoneMutation;
impl PhoneMutation {
    pub async fn create(pool: &PgPool, c: CreatePhone) -> Result<Phone, sqlx::Error> {
        let phone = sqlx::query_as!(
            Phone,
            r#"
            INSERT INTO contacts.tb_phone (pk_phone, tx_phone, fk_contact)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            Uuid::new_v4(),
            &c.tx_phone,
            c.fk_contact,
        )
        .fetch_one(pool)
        .await?;

        Ok(phone)
    }

    pub async fn update(pool: &PgPool, uuid: Uuid, c: UpdatePhone) -> Result<Phone, sqlx::Error> {
        let phone = PhoneQuery::get_by_uuid(pool, uuid).await?;

        match phone {
            Some(_) => {
                let phone = sqlx::query_as!(
                    Phone,
                    r#"
                    UPDATE contacts.tb_phone
                    SET tx_phone = $1
                    WHERE pk_phone = $2
                    RETURNING *
                    "#,
                    &c.tx_phone,
                    uuid,
                )
                .fetch_one(pool)
                .await?;

                Ok(phone)
            }
            None => Err(sqlx::Error::RowNotFound),
        }
    }

    pub async fn delete(pool: &PgPool, uuid: Uuid) -> Result<Phone, sqlx::Error> {
        let phone = PhoneQuery::get_by_uuid(pool, uuid).await?;

        match phone {
            Some(_) => {
                let phone = sqlx::query_as!(
                    Phone,
                    r#"
                    DELETE FROM contacts.tb_phone
                    WHERE pk_phone = $1
                    RETURNING *
                    "#,
                    uuid,
                )
                .fetch_one(pool)
                .await?;

                Ok(phone)
            }
            None => Err(sqlx::Error::RowNotFound),
        }
    }
}
