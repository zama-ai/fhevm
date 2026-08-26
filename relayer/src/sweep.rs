//! The dispatch sweep: the relayer's only Postgres -> dispatch path.
//!
//! Dispatch is gated on the dispatcher lock ([`crate::orchestrator::DispatchGate`]), so a pod
//! that is not the confirmed dispatcher persists an accepted HTTP request to `queued` and
//! drives nothing. Something has to pick those rows up, and it cannot be the pod that accepted
//! them. While this pod holds the lock, this module periodically finds incomplete requests
//! nobody is driving, claims them, and dispatches them through
//! [`crate::startup_recovery::dispatch_recovered_public_decrypt`] and its two siblings.
//!
//! Restart recovery runs through this same path: a restart mints a higher epoch, so every
//! row the previous incarnation owned counts as older and the first tick after acquisition
//! claims the lot. A separate unclaimed startup pass would race this module's claim and
//! drive the same rows twice.
//!
//! # Claiming
//!
//! A claim is one atomic `UPDATE ... RETURNING` per table, and this module never dispatches a
//! row it did not get back from that `UPDATE`; every send-decision write the dispatched handler
//! then makes is itself fenced against `owner_epoch`. What makes the claim safe is all in
//! `PublicDecryptRepository::claim_incomplete_requests`'s doc comment, kept in one place rather
//! than restated here: the eligibility predicate and its lack of a time term, why a row under
//! this pod's own epoch is never claimed, why `tx_in_flight` is reset to `processing`, and why
//! `attempts` is never reset.

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
/// and dispatch what was claimed. Returns `(dispatched, failed, dispatch_failed)`.
///
/// `dispatched` counts claim plus hand-off to `dispatch_event`, which spawns handlers
/// detached and returns at once - there is no synchronous signal for a downstream CAS loss,
/// so such a row still counts. `dispatch_failed` counts claims that produced no event: a
/// poison row (`req` no longer deserializes, or a bad `int_job_id`), re-claimed once per
/// epoch until `max_attempts` fails it out. A no-op returning zeros while the gate is closed.
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

/// Dispatch the rows closest to a receipt first: `processing`, then `queued`. A claim rewrites
/// `tx_in_flight` to `processing` in the same `UPDATE` and returns the post-update status, so
/// `processing` covers both the rows that were mid-send and those that had only passed their
/// readiness check - a row that may have a send in flight is the one whose duplicate costs a
/// fee, so it should not queue behind fresh work. An `UPDATE ... RETURNING` has no defined row
/// order, so the sort happens here rather than in SQL.
fn furthest_along_first(
    mut rows: Vec<(Vec<u8>, serde_json::Value, ReqStatus, i32)>,
) -> Vec<(Vec<u8>, serde_json::Value, ReqStatus, i32)> {
    rows.sort_by_key(|(_, _, status, _)| match status {
        ReqStatus::Processing => 0,
        ReqStatus::Queued => 1,
        _ => 2,
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
                    // Claimed, then no event built from the row - see `run_tick`'s doc comment.
                    // Such a row makes no progress until `max_attempts` fails it out, so this is
                    // worth more than a count in an info line if it ever recurs.
                    warn!(
                        worker = "sweep",
                        dispatch_failed, "Claimed rows could not be turned back into events"
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
