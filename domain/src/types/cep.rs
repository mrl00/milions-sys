use std::hash::Hash;

use snafu::Snafu;

#[derive(Debug)]
pub struct Cep(String);

#[derive(Debug, Snafu)]
pub enum CepError {
    #[snafu(display("CEP cannot be empty"))]
    Empty,

    #[snafu(display("CEP '{value}' has invalid length"))]
    InvalidLength { value: String },

    #[snafu(display("CEP '{value}' is invalid"))]
    InvalidCep { value: String },
}

impl TryFrom<String> for Cep {
    type Error = CepError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return EmptySnafu.fail();
        }

        if value.len() != 8 {
            return InvalidLengthSnafu { value }.fail();
        }

        if !brazilian_utils::cep::is_valid(&value) {
            return InvalidCepSnafu { value }.fail();
        }

        Ok(Self(value))
    }
}

impl AsRef<str> for Cep {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsMut<str> for Cep {
    fn as_mut(&mut self) -> &mut str {
        &mut self.0
    }
}

impl Hash for Cep {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
