use crate::dependence_chain::{self};
use crate::types::CoprocessorError;
use fhevm_engine_common::database::{
    apply_gcs_mode_search_path, connect_pool_with_options,
    connect_pool_with_options_and_connect_options, resolve_database_url_from_option,
};
use fhevm_engine_common::db_keys::DbKeyCache;
use fhevm_engine_common::gcs_activation::{run_gcs_activation_watcher, GCS_NOT_ACTIVATED};
use fhevm_engine_common::telemetry;
use fhevm_engine_common::tfhe_ops::check_fhe_operand_types;
use fhevm_engine_common::types::{FhevmError, Handle, SupportedFheCiphertexts};
use fhevm_engine_common::versioning::{GcsRollbackPolicy, WriteGuard};
use fhevm_engine_common::{tfhe_ops::current_ciphertext_version, types::SupportedFheOperations};
use itertools::Itertools;
use lazy_static::lazy_static;
use prometheus::{register_histogram, register_int_counter, Histogram, IntCounter};
#[cfg(feature = "gpu")]
use scheduler::dfg::scheduler::GpuExecutionLimiter;
#[cfg(not(feature = "gpu"))]
use scheduler::dfg::types::CompressedCiphertext;
use scheduler::dfg::types::{DFGTxInput, SchedulerError};
use scheduler::dfg::{build_component_nodes, ComponentNode, DFComponentGraph, DFGOp};
use scheduler::dfg::{scheduler::Scheduler, types::DFGTaskInput};
use sqlx::types::Uuid;
use sqlx::{postgres::PgListener, query, Postgres};
use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use time::PrimitiveDateTime;
use tracing::{debug, error, info, warn, Instrument};

const EVENT_CIPHERTEXT_COMPUTED: &str = "event_ciphertext_computed";

#[cfg(feature = "gpu")]
fn is_retryable_gpu_reservation_error(error: &FhevmError) -> bool {
    matches!(
        error,
        FhevmError::GpuMemoryReservationError(
            fhevm_engine_common::gpu_memory::GpuMemoryReservationError::TimedOut { .. }
                | fhevm_engine_common::gpu_memory::GpuMemoryReservationError::Cancelled { .. }
        )
    )
}

#[derive(sqlx::FromRow)]
struct WorkItem {
    output_handle: Vec<u8>,
    dependencies: Vec<Vec<u8>>,
    fhe_operation: i16,
    is_scalar: bool,
    is_allowed: bool,
    transaction_id: Vec<u8>,
    schedule_order: PrimitiveDateTime,
    /// Authoritative, listener-derived executor `boundaryBits`. Nullable at
    /// the schema level only for pre-blue-green historical rows; pending work
    /// without it must never be scheduled.
    operand_boundary_mask: Option<Vec<u8>>,
}

const OPERAND_BOUNDARY_MASK_BYTES: usize = 32;

fn operand_is_boundary(
    mask: Option<&[u8]>,
    operand_index: usize,
) -> Result<bool, CoprocessorError> {
    let mask = mask.ok_or_else(|| {
        CoprocessorError::Other(
            std::io::Error::other(
                "refusing computation without authoritative operand boundary mask",
            )
            .into(),
        )
    })?;
    if mask.len() != OPERAND_BOUNDARY_MASK_BYTES {
        return Err(CoprocessorError::Other(
            std::io::Error::other(format!(
                "invalid operand boundary mask length {}; expected {OPERAND_BOUNDARY_MASK_BYTES}",
                mask.len()
            ))
            .into(),
        ));
    }
    if operand_index >= OPERAND_BOUNDARY_MASK_BYTES * 8 {
        return Err(CoprocessorError::Other(
            std::io::Error::other(format!(
                "operand index {operand_index} exceeds executor boundary mask width"
            ))
            .into(),
        ));
    }
    let byte_index = OPERAND_BOUNDARY_MASK_BYTES - 1 - operand_index / 8;
    Ok(mask[byte_index] & (1 << (operand_index % 8)) != 0)
}

#[cfg(test)]
mod operand_boundary_mask_tests {
    use super::*;
    use clap::Parser;
    use sqlx::postgres::PgPoolOptions;
    use test_harness::instance::{setup_test_db, ImportMode};

    #[test]
    fn boundary_mask_is_big_endian_and_null_fails_closed() {
        let mut mask = [0_u8; OPERAND_BOUNDARY_MASK_BYTES];
        mask[OPERAND_BOUNDARY_MASK_BYTES - 1] = 0b10;
        assert!(!operand_is_boundary(Some(&mask), 0).expect("valid mask"));
        assert!(operand_is_boundary(Some(&mask), 1).expect("valid mask"));
        assert!(operand_is_boundary(None, 0).is_err());
        assert!(operand_is_boundary(Some(&mask[..31]), 0).is_err());
    }

    #[tokio::test]
    async fn acquired_dcid_excludes_stale_same_transaction_producer_for_boundary_operand(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let db = setup_test_db(ImportMode::None).await?;
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect(db.db_url())
            .await?;

        let canonical_dcid = vec![0x11_u8; 32];
        let stale_dcid = vec![0x22_u8; 32];
        let transaction_id = vec![0x33_u8; 32];
        let stale_producer = vec![0x44_u8; 32];
        let canonical_consumer = vec![0x55_u8; 32];

        // Only the canonical component is acquired. The stale row models a
        // fork-exposed database retaining the same transaction under a
        // sibling DCID. The consumer's listener-authored bit says the input
        // is a boundary, so it must be sourced from canonical ciphertext
        // storage rather than forwarded from this stale producer.
        sqlx::query(
            "INSERT INTO dependence_chain (dependence_chain_id, status) VALUES ($1, 'updated')",
        )
        .bind(&canonical_dcid)
        .execute(&pool)
        .await?;
        sqlx::query(
            r#"
            INSERT INTO computations (
                output_handle, dependencies, fhe_operation, is_scalar,
                dependence_chain_id, transaction_id, is_allowed,
                is_completed, is_error, host_chain_id, operand_boundary_mask
            ) VALUES ($1, $2, $3, false, $4, $5, $6, false, false, 1, $7)
            "#,
        )
        .bind(&stale_producer)
        .bind(Vec::<Vec<u8>>::new())
        .bind(SupportedFheOperations::FheTrivialEncrypt as i16)
        .bind(&stale_dcid)
        .bind(&transaction_id)
        .bind(true)
        .bind(vec![0_u8; OPERAND_BOUNDARY_MASK_BYTES])
        .execute(&pool)
        .await?;
        let mut boundary_mask = vec![0_u8; OPERAND_BOUNDARY_MASK_BYTES];
        boundary_mask[OPERAND_BOUNDARY_MASK_BYTES - 1] = 1;
        sqlx::query(
            r#"
            INSERT INTO computations (
                output_handle, dependencies, fhe_operation, is_scalar,
                dependence_chain_id, transaction_id, is_allowed,
                is_completed, is_error, host_chain_id, operand_boundary_mask
            ) VALUES ($1, $2, $3, false, $4, $5, true, false, false, 1, $6)
            "#,
        )
        .bind(&canonical_consumer)
        .bind(vec![stale_producer.clone()])
        .bind(SupportedFheOperations::FheNot as i16)
        .bind(&canonical_dcid)
        .bind(&transaction_id)
        .bind(boundary_mask)
        .execute(&pool)
        .await?;

        let mut args = crate::daemon_cli::Args::parse_from([
            "tfhe-worker",
            "--work-items-batch-size",
            "1",
            "--dependence-chains-per-batch",
            "1",
        ]);
        args.database_url = Some(db.db_url.clone());
        let health_check = crate::health_check::HealthCheck::new(db.db_url.clone());
        let mut locks = dependence_chain::LockMngr::new_with_conf(
            Uuid::new_v4(),
            pool.clone(),
            30,
            false,
            None,
            None,
            None,
        );
        let mut no_progress_cycles = 0;
        let mut transaction = pool.begin().await?;
        let (nodes, _, found_work) = query_for_work(
            &args,
            &health_check,
            &mut transaction,
            &mut locks,
            &mut no_progress_cycles,
        )
        .await?;

        assert!(found_work);
        assert_eq!(nodes.len(), 1, "only the acquired DCID may enter the graph");
        assert_eq!(nodes[0].results, vec![canonical_consumer]);
        assert!(
            nodes[0].inputs.contains_key(&stale_producer),
            "the boundary operand must be fetched canonically even though a stale same-transaction producer exists"
        );

        transaction.rollback().await?;
        Ok(())
    }
}

