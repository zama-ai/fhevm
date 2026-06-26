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
use tracing::{debug, info, warn};

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

/// Sentinel for the activation atomic: the GCS row has not yet been observed in
/// `DryRunStarted`. Any other value is the real `start_block`.
pub const GCS_NOT_ACTIVATED: i64 = -1;

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

        if let Some((fsm_state, Some(start_block))) = row {
            if fsm_state == "DryRunStarted" {
                let prev = state.swap(start_block, Ordering::SeqCst);
                if prev != start_block {
                    info!(
                        target: "gcs_activation",
                        start_block,
                        prev,
                        "GCS DryRunStarted observed; releasing worker at start_block"
                    );
                }
            } else {
                debug!(
                    target: "gcs_activation",
                    state = %fsm_state,
                    "GCS not yet in DryRunStarted; worker remains paused"
                );
            }
        } else {
            debug!(
                target: "gcs_activation",
                "GCS row in upgrade_state has no start_block yet"
            );
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
        // early to begin re-randomizing.
        let row: Option<(bool, Option<i64>)> = sqlx::query_as(
            "SELECT gw_dry_run_started, gw_start_block FROM upgrade_state WHERE stack_role = 'GCS'",
        )
        .fetch_optional(pool)
        .await?;

        if let Some((true, Some(gw_start_block))) = row {
            let prev = state.swap(gw_start_block, Ordering::SeqCst);
            if prev != gw_start_block {
                info!(
                    target: "gcs_activation",
                    gw_start_block,
                    prev,
                    "GCS gw_dry_run_started observed; releasing zkproof-worker at gw_start_block"
                );
            }
        } else {
            debug!(
                target: "gcs_activation",
                "GCS gw_dry_run_started not yet set; zkproof-worker remains paused"
            );
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
