use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::errors::client_error::ClientError;
use crate::domain::errors::infra_error::InfraError;
use crate::domain::models::db::client_address_row::ClientAddressRow;
use crate::domain::models::db::client_contact_row::ClientContactRow;
use crate::domain::models::db::client_row::{ClientRow, CreateClientRow, UpdateClientRow};
use crate::domain::ports::repositories::client_repository::*;

pub struct PgClientRepository {
    pool: PgPool,
}

impl PgClientRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_contact<'a, E>(
        executor: E,
        client_uuid: Uuid,
        contact_uuid: Uuid,
    ) -> Result<ClientContactRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query_as!(
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
        .await
    }

    pub async fn create_address<'a, E>(
        executor: E,
        client_uuid: Uuid,
        location_uuid: Uuid,
    ) -> Result<ClientAddressRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query_as!(
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
        .await
    }
}

fn sqlx_err(action: &'static str) -> impl FnOnce(sqlx::Error) -> ClientError {
    move |e| ClientError::Infra(InfraError::Database { action, source: e })
}

#[async_trait]
impl FindById for PgClientRepository {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<ClientRow>, ClientError> {
        sqlx::query_as!(
            ClientRow,
            r#"
            SELECT * FROM clients.tb_client WHERE pk_client = $1
            "#,
            &uuid,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_err("find client by id"))
    }
}

#[async_trait]
impl FindByDocument for PgClientRepository {
    async fn find_by_document(&self, doc: &str) -> Result<Option<ClientRow>, ClientError> {
        sqlx::query_as!(
            ClientRow,
            r#"
            SELECT * FROM clients.tb_client WHERE tx_doc = $1
            "#,
            &doc,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_err("find client by document"))
    }
}

#[async_trait]
impl FindAll for PgClientRepository {
    async fn find_all(&self) -> Result<Vec<ClientRow>, ClientError> {
        sqlx::query_as!(
            ClientRow,
            r#"
            SELECT * FROM clients.tb_client
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(sqlx_err("list clients"))
    }
}

#[async_trait]
impl CreateClient for PgClientRepository {
    async fn create(&self, c: CreateClientRow) -> Result<ClientRow, ClientError> {
        sqlx::query_as!(
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
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_err("create client"))
    }
}

#[async_trait]
impl UpdateClient for PgClientRepository {
    async fn update(&self, uuid: Uuid, input: UpdateClientRow) -> Result<ClientRow, ClientError> {
        sqlx::query_as!(
            ClientRow,
            r#"
            UPDATE clients.tb_client
            SET tx_name = COALESCE($1, tx_name),
                tx_status = COALESCE($2, tx_status),
                tx_doc = COALESCE($3, tx_doc)
            WHERE pk_client = $4
            RETURNING *
            "#,
            input.tx_name,
            input.tx_status.map(|s| s.to_string()),
            input.tx_doc,
            &uuid,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_err("update client"))
    }
}

#[async_trait]
impl DeleteClient for PgClientRepository {
    async fn delete(&self, uuid: Uuid) -> Result<ClientRow, ClientError> {
        sqlx::query_as!(
            ClientRow,
            r#"
            DELETE FROM clients.tb_client
            WHERE pk_client = $1
            RETURNING *
            "#,
            &uuid,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_err("remove client"))
    }
}

#[async_trait]
impl CreateClientWithTx for PgClientRepository {
    async fn create_with_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        c: CreateClientRow,
    ) -> Result<ClientRow, ClientError> {
        sqlx::query_as!(
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
        .fetch_one(&mut **tx)
        .await
        .map_err(sqlx_err("create client in transaction"))
    }
}

#[async_trait]
impl LinkCreatedLocationToClient for PgClientRepository {
    async fn link_created_location_to_client(
        &self,
        location_id: Uuid,
        client_id: Uuid,
    ) -> Result<ClientAddressRow, ClientError> {
        sqlx::query_as!(
            ClientAddressRow,
            r#"
            INSERT INTO clients.tb_client_address(pk_client_address, fk_client, fk_address)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &client_id,
            &location_id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_err("link created location to client"))
    }
}

#[async_trait]
impl LinkCreatedContactToClient for PgClientRepository {
    async fn link_created_contact_to_client(
        &self,
        contact_id: Uuid,
        client_id: Uuid,
    ) -> Result<ClientContactRow, ClientError> {
        sqlx::query_as!(
            ClientContactRow,
            r#"
            INSERT INTO clients.tb_client_contact(pk_client_contact, fk_client, fk_contact)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &client_id,
            &contact_id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_err("link created contact to client"))
    }
}

#[async_trait]
impl FindContactByClientId for PgClientRepository {
    async fn find_contact_by_client_id(
        &self,
        client_id: Uuid,
    ) -> Result<Option<ClientContactRow>, ClientError> {
        sqlx::query_as!(
            ClientContactRow,
            r#"
            SELECT * FROM clients.tb_client_contact WHERE fk_client = $1
            "#,
            &client_id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_err("find contact by client id"))
    }
}

#[async_trait]
impl FindLocationByClientId for PgClientRepository {
    async fn find_location_by_client_id(
        &self,
        client_id: Uuid,
    ) -> Result<Option<ClientAddressRow>, ClientError> {
        sqlx::query_as!(
            ClientAddressRow,
            r#"
            SELECT * FROM clients.tb_client_address WHERE fk_client = $1
            "#,
            &client_id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_err("find location by client id"))
    }
}
