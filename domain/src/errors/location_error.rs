use crate::errors::infra_error::InfraError;
use snafu::Snafu;
use uuid::Uuid;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum LocationError {
    #[snafu(display("localização já existe: hash '{hash}'"))]
    AlreadyExists { hash: i64 },

    #[snafu(display("localização não encontrada: {uuid}"))]
    NotFound { uuid: Uuid },

    #[snafu(display("campo inválido: {field} — {reason}"))]
    InvalidField { field: &'static str, reason: String },

    #[snafu(transparent)]
    Infra { source: InfraError },
}
