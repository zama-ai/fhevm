use crate::blockchain::fhevm::ethereum::transaction::fhevm::FhevmError;
use crate::blockchain::gateway::arbitrum::transaction::engine::GatewayTxnError;
use crate::{
    blockchain::fhevm::ethereum::transaction::TransactionServiceError as FhevmTransactionServiceError,
    config::settings::AppConfigError,
};
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

impl From<FhevmTransactionServiceError> for EventProcessingError {
    fn from(e: FhevmTransactionServiceError) -> Self {
        match e {
            FhevmTransactionServiceError::Failed(msg) => {
                Self::TransactionError(format!("Transaction failed: {msg}"))
            }
            FhevmTransactionServiceError::Timeout(secs) => {
                Self::TransactionError(format!("Transaction timed out after {secs} seconds"))
            }
            FhevmTransactionServiceError::GasEstimation(msg) => {
                Self::TransactionError(format!("Gas estimation failed: {msg}"))
            }
            FhevmTransactionServiceError::NonceError(msg) => {
                Self::TransactionError(format!("Nonce error: {msg}"))
            }
            FhevmTransactionServiceError::Network(msg) => {
                Self::TransactionError(format!("Network error: {msg}"))
            }
            FhevmTransactionServiceError::Config(msg) => {
                Self::HandlerError(format!("Config error: {msg}"))
            }
            FhevmTransactionServiceError::Provider(msg) => {
                Self::TransactionError(format!("Provider error: {msg}"))
            }
            FhevmTransactionServiceError::Other(err) => Self::TransactionError(err.to_string()),
        }
    }
}

impl From<GatewayTxnError> for EventProcessingError {
    fn from(e: GatewayTxnError) -> Self {
        EventProcessingError::TransactionError(e.to_string())
    }
}
