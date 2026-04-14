use actix_web::{HttpResponse, web};
use uuid::Uuid;

use crate::application::collaborator_service::ConcreteCollaboratorService;
use crate::domain::errors::collaborator_error::CollaboratorError;
use crate::domain::models::dtos::collaborator_dto::{
    CollaboratorResponse, RegisterCollaboratorRequest, StatusRequest, UpdateCollaboratorRequest,
};
use crate::domain::ports::use_cases::collaborator_use_cases::{
    ActivateCollaboratorUseCase, DeactivateCollaboratorUseCase, DeleteCollaboratorUseCase,
    FindCollaboratorUseCase, ListCollaboratorsUseCase, RegisterCollaboratorUseCase,
    UpdateCollaboratorUseCase,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/collaborators")
            .route(web::post().to(register_collaborator))
            .route(web::get().to(list_collaborators)),
    )
    .service(
        web::resource("/collaborators/{uuid}")
            .route(web::get().to(get_collaborator))
            .route(web::put().to(update_collaborator))
            .route(web::delete().to(delete_collaborator)),
    )
    .service(
        web::resource("/collaborators/{uuid}/status")
            .route(web::put().to(update_collaborator_status)),
    );
}

async fn register_collaborator(
    service: web::Data<ConcreteCollaboratorService>,
    body: web::Json<RegisterCollaboratorRequest>,
) -> HttpResponse {
    let input =
        crate::domain::ports::use_cases::collaborator_use_cases::RegisterCollaboratorInput {
            name: body.name.clone(),
            cpf: body.cpf.clone(),
        };

    match RegisterCollaboratorUseCase::execute(&**service, input).await {
        Ok(row) => HttpResponse::Created().json(CollaboratorResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn list_collaborators(service: web::Data<ConcreteCollaboratorService>) -> HttpResponse {
    match ListCollaboratorsUseCase::execute(&**service).await {
        Ok(rows) => {
            let resp: Vec<CollaboratorResponse> =
                rows.into_iter().map(CollaboratorResponse::from).collect();
            HttpResponse::Ok().json(resp)
        }
        Err(e) => error_to_response(e),
    }
}

async fn get_collaborator(
    service: web::Data<ConcreteCollaboratorService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let uuid = path.into_inner();
    match FindCollaboratorUseCase::execute(&**service, uuid).await {
        Ok(row) => HttpResponse::Ok().json(CollaboratorResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn update_collaborator(
    service: web::Data<ConcreteCollaboratorService>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateCollaboratorRequest>,
) -> HttpResponse {
    let uuid = path.into_inner();
    let input = crate::domain::ports::use_cases::collaborator_use_cases::UpdateCollaboratorInput {
        name: body.name.clone(),
        cpf: body.cpf.clone(),
        level: body.level.clone(),
    };

    match UpdateCollaboratorUseCase::execute(&**service, uuid, input).await {
        Ok(row) => HttpResponse::Ok().json(CollaboratorResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn delete_collaborator(
    service: web::Data<ConcreteCollaboratorService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let uuid = path.into_inner();
    match DeleteCollaboratorUseCase::execute(&**service, uuid).await {
        Ok(row) => HttpResponse::Ok().json(CollaboratorResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn update_collaborator_status(
    service: web::Data<ConcreteCollaboratorService>,
    path: web::Path<Uuid>,
    body: web::Json<StatusRequest>,
) -> HttpResponse {
    let uuid = path.into_inner();
    let result = match body.status.as_str() {
        "active" => ActivateCollaboratorUseCase::execute(&**service, uuid)
            .await
            .map(CollaboratorResponse::from),
        "inactive" => DeactivateCollaboratorUseCase::execute(&**service, uuid)
            .await
            .map(CollaboratorResponse::from),
        _ => return HttpResponse::BadRequest().body("invalid status: use 'active' or 'inactive'"),
    };

    match result {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => error_to_response(e),
    }
}

fn error_to_response(err: CollaboratorError) -> HttpResponse {
    use CollaboratorError::*;
    match &err {
        NotFound { .. } => HttpResponse::NotFound().json(serde_json::json!({
            "error": "not_found",
            "message": err.to_string(),
        })),
        CpfAlreadyExists { .. } => HttpResponse::Conflict().json(serde_json::json!({
            "error": "conflict",
            "message": err.to_string(),
        })),
        AlreadyActive { .. } | AlreadyInactive { .. } => HttpResponse::BadRequest()
            .json(serde_json::json!({"error": "bad_request", "message": err.to_string()})),
        InvalidCpf(_) | InvalidPhone(_) => {
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
    async fn register_collaborator_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::post()
            .uri("/api/collaborators")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn list_collaborators_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::get()
            .uri("/api/collaborators")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn get_collaborator_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::get()
            .uri("/api/collaborators/01900000-0000-7000-0000-000000000001")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn update_collaborator_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::put()
            .uri("/api/collaborators/01900000-0000-7000-0000-000000000001")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn delete_collaborator_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::delete()
            .uri("/api/collaborators/01900000-0000-7000-0000-000000000001")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn update_collaborator_status_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::put()
            .uri("/api/collaborators/01900000-0000-7000-0000-000000000001/status")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn error_to_response_not_found() {
        let err = CollaboratorError::NotFound {
            uuid: Uuid::now_v7(),
        };
        let resp = error_to_response(err);
        assert_eq!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn error_to_response_conflict() {
        let err = CollaboratorError::CpfAlreadyExists {
            cpf: "123".to_string(),
        };
        let resp = error_to_response(err);
        assert_eq!(resp.status(), 409);
    }

    #[actix_web::test]
    async fn error_to_response_already_active() {
        let err = CollaboratorError::AlreadyActive {
            uuid: Uuid::now_v7(),
        };
        let resp = error_to_response(err);
        assert_eq!(resp.status(), 400);
    }

    #[actix_web::test]
    async fn error_to_response_validation_error() {
        let err = CollaboratorError::InvalidCpf(crate::domain::value_objects::cpf::CpfError::Empty);
        let resp = error_to_response(err);
        assert_eq!(resp.status(), 422);
    }

    #[actix_web::test]
    async fn error_to_response_internal_error() {
        let err = CollaboratorError::Infra {
            source: crate::domain::errors::infra_error::InfraError::BeginTransaction {
                source: sqlx::Error::PoolTimedOut,
            },
        };
        let resp = error_to_response(err);
        assert_eq!(resp.status(), 500);
    }
}
