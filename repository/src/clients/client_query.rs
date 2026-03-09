use sqlx::PgPool;
use uuid::Uuid;

use crate::clients::models::client::Client;

pub struct ClientQuery;

impl ClientQuery {
    /// Busca um cliente pelo seu identificador (`pk_client`) na tabela `clients.tb_client`.
    ///
    /// - **pool**: pool de conexões Postgres usado para executar a consulta.
    /// - **uuid**: identificador UUID do cliente.
    ///
    /// Retorna `Ok(Some(Client))` quando encontrado, `Ok(None)` quando não houver registro
    /// correspondente e `Err(sqlx::Error)` em caso de erro de banco.
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

    /// Retorna todos os clientes cadastrados na tabela `clients.tb_client`.
    ///
    /// - **pool**: pool de conexões Postgres usado para executar a consulta.
    ///
    /// Retorna um vetor com todos os clientes ou erro de banco.
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
