use uuid::Uuid;

use crate::clients::models::{
    client::{Client, ClientStatus, CreateClient},
    client_address::ClientAddress,
    client_contact::ClientContact,
};

pub struct ClientMutation;

impl ClientMutation {
    /// Cria um cliente em `clients.tb_client`.
    pub async fn create<'a, E>(executor: E, c: CreateClient) -> Result<Client, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let client = sqlx::query_as!(
            Client,
            r#"
            INSERT INTO clients.tb_client (pk_client, tx_name, tx_status)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &c.tx_name,
            &c.tx_status.to_string(),
        )
        .fetch_one(executor)
        .await?;

        Ok(client)
    }

    /// Marca um cliente como ativo (`tx_status = Active`).
    pub async fn activate<'a, E>(executor: E, uuid: Uuid) -> Result<Client, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ClientMutation::update_status(executor, uuid, ClientStatus::Active).await
    }

    /// Marca um cliente como inativo (`tx_status = Inactive`).
    pub async fn deactivate<'a, E>(executor: E, uuid: Uuid) -> Result<Client, sqlx::Error>
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
    ) -> Result<Client, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let client = sqlx::query_as!(
            Client,
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

pub struct ClientContactMutation;

impl ClientContactMutation {
    /// Cria vínculo cliente-contato em `clients.tb_client_contact`.
    pub async fn create_contact<'a, E>(
        executor: E,
        client_uuid: Uuid,
        contact_uuid: Uuid,
    ) -> Result<ClientContact, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ClientContact,
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
}

pub struct ClientAddressMutation;

impl ClientAddressMutation {
    /// Cria vínculo cliente-endereço em `clients.tb_client_address`.
    pub async fn create<'a, E>(
        executor: E,
        client_uuid: Uuid,
        location_uuid: Uuid,
    ) -> Result<ClientAddress, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ClientAddress,
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
