#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum InfraError {
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
