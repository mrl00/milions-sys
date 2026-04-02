use actix_web::{HttpResponse, web};
use uuid::Uuid;

use super::dto::{
    AddPhoneRequest, ContactResponse, PhoneResponse, RegisterContactRequest,
    UpdateContactEmailRequest, UpdatePhoneRequest,
};
use crate::application::contact_service::ContactService;
use crate::domain::errors::contact_error::ContactError;
use crate::domain::ports::contact_use_cases::{
    AddPhone, FindContact, ListContacts, ListPhones, RegisterContact, RemovePhone,
    UpdateContactEmail, UpdatePhone,
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
    service: web::Data<ContactService>,
    body: web::Json<RegisterContactRequest>,
) -> HttpResponse {
    let input = crate::domain::ports::contact_use_cases::RegisterContactInput {
        email: body.email.clone(),
    };

    match RegisterContact::execute(&**service, input).await {
        Ok(row) => HttpResponse::Created().json(ContactResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn list_contacts(service: web::Data<ContactService>) -> HttpResponse {
    match ListContacts::execute(&**service).await {
        Ok(rows) => {
            let resp: Vec<ContactResponse> =
                rows.into_iter().map(ContactResponse::from).collect();
            HttpResponse::Ok().json(resp)
        }
        Err(e) => error_to_response(e),
    }
}

async fn get_contact(
    service: web::Data<ContactService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let uuid = path.into_inner();
    match FindContact::execute(&**service, uuid).await {
        Ok(row) => HttpResponse::Ok().json(ContactResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn update_contact_email(
    service: web::Data<ContactService>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateContactEmailRequest>,
) -> HttpResponse {
    let uuid = path.into_inner();
    match UpdateContactEmail::execute(&**service, uuid, body.email.clone()).await {
        Ok(row) => HttpResponse::Ok().json(ContactResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn add_phone(
    service: web::Data<ContactService>,
    path: web::Path<Uuid>,
    body: web::Json<AddPhoneRequest>,
) -> HttpResponse {
    let contact_id = path.into_inner();
    match AddPhone::execute(&**service, contact_id, body.phone.clone()).await {
        Ok(row) => HttpResponse::Created().json(PhoneResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn list_phones(
    service: web::Data<ContactService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let contact_id = path.into_inner();
    match ListPhones::execute(&**service, contact_id).await {
        Ok(rows) => {
            let resp: Vec<PhoneResponse> =
                rows.into_iter().map(PhoneResponse::from).collect();
            HttpResponse::Ok().json(resp)
        }
        Err(e) => error_to_response(e),
    }
}

async fn update_phone(
    service: web::Data<ContactService>,
    path: web::Path<Uuid>,
    body: web::Json<UpdatePhoneRequest>,
) -> HttpResponse {
    let uuid = path.into_inner();
    match UpdatePhone::execute(&**service, uuid, body.phone.clone()).await {
        Ok(row) => HttpResponse::Ok().json(PhoneResponse::from(row)),
        Err(e) => error_to_response(e),
    }
}

async fn remove_phone(
    service: web::Data<ContactService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let uuid = path.into_inner();
    match RemovePhone::execute(&**service, uuid).await {
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
        AlreadyExists { .. } | PhoneAlreadyExists { .. } => HttpResponse::Conflict()
            .json(serde_json::json!({
                "error": "conflict",
                "message": err.to_string(),
            })),
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
