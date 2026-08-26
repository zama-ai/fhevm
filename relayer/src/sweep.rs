//! The dispatch sweep (build-order step 6), and since step 7 the relayer's only
//! Postgres -> dispatch path.
//!
//! Dispatch is gated on the dispatcher lock ([`crate::orchestrator::DispatchGate`]), so a pod
//! that is not the confirmed dispatcher persists an accepted HTTP request to `queued` and
//! drives nothing. Something has to pick those rows up, and it cannot be the pod that accepted
//! them. While this pod holds the lock, this module periodically finds incomplete requests
//! nobody is driving, claims them, and dispatches them through
//! [`crate::startup_recovery::dispatch_recovered_public_decrypt`] and its two siblings.
//!
//! This also subsumes what startup recovery used to do. Recovery re-dispatched every incomplete
//! row at process start *without* claiming it, which was harmless only while the claim needed a
//! staleness window to fire; once a claim takes an unowned or older-epoch row on sight, the two
//! would race and re-dispatch the same row twice on every restart. A restart now recovers by
//! minting a higher epoch: every row the previous incarnation owned becomes "older", and the
//! first tick after acquisition claims the lot. One mechanism, running continuously, instead of
//! one at startup and one after.
//!
//! # Claiming
//!
//! A claim is one atomic `UPDATE ... RETURNING` per table (see
//! `PublicDecryptRepository::claim_incomplete_requests` and its two siblings): it stamps
//! `owner_epoch` and increments `attempts` in the same statement that selects the row, and
//! only the rows that statement actually touched are ever returned to (and dispatched by)
//! this module. This module never dispatches a row it did not get back from the claim
//! `UPDATE`, and every send-decision write the dispatched handler goes on to make is itself
//! fenced against `owner_epoch` (build-order step 8) - so a row this claim wins cannot be
//! written by a stale-epoch holder underneath it either.
//!
//! # Not double-driving a row already in flight in-process
//!
//! Eligibility is `owner_epoch IS NULL OR owner_epoch < $mine`, with no time term. Both cases
//! are provably undriven rather than probably undriven, which is what removed the staleness
//! window an earlier version needed:
//!
//! - A *strictly older* epoch belongs to a predecessor that cannot still be the live holder,
//!   since epochs are minted only on lock acquisition and are monotonic across the database.
//! - `NULL` means intake declined to claim the row, which since step 7 means the accepting pod
//!   also declined to drive it. Before the gate existed, a `NULL` row could be a *live*
//!   non-holder's own in-process work, and claiming it raced a live sender; that is the case
//!   the gate removes.
//!
//! A row under this pod's own current epoch is never claimed here: this pod is driving it, and
//! `updated_at` cannot distinguish "still working" from "silently died" - see
//! `claim_incomplete_requests`'s doc comment for why no window over that signal can be made
//! safe, and for the bounded cost of not having one (a row whose in-process task dies without a
//! terminal write waits for this pod's next restart).
//!
//! A row claimed out of `tx_in_flight` is reset to `processing` in the same statement -
//! without it, `on_tx_in_flight`'s CAS (which requires `processing`) refuses every re-dispatch,
//! the row exhausts its attempts and is failed out, and a transaction that may already have
//! succeeded on chain is orphaned. `attempts` itself is never reset by a claim, including across
//! a change of owner - see the same doc comment for why a reset-on-takeover design was tried and
//! reverted (it let a crash-looping pod, minting a fresh epoch on every restart, retry a row
//! forever).

use std::{sync::Arc, time::Duration};

use futures::FutureExt;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

use crate::{
    config::settings::SweepConfig,
    logging::WorkerStep,
    orchestrator::{DispatchGate, Orchestrator},
    startup_recovery::{
        dispatch_recovered_input_proof, dispatch_recovered_public_decrypt,
        dispatch_recovered_user_decrypt,
    },
    store::sql::{models::req_status_enum_model::ReqStatus, repositories::Repositories},
};

