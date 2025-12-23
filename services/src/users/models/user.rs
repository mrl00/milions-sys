use argon2::{
    Argon2,
    password_hash::{rand_core::OsRng, *},
};
use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct User {
    pub pk_user: Uuid,
    pub idx_user: i64,
    pub tx_name: String,
    pub tx_password_hash: String,
    pub tx_role: String,
    pub fk_contact: Option<Uuid>,
    pub ts_created_at: NaiveDateTime,
    pub ts_updated_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct CreateUser {
    pub tx_name: String,
    pub tx_password_hash: String,
    pub tx_role: String,
    pub fk_contact: Option<Uuid>,
}

impl CreateUser {
    pub fn new(
        tx_name: String,
        tx_password: String,
        tx_role: String,
        fk_contact: Option<Uuid>,
    ) -> Self {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(tx_password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        Self {
            tx_name,
            tx_password_hash: password_hash,
            tx_role,
            fk_contact,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateUser {
    pub tx_name: Option<String>,
    pub tx_password_hash: Option<String>,
    pub tx_role: Option<String>,
    pub fk_contact: Option<Uuid>,
}
