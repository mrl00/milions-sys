use std::hash::Hash;

use domain::{locations::location::*, types::numeric::Numeric};

#[derive(Debug)]
pub struct ServiceCreateLocationModel {
    pub tx_public_space: PublicSpace,
    pub tx_address_complement: AddressComplement,
    pub tx_unit: Unit,
    pub tx_neighborhood: Neighborhood,
    pub tx_locality: Locality,
    pub tx_region: Region,
    pub tx_ibge: Ibge,
    pub tx_gia: Gia,
    pub tx_ddd: Ddd,
    pub tx_siafi: Siafi,
    pub tx_street: Street,
    pub tx_number: Number,
    pub tx_city: City,
    pub tx_state: State,
    pub tx_zipcode: Zipcode,
    pub nr_hash: Numeric,
}

impl Hash for ServiceCreateLocationModel {
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
        self.nr_hash.hash(state);
    }
}
