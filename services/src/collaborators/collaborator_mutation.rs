use uuid::Uuid;

use crate::collaborators::{
    collaborator_query::CollaboratorQuery,
    models::{
        collaborator::{Collaborator, CollaboratorStatus, CreateCollaborator, UpdateCollaborator},
        collaborator_contact::CollaboratorContact,
        collaborator_location::CollaboratorAddress,
    },
};

pub struct CollaboratorMutation;

impl CollaboratorMutation {
    pub async fn create<'a, E>(
        executor: E,
        c: CreateCollaborator,
    ) -> Result<Collaborator, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
    {
        let opt_collaborator = CollaboratorQuery::find_by_cpf(executor, c.tx_cpf.clone()).await?;

        match opt_collaborator {
            Some(_) => Err(sqlx::Error::RowNotFound),
            None => {
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
        }
    }

    pub async fn update<'a, E>(
        executor: E,
        uuid: Uuid,
        c: UpdateCollaborator,
    ) -> Result<Collaborator, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
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

    pub async fn activate<'a, E>(executor: E, uuid: Uuid) -> Result<Collaborator, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
    {
        let collaborator_exist = CollaboratorQuery::find_by_uuid(executor, uuid).await?;

        match collaborator_exist {
            Some(_) => {
                CollaboratorMutation::update_status(executor, uuid, CollaboratorStatus::Active)
                    .await
            }
            None => Err(sqlx::Error::RowNotFound),
        }
    }

    pub async fn deactivate<'a, E>(executor: E, uuid: Uuid) -> Result<Collaborator, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
    {
        let collaborator_exist = CollaboratorQuery::find_by_uuid(executor, uuid).await?;

        match collaborator_exist {
            Some(_) => {
                CollaboratorMutation::update_status(executor, uuid, CollaboratorStatus::Inactive)
                    .await
            }
            None => Err(sqlx::Error::RowNotFound),
        }
    }

    async fn update_status<'a, E>(
        executor: E,
        uuid: Uuid,
        status: CollaboratorStatus,
    ) -> Result<Collaborator, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
    {
        let opt_collaborator = CollaboratorQuery::find_by_uuid(executor, uuid).await?;

        match opt_collaborator {
            Some(_) => {
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
            None => Err(sqlx::Error::RowNotFound),
        }
    }
}

pub struct CollaboratorContactMutation;

impl CollaboratorContactMutation {
    pub async fn create_contact<'a, E>(
        executor: E,
        collaborator_uuid: Uuid,
        contact_uuid: Uuid,
    ) -> Result<CollaboratorContact, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
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
    pub async fn create<'a, E>(
        executor: E,
        collaborator_uuid: Uuid,
        location_uuid: Uuid,
    ) -> Result<CollaboratorAddress, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
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
