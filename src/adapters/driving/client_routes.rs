use actix_web::{HttpResponse, web};
use uuid::Uuid;

use crate::adapters::driving::models::dtos::contact_dto::PhoneResponse;
use crate::adapters::driving::models::dtos::location_dto::LocationResponse;
use crate::adapters::driving::utils::ValidatedJson;

use crate::adapters::driving::models::dtos::client_dto::{
    AddPhonesRequest, ClientResponse, ClientStatusRequest, RegisterClientRequest,
    UpdateClientLocationRequest, UpdateClientRequest, UpdateEmailRequest, UpdatePhoneRequest,
};
use crate::application::client_service::PgClientService;
use crate::domain::ports;
use crate::domain::ports::use_cases::client_use_cases::{
    ActivateClientUseCase, AddClientPhonesUseCase, AssociateClientProjectUseCase,
    DeactivateClientUseCase, DeleteClientUseCase, DissociateClientProjectUseCase,
    FindClientByIdUseCase, ListClientProjectsUseCase, ListClientsUseCase, RegisterClientUseCase,
    UpdateClientEmailUseCase, UpdateClientLocationUseCase, UpdateClientPhoneUseCase,
    UpdateClientUseCase,
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
    .service(web::resource("/clients/{uuid}/status").route(web::put().to(update_client_status)))
    .service(
        web::resource("/clients/{uuid}/contact/email").route(web::patch().to(update_client_email)),
    )
    .service(
        web::resource("/clients/{uuid}/contact/phones").route(web::post().to(add_client_phone)),
    )
    .service(
        web::resource("/clients/{uuid}/contact/phones/{phone}")
            .route(web::put().to(update_client_phone)),
    )
    .service(web::resource("/clients/{uuid}/address").route(web::put().to(update_client_address)))
    .service(
        web::resource("/clients/{uuid}/projects")
            .route(web::post().to(associate_project))
            .route(web::get().to(list_projects)),
    )
    .service(
        web::resource("/clients/{uuid}/projects/{project_uuid}")
            .route(web::delete().to(dissociate_project)),
    );
}

async fn register_client(
    service: web::Data<PgClientService>,
    ValidatedJson(body): ValidatedJson<RegisterClientRequest>,
) -> HttpResponse {
    let input = ports::use_cases::client_use_cases::RegisterClientInput {
        name: body.name.clone(),
        doc: body.document.clone(),
        status: crate::domain::models::db::client_row::ClientStatus::Active,
        location: body.address.as_ref().map(|a| {
            ports::use_cases::client_use_cases::RegisterClientLocationInput {
                street: a.street.clone(),
                number: a.number.clone(),
                city: a.city.clone(),
                state: a.state.clone(),
                zipcode: a.cep.clone(),
                complement: a.complement.clone().unwrap_or_default(),
                public_space: String::new(),
                unit: String::new(),
                neighborhood: a.neighborhood.clone(),
                locality: a.city.clone(),
                region: a.state.clone(),
                ibge: None,
                gia: None,
                ddd: String::new(),
                siafi: None,
            }
        }),
        contact: body.contact.as_ref().map(|c| {
            ports::use_cases::client_use_cases::RegisterClientContactInput {
                email: c.email.clone(),
                phones: c.phones.iter().map(|p| p.value.clone()).collect(),
            }
        }),
    };

    match RegisterClientUseCase::execute(&**service, input).await {
        Ok(row) => HttpResponse::Created().json(ClientResponse::from(row)),
        Err(e) => HttpResponse::from(e),
    }
}

async fn list_clients(service: web::Data<PgClientService>) -> HttpResponse {
    match ListClientsUseCase::execute(&**service).await {
        Ok(rows) => {
            let resp: Vec<ClientResponse> = rows.into_iter().map(ClientResponse::from).collect();
            HttpResponse::Ok().json(resp)
        }
        Err(e) => HttpResponse::from(e),
    }
}

async fn get_client(service: web::Data<PgClientService>, path: web::Path<Uuid>) -> HttpResponse {
    let uuid = path.into_inner();
    match FindClientByIdUseCase::execute(&**service, uuid).await {
        Ok(row) => HttpResponse::Ok().json(ClientResponse::from(row)),
        Err(e) => HttpResponse::from(e),
    }
}

async fn update_client(
    service: web::Data<PgClientService>,
    path: web::Path<Uuid>,
    ValidatedJson(body): ValidatedJson<UpdateClientRequest>,
) -> HttpResponse {
    let uuid = path.into_inner();
    let input = ports::use_cases::client_use_cases::UpdateClientInput {
        name: body.name.clone(),
        doc: body.document.clone(),
    };

    match UpdateClientUseCase::execute(&**service, uuid, input).await {
        Ok(row) => HttpResponse::Ok().json(ClientResponse::from(row)),
        Err(e) => HttpResponse::from(e),
    }
}

