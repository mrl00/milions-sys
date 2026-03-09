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
    /// Cria um novo cliente na tabela `clients.tb_client`.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `INSERT`.
    /// - **c**: dados para criação (`CreateClient`), incluindo nome e status inicial.
    ///
    /// Gera um novo `pk_client`, insere o registro e retorna o cliente criado.
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

    /// Ativa um cliente, alterando seu status para `Active`.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `UPDATE`.
    /// - **uuid**: identificador UUID do cliente.
    ///
    /// Delegado para `update_status` com `ClientStatus::Active`.
    pub async fn activate<'a, E>(executor: E, uuid: Uuid) -> Result<Client, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ClientMutation::update_status(executor, uuid, ClientStatus::Active).await
    }

    /// Desativa um cliente, alterando seu status para `Inactive`.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `UPDATE`.
    /// - **uuid**: identificador UUID do cliente.
    ///
    /// Delegado para `update_status` com `ClientStatus::Inactive`.
    pub async fn deactivate<'a, E>(executor: E, uuid: Uuid) -> Result<Client, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ClientMutation::update_status(executor, uuid, ClientStatus::Inactive).await
    }

    /// Atualiza apenas o status (`tx_status`) de um cliente para o valor informado.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `UPDATE`.
    /// - **uuid**: identificador UUID do cliente.
    /// - **status**: novo status (`ClientStatus`) a ser aplicado.
    ///
    /// Retorna o cliente com status atualizado.
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
    /// Cria o vínculo entre um cliente e um contato na tabela `clients.tb_client_contact`.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `INSERT`.
    /// - **client_uuid**: identificador do cliente (`fk_client`).
    /// - **contact_uuid**: identificador do contato (`fk_contact`).
    ///
    /// Gera um novo `pk_client_contact`, insere o registro e retorna a associação criada.
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
    /// Cria o vínculo entre um cliente e um endereço na tabela `clients.tb_client_address`.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `INSERT`.
    /// - **client_uuid**: identificador do cliente (`fk_client`).
    /// - **location_uuid**: identificador do endereço (`fk_address`).
    ///
    /// Gera um novo `pk_client_address`, insere o registro e retorna a associação criada.
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
    /// Cria um novo projeto para um cliente, associado a um endereço, na tabela `clients.tb_project`.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `INSERT`.
    /// - **client_uuid**: identificador do cliente (`fk_client`).
    /// - **location_uuid**: identificador do endereço (`fk_address`).
    ///
    /// Gera um novo `pk_project`, insere o registro e retorna o projeto criado.
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

    /// Marca um projeto como inativo (`ProjectStatus::Inactive`).
    ///
    /// - **executor**: executor SQL usado para rodar o `UPDATE`.
    /// - **uuid**: identificador UUID do projeto.
    pub async fn deactivate<'a, E>(executor: E, uuid: Uuid) -> Result<ClientProject, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ClientProjectMutation::update_status(executor, uuid, ProjectStatus::Inactive).await
    }

    /// Marca um projeto como ativo (`ProjectStatus::Active`).
    ///
    /// - **executor**: executor SQL usado para rodar o `UPDATE`.
    /// - **uuid**: identificador UUID do projeto.
    pub async fn activate<'a, E>(executor: E, uuid: Uuid) -> Result<ClientProject, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ClientProjectMutation::update_status(executor, uuid, ProjectStatus::Active).await
    }

    /// Coloca um projeto em estado parado (`ProjectStatus::Stopped`).
    ///
    /// - **executor**: executor SQL usado para rodar o `UPDATE`.
    /// - **uuid**: identificador UUID do projeto.
    pub async fn stop<'a, E>(executor: E, uuid: Uuid) -> Result<ClientProject, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ClientProjectMutation::update_status(executor, uuid, ProjectStatus::Stopped).await
    }

    /// Coloca um projeto em andamento (`ProjectStatus::InProgress`).
    ///
    /// - **executor**: executor SQL usado para rodar o `UPDATE`.
    /// - **uuid**: identificador UUID do projeto.
    pub async fn start<'a, E>(executor: E, uuid: Uuid) -> Result<ClientProject, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ClientProjectMutation::update_status(executor, uuid, ProjectStatus::InProgress).await
    }

    /// Marca um projeto como concluído (`ProjectStatus::Done`).
    ///
    /// - **executor**: executor SQL usado para rodar o `UPDATE`.
    /// - **uuid**: identificador UUID do projeto.
    pub async fn done<'a, E>(executor: E, uuid: Uuid) -> Result<ClientProject, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        ClientProjectMutation::update_status(executor, uuid, ProjectStatus::Done).await
    }

    /// Atualiza o status (`tx_status`) de um projeto para o valor informado.
    ///
    /// - **executor**: executor SQL usado para rodar o `UPDATE`.
    /// - **uuid**: identificador UUID do projeto.
    /// - **status**: novo status (`ProjectStatus`) a ser aplicado.
    ///
    /// Retorna o projeto com status atualizado.
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
    /// Cria a alocação de um colaborador em um projeto de cliente na tabela
    /// `clients.tb_allocated_collaborator`.
    ///
    /// - **executor**: executor SQL usado para rodar o `INSERT`.
    /// - **client_project_uuid**: identificador do projeto (`fk_project`).
    /// - **collaborator_uuid**: identificador do colaborador (`fk_collaborator`).
    ///
    /// Gera um novo `pk_allocated_collaborator`, insere o registro e retorna a alocação criada.
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
