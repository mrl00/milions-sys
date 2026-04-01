use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cnpj(String);

#[derive(Debug, thiserror::Error)]
pub enum CnpjError {
    #[error("CNPJ cannot be empty")]
    Empty,

    #[error("CNPJ '{value}' has invalid length")]
    InvalidLength { value: String },

    #[error("CNPJ '{value}' is invalid")]
    InvalidCnpj { value: String },
}

impl TryFrom<String> for Cnpj {
    type Error = CnpjError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(CnpjError::Empty);
        }

        if value.len() != 14 {
            return Err(CnpjError::InvalidLength { value });
        }

        if !brazilian_utils::cnpj::is_valid(&value) {
            return Err(CnpjError::InvalidCnpj { value });
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
