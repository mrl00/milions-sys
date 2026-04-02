use crate::routes::health_check::health_check;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use std::io::Error;
use std::net::TcpListener;

pub fn run(tcp_listener: TcpListener, pool: PgPool) -> Result<Server, Error> {
    let server = HttpServer::new(move || {
        App::new()
            .service(health_check)
            .app_data(web::Data::new(pool.clone()))
    })
    .listen(tcp_listener)?
    .run();

    Ok(server)
}
