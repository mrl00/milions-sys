use uuid::Uuid;

use crate::locations::models::location::Location;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Address {
    pub pk_address: Uuid,
    pub idx_address: i64,
    pub fk_contact: Uuid,
}

impl From<Location> for Address {
    fn from(l: Location) -> Self {
        Self {
            pk_address: l.pk_location,
            idx_address: l.idx_location,
            fk_contact: l.pk_location,
        }
    }
}
