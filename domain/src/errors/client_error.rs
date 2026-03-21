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

impl From<sqlx::Error> for ClientError {
    fn from(source: sqlx::Error) -> Self {
        match source {
            sqlx::Error::RowNotFound => ClientError::NotFound { uuid: Uuid::nil() },
            _ => ClientError::Infra {
                source: InfraError::Database {
                    action: "operação de banco",
                    source,
                },
            },
        }
    }
}
