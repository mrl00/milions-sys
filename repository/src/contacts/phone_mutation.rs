use uuid::Uuid;

use crate::contacts::{contact_query::ContactQuery, models::phone::Phone, phone_query::PhoneQuery};

pub struct PhoneMutation;

impl PhoneMutation {
    pub async fn create<'a, E>(
        executor: E,
        contact_uuid: Uuid,
        phone: String,
    ) -> Result<Phone, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
    {
        let contact_exists = ContactQuery::get_by_uuid(executor, contact_uuid)
            .await?
            .is_some();

        if !contact_exists {
            return Err(sqlx::Error::InvalidArgument(
                "Contact doesnt exists".to_string(),
            ));
        }

        let phone_number_exists =
            PhoneQuery::find_nonexistent_phones(executor, Vec::from([phone.clone()])).await?;

        if !phone_number_exists.is_empty() {
            return Err(sqlx::Error::InvalidArgument(
                "Phone number already exists".to_string(),
            ));
        }

        let created_phone = sqlx::query_as!(
            Phone,
            r#"
                    INSERT INTO contacts.tb_phone (pk_phone, tx_phone, fk_contact)
                    VALUES ($1, $2, $3)
                    RETURNING *
                    "#,
            Uuid::now_v7(),
            &phone,
            &contact_uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(created_phone)
    }

    pub async fn create_many<'a, E>(
        executor: E,
        contact_uuid: Uuid,
        phones: Vec<String>,
    ) -> Result<Vec<Phone>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
    {
        let contact = ContactQuery::get_by_uuid(executor, contact_uuid).await?;
        if contact.is_none() {
            return Err(sqlx::Error::RowNotFound);
        }

        let pks: Vec<Uuid> = phones.iter().map(|_| Uuid::now_v7()).collect();
        let fks: Vec<Uuid> = std::iter::repeat_n(contact_uuid, phones.len()).collect();

        let r = sqlx::query_as!(
            Phone,
            r#"
            INSERT INTO contacts.tb_phone (pk_phone, fk_contact, tx_phone)
            SELECT * FROM UNNEST(
            $1::uuid[],
            $2::uuid[],
            $3::text[])
            RETURNING *
            "#,
            &pks as &[Uuid],
            &fks as &[Uuid],
            &phones as &[String],
        )
        .fetch_all(executor)
        .await?;

        Ok(r)
    }

    pub async fn update<'a, E>(
        executor: E,
        contact_uuid: Uuid,
        phone: String,
    ) -> Result<Phone, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
    {
        let contact = ContactQuery::get_by_uuid(executor, contact_uuid).await?;

        match contact {
            Some(_) => {
                let updated_phone = sqlx::query_as!(
                    Phone,
                    r#"
                    UPDATE contacts.tb_phone
                    SET tx_phone = $1
                    WHERE pk_phone = $2
                    RETURNING *
                    "#,
                    &phone,
                    &contact_uuid,
                )
                .fetch_one(executor)
                .await?;

                Ok(updated_phone)
            }
            None => Err(sqlx::Error::RowNotFound),
        }
    }

    pub async fn delete<'a, E>(executor: E, uuid: Uuid) -> Result<Phone, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
    {
        let deleted_phone = sqlx::query_as!(
            Phone,
            r#"
            DELETE FROM contacts.tb_phone
            WHERE pk_phone = $1
            RETURNING *
            "#,
            &uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(deleted_phone)
    }
}
