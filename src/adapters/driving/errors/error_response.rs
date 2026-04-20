use actix_web::{HttpResponse, http::StatusCode};
use serde::Serialize;

use crate::domain::errors::location_error::LocationError;

use crate::domain::errors::client_error::ClientError;
use crate::domain::errors::collaborator_error::CollaboratorError;
use crate::domain::errors::contact_error::ContactError;
use crate::domain::errors::project_error::ProjectError;

#[derive(Serialize)]
struct ErrorResponse {
    error: &'static str,
    message: String,
}

impl ErrorResponse {
    fn response(status: StatusCode, error: &'static str, message: impl ToString) -> HttpResponse {
        HttpResponse::build(status).json(Self {
            error,
            message: message.to_string(),
        })
    }
}

impl From<LocationError> for HttpResponse {
    fn from(err: LocationError) -> Self {
        use LocationError::*;
        match err {
            NotFound { uuid } => ErrorResponse::response(
                StatusCode::NOT_FOUND,
                "not_found",
                format!("location not found: {uuid}"),
            ),
            AlreadyExists { hash } => ErrorResponse::response(
                StatusCode::CONFLICT,
                "conflict",
                format!("location already exists: {hash}"),
            ),
            InvalidField { field, reason } => ErrorResponse::response(
                StatusCode::UNPROCESSABLE_ENTITY,
                "validation_error",
                format!("{field}: {reason}"),
            ),
            Infra(_e) => {
                //log::error!("infra error: {e}");
                ErrorResponse::response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "internal server error",
                )
            }
        }
    }
}

impl From<CollaboratorError> for HttpResponse {
    fn from(err: CollaboratorError) -> Self {
        use CollaboratorError::*;
        match err {
            NotFound { uuid } => ErrorResponse::response(
                StatusCode::NOT_FOUND,
                "not_found",
                format!("collaborator not found: {uuid}"),
            ),
            CpfAlreadyExists { cpf } => ErrorResponse::response(
                StatusCode::CONFLICT,
                "conflict",
                format!("CPF '{cpf}' already registered"),
            ),
            AlreadyActive { uuid } => ErrorResponse::response(
                StatusCode::CONFLICT,
                "conflict",
                format!("collaborator is already active: {uuid}"),
            ),
            AlreadyInactive { uuid } => ErrorResponse::response(
                StatusCode::CONFLICT,
                "conflict",
                format!("collaborator is already inactive: {uuid}"),
            ),
            InvalidCpf(e) => {
                ErrorResponse::response(StatusCode::UNPROCESSABLE_ENTITY, "validation_error", e)
            }
            InvalidPhone(e) => {
                ErrorResponse::response(StatusCode::UNPROCESSABLE_ENTITY, "validation_error", e)
            }
            Infra(_e) => {
                //log::error!("infra error: {e}");
                ErrorResponse::response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "internal server error",
                )
            }
        }
    }
}

impl From<ContactError> for HttpResponse {
    fn from(err: ContactError) -> Self {
        use ContactError::*;
        match err {
            NotFound { uuid } => ErrorResponse::response(
                StatusCode::NOT_FOUND,
                "not_found",
                format!("contact not found: {uuid}"),
            ),
            AlreadyExists { email } => ErrorResponse::response(
                StatusCode::CONFLICT,
                "conflict",
                format!("contact with email '{email}' already exists"),
            ),
            PhoneAlreadyExists { phone } => ErrorResponse::response(
                StatusCode::CONFLICT,
                "conflict",
                format!("phone '{phone}' already exists"),
            ),
            PhoneNotFound { uuid } => ErrorResponse::response(
                StatusCode::NOT_FOUND,
                "not_found",
                format!("phone not found: {uuid}"),
            ),
            InvalidPhone(e) => {
                ErrorResponse::response(StatusCode::UNPROCESSABLE_ENTITY, "validation_error", e)
            }
            Infra(_e) => {
                //log::error!("infra error: {e}");
                ErrorResponse::response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "internal server error",
                )
            }
        }
    }
}

