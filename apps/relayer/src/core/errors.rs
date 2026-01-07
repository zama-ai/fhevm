use crate::{
    config::settings::AppConfigError,
    gateway::{
        arbitrum::transaction::{engine::GatewayTxnError, fhevm::FhevmError},
        readiness_check::readiness_checker::ReadinessCheckError,
    },
};
use serde::{Deserialize, Serialize};

use thiserror::Error;

// Standardized timeout error messages
pub const READINESS_CHECK_TIMEOUT_MSG: &str =
    "Ciphertext not ready for decryption on the gateway chain";
pub const RESPONSE_TIMEOUT_MSG: &str =
    "Gateway chain did not respond within the expected timeframe";
pub const TIMEOUT_REASON_MISSING_MSG: &str = "Request timed out (reason not available)";

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
    #[error("Request reverted: {0:?}")]
    RequestReverted(Box<FhevmError>),

    #[error("Failed to decode event {event_type}: {reason}")]
    EventDecodingFailed { event_type: String, reason: String },

    #[error("SQL operation '{operation}' failed: {reason}")]
    SqlOperationFailed { operation: String, reason: String },

    #[error("Failed to aggregate decryption shares: {0}")]
    ShareAggregationFailed(String),

    #[error("Contract call failed: {0}")]
    ContractCallFailed(String),

    #[error("Validation failed for {field}: {reason}")]
    ValidationFailed { field: String, reason: String },

    #[error("Transaction failed: {0:?}")]
    TransactionError(Box<GatewayTxnError>),

    #[error("Configuration error: {0}")]
    ConfigError(#[from] AppConfigError),

    #[error("{}", crate::core::errors::READINESS_CHECK_TIMEOUT_MSG)]
    ReadinessCheckTimedOut,

    #[error("Relayer internal queue is full")]
    QueueFull,

    #[error("Relayer queue channel is closed")]
    ChannelClosed,

    #[error("Protocol Overwhelmed: {0}")]
    ProtocolOverload(String),
}

impl From<GatewayTxnError> for EventProcessingError {
    fn from(e: GatewayTxnError) -> Self {
        EventProcessingError::TransactionError(Box::new(e))
    }
}

impl From<ReadinessCheckError> for EventProcessingError {
    fn from(e: ReadinessCheckError) -> Self {
        match e {
            ReadinessCheckError::Timeout => EventProcessingError::ReadinessCheckTimedOut,
            ReadinessCheckError::ContractError(err) => {
                EventProcessingError::ContractCallFailed(err.to_string())
            }
        }
    }
}
