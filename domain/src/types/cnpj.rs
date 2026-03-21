use std::hash::Hash;

use snafu::Snafu;

#[derive(Debug)]
pub struct Cnpj(String);

#[derive(Debug, Snafu)]
pub enum CnpjError {
    #[snafu(display("CNPJ cannot be empty"))]
    Empty,

    #[snafu(display("CNPJ '{value}' has invalid length"))]
    InvalidLength { value: String },

    #[snafu(display("CNPJ '{value}' is invalid"))]
    InvalidCnpj { value: String },
}

impl TryFrom<String> for Cnpj {
    type Error = CnpjError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return EmptySnafu.fail();
        }

        if value.len() != 14 {
            return InvalidLengthSnafu { value }.fail();
        }

        if !brazilian_utils::cnpj::is_valid(&value) {
            return InvalidCnpjSnafu { value }.fail();
        }

        Ok(Self(value))
    }
}

impl AsRef<str> for Cnpj {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsMut<str> for Cnpj {
    fn as_mut(&mut self) -> &mut str {
        &mut self.0
    }
}

impl Hash for Cnpj {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
