use std::hash::Hash;

#[derive(Debug)]
pub struct Email(String);

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("Email cannot be empty")]
    Empty,

    #[error("Email '{value}' is invalid")]
    InvalidEmail { value: String },
}

impl TryFrom<String> for Email {
    type Error = EmailError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(EmailError::Empty);
        }

        if !brazilian_utils::email::is_valid(&value) {
            return Err(EmailError::InvalidEmail { value });
        }

        Ok(Self(value))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsMut<str> for Email {
    fn as_mut(&mut self) -> &mut str {
        &mut self.0
    }
}

impl Hash for Email {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
