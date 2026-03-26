use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViaCepAddressModel {
    pub cep: String,
    pub logradouro: String,
    pub complemento: String,
    pub unidade: String,
    pub bairro: String,
    pub localidade: String,
    pub uf: String,
    pub estado: String,
    pub regiao: String,
    pub ibge: String,
    pub gia: String,
    pub ddd: String,
    pub siafi: String,
}
