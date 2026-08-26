//! The dispatch sweep (build-order step 6).
//!
//! Once dispatch is gated on the dispatcher lock (step 7), a non-holder that accepts an HTTP
//! request only persists it to `queued` - it must not dispatch in-process, since it might not
//! be the pod driving the rest of that request's lifecycle. Nothing then picks the row up: the
//! only Postgres -> dispatch path in this codebase is one-shot startup recovery
//! ([`crate::startup_recovery`]), which only runs once, at process start. This module is the
//! missing path: while this pod holds the dispatcher lock, it periodically finds incomplete
//! requests nobody is driving, claims them, and re-dispatches through the same mechanism
//! startup recovery uses ([`crate::startup_recovery::dispatch_recovered_public_decrypt`] and
//! its two siblings), so there is one re-dispatch mechanism rather than two.
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
//! At one replica (or on the current holder's own traffic at any replica count), the HTTP
//! handler that inserts a `queued` row also calls [`crate::orchestrator::Orchestrator::dispatch_event`]
//! for it in the same request. Since build-order step 8, intake stamps `owner_epoch` with the
//! inserting pod's current epoch (`NULL` if it is not the holder), and the claim query
//! (`PublicDecryptRepository::claim_incomplete_requests` and its two siblings) is two-tier
//! because of it:
//!
//! - A row owned by a *strictly older* epoch than this pod's current one - never `NULL`, see
//!   below - can only belong to a predecessor that is, by construction, no longer the live
//!   holder: epochs are minted only on actual lock acquisition and are monotonic across the
//!   whole database, so an older one can never still be current. It is claimed immediately,
//!   with no staleness window: the epoch fence on every write the dispatched handler goes on to
//!   make is what makes this safe even if that predecessor has not noticed it is dead yet and
//!   is still writing - not the timing of the claim itself.
//! - A row owned by `NULL` or by this pod's own current epoch shares the *other* branch, and
//!   `updated_at`-based timing still guards both: a `NULL` owner can be a *live* non-holder pod
//!   driving its own accepted traffic in-process (not a dead one, until dispatch is gated on
//!   the lock in step 7), and a row already under this pod's own epoch may genuinely still be
//!   in progress (a readiness check, a tx send) without having touched `updated_at` recently -
//!   reclaiming either early would just re-dispatch live work on every tick.
//!   `SweepConfig::claim_after` bounds this branch - see its doc comment for what it is set
//!   against, and for why that bound covers sleeps but not a stalled RPC call. This is not a
//!   hard mutual-exclusion guarantee even so; it degrades to the same tolerated duplicate every
//!   other recovery path in this codebase already accepts (see `startup.rs`'s shutdown-rationale
//!   comment: a duplicate send costs a fee and duplicate KMS work, never a wrong result), not a
//!   correctness bug.
//!
//! A row claimed out of `tx_in_flight` from either branch is also reset to `processing` in the
//! same statement - see `claim_incomplete_requests`'s doc comment for why: without it,
//! `on_tx_in_flight`'s CAS (which requires `processing`) refuses every re-dispatch, the row
//! exhausts its attempts and is failed out, and a transaction that may already have succeeded
//! on chain is orphaned. `attempts` itself is never reset by a claim, including across a change
//! of owner - see the same doc comment for why a reset-on-takeover design was tried and
//! reverted (it let a crash-looping pod, minting a fresh epoch on every restart, retry a row
//! forever).

use std::{sync::Arc, time::Duration};

use futures::FutureExt;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

use crate::{
    config::settings::SweepConfig,
    logging::WorkerStep,
    orchestrator::{DispatcherLock, LockState, Orchestrator},
    startup_recovery::{
        dispatch_recovered_input_proof, dispatch_recovered_public_decrypt,
        dispatch_recovered_user_decrypt,
    },
    store::sql::repositories::Repositories,
};

/// Recorded on a row that exhausted `max_attempts` without completing (see
/// [`run_tick`]'s call into `fail_exhausted_attempts`).
const EXHAUSTED_ATTEMPTS_ERR_REASON: &str =
    "Exceeded maximum sweep re-dispatch attempts without completing";

