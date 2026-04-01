mod infra_error;

pub use infra_error::InfraError;

use types::cep::CepError;
use types::doc::DocError;
use types::email::EmailError;
use types::phone::PhoneError;
use uuid::Uuid;
use viacep::domain::ports::viacep_port::ViaCepError;

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("cliente '{name}' já existe")]
    AlreadyExists { name: String },

    #[error("cliente não encontrado: {uuid}")]
    NotFound { uuid: Uuid },

    #[error("cliente já está ativo: {uuid}")]
    AlreadyActive { uuid: Uuid },

    #[error("cliente já está inativo: {uuid}")]
    AlreadyInactive { uuid: Uuid },

    #[error("contato não encontrado: {uuid}")]
    ContactNotFound { uuid: Uuid },

    #[error("endereço não encontrado: {uuid}")]
    LocationNotFound { uuid: Uuid },

    #[error("documento já cadastrado")]
    DocumentAlreadyExists { doc: String },

    #[error("email já cadastrado")]
    EmailAlreadyExists { email: String },

    #[error("telefone '{phone}' já existe")]
    PhoneAlreadyExists { phone: String },

    #[error(transparent)]
    InvalidDoc(#[from] DocError),

    #[error(transparent)]
    InvalidEmail(#[from] EmailError),

    #[error(transparent)]
    InvalidPhone(#[from] PhoneError),

    #[error(transparent)]
    InvalidCep(#[from] CepError),

    #[error(transparent)]
    ViaCep(#[from] ViaCepError),

    #[error(transparent)]
    Infra(#[from] InfraError),
}
