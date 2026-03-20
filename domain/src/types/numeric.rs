use std::hash::Hash;

#[derive(Debug)]
pub struct Numeric(String);

#[derive(Debug, snafu::Snafu)]
pub enum NumericError {
    #[snafu(display("Number cannot be empty"))]
    Empty,

    #[snafu(display("Number '{value}' is invalid"))]
    NotANumber { value: String },
}

impl TryInto<i64> for Numeric {
    type Error = NumericError;

    fn try_into(self) -> Result<i64, Self::Error> {
        match self.0.parse::<i64>() {
            Ok(value) => Ok(value),
            Err(_) => NotANumberSnafu { value: self.0 }.fail(),
        }
    }
}

impl TryFrom<String> for Numeric {
    type Error = NumericError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return EmptySnafu.fail();
        }

        if !value.chars().all(|c| c.is_ascii_digit()) {
            return NotANumberSnafu { value }.fail();
        }

        Ok(Self(value))
    }
}

impl AsRef<str> for Numeric {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsMut<str> for Numeric {
    fn as_mut(&mut self) -> &mut str {
        &mut self.0
    }
}

impl Hash for Numeric {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
