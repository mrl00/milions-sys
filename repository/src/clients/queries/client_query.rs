use uuid::Uuid;

use crate::clients::models::client::Client;

pub struct ClientQuery;

impl ClientQuery {
    /// Obtém um cliente por `pk_client` em `clients.tb_client`.
    pub async fn get_by_uuid<'a, E>(executor: E, uuid: Uuid) -> Result<Option<Client>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r: Option<Client> = sqlx::query_as!(
            Client,
            r#"
            SELECT * FROM clients.tb_client WHERE pk_client = $1
            "#,
            &uuid,
        )
        .fetch_optional(executor)
        .await?;

        Ok(r)
    }

    /// Lista todos os clientes de `clients.tb_client`.
    pub async fn get_all<'a, E>(executor: E) -> Result<Vec<Client>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r: Vec<Client> = sqlx::query_as!(
            Client,
            r#"
            SELECT * FROM clients.tb_client
            "#,
        )
        .fetch_all(executor)
        .await?;

        Ok(r)
    }
}