lazy_static! {
    pub static ref TIMING: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
}

lazy_static! {
    static ref WORKER_ERRORS_COUNTER: IntCounter =
        register_int_counter!("coprocessor_worker_errors", "worker errors encountered").unwrap();
    static ref WORK_ITEMS_POLL_COUNTER: IntCounter = register_int_counter!(
        "coprocessor_work_items_polls",
        "times work items are polled from database"
    )
    .unwrap();
    static ref WORK_ITEMS_NOTIFICATIONS_COUNTER: IntCounter = register_int_counter!(
        "coprocessor_work_items_notifications",
        "times instant notifications for work items received from the database"
    )
    .unwrap();
    static ref WORK_ITEMS_FOUND_COUNTER: IntCounter = register_int_counter!(
        "coprocessor_work_items_found",
        "work items queried from database"
    )
    .unwrap();
    static ref WORK_ITEMS_ERRORS_COUNTER: IntCounter = register_int_counter!(
        "coprocessor_work_items_errors",
        "work items errored out during computation"
    )
    .unwrap();
    static ref WORK_ITEMS_PROCESSED_COUNTER: IntCounter = register_int_counter!(
        "coprocessor_work_items_processed",
        "work items successfully processed and stored in the database"
    )
    .unwrap();
    static ref WORK_ITEMS_QUERY_HISTOGRAM: Histogram = register_histogram!(
        "coprocessor_tfhe_worker_query_work_items_seconds",
        "Histogram of time spent querying work items in tfhe-worker",
        vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.25, 0.5, 0.75, 1.0, 2.0, 5.0, 10.0]
    )
    .unwrap();
}

pub async fn run_tfhe_worker(
    args: crate::daemon_cli::Args,
    health_check: crate::health_check::HealthCheck,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    run_tfhe_worker_with_readiness(args, health_check, None).await
}

