use std::hash::Hash;

use regex::Regex;
use crate::domain::errors::contact_error::PhoneError;

#[derive(Debug)]
pub struct Phone(String);

impl TryFrom<String> for Phone {
    type Error = PhoneError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let re = Regex::new(r"\+\d{13}\d").unwrap();
        if !re.is_match(&value) {
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
