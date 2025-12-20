use sqlx::PgPool;
use uuid::Uuid;

use crate::locations::models::location::Location;

pub struct LocationQuery;

impl LocationQuery {
    pub async fn get_by_id(pool: &PgPool, uuid: Uuid) -> Result<Option<Location>, sqlx::Error> {
        let r: Option<Location> = sqlx::query_as!(
            Location,
            r#"
            SELECT *
            FROM locations.tb_location
            WHERE pk_location = $1
            "#,
            &uuid,
        )
        .fetch_optional(pool)
        .await?;

        Ok(r)
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Location>, sqlx::Error> {
        let r: Vec<Location> = sqlx::query_as!(
            Location,
            r#"
            SELECT *
            FROM locations.tb_location
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(r)
    }
}
