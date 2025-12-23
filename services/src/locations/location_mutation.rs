use crate::locations::models::location::{CreateLocation, Location, UpdateLocation};
use sqlx::PgPool;
use uuid::Uuid;

pub struct LocationMutation;

#[derive(Debug)]
pub struct Test {
    tx_street: String,
}

impl LocationMutation {
    pub async fn create(pool: &PgPool, c: CreateLocation) -> Result<Location, sqlx::Error> {
        let r: Location = sqlx::query_as!(
            Location,
            r#"
            INSERT INTO locations.tb_location (pk_location, tx_street, tx_number, tx_city, tx_state, tx_zipcode)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
            Uuid::new_v4(),
            &c.tx_street,
            &c.tx_number,
            &c.tx_city,
            &c.tx_state,
            &c.tx_zipcode,
        )
        .fetch_one(pool)
        .await?;

        Ok(r)
    }

    pub async fn update(
        pool: &PgPool,
        uuid: Uuid,
        c: UpdateLocation,
    ) -> Result<Location, sqlx::Error> {
        let r: Location = sqlx::query_as!(
            Location,
            r#"
            UPDATE locations.tb_location
            SET tx_street = $1, tx_number = $2, tx_city = $3, tx_state = $4, tx_zipcode = $5
            WHERE pk_location = $6
            RETURNING *
            "#,
            c.tx_street,
            c.tx_number,
            c.tx_city,
            c.tx_state,
            c.tx_zipcode,
            &uuid,
        )
        .fetch_one(pool)
        .await?;

        Ok(r)
    }

    pub async fn delete(pool: &PgPool, uuid: Uuid) -> Result<Location, sqlx::Error> {
        let r: Location = sqlx::query_as!(
            Location,
            r#"
            DELETE FROM locations.tb_location
            WHERE pk_location = $1
            RETURNING * 
            "#,
            &uuid,
        )
        .fetch_one(pool)
        .await?;

        Ok(r)
    }
}
