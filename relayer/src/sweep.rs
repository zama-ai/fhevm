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
//! this module. `update_status_to_tx_in_flight` is this codebase's example of getting that
//! wrong - a compare-and-set whose `rows_affected` every caller discarded, so two callers
//! could both believe they had won and both send. This module never dispatches a row it did
//! not get back from the claim `UPDATE`.
//!
//! # Not double-driving a row already in flight in-process
//!
//! At one replica (or on the current holder's own traffic at any replica count), the HTTP
//! handler that inserts a `queued` row also calls [`crate::orchestrator::Orchestrator::dispatch_event`]
//! for it in the same request, and nothing yet stamps `owner_epoch` at intake (that is later
//! work - see the `owner_epoch` migration's comment). So the claim query cannot tell "nobody
//! is driving this" from "the in-process handler is driving this and just hasn't updated the
//! row yet" by `owner_epoch` alone; the only signal available without changing intake is time.
//! `SweepConfig::claim_after` is that guard: a row is only claimable once its `updated_at` is
//! older than `claim_after`, which must be set above the slowest readiness-check retry budget
//! configured for this deployment - shorter, and the sweep would race live in-process work
//! under ordinary load, not just after a crash. This is not a hard mutual-exclusion guarantee
//! (a readiness check that runs unusually long could still be claimed and re-dispatched
//! alongside itself); it degrades to the same tolerated duplicate every other recovery path in
//! this codebase already accepts (see `startup.rs`'s shutdown-rationale comment: a duplicate
//! send costs a fee and duplicate KMS work, never a wrong result), not a correctness bug.

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
/// dispatch what was claimed. Returns `(claimed, failed)` counts for logging. A no-op (both
/// zero, no queries issued) when this pod does not hold the lock, or when it holds it but has
/// not yet minted an epoch (a short window right after acquisition - see
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
    let mut claimed = 0usize;
    let mut failed = 0u64;

    failed += repositories
        .public_decrypt
        .fail_exhausted_attempts(
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
            claimed += 1;
        }
    }

    failed += repositories
        .user_decrypt
        .fail_exhausted_attempts(
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
            claimed += 1;
        }
    }

    failed += repositories
        .input_proof
        .fail_exhausted_attempts(
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
            claimed += 1;
        }
    }

    Ok((claimed, failed))
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
            Ok((claimed, failed)) => {
                info!(
                    step = %WorkerStep::RowsProcessed,
                    worker = "sweep",
                    claimed,
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