/// One sweep tick: while holding the lock, fail out exhausted rows, claim the rest, and
/// dispatch what was claimed. Returns `(dispatched, failed)` counts for logging. `dispatched`
/// counts a successful claim *and* a successful hand-off to
/// [`crate::orchestrator::Orchestrator::dispatch_event`] - not a confirmed re-drive.
/// `dispatch_event` only reports whether the event was queued
/// ([`crate::orchestrator::TokioEventDispatcher::dispatch_event`] spawns the handler detached
/// and returns immediately), so a claimed row whose handler chain later loses a downstream CAS
/// (e.g. `on_tx_in_flight` racing a send this same claim's `tx_in_flight` reset just started)
/// still counts here; there is no synchronous signal for that outcome to count instead. A
/// no-op (both zero, no queries issued) when this pod does not hold the lock, or when it holds
/// it but has not yet minted an epoch (a short window right after acquisition - see
/// `DispatcherLock::current_epoch`'s doc comment).
async fn run_tick(
    repositories: &Arc<Repositories>,
    orchestrator: &Arc<Orchestrator>,
    dispatcher_lock: &DispatcherLock,
    config: &SweepConfig,
) -> anyhow::Result<(usize, u64)> {
    if dispatcher_lock.state() != LockState::Held {
        return Ok((0, 0));
    }
    let Some(epoch) = dispatcher_lock.current_epoch() else {
        // Held but not yet minted - see the module docs on `DispatcherLock`. Rare and
        // self-healing within one heartbeat interval; nothing to claim with yet.
        debug!("Sweep tick skipped: lock held but epoch not yet minted");
        return Ok((0, 0));
    };

    let claim_after_secs = config.claim_after.as_secs_f64();
    let mut dispatched = 0usize;
    let mut failed = 0u64;

    failed += repositories
        .public_decrypt
        .fail_exhausted_attempts(
            epoch,
            config.max_attempts,
            claim_after_secs,
            EXHAUSTED_ATTEMPTS_ERR_REASON,
        )
        .await?;
    for (int_job_id, req_json, status, _attempts) in repositories
        .public_decrypt
        .claim_incomplete_requests(epoch, config.max_attempts, claim_after_secs)
        .await?
    {
        if dispatch_recovered_public_decrypt(orchestrator, int_job_id, req_json, status).await {
            dispatched += 1;
        }
    }

    failed += repositories
        .user_decrypt
        .fail_exhausted_attempts(
            epoch,
            config.max_attempts,
            claim_after_secs,
            EXHAUSTED_ATTEMPTS_ERR_REASON,
        )
        .await?;
    for (int_job_id, req_json, status, _attempts) in repositories
        .user_decrypt
        .claim_incomplete_requests(epoch, config.max_attempts, claim_after_secs)
        .await?
    {
        if dispatch_recovered_user_decrypt(orchestrator, int_job_id, req_json, status).await {
            dispatched += 1;
        }
    }

    failed += repositories
        .input_proof
        .fail_exhausted_attempts(
            epoch,
            config.max_attempts,
            claim_after_secs,
            EXHAUSTED_ATTEMPTS_ERR_REASON,
        )
        .await?;
    for (int_job_id, req_json, _status, _attempts) in repositories
        .input_proof
        .claim_incomplete_requests(epoch, config.max_attempts, claim_after_secs)
        .await?
    {
        if dispatch_recovered_input_proof(orchestrator, int_job_id, req_json).await {
            dispatched += 1;
        }
    }

    Ok((dispatched, failed))
}

async fn run_sweep_worker_logic(
    repositories: Arc<Repositories>,
    orchestrator: Arc<Orchestrator>,
    dispatcher_lock: DispatcherLock,
    config: SweepConfig,
    shutdown: CancellationToken,
) {
    let mut interval = tokio::time::interval(config.interval);
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    debug!(
        interval_ms = config.interval.as_millis() as u64,
        claim_after_secs = config.claim_after.as_secs_f64(),
        max_attempts = config.max_attempts,
        "Sweep worker initialized"
    );

    loop {
        tokio::select! {
            _ = interval.tick() => {}
            _ = shutdown.cancelled() => return,
        }

        match run_tick(&repositories, &orchestrator, &dispatcher_lock, &config).await {
            Ok((0, 0)) => {
                debug!(step = %WorkerStep::TickCompleted, worker = "sweep", "Tick complete, nothing to claim");
            }
            Ok((dispatched, failed)) => {
                info!(
                    step = %WorkerStep::RowsProcessed,
                    worker = "sweep",
                    dispatched,
                    failed_exhausted = failed,
                    "Sweep tick claimed and re-dispatched requests"
                );
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
    dispatcher_lock: DispatcherLock,
    config: SweepConfig,
    shutdown: CancellationToken,
) {
    loop {
        info!(step = %WorkerStep::WorkerStarted, worker = "sweep", "Worker started");

        let result = std::panic::AssertUnwindSafe(async {
            run_sweep_worker_logic(
                repositories.clone(),
                orchestrator.clone(),
                dispatcher_lock.clone(),
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
