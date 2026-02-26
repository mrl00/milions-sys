use sqlx::PgPool;
use uuid::Uuid;

use crate::clients::models::client::Client;

pub struct ClientQuery;

impl ClientQuery {
    pub async fn get_by_uuid(pool: &PgPool, uuid: Uuid) -> Result<Option<Client>, sqlx::Error> {
        let r: Option<Client> = sqlx::query_as!(
            Client,
            r#"
            SELECT * FROM clients.tb_client WHERE pk_client = $1
            "#,
            &uuid,
        )
        .fetch_optional(pool)
        .await?;

        Ok(r)
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Client>, sqlx::Error> {
        let r: Vec<Client> = sqlx::query_as!(
            Client,
            r#"
            SELECT * FROM clients.tb_client
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(r)
    }
}
