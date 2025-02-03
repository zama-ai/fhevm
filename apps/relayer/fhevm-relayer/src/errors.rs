use alloy::primitives::Address;
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
}
