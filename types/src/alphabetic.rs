use std::hash::Hash;

#[derive(Debug)]
pub struct Alphabetic(String);

#[derive(Debug, thiserror::Error)]
pub enum AlphabeticError {
    #[error("empty name")]
    Empty,

    #[error("invalid name: '{name}'")]
    InvalidName { name: String },
}

impl TryFrom<String> for Alphabetic {
    type Error = AlphabeticError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(AlphabeticError::Empty);
        }

        if !value.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(AlphabeticError::InvalidName { name: value });
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
