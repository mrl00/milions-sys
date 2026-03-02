use uuid::Uuid;

use crate::contacts::models::{
    contact::{Contact, CreateContact},
    phone::{CreatePhone, Phone},
};

pub struct ContactMutation;

impl ContactMutation {
    pub async fn create(
        pool: &sqlx::PgPool,
        contact: CreateContact,
    ) -> Result<Contact, sqlx::Error> {
        let created_contact = sqlx::query_as!(
            Contact,
            r#"
            INSERT INTO contacts.tb_contact (pk_contact, tx_email)
            VALUES ($1, $2)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &contact.tx_email,
        )
        .fetch_one(pool)
        .await?;

        Ok(created_contact)
    }

    pub async fn update_email(
        pool: &sqlx::PgPool,
        uuid: Uuid,
        email: String,
    ) -> Result<Contact, sqlx::Error> {
        let updated_contact = sqlx::query_as!(
            Contact,
            r#"
            UPDATE contacts.tb_contact
            SET tx_email = $1
            WHERE pk_contact = $2
            RETURNING *
            "#,
            &email,
            &uuid,
        )
        .fetch_one(pool)
        .await?;

        Ok(updated_contact)
    }

    pub async fn add_phone(pool: &sqlx::PgPool, phone: CreatePhone) -> Result<Phone, sqlx::Error> {
        let added_phone = sqlx::query_as!(
            Phone,
            r#"
            INSERT INTO contacts.tb_phone (pk_phone, tx_phone, fk_contact)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &phone.tx_phone,
            &phone.fk_contact,
        )
        .fetch_one(pool)
        .await?;

        Ok(added_phone)
    }
}
