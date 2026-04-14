use actix_web::{HttpResponse, web};
use uuid::Uuid;

use crate::application::location_service::PgLocationService;
use crate::domain::errors::location_error::LocationError;
use crate::domain::models::dtos::location_dto::{
    CreateLocationRequest, LocationResponse, UpdateLocationRequest,
};
use crate::domain::ports::use_cases::location_use_cases;
use crate::domain::ports::use_cases::location_use_cases::{
    CreateLocationUseCase, DeleteLocationUseCase, FindLocationUseCase, ListLocationsUseCase,
    UpdateLocationUseCase,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/locations")
            .route(web::post().to(create_location))
            .route(web::get().to(list_locations)),
    )
    .service(
        web::resource("/locations/{uuid}")
            .route(web::get().to(get_location))
            .route(web::put().to(update_location))
            .route(web::delete().to(delete_location)),
    );
}

async fn create_location(
    service: web::Data<PgLocationService>,
    body: web::Json<CreateLocationRequest>,
) -> HttpResponse {
    let input = location_use_cases::CreateLocationInput {
        street: body.street.clone(),
        number: body.number.clone(),
        city: body.city.clone(),
        state: body.state.clone(),
        zipcode: body.zipcode.clone(),
        complement: body.complement.clone(),
        public_space: body.public_space.clone(),
        unit: body.unit.clone(),
        neighborhood: body.neighborhood.clone(),
        locality: body.locality.clone(),
        region: body.region.clone(),
        ibge: body.ibge.clone(),
        gia: body.gia.clone(),
        ddd: body.ddd.clone(),
        siafi: body.siafi.clone(),
    };

    match CreateLocationUseCase::execute(&**service, input).await {
        Ok(row) => HttpResponse::Created().json(LocationResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn list_locations(service: web::Data<PgLocationService>) -> HttpResponse {
    match ListLocationsUseCase::execute(&**service).await {
        Ok(rows) => {
            let resp: Vec<LocationResponse> =
                rows.into_iter().map(LocationResponse::from).collect();
            HttpResponse::Ok().json(resp)
        }
        Err(e) => error_to_response(e),
    }
}

async fn get_location(
    service: web::Data<PgLocationService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let uuid = path.into_inner();
    match FindLocationUseCase::execute(&**service, uuid).await {
        Ok(row) => HttpResponse::Ok().json(LocationResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn update_location(
    service: web::Data<PgLocationService>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateLocationRequest>,
) -> HttpResponse {
    let uuid = path.into_inner();
    let input = location_use_cases::UpdateLocationInput {
        street: body.street.clone(),
        number: body.number.clone(),
        city: body.city.clone(),
        state: body.state.clone(),
        zipcode: body.zipcode.clone(),
        complement: body.complement.clone(),
        public_space: body.public_space.clone(),
        unit: body.unit.clone(),
        neighborhood: body.neighborhood.clone(),
        locality: body.locality.clone(),
        region: body.region.clone(),
        ibge: body.ibge.clone(),
        gia: body.gia.clone(),
        ddd: body.ddd.clone(),
        siafi: body.siafi.clone(),
    };

    match UpdateLocationUseCase::execute(&**service, uuid, input).await {
        Ok(row) => HttpResponse::Ok().json(LocationResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn delete_location(
    service: web::Data<PgLocationService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let uuid = path.into_inner();
    match DeleteLocationUseCase::execute(&**service, uuid).await {
        Ok(row) => HttpResponse::Ok().json(LocationResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

fn error_to_response(err: LocationError) -> HttpResponse {
    use LocationError::*;
    match &err {
        NotFound { .. } => HttpResponse::NotFound().json(serde_json::json!({
            "error": "not_found",
            "message": err.to_string(),
        })),
        AlreadyExists { .. } => HttpResponse::Conflict().json(serde_json::json!({
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, test, web};
    use uuid::Uuid;

    fn route_config(cfg: &mut web::ServiceConfig) {
        configure(cfg);
    }

    #[actix_web::test]
    async fn create_location_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::post().uri("/api/locations").to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn list_locations_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::get().uri("/api/locations").to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn get_location_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::get()
            .uri("/api/locations/01900000-0000-7000-0000-000000000001")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn update_location_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::put()
            .uri("/api/locations/01900000-0000-7000-0000-000000000001")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn delete_location_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::delete()
            .uri("/api/locations/01900000-0000-7000-0000-000000000001")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn error_to_response_not_found() {
        let err = LocationError::NotFound {
            uuid: Uuid::now_v7(),
        };
        let resp = error_to_response(err);
        assert_eq!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn error_to_response_conflict() {
        let err = LocationError::AlreadyExists { hash: 123 };
        let resp = error_to_response(err);
        assert_eq!(resp.status(), 409);
    }

    #[actix_web::test]
    async fn error_to_response_validation_error() {
        let err = LocationError::InvalidField {
            field: "street",
            reason: "required".to_string(),
        };
        let resp = error_to_response(err);
        assert_eq!(resp.status(), 422);
    }

    #[actix_web::test]
    async fn error_to_response_internal_error() {
        let err = LocationError::Infra {
            source: crate::domain::errors::infra_error::InfraError::BeginTransaction {
                source: sqlx::Error::PoolTimedOut,
            },
        };
        let resp = error_to_response(err);
        assert_eq!(resp.status(), 500);
    }
}
