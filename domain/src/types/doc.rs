use crate::types::{
    cnpj::{Cnpj, CnpjError},
    cpf::{Cpf, CpfError},
};

#[derive(Debug, thiserror::Error)]
pub enum DocError {
    #[error(transparent)]
    DocCpf(#[from] CpfError),

    #[error(transparent)]
    DocCnpj(#[from] CnpjError),

    #[error("documento inválido")]
    InvalidDocument,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Doc {
    Cpf(Cpf),
    Cnpj(Cnpj),
}

impl TryFrom<String> for Doc {
    type Error = DocError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let doc = if value.len() == 11 {
            Doc::Cpf(value.try_into()?)
        } else if value.len() == 14 {
            Doc::Cnpj(value.try_into()?)
        } else {
            return Err(DocError::InvalidDocument);
        };
        Ok(doc)
    }
}
