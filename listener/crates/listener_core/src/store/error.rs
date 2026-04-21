use thiserror::Error;

pub type SqlResult<T> = Result<T, SqlError>;

#[derive(Error, Debug)]
pub enum SqlError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Connection pool error: {0}")]
    Pool(String),
}

impl SqlError {
    /// Returns `true` if the underlying error is a PostgreSQL unique constraint violation (SQLSTATE 23505).
    pub fn is_unique_violation(&self) -> bool {
        matches!(
            self,
            SqlError::Database(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505")
        )
    }
}
