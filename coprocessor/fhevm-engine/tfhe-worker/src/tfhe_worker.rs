use crate::dependence_chain::{self};
use crate::types::CoprocessorError;
use fhevm_engine_common::database::{connect_pool_with_options, resolve_database_url_from_option};
use fhevm_engine_common::db_keys::{load_active_compressed_key_cutover, DbKeyCache};
use fhevm_engine_common::key_material_policy::{KeyMaterialKind, KeyMaterialUnavailable};
use fhevm_engine_common::telemetry;
use fhevm_engine_common::tfhe_ops::check_fhe_operand_types;
use fhevm_engine_common::types::{FhevmError, Handle, SupportedFheCiphertexts};
use fhevm_engine_common::{tfhe_ops::current_ciphertext_version, types::SupportedFheOperations};
use itertools::Itertools;
use lazy_static::lazy_static;
use prometheus::{register_histogram, register_int_counter, Histogram, IntCounter};
use scheduler::dfg::types::{CompressedCiphertext, DFGTxInput, SchedulerError};
use scheduler::dfg::{build_component_nodes, ComponentNode, DFComponentGraph, DFGOp};
use scheduler::dfg::{scheduler::Scheduler, types::DFGTaskInput};
use sqlx::types::Uuid;
use sqlx::Postgres;
use sqlx::{postgres::PgListener, query, Acquire};
use std::collections::HashMap;
use std::time::SystemTime;
use time::PrimitiveDateTime;
use tracing::{debug, error, info, warn, Instrument};

