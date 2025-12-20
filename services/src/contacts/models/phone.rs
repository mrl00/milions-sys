use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Phone {
    pub pk_phone: Uuid,
    pub idx_phone: i64,
    pub tx_phone: String,
    pub fk_contact: Uuid,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreatePhone {
    pub pk_phone: Uuid,
    pub tx_phone: String,
    pub fk_contact: Uuid,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdatePhone {
    pub tx_phone: String,
}