async fn delete_client(service: web::Data<PgClientService>, path: web::Path<Uuid>) -> HttpResponse {
    let uuid = path.into_inner();
    match DeleteClientUseCase::execute(&**service, uuid).await {
        Ok(row) => HttpResponse::Ok().json(ClientResponse::from(row)),
        Err(e) => HttpResponse::from(e),
    }
}

async fn update_client_status(
    service: web::Data<PgClientService>,
    path: web::Path<Uuid>,
    ValidatedJson(body): ValidatedJson<ClientStatusRequest>,
) -> HttpResponse {
    let uuid = path.into_inner();
    let result = match body.status.as_str() {
        "active" => ActivateClientUseCase::execute(&**service, uuid)
            .await
            .map(ClientResponse::from),
        "inactive" => DeactivateClientUseCase::execute(&**service, uuid)
            .await
            .map(ClientResponse::from),
        _ => unreachable!("Status is validated to be 'active' or 'inactive'"),
    };

    match result {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::from(e),
    }
}

async fn update_client_email(
    service: web::Data<PgClientService>,
    path: web::Path<Uuid>,
    ValidatedJson(body): ValidatedJson<UpdateEmailRequest>,
) -> HttpResponse {
    let uuid = path.into_inner();
    match UpdateClientEmailUseCase::execute(&**service, uuid, body.email).await {
        Ok(row) => HttpResponse::Ok().json(ClientResponse::from(row)),
        Err(e) => HttpResponse::from(e),
    }
}

async fn add_client_phone(
    service: web::Data<PgClientService>,
    path: web::Path<Uuid>,
    ValidatedJson(body): ValidatedJson<AddPhonesRequest>,
) -> HttpResponse {
    let uuid = path.into_inner();
    match AddClientPhonesUseCase::execute(
        &**service,
        uuid,
        body.phones.iter().map(|x| x.phone.clone()).collect(),
    )
    .await
    {
        Ok(row) => HttpResponse::Ok().json(ClientResponse::from(row)),
        Err(e) => HttpResponse::from(e),
    }
}

async fn update_client_phone(
    service: web::Data<PgClientService>,
    path: web::Path<(Uuid, String)>,
    ValidatedJson(body): ValidatedJson<UpdatePhoneRequest>,
) -> HttpResponse {
    let (uuid, phone) = path.into_inner();
    match UpdateClientPhoneUseCase::execute(&**service, uuid, phone, body.new_phone).await {
        Ok(row) => HttpResponse::Ok().json(PhoneResponse::from(row)),
        Err(e) => HttpResponse::from(e),
    }
}

async fn update_client_address(
    service: web::Data<PgClientService>,
    path: web::Path<Uuid>,
    ValidatedJson(body): ValidatedJson<UpdateClientLocationRequest>,
) -> HttpResponse {
    let uuid = path.into_inner();
    let input = ports::use_cases::client_use_cases::RegisterClientLocationInput {
        street: body.street.clone().unwrap_or_default(),
        number: body.number.clone().unwrap_or_default(),
        city: body.city.clone().unwrap_or_default(),
        state: body.state.clone().unwrap_or_default(),
        zipcode: body.cep.clone().unwrap_or_default(),
        complement: body.complement.clone().unwrap_or_default(),
        public_space: String::new(),
        unit: String::new(),
        neighborhood: body.neighborhood.clone().unwrap_or_default(),
        locality: body.city.clone().unwrap_or_default(),
        region: body.state.clone().unwrap_or_default(),
        ibge: None,
        gia: None,
        ddd: String::new(),
        siafi: None,
    };
    match UpdateClientLocationUseCase::execute(&**service, uuid, input).await {
        Ok(row) => HttpResponse::Ok().json(LocationResponse::from(row)),
        Err(e) => HttpResponse::from(e),
    }
}

// --- Client/Project association ---

#[derive(Debug, serde::Deserialize, garde::Validate)]
struct AssociateProjectRequest {
    #[garde(skip)]
    pub project_id: Uuid,
}

#[derive(Debug, serde::Serialize)]
struct ClientProjectResponse {
    pub id: Uuid,
    pub client_id: Uuid,
    pub project_id: Uuid,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
}

