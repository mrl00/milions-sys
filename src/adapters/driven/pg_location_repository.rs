use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::errors::infra_error::InfraError;
use crate::domain::errors::location_error::LocationError;
use crate::domain::models::db::location_row::{CreateLocationRow, LocationRow, UpdateLocationRow};
use crate::domain::ports::location_repository::*;

pub struct PgLocationRepository {
    pool: PgPool,
}

impl PgLocationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_address<'a, E>(
        executor: E,
        c: &CreateLocationRow,
    ) -> Result<Option<LocationRow>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query_as!(
            LocationRow,
            r#"
            SELECT *
            FROM locations.tb_location
            WHERE tx_street = $1
              AND tx_number = $2
              AND tx_city = $3
              AND tx_state = $4
              AND tx_zipcode = $5
            LIMIT 1
            "#,
            &c.tx_street,
            &c.tx_number,
            &c.tx_city,
            &c.tx_state,
            &c.tx_zipcode,
        )
        .fetch_optional(executor)
        .await
    }

    pub async fn create_with_executor<'a, E>(
        executor: E,
        c: CreateLocationRow,
    ) -> Result<Option<LocationRow>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        sqlx::query_as!(
            LocationRow,
            r#"
            INSERT INTO locations.tb_location (
                pk_location,
                tx_street,
                tx_number,
                tx_city,
                tx_state,
                tx_zipcode,
                tx_public_space,
                tx_address_complement,
                tx_unit,
                tx_neighborhood,
                tx_locality,
                tx_region,
                tx_ibge,
                tx_gia,
                tx_ddd,
                tx_siafi
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            ON CONFLICT (nr_hash) DO NOTHING
            RETURNING *
            "#,
            Uuid::now_v7(),
            &c.tx_street,
            &c.tx_number,
            &c.tx_city,
            &c.tx_state,
            &c.tx_zipcode,
            &c.tx_public_space,
            &c.tx_address_complement,
            &c.tx_unit,
            &c.tx_neighborhood,
            &c.tx_locality,
            &c.tx_region,
            c.tx_ibge,
            c.tx_gia,
            &c.tx_ddd,
            c.tx_siafi,
        )
        .fetch_optional(executor)
        .await
    }
}

fn sqlx_err(action: &'static str) -> impl FnOnce(sqlx::Error) -> LocationError {
    move |e| LocationError::Infra {
        source: InfraError::Database { action, source: e },
    }
}

#[async_trait]
impl FindLocationById for PgLocationRepository {
    async fn find_by_id(&self, uuid: Uuid) -> Result<Option<LocationRow>, LocationError> {
        sqlx::query_as!(
            LocationRow,
            r#"
            SELECT *
            FROM locations.tb_location
            WHERE pk_location = $1
            "#,
            &uuid,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_err("find location by id"))
    }
}

#[async_trait]
impl FindAllLocations for PgLocationRepository {
    async fn find_all(&self) -> Result<Vec<LocationRow>, LocationError> {
        sqlx::query_as!(
            LocationRow,
            r#"
            SELECT *
            FROM locations.tb_location
            ORDER BY idx_location
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(sqlx_err("list locations"))
    }
}

#[async_trait]
impl CreateLocation for PgLocationRepository {
    async fn create(&self, c: CreateLocationRow) -> Result<LocationRow, LocationError> {
        sqlx::query_as!(
            LocationRow,
            r#"INSERT INTO locations.tb_location (
            pk_location, 
            tx_street, 
            tx_number, 
            tx_city, 
            tx_state, 
            tx_zipcode,  
            tx_public_space, 
            tx_address_complement,
            tx_unit, 
            tx_neighborhood, 
            tx_locality, 
            tx_region, 
            tx_ibge, 
            tx_gia, 
            tx_ddd, 
            tx_siafi
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            RETURNING *"#,
            Uuid::now_v7(),
            &c.tx_street,
            &c.tx_number,
            &c.tx_city,
            &c.tx_state,
            &c.tx_zipcode,
            &c.tx_public_space,
            &c.tx_address_complement,
            &c.tx_unit,
            &c.tx_neighborhood,
            &c.tx_locality,
            &c.tx_region,
            c.tx_ibge,
            c.tx_gia,
            &c.tx_ddd,
            c.tx_siafi,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_err("create location"))
    }
}

#[async_trait]
impl UpdateLocation for PgLocationRepository {
    async fn update(&self, uuid: Uuid, c: UpdateLocationRow) -> Result<LocationRow, LocationError> {
        sqlx::query_as!(
            LocationRow,
            r#"UPDATE locations.tb_location
            SET 
            tx_public_space = COALESCE($1, tx_public_space), 
            tx_address_complement = COALESCE($2, tx_address_complement), 
            tx_unit = COALESCE($3, tx_unit), 
            tx_neighborhood = COALESCE($4, tx_neighborhood), 
            tx_locality = COALESCE($5, tx_locality), 
            tx_region = COALESCE($6, tx_region), 
            tx_ibge = COALESCE($7, tx_ibge), 
            tx_gia = COALESCE($8, tx_gia), 
            tx_ddd = COALESCE($9, tx_ddd), 
            tx_siafi = COALESCE($10, tx_siafi), 
            tx_street = COALESCE($11, tx_street), 
            tx_number = COALESCE($12, tx_number), 
            tx_city = COALESCE($13, tx_city), 
            tx_state = COALESCE($14, tx_state), 
            tx_zipcode = COALESCE($15, tx_zipcode)
            WHERE pk_location = $16
            RETURNING *"#,
            c.tx_public_space,
            c.tx_address_complement,
            c.tx_unit,
            c.tx_neighborhood,
            c.tx_locality,
            c.tx_region,
            c.tx_ibge,
            c.tx_gia,
            c.tx_ddd,
            c.tx_siafi,
            c.tx_street,
            c.tx_number,
            c.tx_city,
            c.tx_state,
            c.tx_zipcode,
            &uuid,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_err("update location"))
    }
}

#[async_trait]
impl DeleteLocation for PgLocationRepository {
    async fn delete(&self, uuid: Uuid) -> Result<LocationRow, LocationError> {
        sqlx::query_as!(
            LocationRow,
            r#"
            DELETE FROM locations.tb_location
            WHERE pk_location = $1
            RETURNING *
            "#,
            &uuid,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_err("remove location"))
    }
}
