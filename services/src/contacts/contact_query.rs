use sqlx::PgPool;
use uuid::Uuid;

use crate::contacts::models::{
    address::Address,
    contact::{Contact, ContactNested},
    phone::Phone,
};

pub struct ContactQuery;

impl ContactQuery {
    pub async fn get_by_uuid(
        pool: &PgPool,
        uuid: Uuid,
    ) -> Result<Option<ContactNested>, sqlx::Error> {
        let r: Option<Contact> = sqlx::query_as!(
            Contact,
            r#"
            SELECT * FROM contacts.tb_contact WHERE pk_contact = $1
            "#,
            &uuid,
        )
        .fetch_optional(pool)
        .await?;

        match r {
            Some(contact) => {
                let phones: Vec<Phone> = PhoneQuery::get_by_contact_uuid(pool, uuid).await?;
                let addresses: Vec<Address> = AddressQuery::get_by_contact_uuid(pool, uuid).await?;

                let r: ContactNested = ContactNested {
                    pk_contact: contact.pk_contact,
                    idx_contact: contact.idx_contact,
                    tx_email: contact.tx_email,
                    ts_contact_created_at: contact.ts_contact_created_at,
                    ts_contact_updated_at: contact.ts_contact_updated_at,
                    phones,
                    addresses,
                };

                Ok(Some(r))
            }
            None => Ok(None),
        }
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Contact>, sqlx::Error> {
        let r: Vec<Contact> = sqlx::query_as!(
            Contact,
            r#"
            SELECT * FROM contacts.tb_contact
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(r)
    }
}

pub struct PhoneQuery;

impl PhoneQuery {
    pub async fn get_by_contact_uuid(
        pool: &PgPool,
        contact_uuid: Uuid,
    ) -> Result<Vec<Phone>, sqlx::Error> {
        let r: Vec<Phone> = sqlx::query_as!(
            Phone,
            r#"
            SELECT *
            FROM contacts.tb_phone
            WHERE fk_contact = $1
            "#,
            &contact_uuid,
        )
        .fetch_all(pool)
        .await?;

        Ok(r)
    }

    pub async fn get_by_uuid(pool: &PgPool, uuid: Uuid) -> Result<Option<Phone>, sqlx::Error> {
        let r: Option<Phone> = sqlx::query_as!(
            Phone,
            r#"
            SELECT *
            FROM contacts.tb_phone
            WHERE pk_phone = $1
            "#,
            &uuid,
        )
        .fetch_optional(pool)
        .await?;

        Ok(r)
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Phone>, sqlx::Error> {
        let r: Vec<Phone> = sqlx::query_as!(
            Phone,
            r#"
            SELECT *
            FROM contacts.tb_phone
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(r)
    }
}

pub struct AddressQuery;

impl AddressQuery {
    pub async fn get_by_contact_uuid(
        pool: &PgPool,
        contact_uuid: Uuid,
    ) -> Result<Vec<Address>, sqlx::Error> {
        let r: Vec<Address> = sqlx::query_as!(
            Address,
            r#"
            SELECT *
            FROM contacts.tb_address
            WHERE fk_contact = $1
            "#,
            &contact_uuid,
        )
        .fetch_all(pool)
        .await?;

        Ok(r)
    }

    pub async fn get_by_uuid(pool: &PgPool, uuid: Uuid) -> Result<Option<Address>, sqlx::Error> {
        let r: Option<Address> = sqlx::query_as!(
            Address,
            r#"
            SELECT *
            FROM contacts.tb_address
            WHERE pk_address = $1
            "#,
            &uuid,
        )
        .fetch_optional(pool)
        .await?;

        Ok(r)
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Address>, sqlx::Error> {
        let r: Vec<Address> = sqlx::query_as!(
            Address,
            r#"
            SELECT *
            FROM contacts.tb_address
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(r)
    }
}
