use alloy::{sol_types::Error as SolError, transports::TransportError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("ABI decode error: {0}")]
    AbiError(#[from] SolError),

    #[error("Transport error: {0}")]
    TransportError(#[from] TransportError),

    #[error("Event processing failed: {0}")]
    ProcessingError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error("Task error: {0}")]
    TaskError(#[from] tokio::task::JoinError),
}

pub type Result<T> = std::result::Result<T, Error>;
