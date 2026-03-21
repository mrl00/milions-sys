use snafu::Snafu;
use sqlx::types::Uuid;

use crate::errors::infra_error::InfraError;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum ClientError {
    #[snafu(display("cliente '{name}' já existe"))]
    AlreadyExists { name: String },

    #[snafu(display("cliente não encontrado: {uuid}"))]
    NotFound { uuid: Uuid },

    #[snafu(display("contato não encontrado: {uuid}"))]
    ContactNotFound { uuid: Uuid },

    #[snafu(display("endereço não encontrado: {uuid}"))]
    LocationNotFound { uuid: Uuid },

    #[snafu(transparent)]
    Infra { source: InfraError },
}
