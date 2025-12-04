/// Blockchain transaction related metrics
use once_cell::sync::OnceCell;
use prometheus::{
    register_counter_vec_with_registry, register_gauge_vec_with_registry, CounterVec, GaugeVec,
    Opts, Registry,
};
use std::fmt;

// 1.TODO: Track latency of the transaction sending (histogram buckets with 100ms ... up to 1000ms, 1200, ... 1500ms, 2000ms..)
// 2. track number of success resp, track nimber of responses (failed) and label them (Counters).
#[derive(Debug)]
struct TransactionMetrics {
    // change pending name to in-flight..
    pending_transactions_gauge: GaugeVec,
    transactions_counter: CounterVec,
}

static TRANSACTION_METRICS: OnceCell<TransactionMetrics> = OnceCell::new();

/// Initialize transaction metrics.
/// Call this once at startup with the Prometheus registry.
pub fn init_transaction_metrics(registry: &Registry) {
    TRANSACTION_METRICS.get_or_init(|| TransactionMetrics {
        transactions_counter: register_counter_vec_with_registry!(
            Opts::new("relayer_transaction_count", "Total number of transactions"),
            &["transaction_type", "transaction_status"],
            registry
        )
        .unwrap(),
        pending_transactions_gauge: register_gauge_vec_with_registry!(
            Opts::new(
                "relayer_transaction_pending_gauge",
                "Total number of pending transactions"
            ),
            &["transaction_type"],
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

pub fn transaction_broadcast(transaction_type: TransactionType) {
    let metrics = TRANSACTION_METRICS
        .get()
        .expect("Transaction metrics not initialized");
    metrics
        .pending_transactions_gauge
        .with_label_values(&[transaction_type.to_string().as_str()])
        .inc();
}

pub fn transaction_confirmed(transaction_type: TransactionType) {
    let metrics = TRANSACTION_METRICS
        .get()
        .expect("Transaction metrics not initialized");
    metrics
        .pending_transactions_gauge
        .with_label_values(&[transaction_type.to_string().as_str()])
        .dec();
    metrics
        .transactions_counter
        .with_label_values(&[
            transaction_type.to_string().as_str(),
            TransactionStatus::Confirmed.as_str(),
        ])
        .inc();
}

pub fn transaction_failure(transaction_type: TransactionType) {
    let metrics = TRANSACTION_METRICS
        .get()
        .expect("Transaction metrics not initialized");
    metrics
        .pending_transactions_gauge
        .with_label_values(&[transaction_type.to_string().as_str()])
        .dec();
    metrics
        .transactions_counter
        .with_label_values(&[
            transaction_type.to_string().as_str(),
            TransactionStatus::Failed.as_str(),
        ])
        .inc();
}
