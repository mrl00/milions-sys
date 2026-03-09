use uuid::Uuid;

use crate::locations::models::location::Location;

pub struct LocationQuery;

impl LocationQuery {
    /// Busca uma localização pelo seu identificador (`pk_location`) na tabela `locations.tb_location`.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar a query.
    /// - **uuid**: identificador UUID da localização.
    ///
    /// Retorna `Ok(Some(Location))` quando encontrada, `Ok(None)` quando não houver registro
    /// correspondente e `Err(sqlx::Error)` em caso de erro de banco.
    pub async fn get_by_uuid<'a, E>(
        executor: E,
        uuid: Uuid,
    ) -> Result<Option<Location>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
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
