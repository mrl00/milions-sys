use crate::routes::health_check::health_check;
use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use sqlx::PgPool;
use std::io::Error;
use std::net::TcpListener;

pub fn run(tcp_listener: TcpListener, _pool: PgPool) -> Result<Server, Error> {
    let server = HttpServer::new(move || {
        App::new().service(health_check)
        // .app_data(client_service.clone())
        // .app_data(collaborator_service.clone())
        // .app_data(contact_service.clone())
        // .app_data(location_service.clone())
        // .app_data(project_service.clone())
        // .service(
        //     web::scope("/api")
        //         .configure(crate::adapters::driving::client_routes::configure)
        //         .configure(collaborator::adapters::driving::routes::configure)
        //         .configure(contact::adapters::driving::routes::configure)
        //         .configure(location::adapters::driving::routes::configure)
        //         .configure(project::adapters::driving::routes::configure),
        // )
    })
    .listen(tcp_listener)?
    .run();

    Ok(server)
}
