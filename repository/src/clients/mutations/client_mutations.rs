use sqlx::PgPool;
use uuid::Uuid;

use domain::{
    clients::ports::ClientMutationRepository,
    errors::ClientError,
    models::client::{ClientModel, ClientStatus, CreateClientModel, UpdateClientModel},
};

/// Client mutation.
pub struct ClientMutation {
    pool: PgPool,
}

impl ClientMutationRepository for ClientMutation {
    async fn create(&self, c: CreateClientModel) -> Result<ClientModel, ClientError> {
        let client = sqlx::query_as!(
            ClientModel,
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
        .fetch_one(&self.pool)
        .await?;

        Ok(client)
    }

    async fn update(
        &self,
        uuid: uuid::Uuid,
        u: UpdateClientModel,
    ) -> Result<ClientModel, ClientError> {
        let client = sqlx::query_as!(
            ClientModel,
            r#"
            UPDATE clients.tb_client
            SET tx_name = $1, tx_status = $2
            WHERE pk_client = $3
            RETURNING *
            "#,
            u.tx_name,
            u.tx_status
                .map_or_else(|| "inactive".to_string(), |s| s.to_string()),
            uuid,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(client)
    }

    async fn activate(&self, uuid: uuid::Uuid) -> Result<ClientModel, ClientError> {
        ClientMutation::update_status(&self.pool, uuid, ClientStatus::Active).await
    }

    async fn deactivate(&self, uuid: uuid::Uuid) -> Result<ClientModel, ClientError> {
        ClientMutation::update_status(&self.pool, uuid, ClientStatus::Inactive).await
    }
}

impl ClientMutation {
    /// Atualiza `tx_status` de um cliente.
    async fn update_status(
        pool: &PgPool,
        uuid: Uuid,
        status: ClientStatus,
    ) -> Result<ClientModel, ClientError> {
        let client = sqlx::query_as!(
            ClientModel,
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
