//! Listener metrics registration and helpers.
//!
//! This module provides:
//! - [`describe_metrics()`]: registers Prometheus HELP strings for all listener metrics
//! - [`init_gauges()`]: initializes gauge values to zero for Grafana discoverability
//! - [`init_counters()`]: initializes gauge counters to zero for Grafana discoverability
//! - [`error_kind_label()`]: maps [`EvmListenerError`] variants to static label strings
//! - [`spawn_active_catchup_poller()`]: keeps the active-requests gauge fresh

use std::time::Duration;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::error;

use crate::core::evm_listener::EvmListenerError;
use crate::store::models::CatchupFlow;
use crate::store::repositories::CatchupRepository;

/// Poll the active catchup request count and publish it as a gauge.
///
/// This is a gauge, so its value is only as true as its refresh rate. It used
/// to ride the hourly block cleaner, which meant it reported whatever was
/// active at the top of the hour and read zero for the fifty-nine minutes
/// during which a catchup actually ran. It polls on the same cadence as the
/// queue depths instead, and no longer stops when `cleaner.active` is false —
/// how many catchups are running is not a question about block retention.
pub fn spawn_active_catchup_poller(
    catchups: CatchupRepository,
    chain_id: u64,
    interval: Duration,
    cancel: CancellationToken,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancel.cancelled() => return,
                _ = tokio::time::sleep(interval) => {}
            }

            // Both flows are always emitted, including as zero. `GROUP BY`
            // returns no row for a flow with nothing active, and leaving the
            // gauge unset there would pin it at the last non-zero value it
            // ever had.
            match catchups.count_active_by_flow().await {
                Ok(counts) => {
                    for flow in [CatchupFlow::Catchup, CatchupFlow::FinalCatchup] {
                        let active = counts
                            .iter()
                            .find(|(f, _)| *f == flow)
                            .map_or(0, |(_, count)| *count);

                        metrics::gauge!(
                            "listener_catchup_active_requests",
                            "chain_id" => chain_id.to_string(),
                            "flow" => flow.metric_label()
                        )
                        .set(active as f64);
                    }
                }
                Err(e) => {
                    error!(
                        error = %e,
                        "Failed to count active catchup requests, leaving gauge unchanged"
                    );
                }
            }
        }
    })
}

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

    // ── Catchup ─────────────────────────────────────────────────────────
    //
    // Both flows report here, separated by the `flow` label
    // (`catchup` / `final_catchup`), rather than by a `listener_final_catchup_*`
    // name. The flow is a dimension of one pipeline, not a different pipeline,
    // so it belongs in a label: `sum by (flow) (...)` splits them and a bare
    // `sum(...)` totals them, neither of which is expressible across two names.
    describe_counter!(
        "listener_catchup_iterations_total",
        Unit::Count,
        "Total CatchupPayloads received on a principal catchup queue (orchestrator invocations)"
    );
    describe_counter!(
        "listener_catchup_skipped_above_head_total",
        Unit::Count,
        "Catchup orchestrator skips: block_start was above the current head (chain head for `catchup`, finalized head for `final_catchup`)"
    );
    describe_counter!(
        "listener_catchup_subranges_total",
        Unit::Count,
        "Total sub-ranges fanned out by a catchup orchestrator. Counts messages published, so a re-fan after a crash mid-fanout counts again"
    );
    describe_histogram!(
        "listener_catchup_range_duration_seconds",
        Unit::Seconds,
        "Wall-clock time to fetch and publish a single catchup sub-range (one range-catchup message). Records both successful and failed sub-ranges"
    );
    describe_counter!(
        "listener_catchup_subrange_discarded_total",
        Unit::Count,
        "Sub-ranges dropped without fetching because their request was already cancelled or skipped"
    );
    describe_counter!(
        "listener_catchup_completed_total",
        Unit::Count,
        "Catchup requests moved to COMPLETED: every block the request fanned out has been fetched and published. Counts requests, not sub-ranges, and fires once per request"
    );
    describe_counter!(
        "listener_catchup_cancelled_total",
        Unit::Count,
        "Catchup requests moved to CANCELLED by their owner"
    );
    describe_counter!(
        "listener_catchup_cancel_rejected_total",
        Unit::Count,
        "Cancels refused because the catchup_id belongs to a different consumer — the request is still running and the caller was not told"
    );
    describe_gauge!(
        "listener_catchup_active_requests",
        Unit::Count,
        "Catchup requests currently ACTIVE. Should track the number of consumers; a climbing value means a consumer is minting ids without cancelling the ones they replace"
    );

    // ── Finality ────────────────────────────────────────────────────────
    describe_counter!(
        "listener_finality_iterations_total",
        Unit::Count,
        "Total finality loop iterations (stall detection: rate should be > 0 when finality is active)"
    );
    describe_gauge!(
        "listener_final_tip_block_number",
        Unit::Count,
        "Latest final block number in the database (final_blocks tip)"
    );
    describe_gauge!(
        "listener_final_height_block_number",
        Unit::Count,
        "Latest final block number reported by the RPC node (finalized tag or head - finality_depth)"
    );
    describe_histogram!(
        "listener_finality_range_fetch_duration_seconds",
        Unit::Seconds,
        "Wall-clock time to fetch, publish, and insert an entire final block range"
    );
    describe_gauge!(
        "listener_finality_active",
        Unit::Count,
        "Whether the finality flow is enabled for this chain (1 = active, 0 = inactive)"
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
        "chain_id" => chain_id_str.clone()
    )
    .set(0.0);

    metrics::gauge!(
        "listener_final_tip_block_number",
        "chain_id" => chain_id_str.clone()
    )
    .set(0.0);

    metrics::gauge!(
        "listener_final_height_block_number",
        "chain_id" => chain_id_str.clone()
    )
    .set(0.0);

    // The active-requests poller sleeps before its first query, so without this
    // the gauge does not exist for the first poll interval and a dashboard
    // opened at boot renders "No data" rather than zero.
    for flow in [CatchupFlow::Catchup, CatchupFlow::FinalCatchup] {
        metrics::gauge!(
            "listener_catchup_active_requests",
            "chain_id" => chain_id_str.clone(),
            "flow" => flow.metric_label()
        )
        .set(0.0);
    }
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

    // Catchup counters — `chain_id` + `flow`, so both flows are seeded.
    // `cancel_rejected` especially: it exists to be alerted on, and an alert
    // cannot fire on a series that first appears at the moment of the event it
    // is supposed to catch.
    for flow in [CatchupFlow::Catchup, CatchupFlow::FinalCatchup] {
        metrics::counter!(
            "listener_catchup_iterations_total",
            "chain_id" => chain_id_str.clone(),
            "flow" => flow.metric_label()
        )
        .increment(0);
        metrics::counter!(
            "listener_catchup_skipped_above_head_total",
            "chain_id" => chain_id_str.clone(),
            "flow" => flow.metric_label()
        )
        .increment(0);
        metrics::counter!(
            "listener_catchup_subranges_total",
            "chain_id" => chain_id_str.clone(),
            "flow" => flow.metric_label()
        )
        .increment(0);
        metrics::counter!(
            "listener_catchup_subrange_discarded_total",
            "chain_id" => chain_id_str.clone(),
            "flow" => flow.metric_label()
        )
        .increment(0);
        metrics::counter!(
            "listener_catchup_completed_total",
            "chain_id" => chain_id_str.clone(),
            "flow" => flow.metric_label()
        )
        .increment(0);
        metrics::counter!(
            "listener_catchup_cancelled_total",
            "chain_id" => chain_id_str.clone(),
            "flow" => flow.metric_label()
        )
        .increment(0);
        metrics::counter!(
            "listener_catchup_cancel_rejected_total",
            "chain_id" => chain_id_str.clone(),
            "flow" => flow.metric_label()
        )
        .increment(0);
    }

    // Finality loop counter — distinct from the final-catchup flow above.
    metrics::counter!(
        "listener_finality_iterations_total",
        "chain_id" => chain_id_str
    )
    .increment(0);
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
