use crate::types::{alphabetic::Alphabetic, alphanumeric::Alphanumeric, numeric::Numeric};

pub type PublicSpace = Alphanumeric;
pub type AddressComplement = Alphanumeric;
pub type Unit = Alphanumeric;
pub type Neighborhood = Alphanumeric;
pub type Locality = Alphabetic;
pub type Region = Option<Alphabetic>;
pub type Ibge = Option<Alphanumeric>;
pub type Gia = Option<Alphanumeric>;
pub type Ddd = Alphanumeric;
pub type Siafi = Option<Alphanumeric>;
pub type Street = Alphanumeric;
pub type Number = Alphanumeric;
pub type City = Alphanumeric;
pub type State = Alphabetic;
pub type Zipcode = Numeric;
