use std::hash::Hash;
use std::sync::LazyLock;

use regex::Regex;

static PHONE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\+\d{12,13}$").unwrap());

#[derive(Debug, thiserror::Error)]
pub enum PhoneError {
    #[error("invalid phone: '{value}'")]
    InvalidPhoneNumber { value: String },
}

#[derive(Debug)]
pub struct Phone(String);

impl TryFrom<String> for Phone {
    type Error = PhoneError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !PHONE_RE.is_match(&value) {
            return Err(PhoneError::InvalidPhoneNumber { value });
        }
        Ok(Self(value))
    }
}

impl AsRef<str> for Phone {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl AsMut<str> for Phone {
    fn as_mut(&mut self) -> &mut str {
        self.0.as_mut()
    }
}

impl Hash for Phone {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
