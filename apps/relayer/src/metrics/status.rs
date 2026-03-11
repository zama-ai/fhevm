use prometheus::{
    register_gauge_vec_with_registry, register_histogram_vec_with_registry, GaugeVec,
    HistogramOpts, HistogramVec, Opts, Registry,
};
use std::sync::OnceLock;

use crate::{
    config::settings::MetricsConfig, store::sql::models::req_status_enum_model::ReqStatus,
};

#[derive(Debug)]
struct StatusMetrics {
    // Count how many requests are in each statuses.
    pub request_status_count: GaugeVec,
    // Histogram for duration.
    pub request_status_duration: HistogramVec,
}

static STATUS_METRICS: OnceLock<StatusMetrics> = OnceLock::new();

pub fn init_statuses_metrics(registry: &Registry, config: MetricsConfig) {
    STATUS_METRICS.get_or_init(|| StatusMetrics {
        request_status_count: register_gauge_vec_with_registry!(
            Opts::new(
                "relayer_request_count",
                "Number of request by table and statuses"
            ),
            &["req_type", "status"],
            registry,
        )
        .unwrap(),
        request_status_duration: register_histogram_vec_with_registry!(
            HistogramOpts::new(
                "relayer_request_status_duration_seconds",
                "Time spent in a status before transitioning to the next"
            )
            .buckets(config.request_status_duration_histogram_bucket.clone()),
            &["req_type", "previous_status"], // We track the status we are LEAVING
            registry,
        )
        .unwrap(),
    });
}

// Reuse your Table enum or define a specific one for DB
pub use crate::metrics::Table;

pub enum RequestType {
    UserDecrypt,
    PublicDecrypt,
    InputProof,
}

impl RequestType {
    pub fn as_str(&self) -> &'static str {
        match self {
            RequestType::UserDecrypt => "user_decrypt",
            RequestType::PublicDecrypt => "public_decrypt",
            RequestType::InputProof => "input_proof",
        }
    }
}

pub fn increment_req_status_count(req_type: RequestType, status: ReqStatus) {
    let metrics = STATUS_METRICS
        .get()
        .expect("Statuses metrics not initialized.");
    metrics
        .request_status_count
        .with_label_values(&[req_type.as_str(), status.as_str()])
        .inc();
}

pub fn decrement_req_status_count(req_type: RequestType, status: ReqStatus) {
    let metrics = STATUS_METRICS
        .get()
        .expect("Statuses metrics not initialized.");
    metrics
        .request_status_count
        .with_label_values(&[req_type.as_str(), status.as_str()])
        .dec();
}

pub fn set_req_status_count(req_type: RequestType, status: ReqStatus, count: i64) {
    let metrics = STATUS_METRICS
        .get()
        .expect("Statuses metrics not initialized.");
    metrics
        .request_status_count
        .with_label_values(&[req_type.as_str(), status.as_str()])
        .set(count as f64);
}

// Helper to handle the logic in one place.
// TODO: verify the seconds logic, and create the subsequent histogram bucket.
pub fn record_status_transition(
    req_type: RequestType,
    old_status: ReqStatus,
    new_status: ReqStatus,
    old_updated_at: chrono::DateTime<chrono::Utc>,
    new_updated_at: chrono::DateTime<chrono::Utc>,
) {
    let metrics = STATUS_METRICS.get().expect("Metrics not initialized");

    // 1. Update Counters
    metrics
        .request_status_count
        .with_label_values(&[req_type.as_str(), old_status.as_str()])
        .dec();
    metrics
        .request_status_count
        .with_label_values(&[req_type.as_str(), new_status.as_str()])
        .inc();

    // 2. Record Duration
    // Calculate how long it was in the old_status
    let duration = new_updated_at.signed_duration_since(old_updated_at);
    // Ensure we don't record negative time (clock skew protection), convert to seconds (f64)
    let seconds = duration.num_milliseconds() as f64 / 1000.0;

    if seconds >= 0.0 {
        metrics
            .request_status_duration
            .with_label_values(&[req_type.as_str(), old_status.as_str()])
            .observe(seconds);
    }
}
