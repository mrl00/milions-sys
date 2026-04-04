use async_trait::async_trait;
use reqwest::Client;

use crate::domain::models::viacep_model::ViaCepAddressModel;
use crate::domain::ports::viacep_port::{ViaCepError, ViaCepPort};

pub struct ViaCepClient {
    client: Client,
    base_url: String,
}

impl ViaCepClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://viacep.com.br/ws".to_string(),
        }
    }

    pub fn with_base_url(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
}

impl Default for ViaCepClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ViaCepPort for ViaCepClient {
    async fn fetch_address(&self, cep: &str) -> Result<ViaCepAddressModel, ViaCepError> {
        let url = format!("{}/{}/json", self.base_url, cep);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| ViaCepError::Service(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ViaCepError::NotFound {
                cep: cep.to_string(),
            });
        }

        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ViaCepError::Service(e.to_string()))?;

        if body.get("erro").and_then(|v| v.as_bool()) == Some(true) {
            return Err(ViaCepError::NotFound {
                cep: cep.to_string(),
            });
        }

        serde_json::from_value(body).map_err(|e| ViaCepError::Service(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn valid_address_json() -> serde_json::Value {
        serde_json::json!({
            "cep": "01001-000",
            "logradouro": "Praça da Sé",
            "complemento": "lado ímpar",
            "unidade": "",
            "bairro": "Sé",
            "localidade": "São Paulo",
            "uf": "SP",
            "estado": "São Paulo",
            "regiao": "Sudeste",
            "ibge": "3550308",
            "gia": "1004",
            "ddd": "11",
            "siafi": "7107"
        })
    }

    #[tokio::test]
    async fn fetch_address_returns_address_for_valid_cep() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/01001-000/json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(valid_address_json()))
            .mount(&server)
            .await;

        let client = ViaCepClient::with_base_url(server.uri());
        let result = client.fetch_address("01001-000").await.unwrap();

        assert_eq!(result.cep, "01001-000");
        assert_eq!(result.logradouro, "Praça da Sé");
        assert_eq!(result.bairro, "Sé");
        assert_eq!(result.localidade, "São Paulo");
        assert_eq!(result.uf, "SP");
        assert_eq!(result.ibge, "3550308");
    }

    #[tokio::test]
    async fn fetch_address_returns_not_found_for_invalid_cep() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/00000-000/json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "erro": true
            })))
            .mount(&server)
            .await;

        let client = ViaCepClient::with_base_url(server.uri());
        let result = client.fetch_address("00000-000").await;

        assert!(matches!(result, Err(ViaCepError::NotFound { .. })));
    }

    #[tokio::test]
    async fn fetch_address_returns_service_error_on_http_failure() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/01001-000/json"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = ViaCepClient::with_base_url(server.uri());
        let result = client.fetch_address("01001-000").await;

        assert!(matches!(result, Err(ViaCepError::NotFound { .. })));
    }
}
