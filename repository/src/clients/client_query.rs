use uuid::Uuid;

use crate::clients::models::client::Client;

pub struct ClientQuery;

impl ClientQuery {
    /// Busca um cliente pelo seu identificador (`pk_client`) na tabela `clients.tb_client`.
    ///
    /// - **executor**: executor de conexões Postgres usado para executar a consulta.
    /// - **uuid**: identificador UUID do cliente.
    ///
    /// Retorna `Ok(Some(Client))` quando encontrado, `Ok(None)` quando não houver registro
    /// correspondente e `Err(sqlx::Error)` em caso de erro de banco.
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

    /// Retorna todos os clientes cadastrados na tabela `clients.tb_client`.
    ///
    /// - **executor**: executor de conexões Postgres usado para executar a consulta.
    ///
    /// Retorna um vetor com todos os clientes ou erro de banco.
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
