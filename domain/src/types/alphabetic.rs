use std::hash::Hash;

use snafu::Snafu;

#[derive(Debug)]
pub struct Alphabetic(String);

#[derive(Debug, Snafu)]
pub enum AlphabeticError {
    #[snafu(display("empty name"))]
    Empty,

    #[snafu(display("invalid name: '{}'", name))]
    InvalidName { name: String },
}

impl TryFrom<String> for Alphabetic {
    type Error = AlphabeticError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return EmptySnafu.fail();
        }

        if !value.chars().all(|c| c.is_ascii_alphabetic()) {
            return InvalidNameSnafu { name: value }.fail();
        }

        Ok(Self(value))
    }
}

impl AsRef<str> for Alphabetic {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsMut<str> for Alphabetic {
    fn as_mut(&mut self) -> &mut str {
        &mut self.0
    }
}

impl Hash for Alphabetic {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
