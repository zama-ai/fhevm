use crate::{config::settings::AppConfigError, transaction::sender::TransactionError};
use alloy::{primitives::Address, transports::TransportError};
use eyre::Report;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Event processing failed: {0}")]
    EventProcessing(#[from] EventProcessingError),

    #[error("Transport error: {0}")]
    Transport(#[from] alloy::transports::TransportError),

    #[error("Configuration error: {0}")]
    Config(String),
}

#[derive(Error, Debug)]
pub enum EventProcessingError {
    #[error("Failed to decode event: {0}")]
    DecodingError(#[from] alloy_sol_types::Error),

    #[error("Missing event topic")]
    MissingTopic,

    #[error("Unknown event type for contract {0}")]
    UnknownEvent(Address),

    #[error("No handler registered for contract {contract}")]
    UnregisteredContract { contract: Address },

    #[error("Handler failed: {0}")]
    HandlerError(String),

    #[error("Transaction failed: {0}")]
    TransactionError(#[from] eyre::Report),

    #[error("Configuration error: {0}")]
    ConfigError(#[from] AppConfigError),

    #[error("Parsing error: {0}")]
    ParseError(String),

    #[error("Signing error: {0}")]
    SigningError(String),

    #[error("Hex conversion error: {0}")]
    HexError(String),
}

#[derive(Error, Debug)]
pub enum TransactionServiceError {
    #[error("Transaction failed: {0}")]
    Failed(String),

    #[error("Transaction timeout after {0} seconds")]
    Timeout(u64),

    #[error("Gas estimation failed: {0}")]
    GasEstimation(String),

    #[error("Nonce error: {0}")]
    NonceError(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Provider error: {0}")]
    Provider(String),

    #[error(transparent)]
    Other(#[from] eyre::Report),
}

// Implement From for better error conversion
impl From<Error> for TransactionServiceError {
    fn from(err: Error) -> Self {
        match err {
            Error::Transport(e) => Self::Network(e.to_string()),
            Error::Config(msg) => Self::Config(msg),
            Error::EventProcessing(e) => Self::Failed(e.to_string()),
        }
    }
}

impl From<TransactionServiceError> for EventProcessingError {
    fn from(e: TransactionServiceError) -> Self {
        match e {
            TransactionServiceError::Failed(msg) => {
                Self::TransactionError(Report::msg(format!("Transaction failed: {}", msg)))
            }
            TransactionServiceError::Timeout(secs) => Self::TransactionError(Report::msg(format!(
                "Transaction timed out after {} seconds",
                secs
            ))),
            TransactionServiceError::GasEstimation(msg) => {
                Self::TransactionError(Report::msg(format!("Gas estimation failed: {}", msg)))
            }
            TransactionServiceError::NonceError(msg) => {
                Self::TransactionError(Report::msg(format!("Nonce error: {}", msg)))
            }
            TransactionServiceError::Network(msg) => {
                Self::TransactionError(Report::msg(format!("Network error: {}", msg)))
            }
            TransactionServiceError::Config(msg) => {
                Self::HandlerError(format!("Config error: {}", msg))
            }
            TransactionServiceError::Provider(msg) => {
                Self::TransactionError(Report::msg(format!("Provider error: {}", msg)))
            }
            TransactionServiceError::Other(err) => Self::TransactionError(err),
        }
    }
}

impl From<TransactionError> for EventProcessingError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidPrivateKey(msg) => {
                Self::HandlerError(format!("Invalid private key: {}", msg))
            }
            TransactionError::InvalidAddress(msg) => {
                Self::HandlerError(format!("Invalid address: {}", msg))
            }
            TransactionError::RpcError(msg) => {
                Self::TransactionError(Report::msg(format!("RPC error: {}", msg)))
            }
            TransactionError::TransactionFailed(msg) => {
                Self::TransactionError(Report::msg(format!("Transaction failed: {}", msg)))
            }
            TransactionError::TransactionTimeout(secs) => Self::TransactionError(Report::msg(
                format!("Transaction timed out after {} seconds", secs),
            )),
            TransactionError::MonitoringTimeout(secs) => {
                Self::TransactionError(Report::msg(format!(
                    "Transaction monitoring timed out after {} seconds, but may still succeed",
                    secs
                )))
            }
            TransactionError::GasEstimationFailed(msg) => {
                Self::TransactionError(Report::msg(format!("Gas estimation failed: {}", msg)))
            }
            TransactionError::ReceiptNotFound(attempts) => Self::TransactionError(Report::msg(
                format!("Receipt not found after {} attempts", attempts),
            )),
            TransactionError::InsufficientConfirmations { required, actual } => {
                Self::TransactionError(Report::msg(format!(
                    "Insufficient confirmations: required {}, got {}",
                    required, actual
                )))
            }
            TransactionError::NetworkError(msg) => {
                Self::TransactionError(Report::msg(format!("Network error: {}", msg)))
            }
        }
    }
}

impl From<TransactionError> for TransactionServiceError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidPrivateKey(msg) => {
                Self::Failed(format!("Invalid private key: {}", msg))
            }
            TransactionError::InvalidAddress(msg) => {
                Self::Failed(format!("Invalid address: {}", msg))
            }
            TransactionError::RpcError(msg) => Self::Network(msg),
            TransactionError::TransactionFailed(msg) => Self::Failed(msg),
            TransactionError::TransactionTimeout(secs) => Self::Timeout(secs),
            TransactionError::GasEstimationFailed(msg) => Self::GasEstimation(msg),
            TransactionError::MonitoringTimeout(secs) => Self::Timeout(secs), // Transaction may still succeed but monitoring timed out
            TransactionError::ReceiptNotFound(attempts) => {
                Self::Failed(format!("Receipt not found after {} attempts", attempts))
            }
            TransactionError::InsufficientConfirmations { required, actual } => {
                Self::Failed(format!(
                    "Insufficient confirmations: required {}, got {}",
                    required, actual
                ))
            }
            TransactionError::NetworkError(msg) => Self::Network(msg),
        }
    }
}

impl From<TransportError> for TransactionServiceError {
    fn from(err: TransportError) -> Self {
        TransactionServiceError::Network(err.to_string())
    }
}

impl From<String> for TransactionServiceError {
    fn from(err: String) -> Self {
        TransactionServiceError::Failed(err)
    }
}

impl From<&str> for TransactionServiceError {
    fn from(err: &str) -> Self {
        TransactionServiceError::Failed(err.to_string())
    }
}

impl From<eyre::Report> for TransactionError {
    fn from(err: eyre::Report) -> Self {
        TransactionError::RpcError(err.to_string())
    }
}