pub async fn run_tfhe_worker_with_readiness(
    args: crate::daemon_cli::Args,
    health_check: crate::health_check::HealthCheck,
    readiness: Option<tokio::sync::oneshot::Sender<Result<(), String>>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Determine worker ID to use for the lifetime of this process
    // In case of a failure in tfhe_worker_cycle, the same id must be reused to quickly unlock any held locks
    let worker_id = args.worker_id.unwrap_or(Uuid::new_v4());

    // GCS mode is auto-detected at startup by comparing this binary's
    // compiled-in `STACK_VERSION` against the live `versioning.stack_version`
    // row.
    let db_url = resolve_database_url_from_option(args.database_url.clone())?;
    let gcs_mode = fhevm_engine_common::versioning::resolve_gcs_mode(db_url.as_str())
        .await
        .map_err(|err| {
            error!(target: "tfhe_worker", error = %err, "Failed to resolve gcs_mode from versioning table");
            err
        })?;

    info!(target: "tfhe_worker", worker_id = %worker_id, gcs_mode = gcs_mode, "Starting tfhe-worker service");

    // Shared GCS activation state. `GCS_NOT_ACTIVATED` means the worker is
    // paused (BCS mode keeps this value for the lifetime of the process).
    let start_block_state = Arc::new(AtomicI64::new(GCS_NOT_ACTIVATED));

    if gcs_mode {
        // Long-lived task that mirrors `upgrade_state.start_block` (stack_role
        // = 'GCS') into the atomic, woken by `event_upgrade_activated`. Lives
        // outside the cycle loop so it survives `tfhe_worker_cycle` restarts.
        let (watcher_pool, _refresh) = connect_pool_with_options(
            &db_url,
            sqlx::postgres::PgPoolOptions::new().max_connections(2),
            None,
        )
        .await?;
        let watcher_state = start_block_state.clone();
        tokio::spawn(async move {
            loop {
                if let Err(err) = run_gcs_activation_watcher(&watcher_pool, &watcher_state).await {
                    error!(target: "tfhe_worker", error = %err, "GCS activation watcher errored; restarting in 5s");
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        });
    }

    let mut readiness = readiness;
    loop {
        // here we log the errors and make sure we retry
        if let Err(cycle_error) = tfhe_worker_cycle(
            &args,
            worker_id,
            gcs_mode,
            start_block_state.clone(),
            health_check.clone(),
            &mut readiness,
        )
        .await
        {
            WORKER_ERRORS_COUNTER.inc();
            if cycle_error.is_fatal_connection() {
                error!(target: "tfhe_worker", error = %cycle_error, "Fatal DB connection error; exiting for k8s restart");
                fhevm_engine_common::telemetry::flush();
                std::process::exit(1);
            }
            error!(target: "tfhe_worker", { error = %cycle_error }, "Error in background worker, retrying shortly");
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    }
}

async fn tfhe_worker_cycle(
    args: &crate::daemon_cli::Args,
    worker_id: Uuid,
    gcs_mode: bool,
    start_block_state: Arc<AtomicI64>,
    health_check: crate::health_check::HealthCheck,
    readiness: &mut Option<tokio::sync::oneshot::Sender<Result<(), String>>>,
) -> Result<(), CoprocessorError> {
    let db_url = resolve_database_url_from_option(args.database_url.clone())
        .map_err(|e| CoprocessorError::Other(e.into()))?;
    // In --gcs-mode, every connection in the data-plane pool is pinned to
    // `search_path = gcs,public` so unqualified writes land in `gcs.*` and
    // shared read-only tables (keys, crs, host_chains, upgrade_state, …)
    // still resolve from `public` via fallback.
    let (pool, _pool_refresh_handle) = connect_pool_with_options_and_connect_options(
        &db_url,
        sqlx::postgres::PgPoolOptions::new().max_connections(args.pg_pool_max_connections),
        None,
        apply_gcs_mode_search_path(gcs_mode),
    )
    .await?;

    let db_key_cache =
        DbKeyCache::new(args.key_cache_size).map_err(|e| CoprocessorError::Other(e.into()))?;
    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen("work_available").await?;

    let mut dcid_mngr = dependence_chain::LockMngr::new_with_conf(
        worker_id,
        pool.clone(),
        args.dcid_ttl_sec,
        args.disable_dcid_locking,
        Some(args.dcid_timeslice_sec),
        Some(args.dcid_cleanup_interval_sec),
        Some(args.processed_dcid_ttl_sec),
    );

    // Release all owned locks on startup to avoid stale locks
    dcid_mngr.release_all_owned_locks().await?;
    dcid_mngr.do_cleanup().await?;

    #[cfg(feature = "bench")]
    {
        let _ = db_key_cache
            .fetch_latest_from_pool(&pool)
            .await
            .map_err(|e| CoprocessorError::Other(e.into()))?;
    }
    if let Some(readiness) = readiness.take() {
        // The listener, DCID-acquisition state, and benchmark key cache are
        // installed. This is an in-process readiness point, not merely an
        // observation of an arbitrary database session.
        let _ = readiness.send(Ok(()));
    }
    let mut immediately_poll_more_work = false;
    let mut no_progress_cycles = 0;
    loop {
        // GCS gating: skip the iteration entirely until the activation
        // watcher has populated `start_block` in `upgrade_state` for
        // `stack_role='GCS'`. Once that's observed, the schema-isolated
        // `search_path = gcs,public` on this pool's connections routes all
        // writes to `gcs.*` automatically — we no longer need the actual
        // start_block value inside the cycle. In BCS mode this branch is a
        // no-op.
        if gcs_mode && start_block_state.load(Ordering::SeqCst) == GCS_NOT_ACTIVATED {
            debug!(target: "tfhe_worker", "GCS not yet activated; sleeping before re-check");
            tokio::time::sleep(tokio::time::Duration::from_millis(
                args.worker_polling_interval_ms,
            ))
            .await;
            continue;
        }

        // only if previous iteration had no work done do the wait
        if !immediately_poll_more_work {
            tokio::select! {
                notification = listener.try_recv() => {
                    match notification? {
                        Some(_) => {
                            WORK_ITEMS_NOTIFICATIONS_COUNTER.inc();
                            info!(target: "tfhe_worker", "Received work_available notification from postgres");
                        }
                        None => {
                            // sqlx already reconnected the LISTEN connection; poll for work.
                            warn!(target: "tfhe_worker", "postgres LISTEN connection reset; reconnected");
                        }
                    }
                },
                _ = tokio::time::sleep(tokio::time::Duration::from_millis(args.worker_polling_interval_ms)) => {
                    WORK_ITEMS_POLL_COUNTER.inc();
                    debug!(target: "tfhe_worker", "Polling the database for more work on timer");
                },
            };
        }

        #[cfg(feature = "bench")]
        let now = std::time::SystemTime::now();
        let loop_span = tracing::info_span!("worker_iteration");
        let acq_span = tracing::info_span!(
            parent: &loop_span,
            "acquire_connection"
        );
        let mut conn = pool.acquire().instrument(acq_span).await?;
        // Begin a write tx under the shared controller lock. A retired BCS stack
        // stops here; a GCS worker skips while a rolled-back dry-run is PAUSED.
        // Holding the lock for the transaction keeps cutover and schema reset
        // from overlapping its reads and writes. Shared locks still allow worker
        // replicas to run concurrently. The state check runs after taking the
        // lock. See versioning::begin_write_guarded.
        let txn_span = tracing::info_span!(parent: &loop_span, "begin_transaction");
        let mut trx = match fhevm_engine_common::versioning::begin_write_guarded_conn(
            &mut conn,
            gcs_mode,
            GcsRollbackPolicy::Skip,
        )
        .instrument(txn_span)
        .await?
        {
            WriteGuard::Proceed(trx) => trx,
            WriteGuard::Stop => {
                info!(target: "tfhe_worker", "Cutover completed — BCS worker exiting cycle");
                return Ok(());
            }
            WriteGuard::Skip => {
                debug!(target: "tfhe_worker", "GCS dry-run rolled back — skipping cycle");
                immediately_poll_more_work = false;
                continue;
            }
        };

        // Query for transactions to execute
        let (mut transactions, _, has_more_work) = query_for_work(
            args,
            &health_check,
            &mut trx,
            &mut dcid_mngr,
            &mut no_progress_cycles,
        )
        .instrument(loop_span.clone())
        .await?;
        if has_more_work {
            // We've fetched work, so we'll poll again without waiting
            // for a notification after this cycle.
            immediately_poll_more_work = true;
        } else {
            dcid_mngr.release_current_lock(true, None).await?;
            dcid_mngr.do_cleanup().await?;
            no_progress_cycles = 0;

            // Lock another dependence chain if available and
            // continue processing without waiting for notification
            let dcid_span = tracing::info_span!(
                parent: &loop_span,
                "query_dependence_chain",
                dependence_chain_id = tracing::field::Empty
            );

            let (dependence_chain_id, _) = dcid_mngr
                .acquire_next_lock()
                .instrument(dcid_span.clone())
                .await?;
            immediately_poll_more_work = dependence_chain_id.is_some();

            dcid_span.record(
                "dependence_chain_id",
                tracing::field::display(
                    dependence_chain_id
                        .as_ref()
                        .map(hex::encode)
                        .unwrap_or_else(|| "none".to_string()),
                ),
            );
            continue;
        }

        if dcid_mngr
            .extend_or_release_current_lock(false)
            .await?
            .is_none()
        {
            // This is a best-effort lease. A competing worker may have
            // acquired an expired DCID before this extension; continuing is
            // safe because result materialization is deterministic, although
            // it can redundantly execute the batch.
            if dcid_mngr.enabled() {
                warn!(target: "tfhe_worker", "Lost dcid lock before processing transactions; continuing with potentially redundant work");
            }
        }

        let mut tx_graph = build_transaction_graph_and_execute(
            &mut transactions,
            db_key_cache.clone(),
            &health_check,
            &mut trx,
            &dcid_mngr,
            gcs_mode,
            std::time::Duration::from_millis(args.gpu_memory_reservation_timeout_ms),
            args.gpu_streams_per_device,
        )
        .instrument(loop_span.clone())
        .await?;
        // A component can outlive the normal lease TTL. Renew once more
        // immediately before persistence to minimize the window in which a
        // second worker can steal and repeat completed FHE work. We do not
        // cancel after a lost lease: duplicate deterministic work is safer
        // and substantially simpler than interrupting an in-flight batch.
        if dcid_mngr
            .extend_or_release_current_lock(false)
            .await?
            .is_none()
            && dcid_mngr.enabled()
        {
            warn!(target: "tfhe_worker", "Lost dcid lock during transaction execution; persisting deterministic results may be redundant");
        }
        let has_progressed =
            upload_transaction_graph_results(&mut tx_graph, &mut trx, &mut dcid_mngr)
                .instrument(loop_span.clone())
                .await?;
        if has_progressed {
            no_progress_cycles = 0;
        } else {
            no_progress_cycles += 1;
            if no_progress_cycles >= args.dcid_max_no_progress_cycles {
                // Stop extending this chain's lock so another chain can run.
                // The parked chain remains pending and can be work-stolen
                // after its existing lock TTL expires.
                info!(target: "tfhe_worker", "no progress on dependence chain, parking until lock expiry");
                dcid_mngr.park_current_lock();
            }
        }
        trx.commit().await?;
        drop(loop_span);
        #[cfg(feature = "bench")]
        {
            let prev_cycle_time = TIMING.load(std::sync::atomic::Ordering::SeqCst);
            TIMING.store(
                now.elapsed().unwrap().as_micros() as u64 + prev_cycle_time,
                std::sync::atomic::Ordering::SeqCst,
            );
        }
    }
}

#[allow(clippy::type_complexity)]
#[tracing::instrument(name = "query_ciphertext_batch", skip_all, fields(count = cts_to_query.len()))]
async fn query_ciphertexts<'a>(
    cts_to_query: &[Vec<u8>],
    trx: &mut sqlx::Transaction<'a, Postgres>,
    gcs_mode: bool,
) -> Result<HashMap<Vec<u8>, (i16, Vec<u8>)>, CoprocessorError> {
    // BCS: the connection's `search_path = public`, so the unqualified
    // `ciphertexts` resolves to `public.ciphertexts` directly. Done in one
    // query.
    //
    // GCS: the connection's `search_path = gcs,public`, so unqualified
    // `ciphertexts` resolves to `gcs.ciphertexts` — the GCS-owned table
    // populated post-activation. Pre-snapshot ciphertexts (produced by BCS
    // before activation) still live in `public.ciphertexts` and must be
    // fetched explicitly. We do this as a two-step query: try GCS first,
    // then fetch the missing handles from `public.ciphertexts`.
    //
    // The public fallback is *block-gated*: `public.ciphertexts` is the live
    // BCS table and keeps growing throughout the dry-run, so an unbounded read
    // could surface a ciphertext BCS produced *after* the snapshot point.
    // Importing such post-start state (which differs across operators and, for
    // a breaking upgrade, differs byte-for-byte from GCS's own re-derivation)
    // would break the consensus gate. We therefore serve a fallback row only
    // when it is not known to have been produced after its track's start block
    // — see the query below.
    let mut ciphertext_map: HashMap<Vec<u8>, (i16, Vec<u8>)> =
        HashMap::with_capacity(cts_to_query.len());

    let rows: Vec<(Vec<u8>, Vec<u8>, i16)> = sqlx::query_as(
        "SELECT handle, ciphertext, ciphertext_type
         FROM ciphertexts
         WHERE handle = ANY($1::BYTEA[])",
    )
    .bind(cts_to_query)
    .fetch_all(trx.as_mut())
    .await
    .map_err(|err| {
        error!(target: "tfhe_worker", { error = %err }, "error while querying ciphertexts");
        err
    })?;
    for (handle, ciphertext, ciphertext_type) in rows {
        let _ = ciphertext_map.insert(handle, (ciphertext_type, ciphertext));
    }

    if gcs_mode {
        let missing: Vec<Vec<u8>> = cts_to_query
            .iter()
            .filter(|h| !ciphertext_map.contains_key(*h))
            .cloned()
            .collect();
        if !missing.is_empty() {
            // Per-chain start-block bounds for this GCS upgrade. `upgrade_state`
            // is a shared (non-duplicated) control-plane table with one row per
            // host chain; read it fully qualified so the result is unambiguous
            // regardless of search_path.
            let windows = sqlx::query!(
                "SELECT host_chain_id, start_block, gw_start_block
                 FROM public.upgrade_state WHERE stack_role = 'GCS'",
            )
            .fetch_all(trx.as_mut())
            .await
            .map_err(|err| {
                error!(target: "tfhe_worker", { error = %err }, "error while reading GCS upgrade_state bounds");
                err
            })?;

            // Fail safe: without at least one window carrying complete bounds we
            // cannot prove a public row is pre-snapshot, so we serve none. The
            // dependent computation then stalls (no consensus) rather than risking
            // divergence — the intended safety behaviour.
            if !windows
                .iter()
                .any(|w| w.start_block.is_some() && w.gw_start_block.is_some())
            {
                warn!(target: "tfhe_worker", { rows = windows.len(), missing = missing.len() },
                    "GCS upgrade_state bounds incomplete; skipping public.ciphertexts fallback");
                return Ok(ciphertext_map);
            }

            // Block-gated fallback into the live `public.ciphertexts`. Fully
            // qualified to bypass search_path. A missing handle is served only
            // if it is NOT known to have been produced after its track's start
            // block. A pre-snapshot row either carries no block lineage
            // (ancient / not-yet-bound, block_number NULL) or lineage at/below
            // the bound; any post-start BCS write carries lineage strictly
            // above it:
            //   - compute outputs -> public.computations.block_number, scoped to
            //     each output's own host chain (its `start_block`, joined by
            //     host_chain_id);
            //   - ZK input ctxts  -> public.input_handles.block_number, which is
            //     the *Gateway* block (`gw_start_block`).
            // Inputs never have a `computations` row and outputs never have an
            // `input_handles` row, so the two guards route by source without an
            // explicit `is_input` branch.
            let rows = sqlx::query!(
                "SELECT c.handle, c.ciphertext, c.ciphertext_type
                 FROM public.ciphertexts c
                 WHERE c.handle = ANY($1::BYTEA[])
                   AND NOT EXISTS (
                       SELECT 1 FROM public.computations comp
                       JOIN public.upgrade_state us
                         ON us.stack_role = 'GCS' AND us.host_chain_id = comp.host_chain_id
                       WHERE comp.output_handle = c.handle
                         AND comp.block_number >= us.start_block)
                   AND NOT EXISTS (
                       SELECT 1 FROM public.input_handles ih
                       WHERE ih.handle = c.handle
                         AND ih.block_number >= (
                             SELECT MIN(gw_start_block) FROM public.upgrade_state WHERE stack_role = 'GCS'))",
                &missing,
            )
            .fetch_all(trx.as_mut())
            .await
            .map_err(|err| {
                error!(target: "tfhe_worker", { error = %err }, "error while querying public.ciphertexts for pre-snapshot handles");
                err
            })?;
            for row in rows {
                let _ = ciphertext_map.insert(row.handle, (row.ciphertext_type, row.ciphertext));
            }

            // Trace every handle the gate withheld — either absent from
            // `public.ciphertexts` or block-gated as post-snapshot. Without this
            // a stalled dry-run (the dependent computation never reaches
            // consensus) gives no hint which handle could not be resolved.
            let withheld: Vec<String> = missing
                .iter()
                .filter(|h| !ciphertext_map.contains_key(*h))
                .map(|h| format!("0x{}", hex::encode(h)))
                .collect();
            if !withheld.is_empty() {
                let gcs_windows: Vec<String> = windows
                    .iter()
                    .map(|w| {
                        format!(
                            "chain {:?} start={:?} gw={:?}",
                            w.host_chain_id, w.start_block, w.gw_start_block
                        )
                    })
                    .collect();
                debug!(target: "tfhe_worker", { gcs_windows = ?gcs_windows, withheld = ?withheld },
                    "GCS public.ciphertexts fallback withheld handles (absent or block-gated as post-snapshot)");
            }
        }
    }

    Ok(ciphertext_map)
}

#[tracing::instrument(skip_all)]
async fn query_for_work<'a>(
    args: &crate::daemon_cli::Args,
    health_check: &crate::health_check::HealthCheck,
    trx: &mut sqlx::Transaction<'a, Postgres>,
    deps_chain_mngr: &mut dependence_chain::LockMngr,
    no_progress_cycles: &mut u32,
) -> Result<(Vec<ComponentNode>, PrimitiveDateTime, bool), CoprocessorError> {
    let s_dcid = tracing::info_span!(
        "query_dependence_chain",
        dependence_chain_id = tracing::field::Empty
    );
    // Lock dependence chain
    let (dependence_chain_ids, locking_reasons) = async {
        let result = match deps_chain_mngr.extend_or_release_current_lock(true).await? {
            // If there is a current lock, we extend it and use its dependence_chain_id
            Some((_id, reason)) => {
                let mut ids = deps_chain_mngr.get_current_lock_ids();
                let mut reasons = vec![reason];
                // A held set must not starve newly-ready chains: without this,
                // chains becoming ready while a long batch is in flight wait
                // for the ENTIRE current set to drain before their first
                // acquisition, serializing independent work behind it.
                if args.dcid_batch_execution {
                    let headroom = args.dependence_chains_per_batch - ids.len() as i32;
                    if headroom > 0 {
                        for (id, joined_reason) in deps_chain_mngr
                            .acquire_next_locks(headroom)
                            .await?
                            .into_iter()
                        {
                            if let Some(id) = id {
                                ids.push(id);
                                reasons.push(joined_reason);
                            }
                        }
                    }
                }
                (ids, reasons)
            }
            None => {
                if *no_progress_cycles
                    < args.dcid_ignore_dependency_count_threshold * args.dcid_max_no_progress_cycles
                {
                    if args.dcid_batch_execution {
                        deps_chain_mngr
                            .acquire_next_locks(args.dependence_chains_per_batch)
                            .await?
                            .into_iter()
                            .filter_map(|(id, reason)| id.map(|id| (id, reason)))
                            .unzip()
                    } else {
                        let (id, reason) = deps_chain_mngr.acquire_next_lock().await?;
                        id.map(|id| (vec![id], vec![reason]))
                            .unwrap_or_else(|| (vec![], vec![]))
                    }
                } else {
                    *no_progress_cycles = 0;
                    let (id, reason) = deps_chain_mngr.acquire_early_lock().await?;
                    id.map(|id| (vec![id], vec![reason]))
                        .unwrap_or_else(|| (vec![], vec![]))
                }
            }
        };
        Ok::<_, CoprocessorError>(result)
    }
    .instrument(s_dcid.clone())
    .await?;
    if deps_chain_mngr.enabled() && dependence_chain_ids.is_empty() {
        // No dependence chain to lock, so no work to do
        health_check.update_db_access();
        health_check.update_activity();
        info!(target: "tfhe_worker", "No dcid found to process");
        return Ok((vec![], PrimitiveDateTime::MAX, false));
    }
    s_dcid.record(
        "dependence_chain_id",
        tracing::field::display(
            dependence_chain_ids
                .first()
                .map(hex::encode)
                .unwrap_or_else(|| "none".to_string()),
        ),
    );
    let s_work = tracing::info_span!("query_work_items", count = tracing::field::Empty);
    let transaction_batch_size = args.work_items_batch_size;
    let started_at = SystemTime::now();
    // Schema isolation: BCS connects with `search_path = public`, GCS with
    // `search_path = gcs,public`. Unqualified `computations` therefore
    // resolves to the stack's own schema. No table-name swaps needed in code.
    // With locking disabled, retain the historical all-ready-DCID query. A
    // batch-enabled worker instead binds all of its fenced DCIDs.
    let dcid_filter = deps_chain_mngr
        .enabled()
        .then(|| dependence_chain_ids.clone());
    let the_work = sqlx::query_as::<_, WorkItem>(
        "
-- Acquire all computations from a transaction set
SELECT
  c.output_handle,
  c.dependencies,
  c.fhe_operation,
  c.is_scalar,
  c.is_allowed,
  c.transaction_id,
  c.schedule_order,
  c.operand_boundary_mask
FROM computations c
WHERE c.transaction_id IN (
    SELECT DISTINCT
      c_schedule_order.transaction_id
    FROM (
      SELECT transaction_id
      FROM computations
      WHERE is_completed = FALSE
        AND is_error = FALSE
        AND is_allowed = TRUE
        AND ($1::bytea[] IS NULL OR dependence_chain_id = ANY($1))
      ORDER BY schedule_order ASC
      LIMIT $2
    ) as c_schedule_order
  )
  -- The transaction-id expansion above is only a convenience for loading
  -- non-allowed intermediates. Re-apply ownership here: a fork-exposed DB
  -- can retain a stale row with the same transaction hash in another DCID,
  -- and it must not enter this worker's graph.
  AND ($1::bytea[] IS NULL OR c.dependence_chain_id = ANY($1))
        ",
    )
    .bind(dcid_filter.as_deref())
    .bind(transaction_batch_size)
    .fetch_all(trx.as_mut())
    .instrument(s_work.clone())
    .await
    .map_err(|err| {
        error!(target: "tfhe_worker", { error = %err }, "error while querying work items");
        err
    })?;

    WORK_ITEMS_QUERY_HISTOGRAM.observe(started_at.elapsed().unwrap_or_default().as_secs_f64());
    s_work.record("count", the_work.len());
    health_check.update_db_access();
    if the_work.is_empty() {
        if !dependence_chain_ids.is_empty() {
            info!(target: "tfhe_worker", dcid_count = dependence_chain_ids.len(), locking_count = locking_reasons.len(), "No work items found to process");
        }
        health_check.update_activity();
        return Ok((vec![], PrimitiveDateTime::MAX, false));
    }
    WORK_ITEMS_FOUND_COUNTER.inc_by(the_work.len() as u64);
    info!(target: "tfhe_worker", count = the_work.len(), dcid_count = dependence_chain_ids.len(), locking_count = locking_reasons.len(), "Processing work items");
    let s_prep = tracing::info_span!("prepare_dataflow_graphs", work_items = the_work.len());
    let (transactions, earliest_schedule_order) = async {
        let mut earliest_schedule_order = the_work.first().unwrap().schedule_order;
        // Partition work directly by transaction
        let work_by_transaction: HashMap<Handle, Vec<_>> = the_work
            .into_iter()
            .into_group_map_by(|k| k.transaction_id.clone());
        // Traverse transactions and build transaction nodes
        let mut transactions: Vec<ComponentNode> = vec![];
        for (transaction_id, txwork) in work_by_transaction.iter() {
            let transaction_id: &Vec<u8> = transaction_id;
            let mut ops = vec![];
            'operations: for w in txwork {
                // A nullable column allows blue-green replacement without an
                // unsafe semantic backfill, but it is never valid for work
                // selected by this binary. Validate it before considering
                // any operation so a malformed row fails closed.
                let operand_boundary_mask = w.operand_boundary_mask.as_deref();
                if let Err(e) = operand_is_boundary(operand_boundary_mask, 0) {
                    set_computation_error(
                        &w.output_handle,
                        transaction_id,
                        &e,
                        trx,
                        deps_chain_mngr,
                    )
                    .await?;
                    continue;
                }
                let fhe_op: SupportedFheOperations = match w.fhe_operation.try_into() {
                    Ok(op) => op,
                    Err(e) => {
                        error!(target: "tfhe_worker", { output_handle = ?w.output_handle, transaction_id = ?hex::encode(transaction_id), error = %e, }, "invalid FHE operation ");
                        set_computation_error(
                            &w.output_handle,
                            transaction_id,
                            &e,
                            trx,
                            deps_chain_mngr,
                        )
                        .await?;
                        continue;
                    }
                };
                let mut inputs: Vec<DFGTaskInput> = Vec::with_capacity(w.dependencies.len());
                let mut this_comp_inputs: Vec<Vec<u8>> = Vec::with_capacity(w.dependencies.len());
                let mut is_scalar_op_vec: Vec<bool> = Vec::with_capacity(w.dependencies.len());
                for (idx, dh) in w.dependencies.iter().enumerate() {
                    let is_operand_scalar =
                        fhe_op.is_operand_scalar(w.is_scalar, idx, w.dependencies.len());
                    is_scalar_op_vec.push(is_operand_scalar);
                    this_comp_inputs.push(dh.clone());
                    if is_operand_scalar {
                        inputs.push(DFGTaskInput::Value(SupportedFheCiphertexts::Scalar(
                            dh.clone(),
                        )));
                    } else {
                        match operand_is_boundary(operand_boundary_mask, idx) {
                            Ok(true) => inputs.push(DFGTaskInput::BoundaryDependence(dh.clone())),
                            Ok(false) => inputs.push(DFGTaskInput::LocalDependence(dh.clone())),
                            Err(e) => {
                                set_computation_error(
                                    &w.output_handle,
                                    transaction_id,
                                    &e,
                                    trx,
                                    deps_chain_mngr,
                                )
                                .await?;
                                continue 'operations;
                            }
                        }
                    }
                }
                if let Err(e) =
                    check_fhe_operand_types(w.fhe_operation.into(), &this_comp_inputs, &is_scalar_op_vec)
                {
                    let error = std::io::Error::other(format!("invalid FHE operands: {e}"));
                    set_computation_error(
                        &w.output_handle,
                        transaction_id,
                        &error,
                        trx,
                        deps_chain_mngr,
                    )
                    .await?;
                    continue;
                }
                ops.push(DFGOp {
                    output_handle: w.output_handle.clone(),
                    fhe_op,
                    inputs,
                    is_allowed: w.is_allowed,
                });
                if w.schedule_order < earliest_schedule_order && w.is_allowed {
                    // Only account for allowed to avoid case of reorg
                    // where trivial encrypts will be in collision in
                    // the same transaction and old ones are re-used
                    earliest_schedule_order = w.schedule_order;
                }
            }
            let operation_handles: Vec<_> = ops
                .iter()
                .map(|op| op.output_handle.clone())
                .collect();
            let (mut components, _) = match build_component_nodes(ops, transaction_id) {
                Ok(components) => components,
                Err(e) => {
                    // A malformed transaction-local graph cannot be safely
                    // partially scheduled. Mark its remaining operations
                    // terminal so this DCID cannot restart the entire worker
                    // cycle forever; unrelated transactions continue below.
                    let error_message = format!("invalid transaction graph: {e}");
                    for output_handle in operation_handles {
                        let error = std::io::Error::other(error_message.clone());
                        set_computation_error(
                            &output_handle,
                            transaction_id,
                            &error,
                            trx,
                            deps_chain_mngr,
                        )
                        .await?;
                    }
                    continue;
                }
            };
            transactions.append(&mut components);
        }
        Ok::<_, CoprocessorError>((transactions, earliest_schedule_order))
    }
    .instrument(s_prep)
    .await?;
    Ok((transactions, earliest_schedule_order, true))
}

