use actix_web::{HttpResponse, web};
use uuid::Uuid;

use super::dto::{ClientResponse, RegisterClientRequest, StatusRequest, UpdateClientRequest};
use crate::application::client_service::ConcreteClientService;
use crate::domain::errors::ClientError;
use crate::domain::ports::client_use_cases::{
    ActivateClient, DeactivateClient, DeleteClient, FindClientById, ListClients, RegisterClient,
    UpdateClient,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/clients")
            .route(web::post().to(register_client))
            .route(web::get().to(list_clients)),
    )
    .service(
        web::resource("/clients/{uuid}")
            .route(web::get().to(get_client))
            .route(web::put().to(update_client))
            .route(web::delete().to(delete_client)),
    )
    .service(web::resource("/clients/{uuid}/status").route(web::put().to(update_client_status)));
}

async fn register_client(
    service: web::Data<ConcreteClientService>,
    body: web::Json<RegisterClientRequest>,
) -> HttpResponse {
    let input = crate::domain::ports::client_use_cases::RegisterClientInput {
        name: body.name.clone(),
        doc: body.document.clone(),
        email: body.contact.email.clone(),
        phones: body.contact.phones.clone(),
        cep: body.address.cep.clone(),
        street: body.address.street.clone(),
        number: body.address.number.clone(),
        complement: body.address.complement.clone().unwrap_or_default(),
        neighborhood: body.address.neighborhood.clone(),
        city: body.address.city.clone(),
        state: body.address.state.clone(),
    };

    match RegisterClient::execute(&**service, input).await {
        Ok(row) => HttpResponse::Created().json(ClientResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn list_clients(service: web::Data<ConcreteClientService>) -> HttpResponse {
    match ListClients::execute(&**service).await {
        Ok(rows) => {
            let resp: Vec<ClientResponse> = rows.into_iter().map(ClientResponse::from).collect();
            HttpResponse::Ok().json(resp)
        }
        Err(e) => error_to_response(e),
    }
}

async fn get_client(
    service: web::Data<ConcreteClientService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let uuid = path.into_inner();
    match FindClientById::execute(&**service, uuid).await {
        Ok(row) => HttpResponse::Ok().json(ClientResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn update_client(
    service: web::Data<ConcreteClientService>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateClientRequest>,
) -> HttpResponse {
    let uuid = path.into_inner();
    let input = crate::domain::ports::client_use_cases::UpdateClientInput {
        name: body.name.clone(),
        doc: body.document.clone(),
    };

    match UpdateClient::execute(&**service, uuid, input).await {
        Ok(row) => HttpResponse::Ok().json(ClientResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn delete_client(
    service: web::Data<ConcreteClientService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let uuid = path.into_inner();
    match DeleteClient::execute(&**service, uuid).await {
        Ok(row) => HttpResponse::Ok().json(ClientResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn update_client_status(
    service: web::Data<ConcreteClientService>,
    path: web::Path<Uuid>,
    body: web::Json<StatusRequest>,
) -> HttpResponse {
    let uuid = path.into_inner();
    let result = match body.status.as_str() {
        "active" => ActivateClient::execute(&**service, uuid)
            .await
            .map(ClientResponse::from),
        "inactive" => DeactivateClient::execute(&**service, uuid)
            .await
            .map(ClientResponse::from),
        _ => return HttpResponse::BadRequest().body("invalid status: use 'active' or 'inactive'"),
    };

    match result {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => error_to_response(e),
    }
}

fn error_to_response(err: ClientError) -> HttpResponse {
    use ClientError::*;
    match &err {
        NotFound { .. } => HttpResponse::NotFound().json(serde_json::json!({
            "error": "not_found",
            "message": err.to_string(),
        })),
        AlreadyExists { .. }
        | DocumentAlreadyExists { .. }
        | EmailAlreadyExists { .. }
        | PhoneAlreadyExists { .. } => HttpResponse::Conflict().json(serde_json::json!({
            "error": "conflict",
            "message": err.to_string(),
        })),
        AlreadyActive { .. } | AlreadyInactive { .. } => HttpResponse::BadRequest()
            .json(serde_json::json!({"error": "bad_request", "message": err.to_string()})),
        InvalidDoc(_) | InvalidEmail(_) | InvalidPhone(_) | InvalidCep(_) => {
            HttpResponse::UnprocessableEntity().json(serde_json::json!({
                "error": "validation_error",
                "message": err.to_string(),
            }))
        }
        _ => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "internal_error",
            "message": "internal server error",
        })),
    }
}
