//! User-decryption spare-share metrics.
//!
//! Reconstruction needs `2t+1` *valid* shares, so a client only survives a
//! corrupted share when the relayer returned more than the threshold: its
//! fault tolerance is exactly `served - threshold`. This histogram records
//! that spare count, making a cluster that has silently stopped delivering
//! spares visible before a corrupted share turns into a failed decryption.

use prometheus::{register_histogram_vec_with_registry, HistogramOpts, HistogramVec, Registry};
use std::sync::OnceLock;

use crate::metrics::RetryAfterRequestType;

/// Spare shares are a small count bounded by the MPC threshold `t`, so the
/// buckets are the integer values themselves rather than a latency-style
/// exponential scale. Zero is its own bucket: it is the alerting case.
const SPARE_SHARES_BUCKETS: &[f64] = &[0.0, 1.0, 2.0, 3.0, 4.0, 6.0, 8.0, 12.0];

#[derive(Debug)]
struct UserDecryptMetrics {
    /// Histogram of shares returned beyond the reconstruction threshold.
    spare_shares: HistogramVec,
}

static USER_DECRYPT_METRICS: OnceLock<UserDecryptMetrics> = OnceLock::new();

/// Initialize user-decryption metrics with the provided registry.
pub fn init_user_decrypt_metrics(registry: &Registry) {
    USER_DECRYPT_METRICS.get_or_init(|| UserDecryptMetrics {
        spare_shares: register_histogram_vec_with_registry!(
            HistogramOpts::new(
                "relayer_user_decrypt_spare_shares",
                "Shares returned beyond the reconstruction threshold (client fault tolerance)"
            )
            .buckets(SPARE_SHARES_BUCKETS.to_vec()),
            &["req_type"],
            registry,
        )
        .expect("Failed to register user_decrypt_spare_shares histogram"),
    });
}

/// Record the spare shares carried by a terminal user-decryption response.
///
/// Call on the terminal 200 only, never on the 202 holds: the GET is polled,
/// so recording on every response would measure how often the client polls
/// rather than how much tolerance it was given.
pub fn observe_spare_shares(req_type: RetryAfterRequestType, spare_shares: usize) {
    if let Some(metrics) = USER_DECRYPT_METRICS.get() {
        metrics
            .spare_shares
            .with_label_values(&[req_type.as_str()])
            .observe(spare_shares as f64);
    }
}
