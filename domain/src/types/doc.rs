use sqlx::Either;

use crate::types::{cnpj::Cnpj, cpf::Cpf};

pub type Doc = Either<Cpf, Cnpj>;
