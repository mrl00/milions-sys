use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

use crate::{
    contacts::models::{
        address::Address,
        phone::{CreatePhone, Phone},
    },
    locations::models::location::CreateLocation,
};

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Contact {
    pub pk_contact: Uuid,
    pub idx_contact: i64,
    pub tx_email: Option<String>,
    pub ts_contact_created_at: NaiveDateTime,
    pub ts_contact_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateContact {
    pub tx_email: String,
    pub phones: Vec<CreatePhone>,
    pub addresses: Vec<CreateLocation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateContact {
    pub tx_email: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreatePartialPhone {
    pub pk_phone: Uuid,
    pub tx_phone: String,
    pub fk_contact: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreatePartialAddress {
    pub pk_address: Uuid,
    pub idx_address: i64,
    pub fk_contact: Option<Uuid>,
}

pub struct ContactNested {
    pub pk_contact: Uuid,
    pub idx_contact: i64,
    pub tx_email: Option<String>,
    pub ts_contact_created_at: NaiveDateTime,
    pub ts_contact_updated_at: NaiveDateTime,
    pub phones: Vec<Phone>,
    pub addresses: Vec<Address>,
}