#[tracing::instrument(name = "build_and_execute", skip_all)]
#[allow(clippy::too_many_arguments)]
async fn build_transaction_graph_and_execute<'a>(
    txs: &mut Vec<ComponentNode>,
    db_key_cache: DbKeyCache,
    health_check: &crate::health_check::HealthCheck,
    trx: &mut sqlx::Transaction<'a, Postgres>,
    dcid_mngr: &dependence_chain::LockMngr,
    gcs_mode: bool,
    gpu_reservation_timeout: std::time::Duration,
    gpu_streams_per_device: usize,
) -> Result<DFComponentGraph, CoprocessorError> {
    let mut tx_graph = DFComponentGraph::default();
    if txs.is_empty() {
        return Ok(tx_graph);
    }
    let _in_flight_work = health_check.begin_work();
    if let Err(e) = tx_graph.build(txs) {
        // If we had an error while building the graph, we don't
        // execute anything and return to allow any set results
        // (essentially errors) to be set in DB.
        warn!(target: "tfhe_worker", { error = %e }, "error while building transaction graph");
        return Ok(tx_graph);
    }
    let cts_to_query = tx_graph.needed_map.keys().cloned().collect::<Vec<_>>();
    let ciphertext_map = query_ciphertexts(&cts_to_query, trx, gcs_mode).await?;
    let fetched_handles: std::collections::HashSet<_> = ciphertext_map.keys().cloned().collect();
    if cts_to_query.len() != fetched_handles.len() {
        if let Some(dcid_lock) = dcid_mngr.get_current_lock() {
            warn!(target: "tfhe_worker", { missing_inputs = ?(cts_to_query.len() - fetched_handles.len()), dcid = %hex::encode(dcid_lock.dependence_chain_id) },
	  "some inputs are missing to execute the dependence chain");
        }
    }
    #[cfg(not(feature = "gpu"))]
    for (handle, (ct_type, mut ct)) in ciphertext_map.into_iter() {
        tx_graph
            .add_input(
                &handle,
                &DFGTxInput::Compressed((
                    CompressedCiphertext {
                        ct_type,
                        ct_bytes: std::mem::take(&mut ct),
                    },
                    true,
                )),
            )
            .map_err(|e| CoprocessorError::Other(e.into()))?;
    }
    // GPU boundary ciphertexts are independent. Materialize them in bounded
    // device-affine lanes instead of serializing every decompression on the
    // partition threads. The injected values are the DECOMPRESSED CANONICAL
    // FORM of the persisted bytes — exactly what each consumer would
    // reconstruct itself — so byte-determinism is unaffected; sharing one
    // decompression per boundary is a pure win when several transactions
    // consume the same handle. The scheduler fetches the same cached key
    // again for execution, preserving the existing key lifecycle.
    #[cfg(feature = "gpu")]
    {
        let gpu_sks_for_materialize = match db_key_cache.fetch_latest(trx.as_mut()).await {
            Ok(keys) => keys.gpu_sks,
            Err(err) => {
                let cerr: CoprocessorError = match err.downcast::<sqlx::Error>() {
                    Ok(sqlx_err) => sqlx_err.into(),
                    Err(other) => CoprocessorError::MissingKeys {
                        reason: other.to_string(),
                    },
                };
                error!(target: "tfhe_worker", { error = %cerr }, "failed to fetch latest key for GPU boundary materialization");
                telemetry::set_current_span_error(&cerr);
                WORKER_ERRORS_COUNTER.inc();
                return Err(cerr);
            }
        };
        if gpu_sks_for_materialize.is_empty() {
            return Err(CoprocessorError::Other(
                std::io::Error::other("no GPU server keys available").into(),
            ));
        }
        let lane_count = ciphertext_map
            .len()
            .min(gpu_sks_for_materialize.len() * gpu_streams_per_device);
        let coordinator_key = gpu_sks_for_materialize[0].clone();
        let cancellation = tokio_util::sync::CancellationToken::new();
        let materialized = tokio::task::spawn_blocking(move || {
            if lane_count == 0 {
                return Ok::<Vec<(Handle, DFGTxInput)>, Box<dyn std::error::Error + Send + Sync>>(
                    Vec::new(),
                );
            }
            let mut lanes: Vec<Vec<_>> = (0..lane_count).map(|_| Vec::new()).collect();
            for (index, ciphertext) in ciphertext_map.into_iter().enumerate() {
                lanes[index % lane_count].push(ciphertext);
            }
            std::thread::scope(|scope| {
                let mut workers = Vec::with_capacity(lane_count);
                for (lane_index, lane) in lanes.into_iter().enumerate() {
                    let gpu_idx = lane_index % gpu_sks_for_materialize.len();
                    let sks = gpu_sks_for_materialize[gpu_idx].clone();
                    let cancellation = cancellation.clone();
                    workers.push(scope.spawn(
                        move || -> Result<_, Box<dyn std::error::Error + Send + Sync>> {
                            tfhe::set_server_key(sks);
                            let mut result = Vec::with_capacity(lane.len());
                            for (handle, (ct_type, ct)) in lane {
                                result.push((
                                    handle,
                                    DFGTxInput::Value((
                                        SupportedFheCiphertexts::decompress(
                                            ct_type,
                                            &ct,
                                            gpu_idx,
                                            &cancellation,
                                            gpu_reservation_timeout,
                                        )?,
                                        true,
                                    )),
                                ));
                            }
                            Ok(result)
                        },
                    ));
                }
                let mut result = Vec::new();
                for worker in workers {
                    result.extend(worker.join().map_err(|_| {
                        Box::<dyn std::error::Error + Send + Sync>::from(std::io::Error::other(
                            "GPU boundary materialization worker panicked",
                        ))
                    })??);
                }
                Ok(result)
            })
        })
        .await
        .map_err(|e| CoprocessorError::Other(e.into()))?
        .map_err(CoprocessorError::Other)?;
        // The lanes ran on blocking CUDA threads; this async coordinator may
        // be on a thread with no thread-local CUDA key. `add_input` clones
        // GPU ciphertexts while routing the same boundary to every consumer,
        // so install a key before that clone boundary. The scheduler later
        // moves each operation-local clone to its selected execution device.
        tfhe::set_server_key(coordinator_key);
        for (handle, input) in materialized {
            tx_graph
                .add_input(&handle, &input)
                .map_err(|e| CoprocessorError::Other(e.into()))?;
        }
    }
    // Resolve deferred cross-transaction dependences: edges whose
    // handle was fetched from DB are dropped (data already available),
    // remaining edges are added after cycle detection.
    if let Err(e) = tx_graph.resolve_dependences(&fetched_handles) {
        warn!(target: "tfhe_worker", { error = %e }, "error resolving cross-transaction dependences");
        return Ok(tx_graph);
    }
    // Execute the DFG
    let s_compute = tracing::info_span!("compute_fhe_ops");
    async {
        // Fetch the latest key from the database
        let keys = match db_key_cache.fetch_latest(trx.as_mut()).await {
            Ok(k) => k,
            Err(err) => {
                // Extract the sqlx error from anyhow so it classifies as a
                // fatal connection (fail fast) instead of looking like missing keys.
                let cerr: CoprocessorError = match err.downcast::<sqlx::Error>() {
                    Ok(sqlx_err) => sqlx_err.into(),
                    Err(other) => CoprocessorError::MissingKeys {
                        reason: other.to_string(),
                    },
                };
                error!(target: "tfhe_worker", { error = %cerr }, "failed to fetch latest key");
                telemetry::set_current_span_error(&cerr);
                WORKER_ERRORS_COUNTER.inc();
                return Err(cerr);
            }
        };

        // Bound concurrent GPU partitions to the configured stream capacity.
        // The limiter is process-wide and sized once from the visible device
        // count; the blocking tasks release their permits themselves.
        #[cfg(feature = "gpu")]
        let gpu_execution_limiter = {
            type LimiterKey = (usize, usize);
            static LIMITERS: std::sync::LazyLock<
                std::sync::Mutex<std::collections::HashMap<LimiterKey, GpuExecutionLimiter>>,
            > = std::sync::LazyLock::new(
                || std::sync::Mutex::new(std::collections::HashMap::new()),
            );
            let key = (keys.gpu_sks.len(), gpu_streams_per_device);
            let mut limiters = LIMITERS.lock().map_err(|_| {
                CoprocessorError::Other(
                    std::io::Error::other("GPU limiter registry poisoned").into(),
                )
            })?;
            if let Some(limiter) = limiters.get(&key) {
                limiter.clone()
            } else {
                let limiter = GpuExecutionLimiter::new(key.0, key.1)
                    .map_err(|e| CoprocessorError::Other(e.into()))?;
                limiters.insert(key, limiter.clone());
                limiter
            }
        };
        #[cfg(not(feature = "gpu"))]
        let _ = gpu_streams_per_device;

        // Schedule computations in parallel as dependences allow
        tfhe::set_server_key(keys.sks.clone());
        let mut sched = Scheduler::new(
            &mut tx_graph,
            #[cfg(not(feature = "gpu"))]
            keys.sks.clone(),
            keys.pks.clone(),
            #[cfg(feature = "gpu")]
            keys.gpu_sks.clone(),
            #[cfg(feature = "gpu")]
            gpu_execution_limiter,
            health_check.activity_heartbeat.clone(),
            // The worker has no per-batch cancellation; the token exists so a
            // GPU memory reservation wait can be interrupted when one is
            // introduced, and reservation waits stay bounded by the timeout.
            tokio_util::sync::CancellationToken::new(),
            gpu_reservation_timeout,
        );
        sched
            .schedule()
            .await
            .map_err(|e| CoprocessorError::Other(e.into()))?;
        Ok::<(), CoprocessorError>(())
    }
    .instrument(s_compute)
    .await?;
    Ok(tx_graph)
}

