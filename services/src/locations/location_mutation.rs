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

    pub async fn create_many<'a, E>(
        executor: E,
        c: Vec<CreateLocation>,
    ) -> Result<Vec<Location>, sqlx::Error>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let pks: Vec<Uuid> = c.iter().map(|_| Uuid::now_v7()).collect();
        let tx_public_space: Vec<String> = c.iter().map(|i| i.tx_public_space.clone()).collect();
        let tx_address_complement: Vec<String> =
            c.iter().map(|i| i.tx_address_complement.clone()).collect();
        let tx_unit: Vec<String> = c.iter().map(|i| i.tx_unit.clone()).collect();
        let tx_neighborhood: Vec<String> = c.iter().map(|i| i.tx_neighborhood.clone()).collect();
        let tx_locality: Vec<String> = c.iter().map(|i| i.tx_locality.clone()).collect();
        let tx_region: Vec<String> = c.iter().map(|i| i.tx_region.clone()).collect();
        let tx_ibge: Vec<Option<String>> = c.iter().map(|i| i.tx_ibge.clone()).collect();
        let tx_gia: Vec<Option<String>> = c.iter().map(|i| i.tx_gia.clone()).collect();
        let tx_ddd: Vec<String> = c.iter().map(|i| i.tx_ddd.clone()).collect();
        let tx_siafi: Vec<Option<String>> = c.iter().map(|i| i.tx_siafi.clone()).collect();
        let tx_street: Vec<String> = c.iter().map(|i| i.tx_street.clone()).collect();
        let tx_number: Vec<String> = c.iter().map(|i| i.tx_number.clone()).collect();
        let tx_city: Vec<String> = c.iter().map(|i| i.tx_city.clone()).collect();
        let tx_state: Vec<String> = c.iter().map(|i| i.tx_state.clone()).collect();
        let tx_zipcode: Vec<String> = c.iter().map(|i| i.tx_zipcode.clone()).collect();
        let hashes: Vec<i64> = c.iter().map(|i| i.gen_hash() as i64).collect();

        let r = sqlx::query_as!(
            Location,
            r#"INSERT INTO locations.tb_location (
            pk_location,
            tx_public_space,
            tx_address_complement,
            tx_unit,
            tx_neighborhood,
            tx_locality,
            tx_region,
            tx_ibge,
            tx_gia,
            tx_ddd,
            tx_siafi,
            tx_street,
            tx_number,
            tx_city,
            tx_state,
            tx_zipcode,
            uin_hash
        )
        SELECT * FROM UNNEST(
            $1::uuid[],
            $2::text[],
            $3::text[],
            $4::text[],
            $5::text[],
            $6::text[],
            $7::text[],
            $8::text[],
            $9::text[],
            $10::text[],
            $11::text[],
            $12::text[],
            $13::text[],
            $14::text[],
            $15::text[],
            $16::text[],
            $17::int8[]
        )
        RETURNING *"#,
            &pks as &[Uuid],
            &tx_public_space as &[String],
            &tx_address_complement as &[String],
            &tx_unit as &[String],
            &tx_neighborhood as &[String],
            &tx_locality as &[String],
            &tx_region as &[String],
            &tx_ibge as &[Option<String>],
            &tx_gia as &[Option<String>],
            &tx_ddd as &[String],
            &tx_siafi as &[Option<String>],
            &tx_street as &[String],
            &tx_number as &[String],
            &tx_city as &[String],
            &tx_state as &[String],
            &tx_zipcode as &[String],
            &hashes as &[i64],
        )
        .fetch_all(executor)
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
