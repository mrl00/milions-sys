use sqlx::PgPool;
use uuid::Uuid;

use crate::clients::models::client::{Client, ClientStatus, CreateClient};

pub struct ClientMutation;

impl ClientMutation {
    pub async fn create(pool: &PgPool, c: CreateClient) -> Result<Client, sqlx::Error> {
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
        .fetch_one(pool)
        .await?;

        Ok(client)
    }

    pub async fn activate(pool: &PgPool, uuid: Uuid) -> Result<Client, sqlx::Error> {
        ClientMutation::update_status(pool, uuid, ClientStatus::Active).await
    }

    pub async fn deactivate(pool: &PgPool, uuid: Uuid) -> Result<Client, sqlx::Error> {
        ClientMutation::update_status(pool, uuid, ClientStatus::Inactive).await
    }

    async fn update_status(
        pool: &PgPool,
        uuid: Uuid,
        status: ClientStatus,
    ) -> Result<Client, sqlx::Error> {
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
        .fetch_one(pool)
        .await?;

        Ok(client)
    }
}

pub struct ClientContactMutation;