impl From<ClientError> for HttpResponse {
    fn from(err: ClientError) -> Self {
        use ClientError::*;
        match err {
            NotFound { uuid } => ErrorResponse::response(
                StatusCode::NOT_FOUND,
                "not_found",
                format!("client not found: {uuid}"),
            ),
            AlreadyExists { name } => ErrorResponse::response(
                StatusCode::CONFLICT,
                "conflict",
                format!("client '{name}' already exists"),
            ),
            AlreadyActive { uuid } => ErrorResponse::response(
                StatusCode::CONFLICT,
                "conflict",
                format!("client is already active: {uuid}"),
            ),
            AlreadyInactive { uuid } => ErrorResponse::response(
                StatusCode::CONFLICT,
                "conflict",
                format!("client is already inactive: {uuid}"),
            ),
            ContactNotFound { uuid } => ErrorResponse::response(
                StatusCode::NOT_FOUND,
                "not_found",
                format!("contact not found: {uuid}"),
            ),
            LocationNotFound { uuid } => ErrorResponse::response(
                StatusCode::NOT_FOUND,
                "not_found",
                format!("address not found: {uuid}"),
            ),
            DocumentAlreadyExists { doc } => ErrorResponse::response(
                StatusCode::CONFLICT,
                "conflict",
                format!("document '{doc}' already registered"),
            ),
            EmailAlreadyExists { email } => ErrorResponse::response(
                StatusCode::CONFLICT,
                "conflict",
                format!("email '{email}' already registered"),
            ),
            PhoneAlreadyExists { phone } => ErrorResponse::response(
                StatusCode::CONFLICT,
                "conflict",
                format!("phone '{phone}' already exists"),
            ),
            InvalidDoc(e) => {
                ErrorResponse::response(StatusCode::UNPROCESSABLE_ENTITY, "validation_error", e)
            }
            InvalidEmail(e) => {
                ErrorResponse::response(StatusCode::UNPROCESSABLE_ENTITY, "validation_error", e)
            }
            InvalidPhone(e) => {
                ErrorResponse::response(StatusCode::UNPROCESSABLE_ENTITY, "validation_error", e)
            }
            InvalidCep(e) => {
                ErrorResponse::response(StatusCode::UNPROCESSABLE_ENTITY, "validation_error", e)
            }
            ViaCep(_e) => {
                //log::error!("viacep error: {e}");
                ErrorResponse::response(
                    StatusCode::BAD_GATEWAY,
                    "external_service_error",
                    "error fetching address data",
                )
            }
            Infra(_e) => {
                //log::error!("infra error: {e}");
                ErrorResponse::response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "internal server error",
                )
            }
        }
    }
}

impl From<ProjectError> for HttpResponse {
    fn from(err: ProjectError) -> Self {
        use ProjectError::*;
        match err {
            NotFound { uuid } => ErrorResponse::response(
                StatusCode::NOT_FOUND,
                "not_found",
                format!("project not found: {uuid}"),
            ),
            AlreadyInStatus { uuid, status } => ErrorResponse::response(
                StatusCode::CONFLICT,
                "conflict",
                format!("project {uuid} is already in status '{status}'"),
            ),
            InvalidTransition { from, to } => ErrorResponse::response(
                StatusCode::CONFLICT,
                "conflict",
                format!("invalid status transition: '{from}' → '{to}'"),
            ),
            StageNotFound { uuid } => ErrorResponse::response(
                StatusCode::NOT_FOUND,
                "not_found",
                format!("stage not found: {uuid}"),
            ),
            AllocationNotFound { uuid } => ErrorResponse::response(
                StatusCode::NOT_FOUND,
                "not_found",
                format!("allocation not found: {uuid}"),
            ),
            CollaboratorNotFound { uuid } => ErrorResponse::response(
                StatusCode::NOT_FOUND,
                "not_found",
                format!("collaborator not found: {uuid}"),
            ),
            InvalidField { field, reason } => ErrorResponse::response(
                StatusCode::UNPROCESSABLE_ENTITY,
                "validation_error",
                format!("{field}: {reason}"),
            ),
            Infra(_e) => {
                //log::error!("infra error: {e}");
                ErrorResponse::response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "internal server error",
                )
            }
        }
    }
}
