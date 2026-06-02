//! Shared GCS activation state used by the host-listener in `--gcs-mode`.
//!
//! Mirrors the pattern in `tfhe-worker`: a long-lived task LISTENs on
//! `event_upgrade_activated` and mirrors `upgrade_state.start_block` (for
//! `stack_role='GCS'`) into a shared atomic. The ingest path checks this
//! atomic to decide whether the listener is paused (sentinel) or activated
//! (real value). Once activated, writes are routed to the `gcs` schema by
//! the connection's `search_path = gcs,public`, not by per-statement
//! table-name swaps.

use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use sqlx::{postgres::PgListener, Pool, Postgres};
use tracing::{debug, error, info, warn};

/// Wake-up channel for GCS activation. Must stay in sync with
/// `upgrade_controller::UPGRADE_ACTIVATED_CHANNEL`.
pub const EVENT_UPGRADE_ACTIVATED: &str = "event_upgrade_activated";

/// Sentinel for the activation atomic: the GCS row in `upgrade_state` has not
/// been observed yet (the listener is paused). Any non-sentinel value is the
/// real start_block.
pub const GCS_NOT_ACTIVATED: i64 = -1;

/// Returns a fresh `Arc<AtomicI64>` initialized to `GCS_NOT_ACTIVATED`.
pub fn new_state() -> Arc<AtomicI64> {
    Arc::new(AtomicI64::new(GCS_NOT_ACTIVATED))
}

/// True iff the listener is running in GCS mode and the activation
/// watcher has populated a real `start_block`.
pub fn is_active(state: &AtomicI64) -> bool {
    state.load(Ordering::SeqCst) != GCS_NOT_ACTIVATED
}

/// Spawns the long-lived activation watcher. No-op when `gcs_mode` is false.
pub fn spawn_watcher(gcs_mode: bool, pool: Pool<Postgres>, state: Arc<AtomicI64>) {
    if !gcs_mode {
        return;
    }
    tokio::spawn(async move {
        loop {
            if let Err(err) = watch_gcs_activation(&pool, &state).await {
                error!(
                    target: "host_listener",
                    error = %err,
                    "GCS activation watcher errored; restarting in 5s"
                );
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    });
}

/// LISTENs on `event_upgrade_activated` and mirrors
/// `upgrade_state.start_block` (for `stack_role='GCS'`) into `state`. Polls
/// once on entry so a worker started AFTER the activation notify fired still
/// picks the value up. A 30s fallback sleep catches any missed NOTIFY.
async fn watch_gcs_activation(
    pool: &Pool<Postgres>,
    state: &AtomicI64,
) -> Result<(), sqlx::Error> {
    let mut listener = PgListener::connect_with(pool).await?;
    listener.listen(EVENT_UPGRADE_ACTIVATED).await?;
    info!(
        target: "host_listener",
        channel = EVENT_UPGRADE_ACTIVATED,
        "GCS activation watcher listening"
    );

    loop {
        let row: Option<(Option<i64>,)> =
            sqlx::query_as("SELECT start_block FROM upgrade_state WHERE stack_role = 'GCS'")
                .fetch_optional(pool)
                .await?;

        if let Some((Some(start_block),)) = row {
            let prev = state.swap(start_block, Ordering::SeqCst);
            if prev != start_block {
                info!(
                    target: "host_listener",
                    start_block,
                    prev,
                    "GCS start_block updated from upgrade_state"
                );
            }
        } else {
            debug!(
                target: "host_listener",
                "GCS row in upgrade_state has no start_block yet"
            );
        }

        tokio::select! {
            recv = listener.recv() => {
                if let Err(err) = recv {
                    warn!(
                        target: "host_listener",
                        error = %err,
                        "GCS activation listener recv error"
                    );
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
            _ = tokio::time::sleep(Duration::from_secs(30)) => {}
        }
    }
}
