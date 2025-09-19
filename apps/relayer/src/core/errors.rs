use crate::transaction::fhevm::FhevmError;
use crate::{config::settings::AppConfigError, transaction::TransactionServiceError};
use alloy::primitives::Address;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum EventProcessingError {
    #[error("Failed to decode event: {0}")]
    DecodingError(String),

    #[error("Missing event topic")]
    MissingTopic,

    #[error("Unknown event type for contract {0}")]
    UnknownEvent(Address),

    #[error("Request reverted: {0:?}")]
    RequestReverted(Box<FhevmError>),

    #[error("No handler registered for contract {contract}")]
    UnregisteredContract { contract: Address },

    #[error("Handler failed: {0}")]
    HandlerError(String),

    #[error("Unknown id: {0}")]
    UnknownId(Uuid),

    #[error("Transaction failed: {0}")]
    TransactionError(String),

    #[error("Configuration error: {0}")]
    ConfigError(#[from] AppConfigError),

    #[error("Parsing error: {0}")]
    ParseError(String),

    #[error("Signing error: {0}")]
    SigningError(String),

    #[error("Hex conversion error: {0}")]
    HexError(String),
}

impl From<TransactionServiceError> for EventProcessingError {
    fn from(e: TransactionServiceError) -> Self {
        match e {
            TransactionServiceError::Failed(msg) => {
                Self::TransactionError(format!("Transaction failed: {msg}"))
            }
            TransactionServiceError::Timeout(secs) => {
                Self::TransactionError(format!("Transaction timed out after {secs} seconds"))
            }
            TransactionServiceError::GasEstimation(msg) => {
                Self::TransactionError(format!("Gas estimation failed: {msg}"))
            }
            TransactionServiceError::NonceError(msg) => {
                Self::TransactionError(format!("Nonce error: {msg}"))
            }
            TransactionServiceError::Network(msg) => {
                Self::TransactionError(format!("Network error: {msg}"))
            }
            TransactionServiceError::Config(msg) => {
                Self::HandlerError(format!("Config error: {msg}"))
            }
            TransactionServiceError::Provider(msg) => {
                Self::TransactionError(format!("Provider error: {msg}"))
            }
            TransactionServiceError::Other(err) => Self::TransactionError(err.to_string()),
        }
    }
}
