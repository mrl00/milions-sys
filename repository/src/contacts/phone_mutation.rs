use uuid::Uuid;

use crate::contacts::models::phone::Phone;

pub struct PhoneMutation;

impl PhoneMutation {
    /// Cria um novo telefone associado a um contato na tabela `contacts.tb_phone`.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `INSERT`.
    /// - **contact_uuid**: identificador UUID do contato (`fk_contact`).
    /// - **phone**: número de telefone a ser cadastrado.
    ///
    /// Gera um novo `pk_phone`, insere o registro e retorna o telefone criado.
    pub async fn create<'a, E>(
        executor: E,
        contact_uuid: Uuid,
        phone: String,
    ) -> Result<Phone, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
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

    /// Cria vários telefones de uma vez para um contato, utilizando `UNNEST` para inserção em lote.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `INSERT`.
    /// - **contact_uuid**: identificador UUID do contato (`fk_contact`).
    /// - **phones**: lista de números de telefone a serem cadastrados.
    ///
    /// Verifica se o contato existe; se não existir retorna `sqlx::Error::RowNotFound`.
    /// Caso exista, insere todos os telefones e retorna o vetor de registros criados.
    pub async fn create_many<'a, E>(
        executor: E,
        contact_uuid: Uuid,
        phones: Vec<String>,
    ) -> Result<Vec<Phone>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
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

    /// Atualiza o número de telefone de um registro existente em `contacts.tb_phone`.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `UPDATE`.
    /// - **contact_uuid**: identificador UUID associado, usado para validar a existência do contato.
    /// - **phone**: novo número de telefone.
    ///
    /// Retorna o telefone atualizado ou `sqlx::Error::RowNotFound` se o contato não existir.
    pub async fn update<'a, E>(
        executor: E,
        contact_uuid: Uuid,
        phone: String,
    ) -> Result<Phone, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
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

    /// Remove um telefone da tabela `contacts.tb_phone` e retorna o registro removido.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `DELETE`.
    /// - **uuid**: identificador UUID do telefone (`pk_phone`).
    ///
    /// Retorna o telefone deletado ou erro de banco.
    pub async fn delete<'a, E>(executor: E, uuid: Uuid) -> Result<Phone, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
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
