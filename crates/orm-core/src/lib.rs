use sqlx::{postgres::PgPoolOptions, PgPool, Postgres, Transaction};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrmError {
    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("missing DATABASE_URL environment variable")]
    MissingDatabaseUrl
}