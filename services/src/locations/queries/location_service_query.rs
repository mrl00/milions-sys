use repository::locations::queries::location_query::LocationQuery;
use snafu::ResultExt;

pub struct LocationServiceQuery;

#[derive(Debug, snafu::Snafu)]
pub enum ServiceLocationQueryError {
    #[snafu(display("localização já existe"))]
    AlreadyExists { name: String },

    #[snafu(display("localização não encontrada"))]
    NotFound,

    #[snafu(display("erro de banco ao {action}"))]
    Database {
        action: &'static str,
        source: sqlx::Error,
    },

    #[snafu(display("erro ao iniciar transaction"))]
    BeginTransaction { source: sqlx::Error },

    #[snafu(display("erro ao commitar transaction"))]
    CommitTransaction { source: sqlx::Error },
}

impl From<sqlx::Error> for ServiceLocationQueryError {
    fn from(source: sqlx::Error) -> Self {
        ServiceLocationQueryError::Database {
            action: "obter localização",
            source,
        }
    }
}

impl LocationServiceQuery {
    /// Obtém uma localização pelo seu documento.
    pub async fn check_location_exists(
        pool: &sqlx::PgPool,
        hash: i64,
    ) -> Result<bool, ServiceLocationQueryError> {
        match LocationQuery::find_by_hash(pool, hash)
            .await
            .context(DatabaseSnafu {
                action: "location by hash",
            })? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}
