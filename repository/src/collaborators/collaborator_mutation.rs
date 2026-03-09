use uuid::Uuid;

use crate::collaborators::models::{
    collaborator::{Collaborator, CollaboratorStatus, CreateCollaborator, UpdateCollaborator},
    collaborator_contact::CollaboratorContact,
    collaborator_location::CollaboratorAddress,
};

pub struct CollaboratorMutation;

impl CollaboratorMutation {
    /// Cria um novo colaborador na tabela `collaborators.tb_collaborator`, garantindo unicidade de CPF.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `INSERT`.
    /// - **c**: dados para criação (`CreateCollaborator`).
    ///
    /// Se já existir colaborador com o mesmo CPF, retorna `sqlx::Error::RowNotFound` como sinal de conflito.
    /// Caso contrário, insere e retorna o colaborador criado.
    pub async fn create<'a, E>(
        executor: E,
        c: CreateCollaborator,
    ) -> Result<Collaborator, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r: Collaborator = sqlx::query_as!(
                    Collaborator,
                    r#"
                    INSERT INTO collaborators.tb_collaborator (pk_collaborator, tx_name, tx_cpf, tx_level, tx_status)
                    VALUES ($1, $2, $3, $4, $5)
                    RETURNING *
                    "#,
                    Uuid::now_v7(),
                    &c.tx_name,
                    &c.tx_cpf,
                    &c.tx_level.to_string(),
                    &c.tx_status.to_string(),
                )
                .fetch_one(executor)
                .await?;

        Ok(r)
    }

    /// Atualiza os dados principais de um colaborador existente.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `UPDATE`.
    /// - **uuid**: identificador UUID do colaborador.
    /// - **c**: dados a serem atualizados (`UpdateCollaborator`).
    ///
    /// Retorna o colaborador atualizado ou erro de banco.
    pub async fn update<'a, E>(
        executor: E,
        uuid: Uuid,
        c: UpdateCollaborator,
    ) -> Result<Collaborator, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r: Collaborator = sqlx::query_as!(
            Collaborator,
            r#"
            UPDATE collaborators.tb_collaborator
            SET tx_name = $1, tx_level = $2, tx_status = $3, tx_cpf = $4
            WHERE pk_collaborator = $5
            RETURNING *
            "#,
            c.tx_name,
            c.tx_level,
            c.tx_status,
            c.tx_cpf,
            &uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(r)
    }

    /// Ativa um colaborador, alterando seu status para `Active`.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `UPDATE`.
    /// - **uuid**: identificador UUID do colaborador.
    ///
    /// Verifica se o colaborador existe; se não existir retorna `sqlx::Error::RowNotFound`.
    /// Caso exista, atualiza o status e retorna o registro atualizado.
    pub async fn activate<'a, E>(executor: E, uuid: Uuid) -> Result<Collaborator, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        CollaboratorMutation::update_status(executor, uuid, CollaboratorStatus::Active).await
    }

    /// Desativa um colaborador, alterando seu status para `Inactive`.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `UPDATE`.
    /// - **uuid**: identificador UUID do colaborador.
    ///
    /// Verifica se o colaborador existe; se não existir retorna `sqlx::Error::RowNotFound`.
    /// Caso exista, atualiza o status e retorna o registro atualizado.
    pub async fn deactivate<'a, E>(executor: E, uuid: Uuid) -> Result<Collaborator, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        CollaboratorMutation::update_status(executor, uuid, CollaboratorStatus::Inactive).await
    }

    /// Atualiza apenas o status (`tx_status`) de um colaborador para o valor informado.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `UPDATE`.
    /// - **uuid**: identificador UUID do colaborador.
    /// - **status**: novo status (`CollaboratorStatus`) a ser aplicado.
    ///
    /// Verifica se o colaborador existe; se não existir retorna `sqlx::Error::RowNotFound`.
    /// Caso exista, atualiza o status e retorna o registro atualizado.
    async fn update_status<'a, E>(
        executor: E,
        uuid: Uuid,
        status: CollaboratorStatus,
    ) -> Result<Collaborator, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r: Collaborator = sqlx::query_as!(
            Collaborator,
            r#"
                    UPDATE collaborators.tb_collaborator
                    SET tx_status = $1
                    WHERE pk_collaborator = $2
                    RETURNING *
                    "#,
            status.to_string(),
            &uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(r)
    }
}

pub struct CollaboratorContactMutation;

impl CollaboratorContactMutation {
    /// Cria um vínculo entre colaborador e contato na tabela `collaborators.tb_collaborator_contact`.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `INSERT`.
    /// - **collaborator_uuid**: identificador do colaborador (`fk_collaborator`).
    /// - **contact_uuid**: identificador do contato (`fk_contact`).
    ///
    /// Retorna o registro de relação criado.
    pub async fn create_contact<'a, E>(
        executor: E,
        collaborator_uuid: Uuid,
        contact_uuid: Uuid,
    ) -> Result<CollaboratorContact, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            CollaboratorContact,
            r#"
            INSERT INTO collaborators.tb_collaborator_contact (fk_collaborator, fk_contact)
            VALUES ($1, $2)
            RETURNING *
            "#,
            &collaborator_uuid,
            &contact_uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(r)
    }
}

pub struct CollaboratorAddressMutation;

impl CollaboratorAddressMutation {
    /// Cria um vínculo entre colaborador e endereço na tabela `collaborators.tb_collaborator_address`.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `INSERT`.
    /// - **collaborator_uuid**: identificador do colaborador (`fk_collaborator`).
    /// - **location_uuid**: identificador do endereço (`fk_address`).
    ///
    /// Gera um novo `pk_collaborator_address`, insere o registro e retorna a associação criada.
    pub async fn create<'a, E>(
        executor: E,
        collaborator_uuid: Uuid,
        location_uuid: Uuid,
    ) -> Result<CollaboratorAddress, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            CollaboratorAddress,
            r#"
            INSERT INTO collaborators.tb_collaborator_address(pk_collaborator_address, fk_collaborator, fk_address)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &collaborator_uuid,
            &location_uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(r)
    }
}
