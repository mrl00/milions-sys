use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::errors::LocationError;
use crate::domain::models::db::location_row::{CreateLocationRow, LocationRow, UpdateLocationRow};
use crate::domain::ports::location_repository::*;
use types::errors::infra_error::InfraError;

pub struct PgLocationRepository {
    pool: PgPool,
}

impl PgLocationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
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
        .map_err(sqlx_err("buscar localização por id"))
    }
}

#[async_trait]
impl FindLocationByHash for PgLocationRepository {
    async fn find_by_hash(&self, hash: i64) -> Result<Option<LocationRow>, LocationError> {
        sqlx::query_as!(
            LocationRow,
            r#"
            SELECT *
            FROM locations.tb_location
            WHERE nr_hash = $1
            "#,
            hash,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_err("buscar localização por hash"))
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
        .map_err(sqlx_err("listar localizações"))
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
            tx_siafi, 
            nr_hash
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
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
            c.nr_hash as i64,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_err("criar localização"))
    }
}

#[async_trait]
impl UpdateLocation for PgLocationRepository {
    async fn update(&self, uuid: Uuid, c: UpdateLocationRow) -> Result<LocationRow, LocationError> {
        sqlx::query_as!(
            LocationRow,
            r#"UPDATE locations.tb_location
            SET 
            tx_public_space = $1, 
            tx_address_complement = $2, 
            tx_unit = $3, 
            tx_neighborhood = $4, 
            tx_locality = $5, 
            tx_region = $6, 
            tx_ibge = $7, 
            tx_gia = $8, 
            tx_ddd = $9, 
            tx_siafi = $10, 
            tx_street = $11, 
            tx_number = $12, 
            tx_city = $13, 
            tx_state = $14, 
            tx_zipcode = $15
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
        .map_err(sqlx_err("atualizar localização"))
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
        .map_err(sqlx_err("remover localização"))
    }
}

impl FindOrCreateLocation for PgLocationRepository {}
impl FindAndUpdateLocation for PgLocationRepository {}
impl FindAndDeleteLocation for PgLocationRepository {}
