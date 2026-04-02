use std::net::TcpListener;

use milions_sys::startup;
use secrecy::ExposeSecret;
use settings::Settings;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Starting Milions");
    let settings: Settings = settings::get_config().expect("Failed to get config");

    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(10)
        .idle_timeout(std::time::Duration::from_secs(5))
        .connect_lazy(settings.database.connection_string().expose_secret())
        .expect("Failed to create pool");

    let address = format!(
        "{}:{}",
        settings.application.host, settings.application.port
    );

    println!("Running on http://{}", address);

    let tcp_listener = TcpListener::bind(address)?;

    startup::run(tcp_listener, pool)?.await
}