#[tracing::instrument(name = "upload_results", skip_all)]
async fn upload_transaction_graph_results<'a>(
    tx_graph: &mut DFComponentGraph,
    trx: &mut sqlx::Transaction<'a, Postgres>,
    deps_mngr: &mut dependence_chain::LockMngr,
) -> Result<bool, CoprocessorError> {
    // Schema isolation: the connection's `search_path` already routes
    // unqualified writes to the stack's own schema (`public` for BCS,
    // `gcs` for GCS post-activation). The two-step ciphertext read in
    // `query_ciphertexts` is the only place where the cross-schema fallback
    // is explicit.
    // Get computation results
    let graph_results = tx_graph.get_results();
    let mut handles_to_update = vec![];
    let mut res = false;

    // Traverse computations that have been scheduled and
    // upload their results/errors.
    let mut cts_to_insert = vec![];
    for result in graph_results.into_iter() {
        match result.compressed_ct {
            Ok(cct) => {
                cts_to_insert.push((
                    result.handle.clone(),
                    (cct.ct_bytes, (current_ciphertext_version(), cct.ct_type)),
                ));
                handles_to_update.push((result.handle.clone(), result.transaction_id.clone()));
                WORK_ITEMS_PROCESSED_COUNTER.inc();
            }
            Err(mut err) => {
                #[cfg(feature = "gpu")]
                if matches!(
                    err.downcast_ref::<FhevmError>(),
                    Some(error) if is_retryable_gpu_reservation_error(error)
                ) {
                    warn!(
                        target: "tfhe_worker",
                        error = %err,
                        output_handle = %format!("0x{}", hex::encode(&result.handle)),
                        "transient GPU memory reservation failure; leaving computation pending for retry"
                    );
                    continue;
                }
                let cerr: Box<dyn std::error::Error + Send + Sync> =
                    if let Some(fhevm_error) = err.downcast_mut::<FhevmError>() {
                        let mut swap_val = FhevmError::BadInputs;
                        std::mem::swap(fhevm_error, &mut swap_val);
                        CoprocessorError::FhevmError(swap_val).into()
                    } else {
                        CoprocessorError::SchedulerError(
                            err.downcast_ref::<SchedulerError>()
                                .cloned()
                                .unwrap_or(SchedulerError::SchedulerError),
                        )
                        .into()
                    };
                // Downgrade SchedulerError to warning when the
                // error is not about the operations themselves.
                // Do not set the error flag in the DB in such cases.
                if let Some(err) = cerr.downcast_ref::<CoprocessorError>() {
                    if matches!(
                        err,
                        CoprocessorError::SchedulerError(SchedulerError::DataflowGraphError)
                    ) || matches!(
                        err,
                        CoprocessorError::SchedulerError(SchedulerError::SchedulerError)
                    ) {
                        warn!(target: "tfhe_worker",
                                          { error = cerr,
                        output_handle = format!("0x{}", hex::encode(&result.handle)) },
                                        "scheduler encountered an error while processing work item"
                                    );
                        continue;
                    }
                    if matches!(
                        err,
                        CoprocessorError::SchedulerError(SchedulerError::MissingInputs)
                    ) {
                        // Make sure we don't mark this as an error since this simply means that the
                        // inputs weren't available when we tried scheduling these operations.
                        continue;
                    }
                }
                set_computation_error(
                    &result.handle,
                    &result.transaction_id,
                    &*cerr,
                    trx,
                    deps_mngr,
                )
                .await?;
            }
        }
    }
    if !cts_to_insert.is_empty() {
        let s_insert = tracing::info_span!("insert_ct_into_db", count = cts_to_insert.len());
        let cts_inserted = async {
            #[allow(clippy::type_complexity)]
            let (handles, (ciphertexts, (ciphertext_versions, ciphertext_types))): (
                Vec<_>,
                (Vec<_>, (Vec<_>, Vec<_>)),
            ) = cts_to_insert.into_iter().unzip();
            let cts_inserted = sqlx::query!(
                "INSERT INTO ciphertexts(handle, ciphertext, ciphertext_version, ciphertext_type)
                 SELECT * FROM UNNEST($1::BYTEA[], $2::BYTEA[], $3::SMALLINT[], $4::SMALLINT[])
                 ON CONFLICT (handle, ciphertext_version) DO NOTHING",
                &handles,
                &ciphertexts,
                &ciphertext_versions,
                &ciphertext_types,
            )
            .execute(trx.as_mut())
            .await.map_err(|err| {
                error!(target: "tfhe_worker", { error = %err }, "error while inserting new ciphertexts");
                err
            })?.rows_affected();
            // Notify all workers that new ciphertext is inserted
            // For now, it's only the SnS workers that are listening for these events
            let _ = sqlx::query!("SELECT pg_notify($1, '')", EVENT_CIPHERTEXT_COMPUTED)
                .execute(trx.as_mut())
                .await?;
            Ok::<u64, CoprocessorError>(cts_inserted)
        }
        .instrument(s_insert)
        .await?;
        res |= cts_inserted > 0;
    }

    if !handles_to_update.is_empty() {
        let s_update = tracing::info_span!("update_computation", count = handles_to_update.len());
        let comp_updated = async {
            let (handles_vec, txn_ids_vec): (Vec<_>, Vec<_>) = handles_to_update.into_iter().unzip();
            let comp_updated = query!(
                "
            UPDATE computations
            SET is_completed = true, completed_at = CURRENT_TIMESTAMP
            WHERE is_completed = false
            AND (output_handle, transaction_id) IN (
                SELECT * FROM unnest($1::BYTEA[], $2::BYTEA[])
            )
            ",
                &handles_vec,
                &txn_ids_vec
            )
            .execute(trx.as_mut())
            .await.map_err(|err| {
                error!(target: "tfhe_worker", { error = %err }, "error while updating computations as completed");
                err
            })?.rows_affected();
            Ok::<u64, CoprocessorError>(comp_updated)
        }
        .instrument(s_update)
        .await?;
        res |= comp_updated > 0;
    }
    Ok(res)
}

