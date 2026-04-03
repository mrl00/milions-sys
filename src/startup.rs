use crate::routes::health_check::health_check;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use std::io::Error;
use std::net::TcpListener;

pub fn run(tcp_listener: TcpListener, pool: PgPool) -> Result<Server, Error> {
    let client_service = web::Data::new(client::application::client_service::ClientService::new(
        pool.clone(),
    ));
    let collaborator_service = web::Data::new(
        collaborator::application::collaborator_service::CollaboratorService::new(pool.clone()),
    );
    let contact_service = web::Data::new(
        contact::application::contact_service::ContactService::new(pool.clone()),
    );
    let location_service = web::Data::new(
        location::application::location_service::ConcreteLocationService::new(
            location::adapters::driven::postgres::pg_location_repository::PgLocationRepository::new(
                pool.clone(),
            ),
        ),
    );
    let project_service = web::Data::new(
        project::application::project_service::ProjectService::new(pool.clone()),
    );

    let server = HttpServer::new(move || {
        App::new()
            .service(health_check)
            .app_data(client_service.clone())
            .app_data(collaborator_service.clone())
            .app_data(contact_service.clone())
            .app_data(location_service.clone())
            .app_data(project_service.clone())
            .service(
                web::scope("/api")
                    .configure(client::adapters::driving::routes::configure)
                    .configure(collaborator::adapters::driving::routes::configure)
                    .configure(contact::adapters::driving::routes::configure)
                    .configure(location::adapters::driving::routes::configure)
                    .configure(project::adapters::driving::routes::configure),
            )
    })
    .listen(tcp_listener)?
    .run();

    Ok(server)
}
