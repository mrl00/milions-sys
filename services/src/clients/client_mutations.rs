use uuid::Uuid;

use crate::clients::models::{
    client::{Client, ClientStatus, CreateClient},
    client_address::ClientAddress,
    client_contact::ClientContact,
    client_project_collaborator::ClientProjectCollaborator,
    client_projects::{ClientProject, ProjectStatus},
};

pub struct ClientMutation;

impl ClientMutation {
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

    pub async fn activate<'a, E>(executor: E, uuid: Uuid) -> Result<Client, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ClientMutation::update_status(executor, uuid, ClientStatus::Active).await
    }

    pub async fn deactivate<'a, E>(executor: E, uuid: Uuid) -> Result<Client, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ClientMutation::update_status(executor, uuid, ClientStatus::Inactive).await
    }

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

pub struct ClientProjectMutation;

impl ClientProjectMutation {
    pub async fn create_project<'a, E>(
        executor: E,
        client_uuid: Uuid,
        location_uuid: Uuid,
    ) -> Result<ClientProject, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ClientProject,
            r#"
            INSERT INTO clients.tb_project (pk_project, fk_client, fk_address)
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

    pub async fn deactivate<'a, E>(executor: E, uuid: Uuid) -> Result<ClientProject, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ClientProjectMutation::update_status(executor, uuid, ProjectStatus::Inactive).await
    }

    pub async fn activate<'a, E>(executor: E, uuid: Uuid) -> Result<ClientProject, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ClientProjectMutation::update_status(executor, uuid, ProjectStatus::Active).await
    }

    pub async fn stop<'a, E>(executor: E, uuid: Uuid) -> Result<ClientProject, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ClientProjectMutation::update_status(executor, uuid, ProjectStatus::Stopped).await
    }

    pub async fn start<'a, E>(executor: E, uuid: Uuid) -> Result<ClientProject, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ClientProjectMutation::update_status(executor, uuid, ProjectStatus::InProgress).await
    }

    pub async fn done<'a, E>(executor: E, uuid: Uuid) -> Result<ClientProject, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ClientProjectMutation::update_status(executor, uuid, ProjectStatus::Done).await
    }

    async fn update_status<'a, E>(
        executor: E,
        uuid: Uuid,
        status: ProjectStatus,
    ) -> Result<ClientProject, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ClientProject,
            r#"
            UPDATE clients.tb_project
            SET tx_status = $1
            WHERE pk_project = $2
            RETURNING *
            "#,
            &status.to_string(),
            &uuid,
        )
        .fetch_one(executor)
        .await?;
        Ok(r)
    }
}

pub struct ClientProjectCollaboratorMutation;

impl ClientProjectCollaboratorMutation {
    pub async fn create_collaborator<'a, E>(
        executor: E,
        client_project_uuid: Uuid,
        collaborator_uuid: Uuid,
    ) -> Result<ClientProjectCollaborator, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ClientProjectCollaborator,
            r#"
            INSERT INTO clients.tb_allocated_collaborator (pk_allocated_collaborator, fk_project, fk_collaborator)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &client_project_uuid,
            &collaborator_uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(r)
    }
}
