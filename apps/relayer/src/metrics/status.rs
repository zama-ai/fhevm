use once_cell::sync::OnceCell;
use prometheus::{
    register_gauge_vec_with_registry, register_histogram_vec_with_registry, GaugeVec,
    HistogramOpts, HistogramVec, Opts, Registry,
};

use crate::store::sql::models::req_status_enum_model::ReqStatus;

#[derive(Debug)]
struct InternalMetrics {
    // Count how many requests are in each statuses.
    pub request_status_count: GaugeVec,
    // Histogram for duration.
    pub request_status_duration: HistogramVec,
}

static STATUS_METRICS: OnceCell<InternalMetrics> = OnceCell::new();

pub fn init_statuses_metrics(registry: &Registry) {
    STATUS_METRICS.get_or_init(|| InternalMetrics {
        request_status_count: register_gauge_vec_with_registry!(
            Opts::new(
                "relayer_request_count",
                "Number of request by table and statuses"
            ),
            &["table", "status"],
            registry,
        )
        .unwrap(),
        request_status_duration: register_histogram_vec_with_registry!(
            HistogramOpts::new(
                "relayer_request_status_duration_seconds",
                "Time spent in a status before transitioning to the next"
            ) // Bucket Strategy:
            // - Fast/Internal Logic: 0.1s (100ms) to 1.0s
            // - Blockchain/Network: 2.5s to 60s
            // - Long Polling/Timeouts: 5 min, 10 min, 30 min, 1 hour
            .buckets(vec![
                0.1, 0.25, 0.5, 1.0, // Sub-second (Internal processing)
                2.5, 5.0, 10.0, 30.0, 60.0, // Seconds (Network/RPC latency)
                300.0, 600.0, 1800.0, 3600.0 // Minutes (Timeouts/Stuck detection)
            ]),
            &["table", "previous_status"], // We track the status we are LEAVING
            registry,
        )
        .unwrap(),
    });
}

// Reuse your Table enum or define a specific one for DB
pub use crate::metrics::Table;

pub fn increment_req_status_count(table: Table, status: ReqStatus) {
    let metrics = STATUS_METRICS
        .get()
        .expect("Statuses metrics not initialized.");
    metrics
        .request_status_count
        .with_label_values(&[table.as_str(), status.as_str()])
        .inc();
}

pub fn decrement_req_status_count(table: Table, status: ReqStatus) {
    let metrics = STATUS_METRICS
        .get()
        .expect("Statuses metrics not initialized.");
    metrics
        .request_status_count
        .with_label_values(&[table.as_str(), status.as_str()])
        .dec();
}

// Helper to handle the logic in one place.
// TODO: verify the seconds logic, and create the subsequent histogram bucket.
pub fn record_status_transition(
    table: Table,
    old_status: ReqStatus,
    new_status: ReqStatus,
    old_updated_at: chrono::DateTime<chrono::Utc>,
    new_updated_at: chrono::DateTime<chrono::Utc>,
) {
    let metrics = STATUS_METRICS.get().expect("Metrics not initialized");

    // 1. Update Counters
    metrics
        .request_status_count
        .with_label_values(&[table.as_str(), old_status.as_str()])
        .dec();
    metrics
        .request_status_count
        .with_label_values(&[table.as_str(), new_status.as_str()])
        .inc();

    // 2. Record Duration
    // Calculate how long it was in the old_status
    let duration = new_updated_at.signed_duration_since(old_updated_at);
    // Ensure we don't record negative time (clock skew protection), convert to seconds (f64)
    let seconds = duration.num_milliseconds() as f64 / 1000.0;

    if seconds >= 0.0 {
        metrics
            .request_status_duration
            .with_label_values(&[table.as_str(), old_status.as_str()])
            .observe(seconds);
    }
}
