//! POST request-deduplication metrics.
//!
//! Every v2 POST handler inserts first and dispatches an orchestrator event
//! only when the insert was new; a repeated request is served from the row
//! that already exists. This counter splits those two branches so the dedup
//! hit rate is observable — a sudden collapse in hits means clients stopped
//! reusing request identifiers, and a sudden spike means they are retrying.

use prometheus::{register_counter_vec_with_registry, CounterVec, Opts, Registry};
use std::sync::OnceLock;

// The request-type label reuses `RetryAfterRequestType` as-is rather than
// promoting it to a shared type in `metrics/mod.rs`, to keep this diff
// minimal. Direct and delegated user-decryption are counted together, since
// the delegated handler forwards to the same insert path.
use crate::metrics::RetryAfterRequestType;

/// Outcome of the deduplication check on a POST request.
#[derive(Debug, Clone, Copy)]
pub enum RequestCacheResult {
    /// The request was already known, so no event was dispatched.
    Hit,
    /// The request was new and was dispatched to the orchestrator.
    Miss,
}

impl RequestCacheResult {
    pub fn as_str(&self) -> &'static str {
        match self {
            RequestCacheResult::Hit => "hit",
            RequestCacheResult::Miss => "miss",
        }
    }
}

#[derive(Debug)]
struct RequestCacheMetrics {
    /// Counter of POST deduplication outcomes, by request type and result.
    request_cache_total: CounterVec,
}

static REQUEST_CACHE_METRICS: OnceLock<RequestCacheMetrics> = OnceLock::new();

/// Initialize request-cache metrics with the provided registry.
pub fn init_request_cache_metrics(registry: &Registry) {
    REQUEST_CACHE_METRICS.get_or_init(|| RequestCacheMetrics {
        request_cache_total: register_counter_vec_with_registry!(
            Opts::new(
                "relayer_request_cache_total",
                "POST requests served from an existing row (hit) or newly dispatched (miss)"
            ),
            &["req_type", "result"],
            registry,
        )
        .expect("Failed to register request_cache_total counter"),
    });
}

/// Count one POST deduplication outcome.
pub fn increment_request_cache(req_type: RetryAfterRequestType, result: RequestCacheResult) {
    if let Some(metrics) = REQUEST_CACHE_METRICS.get() {
        metrics
            .request_cache_total
            .with_label_values(&[req_type.as_str(), result.as_str()])
            .inc();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_result_as_str() {
        assert_eq!(RequestCacheResult::Hit.as_str(), "hit");
        assert_eq!(RequestCacheResult::Miss.as_str(), "miss");
    }
}
