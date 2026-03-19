use uuid::Uuid;

use crate::clients::models::client::ClientModel;

pub struct ClientQuery;

impl ClientQuery {
    /// Obtém um cliente por `pk_client` em `clients.tb_client`.
    pub async fn get_by_uuid<'a, E>(
        executor: E,
        uuid: Uuid,
    ) -> Result<Option<ClientModel>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r: Option<ClientModel> = sqlx::query_as!(
            ClientModel,
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
    pub async fn get_by_document<'a, E>(
        executor: E,
        document: String,
    ) -> Result<Option<ClientModel>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r: Option<ClientModel> = sqlx::query_as!(
            ClientModel,
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
    pub async fn get_all<'a, E>(executor: E) -> Result<Vec<ClientModel>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r: Vec<ClientModel> = sqlx::query_as!(
            ClientModel,
            r#"
            SELECT * FROM clients.tb_client
            "#,
        )
        .fetch_all(executor)
        .await?;

        Ok(r)
    }
}
