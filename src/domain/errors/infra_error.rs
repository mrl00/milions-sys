#[derive(Debug, thiserror::Error)]
pub enum InfraError {
    #[error("database error while {action}")]
    Database {
        action: &'static str,
        #[source]
        source: sqlx::Error,
    },

    #[error("error starting transaction")]
    BeginTransaction {
        #[source]
        source: sqlx::Error,
    },

    #[error("error committing transaction")]
    CommitTransaction {
        #[source]
        source: sqlx::Error,
    },
}

impl InfraError {
    pub fn db(action: &'static str) -> impl FnOnce(sqlx::Error) -> Self {
        move |source| InfraError::Database { action, source }
    }

    pub fn begin_tx() -> impl FnOnce(sqlx::Error) -> Self {
        |source| InfraError::BeginTransaction { source }
    }

    pub fn commit_tx() -> impl FnOnce(sqlx::Error) -> Self {
        |source| InfraError::CommitTransaction { source }
    }
}
