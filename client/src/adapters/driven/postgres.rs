use crate::domain::model::{ClientRow, ClientStatus, CreateClientRow};
use domain::models::db::client_address_row::ClientAddressRow;
use domain::models::db::client_contact_row::ClientContactRow;
use sqlx::PgPool;
use uuid::Uuid;

pub struct PgClientRepository {
    pool: PgPool,
}

impl PgClientRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_uuid<'a, E>(
        executor: E,
        uuid: Uuid,
    ) -> Result<Option<ClientRow>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r: Option<ClientRow> = sqlx::query_as!(
            ClientRow,
            r#"
            SELECT * FROM clients.tb_client WHERE pk_client = $1
            "#,
            &uuid,
        )
        .fetch_optional(executor)
        .await?;

        Ok(r)
    }

    pub async fn find_by_document<'a, E>(
        executor: E,
        document: String,
    ) -> Result<Option<ClientRow>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r: Option<ClientRow> = sqlx::query_as!(
            ClientRow,
            r#"
            SELECT * FROM clients.tb_client WHERE tx_doc = $1
            "#,
            &document,
        )
        .fetch_optional(executor)
        .await?;

        Ok(r)
    }

    pub async fn find_all<'a, E>(executor: E) -> Result<Vec<ClientRow>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r: Vec<ClientRow> = sqlx::query_as!(
            ClientRow,
            r#"
            SELECT * FROM clients.tb_client
            "#,
        )
        .fetch_all(executor)
        .await?;

        Ok(r)
    }

    pub async fn create<'a, E>(executor: E, c: CreateClientRow) -> Result<ClientRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let client = sqlx::query_as!(
            ClientRow,
            r#"
            INSERT INTO clients.tb_client (pk_client, tx_name, tx_status, tx_doc)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &c.tx_name,
            &c.tx_status.to_string(),
            &c.tx_doc,
        )
        .fetch_one(executor)
        .await?;

        Ok(client)
    }

    pub async fn activate<'a, E>(executor: E, uuid: Uuid) -> Result<ClientRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        Self::update_status(executor, uuid, ClientStatus::Active).await
    }

    pub async fn deactivate<'a, E>(executor: E, uuid: Uuid) -> Result<ClientRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        Self::update_status(executor, uuid, ClientStatus::Inactive).await
    }

    async fn update_status<'a, E>(
        executor: E,
        uuid: Uuid,
        status: ClientStatus,
    ) -> Result<ClientRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let client = sqlx::query_as!(
            ClientRow,
            r#"
            UPDATE clients.tb_client
            SET tx_status = $1
            WHERE pk_client = $2
            RETURNING *
            "#,
            &status.to_string(),
            &uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(client)
    }

    pub async fn create_contact<'a, E>(
        executor: E,
        client_uuid: Uuid,
        contact_uuid: Uuid,
    ) -> Result<ClientContactRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ClientContactRow,
            r#"
            INSERT INTO clients.tb_client_contact (pk_client_contact, fk_client, fk_contact)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &client_uuid,
            &contact_uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(r)
    }

    pub async fn create_address<'a, E>(
        executor: E,
        client_uuid: Uuid,
        location_uuid: Uuid,
    ) -> Result<ClientAddressRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ClientAddressRow,
            r#"
            INSERT INTO clients.tb_client_address(pk_client_address, fk_client, fk_address)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &client_uuid,
            &location_uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(r)
    }
}
