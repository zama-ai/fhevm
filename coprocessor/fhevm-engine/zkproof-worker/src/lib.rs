pub mod auxiliary;

#[cfg(test)]
mod tests;

pub mod verifier;
use std::{
    fmt::{self, Display},
    io,
    time::Duration,
};

use fhevm_engine_common::{pg_pool::ServiceError, types::FhevmError, utils::DatabaseURL};
use thiserror::Error;
use tracing_subscriber::registry::Data;

/// The highest index of an input is 254,
/// cause 255 (0xff) is reserved for handles originating from the FHE operations
pub const MAX_INPUT_INDEX: u8 = u8::MAX - 1;

#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),

    #[error("Connection to PostgreSQL is lost")]
    LostDbConnection,

    #[error("Serialization error: {0}")]
    SerializationError(#[from] bincode::Error),

    #[error("IO error: {0}")]
    IOError(#[from] io::Error),

    #[error("Invalid CRS bytes {0}")]
    InvalidCrsBytes(String),

    #[error("Invalid Ciphertext bytes {0}")]
    InvalidCiphertextBytes(String),

    #[error("Invalid Compact Public key bytes {0}")]
    InvalidPkBytes(String),

    #[error("Invalid Proof({0}, {1})")]
    InvalidProof(i64, String),

    #[error("Fhevm error: {0}")]
    FaildFhevm(#[from] FhevmError),

    #[error("Server keys not found {0}")]
    ServerKeysNotFound(String),

    #[error("Invalid auxiliary data {0}")]
    InvalidAuxData(String),

    #[error("JoinError error: {0}")]
    JoinError(#[from] tokio::task::JoinError),

    #[error("Too many inputs: {0}")]
    TooManyInputs(usize),
}

impl From<ExecutionError> for ServiceError {
    fn from(err: ExecutionError) -> Self {
        match err {
            ExecutionError::DbError(e) => ServiceError::Database(e),

            // collapse everything else into InternalError
            other => ServiceError::InternalError(other.to_string()),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Config {
    pub database_url: DatabaseURL,
    pub listen_database_channel: String,
    pub notify_database_channel: String,
    pub pg_pool_connections: u32,
    pub pg_polling_interval: u32,
    pub pg_timeout: Duration,
    pub pg_auto_explain_with_min_duration: Option<Duration>,

    pub worker_thread_count: u32,
}

impl Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Config {{ database_url: {}, listen_database_channel: {}, notify_database_channel: {}, pg_pool_connections: {}, pg_polling_interval: {}, pg_timeout: {:?}, pg_auto_explain_with_min_duration: {:?}, worker_thread_count: {} }}",
            self.database_url,
            self.listen_database_channel,
            self.notify_database_channel,
            self.pg_pool_connections,
            self.pg_polling_interval,
            self.pg_timeout,
            self.pg_auto_explain_with_min_duration,
            self.worker_thread_count
        )
    }
}
