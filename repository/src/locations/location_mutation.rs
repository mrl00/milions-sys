use crate::locations::models::location::{CreateLocation, Location, UpdateLocation};
use sqlx::{Executor, Postgres};
use uuid::Uuid;

pub struct LocationMutation;

impl LocationMutation {
    /// Cria uma nova localização na tabela `locations.tb_location`.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `INSERT`.
    /// - **c**: dados para criação (`CreateLocation`), incluindo endereço completo.
    ///
    /// Gera um novo `pk_location` e um hash (`uin_hash`) a partir do conteúdo,
    /// insere o registro e retorna a localização criada.
    pub async fn create<'a, E>(executor: E, c: CreateLocation) -> Result<Location, sqlx::Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        let r = sqlx::query_as!(
            Location,
            r#"INSERT INTO locations.tb_location (
            pk_location, 
            tx_street, 
            tx_number, 
            tx_city, 
            tx_state, 
            tx_zipcode,  
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
            uin_hash
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            RETURNING *"#,
            Uuid::now_v7(),
            &c.tx_street,
            &c.tx_number,
            &c.tx_city,
            &c.tx_state,
            &c.tx_zipcode,
            &c.tx_public_space,
            &c.tx_address_complement,
            &c.tx_unit,
            &c.tx_neighborhood,
            &c.tx_locality,
            &c.tx_region,
            c.tx_ibge,
            c.tx_gia,
            &c.tx_ddd,
            c.tx_siafi,
            c.gen_hash() as i64,
        )
        .fetch_one(executor)
        .await?;

        Ok(r)
    }

    /// Cria várias localizações de uma vez utilizando `UNNEST` para inserção em lote.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `INSERT`.
    /// - **c**: vetor de `CreateLocation` contendo os endereços a serem cadastrados.
    ///
    /// Para cada localização gera um `pk_location` e um `uin_hash`, insere todas em uma única
    /// operação SQL e retorna o vetor de localizações criadas.
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

    /// Atualiza os campos de endereço de uma localização existente.
    ///
    /// - **executor**: executor SQL (`PgPool`, transação, etc.) usado para rodar o `UPDATE`.
    /// - **uuid**: identificador UUID da localização (`pk_location`).
    /// - **c**: dados atualizados (`UpdateLocation`).
    ///
    /// Retorna a localização atualizada; se não houver registro para o UUID informado,
    /// o `unwrap()` atual lançará pânico (comportamento atual da função).
    pub async fn update<'a, E>(
        executor: E,
        uuid: Uuid,
        c: UpdateLocation,
    ) -> Result<Location, sqlx::Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        let r = sqlx::query_as!(
            Location,
            r#"UPDATE locations.tb_location
            SET 
            tx_public_space = $1, 
            tx_address_complement = $2, 
            tx_unit = $3, 
            tx_neighborhood = $4, 
            tx_locality = $5, 
            tx_region = $6, 
            tx_ibge = $7, 
            tx_gia = $8, 
            tx_ddd = $9, 
            tx_siafi = $10, 
            tx_street = $11, 
            tx_number = $12, 
            tx_city = $13, 
            tx_state = $14, 
            tx_zipcode = $15
            WHERE pk_location = $16
            RETURNING *"#,
            c.tx_public_space,
            c.tx_address_complement,
            c.tx_unit,
            c.tx_neighborhood,
            c.tx_locality,
            c.tx_region,
            c.tx_ibge,
            c.tx_gia,
            c.tx_ddd,
            c.tx_siafi,
            c.tx_street,
            c.tx_number,
            c.tx_city,
            c.tx_state,
            c.tx_zipcode,
            uuid,
        )
        .fetch_one(executor)
        .await?;

        Ok(r)
    }
}
