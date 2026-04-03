#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum InfraError {
    #[snafu(display("database error while {action}"))]
    Database {
        action: &'static str,
        source: sqlx::Error,
    },

    #[snafu(display("error starting transaction"))]
    BeginTransaction { source: sqlx::Error },

    #[snafu(display("error committing transaction"))]
    CommitTransaction { source: sqlx::Error },
}
