use actix_web::{HttpResponse, web};
use uuid::Uuid;

use crate::application::contact_service::PgContactService;
use crate::domain::errors::contact_error::ContactError;
use crate::domain::models::dtos::contact_dto::{
    AddPhoneRequest, ContactResponse, PhoneResponse, RegisterContactRequest,
    UpdateContactEmailRequest, UpdatePhoneRequest,
};
use crate::domain::ports::use_cases::contact_use_cases;
use crate::domain::ports::use_cases::contact_use_cases::{
    AddPhoneUseCase, FindContactUseCase, ListContactsUseCase, ListPhonesUseCase,
    RegisterContactUseCase, RemovePhoneUseCase, UpdateContactEmailUseCase, UpdatePhoneUseCase,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/contacts")
            .route(web::post().to(register_contact))
            .route(web::get().to(list_contacts)),
    )
    .service(
        web::resource("/contacts/{uuid}")
            .route(web::get().to(get_contact))
            .route(web::put().to(update_contact_email)),
    )
    .service(
        web::resource("/contacts/{uuid}/phones")
            .route(web::post().to(add_phone))
            .route(web::get().to(list_phones)),
    )
    .service(
        web::resource("/phones/{uuid}")
            .route(web::put().to(update_phone))
            .route(web::delete().to(remove_phone)),
    );
}

async fn register_contact(
    service: web::Data<PgContactService>,
    body: web::Json<RegisterContactRequest>,
) -> HttpResponse {
    let input = contact_use_cases::RegisterContactInput {
        email: body.email.clone(),
    };

    match RegisterContactUseCase::execute(&**service, input).await {
        Ok(row) => HttpResponse::Created().json(ContactResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn list_contacts(service: web::Data<PgContactService>) -> HttpResponse {
    match ListContactsUseCase::execute(&**service).await {
        Ok(rows) => {
            let resp: Vec<ContactResponse> = rows.into_iter().map(ContactResponse::from).collect();
            HttpResponse::Ok().json(resp)
        }
        Err(e) => error_to_response(e),
    }
}

async fn get_contact(service: web::Data<PgContactService>, path: web::Path<Uuid>) -> HttpResponse {
    let uuid = path.into_inner();
    match FindContactUseCase::execute(&**service, uuid).await {
        Ok(row) => HttpResponse::Ok().json(ContactResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn update_contact_email(
    service: web::Data<PgContactService>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateContactEmailRequest>,
) -> HttpResponse {
    let uuid = path.into_inner();
    match UpdateContactEmailUseCase::execute(&**service, uuid, body.email.clone()).await {
        Ok(row) => HttpResponse::Ok().json(ContactResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn add_phone(
    service: web::Data<PgContactService>,
    path: web::Path<Uuid>,
    body: web::Json<AddPhoneRequest>,
) -> HttpResponse {
    let contact_id = path.into_inner();
    match AddPhoneUseCase::execute(&**service, contact_id, body.phone.clone()).await {
        Ok(row) => HttpResponse::Created().json(PhoneResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn list_phones(service: web::Data<PgContactService>, path: web::Path<Uuid>) -> HttpResponse {
    let contact_id = path.into_inner();
    match ListPhonesUseCase::execute(&**service, contact_id).await {
        Ok(rows) => {
            let resp: Vec<PhoneResponse> = rows.into_iter().map(PhoneResponse::from).collect();
            HttpResponse::Ok().json(resp)
        }
        Err(e) => error_to_response(e),
    }
}

async fn update_phone(
    service: web::Data<PgContactService>,
    path: web::Path<Uuid>,
    body: web::Json<UpdatePhoneRequest>,
) -> HttpResponse {
    let uuid = path.into_inner();
    match UpdatePhoneUseCase::execute(&**service, uuid, body.phone.clone()).await {
        Ok(row) => HttpResponse::Ok().json(PhoneResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn remove_phone(service: web::Data<PgContactService>, path: web::Path<Uuid>) -> HttpResponse {
    let uuid = path.into_inner();
    match RemovePhoneUseCase::execute(&**service, uuid).await {
        Ok(row) => HttpResponse::Ok().json(PhoneResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

fn error_to_response(err: ContactError) -> HttpResponse {
    use ContactError::*;
    match &err {
        NotFound { .. } => HttpResponse::NotFound().json(serde_json::json!({
            "error": "not_found",
            "message": err.to_string(),
        })),
        PhoneNotFound { .. } => HttpResponse::NotFound().json(serde_json::json!({
            "error": "not_found",
            "message": err.to_string(),
        })),
        AlreadyExists { .. } | PhoneAlreadyExists { .. } => {
            HttpResponse::Conflict().json(serde_json::json!({
                "error": "conflict",
                "message": err.to_string(),
            }))
        }
        InvalidPhone(_) => HttpResponse::UnprocessableEntity().json(serde_json::json!({
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
    async fn register_contact_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::post().uri("/api/contacts").to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn list_contacts_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::get().uri("/api/contacts").to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn get_contact_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::get()
            .uri("/api/contacts/01900000-0000-7000-0000-000000000001")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn update_contact_email_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::put()
            .uri("/api/contacts/01900000-0000-7000-0000-000000000001")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn add_phone_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::post()
            .uri("/api/contacts/01900000-0000-7000-0000-000000000001/phones")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn list_phones_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::get()
            .uri("/api/contacts/01900000-0000-7000-0000-000000000001/phones")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn update_phone_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::put()
            .uri("/api/phones/01900000-0000-7000-0000-000000000001")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn remove_phone_route_exists() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(route_config)))
                .await;
        let req = test::TestRequest::delete()
            .uri("/api/phones/01900000-0000-7000-0000-000000000001")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_ne!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn error_to_response_not_found() {
        let err = ContactError::NotFound {
            uuid: Uuid::now_v7(),
        };
        let resp = error_to_response(err);
        assert_eq!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn error_to_response_phone_not_found() {
        let err = ContactError::PhoneNotFound {
            uuid: Uuid::now_v7(),
        };
        let resp = error_to_response(err);
        assert_eq!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn error_to_response_conflict() {
        let err = ContactError::AlreadyExists {
            email: "a@b.com".to_string(),
        };
        let resp = error_to_response(err);
        assert_eq!(resp.status(), 409);
    }

    #[actix_web::test]
    async fn error_to_response_validation_error() {
        let err = ContactError::InvalidPhone(
            crate::domain::value_objects::phone::PhoneError::InvalidPhoneNumber {
                value: "bad".to_string(),
            },
        );
        let resp = error_to_response(err);
        assert_eq!(resp.status(), 422);
    }

    #[actix_web::test]
    async fn error_to_response_internal_error() {
        let err = ContactError::Infra {
            source: crate::domain::errors::infra_error::InfraError::BeginTransaction {
                source: sqlx::Error::PoolTimedOut,
            },
        };
        let resp = error_to_response(err);
        assert_eq!(resp.status(), 500);
    }
}
