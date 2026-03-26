use uuid::Uuid;

use domain::models::db::client::ClientRow;

pub struct ClientQuery;

impl ClientQuery {
    /// Obtém um cliente por `pk_client` em `clients.tb_client`.
    pub async fn find_by_uuid<'a, E>(
        executor: E,
        uuid: Uuid,
    ) -> Result<Option<ClientRow>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r: Option<ClientRow> = sqlx::query_as!(
            ClientRow,
            r#"
            SELECT * FROM clients.tb_client WHERE pk_client = $1
            "#,
            &uuid,
        )
        .fetch_optional(executor)
        .await?;

        Ok(r)
    }

    /// Obtém um cliente por `documento` em `clients.tb_client`.
    pub async fn find_by_document<'a, E>(
        executor: E,
        document: String,
    ) -> Result<Option<ClientRow>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r: Option<ClientRow> = sqlx::query_as!(
            ClientRow,
            r#"
            SELECT * FROM clients.tb_client WHERE tx_doc = $1
            "#,
            &document,
        )
        .fetch_optional(executor)
        .await?;

        Ok(r)
    }

    /// Lista todos os clientes de `clients.tb_client`.
    pub async fn find_all<'a, E>(executor: E) -> Result<Vec<ClientRow>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r: Vec<ClientRow> = sqlx::query_as!(
            ClientRow,
            r#"
            SELECT * FROM clients.tb_client
            "#,
        )
        .fetch_all(executor)
        .await?;

        Ok(r)
    }
}
