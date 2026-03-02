use crate::locations::models::location::{CreateLocation, Location, UpdateLocation};
use uuid::Uuid;

pub struct LocationMutation;

#[derive(Debug)]
pub struct Test {
    tx_street: String,
}

impl LocationMutation {
    pub async fn create<'a, E>(executor: E, c: CreateLocation) -> Result<Location, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
    {
        let r: Location = sqlx::query_as!(
            Location,
            r#"
            INSERT INTO locations.tb_location (pk_location, tx_street, tx_number, tx_city, tx_state, tx_zipcode)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
            Uuid::now_v7(),
            &c.tx_street,
            &c.tx_number,
            &c.tx_city,
            &c.tx_state,
            &c.tx_zipcode,
        )
        .fetch_one(executor)
        .await?;

        Ok(r)
    }

    pub async fn update<'a, E>(
        executor: E,
        uuid: Uuid,
        c: UpdateLocation,
    ) -> Result<Location, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres> + std::marker::Copy,
    {
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
        .fetch_one(executor)
        .await?;

        Ok(r)
    }
}