/// Recorded on a row that exhausted `max_attempts` without completing (see
/// [`run_tick`]'s call into `fail_exhausted_attempts`).
const EXHAUSTED_ATTEMPTS_ERR_REASON: &str =
    "Exceeded maximum sweep re-dispatch attempts without completing";

/// One sweep tick: while the dispatch gate is open, fail out exhausted rows, claim the rest,
/// and dispatch what was claimed. Returns `(dispatched, failed)` counts for logging.
///
/// `dispatched` counts a successful claim *and* a successful hand-off to
/// [`crate::orchestrator::Orchestrator::dispatch_event`] - not a confirmed re-drive.
/// `dispatch_event` only reports whether the event was queued
/// ([`crate::orchestrator::TokioEventDispatcher::dispatch_event`] spawns the handler detached
/// and returns immediately), so a claimed row whose handler chain later loses a downstream CAS
/// (e.g. `on_tx_in_flight` racing a send this same claim's `tx_in_flight` reset just started)
/// still counts here; there is no synchronous signal for that outcome to count instead.
///
/// A claim that then fails to hand off is worse than it looks and is reported separately as
/// `dispatch_failed`: the claim already stamped this pod's epoch, and this pod's own epoch is
/// exactly what the next tick will not claim, so the row waits for a restart rather than the
/// next tick. Bounded (a restart mints a higher epoch and sweeps it) and rare (the hand-off is
/// a channel send into a detached spawn), but counted rather than folded into "nothing to do".
///
/// A no-op (both zero, no queries issued) whenever the gate is closed - not the dispatcher, or
/// holding without a minted epoch yet.
async fn run_tick(
    repositories: &Arc<Repositories>,
    orchestrator: &Arc<Orchestrator>,
    gate: &DispatchGate,
    config: &SweepConfig,
) -> anyhow::Result<(usize, u64, usize)> {
    let Some(epoch) = gate.epoch() else {
        // Not the dispatcher, or holding without a minted epoch yet (a short window right after
        // acquisition, self-healing within one heartbeat interval). Nothing to claim with.
        debug!("Sweep tick skipped: dispatch gate closed");
        return Ok((0, 0, 0));
    };

    let mut dispatched = 0usize;
    let mut failed = 0u64;
    let mut dispatch_failed = 0usize;

    failed += repositories
        .public_decrypt
        .fail_exhausted_attempts(epoch, config.max_attempts, EXHAUSTED_ATTEMPTS_ERR_REASON)
        .await?;
    for (int_job_id, req_json, status, _attempts) in furthest_along_first(
        repositories
            .public_decrypt
            .claim_incomplete_requests(epoch, config.max_attempts)
            .await?,
    ) {
        if dispatch_recovered_public_decrypt(orchestrator, int_job_id, req_json, status).await {
            dispatched += 1;
        } else {
            dispatch_failed += 1;
        }
    }

    failed += repositories
        .user_decrypt
        .fail_exhausted_attempts(epoch, config.max_attempts, EXHAUSTED_ATTEMPTS_ERR_REASON)
        .await?;
    for (int_job_id, req_json, status, _attempts) in furthest_along_first(
        repositories
            .user_decrypt
            .claim_incomplete_requests(epoch, config.max_attempts)
            .await?,
    ) {
        if dispatch_recovered_user_decrypt(orchestrator, int_job_id, req_json, status).await {
            dispatched += 1;
        } else {
            dispatch_failed += 1;
        }
    }

    failed += repositories
        .input_proof
        .fail_exhausted_attempts(epoch, config.max_attempts, EXHAUSTED_ATTEMPTS_ERR_REASON)
        .await?;
    for (int_job_id, req_json, _status, _attempts) in furthest_along_first(
        repositories
            .input_proof
            .claim_incomplete_requests(epoch, config.max_attempts)
            .await?,
    ) {
        if dispatch_recovered_input_proof(orchestrator, int_job_id, req_json).await {
            dispatched += 1;
        } else {
            dispatch_failed += 1;
        }
    }

    Ok((dispatched, failed, dispatch_failed))
}

