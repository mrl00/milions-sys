use uuid::Uuid;

use crate::locations::models::location::Location;

pub struct LocationQuery;

impl LocationQuery {
    pub async fn get_by_uuid<'a, E>(
        executor: E,
        uuid: Uuid,
    ) -> Result<Option<Location>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
    {
        let r: Option<Location> = sqlx::query_as!(
            Location,
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
}
