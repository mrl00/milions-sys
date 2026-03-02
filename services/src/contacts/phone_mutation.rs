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

        let phone_number_exists =
            PhoneQuery::check_by_phone_number(executor, phone.clone()).await?;

        if !contact_exists {
            Err(sqlx::Error::InvalidArgument(
                "Contact doesnt exists".to_string(),
            ))
        } else if !phone_number_exists {
            Err(sqlx::Error::InvalidArgument(
                "Phone number doesnt exists".to_string(),
            ))
        } else {
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
