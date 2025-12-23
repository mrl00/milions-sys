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
pub struct CreateContactAddress {
    pub fk_contact: Uuid,
    pub addresses: Vec<CreateLocation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateContact {
    pub tx_email: String,
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

impl From<Contact> for ContactNested {
    fn from(c: Contact) -> Self {
        Self {
            pk_contact: c.pk_contact,
            idx_contact: c.idx_contact,
            tx_email: c.tx_email,
            ts_contact_created_at: c.ts_contact_created_at,
            ts_contact_updated_at: c.ts_contact_updated_at,
            phones: vec![],
            addresses: vec![],
        }
    }
}