const EVENT_CIPHERTEXT_COMPUTED: &str = "event_ciphertext_computed";

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
    // Determine worker ID to use for the lifetime of this process
    // In case of a failure in tfhe_worker_cycle, the same id must be reused to quickly unlock any held locks
    let worker_id = args.worker_id.unwrap_or(Uuid::new_v4());
    info!(target: "tfhe_worker", worker_id = %worker_id, "Starting tfhe-worker service");
    loop {
        // here we log the errors and make sure we retry
        if let Err(cycle_error) = tfhe_worker_cycle(&args, worker_id, health_check.clone()).await {
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
    health_check: crate::health_check::HealthCheck,
) -> Result<(), CoprocessorError> {
    let db_url = resolve_database_url_from_option(args.database_url.clone())
        .map_err(|e| CoprocessorError::Other(e.into()))?;
    let (pool, _pool_refresh_handle) = connect_pool_with_options(
        &db_url,
        sqlx::postgres::PgPoolOptions::new().max_connections(args.pg_pool_max_connections),
        None,
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
    let mut immediately_poll_more_work = false;
    let mut no_progress_cycles = 0;
    loop {
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
        let txn_span = tracing::info_span!(parent: &loop_span, "begin_transaction");
        let mut trx = conn.begin().instrument(txn_span).await?;

        // Query for transactions to execute
        let (transactions, block_context, earliest_computation, has_more_work) = query_for_work(
            args,
            &health_check,
            &mut trx,
            &mut dcid_mngr,
            &mut no_progress_cycles,
        )
        .instrument(loop_span.clone())
        .await?;

        // RFC-029: under a scheduled cutover, partition the batch by
        // the material kind selected from each transaction's block
        // anchor, and execute the groups sequentially in a fixed order
        // (legacy first). Without a cutover, everything runs on the
        // default (pre-feature) key-loading path.
        let cutover = load_active_compressed_key_cutover(trx.as_mut())
            .await
            .map_err(|e| CoprocessorError::Other(e.into()))?;
        let mut groups: Vec<(Option<KeyMaterialKind>, Vec<ComponentNode>)> = match &cutover {
            None => vec![(None, transactions)],
            Some(cutover) => {
                let mut legacy = vec![];
                let mut compressed = vec![];
                for node in transactions {
                    let kind = match block_context.get(&node.transaction_id) {
                        // A missing block number predates the feature
                        // and thus any cutover: legacy by construction.
                        Some((chain_id, block_number)) => {
                            cutover.kind_for_host_block(*chain_id, block_number.unwrap_or(0))
                        }
                        None => KeyMaterialKind::Legacy,
                    };
                    match kind {
                        KeyMaterialKind::Legacy => legacy.push(node),
                        KeyMaterialKind::CompressedXof => compressed.push(node),
                    }
                }
                let mut groups = vec![];
                if !legacy.is_empty() {
                    groups.push((Some(KeyMaterialKind::Legacy), legacy));
                }
                if !compressed.is_empty() {
                    groups.push((Some(KeyMaterialKind::CompressedXof), compressed));
                }
                groups
            }
        };
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
            // best-effort attempt to extend the lock and prevent other replicas from trying to lock the same DCID.
            // Worst-case scenario, it returns None if the lock has expired.
            // However, the worker has already secured exclusive access to the txn computations in the Computations table.
            if dcid_mngr.enabled() {
                warn!(target: "tfhe_worker", "Lost dcid lock while processing transactions, but continuing since computations are locked");
            }
        }

        let mut has_progressed = false;
        for (pinned_kind, mut group) in groups.drain(..) {
            let (mut tx_graph, loaded_kind) = build_transaction_graph_and_execute(
                &mut group,
                db_key_cache.clone(),
                &health_check,
                &mut trx,
                &dcid_mngr,
                pinned_kind,
            )
            .instrument(loop_span.clone())
            .await?;
            has_progressed |= upload_transaction_graph_results(
                &mut tx_graph,
                &mut trx,
                &mut dcid_mngr,
                loaded_kind,
            )
            .instrument(loop_span.clone())
            .await?;
        }
        if has_progressed {
            no_progress_cycles = 0;
        } else {
            no_progress_cycles += 1;
            if no_progress_cycles >= args.dcid_max_no_progress_cycles {
                // If we're not making progress on this dependence
                // chain, update the last_updated_at field and
                // release the lock so we can try to execute
                // another chain.
                info!(target: "tfhe_worker", "no progress on dependence chain, releasing");
                dcid_mngr
                    .release_current_lock(false, Some(earliest_computation))
                    .await?;
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
) -> Result<HashMap<Vec<u8>, (i16, Vec<u8>)>, CoprocessorError> {
    // TODO: select all the ciphertexts where they're contained in the tuples
    let ciphertexts_rows = query!(
        "
                SELECT handle, ciphertext, ciphertext_type
                FROM ciphertexts
                WHERE handle = ANY($1::BYTEA[])
            ",
        &cts_to_query
    )
    .fetch_all(trx.as_mut())
    .await
    .map_err(|err| {
        error!(target: "tfhe_worker", { error = %err }, "error while querying ciphertexts");
        err
    })?;
    // index ciphertexts in hashmap
    let mut ciphertext_map: HashMap<Vec<u8>, (i16, Vec<u8>)> =
        HashMap::with_capacity(ciphertexts_rows.len());
    for row in ciphertexts_rows {
        let _ = ciphertext_map.insert(row.handle, (row.ciphertext_type, row.ciphertext));
    }
    Ok(ciphertext_map)
}

/// The (host chain, block) anchor of each transaction in a batch,
/// used to select the key material kind under a scheduled cutover.
type BlockContextByTransaction = HashMap<Vec<u8>, (u64, Option<u64>)>;

#[tracing::instrument(skip_all)]
async fn query_for_work<'a>(
    args: &crate::daemon_cli::Args,
    health_check: &crate::health_check::HealthCheck,
    trx: &mut sqlx::Transaction<'a, Postgres>,
    deps_chain_mngr: &mut dependence_chain::LockMngr,
    no_progress_cycles: &mut u32,
) -> Result<
    (
        Vec<ComponentNode>,
        BlockContextByTransaction,
        PrimitiveDateTime,
        bool,
    ),
    CoprocessorError,
> {
    let s_dcid = tracing::info_span!(
        "query_dependence_chain",
        dependence_chain_id = tracing::field::Empty
    );
    // Lock dependence chain
    let (dependence_chain_id, locking_reason) = async {
        let result = match deps_chain_mngr.extend_or_release_current_lock(true).await? {
            // If there is a current lock, we extend it and use its dependence_chain_id
            Some((id, reason)) => (Some(id), reason),
            None => {
                if *no_progress_cycles
                    < args.dcid_ignore_dependency_count_threshold * args.dcid_max_no_progress_cycles
                {
                    deps_chain_mngr.acquire_next_lock().await?
                } else {
                    *no_progress_cycles = 0;
                    deps_chain_mngr.acquire_early_lock().await?
                }
            }
        };
        Ok::<_, CoprocessorError>(result)
    }
    .instrument(s_dcid.clone())
    .await?;
    if deps_chain_mngr.enabled() && dependence_chain_id.is_none() {
        // No dependence chain to lock, so no work to do
        health_check.update_db_access();
        health_check.update_activity();
        info!(target: "tfhe_worker", "No dcid found to process");
        return Ok((vec![], HashMap::new(), PrimitiveDateTime::MAX, false));
    }
    s_dcid.record(
        "dependence_chain_id",
        tracing::field::display(
            dependence_chain_id
                .as_ref()
                .map(hex::encode)
                .unwrap_or_else(|| "none".to_string()),
        ),
    );
    let s_work = tracing::info_span!("query_work_items", count = tracing::field::Empty);
    let transaction_batch_size = args.work_items_batch_size;
    let started_at = SystemTime::now();
    let the_work = query!(
        "
-- Acquire all computations from a transaction set
SELECT
  c.output_handle, 
  c.dependencies, 
  c.fhe_operation, 
  c.is_scalar,
  c.is_allowed, 
  c.dependence_chain_id,
  c.transaction_id,
  c.schedule_order,
  c.host_chain_id,
  c.block_number
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
        AND ($1::bytea IS NULL OR dependence_chain_id = $1)
      ORDER BY schedule_order ASC
      LIMIT $2
    ) as c_schedule_order
  )
        ",
        dependence_chain_id,
        transaction_batch_size as i32,
    )
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
        if let Some(dependence_chain_id) = &dependence_chain_id {
            info!(target: "tfhe_worker", dcid = %hex::encode(dependence_chain_id), locking = ?locking_reason, "No work items found to process");
        }
        health_check.update_activity();
        return Ok((vec![], HashMap::new(), PrimitiveDateTime::MAX, false));
    }
    WORK_ITEMS_FOUND_COUNTER.inc_by(the_work.len() as u64);
    info!(target: "tfhe_worker", { count = the_work.len(), dcid = ?dependence_chain_id.as_ref().map(hex::encode),
				    locking = ?locking_reason }, "Processing work items");
    let s_prep = tracing::info_span!("prepare_dataflow_graphs", work_items = the_work.len());
    let (transactions, block_context, earliest_schedule_order) = async {
        let mut earliest_schedule_order = the_work.first().unwrap().schedule_order;
        // Partition work directly by transaction
        let work_by_transaction: HashMap<Handle, Vec<_>> = the_work
            .into_iter()
            .into_group_map_by(|k| k.transaction_id.clone());
        // RFC-029: the (host chain, block) anchor of each transaction,
        // used to select the key material kind under a scheduled
        // cutover. All computations of a transaction share a block.
        let block_context: BlockContextByTransaction = work_by_transaction
            .iter()
            .map(|(txid, work)| {
                let w = work.first().expect("non-empty transaction group");
                (
                    txid.clone(),
                    (w.host_chain_id as u64, w.block_number.map(|b| b as u64)),
                )
            })
            .collect();
        // Traverse transactions and build transaction nodes
        let mut transactions: Vec<ComponentNode> = vec![];
        for (transaction_id, txwork) in work_by_transaction.iter() {
            let transaction_id: &Vec<u8> = transaction_id;
            let mut ops = vec![];
            for w in txwork {
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
                        inputs.push(DFGTaskInput::Dependence(dh.clone()));
                    }
                }
                check_fhe_operand_types(w.fhe_operation.into(), &this_comp_inputs, &is_scalar_op_vec)?;
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
            let (mut components, _) = build_component_nodes(ops, transaction_id)
                .map_err(|e| CoprocessorError::Other(e.into()))?;
            transactions.append(&mut components);
        }
        Ok::<_, CoprocessorError>((transactions, block_context, earliest_schedule_order))
    }
    .instrument(s_prep)
    .await?;
    Ok((transactions, block_context, earliest_schedule_order, true))
}

