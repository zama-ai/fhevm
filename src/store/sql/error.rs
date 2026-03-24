#[derive(Debug, thiserror::Error)]
pub enum SqlError {
    #[error("Data conversion failed for field '{field}' with value '{value}': {message}")]
    DataConversion {
        field: String,
        value: String,
        message: String,
    },

    #[error("SQL execution error: {0}")]
    Execution(#[from] sqlx::Error),

    #[error("Database connection error: {0}")]
    Connection(String),

    #[error("Transaction failed: {0}")]
    Transaction(String),
}

impl SqlError {
    pub fn conversion_error(
        field: &str,
        value: impl std::fmt::Display,
        error: impl std::fmt::Display,
    ) -> Self {
        SqlError::DataConversion {
            field: field.to_string(),
            value: value.to_string(),
            message: error.to_string(),
        }
    }
}

pub type SqlResult<T> = Result<T, SqlError>;
