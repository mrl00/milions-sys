use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use sqlx::{PgPool, query_as};
use uuid::Uuid;

use crate::{
    mutation::{Create, Delete, Update},
    users::models::user::User,
};

#[derive(Debug)]
pub struct CreateUser {
    pub tx_name: String,
    pub tx_password_hash: String,
    pub tx_role: String,
}

impl CreateUser {
    pub fn new(tx_name: String, tx_password: String, tx_role: String) -> Self {
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
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateUserWithContact {
    pub tx_name: String,
    pub tx_password_hash: String,
    pub tx_role: String,
    pub fk_contact: Uuid,
}

impl Create for CreateUser {
    type Model = User;

    async fn create(&self, pool: &PgPool) -> Result<Self::Model, sqlx::Error> {
        let user = query_as!(
            User,
            r#"
                    INSERT INTO users.tb_user (pk_user, tx_name, tx_password_hash, tx_role) 
                    VALUES ($1, $2, $3, $4) 
                    RETURNING *
                    "#,
            Uuid::new_v4(),
            &self.tx_name,
            &self.tx_password_hash,
            &self.tx_role
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateUser {
    pub uuid: Uuid,
    pub tx_name: Option<String>,
    pub tx_password_hash: Option<String>,
    pub tx_role: Option<String>,
    pub fk_contact: Option<Uuid>,
}

impl Update for UpdateUser {
    type Model = User;

    async fn update(&self, pool: &PgPool) -> Result<Self::Model, sqlx::Error> {
        let user: User = query_as!(
            User,
            r#"
            UPDATE users.tb_user
            SET tx_name = $1, tx_password_hash = $2, tx_role = $3, fk_contact = $4
            WHERE pk_user = $5
            RETURNING *
            "#,
            self.tx_name,
            self.tx_password_hash,
            self.tx_role,
            self.fk_contact,
            self.uuid,
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}

pub struct DeleteUser {
    pub uuid: Uuid,
}

impl Delete for DeleteUser {
    type Model = User;

    async fn delete(&self, pool: &PgPool) -> Result<Self::Model, sqlx::Error> {
        let user: User = query_as!(
            User,
            r#"
            DELETE FROM users.tb_user
            WHERE pk_user = $1
            RETURNING * 
            "#,
            self.uuid,
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }
}