impl From<crate::domain::models::db::client_project_row::ClientProjectRow>
    for ClientProjectResponse
{
    fn from(row: crate::domain::models::db::client_project_row::ClientProjectRow) -> Self {
        Self {
            id: row.pk_client_project,
            client_id: row.fk_client,
            project_id: row.fk_project,
            created_at: row.ts_client_project_created_at,
        }
    }
}

async fn associate_project(
    service: web::Data<PgClientService>,
    path: web::Path<Uuid>,
    ValidatedJson(body): ValidatedJson<AssociateProjectRequest>,
) -> HttpResponse {
    let client_uuid = path.into_inner();
    match AssociateClientProjectUseCase::execute(&**service, client_uuid, body.project_id).await {
        Ok(row) => HttpResponse::Created().json(ClientProjectResponse::from(row)),
        Err(e) => HttpResponse::from(e),
    }
}

async fn list_projects(service: web::Data<PgClientService>, path: web::Path<Uuid>) -> HttpResponse {
    let client_uuid = path.into_inner();
    match ListClientProjectsUseCase::execute(&**service, client_uuid).await {
        Ok(rows) => {
            let resp: Vec<ClientProjectResponse> =
                rows.into_iter().map(ClientProjectResponse::from).collect();
            HttpResponse::Ok().json(resp)
        }
        Err(e) => HttpResponse::from(e),
    }
}

async fn dissociate_project(
    service: web::Data<PgClientService>,
    path: web::Path<(Uuid, Uuid)>,
) -> HttpResponse {
    let (client_uuid, project_uuid) = path.into_inner();
    match DissociateClientProjectUseCase::execute(&**service, client_uuid, project_uuid).await {
        Ok(row) => HttpResponse::Ok().json(ClientProjectResponse::from(row)),
        Err(e) => HttpResponse::from(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::errors::client_error::ClientError;

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
        let resp = HttpResponse::from(err);
        assert_eq!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn error_to_response_conflict() {
        let err = ClientError::DocumentAlreadyExists {
            doc: "123".to_string(),
        };
        let resp = HttpResponse::from(err);
        assert_eq!(resp.status(), 409);
    }

    #[actix_web::test]
    async fn error_to_response_already_active() {
        let err = ClientError::AlreadyActive {
            uuid: Uuid::now_v7(),
        };
        let resp = HttpResponse::from(err);
        assert_eq!(resp.status(), 409);
    }

    #[actix_web::test]
    async fn error_to_response_already_inactive() {
        let err = ClientError::AlreadyInactive {
            uuid: Uuid::now_v7(),
        };
        let resp = HttpResponse::from(err);
        assert_eq!(resp.status(), 409);
    }

    #[actix_web::test]
    async fn error_to_response_validation_error() {
        let err = ClientError::InvalidDocument(
            crate::domain::value_objects::doc::DocError::InvalidDocument,
        );
        let resp = HttpResponse::from(err);
        assert_eq!(resp.status(), 422);
    }

    #[actix_web::test]
    async fn error_to_response_internal_error() {
        let err = ClientError::ViaCep(viacep::domain::ports::viacep_port::ViaCepError::Service(
            "err".to_string(),
        ));
        let resp = HttpResponse::from(err);
        assert_eq!(resp.status(), 502);
    }

    #[actix_web::test]
    async fn update_client_email_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::patch()
            .uri("/api/clients/01900000-0000-7000-0000-000000000001/contact/email")
            .set_json(serde_json::json!({ "email": "new@example.com" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn add_client_phone_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::post()
            .uri("/api/clients/01900000-0000-7000-0000-000000000001/contact/phones")
            .set_json(serde_json::json!({ "phone": "+5561999990001" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn update_client_phone_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::put()
            .uri(
                "/api/clients/01900000-0000-7000-0000-000000000001/contact/phones/%2B5561999990001",
            )
            .set_json(serde_json::json!({ "new_phone": "+5561999990002" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn update_client_address_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::put()
            .uri("/api/clients/01900000-0000-7000-0000-000000000001/address")
            .set_json(serde_json::json!({
                "cep": "01310100",
                "street": "Avenida Paulista",
                "number": "1578",
                "neighborhood": "Bela Vista",
                "city": "São Paulo",
                "state": "SP"
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn associate_project_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::post()
            .uri("/api/clients/01900000-0000-7000-0000-000000000001/projects")
            .set_json(serde_json::json!({ "project_id": "01900000-0000-7000-0000-000000000002" }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn list_client_projects_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::get()
            .uri("/api/clients/01900000-0000-7000-0000-000000000001/projects")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn dissociate_project_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::delete()
            .uri("/api/clients/01900000-0000-7000-0000-000000000001/projects/01900000-0000-7000-0000-000000000002")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }
}
