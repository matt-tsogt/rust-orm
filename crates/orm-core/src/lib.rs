use std::io::Seek;

use sqlx::{postgres::PgPoolOptions, PgPool, Postgres, Transaction};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrmError {
    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("missing DATABASE_URL environment variable")]
    MissingDatabaseUrl
}

pub type OrmResult<T> = Result<T, OrmError>;

#[derive(Clone)]
pub struct Db {
    pool: PgPool
}

pub struct Tx<'a> {
    inner: Transaction<'a, Postgres>
}


impl Db {
    pub async fn connect(database_url: &str) -> OrmResult<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self{pool})
    }

    pub async fn execute(&self, sql: &str) -> OrmResult<u64> {
        let res = sqlx::query(sql).execute(&self.pool).await?;
        Ok(res.rows_affected())
    }

    pub async fn fetch_i64(&self, sql: &str) -> OrmResult<i64> {
        let val: i64 = sqlx::query_scalar(sql).fetch_one(&self.pool).await?;
        Ok(val)
    }

    pub async fn transaction<F, Fut, T>(&self, f: F) -> OrmResult<T> {
        where
            F: for<'a> Fn

    }
}