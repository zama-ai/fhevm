//! Settlement-frontier observability.
//!
//! Settlement freezing is by design (undrained work, incomplete orphan
//! cleanup, unverified publication, a quarantined cleanup job), but a frozen
//! frontier is an operator-attention state: branch tables grow while it
//! lasts, and some causes (quarantine, a frontier beyond the bounded restart
//! replay) never resolve on their own. These gauges make the frontier
//! alertable; a suggested rule is
//!
//! ```text
//! time() - host_listener_settlement_frontier_updated_seconds > 900
//!   and host_listener_settlement_frontier_lag_blocks > 0
//! ```
//!
//! i.e. the frontier has not advanced for 15 minutes while verified ingestion
//! is ahead of it. Lag alone is not an alert signal (it is routinely non-zero
//! for one settlement-lag window); age alone misfires on an idle chain.

use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};

use prometheus::{register_int_gauge_vec, IntGaugeVec};
use tracing::warn;

/// The coprocessor branch settlement frontier (`settled_height`).
pub(crate) static SETTLED_HEIGHT: LazyLock<IntGaugeVec> = LazyLock::new(|| {
    register_int_gauge_vec!(
        "host_listener_settled_height",
        "Coprocessor branch settlement frontier (RFC-011 settled_height)",
        &["chain_id"]
    )
    .expect("host-listener settled-height metric must register")
});

/// Blocks between the settlement candidate this pass could certify and the
/// frontier. Non-zero is normal within one settlement-lag window; growing
/// while the updated-timestamp stalls means the frontier is blocked.
pub(crate) static SETTLEMENT_FRONTIER_LAG_BLOCKS: LazyLock<IntGaugeVec> =
    LazyLock::new(|| {
        register_int_gauge_vec!(
            "host_listener_settlement_frontier_lag_blocks",
            "Blocks between the settlement candidate and the settled frontier",
            &["chain_id"]
        )
        .expect("host-listener settlement-lag metric must register")
    });

/// Unix epoch seconds of the last frontier advance, sourced from the durable
/// `coprocessor_settlement.updated_at` so it stays correct across restarts.
pub(crate) static SETTLEMENT_FRONTIER_UPDATED_SECONDS: LazyLock<IntGaugeVec> =
    LazyLock::new(|| {
        register_int_gauge_vec!(
            "host_listener_settlement_frontier_updated_seconds",
            "Unix time of the last settlement frontier advance",
            &["chain_id"]
        )
        .expect("host-listener settlement-updated metric must register")
    });

/// In-process fallback alert: warn when the frontier has not advanced for
/// this long while a settlement candidate is ahead of it.
const SETTLEMENT_STALL_WARN_SECS: i64 = 600;

/// Rate limit for the stall warning (one per interval, process-wide; the
/// host-listener binaries are single-chain).
static LAST_STALL_WARN_EPOCH: AtomicI64 = AtomicI64::new(0);

/// When the chain has NO settlement row yet (the frontier never advanced —
/// e.g. blocked since genesis), there is no durable `updated_at` to age
/// against. Fall back to the time this process first observed the missing
/// row: a correct upper bound on the last advance, so the alert still fires
/// instead of the worst stall state being the one silent case.
static FIRST_UNSETTLED_OBSERVED_EPOCH: AtomicI64 = AtomicI64::new(0);

fn now_epoch() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// Records the settlement frontier after a finalization/settlement pass and
/// emits a rate-limited warning when it looks stalled. `updated_at_epoch` is
/// `None` until the chain's first explicit settlement row exists.
pub(crate) fn record_settlement_frontier(
    chain_id: i64,
    settled_height: i64,
    candidate_height: i64,
    updated_at_epoch: Option<i64>,
) {
    let chain_label = chain_id.to_string();
    SETTLED_HEIGHT
        .with_label_values(&[chain_label.as_str()])
        .set(settled_height);
    let lag = candidate_height.saturating_sub(settled_height).max(0);
    SETTLEMENT_FRONTIER_LAG_BLOCKS
        .with_label_values(&[chain_label.as_str()])
        .set(lag);
    let updated_at_epoch = updated_at_epoch.unwrap_or_else(|| {
        let _ = FIRST_UNSETTLED_OBSERVED_EPOCH.compare_exchange(
            0,
            now_epoch(),
            Ordering::Relaxed,
            Ordering::Relaxed,
        );
        FIRST_UNSETTLED_OBSERVED_EPOCH.load(Ordering::Relaxed)
    });
    SETTLEMENT_FRONTIER_UPDATED_SECONDS
        .with_label_values(&[chain_label.as_str()])
        .set(updated_at_epoch);

    let age = now_epoch().saturating_sub(updated_at_epoch);
    if lag == 0 || age < SETTLEMENT_STALL_WARN_SECS {
        return;
    }
    let last_warn = LAST_STALL_WARN_EPOCH.load(Ordering::Relaxed);
    let now = now_epoch();
    if now.saturating_sub(last_warn) < SETTLEMENT_STALL_WARN_SECS {
        return;
    }
    if LAST_STALL_WARN_EPOCH
        .compare_exchange(last_warn, now, Ordering::Relaxed, Ordering::Relaxed)
        .is_ok()
    {
        warn!(
            chain_id,
            settled_height,
            candidate_height,
            frontier_age_secs = age,
            "Settlement frontier is stalled with a candidate ahead of it; \
             check for undrained work, incomplete or quarantined branch \
             cleanup jobs, unverified S3 publication, or a stale finalized \
             row awaiting re-ingestion"
        );
    }
}
