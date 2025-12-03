use crate::{
    config::settings::AppConfigError,
    gateway::{
        arbitrum::transaction::{engine::GatewayTxnError, fhevm::FhevmError},
        readiness_checker::ReadinessCheckError,
    },
};
use serde::{Deserialize, Serialize};

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
    #[error("Request reverted: {0:?}")]
    RequestReverted(Box<FhevmError>),


    #[error("Handler failed: {0}")]
    HandlerError(String),


    #[error("Transaction failed: {0}")]
    TransactionError(String),

    #[error("Configuration error: {0}")]
    ConfigError(#[from] AppConfigError),




    #[error("Ciphertext not ready for decryption")]
    ReadinessCheckFailed,
}

impl From<GatewayTxnError> for EventProcessingError {
    fn from(e: GatewayTxnError) -> Self {
        EventProcessingError::TransactionError(e.to_string())
    }
}

impl From<ReadinessCheckError> for EventProcessingError {
    fn from(e: ReadinessCheckError) -> Self {
        match e {
            ReadinessCheckError::Timeout => EventProcessingError::ReadinessCheckFailed,
            ReadinessCheckError::ContractError(err) => {
                EventProcessingError::HandlerError(err.to_string())
            }
        }
    }
}
