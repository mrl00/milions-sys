use actix_web::{HttpResponse, web};
use uuid::Uuid;

use super::dto::{CreateLocationRequest, LocationResponse, UpdateLocationRequest};
use crate::application::location_service::ConcreteLocationService;
use crate::domain::errors::LocationError;
use crate::domain::ports::location_use_cases::{
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

fn compute_hash(input: &CreateLocationRequest) -> i64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    input.street.hash(&mut hasher);
    input.number.hash(&mut hasher);
    input.city.hash(&mut hasher);
    input.state.hash(&mut hasher);
    input.zipcode.hash(&mut hasher);
    hasher.finish() as i64
}

async fn create_location(
    service: web::Data<ConcreteLocationService>,
    body: web::Json<CreateLocationRequest>,
) -> HttpResponse {
    let hash = compute_hash(&body);
    let input = crate::domain::ports::location_use_cases::CreateLocationInput {
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
        hash,
    };

    match CreateLocationUseCase::execute(&**service, input).await {
        Ok(row) => HttpResponse::Created().json(LocationResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn list_locations(service: web::Data<ConcreteLocationService>) -> HttpResponse {
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
    service: web::Data<ConcreteLocationService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let uuid = path.into_inner();
    match FindLocationUseCase::execute(&**service, uuid).await {
        Ok(row) => HttpResponse::Ok().json(LocationResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn update_location(
    service: web::Data<ConcreteLocationService>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateLocationRequest>,
) -> HttpResponse {
    let uuid = path.into_inner();
    let input = crate::domain::ports::location_use_cases::UpdateLocationInput {
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
    service: web::Data<ConcreteLocationService>,
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
