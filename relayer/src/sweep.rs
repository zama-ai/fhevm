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
//! this pod's own epoch is never claimed, why `tx_in_flight` is reset to `processing`, and
//! and why `tx_in_flight` is reset to `processing`.

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

/// Rows one tick claims per table. A backlog larger than this is drained over consecutive
/// ticks: each claim stamps the current epoch, so the next tick's predicate skips what this
/// one took and no row is passed over twice.
///
/// Deliberately not configurable. A bound exists so that one statement's duration and lock
/// footprint stay independent of how much work a failover inherits, and so dispatch starts on
/// the first rows rather than after the whole backlog commits; at the default 500 ms interval
/// this drains 200 rows a second, far above any backlog seen. Nothing known would tune it, and
/// the config already carries more knobs than the deployment sets.
const CLAIM_BATCH: i64 = 100;

/// One sweep tick: while the dispatch gate is open, claim incomplete requests and dispatch
/// what was claimed. Returns `(dispatched, dispatch_failed)`.
///
/// `dispatched` counts claim plus hand-off to `dispatch_event`, which spawns handlers
/// detached and returns at once - there is no synchronous signal for a downstream CAS loss,
/// so such a row still counts. `dispatch_failed` counts claims that produced no event, which
/// takes a malformed `int_job_id`. A no-op returning zeros while the gate is closed.
async fn run_tick(
    repositories: &Arc<Repositories>,
    orchestrator: &Arc<Orchestrator>,
    gate: &DispatchGate,
) -> anyhow::Result<(usize, usize)> {
    let Some(epoch) = gate.epoch() else {
        // Not the dispatcher, or holding without a minted epoch yet (a short window right after
        // acquisition, self-healing within one heartbeat interval). Nothing to claim with.
        debug!("Sweep tick skipped: dispatch gate closed");
        return Ok((0, 0));
    };

    let mut dispatched = 0usize;
    let mut dispatch_failed = 0usize;

    for row in furthest_along_first(
        repositories
            .public_decrypt
            .claim_incomplete_requests(epoch, CLAIM_BATCH)
            .await?,
        |row| row.status,
    ) {
        if dispatch_recovered_public_decrypt(orchestrator, row.int_job_id, row.req, row.status)
            .await
        {
            dispatched += 1;
        } else {
            dispatch_failed += 1;
        }
    }

    for row in furthest_along_first(
        repositories
            .user_decrypt
            .claim_incomplete_requests(epoch, CLAIM_BATCH)
            .await?,
        |row| row.status,
    ) {
        if dispatch_recovered_user_decrypt(
            orchestrator,
            row.int_job_id,
            row.req,
            row.req_type,
            row.status,
        )
        .await
        {
            dispatched += 1;
        } else {
            dispatch_failed += 1;
        }
    }

    for row in furthest_along_first(
        repositories
            .input_proof
            .claim_incomplete_requests(epoch, CLAIM_BATCH)
            .await?,
        |row| row.status,
    ) {
        if dispatch_recovered_input_proof(orchestrator, row.int_job_id, row.req).await {
            dispatched += 1;
        } else {
            dispatch_failed += 1;
        }
    }

    Ok((dispatched, dispatch_failed))
}

/// Dispatch the rows closest to a receipt first: `processing`, then `queued`. A claim rewrites
/// `tx_in_flight` to `processing` in the same `UPDATE` and returns the post-update status, so
/// `processing` covers both the rows that were mid-send and those that had only passed their
/// readiness check - a row that may have a send in flight is the one whose duplicate costs a
/// fee, so it should not queue behind fresh work. An `UPDATE ... RETURNING` has no defined row
/// order, so the sort happens here rather than in SQL.
///
/// Orders within one [`CLAIM_BATCH`], not across a whole backlog: a `processing` row can fall
/// into a later batch than a `queued` row. Only the dispatch order shifts - the fence, not the
/// ordering, is what keeps a possible in-flight send safe.
fn furthest_along_first<T>(mut rows: Vec<T>, status_of: impl Fn(&T) -> ReqStatus) -> Vec<T> {
    rows.sort_by_key(|row| match status_of(row) {
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

        match run_tick(&repositories, &orchestrator, &gate).await {
            Ok((0, 0)) => {
                debug!(step = %WorkerStep::TickCompleted, worker = "sweep", "Tick complete, nothing to claim");
            }
            Ok((dispatched, dispatch_failed)) => {
                info!(
                    step = %WorkerStep::RowsProcessed,
                    worker = "sweep",
                    dispatched,
                    dispatch_failed,
                    "Sweep tick claimed and re-dispatched requests"
                );
                if dispatch_failed > 0 {
                    // Claimed, then no event built from the row - see `run_tick`'s doc comment.
                    // Such a row is re-claimed on every later epoch and never progresses.
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
