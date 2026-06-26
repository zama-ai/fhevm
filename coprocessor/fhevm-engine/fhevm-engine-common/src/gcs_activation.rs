//! Shared GCS activation state for the coprocessor worker fleet.
//!
//! A long-lived task LISTENs on `event_dry_run_started` (emitted by the
//! upgrade-controller when the GCS row transitions to `DryRunStarted`) and
//! mirrors `upgrade_state.start_block` (for `stack_role='GCS'`) into a shared
//! atomic. Workers (`tfhe-worker`, `zkproof-worker`, `sns-worker`,
//! `transaction-sender`) check this atomic to decide whether they are still
//! paused (sentinel) or released (real `start_block`).
//!
//! Worker writes are routed to the `gcs` schema by the connection's
//! `search_path = gcs,public`, not by per-statement table-name swaps.

use std::sync::atomic::{AtomicI64, Ordering};
use std::time::Duration;

use sqlx::{postgres::PgListener, Pool, Postgres};
use tracing::{debug, info, warn};

/// Wake-up channel for GCS activation: emitted by `upgrade-controller` once
/// the GCS row reaches `DryRunStarted`. Must stay in sync with
/// `upgrade_controller::DRY_RUN_STARTED_CHANNEL`.
pub const EVENT_DRY_RUN_STARTED: &str = "event_dry_run_started";

/// Channel name kept for back-compat with callers (host-listener and the
/// upgrade-controller's `UPGRADE_ACTIVATED_CHANNEL`) that still refer to the
/// initial activation event by this name. The GCS-worker watcher does not
/// LISTEN on it directly — release is gated on `DryRunStarted`.
pub const EVENT_UPGRADE_ACTIVATED: &str = "event_upgrade_activated";

/// Sentinel for the activation atomic: the GCS row in `upgrade_state` has not
/// yet reached `DryRunStarted` (the worker is paused). Any non-sentinel value
/// is the real `start_block`.
pub const GCS_NOT_ACTIVATED: i64 = -1;

/// Polls `upgrade_state` for the GCS row's `start_block` and updates `state`
/// each time the row reaches `DryRunStarted`. Wakes on `event_dry_run_started`
/// notifications; a 30s fallback sleep catches any missed NOTIFY (dropped
/// connection, late start).
pub async fn run_gcs_activation_watcher(
    pool: &Pool<Postgres>,
    state: &AtomicI64,
) -> Result<(), sqlx::Error> {
    let mut listener = PgListener::connect_with(pool).await?;
    listener.listen(EVENT_DRY_RUN_STARTED).await?;
    info!(
        channel = EVENT_DRY_RUN_STARTED,
        "GCS activation watcher listening"
    );

    loop {
        let row: Option<(Option<i64>,)> = sqlx::query_as(
            "SELECT start_block FROM upgrade_state \
             WHERE stack_role = 'GCS' AND state = 'DryRunStarted'",
        )
        .fetch_optional(pool)
        .await?;

        if let Some((Some(start_block),)) = row {
            let prev = state.swap(start_block, Ordering::SeqCst);
            if prev != start_block {
                info!(
                    start_block,
                    prev, "GCS start_block updated from upgrade_state"
                );
            }
        } else {
            debug!("GCS row in upgrade_state is not yet DryRunStarted");
        }

        tokio::select! {
            recv = listener.recv() => {
                if let Err(err) = recv {
                    warn!(error = %err, "GCS activation listener recv error");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
            _ = tokio::time::sleep(Duration::from_secs(30)) => {}
        }
    }
}
