use uuid::Uuid;

use crate::contacts::models::phone::PhoneModel;

pub struct PhoneMutation;

impl PhoneMutation {
    /// Cria um telefone em `contacts.tb_phone` associado a um contato.
    pub async fn create<'a, E>(
        executor: E,
        contact_uuid: Uuid,
        phone: String,
    ) -> Result<PhoneModel, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let created_phone = sqlx::query_as!(
            PhoneModel,
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

    /// Cria vários telefones em `contacts.tb_phone` para um contato.
    pub async fn create_many<'a, E>(
        executor: E,
        contact_uuid: Uuid,
        phones: Vec<String>,
    ) -> Result<Vec<PhoneModel>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let pks: Vec<Uuid> = phones.iter().map(|_| Uuid::now_v7()).collect();
        let fks: Vec<Uuid> = std::iter::repeat_n(contact_uuid, phones.len()).collect();

        let r = sqlx::query_as!(
            PhoneModel,
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

    /// Atualiza `tx_phone` de um telefone em `contacts.tb_phone`.
    pub async fn update<'a, E>(
        executor: E,
        contact_uuid: Uuid,
        phone: String,
    ) -> Result<PhoneModel, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let updated_phone = sqlx::query_as!(
            PhoneModel,
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

    /// Remove um telefone de `contacts.tb_phone` e retorna o registro removido.
    pub async fn delete<'a, E>(executor: E, uuid: Uuid) -> Result<PhoneModel, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let deleted_phone = sqlx::query_as!(
            PhoneModel,
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
