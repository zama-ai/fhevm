//! Retry-after raw ETA histogram metrics.
//!
//! Tracks the raw ETA (before margin and clamping) computed during POST requests.
//! This allows observing the actual estimated completion times before they are
//! adjusted and capped.

use once_cell::sync::OnceCell;
use prometheus::{register_histogram_vec_with_registry, HistogramOpts, HistogramVec, Registry};

/// Request type for retry-after metrics.
#[derive(Debug, Clone, Copy)]
pub enum RetryAfterRequestType {
    InputProof,
    UserDecrypt,
    PublicDecrypt,
}

impl RetryAfterRequestType {
    pub fn as_str(&self) -> &'static str {
        match self {
            RetryAfterRequestType::InputProof => "input_proof",
            RetryAfterRequestType::UserDecrypt => "user_decrypt",
            RetryAfterRequestType::PublicDecrypt => "public_decrypt",
        }
    }
}

#[derive(Debug)]
struct RetryAfterMetrics {
    /// Histogram tracking raw ETA values (before safety margin and clamping) in seconds.
    /// Only recorded on POST requests since GET doesn't compute meaningful raw ETA.
    raw_eta_histogram: HistogramVec,
}

static RETRY_AFTER_METRICS: OnceCell<RetryAfterMetrics> = OnceCell::new();

/// Initialize retry-after metrics with the provided registry and histogram buckets.
pub fn init_retry_after_metrics(registry: &Registry, buckets: Vec<f64>) {
    RETRY_AFTER_METRICS.get_or_init(|| RetryAfterMetrics {
        raw_eta_histogram: register_histogram_vec_with_registry!(
            HistogramOpts::new(
                "relayer_retry_after_raw_eta_seconds",
                "Raw ETA in seconds before safety margin and clamping (POST only)"
            )
            .buckets(buckets),
            &["req_type"],
            registry,
        )
        .expect("Failed to register retry_after_raw_eta_seconds histogram"),
    });
}

/// Record a raw ETA observation for the given request type.
///
/// This should only be called on POST requests where we compute a meaningful ETA.
/// GET requests don't have a meaningful raw ETA to record.
pub fn observe_raw_eta_seconds(req_type: RetryAfterRequestType, raw_eta_seconds: f64) {
    if let Some(metrics) = RETRY_AFTER_METRICS.get() {
        metrics
            .raw_eta_histogram
            .with_label_values(&[req_type.as_str()])
            .observe(raw_eta_seconds);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_type_as_str() {
        assert_eq!(RetryAfterRequestType::InputProof.as_str(), "input_proof");
        assert_eq!(RetryAfterRequestType::UserDecrypt.as_str(), "user_decrypt");
        assert_eq!(
            RetryAfterRequestType::PublicDecrypt.as_str(),
            "public_decrypt"
        );
    }
}
