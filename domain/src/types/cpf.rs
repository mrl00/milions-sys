use std::hash::Hash;

use snafu::Snafu;

#[derive(Debug)]
pub struct Cpf(String);

#[derive(Debug, Snafu)]
pub enum CpfError {
    #[snafu(display("CPF cannot be empty"))]
    Empty,

    #[snafu(display("CPF '{value}' has invalid length"))]
    InvalidLength { value: String },

    #[snafu(display("CPF '{value}' is invalid"))]
    InvalidCpf { value: String },
}

impl TryFrom<String> for Cpf {
    type Error = CpfError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return EmptySnafu.fail();
        }

        if value.len() != 11 {
            return InvalidLengthSnafu { value }.fail();
        }

        if !brazilian_utils::cpf::is_valid(&value) {
            return InvalidCpfSnafu { value }.fail();
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
