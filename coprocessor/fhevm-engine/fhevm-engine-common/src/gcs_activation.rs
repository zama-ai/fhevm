//! Shared GCS activation gate for blue/green worker fleets.
//!
//! A GCS (green) worker must not begin computing until the upgrade-controller
//! has moved the GCS `upgrade_state` row into `DryRunStarted` — i.e. after the
//! BCS (blue) stack has settled up to `start_block` and pre-start rows have been
//! pruned from the GCS schema. Each worker holds an `AtomicI64` seeded with
//! [`GCS_NOT_ACTIVATED`] and parks its work loop until
//! [`run_gcs_activation_watcher`] mirrors the GCS `start_block` into it.

use std::sync::atomic::{AtomicI64, Ordering};
use std::time::Duration;

use sqlx::postgres::PgListener;
use sqlx::{Pool, Postgres};
use tracing::{info, warn};

/// pg_notify channel the upgrade-controller fires when the GCS `upgrade_state`
/// row is created in `UpgradeActivated`. Workers wake on it but stay paused
/// until the row reaches `DryRunStarted`.
pub const EVENT_UPGRADE_ACTIVATED: &str = "event_upgrade_activated";

/// pg_notify channel the upgrade-controller fires when the GCS row enters
/// `DryRunStarted`, releasing the GCS-fleet workers.
pub const EVENT_DRY_RUN_STARTED: &str = "event_dry_run_started";

/// pg_notify channel the `gw-listener` fires on each ingested Gateway block
/// (carrying the new `last_block_num`). The upgrade-controller's Gateway-side
/// readiness task wakes on it to re-check whether the GCS gw-listener has
/// reached `gw_start_block`.
pub const EVENT_GW_NEW_BLOCK: &str = "event_gw_new_block";

/// pg_notify channel the upgrade-controller fires once the GCS gw-listener has
/// reached `gw_start_block` and the pre-start rows have been pruned from
/// `gcs.verify_proofs`, releasing the GCS `zkproof-worker` into its dry-run /
/// new re-randomization strategy. The host-chain `EVENT_DRY_RUN_STARTED`
/// releases the tfhe-/sns-workers; this one is the Gateway-keyed analogue,
/// scoped to the zkproof-worker.
pub const EVENT_GW_DRY_RUN_STARTED: &str = "event_gw_dry_run_started";

/// Fired when a dry-run is rolled back (unanimity timeout); worker watchers wake
/// on it to re-pause without waiting for the fallback poll.
pub const EVENT_DRY_RUN_ROLLED_BACK: &str = "event_dry_run_rolled_back";

/// Sentinel for the activation atomic: the GCS row has not yet been observed in
/// `DryRunStarted`. Any other value is the real `start_block`.
pub const GCS_NOT_ACTIVATED: i64 = -1;

/// Pause flag for the tfhe/sns workers: run from `start_block` while dry-running,
/// keep the released value through cutover (`UpgradeAuthorized`/`LIVE`), pause on
/// any other state — including a fresh re-proposal (`UpgradeActivated`) that races
/// in after a rollback, so the worker never runs before the new `DryRunStarted`.
fn host_gate_value(state: &str, start_block: Option<i64>, current: i64) -> i64 {
    match state {
        "DryRunStarted" => start_block.unwrap_or(current),
        "UpgradeAuthorized" | "LIVE" => current,
        _ => GCS_NOT_ACTIVATED,
    }
}

/// Pause flag for the zkproof-worker: run from `gw_start_block` once the Gateway
/// gate clears, keep the released value through cutover, pause on any other state
/// (including a racing re-proposal before its gate clears).
fn gw_gate_value(
    state: &str,
    gw_dry_run_started: bool,
    gw_start_block: Option<i64>,
    current: i64,
) -> i64 {
    if gw_dry_run_started {
        gw_start_block.unwrap_or(current)
    } else if state == "UpgradeAuthorized" || state == "LIVE" {
        current
    } else {
        GCS_NOT_ACTIVATED
    }
}