#[tracing::instrument(name = "build_and_execute", skip_all)]
async fn build_transaction_graph_and_execute<'a>(
    txs: &mut Vec<ComponentNode>,
    db_key_cache: DbKeyCache,
    health_check: &crate::health_check::HealthCheck,
    trx: &mut sqlx::Transaction<'a, Postgres>,
    dcid_mngr: &dependence_chain::LockMngr,
    pinned_kind: Option<KeyMaterialKind>,
) -> Result<(DFComponentGraph, KeyMaterialKind), CoprocessorError> {
    let mut tx_graph = DFComponentGraph::default();
    if let Err(e) = tx_graph.build(txs) {
        // If we had an error while building the graph, we don't
        // execute anything and return to allow any set results
        // (essentially errors) to be set in DB.
        warn!(target: "tfhe_worker", { error = %e }, "error while building transaction graph");
        return Ok((tx_graph, KeyMaterialKind::Legacy));
    }
    let cts_to_query = tx_graph.needed_map.keys().cloned().collect::<Vec<_>>();
    let ciphertext_map = query_ciphertexts(&cts_to_query, trx).await?;
    let fetched_handles: std::collections::HashSet<_> = ciphertext_map.keys().cloned().collect();
    if cts_to_query.len() != fetched_handles.len() {
        if let Some(dcid_lock) = dcid_mngr.get_current_lock() {
            warn!(target: "tfhe_worker", { missing_inputs = ?(cts_to_query.len() - fetched_handles.len()), dcid = %hex::encode(dcid_lock.dependence_chain_id) },
	      "some inputs are missing to execute the dependence chain");
        }
    }
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
    // Resolve deferred cross-transaction dependences: edges whose
    // handle was fetched from DB are dropped (data already available),
    // remaining edges are added after cycle detection.
    if let Err(e) = tx_graph.resolve_dependences(&fetched_handles) {
        warn!(target: "tfhe_worker", { error = %e }, "error resolving cross-transaction dependences");
        return Ok((tx_graph, KeyMaterialKind::Legacy));
    }
    // Execute the DFG
    let s_compute = tracing::info_span!("compute_fhe_ops");
    let loaded_kind = async {
        // Fetch the latest key from the database, pinned to the
        // cutover-selected material when a cutover is scheduled.
        let fetched = match pinned_kind {
            Some(kind) => db_key_cache.fetch_latest_pinned(trx.as_mut(), kind).await,
            None => db_key_cache.fetch_latest(trx.as_mut()).await,
        };
        let keys = match fetched {
            Ok(k) => k,
            Err(err) if err.downcast_ref::<KeyMaterialUnavailable>().is_some() => {
                // RFC-029: availability incident, never a consensus
                // one. Fail the cycle so the worker retries shortly;
                // it must never substitute the other material.
                warn!(target: "tfhe_worker", { error = %err }, "selected key material unavailable; retrying");
                return Err(CoprocessorError::MissingKeys {
                    reason: err.to_string(),
                });
            }
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

        // Schedule computations in parallel as dependences allow
        tfhe::set_server_key(keys.sks.clone());
        let mut sched = Scheduler::new(
            &mut tx_graph,
            #[cfg(not(feature = "gpu"))]
            keys.sks.clone(),
            keys.pks.clone(),
            #[cfg(feature = "gpu")]
            keys.gpu_sks.clone(),
            health_check.activity_heartbeat.clone(),
        );
        sched
            .schedule()
            .await
            .map_err(|e| CoprocessorError::Other(e.into()))?;
        Ok::<KeyMaterialKind, CoprocessorError>(keys.material_kind)
    }
    .instrument(s_compute)
    .await?;
    Ok((tx_graph, loaded_kind))
}

#[tracing::instrument(name = "upload_results", skip_all)]
async fn upload_transaction_graph_results<'a>(
    tx_graph: &mut DFComponentGraph,
    trx: &mut sqlx::Transaction<'a, Postgres>,
    deps_mngr: &mut dependence_chain::LockMngr,
    loaded_kind: KeyMaterialKind,
) -> Result<bool, CoprocessorError> {
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
            // Outputs are labeled with the material kind that ACTUALLY
            // produced them (RFC-029) — the kind of the loaded key, so
            // the label stays truthful on no-cutover deployments where
            // the default read resolves to native compressed material.
            let key_material_kind = loaded_kind.as_i16();
            let cts_inserted = query!(
                "
            INSERT INTO ciphertexts(handle, ciphertext, ciphertext_version, ciphertext_type, key_material_kind)
            SELECT *, $5::SMALLINT FROM UNNEST($1::BYTEA[], $2::BYTEA[], $3::SMALLINT[], $4::SMALLINT[])
            ON CONFLICT (handle, ciphertext_version) DO NOTHING
            ",
                &handles, &ciphertexts, &ciphertext_versions, &ciphertext_types,
                key_material_kind
            )
                .execute(trx.as_mut())
                .await.map_err(|err| {
                    error!(target: "tfhe_worker", { error = %err }, "error while inserting new ciphertexts");
                    err
                })?.rows_affected();
            // Pin the SnS tasks of these handles to the CANONICAL
            // ciphertext row's material kind (RFC-029: the task row is
            // the sns-worker's only selection authority). Copying from
            // the stored row keeps the pin deterministic even when the
            // insert above was a conflict no-op and the canonical
            // bytes were produced earlier.
            let _ = query!(
                "UPDATE pbs_computations AS p
                 SET key_material_kind = c.key_material_kind
                 FROM ciphertexts AS c
                 WHERE p.handle = ANY($1::BYTEA[]) AND p.is_completed = false
                   AND c.handle = p.handle AND c.ciphertext_version = $2",
                &handles,
                current_ciphertext_version()
            )
            .execute(trx.as_mut())
            .await?;
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
