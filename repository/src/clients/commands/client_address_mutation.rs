use domain::models::db::client_address::ClientAddressRow;
use uuid::Uuid;

pub struct ClientAddressMutation;

impl ClientAddressMutation {
    /// Cria vínculo cliente-endereço em `clients.tb_client_address`.
    pub async fn create<'a, E>(
        executor: E,
        client_uuid: Uuid,
        location_uuid: Uuid,
    ) -> Result<ClientAddressRow, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let r = sqlx::query_as!(
            ClientAddressRow,
            r#"
            INSERT INTO clients.tb_client_address(pk_client_address, fk_client, fk_address)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &client_uuid,
            &location_uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(r)
    }
}
