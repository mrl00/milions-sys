use sqlx::{PgPool, PgTransaction};
use uuid::Uuid;

use crate::clients::models::client::{Client, CreateClient};

pub struct ClientMutation;

impl ClientMutation {
    pub async fn create(pool: &PgPool, c: CreateClient) -> Result<Client, sqlx::Error> {
        let client = sqlx::query_as!(
            Client,
            r#"
            INSERT INTO clients.tb_client (pk_client, tx_name, tx_email)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            Uuid::new_v4(),
            &c.tx_name,
            &c.tx_email,
        )
        .fetch_one(pool)
        .await?;

        Ok(client)
    }

    pub async fn update(pool: &PgPool, uuid: Uuid, c: CreateClient) -> Result<Client, sqlx::Error> {
        let mut tx: PgTransaction = pool.begin().await?;

        let client: Option<Client> = sqlx::query_as!(
            Client,
            r#"
            SELECT * FROM clients.tb_client WHERE pk_client = $1
            "#,
            &uuid,
        )
        .fetch_optional(&mut *tx)
        .await?;

        match client {
            Some(_) => {
                let client = sqlx::query_as!(
                    Client,
                    r#"
                    UPDATE clients.tb_client
                    SET tx_name = $1, tx_email = $2
                    WHERE pk_client = $3
                    RETURNING *
                    "#,
                    &c.tx_name,
                    &c.tx_email,
                    &uuid,
                )
                .fetch_one(&mut *tx)
                .await?;

                Ok(client)
            }
            None => Err(sqlx::Error::RowNotFound),
        }
    }
}
