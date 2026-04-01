use std::hash::Hash;

#[derive(Debug)]
pub struct Alphanumeric(String);

#[derive(Debug, thiserror::Error)]
pub enum AlphanumericError {
    #[error("Alphanumeric cannot be empty")]
    Empty,

    #[error("Alphanumeric '{value}' is invalid")]
    InvalidAlphanumeric { value: String },
}

impl TryFrom<String> for Alphanumeric {
    type Error = AlphanumericError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(AlphanumericError::Empty);
        }

        if !value.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(AlphanumericError::InvalidAlphanumeric { value });
        }

        Ok(Self(value))
    }
}

impl AsRef<str> for Alphanumeric {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsMut<str> for Alphanumeric {
    fn as_mut(&mut self) -> &mut str {
        &mut self.0
    }
}

impl Hash for Alphanumeric {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
