use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cpf(String);

#[derive(Debug, thiserror::Error)]
pub enum CpfError {
    #[error("CPF cannot be empty")]
    Empty,

    #[error("CPF '{value}' has invalid length")]
    InvalidLength { value: String },

    #[error("CPF '{value}' is invalid")]
    InvalidCpf { value: String },
}

impl TryFrom<String> for Cpf {
    type Error = CpfError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(CpfError::Empty);
        }

        if value.len() != 11 {
            return Err(CpfError::InvalidLength { value });
        }

        if !brazilian_utils::cpf::is_valid(&value) {
            return Err(CpfError::InvalidCpf { value });
        }

        Ok(Self(value))
    }
}

impl AsRef<str> for Cpf {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsMut<str> for Cpf {
    fn as_mut(&mut self) -> &mut str {
        &mut self.0
    }
}

impl Hash for Cpf {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
