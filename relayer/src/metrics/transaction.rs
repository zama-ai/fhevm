use prometheus::{
    register_counter_vec_with_registry, register_gauge_vec_with_registry,
    register_histogram_vec_with_registry, CounterVec, GaugeVec, HistogramOpts, HistogramVec, Opts,
    Registry,
};
use std::fmt;
/// Blockchain transaction related metrics
use std::sync::OnceLock;

use crate::config::settings::MetricsConfig;
use crate::gateway::utils::RevertReason;

// 1.TODO: Track latency of the transaction sending (histogram buckets with 100ms ... up to 1000ms, 1200, ... 1500ms, 2000ms..)
// Track the status of failure after max retries (Counter + Critical alerting !!!!)
// Counter on nonces error.
// Counter on Transport errors.
// Counter on RPC errors which are not nonce errors.
// 2. track number of success resp, track number of responses (failed) and label them (Counters).
#[derive(Debug)]
struct TransactionMetrics {
    // Transaction currently in the engine.
    in_flight_transactions: GaugeVec,
    // All the transaction emitted.
    transaction_total: CounterVec,
    // Latency tracking
    transaction_duration_secs: HistogramVec,
    // Transaction errors.
    transaction_errors_total: CounterVec,
}

static TRANSACTION_METRICS: OnceLock<TransactionMetrics> = OnceLock::new();

/// Initialize transaction metrics.
/// Call this once at startup with the Prometheus registry.
pub fn init_transaction_metrics(registry: &Registry, config: MetricsConfig) {
    TRANSACTION_METRICS.get_or_init(|| TransactionMetrics {
        transaction_total: register_counter_vec_with_registry!(
            Opts::new("relayer_transaction_count", "Total number of transactions"),
            &["transaction_type", "transaction_status"],
            registry
        )
        .unwrap(),
        in_flight_transactions: register_gauge_vec_with_registry!(
            Opts::new(
                "relayer_transaction_pending_gauge",
                "Total number of pending transactions"
            ),
            &["transaction_type"],
            registry
        )
        .unwrap(),

        transaction_duration_secs: register_histogram_vec_with_registry!(
            HistogramOpts::new(
                "relayer_transaction_duration_secs",
                "Latency of transaction submission and confirmation"
            )
            .buckets(config.transaction_duration_secs_histogram_bucket.clone()),
            &["transaction_type", "status"],
            registry
        )
        .unwrap(),

        // THIS IS THE CRITICAL ONE FOR ALERTING
        transaction_errors_total: register_counter_vec_with_registry!(
            Opts::new(
                "relayer_transaction_errors_total",
                "Specific count of transaction errors by type, revert category, and request type"
            ),
            &["error_type", "revert_category", "request_type"],
            registry
        )
        .unwrap(),
    });
}

#[derive(Debug, Clone, Copy)]
pub enum TransactionType {
    UserDecryptRequest,
    InputRequest,
    PublicDecryptRequest,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionType::InputRequest => write!(f, "input_request"),
            TransactionType::UserDecryptRequest => write!(f, "user_decrypt_request"),
            TransactionType::PublicDecryptRequest => write!(f, "public_decrypt_request"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TransactionStatus {
    Failed,
    Confirmed,
}

impl TransactionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TransactionStatus::Failed => "failed",
            TransactionStatus::Confirmed => "confirmed",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TransactionErrorType {
    InvalidAddress,
    Nonce,
    Transport,
    Reverted,
    RevertedACLSelector,
    Rpc,
    Unknown,
    // This should raise a critical alert.
    MaxRetriesExceeded,
}

impl TransactionErrorType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TransactionErrorType::InvalidAddress => "invalid_address",
            TransactionErrorType::Nonce => "nonce_error",
            TransactionErrorType::Transport => "transport_error",
            TransactionErrorType::Reverted => "reverted",
            TransactionErrorType::RevertedACLSelector => "reverted_acl_selector",
            TransactionErrorType::Rpc => "rpc_error",
            TransactionErrorType::Unknown => "unknown_error",
            TransactionErrorType::MaxRetriesExceeded => "max_retries_exceeded",
        }
    }
}

