use actix_web::{HttpResponse, web};
use uuid::Uuid;

use crate::application::client_service::ConcreteClientService;
use crate::domain::errors::client_error::ClientError;
use crate::domain::models::dtos::client_dto::{
    ClientResponse, RegisterClientRequest, StatusRequest, UpdateClientRequest,
};
use crate::domain::ports;
use crate::domain::ports::use_cases::client_use_cases::{
    ActivateClientUseCase, DeactivateClientUseCase, DeleteClientUseCase, FindClientByIdUseCase,
    ListClientsUseCase, RegisterClientUseCase, UpdateClientUseCase,
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
    let input = ports::use_cases::client_use_cases::RegisterClientInput {
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

    match RegisterClientUseCase::execute(&**service, input).await {
        Ok(row) => HttpResponse::Created().json(ClientResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn list_clients(service: web::Data<ConcreteClientService>) -> HttpResponse {
    match ListClientsUseCase::execute(&**service).await {
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
    match FindClientByIdUseCase::execute(&**service, uuid).await {
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
    let input = ports::use_cases::client_use_cases::UpdateClientInput {
        name: body.name.clone(),
        doc: body.document.clone(),
    };

    match UpdateClientUseCase::execute(&**service, uuid, input).await {
        Ok(row) => HttpResponse::Ok().json(ClientResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn delete_client(
    service: web::Data<ConcreteClientService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let uuid = path.into_inner();
    match DeleteClientUseCase::execute(&**service, uuid).await {
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
        "active" => ActivateClientUseCase::execute(&**service, uuid)
            .await
            .map(ClientResponse::from),
        "inactive" => DeactivateClientUseCase::execute(&**service, uuid)
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, test, web};
    use uuid::Uuid;

    fn route_config(cfg: &mut web::ServiceConfig) {
        configure(cfg);
    }

    #[actix_web::test]
    async fn register_client_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::post()
            .uri("/api/clients")
            .set_json(serde_json::json!({
                "name": "test",
                "document": "12345678909",
                "contact": { "email": "a@b.com", "phones": [] },
                "address": { "cep": "00000000", "number": "1", "street": "X", "city": "X", "state": "SP", "complement": null, "neighborhood": "X" }
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn list_clients_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::get().uri("/api/clients").to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn get_client_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::get()
            .uri("/api/clients/01900000-0000-7000-0000-000000000001")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn update_client_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::put()
            .uri("/api/clients/01900000-0000-7000-0000-000000000001")
            .set_json(serde_json::json!({ "name": "x" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn delete_client_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::delete()
            .uri("/api/clients/01900000-0000-7000-0000-000000000001")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn update_client_status_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::put()
            .uri("/api/clients/01900000-0000-7000-0000-000000000001/status")
            .set_json(serde_json::json!({ "status": "active" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn error_to_response_not_found() {
        let err = ClientError::NotFound {
            uuid: Uuid::now_v7(),
        };
        let resp = error_to_response(err);
        assert_eq!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn error_to_response_conflict() {
        let err = ClientError::DocumentAlreadyExists {
            doc: "123".to_string(),
        };
        let resp = error_to_response(err);
        assert_eq!(resp.status(), 409);
    }

    #[actix_web::test]
    async fn error_to_response_already_active() {
        let err = ClientError::AlreadyActive {
            uuid: Uuid::now_v7(),
        };
        let resp = error_to_response(err);
        assert_eq!(resp.status(), 400);
    }

    #[actix_web::test]
    async fn error_to_response_already_inactive() {
        let err = ClientError::AlreadyInactive {
            uuid: Uuid::now_v7(),
        };
        let resp = error_to_response(err);
        assert_eq!(resp.status(), 400);
    }

    #[actix_web::test]
    async fn error_to_response_validation_error() {
        let err =
            ClientError::InvalidDoc(crate::domain::value_objects::doc::DocError::InvalidDocument);
        let resp = error_to_response(err);
        assert_eq!(resp.status(), 422);
    }

    #[actix_web::test]
    async fn error_to_response_internal_error() {
        let err = ClientError::ViaCep(viacep::domain::ports::viacep_port::ViaCepError::Service(
            "err".to_string(),
        ));
        let resp = error_to_response(err);
        assert_eq!(resp.status(), 500);
    }
}
