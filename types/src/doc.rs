use crate::cnpj::{Cnpj, CnpjError};
use crate::cpf::{Cpf, CpfError};

#[derive(Debug, thiserror::Error)]
pub enum DocError {
    #[error(transparent)]
    DocCpf(#[from] CpfError),

    #[error(transparent)]
    DocCnpj(#[from] CnpjError),

    #[error("invalid document")]
    InvalidDocument,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Doc {
    Cpf(Cpf),
    Cnpj(Cnpj),
}

impl Doc {
    pub fn as_str(&self) -> &str {
        match self {
            Doc::Cpf(cpf) => cpf.as_ref(),
            Doc::Cnpj(cnpj) => cnpj.as_ref(),
        }
    }
}

impl std::fmt::Display for Doc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
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
