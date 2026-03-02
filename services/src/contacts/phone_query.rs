use uuid::Uuid;

use crate::contacts::models::phone::Phone;

pub struct PhoneQuery;

impl PhoneQuery {
    pub async fn get_by_uuid(pool: &sqlx::PgPool, uuid: Uuid) -> Result<Phone, sqlx::Error> {
        let phone = sqlx::query_as!(
            Phone,
            r#"
            SELECT *
            FROM contacts.tb_phone
            WHERE pk_phone = $1
            "#,
            &uuid,
        )
        .fetch_one(pool)
        .await?;

        Ok(phone)
    }

    pub async fn get_by_contact(
        pool: &sqlx::PgPool,
        contact: Uuid,
    ) -> Result<Vec<Phone>, sqlx::Error> {
        let phones = sqlx::query_as!(
            Phone,
            r#"
            SELECT *
            FROM contacts.tb_phone
            WHERE fk_contact = $1
            "#,
            &contact,
        )
        .fetch_all(pool)
        .await?;

        Ok(phones)
    }

    pub async fn check_phone(pool: &sqlx::PgPool, phone: String) -> Result<bool, sqlx::Error> {
        let exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM contacts.tb_phone
                WHERE tx_phone = $1
            )
            "#,
            &phone,
        )
        .fetch_one(pool)
        .await?;

        Ok(exists.unwrap_or(false))
    }
}
