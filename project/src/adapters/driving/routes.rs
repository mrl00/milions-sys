use actix_web::{HttpResponse, web};
use sqlx::types::BigDecimal;
use uuid::Uuid;

use super::dto::{
    AllocationResponse, CostReportResponse, CreateAllocationRequest, CreateProjectRequest,
    CreateStageRequest, HistoryReportResponse, ProgressReportResponse, ProjectResponse,
    ProjectStatusRequest, StageResponse, UpdateAllocationRequest, UpdateProjectRequest,
    UpdateStageRequest,
};
use crate::application::project_service::ProjectService;
use crate::domain::errors::ProjectError;
use crate::domain::ports::project_use_cases::{
    CancelProject, CompleteProject, CreateAllocation, CreateProject, CreateStage, DeleteProject,
    FindProject, GetCostReport, GetHistoryReport, GetProgressReport, ListAllocations, ListProjects,
    PauseProject, StartProject, UpdateAllocation, UpdateProject, UpdateStage,
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
    .service(web::resource("/projects/{uuid}/status").route(web::put().to(update_project_status)))
    .service(web::resource("/projects/{project_id}/stages").route(web::post().to(create_stage)))
    .service(
        web::resource("/projects/{project_id}/stages/{stage_id}")
            .route(web::put().to(update_stage)),
    )
    .service(
        web::resource("/projects/{project_id}/allocations")
            .route(web::post().to(create_allocation))
            .route(web::get().to(list_allocations)),
    )
    .service(
        web::resource("/projects/{project_id}/allocations/{allocation_id}")
            .route(web::put().to(update_allocation)),
    )
    .service(
        web::resource("/reports/projects/{project_id}/cost").route(web::get().to(get_cost_report)),
    )
    .service(
        web::resource("/reports/projects/{project_id}/progress")
            .route(web::get().to(get_progress_report)),
    )
    .service(
        web::resource("/reports/collaborators/{collaborator_id}/history")
            .route(web::get().to(get_history_report)),
    );
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
        AllocationNotFound { .. } => HttpResponse::NotFound().json(serde_json::json!({
            "error": "not_found",
            "message": err.to_string(),
        })),
        CollaboratorNotFound { .. } => HttpResponse::NotFound().json(serde_json::json!({
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

async fn create_stage(
    service: web::Data<ProjectService>,
    path: web::Path<Uuid>,
    body: web::Json<CreateStageRequest>,
) -> HttpResponse {
    let project_id = path.into_inner();
    let input = crate::domain::ports::project_use_cases::CreateStageInput {
        name: body.name.clone(),
        description: body.description.clone(),
        order: body.order,
        start_date: body.start_date,
        end_date: body.end_date,
    };

    match CreateStage::execute(&**service, project_id, input).await {
        Ok(row) => HttpResponse::Created().json(StageResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn update_stage(
    service: web::Data<ProjectService>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<UpdateStageRequest>,
) -> HttpResponse {
    let (project_id, stage_id) = path.into_inner();
    let input = crate::domain::ports::project_use_cases::UpdateStageInput {
        name: body.name.clone(),
        description: body.description.clone(),
        order: body.order,
        status: body.status.clone(),
        start_date: body.start_date,
        end_date: body.end_date,
    };

    match UpdateStage::execute(&**service, project_id, stage_id, input).await {
        Ok(row) => HttpResponse::Ok().json(StageResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn create_allocation(
    service: web::Data<ProjectService>,
    path: web::Path<Uuid>,
    body: web::Json<CreateAllocationRequest>,
) -> HttpResponse {
    let project_id = path.into_inner();
    let input = crate::domain::ports::project_use_cases::CreateAllocationInput {
        collaborator_id: body.collaborator_id,
        work_date: body.work_date,
        hours_worked: parse_bd(&body.hours_worked),
        hourly_rate_snapshot: parse_bd(&body.hourly_rate_snapshot),
        notes: body.notes.clone(),
        present: body.present,
    };

    match CreateAllocation::execute(&**service, project_id, input).await {
        Ok(row) => HttpResponse::Created().json(AllocationResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn list_allocations(
    service: web::Data<ProjectService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let project_id = path.into_inner();
    match ListAllocations::execute(&**service, project_id).await {
        Ok(rows) => {
            let resp: Vec<AllocationResponse> =
                rows.into_iter().map(AllocationResponse::from).collect();
            HttpResponse::Ok().json(resp)
        }
        Err(e) => error_to_response(e),
    }
}

async fn update_allocation(
    service: web::Data<ProjectService>,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<UpdateAllocationRequest>,
) -> HttpResponse {
    let (project_id, allocation_id) = path.into_inner();
    let input = crate::domain::ports::project_use_cases::UpdateAllocationInput {
        hours_worked: parse_bd(&body.hours_worked),
        hourly_rate_snapshot: parse_bd(&body.hourly_rate_snapshot),
        notes: body.notes.clone(),
        present: body.present,
    };

    match UpdateAllocation::execute(&**service, project_id, allocation_id, input).await {
        Ok(row) => HttpResponse::Ok().json(AllocationResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn get_cost_report(
    service: web::Data<ProjectService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let project_id = path.into_inner();
    match GetCostReport::execute(&**service, project_id).await {
        Ok(report) => HttpResponse::Ok().json(CostReportResponse {
            project_id: report.project_id,
            project_name: report.project_name,
            estimated_cost: report.estimated_cost.map(|v| v.to_string()),
            actual_cost: report.actual_cost.to_string(),
            variance: report.variance.to_string(),
            variance_pct: report
                .variance_pct
                .map(|v| v.to_string())
                .unwrap_or_default(),
        }),
        Err(e) => error_to_response(e),
    }
}

async fn get_progress_report(
    service: web::Data<ProjectService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let project_id = path.into_inner();
    match GetProgressReport::execute(&**service, project_id).await {
        Ok(report) => HttpResponse::Ok().json(ProgressReportResponse {
            project_id: report.project_id,
            project_name: report.project_name,
            stages: report
                .stages
                .into_iter()
                .map(|s| crate::adapters::driving::dto::StageProgress {
                    stage_id: s.pk_project_stage,
                    name: s.tx_name,
                    order: s.nr_order,
                    status: s.tx_status,
                    start_date: s.dt_start_date,
                    end_date: s.dt_end_date,
                })
                .collect(),
            total_stages: report.total_stages,
            completed_stages: report.completed_stages,
            progress_pct: report.progress_pct.to_string(),
        }),
        Err(e) => error_to_response(e),
    }
}

async fn get_history_report(
    service: web::Data<ProjectService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let collaborator_id = path.into_inner();
    match GetHistoryReport::execute(&**service, collaborator_id).await {
        Ok(report) => HttpResponse::Ok().json(HistoryReportResponse {
            collaborator_id: report.collaborator_id,
            collaborator_name: report.collaborator_name,
            allocations: report
                .allocations
                .into_iter()
                .map(|a| crate::adapters::driving::dto::AllocationHistory {
                    allocation_id: a.allocation_id,
                    project_id: a.project_id,
                    project_name: a.project_name,
                    work_date: a.work_date,
                    hours_worked: a.hours_worked.map(|v| v.to_string()),
                    hourly_rate_snapshot: a.hourly_rate_snapshot.map(|v| v.to_string()),
                    present: a.present,
                })
                .collect(),
            total_days: report.total_days,
            total_hours: report.total_hours.to_string(),
        }),
        Err(e) => error_to_response(e),
    }
}
