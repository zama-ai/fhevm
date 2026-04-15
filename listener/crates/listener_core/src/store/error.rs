use thiserror::Error;

pub type SqlResult<T> = Result<T, SqlError>;

#[derive(Error, Debug)]
pub enum SqlError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Connection pool error: {0}")]
    Pool(String),
}
