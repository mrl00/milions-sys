use actix_web::{HttpResponse, web};
use sqlx::types::BigDecimal;
use uuid::Uuid;

use super::dto::{
    CreateProjectRequest, ProjectResponse, ProjectStatusRequest, UpdateProjectRequest,
};
use crate::application::project_service::ProjectService;
use crate::domain::errors::ProjectError;
use crate::domain::ports::project_use_cases::{
    CancelProject, CompleteProject, CreateProject, DeleteProject, FindProject, ListProjects,
    PauseProject, StartProject, UpdateProject,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/projects")
            .route(web::post().to(create_project))
            .route(web::get().to(list_projects)),
    )
    .service(
        web::resource("/projects/{uuid}")
            .route(web::get().to(get_project))
            .route(web::put().to(update_project))
            .route(web::delete().to(delete_project)),
    )
    .service(web::resource("/projects/{uuid}/status").route(web::put().to(update_project_status)));
}

fn parse_bd(val: &Option<String>) -> Option<BigDecimal> {
    val.as_deref().and_then(|s| s.parse().ok())
}

async fn create_project(
    service: web::Data<ProjectService>,
    body: web::Json<CreateProjectRequest>,
) -> HttpResponse {
    let input = crate::domain::ports::project_use_cases::CreateProjectInput {
        name: body.name.clone(),
        description: body.description.clone(),
        start_date: body.start_date,
        estimated_end_date: body.estimated_end_date,
        total_area_m2: parse_bd(&body.total_area_m2),
        estimated_cost: parse_bd(&body.estimated_cost),
        notes: body.notes.clone(),
        client_id: body.client_id,
        address_id: body.address_id,
    };

    match CreateProject::execute(&**service, input).await {
        Ok(row) => HttpResponse::Created().json(ProjectResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn list_projects(service: web::Data<ProjectService>) -> HttpResponse {
    match ListProjects::execute(&**service).await {
        Ok(rows) => {
            let resp: Vec<ProjectResponse> = rows.into_iter().map(ProjectResponse::from).collect();
            HttpResponse::Ok().json(resp)
        }
        Err(e) => error_to_response(e),
    }
}

async fn get_project(service: web::Data<ProjectService>, path: web::Path<Uuid>) -> HttpResponse {
    let uuid = path.into_inner();
    match FindProject::execute(&**service, uuid).await {
        Ok(row) => HttpResponse::Ok().json(ProjectResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn update_project(
    service: web::Data<ProjectService>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateProjectRequest>,
) -> HttpResponse {
    let uuid = path.into_inner();
    let input = crate::domain::ports::project_use_cases::UpdateProjectInput {
        name: body.name.clone(),
        description: body.description.clone(),
        start_date: body.start_date,
        estimated_end_date: body.estimated_end_date,
        actual_end_date: body.actual_end_date,
        total_area_m2: parse_bd(&body.total_area_m2),
        estimated_cost: parse_bd(&body.estimated_cost),
        actual_cost: parse_bd(&body.actual_cost),
        notes: body.notes.clone(),
        active: body.active,
    };

    match UpdateProject::execute(&**service, uuid, input).await {
        Ok(row) => HttpResponse::Ok().json(ProjectResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn delete_project(service: web::Data<ProjectService>, path: web::Path<Uuid>) -> HttpResponse {
    let uuid = path.into_inner();
    match DeleteProject::execute(&**service, uuid).await {
        Ok(row) => HttpResponse::Ok().json(ProjectResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn update_project_status(
    service: web::Data<ProjectService>,
    path: web::Path<Uuid>,
    body: web::Json<ProjectStatusRequest>,
) -> HttpResponse {
    let uuid = path.into_inner();
    let result = match body.status.as_str() {
        "in_progress" => StartProject::execute(&**service, uuid)
            .await
            .map(ProjectResponse::from),
        "paused" => PauseProject::execute(&**service, uuid)
            .await
            .map(ProjectResponse::from),
        "completed" => CompleteProject::execute(&**service, uuid)
            .await
            .map(ProjectResponse::from),
        "cancelled" => CancelProject::execute(&**service, uuid)
            .await
            .map(ProjectResponse::from),
        _ => {
            return HttpResponse::BadRequest()
                .body("invalid status: use 'in_progress', 'paused', 'completed', or 'cancelled'");
        }
    };

    match result {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => error_to_response(e),
    }
}

fn error_to_response(err: ProjectError) -> HttpResponse {
    use ProjectError::*;
    match &err {
        NotFound { .. } => HttpResponse::NotFound().json(serde_json::json!({
            "error": "not_found",
            "message": err.to_string(),
        })),
        StageNotFound { .. } => HttpResponse::NotFound().json(serde_json::json!({
            "error": "not_found",
            "message": err.to_string(),
        })),
        AlreadyInStatus { .. } => HttpResponse::Conflict().json(serde_json::json!({
            "error": "conflict",
            "message": err.to_string(),
        })),
        InvalidField { .. } => HttpResponse::UnprocessableEntity().json(serde_json::json!({
            "error": "validation_error",
            "message": err.to_string(),
        })),
        _ => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "internal_error",
            "message": "internal server error",
        })),
    }
}
