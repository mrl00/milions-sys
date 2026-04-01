use async_trait::async_trait;

use crate::models::db::viacep::ViaCepAddressModel;

#[derive(Debug, thiserror::Error)]
pub enum ViaCepError {
    #[error("CEP '{cep}' não encontrado")]
    NotFound { cep: String },

    #[error("erro ao consultar ViaCEP: {0}")]
    Service(String),
}

#[async_trait]
pub trait ViaCepPort: Send + Sync {
    async fn fetch_address(&self, cep: &str) -> Result<ViaCepAddressModel, ViaCepError>;
}
