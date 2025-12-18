#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub pk_location: String,
    pub idx_location: i64,
    pub street: String,
    pub number: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateLocation {
    pub street: String,
    pub number: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
}
