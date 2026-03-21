use sqlx::PgPool;

use crate::clients::models::service_create_client::ServiceCreateClient;

pub struct ClientServiceMutation;

impl ClientServiceMutation {
    /// Registra um novo cliente.
    pub async fn register_new_client(
        _pool: &PgPool,
        _create_client: ServiceCreateClient,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
