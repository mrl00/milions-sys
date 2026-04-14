use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use std::io::Error;
use std::net::TcpListener;

use crate::application::pg_location_serv_build;
use crate::routes::health_check::health_check;

pub fn run(tcp_listener: TcpListener, pool: PgPool) -> Result<Server, Error> {
    // let client_service = web::Data::new(client::build(pool.clone()));
    // let collaborator_service = web::Data::new(collaborator::build(pool.clone()));
    // let contact_service = web::Data::new(contact::build(pool.clone()));
    let location_service = web::Data::new(pg_location_serv_build(pool.clone()));
    // let project_service = web::Data::new(project::build(pool.clone()));

    let server = HttpServer::new(move || {
        App::new()
            .service(health_check)
            // .app_data(client_service.clone())
            // .app_data(collaborator_service.clone())
            // .app_data(contact_service.clone())
            .app_data(location_service.clone())
            // .app_data(project_service.clone())
            .service(
                web::scope("/api").configure(crate::adapters::driving::location_routes::configure),
            )
    })
    .listen(tcp_listener)?
    .run();

    Ok(server)
}
