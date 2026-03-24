use prometheus::{register_gauge_vec_with_registry, GaugeVec, Opts, Registry};
use std::sync::OnceLock;

#[derive(Debug)]
struct QueueMetrics {
    // Count how many requests are in a given queue waiting to be drained.
    pub queue_size_count: GaugeVec,
}

static QUEUE_METRICS: OnceLock<QueueMetrics> = OnceLock::new();

pub fn init_queue_metrics(registry: &Registry) {
    QUEUE_METRICS.get_or_init(|| QueueMetrics {
        queue_size_count: register_gauge_vec_with_registry!(
            Opts::new(
                "relayer_queue_size_count", // Added 'relayer_' prefix for consistency
                "Number of request in a given throttling queue"
            ),
            &["queue_type"],
            registry,
        )
        .unwrap(),
    });

    // Initialize all queues to 0.0 immediately
    let metrics = QUEUE_METRICS.get().expect("Queue metrics not initialized");
    for queue in QueueType::all() {
        metrics
            .queue_size_count
            .with_label_values(&[queue.as_str()])
            .set(0.0);
    }
}

// Reuse your Table enum or define a specific one for DB
pub use crate::metrics::Table;

#[derive(Debug, Clone, Copy)] // Added Clone, Copy for iteration
pub enum QueueType {
    InputProofTxThrottler,
    UserDecryptTxThrottler,
    PublicDecryptTxThrottler,
    UserDecryptReadinessThrottler,
    PublicDecryptReadinessThrottler,
}

impl QueueType {
    pub fn as_str(&self) -> &'static str {
        match self {
            QueueType::InputProofTxThrottler => "input_proof_tx_throttler",
            QueueType::UserDecryptTxThrottler => "user_decrypt_tx_throttler",
            QueueType::PublicDecryptTxThrottler => "public_decrypt_tx_throttler",
            QueueType::UserDecryptReadinessThrottler => "user_decrypt_readiness_throttler",
            QueueType::PublicDecryptReadinessThrottler => "public_decrypt_readiness_throttler",
        }
    }

    /// Helper to iterate over all variants for initialization
    pub fn all() -> &'static [QueueType] {
        &[
            QueueType::InputProofTxThrottler,
            QueueType::UserDecryptTxThrottler,
            QueueType::PublicDecryptTxThrottler,
            QueueType::UserDecryptReadinessThrottler,
            QueueType::PublicDecryptReadinessThrottler,
        ]
    }
}

pub fn increment_queue_size(queue_type: QueueType) {
    let metrics = QUEUE_METRICS.get().expect("Queue metrics not initialized.");
    metrics
        .queue_size_count
        .with_label_values(&[queue_type.as_str()])
        .inc();
}

pub fn decrement_queue_size(queue_type: QueueType) {
    let metrics = QUEUE_METRICS.get().expect("Queue metrics not initialized.");
    metrics
        .queue_size_count
        .with_label_values(&[queue_type.as_str()])
        .dec();
}
