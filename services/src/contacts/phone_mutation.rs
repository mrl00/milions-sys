use uuid::Uuid;

use crate::contacts::models::phone::{CreatePhone, Phone};

pub struct PhoneMutation;

impl PhoneMutation {
    pub async fn create(pool: &sqlx::PgPool, phone: CreatePhone) -> Result<Phone, sqlx::Error> {
        let created_phone = sqlx::query_as!(
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

        Ok(created_phone)
    }

    pub async fn update(
        pool: &sqlx::PgPool,
        uuid: Uuid,
        phone: String,
    ) -> Result<Phone, sqlx::Error> {
        let updated_phone = sqlx::query_as!(
            Phone,
            r#"
            UPDATE contacts.tb_phone
            SET tx_phone = $1
            WHERE pk_phone = $2
            RETURNING *
            "#,
            &phone,
            &uuid,
        )
        .fetch_one(pool)
        .await?;

        Ok(updated_phone)
    }

    pub async fn delete(pool: &sqlx::PgPool, uuid: Uuid) -> Result<Phone, sqlx::Error> {
        let deleted_phone = sqlx::query_as!(
            Phone,
            r#"
            DELETE FROM contacts.tb_phone
            WHERE pk_phone = $1
            RETURNING *
            "#,
            &uuid,
        )
        .fetch_one(pool)
        .await?;

        Ok(deleted_phone)
    }
}
