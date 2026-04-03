use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::errors::collaborator_error::CollaboratorError;
use crate::domain::models::db::collaborator_contact_row::CollaboratorContactRow;
use crate::domain::models::db::collaborator_location_row::CollaboratorAddressRow;
use crate::domain::models::db::collaborator_row::{
    CollaboratorRow, CreateCollaboratorRow, UpdateCollaboratorRow,
};
use crate::domain::ports::collaborator_repository::*;
use types::errors::infra_error::InfraError;

pub struct PgCollaboratorRepository {
    pool: PgPool,
}

impl PgCollaboratorRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_contact<'a, E>(
        executor: E,
        collaborator_uuid: Uuid,
        contact_uuid: Uuid,
    ) -> Result<CollaboratorContactRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query_as!(
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
        .await
    }

    pub async fn create_address<'a, E>(
        executor: E,
        collaborator_uuid: Uuid,
        location_uuid: Uuid,
    ) -> Result<CollaboratorAddressRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query_as!(
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
        .await
    }
}

fn sqlx_err(action: &'static str) -> impl FnOnce(sqlx::Error) -> CollaboratorError {
    move |e| CollaboratorError::Infra {
        source: InfraError::Database { action, source: e },
    }
}

#[async_trait]
impl FindCollaboratorById for PgCollaboratorRepository {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<CollaboratorRow>, CollaboratorError> {
        sqlx::query_as!(
            CollaboratorRow,
            r#"
            SELECT *
            FROM collaborators.tb_collaborator
            WHERE pk_collaborator = $1
            "#,
            &uuid
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_err("find collaborator by id"))
    }
}

#[async_trait]
impl FindCollaboratorByCpf for PgCollaboratorRepository {
    async fn find_by_cpf(&self, cpf: &str) -> Result<Option<CollaboratorRow>, CollaboratorError> {
        sqlx::query_as!(
            CollaboratorRow,
            r#"
            SELECT *
            FROM collaborators.tb_collaborator
            WHERE tx_cpf = $1
            "#,
            &cpf
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_err("find collaborator by CPF"))
    }
}

#[async_trait]
impl FindAllCollaborators for PgCollaboratorRepository {
    async fn find_all(&self) -> Result<Vec<CollaboratorRow>, CollaboratorError> {
        sqlx::query_as!(
            CollaboratorRow,
            r#"
            SELECT *
            FROM collaborators.tb_collaborator
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(sqlx_err("list collaborators"))
    }
}

#[async_trait]
impl CreateCollaborator for PgCollaboratorRepository {
    async fn create(&self, c: CreateCollaboratorRow) -> Result<CollaboratorRow, CollaboratorError> {
        sqlx::query_as!(
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
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_err("create collaborator"))
    }
}

#[async_trait]
impl UpdateCollaborator for PgCollaboratorRepository {
    async fn update(
        &self,
        uuid: Uuid,
        c: UpdateCollaboratorRow,
    ) -> Result<CollaboratorRow, CollaboratorError> {
        sqlx::query_as!(
            CollaboratorRow,
            r#"
            UPDATE collaborators.tb_collaborator
            SET tx_name = COALESCE($1, tx_name),
                tx_level = COALESCE($2, tx_level),
                tx_status = COALESCE($3, tx_status),
                tx_cpf = COALESCE($4, tx_cpf)
            WHERE pk_collaborator = $5
            RETURNING *
            "#,
            c.tx_name,
            c.tx_level,
            c.tx_status,
            c.tx_cpf,
            &uuid,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_err("update collaborator"))
    }
}

#[async_trait]
impl DeleteCollaborator for PgCollaboratorRepository {
    async fn delete(&self, uuid: Uuid) -> Result<CollaboratorRow, CollaboratorError> {
        sqlx::query_as!(
            CollaboratorRow,
            r#"
            DELETE FROM collaborators.tb_collaborator
            WHERE pk_collaborator = $1
            RETURNING *
            "#,
            &uuid,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_err("remove collaborator"))
    }
}
