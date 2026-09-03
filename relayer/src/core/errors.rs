use crate::{
    config::settings::AppConfigError,
    gateway::arbitrum::transaction::{engine::GatewayTxnError, fhevm::FhevmError},
    host::{
        extra_data::ExtraDataError, redact_alloy_error, threshold_resolver::ThresholdResolverError,
    },
    readiness::ReadinessCheckError,
};
use serde::{Deserialize, Serialize};

use thiserror::Error;

// Standardized timeout error messages
pub const READINESS_CHECK_TIMEOUT_MSG: &str =
    "Ciphertext not ready for decryption on the gateway chain";

/// Terminal counterpart to [`READINESS_CHECK_TIMEOUT_MSG`]: the Coprocessors were reachable and
/// did not agree, so waiting cannot make the request ready. A prefix rather than a whole message,
/// following the host ACL errors, so the per-handle detail survives into the stored reason.
pub const NO_ATTESTATION_CONSENSUS_PREFIX: &str =
    "Coprocessors did not reach consensus on the ciphertext material:";

/// The registry could not support consensus, so nothing was probed. Retryable — the fault is on
/// the gateway chain and clears without a redeploy — unlike [`NO_ATTESTATION_CONSENSUS_PREFIX`],
/// where the Coprocessors answered and disagreed.
pub const GATEWAY_NOT_REACHABLE_PREFIX: &str =
    "Coprocessor registry on the gateway chain is not usable:";

/// Prefix for host ACL not-allowed errors, used for error classification.
pub const NOT_ALLOWED_ON_HOST_ACL_PREFIX: &str = "Not allowed on host ACL:";
/// Prefix for host ACL infra errors (RPC / unsupported chain), used for error classification.
pub const HOST_ACL_FAILED_PREFIX: &str = "Host ACL check failed:";
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

    /// Sibling to [`Self::ReadinessCheckTimedOut`] for the off-chain Coprocessor attestation
    /// check (`source: coprocessor_attestations`): retries were exhausted while attestation
    /// consensus was still short of threshold. `round` is the redacted (`Display`) rendering of
    /// the last round, appended so the evidence survives into the stored reason and the response
    /// body. `READINESS_CHECK_TIMEOUT_MSG` stays a prefix — matched with `starts_with` by the
    /// status handlers — so the label and HTTP status are unaffected.
    #[error("{}: {round}", crate::core::errors::READINESS_CHECK_TIMEOUT_MSG)]
    AttestationsNotReady { round: String },

    #[error("{prefix} {reason}", prefix = crate::core::errors::NO_ATTESTATION_CONSENSUS_PREFIX)]
    NoAttestationConsensus { reason: String },

    #[error("{prefix} {reason}", prefix = crate::core::errors::GATEWAY_NOT_REACHABLE_PREFIX)]
    GatewayNotReachable { reason: String },

    #[error("Relayer internal queue is full")]
    QueueFull,

    #[error("Relayer queue channel is closed")]
    ChannelClosed,

    #[error("Protocol Overwhelmed: {0}")]
    ProtocolOverload(String),

    #[error("Not allowed on host ACL: {0}")]
    NotAllowedOnHostAcl(String),

    #[error("Host ACL check failed: {0}")]
    HostAclFailed(String),

    #[error("Threshold resolution failed: {0}")]
    ThresholdResolutionFailed(String),
}

impl From<GatewayTxnError> for EventProcessingError {
    fn from(e: GatewayTxnError) -> Self {
        EventProcessingError::TransactionError(Box::new(e))
    }
}

impl From<ThresholdResolverError> for EventProcessingError {
    fn from(e: ThresholdResolverError) -> Self {
        EventProcessingError::ThresholdResolutionFailed(e.to_string())
    }
}

impl From<ExtraDataError> for EventProcessingError {
    fn from(e: ExtraDataError) -> Self {
        EventProcessingError::ThresholdResolutionFailed(e.to_string())
    }
}

impl From<ReadinessCheckError> for EventProcessingError {
    fn from(e: ReadinessCheckError) -> Self {
        match e {
            ReadinessCheckError::GwTimeout => EventProcessingError::ReadinessCheckTimedOut,
            ReadinessCheckError::GwContractError(err) => {
                EventProcessingError::ContractCallFailed(redact_alloy_error(&err))
            }
            ReadinessCheckError::NoAttestationConsensus { round } => {
                EventProcessingError::NoAttestationConsensus {
                    reason: round.to_string(),
                }
            }
            ReadinessCheckError::RegistryError { reason } => {
                EventProcessingError::GatewayNotReachable { reason }
            }
            ReadinessCheckError::AttestationsNotReady { last_round, .. } => {
                EventProcessingError::AttestationsNotReady {
                    round: last_round.to_string(),
                }
            }
            ReadinessCheckError::NotAllowedOnHostAcl(err) => {
                EventProcessingError::NotAllowedOnHostAcl(err.to_string())
            }
            ReadinessCheckError::HostAclFailed(err) => {
                EventProcessingError::HostAclFailed(err.to_string())
            }
        }
    }
}
