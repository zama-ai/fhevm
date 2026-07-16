//! Bounded Prometheus metrics for the proof service.

use axum::http::StatusCode;
use prometheus::{Encoder, IntCounterVec, Opts, Registry, TextEncoder};
use std::sync::OnceLock;

static REGISTRY: OnceLock<Registry> = OnceLock::new();
static HTTP_RESPONSES: OnceLock<IntCounterVec> = OnceLock::new();
static READINESS: OnceLock<IntCounterVec> = OnceLock::new();
static PROOF_OUTCOMES: OnceLock<IntCounterVec> = OnceLock::new();

pub fn init_metrics() {
    let registry = Registry::new();

    let http = IntCounterVec::new(
        Opts::new(
            "solana_proof_http_responses_total",
            "HTTP responses by endpoint, method, and status class",
        ),
        &["endpoint", "method", "status_class"],
    )
    .expect("http counter");
    let readiness = IntCounterVec::new(
        Opts::new(
            "solana_proof_readiness_total",
            "Readiness probe outcomes by bounded classification",
        ),
        &["status"],
    )
    .expect("readiness counter");
    let proofs = IntCounterVec::new(
        Opts::new(
            "solana_proof_requests_total",
            "Proof request outcomes by bounded status label",
        ),
        &["status"],
    )
    .expect("proof counter");

    registry.register(Box::new(http.clone())).unwrap();
    registry.register(Box::new(readiness.clone())).unwrap();
    registry.register(Box::new(proofs.clone())).unwrap();

    HTTP_RESPONSES.set(http).ok();
    READINESS.set(readiness).ok();
    PROOF_OUTCOMES.set(proofs).ok();
    REGISTRY.set(registry).ok();
}

pub fn record_http(endpoint: &str, method: &str, status: StatusCode) {
    let class = status_class(status);
    if let Some(counter) = HTTP_RESPONSES.get() {
        counter.with_label_values(&[endpoint, method, class]).inc();
    }
}

pub fn record_readiness(status: &str) {
    if let Some(counter) = READINESS.get() {
        counter.with_label_values(&[status]).inc();
    }
}

pub fn record_proof(status: &str) {
    if let Some(counter) = PROOF_OUTCOMES.get() {
        counter.with_label_values(&[status]).inc();
    }
}

fn status_class(status: StatusCode) -> &'static str {
    match status.as_u16() {
        200..=299 => "2xx",
        400..=499 => "4xx",
        500..=599 => "5xx",
        _ => "other",
    }
}

pub async fn metrics_handler() -> (StatusCode, String) {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.get().map(|r| r.gather()).unwrap_or_default();
    let mut buf = Vec::new();
    match encoder.encode(&metric_families, &mut buf) {
        Ok(()) => (StatusCode::OK, String::from_utf8_lossy(&buf).into_owned()),
        Err(err) => {
            tracing::error!(%err, "failed to encode prometheus metrics");
            (StatusCode::INTERNAL_SERVER_ERROR, String::new())
        }
    }
}
