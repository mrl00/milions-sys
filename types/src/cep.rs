use std::hash::Hash;

#[derive(Debug)]
pub struct Cep(String);

#[derive(Debug, thiserror::Error)]
pub enum CepError {
    #[error("CEP cannot be empty")]
    Empty,

    #[error("CEP '{value}' has invalid length")]
    InvalidLength { value: String },

    #[error("CEP '{value}' is invalid")]
    InvalidCep { value: String },
}

impl TryFrom<String> for Cep {
    type Error = CepError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(CepError::Empty);
        }

        if value.len() != 8 {
            return Err(CepError::InvalidLength { value });
        }

        if !brazilian_utils::cep::is_valid(&value) {
            return Err(CepError::InvalidCep { value });
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
