use services::locations::{location_mutation::LocationMutation, models::location::CreateLocation};

#[tokio::main]
async fn main() {
    let locations = vec![
        CreateLocation {
            tx_public_space: "Praça da Sé".to_string(),
            tx_address_complement: "lado ímpar".to_string(),
            tx_unit: "".to_string(),
            tx_neighborhood: "Sé".to_string(),
            tx_locality: "São Paulo".to_string(),
            tx_region: "Sudeste".to_string(),
            tx_ibge: Some("3550308".to_string()),
            tx_gia: Some("1004".to_string()),
            tx_ddd: "11".to_string(),
            tx_siafi: Some("7107".to_string()),
            tx_street: "Praça da Sé".to_string(),
            tx_number: "S/N".to_string(),
            tx_city: "São Paulo".to_string(),
            tx_state: "SP".to_string(),
            tx_zipcode: "01001-000".to_string(),
        },
        CreateLocation {
            tx_public_space: "Avenida Paulista".to_string(),
            tx_address_complement: "conjunto 51".to_string(),
            tx_unit: "Bloco B".to_string(),
            tx_neighborhood: "Bela Vista".to_string(),
            tx_locality: "São Paulo".to_string(),
            tx_region: "Sudeste".to_string(),
            tx_ibge: Some("3550308".to_string()),
            tx_gia: Some("1004".to_string()),
            tx_ddd: "11".to_string(),
            tx_siafi: Some("7107".to_string()),
            tx_street: "Avenida Paulista".to_string(),
            tx_number: "1578".to_string(),
            tx_city: "São Paulo".to_string(),
            tx_state: "SP".to_string(),
            tx_zipcode: "01310-200".to_string(),
        },
        CreateLocation {
            tx_public_space: "Rua das Flores".to_string(),
            tx_address_complement: "apto 202".to_string(),
            tx_unit: "Torre 1".to_string(),
            tx_neighborhood: "Centro".to_string(),
            tx_locality: "Curitiba".to_string(),
            tx_region: "Sul".to_string(),
            tx_ibge: Some("4106902".to_string()),
            tx_gia: Some("0".to_string()),
            tx_ddd: "41".to_string(),
            tx_siafi: Some("7535".to_string()),
            tx_street: "Rua das Flores".to_string(),
            tx_number: "300".to_string(),
            tx_city: "Curitiba".to_string(),
            tx_state: "PR".to_string(),
            tx_zipcode: "80010-000".to_string(),
        },
        CreateLocation {
            tx_public_space: "Rua XV de Novembro".to_string(),
            tx_address_complement: "".to_string(),
            tx_unit: "".to_string(),
            tx_neighborhood: "Centro".to_string(),
            tx_locality: "Florianópolis".to_string(),
            tx_region: "Sul".to_string(),
            tx_ibge: Some("4205407".to_string()),
            tx_gia: Some("0".to_string()),
            tx_ddd: "48".to_string(),
            tx_siafi: Some("8105".to_string()),
            tx_street: "Rua XV de Novembro".to_string(),
            tx_number: "100".to_string(),
            tx_city: "Florianópolis".to_string(),
            tx_state: "SC".to_string(),
            tx_zipcode: "88010-400".to_string(),
        },
        CreateLocation {
            tx_public_space: "Avenida Rio Branco".to_string(),
            tx_address_complement: "sala 301".to_string(),
            tx_unit: "".to_string(),
            tx_neighborhood: "Centro".to_string(),
            tx_locality: "Rio de Janeiro".to_string(),
            tx_region: "Sudeste".to_string(),
            tx_ibge: Some("3304557".to_string()),
            tx_gia: Some("0".to_string()),
            tx_ddd: "21".to_string(),
            tx_siafi: Some("6001".to_string()),
            tx_street: "Avenida Rio Branco".to_string(),
            tx_number: "45".to_string(),
            tx_city: "Rio de Janeiro".to_string(),
            tx_state: "RJ".to_string(),
            tx_zipcode: "20040-004".to_string(),
        },
    ];

    let cfg = config::get_config().expect("Failed to get config");
    let db = cfg.database.with_db();
    let p = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect_with(db)
        .await
        .expect("Failed to connect to database");

    let mut tx = p.begin().await.unwrap();

    let created_location = LocationMutation::create_many(&mut *tx, locations).await;

    tx.commit().await.unwrap();

    match created_location {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{:?}", e),
    }
}
