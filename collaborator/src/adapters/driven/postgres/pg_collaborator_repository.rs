use crate::domain::model::{
    CollaboratorAddressRow, CollaboratorContactRow, CollaboratorRow, CollaboratorStatus,
    CreateCollaboratorRow, UpdateCollaboratorRow,
};
use sqlx::PgPool;
use uuid::Uuid;

pub struct PgCollaboratorRepository {
    pool: PgPool,
}

impl PgCollaboratorRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create<'a, E>(
        executor: E,
        c: CreateCollaboratorRow,
    ) -> Result<CollaboratorRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r: CollaboratorRow = sqlx::query_as!(
                    CollaboratorRow,
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

    pub async fn update<'a, E>(
        executor: E,
        uuid: Uuid,
        c: UpdateCollaboratorRow,
    ) -> Result<CollaboratorRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r: CollaboratorRow = sqlx::query_as!(
            CollaboratorRow,
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

    pub async fn activate<'a, E>(executor: E, uuid: Uuid) -> Result<CollaboratorRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        Self::update_status(executor, uuid, CollaboratorStatus::Active).await
    }

    pub async fn deactivate<'a, E>(executor: E, uuid: Uuid) -> Result<CollaboratorRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        Self::update_status(executor, uuid, CollaboratorStatus::Inactive).await
    }

    async fn update_status<'a, E>(
        executor: E,
        uuid: Uuid,
        status: CollaboratorStatus,
    ) -> Result<CollaboratorRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r: CollaboratorRow = sqlx::query_as!(
            CollaboratorRow,
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

    pub async fn find_by_uuid<'a, E>(
        executor: E,
        uuid: Uuid,
    ) -> Result<Option<CollaboratorRow>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let collaborator = sqlx::query_as!(
            CollaboratorRow,
            r#"
            SELECT *
            FROM collaborators.tb_collaborator
            WHERE pk_collaborator = $1
            "#,
            &uuid
        )
        .fetch_optional(executor)
        .await?;

        Ok(collaborator)
    }

    pub async fn find_by_cpf<'a, E>(
        executor: E,
        cpf: String,
    ) -> Result<Option<CollaboratorRow>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let collaborator = sqlx::query_as!(
            CollaboratorRow,
            r#"
            SELECT *
            FROM collaborators.tb_collaborator
            WHERE tx_cpf = $1
            "#,
            &cpf
        )
        .fetch_optional(executor)
        .await?;

        Ok(collaborator)
    }

    pub async fn create_contact<'a, E>(
        executor: E,
        collaborator_uuid: Uuid,
        contact_uuid: Uuid,
    ) -> Result<CollaboratorContactRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            CollaboratorContactRow,
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

    pub async fn create_address<'a, E>(
        executor: E,
        collaborator_uuid: Uuid,
        location_uuid: Uuid,
    ) -> Result<CollaboratorAddressRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            CollaboratorAddressRow,
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
