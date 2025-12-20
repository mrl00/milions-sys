use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    contacts::{
        contact_query::{ContactQuery, PhoneQuery},
        models::{
            address::Address,
            contact::{Contact, CreateContact, UpdateContact},
            phone::{CreatePhone, Phone, UpdatePhone},
        },
    },
    locations::{
        location_mutation::LocationMutation,
        models::location::{CreateLocation, UpdateLocation},
    },
};

pub struct ContactMutation;

impl ContactMutation {
    pub async fn create(pool: &PgPool, c: CreateContact) -> Result<Contact, sqlx::Error> {
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

        let mut addresses = vec![];
        for address in c.addresses {
            let mut address = AddressMutation::create(pool, address).await?;
            address.fk_contact = contact.pk_contact;
            addresses.push(address);
        }

        let mut phones = vec![];
        for phone in c.phones {
            let location = PhoneMutation::create(pool, phone).await?;
            phones.push(location);
        }

        Ok(contact)
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

pub struct AddressMutation;
impl AddressMutation {
    pub async fn create(pool: &PgPool, c: CreateLocation) -> Result<Address, sqlx::Error> {
        let location = LocationMutation::create(pool, c).await?;
        Ok(location.into())
    }

    pub async fn update(
        pool: &PgPool,
        uuid: Uuid,
        c: UpdateLocation,
    ) -> Result<Address, sqlx::Error> {
        let location = LocationMutation::update(pool, uuid, c).await?;
        Ok(location.into())
    }

    pub async fn delete(pool: &PgPool, uuid: Uuid) -> Result<Address, sqlx::Error> {
        let location = LocationMutation::delete(pool, uuid).await?;
        Ok(location.into())
    }

    pub async fn create_batch(
        _pool: &PgPool,
        _c: Vec<CreateLocation>,
    ) -> Result<Vec<Address>, sqlx::Error> {
        todo!()
    }
}
