use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

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

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct LocationModel {
    pub pk_location: Uuid,
    pub idx_location: i64,
    pub tx_public_space: String, //logradouro
    pub tx_address_complement: Option<String>,
    pub tx_unit: String,
    pub tx_neighborhood: String,
    pub tx_locality: String,
    pub tx_region: String,
    pub tx_ibge: Option<String>,
    pub tx_gia: Option<String>,
    pub tx_ddd: String,
    pub tx_siafi: Option<String>,
    pub tx_street: String,
    pub tx_number: String,
    pub tx_city: String,
    pub tx_state: String,
    pub tx_zipcode: String,
    pub nr_hash: i64,
    pub ts_location_created_at: NaiveDateTime,
    pub ts_location_updated_at: NaiveDateTime,
}

impl Hash for LocationModel {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tx_public_space.hash(state);
        self.tx_address_complement.hash(state);
        self.tx_unit.hash(state);
        self.tx_neighborhood.hash(state);
        self.tx_locality.hash(state);
        self.tx_region.hash(state);
        self.tx_ibge.hash(state);
        self.tx_gia.hash(state);
        self.tx_ddd.hash(state);
        self.tx_siafi.hash(state);
        self.tx_street.hash(state);
        self.tx_number.hash(state);
        self.tx_city.hash(state);
        self.tx_state.hash(state);
        self.tx_zipcode.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateLocationModel {
    pub tx_public_space: String,
    pub tx_address_complement: String,
    pub tx_unit: String,
    pub tx_neighborhood: String,
    pub tx_locality: String,
    pub tx_region: String,
    pub tx_ibge: Option<String>,
    pub tx_gia: Option<String>,
    pub tx_ddd: String,
    pub tx_siafi: Option<String>,
    pub tx_street: String,
    pub tx_number: String,
    pub tx_city: String,
    pub tx_state: String,
    pub tx_zipcode: String,
}

impl CreateLocationModel {
    pub fn gen_hash(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.tx_public_space.hash(&mut hasher);
        self.tx_address_complement.hash(&mut hasher);
        self.tx_unit.hash(&mut hasher);
        self.tx_neighborhood.hash(&mut hasher);
        self.tx_locality.hash(&mut hasher);
        self.tx_region.hash(&mut hasher);
        self.tx_ddd.hash(&mut hasher);
        self.tx_state.hash(&mut hasher);
        self.tx_zipcode.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateLocationModel {
    pub tx_public_space: Option<String>,
    pub tx_address_complement: Option<String>,
    pub tx_unit: Option<String>,
    pub tx_neighborhood: Option<String>,
    pub tx_locality: Option<String>,
    pub tx_region: Option<String>,
    pub tx_ibge: Option<String>,
    pub tx_gia: Option<String>,
    pub tx_ddd: Option<String>,
    pub tx_siafi: Option<String>,
    pub tx_street: Option<String>,
    pub tx_number: Option<String>,
    pub tx_city: Option<String>,
    pub tx_state: Option<String>,
    pub tx_zipcode: Option<String>,
}