pub fn transaction_broadcast(transaction_type: TransactionType) {
    let metrics = TRANSACTION_METRICS
        .get()
        .expect("Transaction metrics not initialized");
    metrics
        .in_flight_transactions
        .with_label_values(&[transaction_type.to_string().as_str()])
        .inc();
}

pub fn transaction_confirmed(transaction_type: TransactionType, duration_millis: f64) {
    let metrics = TRANSACTION_METRICS
        .get()
        .expect("Transaction metrics not initialized");
    metrics
        .in_flight_transactions
        .with_label_values(&[transaction_type.to_string().as_str()])
        .dec();
    metrics
        .transaction_total
        .with_label_values(&[
            transaction_type.to_string().as_str(),
            TransactionStatus::Confirmed.as_str(),
        ])
        .inc();

    let duration_secs = duration_millis / 1000.0;
    metrics
        .transaction_duration_secs
        .with_label_values(&[
            transaction_type.to_string().as_str(),
            TransactionStatus::Confirmed.as_str(),
        ])
        .observe(duration_secs);
}

pub fn transaction_failure(transaction_type: TransactionType, duration_millis: f64) {
    let metrics = TRANSACTION_METRICS
        .get()
        .expect("Transaction metrics not initialized");
    metrics
        .in_flight_transactions
        .with_label_values(&[transaction_type.to_string().as_str()])
        .dec();
    metrics
        .transaction_total
        .with_label_values(&[
            transaction_type.to_string().as_str(),
            TransactionStatus::Failed.as_str(),
        ])
        .inc();

    let duration_secs = duration_millis / 1000.0;
    metrics
        .transaction_duration_secs
        .with_label_values(&[
            transaction_type.to_string().as_str(),
            TransactionStatus::Failed.as_str(),
        ])
        .observe(duration_secs);
}

/// Call this SPECIFICALLY when an error occurs during the process with optional revert category and request type.
/// You can call this multiple times per transaction (e.g. 3 nonce errors before success).
pub fn track_engine_error_with_label(
    error_type: TransactionErrorType,
    revert_category: Option<&str>,
    request_type: Option<&str>,
) {
    let metrics = TRANSACTION_METRICS.get().expect("Metrics not initialized");
    let category = revert_category.unwrap_or("");
    let req_type = request_type.unwrap_or("unknown");
    metrics
        .transaction_errors_total
        .with_label_values(&[error_type.as_str(), category, req_type])
        .inc();
}

/// Call this SPECIFICALLY when an error occurs during the process (without revert category).
/// You can call this multiple times per transaction (e.g. 3 nonce errors before success).
pub fn track_engine_error(error_type: TransactionErrorType) {
    track_engine_error_with_label(error_type, None, None);
}

// Metric labels for revert reasons (for alerting)
pub const REVERT_INSUFFICIENT_BALANCE: &str = "insufficient_balance";
pub const REVERT_INSUFFICIENT_ALLOWANCE: &str = "insufficient_allowance";
pub const REVERT_CONTRACT_PAUSED: &str = "contract_paused";
pub const REVERT_INVALID_SIGNATURE: &str = "invalid_signature";
pub const REVERT_UNKNOWN: &str = "unknown";

/// Track a contract revert by reason with request type context (for alerting)
/// This is called from gateway handler on_failure hooks
pub fn track_revert_with_request_type(reason: RevertReason, request_type: &str) {
    let label = match reason {
        RevertReason::InsufficientBalance => REVERT_INSUFFICIENT_BALANCE,
        RevertReason::InsufficientAllowance => REVERT_INSUFFICIENT_ALLOWANCE,
        RevertReason::ContractPaused => REVERT_CONTRACT_PAUSED,
        RevertReason::InvalidSignature => REVERT_INVALID_SIGNATURE,
        RevertReason::Unknown => REVERT_UNKNOWN,
    };

    track_engine_error_with_label(
        TransactionErrorType::Reverted,
        Some(label),
        Some(request_type),
    );
}