/// LISTENs on [`EVENT_UPGRADE_ACTIVATED`] / [`EVENT_DRY_RUN_STARTED`] and
/// releases the worker only once the GCS `upgrade_state` row reaches
/// `DryRunStarted`, mirroring its `start_block` into `state`. Polls once on
/// entry (so a worker started after the notify fired still picks it up) and
/// every 30s as a fallback for a missed NOTIFY. Runs until an unrecoverable
/// error; callers spawn it in a restart-on-error loop.
pub async fn run_gcs_activation_watcher(
    pool: &Pool<Postgres>,
    state: &AtomicI64,
) -> Result<(), sqlx::Error> {
    let mut listener = PgListener::connect_with(pool).await?;
    listener.listen(EVENT_UPGRADE_ACTIVATED).await?;
    listener.listen(EVENT_DRY_RUN_STARTED).await?;
    listener.listen(EVENT_DRY_RUN_ROLLED_BACK).await?;
    info!(
        target: "gcs_activation",
        channel = EVENT_DRY_RUN_STARTED,
        "GCS activation watcher listening"
    );

    loop {
        // Release the worker only once the GCS row is in `DryRunStarted`. The
        // `start_block` column is populated one state earlier (UpgradeActivated),
        // before BCS has settled up to start_block and before pre-start rows are
        // pruned — too early to begin computing.
        let row: Option<(String, Option<i64>)> =
            sqlx::query_as("SELECT state, start_block FROM upgrade_state WHERE stack_role = 'GCS'")
                .fetch_optional(pool)
                .await?;

        let current = state.load(Ordering::SeqCst);
        let next = row.as_ref().map_or(current, |(fsm_state, start_block)| {
            host_gate_value(fsm_state, *start_block, current)
        });
        if next != current {
            state.store(next, Ordering::SeqCst);
            if next == GCS_NOT_ACTIVATED {
                info!(
                    target: "gcs_activation",
                    prev = current,
                    "GCS dry-run rolled back; re-pausing worker until next start_block"
                );
            } else {
                info!(
                    target: "gcs_activation",
                    start_block = next,
                    "GCS DryRunStarted observed; releasing worker at start_block"
                );
            }
        }

        // Fallback poll catches a missed NOTIFY (dropped connection, late start).
        tokio::select! {
            recv = listener.recv() => {
                if let Err(err) = recv {
                    warn!(target: "gcs_activation", error = %err, "GCS activation listener recv error");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
            _ = tokio::time::sleep(Duration::from_secs(30)) => {}
        }
    }
}

/// Gateway-side activation watcher for the GCS `zkproof-worker`.
///
/// The zkproof-worker re-randomizes verified input ciphertexts, and a
/// consensus-breaking upgrade can change that strategy. All operators must
/// switch at the same *Gateway* block, so the worker releases off
/// `gw_start_block` rather than the host-chain `start_block` other GCS workers
/// use. LISTENs on [`EVENT_UPGRADE_ACTIVATED`] / [`EVENT_GW_DRY_RUN_STARTED`]
/// and releases the worker only once the GCS `upgrade_state` row has
/// `gw_dry_run_started = TRUE` — the durable marker the upgrade-controller sets
/// after the GCS gw-listener reaches `gw_start_block` and pre-start rows are
/// pruned from `gcs.verify_proofs`. Mirrors `gw_start_block` into `state`.
/// Polls on entry and every 30s as a fallback for a missed NOTIFY. Runs until
/// an unrecoverable error; callers spawn it in a restart-on-error loop.
pub async fn run_gcs_gw_activation_watcher(
    pool: &Pool<Postgres>,
    state: &AtomicI64,
) -> Result<(), sqlx::Error> {
    let mut listener = PgListener::connect_with(pool).await?;
    listener.listen(EVENT_UPGRADE_ACTIVATED).await?;
    listener.listen(EVENT_GW_DRY_RUN_STARTED).await?;
    listener.listen(EVENT_DRY_RUN_ROLLED_BACK).await?;
    info!(
        target: "gcs_activation",
        channel = EVENT_GW_DRY_RUN_STARTED,
        "GCS gateway activation watcher listening"
    );

    loop {
        // Release the worker only once the controller has marked
        // `gw_dry_run_started` (set after the GCS gw-listener reached
        // `gw_start_block` and pre-start proofs were pruned). The
        // `gw_start_block` column itself is populated one state earlier (at
        // `UpgradeActivated`), before the GCS gw-listener has caught up — too
        // early to begin re-randomizing. A `PAUSED` row (rolled-back dry-run)
        // re-pauses the worker.
        let row: Option<(String, bool, Option<i64>)> = sqlx::query_as(
            "SELECT state, gw_dry_run_started, gw_start_block FROM upgrade_state WHERE stack_role = 'GCS'",
        )
        .fetch_optional(pool)
        .await?;

        let current = state.load(Ordering::SeqCst);
        let next = row
            .as_ref()
            .map_or(current, |(fsm_state, gw_started, gw_start_block)| {
                gw_gate_value(fsm_state, *gw_started, *gw_start_block, current)
            });
        if next != current {
            state.store(next, Ordering::SeqCst);
            if next == GCS_NOT_ACTIVATED {
                info!(
                    target: "gcs_activation",
                    prev = current,
                    "GCS dry-run rolled back; re-pausing zkproof-worker until next gw_start_block"
                );
            } else {
                info!(
                    target: "gcs_activation",
                    gw_start_block = next,
                    "GCS gw_dry_run_started observed; releasing zkproof-worker at gw_start_block"
                );
            }
        }

        // Fallback poll catches a missed NOTIFY (dropped connection, late start).
        tokio::select! {
            recv = listener.recv() => {
                if let Err(err) = recv {
                    warn!(target: "gcs_activation", error = %err, "GCS gateway activation listener recv error");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
            _ = tokio::time::sleep(Duration::from_secs(30)) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Retry lifecycle: activate -> release -> rollback re-pauses -> stays paused
    /// until the next window -> cutover keeps it live.
    #[test]
    fn gw_gate_repauses_on_rollback_then_holds_until_next_window() {
        assert_eq!(
            gw_gate_value("UpgradeActivated", false, Some(100), GCS_NOT_ACTIVATED),
            GCS_NOT_ACTIVATED
        );
        assert_eq!(
            gw_gate_value("DryRunStarted", true, Some(100), GCS_NOT_ACTIVATED),
            100
        );
        assert_eq!(
            gw_gate_value("PAUSED", false, Some(100), 100),
            GCS_NOT_ACTIVATED
        );
        assert_eq!(
            gw_gate_value("DryRunStarted", false, Some(200), GCS_NOT_ACTIVATED),
            GCS_NOT_ACTIVATED
        );
        assert_eq!(
            gw_gate_value("DryRunStarted", true, Some(200), GCS_NOT_ACTIVATED),
            200
        );
        assert_eq!(gw_gate_value("LIVE", true, Some(200), 200), 200);
        assert_eq!(
            gw_gate_value("UpgradeActivated", false, Some(200), 100),
            GCS_NOT_ACTIVATED
        );
    }

    /// Same lifecycle for the host gate (tfhe/sns workers).
    #[test]
    fn host_gate_repauses_on_rollback_then_holds_until_next_window() {
        assert_eq!(
            host_gate_value("UpgradeActivated", Some(100), GCS_NOT_ACTIVATED),
            GCS_NOT_ACTIVATED
        );
        assert_eq!(
            host_gate_value("DryRunStarted", Some(100), GCS_NOT_ACTIVATED),
            100
        );
        assert_eq!(host_gate_value("PAUSED", Some(100), 100), GCS_NOT_ACTIVATED);
        assert_eq!(
            host_gate_value("UpgradeActivated", Some(200), GCS_NOT_ACTIVATED),
            GCS_NOT_ACTIVATED
        );
        assert_eq!(
            host_gate_value("DryRunStarted", Some(200), GCS_NOT_ACTIVATED),
            200
        );
        assert_eq!(host_gate_value("LIVE", Some(200), 200), 200);
        assert_eq!(
            host_gate_value("UpgradeActivated", Some(200), 100),
            GCS_NOT_ACTIVATED
        );
    }
}
