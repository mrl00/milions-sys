use uuid::Uuid;

use domain::models::db::location_row::LocationRow;

pub struct LocationQuery;

impl LocationQuery {
    /// Obtém uma localização por `pk_location` em `locations.tb_location`.
    pub async fn get_by_uuid<'a, E>(
        executor: E,
        uuid: Uuid,
    ) -> Result<Option<LocationRow>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r: Option<LocationRow> = sqlx::query_as!(
            LocationRow,
            r#"
            SELECT *
            FROM locations.tb_location
            WHERE pk_location = $1
            "#,
            &uuid,
        )
        .fetch_optional(executor)
        .await?;

        Ok(r)
    }

    pub async fn find_by_hash<'a, E>(
        executor: E,
        hash: i64,
    ) -> Result<Option<LocationRow>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r: Option<LocationRow> = sqlx::query_as!(
            LocationRow,
            r#"
            SELECT *
            FROM locations.tb_location
            WHERE nr_hash = $1
            "#,
            hash,
        )
        .fetch_optional(executor)
        .await?;

        Ok(r)
    }
}
