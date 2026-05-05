//! Listener metrics registration and helpers.
//!
//! This module provides:
//! - [`describe_metrics()`]: registers Prometheus HELP strings for all listener metrics
//! - [`init_gauges()`]: initializes gauge values to zero for Grafana discoverability
//! - [`error_kind_label()`]: maps [`EvmListenerError`] variants to static label strings

use crate::core::evm_listener::EvmListenerError;

/// Register metric descriptions with the global recorder.
///
/// Call once at application startup, after installing the metrics exporter.
/// Safe to call multiple times (describe is idempotent).
pub fn describe_metrics() {
    use metrics::{Unit, describe_counter, describe_gauge, describe_histogram};

    // ── Cursor liveness ─────────────────────────────────────────────────
    describe_counter!(
        "listener_cursor_iterations_total",
        Unit::Count,
        "Total main cursor loop iterations (stall detection: rate should be > 0)"
    );

    // ── Reorgs ──────────────────────────────────────────────────────────
    describe_counter!(
        "listener_reorgs_total",
        Unit::Count,
        "Total chain reorganizations detected"
    );

    // ── Block heights ───────────────────────────────────────────────────
    describe_gauge!(
        "listener_db_tip_block_number",
        Unit::Count,
        "Latest canonical block number in the database"
    );
    describe_gauge!(
        "listener_chain_height_block_number",
        Unit::Count,
        "Latest block number reported by the RPC node"
    );

    // ── Fetch timing ────────────────────────────────────────────────────
    describe_histogram!(
        "listener_block_fetch_duration_seconds",
        Unit::Seconds,
        "Wall-clock time to fetch a single block with receipts"
    );
    describe_histogram!(
        "listener_range_fetch_duration_seconds",
        Unit::Seconds,
        "Wall-clock time to fetch and process an entire block range"
    );

    // ── Publish errors ──────────────────────────────────────────────────
    describe_counter!(
        "listener_publish_errors_total",
        Unit::Count,
        "Failures during event publishing to broker"
    );

    // ── Error classification ────────────────────────────────────────────
    describe_counter!(
        "listener_transient_errors_total",
        Unit::Count,
        "Transient (infrastructure) errors from handler error classification"
    );
    describe_counter!(
        "listener_permanent_errors_total",
        Unit::Count,
        "Permanent (logic) errors from handler error classification"
    );

    // ── Block compute verification ─────────────────────────────────────
    describe_counter!(
        "listener_compute_block_failure_total",
        Unit::Count,
        "Block hash verification failures during block compute"
    );
    describe_counter!(
        "listener_compute_transaction_failure_total",
        Unit::Count,
        "Transaction root verification failures during block compute"
    );
    describe_counter!(
        "listener_compute_receipt_failure_total",
        Unit::Count,
        "Receipt root verification failures during block compute"
    );

    // ── RPC provider ────────────────────────────────────────────────────
    describe_histogram!(
        "listener_rpc_request_duration_seconds",
        Unit::Seconds,
        "Wall-clock time per JSON-RPC call (includes semaphore wait)"
    );
    describe_counter!(
        "listener_rpc_requests_total",
        Unit::Count,
        "Total RPC requests partitioned by outcome"
    );
    describe_counter!(
        "listener_rpc_errors_total",
        Unit::Count,
        "RPC errors by method and error type"
    );
    describe_gauge!(
        "listener_rpc_semaphore_available",
        Unit::Count,
        "Available permits in the RPC concurrency semaphore"
    );
}

/// Initialize gauges to zero so Grafana discovers the time series on the first scrape,
/// even before the first cursor iteration completes.
///
/// Call once at startup, after [`describe_metrics()`].
pub fn init_gauges(chain_id: u64) {
    let chain_id_str = chain_id.to_string();

    metrics::gauge!(
        "listener_db_tip_block_number",
        "chain_id" => chain_id_str.clone()
    )
    .set(0.0);

    metrics::gauge!(
        "listener_chain_height_block_number",
        "chain_id" => chain_id_str
    )
    .set(0.0);
}

/// Initialize block-compute failure counters to zero for every `stalling` label
/// combination, so the time series exist from startup.
///
/// Why: `increase()` / `rate()` need at least two samples in the lookback window
/// to compute a delta. If a counter goes from "absent" to `1` on the first
/// failure, a Grafana stat panel using `increase(...[24h])` will report `0`
/// because there is no baseline to compare against. Seeding the series at `0`
/// makes the first real failure show up immediately as `1`.
///
/// Call once at startup, after [`describe_metrics()`].
pub fn init_counters(chain_id: u64) {
    let chain_id_str = chain_id.to_string();

    for stalling in ["true", "false"] {
        metrics::counter!(
            "listener_compute_block_failure_total",
            "chain_id" => chain_id_str.clone(),
            "stalling" => stalling
        )
        .increment(0);
        metrics::counter!(
            "listener_compute_transaction_failure_total",
            "chain_id" => chain_id_str.clone(),
            "stalling" => stalling
        )
        .increment(0);
        metrics::counter!(
            "listener_compute_receipt_failure_total",
            "chain_id" => chain_id_str.clone(),
            "stalling" => stalling
        )
        .increment(0);
    }
}

/// Map an [`EvmListenerError`] variant to a static label string for the `error_kind` label.
pub(crate) fn error_kind_label(err: &EvmListenerError) -> &'static str {
    match err {
        EvmListenerError::CouldNotFetchBlock { .. } => "block_fetch",
        EvmListenerError::CouldNotComputeBlock { .. } => "block_compute",
        EvmListenerError::DatabaseError { .. } => "database",
        EvmListenerError::ChainHeightError { .. } => "chain_height",
        EvmListenerError::SlotBufferError { .. } => "slot_buffer",
        EvmListenerError::BrokerPublishError { .. } => "broker_publish",
        EvmListenerError::PayloadBuildError { .. } => "payload_build",
        EvmListenerError::InvariantViolation { .. } => "invariant_violation",
        EvmListenerError::MessageProcessingError { .. } => "message_processing",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describe_metrics_does_not_panic() {
        describe_metrics();
    }

    #[test]
    fn init_gauges_does_not_panic() {
        init_gauges(1);
    }

    #[test]
    fn init_counters_does_not_panic() {
        init_counters(1);
    }
}
