use actix_web::{HttpResponse, web};
use uuid::Uuid;

use crate::adapters::driving::models::dtos::project_dto::{
    AllocationResponse, CostReportResponse, CreateAllocationRequest, CreateProjectRequest,
    CreateStageRequest, HistoryReportResponse, ProgressReportResponse, ProjectResponse,
    ProjectStatusRequest, StageResponse, UpdateAllocationRequest, UpdateProjectRequest,
    UpdateStageRequest,
};
use crate::adapters::driving::utils::ValidatedJson;
use crate::application::project_service::PgProjectService;
use crate::domain::ports;
use crate::domain::ports::use_cases::project_use_cases::{
    CancelProjectUseCase, CompleteProjectUseCase, CreateAllocationUseCase, CreateProjectUseCase,
    CreateStageUseCase, DeleteProjectUseCase, FindProjectUseCase, GetCostReportUseCase,
    GetHistoryReportUseCase, GetProgressReportUseCase, ListAllocationsUseCase, ListProjectsUseCase,
    PauseProjectUseCase, StartProjectUseCase, UpdateAllocationUseCase, UpdateProjectUseCase,
    UpdateStageUseCase,
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

async fn create_project(
    service: web::Data<PgProjectService>,
    ValidatedJson(body): ValidatedJson<CreateProjectRequest>,
) -> HttpResponse {
    let input = ports::use_cases::project_use_cases::CreateProjectInput {
        name: body.name.clone(),
        description: body.description.clone(),
        start_date: body.start_date,
        estimated_end_date: body.estimated_end_date,
        total_area_m2: body.total_area_m2.clone(),
        estimated_cost: body.estimated_cost.clone(),
        notes: body.notes.clone(),
        address_id: body.address_id,
    };

    match CreateProjectUseCase::execute(&**service, input).await {
        Ok(row) => HttpResponse::Created().json(ProjectResponse::from(row)),
        Err(e) => HttpResponse::from(e),
    }
}

async fn list_projects(service: web::Data<PgProjectService>) -> HttpResponse {
    match ListProjectsUseCase::execute(&**service).await {
        Ok(rows) => {
            let resp: Vec<ProjectResponse> = rows.into_iter().map(ProjectResponse::from).collect();
            HttpResponse::Ok().json(resp)
        }
        Err(e) => HttpResponse::from(e),
    }
}

async fn get_project(service: web::Data<PgProjectService>, path: web::Path<Uuid>) -> HttpResponse {
    let uuid = path.into_inner();
    match FindProjectUseCase::execute(&**service, uuid).await {
        Ok(row) => HttpResponse::Ok().json(ProjectResponse::from(row)),
        Err(e) => HttpResponse::from(e),
    }
}

async fn update_project(
    service: web::Data<PgProjectService>,
    path: web::Path<Uuid>,
    ValidatedJson(body): ValidatedJson<UpdateProjectRequest>,
) -> HttpResponse {
    let uuid = path.into_inner();
    let input = ports::use_cases::project_use_cases::UpdateProjectInput {
        name: body.name.clone(),
        description: body.description.clone(),
        start_date: body.start_date,
        estimated_end_date: body.estimated_end_date,
        actual_end_date: body.actual_end_date,
        total_area_m2: body.total_area_m2.clone(),
        estimated_cost: body.estimated_cost.clone(),
        actual_cost: body.actual_cost.clone(),
        notes: body.notes.clone(),
        active: body.active,
    };

    match UpdateProjectUseCase::execute(&**service, uuid, input).await {
        Ok(row) => HttpResponse::Ok().json(ProjectResponse::from(row)),
        Err(e) => HttpResponse::from(e),
    }
}

async fn delete_project(
    service: web::Data<PgProjectService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let uuid = path.into_inner();
    match DeleteProjectUseCase::execute(&**service, uuid).await {
        Ok(row) => HttpResponse::Ok().json(ProjectResponse::from(row)),
        Err(e) => HttpResponse::from(e),
    }
}

async fn update_project_status(
    service: web::Data<PgProjectService>,
    path: web::Path<Uuid>,
    ValidatedJson(body): ValidatedJson<ProjectStatusRequest>,
) -> HttpResponse {
    let uuid = path.into_inner();
    let result = match body.status.as_str() {
        "in_progress" => StartProjectUseCase::execute(&**service, uuid)
            .await
            .map(ProjectResponse::from),
        "paused" => PauseProjectUseCase::execute(&**service, uuid)
            .await
            .map(ProjectResponse::from),
        "completed" => CompleteProjectUseCase::execute(&**service, uuid)
            .await
            .map(ProjectResponse::from),
        "cancelled" => CancelProjectUseCase::execute(&**service, uuid)
            .await
            .map(ProjectResponse::from),
        _ => unreachable!("ValidatedJson garde pattern already rejected invalid status values"),
    };

    match result {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::from(e),
    }
}

async fn create_stage(
    service: web::Data<PgProjectService>,
    path: web::Path<Uuid>,
    ValidatedJson(body): ValidatedJson<CreateStageRequest>,
) -> HttpResponse {
    let project_id = path.into_inner();
    let input = ports::use_cases::project_use_cases::CreateStageInput {
        name: body.name.clone(),
        description: body.description.clone(),
        order: body.order,
        start_date: body.start_date,
        end_date: body.end_date,
    };

    match CreateStageUseCase::execute(&**service, project_id, input).await {
        Ok(row) => HttpResponse::Created().json(StageResponse::from(row)),
        Err(e) => HttpResponse::from(e),
    }
}

async fn update_stage(
    service: web::Data<PgProjectService>,
    path: web::Path<(Uuid, Uuid)>,
    ValidatedJson(body): ValidatedJson<UpdateStageRequest>,
) -> HttpResponse {
    let (project_id, stage_id) = path.into_inner();
    let input = ports::use_cases::project_use_cases::UpdateStageInput {
        name: body.name.clone(),
        description: body.description.clone(),
        order: body.order,
        status: body.status.clone(),
        start_date: body.start_date,
        end_date: body.end_date,
    };

    match UpdateStageUseCase::execute(&**service, project_id, stage_id, input).await {
        Ok(row) => HttpResponse::Ok().json(StageResponse::from(row)),
        Err(e) => HttpResponse::from(e),
    }
}

async fn create_allocation(
    service: web::Data<PgProjectService>,
    path: web::Path<Uuid>,
    ValidatedJson(body): ValidatedJson<CreateAllocationRequest>,
) -> HttpResponse {
    let project_id = path.into_inner();
    let input = ports::use_cases::project_use_cases::CreateAllocationInput {
        collaborator_id: body.collaborator_id,
        work_date: body.work_date,
        hours_worked: body.hours_worked.clone(),
        hourly_rate_snapshot: body.hourly_rate_snapshot.clone(),
        notes: body.notes.clone(),
        present: body.present,
    };

    match CreateAllocationUseCase::execute(&**service, project_id, input).await {
        Ok(row) => HttpResponse::Created().json(AllocationResponse::from(row)),
        Err(e) => HttpResponse::from(e),
    }
}

async fn list_allocations(
    service: web::Data<PgProjectService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let project_id = path.into_inner();
    match ListAllocationsUseCase::execute(&**service, project_id).await {
        Ok(rows) => {
            let resp: Vec<AllocationResponse> =
                rows.into_iter().map(AllocationResponse::from).collect();
            HttpResponse::Ok().json(resp)
        }
        Err(e) => HttpResponse::from(e),
    }
}

async fn update_allocation(
    service: web::Data<PgProjectService>,
    path: web::Path<(Uuid, Uuid)>,
    ValidatedJson(body): ValidatedJson<UpdateAllocationRequest>,
) -> HttpResponse {
    let (project_id, allocation_id) = path.into_inner();
    let input = ports::use_cases::project_use_cases::UpdateAllocationInput {
        hours_worked: body.hours_worked.clone(),
        hourly_rate_snapshot: body.hourly_rate_snapshot.clone(),
        notes: body.notes.clone(),
        present: body.present,
    };

    match UpdateAllocationUseCase::execute(&**service, project_id, allocation_id, input).await {
        Ok(row) => HttpResponse::Ok().json(AllocationResponse::from(row)),
        Err(e) => HttpResponse::from(e),
    }
}

async fn get_cost_report(
    service: web::Data<PgProjectService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let project_id = path.into_inner();
    match GetCostReportUseCase::execute(&**service, project_id).await {
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
        Err(e) => HttpResponse::from(e),
    }
}

async fn get_progress_report(
    service: web::Data<PgProjectService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let project_id = path.into_inner();
    match GetProgressReportUseCase::execute(&**service, project_id).await {
        Ok(report) => HttpResponse::Ok().json(ProgressReportResponse {
            project_id: report.project_id,
            project_name: report.project_name,
            stages: report
                .stages
                .into_iter()
                .map(
                    |s| crate::adapters::driving::models::dtos::project_dto::StageProgress {
                        stage_id: s.pk_project_stage,
                        name: s.tx_name,
                        order: s.nr_order,
                        status: s.tx_status,
                        start_date: s.dt_start_date,
                        end_date: s.dt_end_date,
                    },
                )
                .collect(),
            total_stages: report.total_stages,
            completed_stages: report.completed_stages,
            progress_pct: report.progress_pct.to_string(),
        }),
        Err(e) => HttpResponse::from(e),
    }
}

async fn get_history_report(
    service: web::Data<PgProjectService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let collaborator_id = path.into_inner();
    match GetHistoryReportUseCase::execute(&**service, collaborator_id).await {
        Ok(report) => HttpResponse::Ok().json(HistoryReportResponse {
            collaborator_id: report.collaborator_id,
            collaborator_name: report.collaborator_name,
            allocations: report
                .allocations
                .into_iter()
                .map(
                    |a| crate::adapters::driving::models::dtos::project_dto::AllocationHistory {
                        allocation_id: a.allocation_id,
                        project_id: a.project_id,
                        project_name: a.project_name,
                        work_date: a.work_date,
                        hours_worked: a.hours_worked.map(|v| v.to_string()),
                        hourly_rate_snapshot: a.hourly_rate_snapshot.map(|v| v.to_string()),
                        present: a.present,
                    },
                )
                .collect(),
            total_days: report.total_days,
            total_hours: report.total_hours.to_string(),
        }),
        Err(e) => HttpResponse::from(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::errors::project_error::ProjectError;

    use super::*;
    use actix_web::{App, test, web};
    use uuid::Uuid;

    fn route_config(cfg: &mut web::ServiceConfig) {
        configure(cfg);
    }

    #[actix_web::test]
    async fn create_project_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::post().uri("/api/projects").to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn list_projects_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::get().uri("/api/projects").to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn get_project_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::get()
            .uri("/api/projects/01900000-0000-7000-0000-000000000001")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn update_project_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::put()
            .uri("/api/projects/01900000-0000-7000-0000-000000000001")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn delete_project_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::delete()
            .uri("/api/projects/01900000-0000-7000-0000-000000000001")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn update_project_status_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::put()
            .uri("/api/projects/01900000-0000-7000-0000-000000000001/status")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn create_stage_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::post()
            .uri("/api/projects/01900000-0000-7000-0000-000000000001/stages")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn update_stage_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::put()
            .uri("/api/projects/01900000-0000-7000-0000-000000000001/stages/01900000-0000-7000-0000-000000000002")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn create_allocation_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::post()
            .uri("/api/projects/01900000-0000-7000-0000-000000000001/allocations")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn list_allocations_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::get()
            .uri("/api/projects/01900000-0000-7000-0000-000000000001/allocations")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn update_allocation_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::put()
            .uri("/api/projects/01900000-0000-7000-0000-000000000001/allocations/01900000-0000-7000-0000-000000000002")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn get_cost_report_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::get()
            .uri("/api/reports/projects/01900000-0000-7000-0000-000000000001/cost")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn get_progress_report_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::get()
            .uri("/api/reports/projects/01900000-0000-7000-0000-000000000001/progress")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn get_history_report_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::get()
            .uri("/api/reports/collaborators/01900000-0000-7000-0000-000000000001/history")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn error_to_response_not_found() {
        let err = ProjectError::NotFound {
            uuid: Uuid::now_v7(),
        };
        let resp = HttpResponse::from(err);
        assert_eq!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn error_to_response_already_in_status() {
        let err = ProjectError::AlreadyInStatus {
            uuid: Uuid::now_v7(),
            status: "planning".to_string(),
        };
        let resp = HttpResponse::from(err);
        assert_eq!(resp.status(), 409);
    }

    #[actix_web::test]
    async fn error_to_response_validation_error() {
        let err = ProjectError::InvalidField {
            field: "name",
            reason: "required".to_string(),
        };
        let resp = HttpResponse::from(err);
        assert_eq!(resp.status(), 422);
    }

    #[actix_web::test]
    async fn error_to_response_internal_error() {
        let err = ProjectError::Infra(
            crate::domain::errors::infra_error::InfraError::BeginTransaction {
                source: sqlx::Error::PoolTimedOut,
            },
        );
        let resp = HttpResponse::from(err);
        assert_eq!(resp.status(), 500);
    }
}
