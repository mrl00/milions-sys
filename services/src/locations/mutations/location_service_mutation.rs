use sqlx::{PgPool, error::BoxDynError};

use crate::locations::models::service_create_location::ServiceCreateLocationModel;

#[derive(Debug, snafu::Snafu)]
pub enum LocationError {
    #[snafu(display("localização já existe"))]
    AlreadyExists { name: String },

    #[snafu(display("localização não encontrada"))]
    NotFound,

    #[snafu(display("invalid field: {source}"))]
    InvalidField {
        action: &'static str,
        source: BoxDynError,
    },
}

pub struct LocationServiceMutation;

impl LocationServiceMutation {
    /// Cria uma localização.
    pub async fn create_location(
        _pool: &PgPool,
        _c: ServiceCreateLocationModel,
    ) -> Result<(), LocationError> {
        Ok(())
    }
}
