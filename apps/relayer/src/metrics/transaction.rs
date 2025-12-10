/// Blockchain transaction related metrics
use once_cell::sync::OnceCell;
use prometheus::{
    register_counter_vec_with_registry, register_gauge_vec_with_registry,
    register_histogram_vec_with_registry, CounterVec, GaugeVec, HistogramOpts, HistogramVec, Opts,
    Registry,
};
use std::fmt;

// 1.TODO: Track latency of the transaction sending (histogram buckets with 100ms ... up to 1000ms, 1200, ... 1500ms, 2000ms..)
// Track the status of failure after max retries (Counter + Critical alerting !!!!)
// Counter on nonces error.
// Counter on Transport errors.
// Counter on RPC errors which are not nonce errors.
// 2. track number of success resp, track nimber of responses (failed) and label them (Counters).
#[derive(Debug)]
struct TransactionMetrics {
    // Transaction currently in the engine.
    in_flight_transactions: GaugeVec,
    // All the transaction emitted.
    transaction_total: CounterVec,
    // Latency tracking
    transaction_duration_millis: HistogramVec,
    // Transaction errors.
    transaction_errors_total: CounterVec,
}

static TRANSACTION_METRICS: OnceCell<TransactionMetrics> = OnceCell::new();

/// Initialize transaction metrics.
/// Call this once at startup with the Prometheus registry.
pub fn init_transaction_metrics(registry: &Registry) {
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

        transaction_duration_millis: register_histogram_vec_with_registry!(
            HistogramOpts::new(
                "relayer_transaction_duration_milliseconds",
                "Latency of transaction submission and confirmation"
            )
            // Buckets: 10ms, 100ms, 250ms, 500ms, 750ms, 1s, 1.25s, 1.5s, 2s, 5s, 10s
            .buckets(vec![
                10.0, 100.0, 250.0, 500.0, 750.0, 1000.0, 1250.0, 1500.0, 2000.0, 5000.0, 10000.0
            ]),
            &["transaction_type", "status"],
            registry
        )
        .unwrap(),

        // THIS IS THE CRITICAL ONE FOR ALERTING
        transaction_errors_total: register_counter_vec_with_registry!(
            Opts::new(
                "relayer_transaction_errors_total",
                "Specific count of transaction errors by type"
            ),
            &["transaction_type", "error_type"],
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
pub enum ErrorType {
    InvalidAddress,
    Nonce,
    Transport,
    Simulation,
    Rpc,
    // This alert is critical, at 1 for count we should raise a CRITICAL alert.
    MaxRetriesExceeded,
}

impl ErrorType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorType::InvalidAddress => "invalid_address",
            ErrorType::Nonce => "nonce_error",
            ErrorType::Transport => "transport_error",
            ErrorType::Simulation => "simulation_failed",
            ErrorType::Rpc => "rpc_error",
            ErrorType::MaxRetriesExceeded => "max_retries_exceeded",
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
    metrics
        .transaction_duration_millis
        .with_label_values(&[
            transaction_type.to_string().as_str(),
            TransactionStatus::Confirmed.as_str(),
        ])
        .observe(duration_millis);
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
    metrics
        .transaction_duration_millis
        .with_label_values(&[
            transaction_type.to_string().as_str(),
            TransactionStatus::Failed.as_str(),
        ])
        .observe(duration_millis);
}

/// Call this SPECIFICALLY when an error occurs during the process.
/// You can call this multiple times per transaction (e.g. 3 nonce errors before success).
pub fn track_manager_error(tx_type: TransactionType, error_type: ErrorType) {
    let metrics = TRANSACTION_METRICS.get().expect("Metrics not initialized");
    metrics
        .transaction_errors_total
        .with_label_values(&[tx_type.to_string().as_str(), error_type.as_str()])
        .inc();
}
