use domain::models::db::client_row::{ClientRow, ClientStatus, CreateClientRow};
use uuid::Uuid;

pub struct ClientMutation;

impl ClientMutation {
    /// Cria um cliente em `clients.tb_client`.
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

    /// Marca um cliente como ativo (`tx_status = Active`).
    pub async fn activate<'a, E>(executor: E, uuid: Uuid) -> Result<ClientRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ClientMutation::update_status(executor, uuid, ClientStatus::Active).await
    }

    /// Marca um cliente como inativo (`tx_status = Inactive`).
    pub async fn deactivate<'a, E>(executor: E, uuid: Uuid) -> Result<ClientRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ClientMutation::update_status(executor, uuid, ClientStatus::Inactive).await
    }

    /// Atualiza `tx_status` de um cliente.
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
}
