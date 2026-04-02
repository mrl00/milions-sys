use actix_web::{HttpResponse, web};
use uuid::Uuid;

use super::dto::{
    CollaboratorResponse, RegisterCollaboratorRequest, StatusRequest, UpdateCollaboratorRequest,
};
use crate::application::collaborator_service::CollaboratorService;
use crate::domain::errors::collaborator_error::CollaboratorError;
use crate::domain::ports::collaborator_use_cases::{
    ActivateCollaborator, DeactivateCollaborator, DeleteCollaborator, FindCollaborator,
    ListCollaborators, RegisterCollaborator, UpdateCollaborator,
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
    service: web::Data<CollaboratorService>,
    body: web::Json<RegisterCollaboratorRequest>,
) -> HttpResponse {
    let input = crate::domain::ports::collaborator_use_cases::RegisterCollaboratorInput {
        name: body.name.clone(),
        cpf: body.cpf.clone(),
    };

    match RegisterCollaborator::execute(&**service, input).await {
        Ok(row) => HttpResponse::Created().json(CollaboratorResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn list_collaborators(service: web::Data<CollaboratorService>) -> HttpResponse {
    match ListCollaborators::execute(&**service).await {
        Ok(rows) => {
            let resp: Vec<CollaboratorResponse> =
                rows.into_iter().map(CollaboratorResponse::from).collect();
            HttpResponse::Ok().json(resp)
        }
        Err(e) => error_to_response(e),
    }
}

async fn get_collaborator(
    service: web::Data<CollaboratorService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let uuid = path.into_inner();
    match FindCollaborator::execute(&**service, uuid).await {
        Ok(row) => HttpResponse::Ok().json(CollaboratorResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn update_collaborator(
    service: web::Data<CollaboratorService>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateCollaboratorRequest>,
) -> HttpResponse {
    let uuid = path.into_inner();
    let input = crate::domain::ports::collaborator_use_cases::UpdateCollaboratorInput {
        name: body.name.clone(),
        cpf: body.cpf.clone(),
        level: body.level.clone(),
    };

    match UpdateCollaborator::execute(&**service, uuid, input).await {
        Ok(row) => HttpResponse::Ok().json(CollaboratorResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn delete_collaborator(
    service: web::Data<CollaboratorService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let uuid = path.into_inner();
    match DeleteCollaborator::execute(&**service, uuid).await {
        Ok(row) => HttpResponse::Ok().json(CollaboratorResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn update_collaborator_status(
    service: web::Data<CollaboratorService>,
    path: web::Path<Uuid>,
    body: web::Json<StatusRequest>,
) -> HttpResponse {
    let uuid = path.into_inner();
    let result = match body.status.as_str() {
        "active" => ActivateCollaborator::execute(&**service, uuid)
            .await
            .map(CollaboratorResponse::from),
        "inactive" => DeactivateCollaborator::execute(&**service, uuid)
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
        InvalidCpf(_) | InvalidPhone(_) => HttpResponse::UnprocessableEntity()
            .json(serde_json::json!({
                "error": "validation_error",
                "message": err.to_string(),
            })),
        _ => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "internal_error",
            "message": "internal server error",
        })),
    }
}
