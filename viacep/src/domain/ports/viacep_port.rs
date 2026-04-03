use crate::domain::models::viacep_model::ViaCepAddressModel;
use async_trait::async_trait;
#[derive(Debug, thiserror::Error)]
pub enum ViaCepError {
    #[error("CEP '{cep}' not found")]
    NotFound { cep: String },

    #[error("error querying ViaCEP: {0}")]
    Service(String),
}

#[async_trait]
pub trait ViaCepPort: Send + Sync {
    async fn fetch_address(&self, cep: &str) -> Result<ViaCepAddressModel, ViaCepError>;
}
