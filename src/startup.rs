use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use std::io::Error;
use std::net::TcpListener;

use crate::application::{
    pg_collaborator_serv_build, pg_contact_serv_build, pg_location_serv_build,
};
use crate::routes::health_check::health_check;

fn json_error_handler(
    err: actix_web::error::JsonPayloadError,
    _req: &actix_web::HttpRequest,
) -> actix_web::Error {
    let msg = err.to_string();
    actix_web::error::InternalError::from_response(
        err,
        actix_web::HttpResponse::BadRequest()
            .content_type("application/json")
            .json(serde_json::json!({
                "error": "invalid_body",
                "message": msg
            })),
    )
    .into()
}

fn path_error_handler(
    err: actix_web::error::PathError,
    _req: &actix_web::HttpRequest,
) -> actix_web::Error {
    let msg = err.to_string();
    actix_web::error::InternalError::from_response(
        err,
        actix_web::HttpResponse::BadRequest()
            .content_type("application/json")
            .json(serde_json::json!({
                "error": "invalid_path",
                "message": msg
            })),
    )
    .into()
}

fn query_error_handler(
    err: actix_web::error::QueryPayloadError,
    _req: &actix_web::HttpRequest,
) -> actix_web::Error {
    let msg = err.to_string();
    actix_web::error::InternalError::from_response(
        err,
        actix_web::HttpResponse::BadRequest()
            .content_type("application/json")
            .json(serde_json::json!({
                "error": "invalid_query",
                "message": msg
            })),
    )
    .into()
}

pub fn run(tcp_listener: TcpListener, pool: PgPool) -> Result<Server, Error> {
    // let client_service = web::Data::new(client::build(pool.clone()));
    // let collaborator_service = web::Data::new(collaborator::build(pool.clone()));
    let collaborator_service = web::Data::new(pg_collaborator_serv_build(pool.clone()));
    let contact_service = web::Data::new(pg_contact_serv_build(pool.clone()));
    let location_service = web::Data::new(pg_location_serv_build(pool.clone()));
    // let project_service = web::Data::new(project::build(pool.clone()));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::JsonConfig::default().error_handler(json_error_handler))
            .app_data(web::PathConfig::default().error_handler(path_error_handler))
            .app_data(web::QueryConfig::default().error_handler(query_error_handler))
            .service(health_check)
            // .app_data(client_service.clone())
            .app_data(collaborator_service.clone())
            .app_data(contact_service.clone())
            .app_data(location_service.clone())
            // .app_data(project_service.clone())
            .service(
                web::scope("/api")
                    .configure(crate::adapters::driving::location_routes::configure)
                    .configure(crate::adapters::driving::contact_routes::configure)
                    .configure(crate::adapters::driving::collaborator_routes::configure),
            )
    })
    .listen(tcp_listener)?
    .run();

    Ok(server)
}