/// Dispatch the rows closest to a receipt first: `tx_in_flight`, then `processing`, then
/// `queued`. Carried over from startup recovery, which sorted the same way before the sweep
/// took over its job - a row mid-send is the one whose duplicate costs a fee, so it should not
/// queue behind fresh work. An `UPDATE ... RETURNING` has no defined row order, so the sort
/// happens here rather than in SQL.
fn furthest_along_first(
    mut rows: Vec<(Vec<u8>, serde_json::Value, ReqStatus, i32)>,
) -> Vec<(Vec<u8>, serde_json::Value, ReqStatus, i32)> {
    rows.sort_by_key(|(_, _, status, _)| match status {
        ReqStatus::TxInFlight => 0,
        ReqStatus::Processing => 1,
        ReqStatus::Queued => 2,
        _ => 3,
    });
    rows
}

async fn run_sweep_worker_logic(
    repositories: Arc<Repositories>,
    orchestrator: Arc<Orchestrator>,
    gate: DispatchGate,
    config: SweepConfig,
    shutdown: CancellationToken,
) {
    let mut interval = tokio::time::interval(config.interval);
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    debug!(
        interval_ms = config.interval.as_millis() as u64,
        max_attempts = config.max_attempts,
        "Sweep worker initialized"
    );

    loop {
        // Wait for the gate rather than ticking against a closed one: on a standby pod that is
        // its whole life, and the tick would otherwise wake twice a second to do nothing.
        tokio::select! {
            _ = gate.wait_open() => {}
            _ = shutdown.cancelled() => return,
        }

        tokio::select! {
            _ = interval.tick() => {}
            _ = shutdown.cancelled() => return,
        }

        match run_tick(&repositories, &orchestrator, &gate, &config).await {
            Ok((0, 0, 0)) => {
                debug!(step = %WorkerStep::TickCompleted, worker = "sweep", "Tick complete, nothing to claim");
            }
            Ok((dispatched, failed, dispatch_failed)) => {
                info!(
                    step = %WorkerStep::RowsProcessed,
                    worker = "sweep",
                    dispatched,
                    failed_exhausted = failed,
                    dispatch_failed,
                    "Sweep tick claimed and re-dispatched requests"
                );
                if dispatch_failed > 0 {
                    // Claimed under this pod's epoch, then not handed off - see `run_tick`'s
                    // doc comment. The row waits for this pod's next restart, so this is worth
                    // more than a count in an info line if it ever recurs.
                    warn!(
                        worker = "sweep",
                        dispatch_failed,
                        "Claimed rows could not be dispatched; they wait for a restart"
                    );
                }
            }
            Err(e) => {
                error!(error = ?e, worker = "sweep", "Database error, retrying next tick");
            }
        }
    }
}

/// Panic-restart supervisor, matching the shape of the other periodic workers in
/// `store::sql::repositories::cron_task`. Cancelled by `dequeue_shutdown` - see the seam
/// named at the shutdown sequence in `startup.rs`.
pub async fn create_sweep_worker_future(
    repositories: Arc<Repositories>,
    orchestrator: Arc<Orchestrator>,
    gate: DispatchGate,
    config: SweepConfig,
    shutdown: CancellationToken,
) {
    loop {
        info!(step = %WorkerStep::WorkerStarted, worker = "sweep", "Worker started");

        let result = std::panic::AssertUnwindSafe(async {
            run_sweep_worker_logic(
                repositories.clone(),
                orchestrator.clone(),
                gate.clone(),
                config.clone(),
                shutdown.clone(),
            )
            .await;
        })
        .catch_unwind()
        .await;

        if shutdown.is_cancelled() {
            info!(step = %WorkerStep::WorkerStopped, worker = "sweep", "Worker stopped (shutdown)");
            return;
        }

        match result {
            Ok(_) => {
                warn!(step = %WorkerStep::WorkerRestarting, worker = "sweep", delay_secs = 5, "Worker stopped unexpectedly, restarting");
            }
            Err(_) => {
                error!(step = %WorkerStep::WorkerPanicked, worker = "sweep", delay_secs = 5, "Worker panicked, restarting");
            }
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
