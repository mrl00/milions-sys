use crate::errors::infra_error::InfraError;
use snafu::Snafu;
use uuid::Uuid;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum ContactError {
    #[snafu(display("contato com email '{email}' já existe"))]
    AlreadyExists { email: String },

    #[snafu(display("contato não encontrado: {uuid}"))]
    NotFound { uuid: Uuid },

    #[snafu(display("telefone '{phone}' já existe"))]
    PhoneAlreadyExists { phone: String },

    #[snafu(display("telefone não encontrado: {uuid}"))]
    PhoneNotFound { uuid: Uuid },

    #[snafu(transparent)]
    Infra { source: InfraError },
}