#[cfg(all(test, feature = "gpu"))]
mod gpu_reservation_error_tests {
    use super::*;
    use fhevm_engine_common::gpu_memory::GpuMemoryReservationError;

    #[test]
    fn timeout_and_cancellation_are_retryable() {
        for reservation_error in [
            GpuMemoryReservationError::TimedOut {
                gpu_idx: 0,
                amount: 1,
                waited_ms: 10,
            },
            GpuMemoryReservationError::Cancelled {
                gpu_idx: 0,
                amount: 1,
            },
        ] {
            assert!(is_retryable_gpu_reservation_error(
                &FhevmError::GpuMemoryReservationError(reservation_error)
            ));
        }
    }

    #[test]
    fn invariant_failures_are_not_retryable() {
        for reservation_error in [
            GpuMemoryReservationError::UnknownDevice { gpu_idx: 7 },
            GpuMemoryReservationError::AccountingOverflow { gpu_idx: 0 },
        ] {
            assert!(!is_retryable_gpu_reservation_error(
                &FhevmError::GpuMemoryReservationError(reservation_error)
            ));
        }
    }
}

#[tracing::instrument(skip_all)]
async fn set_computation_error<'a>(
    output_handle: &[u8],
    transaction_id: &[u8],
    cerr: &(dyn std::error::Error + Send + Sync),
    trx: &mut sqlx::Transaction<'a, Postgres>,
    deps_mngr: &mut dependence_chain::LockMngr,
) -> Result<(), CoprocessorError> {
    WORKER_ERRORS_COUNTER.inc();
    let err_string = cerr.to_string();
    error!(target: "tfhe_worker", error = %err_string, output_handle = %format!("0x{}", hex::encode(output_handle)), "error while processing work item");
    telemetry::set_current_span_error(&err_string);

    let _ = query!(
        "
        UPDATE computations
        SET is_error = true, error_message = $1
        WHERE output_handle = $2
        AND transaction_id = $3
        ",
        err_string,
        output_handle,
        transaction_id
    )
    .execute(trx.as_mut())
    .await?;

    deps_mngr.set_processing_error(Some(err_string)).await?;
    Ok(())
}
