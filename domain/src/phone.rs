use regex::Regex;

pub struct Phone(String);

#[derive(Debug, thiserror::Error)]
pub enum PhoneError {
    InvalidPhoneNumber,
}

impl std::fmt::Display for PhoneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PhoneError::InvalidPhoneNumber => write!(f, "Invalid phone number"),
        }
    }
}

impl Phone {
    pub fn parse(phone: &str) -> Result<Self, PhoneError> {
        let re = Regex::new(r"\+\d{13}\d{1}?").unwrap();
        if phone.len() < 13 || !re.is_match(phone) {
            return Err(PhoneError::InvalidPhoneNumber);
        }
        Ok(Self(phone.to_string()))
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
