use crate::dependence_chain::{self};
use crate::types::CoprocessorError;
use fhevm_engine_common::branch::{
    is_branchless_producer, read_settled_height, select_producer_candidate, ProducerBlockHashed,
};
use fhevm_engine_common::database::{connect_pool_with_options, resolve_database_url_from_option};
use fhevm_engine_common::db_keys::DbKeyCache;
use fhevm_engine_common::telemetry;
use fhevm_engine_common::tfhe_ops::check_fhe_operand_types;
use fhevm_engine_common::types::{FhevmError, Handle, SupportedFheCiphertexts};
use fhevm_engine_common::{tfhe_ops::current_ciphertext_version, types::SupportedFheOperations};
use lazy_static::lazy_static;
use prometheus::{register_histogram, register_int_counter, Histogram, IntCounter};
use scheduler::dfg::scheduler::{re_randomise_boundary_input, Scheduler};
use scheduler::dfg::types::{DFGTaskInput, DFGTxInput, SchedulerError};
use scheduler::dfg::{build_component_nodes, ComponentNode, DFComponentGraph, DFGOp};
use sha3::{Digest, Keccak256};
use sqlx::types::Uuid;
use sqlx::Postgres;
use sqlx::{postgres::PgListener, query, Acquire, Row};
use std::collections::{HashMap, HashSet};
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
    static ref CIPHERTEXT_DIVERGENCE_COUNTER: IntCounter = register_int_counter!(
        "coprocessor_ciphertext_divergence_total",
        "number of times two same-block producers of the same handle wrote \
         divergent compressed ciphertext bytes or types. Non-zero implies \
         either an FHE non-determinism bug or a coprocessor scheduler bug \
         routing different effective inputs to the two producers."
    )
    .unwrap();
    static ref SETTLED_CIPHERTEXT_DIVERGENCE_COUNTER: IntCounter = register_int_counter!(
        "coprocessor_settled_ciphertext_divergence_total",
        "number of times settled same-handle ciphertext rows selected by RFC 011 \
         metadata resolution had divergent compressed ciphertext bytes or types"
    )
    .unwrap();
    static ref WORK_ITEMS_QUERY_HISTOGRAM: Histogram = register_histogram!(
        "coprocessor_tfhe_worker_query_work_items_seconds",
        "Histogram of time spent querying work items in tfhe-worker",
        vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.25, 0.5, 0.75, 1.0, 2.0, 5.0, 10.0]
    )
    .unwrap();
}

fn execution_transaction_id(transaction_id: &[u8], producer_block_hash: &[u8]) -> Handle {
    let mut hasher = Keccak256::new();
    hasher.update(transaction_id);
    hasher.update(producer_block_hash);
    hasher.finalize().to_vec()
}

#[cfg(feature = "gpu")]
fn next_gpu_index(num_gpus: usize) -> Result<usize, std::io::Error> {
    if num_gpus == 0 {
        return Err(std::io::Error::other("no GPU server keys available"));
    }
    static LAST_GPU_INDEX: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    Ok(LAST_GPU_INDEX.fetch_add(1, std::sync::atomic::Ordering::Relaxed) % num_gpus)
}

#[derive(Clone, Debug)]
struct BatchExecutionContext {
    host_chain_id: i64,
    block_number: Option<i64>,
    producer_block_hash: Handle,
}

#[derive(Clone, Debug)]
struct ExecutionTransactionContext {
    transaction_id: Handle,
    producer_block_hash: Handle,
    block_number: Option<i64>,
}

#[derive(Clone, Debug)]
struct DependencyMetadata {
    producer_block_hash: Handle,
    block_number: Option<i64>,
    settled_producer_block_hashes: Vec<Handle>,
}

#[derive(Clone, Debug)]
struct ProducerCandidate {
    block_number: Option<i64>,
    producer_block_hash: Handle,
}

#[derive(Clone, Debug)]
struct BlockAncestorRow {
    block_hash: Handle,
    parent_hash: Option<Handle>,
    block_number: i64,
}

#[derive(Clone, Debug)]
struct DependencyRow {
    handle: Handle,
    producer_block_hash: Handle,
    block_number: Option<i64>,
}

#[derive(Clone, Debug)]
struct CiphertextCandidate {
    producer_block_hash: Handle,
    ciphertext_type: i16,
    ciphertext: Vec<u8>,
}

#[derive(Clone, Debug)]
struct CiphertextInput {
    ciphertext_type: i16,
    ciphertext: Vec<u8>,
    rerandomize_as_boundary: bool,
}

impl ProducerBlockHashed for CiphertextCandidate {
    fn producer_block_hash(&self) -> &[u8] {
        &self.producer_block_hash
    }
}

#[derive(Clone, Debug)]
struct WorkRow {
    output_handle: Handle,
    dependencies: Vec<Handle>,
    fhe_operation: i16,
    is_scalar: bool,
    is_allowed: bool,
    transaction_id: Handle,
    producer_block_hash: Handle,
    host_chain_id: i64,
    block_number: Option<i64>,
    schedule_order: PrimitiveDateTime,
}

#[derive(Clone, Debug)]
struct CiphertextInsert {
    handle: Handle,
    producer_block_hash: Handle,
    block_number: Option<i64>,
    ct_bytes: Vec<u8>,
    ciphertext_version: i16,
    ciphertext_type: i16,
}

/// Input entry to `dedupe_ciphertext_inserts`. Carries the producer
/// `transaction_id` so the dedupe can pick a deterministic canonical
/// version (lex-smallest tid) when two producers in the same block
/// emit divergent bytes for the same handle.
struct CiphertextDedupeEntry {
    handle: Handle,
    producer_block_hash: Handle,
    block_number: Option<i64>,
    ct_bytes: Vec<u8>,
    ciphertext_version: i16,
    ciphertext_type: i16,
    transaction_id: Handle,
}

/// Returns whether strict ciphertext dedup is enabled. In strict mode, a
/// byte or type divergence between two same-block producers of the same
/// handle is a hard error that aborts the batch — preserved as the test
/// behaviour. In non-strict mode the canonical (lex-smallest `transaction_id`)
/// bytes win, divergence is logged and counted via
/// `coprocessor_ciphertext_divergence_total`, and the batch proceeds.
///
/// Defaults: strict in debug builds (`cfg!(debug_assertions)`) so
/// `cargo test` fails loudly; permissive in release builds. Override
/// via `FHEVM_STRICT_CIPHERTEXT_DEDUP=1` or `=0`.
fn strict_ciphertext_dedup_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| match std::env::var("FHEVM_STRICT_CIPHERTEXT_DEDUP") {
        Ok(value) => matches!(
            value.as_str(),
            "1" | "true" | "TRUE" | "True" | "yes" | "YES"
        ),
        Err(_) => cfg!(debug_assertions),
    })
}

/// Short hex fingerprint of `bytes`, suitable for divergence logs without
/// dumping the full ciphertext (which can be megabytes).
fn short_byte_fingerprint(bytes: &[u8]) -> String {
    let mut hasher = Keccak256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    hex::encode(&digest[..8])
}

/// Consensus guardrail enforced once at startup.
///
/// A block-scoped (wave-2) worker keyed at cutover 0 executes *every* block
/// under block-scoped semantics. On a chain that already has legacy-executed
/// state, that re-derives ciphertext bytes incompatibly with peers and with
/// the digests the legacy pipeline already published, so it diverges from the
/// rest of the consensus set. The cutover block is the agreed semantic
/// boundary; a forgotten/zero cutover on an existing chain is a silent
/// consensus hazard. Refuse to start in that case unless the operator
/// explicitly opts in for a genuinely fresh chain or tests.
async fn ensure_cutover_safe(
    args: &crate::daemon_cli::Args,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if args.branch_cutover_block != 0 {
        return Ok(());
    }
    let allow_zero = std::env::var("FHEVM_ALLOW_ZERO_CUTOVER")
        .ok()
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    if allow_zero {
        warn!(
            target: "tfhe_worker",
            "FHEVM_BRANCH_CUTOVER_BLOCK is 0/unset with FHEVM_ALLOW_ZERO_CUTOVER set: \
             executing all blocks under block-scoped semantics (intended only for fresh \
             chains / tests)"
        );
        return Ok(());
    }
    let db_url = resolve_database_url_from_option(args.database_url.clone())?;
    let (pool, _pool_refresh_handle) = connect_pool_with_options(
        &db_url,
        sqlx::postgres::PgPoolOptions::new().max_connections(1),
        None,
    )
    .await?;
    // "The legacy pipeline already executed work on this chain" == at least one
    // completed row in the legacy computations table. Pending dual-write rows
    // and branchless input ciphertexts are excluded: they exist on fresh
    // wave-2 deployments too and are not a divergence signal on their own.
    let legacy_executed: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM computations WHERE is_completed = TRUE)")
            .fetch_one(&pool)
            .await?;
    if legacy_executed {
        return Err(
            "Refusing to start: FHEVM_BRANCH_CUTOVER_BLOCK is 0/unset but this chain already \
             has legacy-executed computations. A cutover-0 wave-2 worker would re-execute \
             pre-cutover blocks under block-scoped semantics and diverge from peers and from \
             the digests the legacy pipeline already published. Set the agreed cutover block on \
             all services (tfhe-worker, sns-worker, host-listener), or set \
             FHEVM_ALLOW_ZERO_CUTOVER=1 for a genuinely fresh chain / tests."
                .into(),
        );
    }
    Ok(())
}

pub async fn run_tfhe_worker(
    args: crate::daemon_cli::Args,
    health_check: crate::health_check::HealthCheck,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Determine worker ID to use for the lifetime of this process
    // In case of a failure in tfhe_worker_cycle, the same id must be reused to quickly unlock any held locks
    let worker_id = args.worker_id.unwrap_or(Uuid::new_v4());
    info!(target: "tfhe_worker", worker_id = %worker_id, "Starting tfhe-worker service");
    // Fail fast (before any work) if a zero cutover would re-execute
    // pre-existing legacy state and break cross-coprocessor consensus.
    if let Err(guard_error) = ensure_cutover_safe(&args).await {
        error!(target: "tfhe_worker", { error = %guard_error }, "Cutover safety check failed; not starting worker");
        return Err(guard_error);
    }
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
        false,
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
        let (
            mut transactions,
            batch_context,
            execution_transaction_contexts,
            earliest_computation,
            has_more_work,
            locked_dcid_has_pending_work,
        ) = query_for_work(
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
            if locked_dcid_has_pending_work {
                info!(
                    target: "tfhe_worker",
                    "dcid still has pending work after empty acquisition; releasing without marking processed"
                );
                dcid_mngr.release_current_lock(false, None).await?;
                dcid_mngr.do_cleanup().await?;
                no_progress_cycles = 0;
                immediately_poll_more_work = false;
                continue;
            }
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

        let mut tx_graph = build_transaction_graph_and_execute(
            &mut transactions,
            &batch_context,
            args.branch_cutover_block,
            db_key_cache.clone(),
            &health_check,
            &mut trx,
            &dcid_mngr,
        )
        .instrument(loop_span.clone())
        .await?;
        let has_progressed = upload_transaction_graph_results(
            &mut tx_graph,
            &execution_transaction_contexts,
            &batch_context,
            &mut trx,
            &mut dcid_mngr,
        )
        .instrument(loop_span.clone())
        .await?;
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
#[tracing::instrument(name = "query_ciphertext_batch", skip_all, fields(count = dependency_metadata.len()))]
async fn query_ciphertexts<'a>(
    dependency_metadata: &HashMap<Handle, DependencyMetadata>,
    batch_context: &BatchExecutionContext,
    trx: &mut sqlx::Transaction<'a, Postgres>,
) -> Result<HashMap<Vec<u8>, CiphertextInput>, Box<dyn std::error::Error + Send + Sync>> {
    if dependency_metadata.is_empty() {
        return Ok(HashMap::new());
    }

    let requested_handles = dependency_metadata.keys().cloned().collect::<Vec<_>>();
    let mut ciphertext_candidates: HashMap<Vec<u8>, Vec<CiphertextCandidate>> =
        HashMap::with_capacity(dependency_metadata.len());
    let rows = sqlx::query(
        "
        SELECT handle, producer_block_hash, ciphertext, ciphertext_type
        FROM ciphertexts_branch
        WHERE handle = ANY($1::BYTEA[])
          AND ciphertext_version = $2
        ",
    )
    .bind(&requested_handles)
    .bind(current_ciphertext_version())
    .fetch_all(trx.as_mut())
    .await
    .map_err(|err| {
        error!(target: "tfhe_worker", { error = %err }, "error while querying branch ciphertexts");
        err
    })?;
    for row in rows {
        let handle: Vec<u8> = row.try_get("handle")?;
        ciphertext_candidates
            .entry(handle)
            .or_default()
            .push(CiphertextCandidate {
                producer_block_hash: row.try_get("producer_block_hash")?,
                ciphertext: row.try_get("ciphertext")?,
                ciphertext_type: row.try_get("ciphertext_type")?,
            });
    }

    // Branchless dependencies may predate the branch tables entirely (legacy
    // pipeline output below the cutover block): fall back to the legacy
    // ciphertexts table when no branchless branch row exists for them.
    let legacy_fallback_handles: Vec<Vec<u8>> = dependency_metadata
        .iter()
        .filter(|(handle, metadata)| {
            metadata.producer_block_hash.is_empty()
                && !ciphertext_candidates
                    .get(*handle)
                    .is_some_and(|candidates| {
                        candidates
                            .iter()
                            .any(|candidate| candidate.producer_block_hash.is_empty())
                    })
        })
        .map(|(handle, _)| handle.clone())
        .collect();
    if !legacy_fallback_handles.is_empty() {
        let rows = sqlx::query(
            "
            SELECT handle, ciphertext, ciphertext_type
            FROM ciphertexts
            WHERE handle = ANY($1::BYTEA[])
              AND ciphertext_version = $2
            ",
        )
        .bind(&legacy_fallback_handles)
        .bind(current_ciphertext_version())
        .fetch_all(trx.as_mut())
        .await
        .map_err(|err| {
            error!(target: "tfhe_worker", { error = %err }, "error while querying legacy ciphertexts fallback");
            err
        })?;
        for row in rows {
            let handle: Vec<u8> = row.try_get("handle")?;
            ciphertext_candidates
                .entry(handle)
                .or_default()
                .push(CiphertextCandidate {
                    producer_block_hash: Vec::new(),
                    ciphertext: row.try_get("ciphertext")?,
                    ciphertext_type: row.try_get("ciphertext_type")?,
                });
        }
    }

    let mut ciphertext_map: HashMap<Vec<u8>, CiphertextInput> =
        HashMap::with_capacity(dependency_metadata.len());
    for handle in requested_handles {
        let Some(metadata) = dependency_metadata.get(&handle) else {
            continue;
        };
        let Some(candidates) = ciphertext_candidates.get(&handle) else {
            continue;
        };
        ensure_not_same_block_db_dependency(&handle, metadata, batch_context)?;
        if let Some(candidate) = select_ciphertext_candidate(candidates, metadata) {
            observe_settled_ciphertext_equivalence(&handle, candidates, metadata);
            ciphertext_map.insert(
                handle.clone(),
                CiphertextInput {
                    ciphertext_type: candidate.ciphertext_type,
                    ciphertext: candidate.ciphertext.clone(),
                    rerandomize_as_boundary: true,
                },
            );
        }
    }

    Ok(ciphertext_map)
}

fn ensure_not_same_block_db_dependency(
    handle: &[u8],
    metadata: &DependencyMetadata,
    batch_context: &BatchExecutionContext,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if is_same_block_materialized_dependency(metadata, batch_context) {
        return Err(std::io::Error::other(format!(
            "same-block DB fetch invariant violation for handle {} producer_block_hash {} block {:?}: \
             same-block dependencies must be selected in the worker component and propagated in memory",
            hex::encode(handle),
            hex::encode(&metadata.producer_block_hash),
            metadata.block_number,
        ))
        .into());
    }
    Ok(())
}

fn is_same_block_materialized_dependency(
    metadata: &DependencyMetadata,
    batch_context: &BatchExecutionContext,
) -> bool {
    !metadata.producer_block_hash.is_empty()
        && metadata.producer_block_hash == batch_context.producer_block_hash
        && metadata.block_number == batch_context.block_number
}

fn select_ciphertext_candidate<'a>(
    candidates: &'a [CiphertextCandidate],
    metadata: &DependencyMetadata,
) -> Option<&'a CiphertextCandidate> {
    select_producer_candidate(candidates, &metadata.producer_block_hash)
}

fn settled_ciphertext_divergence<'a>(
    candidates: &'a [CiphertextCandidate],
    metadata: &DependencyMetadata,
) -> Option<(&'a CiphertextCandidate, &'a CiphertextCandidate)> {
    let mut baseline = None;
    for producer_block_hash in &metadata.settled_producer_block_hashes {
        let Some(candidate) = candidates
            .iter()
            .find(|candidate| &candidate.producer_block_hash == producer_block_hash)
        else {
            continue;
        };
        let Some(first) = baseline else {
            baseline = Some(candidate);
            continue;
        };
        if first.ciphertext_type != candidate.ciphertext_type
            || first.ciphertext != candidate.ciphertext
        {
            return Some((first, candidate));
        }
    }
    None
}

fn observe_settled_ciphertext_equivalence(
    handle: &[u8],
    candidates: &[CiphertextCandidate],
    metadata: &DependencyMetadata,
) {
    let Some((first, divergent)) = settled_ciphertext_divergence(candidates, metadata) else {
        return;
    };

    SETTLED_CIPHERTEXT_DIVERGENCE_COUNTER.inc();
    warn!(
        target: "tfhe_worker",
        handle = %hex::encode(handle),
        first_producer_block_hash = %hex::encode(&first.producer_block_hash),
        divergent_producer_block_hash = %hex::encode(&divergent.producer_block_hash),
        "Settled same-handle ciphertext rows diverged in bytes or type"
    );
    debug_assert_eq!(
        (first.ciphertext_type, first.ciphertext.as_slice()),
        (divergent.ciphertext_type, divergent.ciphertext.as_slice()),
        "settled same-handle ciphertext rows must be byte-equivalent"
    );
}

fn build_current_branch_ancestry(
    rows: Vec<BlockAncestorRow>,
    current_block_hash: &[u8],
    current_block_number: i64,
    min_block_number: i64,
) -> Result<HashMap<i64, Handle>, Box<dyn std::error::Error + Send + Sync>> {
    let mut blocks_by_hash: HashMap<Handle, (i64, Option<Handle>)> =
        HashMap::with_capacity(rows.len());
    for row in rows {
        blocks_by_hash.insert(row.block_hash, (row.block_number, row.parent_hash));
    }

    let mut current_hash = current_block_hash.to_vec();
    let mut current_number = current_block_number;
    let mut ancestry = HashMap::new();
    while current_number >= min_block_number {
        ancestry.insert(current_number, current_hash.clone());
        if current_number == min_block_number {
            return Ok(ancestry);
        }
        let Some((stored_block_number, parent_hash)) = blocks_by_hash.get(&current_hash) else {
            return Err(std::io::Error::other(format!(
                "missing block {} for branch context {}",
                current_number,
                hex::encode(&current_hash),
            ))
            .into());
        };
        if *stored_block_number != current_number {
            return Err(std::io::Error::other(format!(
                "block number mismatch for branch context {}: expected {}, got {}",
                hex::encode(&current_hash),
                current_number,
                stored_block_number,
            ))
            .into());
        }
        let Some(parent_hash) = parent_hash.as_ref() else {
            return Err(std::io::Error::other(format!(
                "missing parent_hash for branch context {} at block {}",
                hex::encode(&current_hash),
                current_number,
            ))
            .into());
        };
        current_hash = parent_hash.clone();
        current_number -= 1;
    }

    Err(std::io::Error::other(format!(
        "could not resolve ancestry for branch context {} down to block {}",
        hex::encode(current_block_hash),
        min_block_number,
    ))
    .into())
}

async fn load_current_branch_ancestry<'a>(
    batch_context: &BatchExecutionContext,
    source_block_numbers: &HashSet<i64>,
    trx: &mut sqlx::Transaction<'a, Postgres>,
) -> Result<HashMap<i64, Handle>, Box<dyn std::error::Error + Send + Sync>> {
    let Some(current_block_number) = batch_context.block_number else {
        return Ok(HashMap::new());
    };
    if batch_context.producer_block_hash.is_empty() || source_block_numbers.is_empty() {
        return Ok(HashMap::new());
    }

    let min_block_number = *source_block_numbers
        .iter()
        .min()
        .unwrap_or(&current_block_number);
    if min_block_number > current_block_number {
        return Err(std::io::Error::other(format!(
            "cannot resolve future dependency block {} from current block {}",
            min_block_number, current_block_number,
        ))
        .into());
    }

    let rows = query!(
        "
            SELECT block_hash, parent_hash, block_number
            FROM host_chain_blocks_valid
            WHERE chain_id = $1
            AND block_number <= $2
            AND block_number >= $3
            AND block_status <> 'orphaned'
        ",
        batch_context.host_chain_id,
        current_block_number,
        min_block_number,
    )
    .fetch_all(trx.as_mut())
    .await
    .map_err(|err| {
        error!(target: "tfhe_worker", { error = %err }, "error while loading block ancestry");
        err
    })?;

    let rows = rows
        .into_iter()
        .map(|row| BlockAncestorRow {
            block_hash: row.block_hash,
            parent_hash: row.parent_hash,
            block_number: row.block_number,
        })
        .collect::<Vec<_>>();
    build_current_branch_ancestry(
        rows,
        &batch_context.producer_block_hash,
        current_block_number,
        min_block_number,
    )
}

async fn query_dependency_metadata<'a>(
    cts_to_query: &[Vec<u8>],
    batch_context: &BatchExecutionContext,
    branch_cutover_block: i64,
    trx: &mut sqlx::Transaction<'a, Postgres>,
) -> Result<HashMap<Handle, DependencyMetadata>, Box<dyn std::error::Error + Send + Sync>> {
    if cts_to_query.is_empty() {
        return Ok(HashMap::new());
    }

    let mut computation_rows = Vec::new();
    let rows = sqlx::query(
        "
        SELECT output_handle AS handle, producer_block_hash, block_number
        FROM computations_branch
        WHERE host_chain_id = $2
        AND output_handle = ANY($1::BYTEA[])
        ",
    )
    .bind(cts_to_query)
    .bind(batch_context.host_chain_id)
    .fetch_all(trx.as_mut())
    .await
    .map_err(|err| {
        error!(target: "tfhe_worker", { error = %err }, "error while querying branch computation dependency metadata");
        err
    })?;
    for row in rows {
        computation_rows.push(DependencyRow {
            handle: row.try_get("handle")?,
            producer_block_hash: row.try_get("producer_block_hash")?,
            block_number: row.try_get("block_number")?,
        });
    }

    let mut allowed_handle_rows = Vec::new();
    let rows = sqlx::query(
        "
        SELECT handle, producer_block_hash, block_number
        FROM allowed_handles_branch
        WHERE (host_chain_id = $2 OR host_chain_id IS NULL)
        AND handle = ANY($1::BYTEA[])
        ",
    )
    .bind(cts_to_query)
    .bind(batch_context.host_chain_id)
    .fetch_all(trx.as_mut())
    .await
    .map_err(|err| {
        error!(target: "tfhe_worker", { error = %err }, "error while querying branch allowed-handle dependency metadata");
        err
    })?;
    for row in rows {
        allowed_handle_rows.push(DependencyRow {
            handle: row.try_get("handle")?,
            producer_block_hash: row.try_get("producer_block_hash")?,
            block_number: row.try_get("block_number")?,
        });
    }

    let settled_height = read_settled_height(trx, batch_context.host_chain_id).await?;

    // Candidates below the cutover block resolve as branchless and need no
    // ancestry. Candidates at or below settled_height are trusted through the
    // settlement invariant, so only recent post-cutover candidates need the
    // current-branch ancestry walk.
    let candidate_block_numbers = computation_rows
        .iter()
        .filter_map(|row| row.block_number)
        .chain(
            allowed_handle_rows
                .iter()
                .filter_map(|row| row.block_number),
        )
        .filter(|block_number| {
            *block_number >= branch_cutover_block && *block_number > settled_height
        })
        .collect::<HashSet<_>>();
    let ancestry =
        load_current_branch_ancestry(batch_context, &candidate_block_numbers, trx).await?;

    let mut metadata = HashMap::with_capacity(cts_to_query.len());
    let mut computation_candidates: HashMap<Handle, Vec<ProducerCandidate>> = HashMap::new();
    for row in computation_rows {
        computation_candidates
            .entry(row.handle)
            .or_default()
            .push(ProducerCandidate {
                block_number: row.block_number,
                producer_block_hash: row.producer_block_hash,
            });
    }
    for handle in cts_to_query {
        let Some(candidates) = computation_candidates.get(handle) else {
            continue;
        };
        if let Some(resolved) = resolve_dependency_metadata(
            candidates,
            &ancestry,
            branch_cutover_block,
            settled_height,
        )? {
            metadata.insert(handle.clone(), resolved);
        }
    }

    let mut allowed_handle_candidates: HashMap<Handle, Vec<ProducerCandidate>> = HashMap::new();
    for row in allowed_handle_rows {
        allowed_handle_candidates
            .entry(row.handle)
            .or_default()
            .push(ProducerCandidate {
                block_number: row.block_number,
                producer_block_hash: row.producer_block_hash,
            });
    }
    for handle in cts_to_query {
        if metadata.contains_key(handle) {
            continue;
        }
        let Some(candidates) = allowed_handle_candidates.get(handle) else {
            continue;
        };
        if let Some(resolved) = resolve_dependency_metadata(
            candidates,
            &ancestry,
            branch_cutover_block,
            settled_height,
        )? {
            metadata.insert(handle.clone(), resolved);
        }
    }

    let branchless_ciphertext_rows =
        query_branchless_ciphertext_handles(cts_to_query, current_ciphertext_version(), trx)
            .await?;
    let handles_with_candidate_metadata = computation_candidates
        .keys()
        .chain(allowed_handle_candidates.keys())
        .cloned()
        .collect::<HashSet<_>>();
    add_branchless_ciphertext_metadata_fallback(
        &mut metadata,
        cts_to_query,
        &branchless_ciphertext_rows,
        &handles_with_candidate_metadata,
    );

    Ok(metadata)
}

async fn query_branchless_ciphertext_handles<'a>(
    cts_to_query: &[Vec<u8>],
    ciphertext_version: i16,
    trx: &mut sqlx::Transaction<'a, Postgres>,
) -> Result<HashSet<Handle>, Box<dyn std::error::Error + Send + Sync>> {
    let mut handles = HashSet::new();
    let rows = sqlx::query(
        "
        SELECT handle
        FROM ciphertexts_branch
        WHERE handle = ANY($1::BYTEA[])
          AND ciphertext_version = $2
          AND producer_block_hash = '\\x'::BYTEA
        UNION
        SELECT handle
        FROM ciphertexts
        WHERE handle = ANY($1::BYTEA[])
          AND ciphertext_version = $2
        ",
    )
    .bind(cts_to_query)
    .bind(ciphertext_version)
    .fetch_all(trx.as_mut())
    .await
    .map_err(|err| {
        error!(target: "tfhe_worker", { error = %err }, "error while querying branchless ciphertext dependency fallback");
        err
    })?;
    for row in rows {
        handles.insert(row.try_get("handle")?);
    }
    Ok(handles)
}

fn add_branchless_ciphertext_metadata_fallback(
    metadata: &mut HashMap<Handle, DependencyMetadata>,
    cts_to_query: &[Vec<u8>],
    branchless_ciphertext_rows: &HashSet<Handle>,
    handles_with_candidate_metadata: &HashSet<Handle>,
) {
    for handle in cts_to_query {
        if metadata.contains_key(handle)
            || handles_with_candidate_metadata.contains(handle)
            || !branchless_ciphertext_rows.contains(handle)
        {
            continue;
        }
        metadata.insert(
            handle.clone(),
            DependencyMetadata {
                producer_block_hash: Vec::new(),
                block_number: None,
                settled_producer_block_hashes: Vec::new(),
            },
        );
    }
}

fn resolve_dependency_metadata(
    candidates: &[ProducerCandidate],
    ancestry: &HashMap<i64, Handle>,
    branch_cutover_block: i64,
    settled_height: i64,
) -> Result<Option<DependencyMetadata>, std::io::Error> {
    // Recent candidates produced after both the cutover and settlement
    // frontier resolve through branch ancestry; an exact current-branch
    // producer always wins.
    let branch_candidates = candidates
        .iter()
        .filter(|candidate| {
            !candidate.producer_block_hash.is_empty()
                && candidate.block_number.is_some_and(|block_number| {
                    block_number >= branch_cutover_block && block_number > settled_height
                })
        })
        .cloned()
        .collect::<Vec<_>>();
    if let Some(candidate) = resolve_current_branch_candidate(&branch_candidates, ancestry) {
        return Ok(Some(DependencyMetadata {
            producer_block_hash: candidate.producer_block_hash,
            block_number: candidate.block_number,
            settled_producer_block_hashes: Vec::new(),
        }));
    }

    // Settled candidates are already below the trust frontier: orphan cleanup
    // has removed dropped-fork rows and the write guard prevents new stale
    // rows. Prefer the newest settled derivation for same-fork repeatable
    // handles; for deterministic non-compute handles any same-fork row would
    // have equivalent bytes, but newest is still the RFC tie-breaker.
    let settled_candidates = candidates
        .iter()
        .filter(|candidate| {
            !candidate.producer_block_hash.is_empty()
                && candidate.block_number.is_some_and(|block_number| {
                    block_number >= branch_cutover_block && block_number <= settled_height
                })
        })
        .collect::<Vec<_>>();
    let settled_candidate = settled_candidates
        .iter()
        .copied()
        .max_by_key(|candidate| candidate.block_number.unwrap_or(i64::MIN));
    if let Some(candidate) = settled_candidate {
        return Ok(Some(DependencyMetadata {
            producer_block_hash: candidate.producer_block_hash.clone(),
            block_number: candidate.block_number,
            settled_producer_block_hashes: settled_candidates
                .into_iter()
                .map(|candidate| candidate.producer_block_hash.clone())
                .collect(),
        }));
    }

    // Branchless candidates: rows stored without block derivation (ZK user
    // inputs, backfilled pre-upgrade state) or produced below the cutover by
    // the legacy pipeline. Their bytes live in branchless branch rows or the
    // legacy ciphertexts table, on every branch.
    let has_branchless_candidate = candidates.iter().any(|candidate| {
        candidate.producer_block_hash.is_empty()
            || candidate
                .block_number
                .is_some_and(|block_number| block_number < branch_cutover_block)
    });
    if has_branchless_candidate {
        return Ok(Some(DependencyMetadata {
            producer_block_hash: Vec::new(),
            block_number: None,
            settled_producer_block_hashes: Vec::new(),
        }));
    }
    Ok(None)
}

fn resolve_current_branch_candidate(
    candidates: &[ProducerCandidate],
    ancestry: &HashMap<i64, Handle>,
) -> Option<ProducerCandidate> {
    candidates
        .iter()
        .filter(|candidate| {
            candidate
                .block_number
                .is_some_and(|candidate_block_number| {
                    ancestry.get(&candidate_block_number) == Some(&candidate.producer_block_hash)
                })
        })
        .max_by_key(|candidate| candidate.block_number.unwrap_or(i64::MIN))
        .cloned()
}

#[cfg(test)]
fn resolve_current_branch_candidates(
    candidates: &[ProducerCandidate],
    ancestry: &HashMap<i64, Handle>,
) -> HashSet<Handle> {
    candidates
        .iter()
        .filter(|candidate| {
            candidate
                .block_number
                .is_some_and(|candidate_block_number| {
                    ancestry.get(&candidate_block_number) == Some(&candidate.producer_block_hash)
                })
        })
        .map(|candidate| candidate.producer_block_hash.clone())
        .collect()
}

async fn resolve_ciphertext_handles<'a>(
    cts_to_query: &[Vec<u8>],
    batch_context: &BatchExecutionContext,
    branch_cutover_block: i64,
    trx: &mut sqlx::Transaction<'a, Postgres>,
) -> Result<HashMap<Handle, DependencyMetadata>, Box<dyn std::error::Error + Send + Sync>> {
    let dependency_metadata =
        query_dependency_metadata(cts_to_query, batch_context, branch_cutover_block, trx).await?;
    Ok(dependency_metadata)
}

async fn dependence_chain_has_pending_branch_work<'a>(
    dependence_chain_id: &[u8],
    branch_cutover_block: i64,
    trx: &mut sqlx::Transaction<'a, Postgres>,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let has_pending = sqlx::query_scalar::<_, bool>(
        "
        SELECT EXISTS (
            SELECT 1
            FROM computations_branch
            WHERE is_completed = FALSE
              AND is_error = FALSE
              AND is_allowed = TRUE
              AND (block_number IS NULL OR block_number >= $2)
              AND dependence_chain_id = $1
        )
        ",
    )
    .bind(dependence_chain_id)
    .bind(branch_cutover_block)
    .fetch_one(trx.as_mut())
    .await
    .map_err(|err| {
        error!(
            target: "tfhe_worker",
            { error = %err },
            "error while checking pending branch work for dependence chain"
        );
        err
    })?;

    Ok(has_pending)
}

async fn query_work_rows_for_dcid<'a>(
    trx: &mut sqlx::Transaction<'a, Postgres>,
    dependence_chain_id: Option<Vec<u8>>,
    branch_cutover_block: i64,
) -> Result<Vec<WorkRow>, sqlx::Error> {
    let sql = "
-- DCIDs are ingestion-time same-block connected components. A repeated
-- transaction can appear on sibling forks with the same DCID, so select the
-- earliest eligible block context in the acquired lane. Lock the full DCID
-- row set first so expired-lock stealing cannot skip a locked sibling context
-- and process another fork concurrently.
WITH expected_dcid_rows AS (
    SELECT COUNT(*) AS count
    FROM computations_branch c
    WHERE c.dependence_chain_id = $1
      AND c.is_error = FALSE
),
locked_dcid_rows AS MATERIALIZED (
    SELECT
      c.output_handle,
      c.dependencies,
      c.fhe_operation,
      c.is_scalar,
      c.is_allowed,
      c.dependence_chain_id,
      c.transaction_id,
      c.producer_block_hash,
      c.host_chain_id,
      c.block_number,
      c.schedule_order,
      c.is_completed
    FROM computations_branch c
    WHERE c.dependence_chain_id = $1
      AND c.is_error = FALSE
    ORDER BY c.schedule_order ASC
    FOR UPDATE OF c SKIP LOCKED
),
locked_dcid_count AS (
    SELECT COUNT(*) AS count FROM locked_dcid_rows
),
selected_context AS MATERIALIZED (
    SELECT
      c.dependence_chain_id,
      c.host_chain_id,
      c.block_number,
      c.producer_block_hash
    FROM locked_dcid_rows c
    WHERE c.is_completed = FALSE
      AND c.is_allowed = TRUE
      AND (c.block_number IS NULL OR c.block_number >= $2)
      AND (SELECT count FROM locked_dcid_count) = (SELECT count FROM expected_dcid_rows)
    ORDER BY c.schedule_order ASC
    LIMIT 1
),
expected_rows AS (
    SELECT COUNT(*) AS count
    FROM locked_dcid_rows c
    JOIN selected_context sc
      ON c.dependence_chain_id = sc.dependence_chain_id
     AND c.host_chain_id = sc.host_chain_id
     AND c.block_number IS NOT DISTINCT FROM sc.block_number
     AND c.producer_block_hash = sc.producer_block_hash
),
locked_rows AS MATERIALIZED (
    SELECT
      c.output_handle,
      c.dependencies,
      c.fhe_operation,
      c.is_scalar,
      c.is_allowed,
      c.dependence_chain_id,
      c.transaction_id,
      c.producer_block_hash,
      c.host_chain_id,
      c.block_number,
      c.schedule_order
    FROM locked_dcid_rows c
    JOIN selected_context sc
      ON c.dependence_chain_id = sc.dependence_chain_id
     AND c.host_chain_id = sc.host_chain_id
     AND c.block_number IS NOT DISTINCT FROM sc.block_number
     AND c.producer_block_hash = sc.producer_block_hash
    ORDER BY c.schedule_order ASC
),
locked_count AS (
    SELECT COUNT(*) AS count FROM locked_rows
)
SELECT
  output_handle,
  dependencies,
  fhe_operation,
  is_scalar,
  is_allowed,
  dependence_chain_id,
  transaction_id,
  producer_block_hash,
  host_chain_id,
  block_number,
  schedule_order
FROM locked_rows
WHERE (SELECT count FROM locked_count) = (SELECT count FROM expected_rows)
ORDER BY schedule_order ASC
        ";
    sqlx::query(sql)
        .bind(dependence_chain_id)
        .bind(branch_cutover_block)
        .fetch_all(trx.as_mut())
        .await?
        .into_iter()
        .map(|row| -> Result<WorkRow, sqlx::Error> {
            Ok(WorkRow {
                output_handle: row.try_get("output_handle")?,
                dependencies: row.try_get("dependencies")?,
                fhe_operation: row.try_get("fhe_operation")?,
                is_scalar: row.try_get("is_scalar")?,
                is_allowed: row.try_get("is_allowed")?,
                transaction_id: row.try_get("transaction_id")?,
                producer_block_hash: row.try_get("producer_block_hash")?,
                host_chain_id: row.try_get("host_chain_id")?,
                block_number: row.try_get("block_number")?,
                schedule_order: row.try_get("schedule_order")?,
            })
        })
        .collect::<Result<Vec<_>, _>>()
}

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
        BatchExecutionContext,
        HashMap<Handle, ExecutionTransactionContext>,
        PrimitiveDateTime,
        bool,
        bool,
    ),
    Box<dyn std::error::Error + Send + Sync>,
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
        return Ok((
            vec![],
            BatchExecutionContext {
                host_chain_id: 0,
                block_number: None,
                producer_block_hash: vec![],
            },
            HashMap::new(),
            PrimitiveDateTime::MAX,
            false,
            false,
        ));
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
    let started_at = SystemTime::now();
    let the_work = query_work_rows_for_dcid(
        trx,
        dependence_chain_id.clone(),
        args.branch_cutover_block,
    )
    .instrument(s_work.clone())
    .await
    .map_err(|err| {
        error!(target: "tfhe_worker", { error = %err }, "error while querying branch work items");
        err
    })?;

    WORK_ITEMS_QUERY_HISTOGRAM.observe(started_at.elapsed().unwrap_or_default().as_secs_f64());
    s_work.record("count", the_work.len());
    health_check.update_db_access();
    if the_work.is_empty() {
        let locked_dcid_has_pending_work = match &dependence_chain_id {
            Some(dependence_chain_id) => {
                dependence_chain_has_pending_branch_work(
                    dependence_chain_id,
                    args.branch_cutover_block,
                    trx,
                )
                .await?
            }
            None => false,
        };
        if let Some(dependence_chain_id) = &dependence_chain_id {
            info!(
                target: "tfhe_worker",
                dcid = %hex::encode(dependence_chain_id),
                locking = ?locking_reason,
                locked_dcid_has_pending_work,
                "No work items found to process"
            );
        }
        health_check.update_activity();
        return Ok((
            vec![],
            BatchExecutionContext {
                host_chain_id: 0,
                block_number: None,
                producer_block_hash: vec![],
            },
            HashMap::new(),
            PrimitiveDateTime::MAX,
            false,
            locked_dcid_has_pending_work,
        ));
    }
    WORK_ITEMS_FOUND_COUNTER.inc_by(the_work.len() as u64);
    info!(target: "tfhe_worker", { count = the_work.len(), dcid = ?dependence_chain_id.as_ref().map(hex::encode),
				    locking = ?locking_reason }, "Processing work items");
    let s_prep = tracing::info_span!("prepare_dataflow_graphs", work_items = the_work.len());
    let (transactions, batch_context, execution_transaction_contexts, earliest_schedule_order) = async {
        let mut earliest_schedule_order = the_work.first().unwrap().schedule_order;
        let batch_context = BatchExecutionContext {
            host_chain_id: the_work.first().unwrap().host_chain_id,
            block_number: the_work.first().unwrap().block_number,
            producer_block_hash: the_work.first().unwrap().producer_block_hash.clone(),
        };
        let mut execution_transaction_contexts = HashMap::new();
        let mut work_by_execution_transaction: HashMap<Handle, Vec<_>> = HashMap::new();
        for work in the_work {
            let work_producer_block_hash = work.producer_block_hash.clone();
            if work.host_chain_id != batch_context.host_chain_id
                || work.block_number != batch_context.block_number
                || work_producer_block_hash != batch_context.producer_block_hash
            {
                return Err(std::io::Error::other(format!(
                    "mixed block contexts in worker batch: expected chain {} block {:?} hash {}, found chain {} block {:?} hash {}",
                    batch_context.host_chain_id,
                    batch_context.block_number,
                    hex::encode(&batch_context.producer_block_hash),
                    work.host_chain_id,
                    work.block_number,
                    hex::encode(&work.producer_block_hash),
                ))
                .into());
            }
            let execution_transaction_id =
                execution_transaction_id(&work.transaction_id, &work_producer_block_hash);
            execution_transaction_contexts
                .entry(execution_transaction_id.clone())
                .or_insert_with(|| {
                    ExecutionTransactionContext {
                        transaction_id: work.transaction_id.clone(),
                        producer_block_hash: work_producer_block_hash.clone(),
                        block_number: work.block_number,
                    }
                });
            work_by_execution_transaction
                .entry(execution_transaction_id)
                .or_default()
                .push(work);
        }
        // Traverse transactions and build transaction nodes
        let mut transactions: Vec<ComponentNode> = vec![];
        for (execution_transaction_id, txwork) in work_by_execution_transaction.iter() {
            let context = execution_transaction_contexts
                .get(execution_transaction_id)
                .ok_or_else(|| {
                    std::io::Error::other(format!(
                        "missing execution context for transaction {}",
                        hex::encode(execution_transaction_id)
                    ))
                })?;
            let mut ops = vec![];
            for w in txwork {
                let fhe_op: SupportedFheOperations = match w.fhe_operation.try_into() {
                    Ok(op) => op,
                    Err(e) => {
                        error!(target: "tfhe_worker", { output_handle = ?w.output_handle, transaction_id = ?hex::encode(&context.transaction_id), error = %e, }, "invalid FHE operation ");
                        set_computation_error(
                            &w.output_handle,
                            &context.transaction_id,
                            &context.producer_block_hash,
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
            let (mut components, _) = build_component_nodes(ops, execution_transaction_id)?;
            transactions.append(&mut components);
        }
        Ok::<_, Box<dyn std::error::Error + Send + Sync>>((
            transactions,
            batch_context,
            execution_transaction_contexts,
            earliest_schedule_order,
        ))
    }
    .instrument(s_prep)
    .await?;
    Ok((
        transactions,
        batch_context,
        execution_transaction_contexts,
        earliest_schedule_order,
        true,
        false,
    ))
}

#[tracing::instrument(name = "build_and_execute", skip_all)]
async fn build_transaction_graph_and_execute<'a>(
    txs: &mut Vec<ComponentNode>,
    batch_context: &BatchExecutionContext,
    branch_cutover_block: i64,
    db_key_cache: DbKeyCache,
    health_check: &crate::health_check::HealthCheck,
    trx: &mut sqlx::Transaction<'a, Postgres>,
    dcid_mngr: &dependence_chain::LockMngr,
) -> Result<DFComponentGraph, CoprocessorError> {
    let mut tx_graph = DFComponentGraph::default();
    if txs.is_empty() {
        return Ok(tx_graph);
    }
    if let Err(e) = tx_graph.build(txs) {
        // If we had an error while building the graph, we don't
        // execute anything and return to allow any set results
        // (essentially errors) to be set in DB.
        warn!(target: "tfhe_worker", { error = %e }, "error while building transaction graph");
        return Ok(tx_graph);
    }
    // Same-block producers selected in this graph must feed consumers through
    // in-memory edges. Fetching their compressed DB image would force a
    // Compress -> Decompress path for a same-block intermediate, which RFC 020
    // treats as consensus-invalid because decompression is not bitwise
    // preserving. Keep those handles out of the DB materialization path so
    // resolve_dependences() retains the producer -> consumer edge.
    let cts_to_query = tx_graph
        .needed_map
        .keys()
        .filter(|handle| !tx_graph.produced.contains_key(*handle))
        .cloned()
        .collect::<Vec<_>>();
    let resolved_handles =
        resolve_ciphertext_handles(&cts_to_query, batch_context, branch_cutover_block, trx).await?;
    let ciphertext_map = query_ciphertexts(&resolved_handles, batch_context, trx).await?;
    let fetched_handles: HashSet<_> = ciphertext_map.keys().cloned().collect();
    if cts_to_query.len() != fetched_handles.len() {
        if let Some(dcid_lock) = dcid_mngr.get_current_lock() {
            warn!(target: "tfhe_worker", { missing_inputs = ?(cts_to_query.len() - fetched_handles.len()), dcid = %hex::encode(dcid_lock.dependence_chain_id) },
	      "some inputs are missing to execute the dependence chain");
        }
    }
    // Block-scoped execution is the only path: decompress + re-rand boundary
    // inputs at the worker level so all partitions receive pre-materialized
    // working ciphertexts regardless of how the partitioner distributes them.
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
            error!(target: "tfhe_worker", { error = %cerr }, "failed to fetch latest key for boundary materialization");
            telemetry::set_current_span_error(&cerr);
            WORKER_ERRORS_COUNTER.inc();
            return Err(cerr);
        }
    };
    {
        let block_hash = batch_context.producer_block_hash.clone();
        let cpk = keys.pks.clone();
        #[cfg(feature = "gpu")]
        let materialization_gpu_idx = next_gpu_index(keys.gpu_sks.len())?;
        #[cfg(not(feature = "gpu"))]
        let materialization_gpu_idx = 0;
        let materialized: Vec<(Vec<u8>, DFGTxInput)> = tokio::task::spawn_blocking({
            let ciphertext_map = ciphertext_map;
            #[cfg(not(feature = "gpu"))]
            let sks_for_materialize = keys.sks.clone();
            #[cfg(feature = "gpu")]
            let sks_for_materialize = keys.gpu_sks[materialization_gpu_idx].clone();
            let gpu_idx_for_materialize = materialization_gpu_idx;
            move || -> Result<Vec<(Vec<u8>, DFGTxInput)>, Box<dyn std::error::Error + Send + Sync>> {
                tfhe::set_server_key(sks_for_materialize);
                let mut results = Vec::with_capacity(ciphertext_map.len());
                for (handle, input) in ciphertext_map {
                    let mut working = SupportedFheCiphertexts::decompress(
                        input.ciphertext_type,
                        &input.ciphertext,
                        gpu_idx_for_materialize,
                    )?;
                    if input.rerandomize_as_boundary {
                        re_randomise_boundary_input(&mut working, &block_hash, &cpk)?;
                    }
                    results.push((handle, DFGTxInput::Value((working, true))));
                }
                Ok(results)
            }
        })
        .await
        .map_err(|err| CoprocessorError::Other(err.into()))?
        .map_err(CoprocessorError::from)?;
        for (handle, input) in materialized.into_iter() {
            tx_graph
                .add_input(&handle, &input)
                .map_err(|err| CoprocessorError::Other(err.into()))?;
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
        // Schedule computations in parallel as dependences allow
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
        Ok::<(), CoprocessorError>(())
    }
    .instrument(s_compute)
    .await?;
    Ok(tx_graph)
}

fn dedupe_ciphertext_inserts(
    entries: Vec<CiphertextDedupeEntry>,
) -> Result<Vec<CiphertextInsert>, Box<dyn std::error::Error + Send + Sync>> {
    dedupe_ciphertext_inserts_inner(entries, strict_ciphertext_dedup_enabled())
}

fn dedupe_ciphertext_inserts_inner(
    mut entries: Vec<CiphertextDedupeEntry>,
    strict: bool,
) -> Result<Vec<CiphertextInsert>, Box<dyn std::error::Error + Send + Sync>> {
    // Sort by transaction_id ascending so the lex-smallest tid is seen
    // first per (handle, producer_block_hash, ciphertext_version) key.
    // This matches the canonical-producer choice made in
    // `scheduler::dfg::DFComponentGraph::build` for `aliased_tids` —
    // keeping in-memory consumer routing and persisted bytes consistent
    // on the determinism-violation path.
    entries.sort_by(|a, b| a.transaction_id.cmp(&b.transaction_id));

    let mut cts_seen: HashMap<(Handle, Handle, i16), CiphertextDedupeEntry> =
        HashMap::with_capacity(entries.len());
    for entry in entries.into_iter() {
        let key = (
            entry.handle.clone(),
            entry.producer_block_hash.clone(),
            entry.ciphertext_version,
        );
        match cts_seen.entry(key) {
            std::collections::hash_map::Entry::Vacant(slot) => {
                slot.insert(entry);
            }
            std::collections::hash_map::Entry::Occupied(slot) => {
                let canonical = slot.get();
                let type_diverges = canonical.ciphertext_type != entry.ciphertext_type;
                let bytes_diverge = canonical.ct_bytes.as_slice() != entry.ct_bytes.as_slice();
                if type_diverges || bytes_diverge {
                    if strict {
                        return Err(std::io::Error::other(format!(
                            "multi-producer ciphertext divergence for handle {} producer_block_hash {} version {}: \
                             canonical tid {} type {} bytes {}, divergent tid {} type {} bytes {}",
                            hex::encode(&entry.handle),
                            hex::encode(&entry.producer_block_hash),
                            entry.ciphertext_version,
                            hex::encode(&canonical.transaction_id),
                            canonical.ciphertext_type,
                            short_byte_fingerprint(&canonical.ct_bytes),
                            hex::encode(&entry.transaction_id),
                            entry.ciphertext_type,
                            short_byte_fingerprint(&entry.ct_bytes),
                        ))
                        .into());
                    }
                    CIPHERTEXT_DIVERGENCE_COUNTER.inc();
                    warn!(
                        target: "tfhe_worker",
                        handle = %hex::encode(&entry.handle),
                        producer_block_hash = %hex::encode(&entry.producer_block_hash),
                        ciphertext_version = entry.ciphertext_version,
                        canonical_transaction_id = %hex::encode(&canonical.transaction_id),
                        divergent_transaction_id = %hex::encode(&entry.transaction_id),
                        canonical_ciphertext_type = canonical.ciphertext_type,
                        divergent_ciphertext_type = entry.ciphertext_type,
                        canonical_bytes_fingerprint = %short_byte_fingerprint(&canonical.ct_bytes),
                        divergent_bytes_fingerprint = %short_byte_fingerprint(&entry.ct_bytes),
                        type_diverges,
                        bytes_diverge,
                        "Ciphertext divergence between two same-block producers of the same handle; \
                         keeping canonical (lex-smallest transaction_id) bytes. This indicates \
                         either an FHE non-determinism bug or a coprocessor scheduler bug giving \
                         the two producers different effective inputs. Set \
                         FHEVM_STRICT_CIPHERTEXT_DEDUP=1 to abort the batch on this condition."
                    );
                }
                // Canonical wins regardless; entry is dropped.
            }
        }
    }
    let cts_deduped = cts_seen
        .into_values()
        .map(|e| CiphertextInsert {
            handle: e.handle,
            producer_block_hash: e.producer_block_hash,
            block_number: e.block_number,
            ct_bytes: e.ct_bytes,
            ciphertext_version: e.ciphertext_version,
            ciphertext_type: e.ciphertext_type,
        })
        .collect();
    Ok(cts_deduped)
}

fn ensure_ciphertext_write_above_settled(
    entry: &CiphertextInsert,
    settled_height: i64,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if is_branchless_producer(&entry.producer_block_hash) {
        return Ok(());
    }
    let Some(block_number) = entry.block_number else {
        return Err(std::io::Error::other(format!(
            "refusing branch ciphertext write without block_number for handle {} producer_block_hash {}",
            hex::encode(&entry.handle),
            hex::encode(&entry.producer_block_hash),
        ))
        .into());
    };
    if block_number <= settled_height {
        return Err(std::io::Error::other(format!(
            "refusing settled branch ciphertext write for handle {} producer_block_hash {} block {} <= settled_height {}",
            hex::encode(&entry.handle),
            hex::encode(&entry.producer_block_hash),
            block_number,
            settled_height,
        ))
        .into());
    }
    Ok(())
}

#[tracing::instrument(name = "upload_results", skip_all)]
async fn upload_transaction_graph_results<'a>(
    tx_graph: &mut DFComponentGraph,
    execution_transaction_contexts: &HashMap<Handle, ExecutionTransactionContext>,
    batch_context: &BatchExecutionContext,
    trx: &mut sqlx::Transaction<'a, Postgres>,
    deps_mngr: &mut dependence_chain::LockMngr,
) -> Result<bool, CoprocessorError> {
    // Get computation results
    let graph_results = tx_graph.get_results();
    let mut handles_to_update = vec![];
    let mut res = false;

    // Traverse computations that have been scheduled and
    // upload their results/errors.
    let mut cts_to_insert = vec![];
    for result in graph_results.into_iter() {
        let context = execution_transaction_contexts
            .get(&result.transaction_id)
            .cloned()
            .ok_or_else(|| {
                CoprocessorError::Other(
                    std::io::Error::other(format!(
                        "missing execution context for transaction {}",
                        hex::encode(&result.transaction_id)
                    ))
                    .into(),
                )
            })?;
        match result.compressed_ct {
            Ok(cct) => {
                cts_to_insert.push(CiphertextDedupeEntry {
                    handle: result.handle.clone(),
                    producer_block_hash: context.producer_block_hash.clone(),
                    block_number: context.block_number,
                    ct_bytes: cct.ct_bytes,
                    ciphertext_version: current_ciphertext_version(),
                    ciphertext_type: cct.ct_type,
                    transaction_id: result.transaction_id.clone(),
                });
                handles_to_update.push((
                    result.handle.clone(),
                    (
                        context.transaction_id.clone(),
                        context.producer_block_hash.clone(),
                    ),
                ));
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
                    &context.transaction_id,
                    &context.producer_block_hash,
                    &*cerr,
                    trx,
                    deps_mngr,
                )
                .await?;
            }
        }
    }
    if !cts_to_insert.is_empty() {
        // Multi-producer coalesce + consensus-safety check.
        // Two transactions in the same block can deterministically derive
        // the same output handle, which means both producers push a
        // `DFGTxResult` and both land here with equal `(handle,
        // producer_block_hash, ciphertext_version)`. The INSERTs collapse
        // via `ON CONFLICT DO NOTHING` anyway, but we prune duplicates
        // up front so the UNNEST batch stays one row per unique key and
        // the persisted bytes are deterministic across coprocessors:
        // `dedupe_ciphertext_inserts` keeps the lex-smallest tid's bytes
        // (the same canonical the scheduler picks for cross-tx routing).
        // On byte/type divergence between two producers we warn + bump
        // `coprocessor_ciphertext_divergence_total`; set
        // `FHEVM_STRICT_CIPHERTEXT_DEDUP=1` (or run a debug build) to
        // abort the batch instead.
        let cts_to_insert = dedupe_ciphertext_inserts(cts_to_insert)?;
        let settled_height = read_settled_height(trx, batch_context.host_chain_id).await?;
        for entry in &cts_to_insert {
            ensure_ciphertext_write_above_settled(entry, settled_height)?;
        }
        let s_insert = tracing::info_span!("insert_ct_into_db", count = cts_to_insert.len());
        let cts_inserted = async {
            let handles = cts_to_insert
                .iter()
                .map(|entry| entry.handle.clone())
                .collect::<Vec<_>>();
            let producer_block_hashes = cts_to_insert
                .iter()
                .map(|entry| entry.producer_block_hash.clone())
                .collect::<Vec<_>>();
            let block_numbers = cts_to_insert
                .iter()
                .map(|entry| entry.block_number)
                .collect::<Vec<_>>();
            let ciphertexts = cts_to_insert
                .iter()
                .map(|entry| entry.ct_bytes.clone())
                .collect::<Vec<_>>();
            let ciphertext_versions = cts_to_insert
                .iter()
                .map(|entry| entry.ciphertext_version)
                .collect::<Vec<_>>();
            let ciphertext_types = cts_to_insert
                .iter()
                .map(|entry| entry.ciphertext_type)
                .collect::<Vec<_>>();
            let cts_inserted = sqlx::query(
                "
            INSERT INTO ciphertexts_branch(handle, producer_block_hash, block_number, ciphertext, ciphertext_version, ciphertext_type)
            SELECT * FROM UNNEST($1::BYTEA[], $2::BYTEA[], $3::BIGINT[], $4::BYTEA[], $5::SMALLINT[], $6::SMALLINT[])
            ON CONFLICT (handle, ciphertext_version, producer_block_hash) DO NOTHING
            ",
            )
            .bind(&handles)
            .bind(&producer_block_hashes)
            .bind(&block_numbers)
            .bind(&ciphertexts)
            .bind(&ciphertext_versions)
            .bind(&ciphertext_types)
            .execute(trx.as_mut())
            .await
            .map_err(|err| {
                error!(target: "tfhe_worker", { error = %err }, "error while inserting new branch ciphertexts");
                err
            })?
            .rows_affected();
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
            let (
                handles_vec,
                (txn_ids_vec, producer_block_hashes_vec),
            ): (
                Vec<_>,
                (Vec<_>, Vec<_>),
            ) = handles_to_update.into_iter().unzip();
            let comp_updated = sqlx::query(
                "
            WITH requested_updates AS (
                SELECT *
                FROM unnest($1::BYTEA[], $2::BYTEA[], $3::BYTEA[])
                    AS t(output_handle, transaction_id, producer_block_hash)
            )
            UPDATE computations_branch
            SET is_completed = true, completed_at = CURRENT_TIMESTAMP
            FROM requested_updates r
            WHERE computations_branch.is_completed = false
              AND computations_branch.output_handle = r.output_handle
              AND computations_branch.transaction_id = r.transaction_id
              AND computations_branch.producer_block_hash = r.producer_block_hash
            ",
            )
            .bind(&handles_vec)
            .bind(&txn_ids_vec)
            .bind(&producer_block_hashes_vec)
            .execute(trx.as_mut())
            .await
            .map_err(|err| {
                error!(target: "tfhe_worker", { error = %err }, "error while updating branch computations as completed");
                err
            })?
            .rows_affected();
            Ok::<u64, Box<dyn std::error::Error + Send + Sync>>(comp_updated)
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
    producer_block_hash: &[u8],
    cerr: &(dyn std::error::Error + Send + Sync),
    trx: &mut sqlx::Transaction<'a, Postgres>,
    deps_mngr: &mut dependence_chain::LockMngr,
) -> Result<(), CoprocessorError> {
    WORKER_ERRORS_COUNTER.inc();
    let err_string = cerr.to_string();
    error!(target: "tfhe_worker", error = %err_string, output_handle = %format!("0x{}", hex::encode(output_handle)), "error while processing work item");
    telemetry::set_current_span_error(&err_string);

    let _ = sqlx::query(
        "
        UPDATE computations_branch
        SET is_error = true, error_message = $1
        WHERE output_handle = $2
          AND transaction_id = $3
          AND producer_block_hash = $4
        ",
    )
    .bind(&err_string)
    .bind(output_handle)
    .bind(transaction_id)
    .bind(producer_block_hash)
    .execute(trx.as_mut())
    .await?;

    deps_mngr.set_processing_error(Some(err_string)).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use std::time::Duration;

    use super::{
        add_branchless_ciphertext_metadata_fallback, build_current_branch_ancestry,
        dedupe_ciphertext_inserts, dedupe_ciphertext_inserts_inner,
        ensure_ciphertext_write_above_settled, ensure_not_same_block_db_dependency,
        query_work_rows_for_dcid, resolve_current_branch_candidates, resolve_dependency_metadata,
        select_ciphertext_candidate, settled_ciphertext_divergence, BatchExecutionContext,
        BlockAncestorRow, CiphertextCandidate, CiphertextDedupeEntry, CiphertextInsert,
        DependencyMetadata, ProducerCandidate,
    };
    use fhevm_engine_common::drift_revert::WatcherTimeouts;
    use fhevm_engine_common::telemetry::MetricsConfig;
    use serial_test::serial;
    use test_harness::instance::{setup_test_db, ImportMode};
    use tracing::Level;

    fn ciphertext_candidate(producer_hash: &[u8], ct_bytes: &[u8]) -> CiphertextCandidate {
        CiphertextCandidate {
            producer_block_hash: producer_hash.to_vec(),
            ciphertext_type: 4,
            ciphertext: ct_bytes.to_vec(),
        }
    }

    async fn insert_work_row(
        pool: &sqlx::PgPool,
        dcid: &[u8],
        transaction_id: &[u8],
        output_handle: &[u8],
        producer_block_hash: &[u8],
        block_number: Option<i64>,
        schedule_offset_secs: i32,
        is_completed: bool,
    ) {
        sqlx::query(
            "
            INSERT INTO computations_branch (
                output_handle,
                dependencies,
                fhe_operation,
                is_scalar,
                is_allowed,
                dependence_chain_id,
                transaction_id,
                host_chain_id,
                block_number,
                producer_block_hash,
                is_completed,
                is_error,
                schedule_order
            )
            VALUES (
                $1,
                ARRAY[]::BYTEA[],
                1,
                false,
                true,
                $2,
                $3,
                1,
                $4,
                $5,
                $6,
                false,
                NOW() + ($7 * INTERVAL '1 second')
            )
            ",
        )
        .bind(output_handle)
        .bind(dcid)
        .bind(transaction_id)
        .bind(block_number)
        .bind(producer_block_hash)
        .bind(is_completed)
        .bind(schedule_offset_secs)
        .execute(pool)
        .await
        .expect("insert computations_branch row");
    }

    fn cutover_guard_args(db_url: &str, branch_cutover_block: i64) -> crate::daemon_cli::Args {
        crate::daemon_cli::Args {
            run_bg_worker: true,
            worker_polling_interval_ms: 1000,
            generate_fhe_keys: false,
            key_cache_size: 4,
            coprocessor_fhe_threads: 4,
            tokio_threads: 2,
            pg_pool_max_connections: 1,
            metrics_addr: None,
            database_url: Some(db_url.into()),
            service_name: "tfhe-worker-test".to_string(),
            worker_id: None,
            dcid_ttl_sec: 30,
            dcid_timeslice_sec: 90,
            processed_dcid_ttl_sec: 0,
            dcid_cleanup_interval_sec: 0,
            dcid_max_no_progress_cycles: 2,
            dcid_ignore_dependency_count_threshold: 100,
            branch_cutover_block,
            log_level: Level::INFO,
            health_check_port: 0,
            metric_fhe_batch_latency: MetricsConfig::default(),
            drift_revert_watcher_timeouts: WatcherTimeouts {
                poll_query_timeout: Duration::from_secs(5),
                db_down_limit: Duration::from_secs(5),
            },
        }
    }

    async fn insert_legacy_computation(pool: &sqlx::PgPool, output_byte: u8, is_completed: bool) {
        sqlx::query(
            "
            INSERT INTO computations (
                tenant_id,
                output_handle,
                dependencies,
                fhe_operation,
                is_scalar,
                is_completed,
                transaction_id
            )
            VALUES (0, $1, ARRAY[]::BYTEA[], 1, false, $2, $3)
            ",
        )
        .bind(vec![output_byte; 32])
        .bind(is_completed)
        .bind(vec![output_byte.wrapping_add(1); 32])
        .execute(pool)
        .await
        .expect("insert legacy computation");
    }

    struct AllowZeroCutoverEnvGuard(Option<String>);

    impl AllowZeroCutoverEnvGuard {
        fn clear() -> Self {
            let previous = std::env::var("FHEVM_ALLOW_ZERO_CUTOVER").ok();
            std::env::remove_var("FHEVM_ALLOW_ZERO_CUTOVER");
            Self(previous)
        }
    }

    impl Drop for AllowZeroCutoverEnvGuard {
        fn drop(&mut self) {
            match &self.0 {
                Some(value) => std::env::set_var("FHEVM_ALLOW_ZERO_CUTOVER", value),
                None => std::env::remove_var("FHEVM_ALLOW_ZERO_CUTOVER"),
            }
        }
    }

    #[tokio::test]
    #[serial(db)]
    async fn zero_cutover_guard_allows_fresh_or_pending_only_db() {
        let _env = AllowZeroCutoverEnvGuard::clear();
        let test_instance = setup_test_db(ImportMode::None)
            .await
            .expect("valid db instance");
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(test_instance.db_url())
            .await
            .expect("connect test db");

        let args = cutover_guard_args(test_instance.db_url(), 0);
        super::ensure_cutover_safe(&args)
            .await
            .expect("fresh DB should allow cutover 0");

        insert_legacy_computation(&pool, 0x10, false).await;
        super::ensure_cutover_safe(&args)
            .await
            .expect("pending legacy rows alone should not block cutover 0");
    }

    #[tokio::test]
    #[serial(db)]
    async fn zero_cutover_guard_rejects_completed_legacy_rows() {
        let _env = AllowZeroCutoverEnvGuard::clear();
        let test_instance = setup_test_db(ImportMode::None)
            .await
            .expect("valid db instance");
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(test_instance.db_url())
            .await
            .expect("connect test db");
        insert_legacy_computation(&pool, 0x20, true).await;

        let args = cutover_guard_args(test_instance.db_url(), 0);
        let err = super::ensure_cutover_safe(&args)
            .await
            .expect_err("completed legacy row must reject cutover 0");
        let msg = err.to_string();
        assert!(msg.contains("FHEVM_BRANCH_CUTOVER_BLOCK is 0/unset"));
        assert!(msg.contains("legacy-executed computations"));
    }

    #[tokio::test]
    #[serial(db)]
    async fn cutover_guard_allows_explicit_nonzero_or_override() {
        let _env = AllowZeroCutoverEnvGuard::clear();
        let test_instance = setup_test_db(ImportMode::None)
            .await
            .expect("valid db instance");
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(test_instance.db_url())
            .await
            .expect("connect test db");
        insert_legacy_computation(&pool, 0x30, true).await;

        super::ensure_cutover_safe(&cutover_guard_args(test_instance.db_url(), 42))
            .await
            .expect("explicit nonzero cutover should bypass zero-cutover guard");

        std::env::set_var("FHEVM_ALLOW_ZERO_CUTOVER", "true");
        super::ensure_cutover_safe(&cutover_guard_args(test_instance.db_url(), 0))
            .await
            .expect("explicit allow env should permit cutover 0");
    }

    #[tokio::test]
    #[serial(db)]
    async fn query_work_rows_locks_one_branch_context_per_dcid() {
        let test_instance = setup_test_db(ImportMode::None)
            .await
            .expect("valid db instance");
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect(test_instance.db_url())
            .await
            .expect("connect test db");

        let dcid = vec![0xD0; 32];
        let first_hash = vec![0xA0; 32];
        let second_hash = vec![0xB0; 32];
        insert_work_row(
            &pool,
            &dcid,
            &[0x01; 32],
            &[0x11; 32],
            &first_hash,
            Some(10),
            0,
            false,
        )
        .await;
        insert_work_row(
            &pool,
            &dcid,
            &[0x02; 32],
            &[0x12; 32],
            &first_hash,
            Some(10),
            1,
            true,
        )
        .await;
        insert_work_row(
            &pool,
            &dcid,
            &[0x03; 32],
            &[0x13; 32],
            &second_hash,
            Some(10),
            2,
            false,
        )
        .await;

        let mut first_trx = pool.begin().await.expect("begin first transaction");
        let rows = query_work_rows_for_dcid(&mut first_trx, Some(dcid.clone()), 0)
            .await
            .expect("query work rows");

        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|row| row.producer_block_hash == first_hash));
        assert!(rows.iter().any(|row| row.output_handle == [0x11; 32]));
        assert!(rows.iter().any(|row| row.output_handle == [0x12; 32]));

        let mut second_trx = pool.begin().await.expect("begin second transaction");
        let locked_rows = query_work_rows_for_dcid(&mut second_trx, Some(dcid.clone()), 0)
            .await
            .expect("query while first context is locked");
        second_trx.rollback().await.expect("rollback second");
        assert!(
            locked_rows.is_empty(),
            "a concurrent acquisition must not skip a locked branch context and process a sibling fork"
        );
        first_trx.rollback().await.expect("rollback first");

        sqlx::query(
            "
            UPDATE computations_branch
            SET is_completed = TRUE
            WHERE dependence_chain_id = $1
              AND producer_block_hash = $2
            ",
        )
        .bind(&dcid)
        .bind(&first_hash)
        .execute(&pool)
        .await
        .expect("complete first branch context");

        let mut trx = pool.begin().await.expect("begin transaction");
        let rows = query_work_rows_for_dcid(&mut trx, Some(dcid), 0)
            .await
            .expect("query second work context");
        trx.rollback().await.expect("rollback");

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].producer_block_hash, second_hash);
    }

    #[tokio::test]
    #[serial(db)]
    async fn query_work_rows_for_dcid_respects_nonzero_cutover() {
        let test_instance = setup_test_db(ImportMode::None)
            .await
            .expect("valid db instance");
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(test_instance.db_url())
            .await
            .expect("connect test db");

        let branch_dcid = vec![0xD1; 32];
        let pre_cutover_hash = vec![0xA1; 32];
        let post_cutover_hash = vec![0xB1; 32];
        insert_work_row(
            &pool,
            &branch_dcid,
            &[0x01; 32],
            &[0x21; 32],
            &pre_cutover_hash,
            Some(9),
            0,
            false,
        )
        .await;
        insert_work_row(
            &pool,
            &branch_dcid,
            &[0x02; 32],
            &[0x22; 32],
            &post_cutover_hash,
            Some(10),
            1,
            false,
        )
        .await;

        let mut trx = pool.begin().await.expect("begin branch transaction");
        let rows = query_work_rows_for_dcid(&mut trx, Some(branch_dcid), 10)
            .await
            .expect("query post-cutover work");
        trx.rollback().await.expect("rollback branch transaction");

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].output_handle, [0x22; 32]);
        assert_eq!(rows[0].producer_block_hash, post_cutover_hash);
        assert_eq!(rows[0].block_number, Some(10));

        let branchless_dcid = vec![0xD2; 32];
        insert_work_row(
            &pool,
            &branchless_dcid,
            &[0x03; 32],
            &[0x23; 32],
            &[],
            None,
            0,
            false,
        )
        .await;
        insert_work_row(
            &pool,
            &branchless_dcid,
            &[0x04; 32],
            &[0x24; 32],
            &pre_cutover_hash,
            Some(9),
            1,
            false,
        )
        .await;

        let mut trx = pool.begin().await.expect("begin branchless transaction");
        let rows = query_work_rows_for_dcid(&mut trx, Some(branchless_dcid), 10)
            .await
            .expect("query branchless work");
        trx.rollback()
            .await
            .expect("rollback branchless transaction");

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].output_handle, [0x23; 32]);
        assert!(rows[0].producer_block_hash.is_empty());
        assert_eq!(rows[0].block_number, None);
    }

    #[test]
    fn select_ciphertext_candidate_prefers_exact_branch_match() {
        let candidates = vec![
            ciphertext_candidate(&[], b"user input"),
            ciphertext_candidate(&[0xAA; 32], b"computed"),
        ];
        let metadata = DependencyMetadata {
            producer_block_hash: vec![0xAA; 32],
            block_number: Some(10),
            settled_producer_block_hashes: Vec::new(),
        };

        let selected = select_ciphertext_candidate(&candidates, &metadata)
            .expect("exact branch match should resolve");
        assert_eq!(selected.ciphertext.as_slice(), b"computed");
    }

    #[test]
    fn select_ciphertext_candidate_resolves_branchless_dependency() {
        let candidates = vec![ciphertext_candidate(&[], b"user input")];
        let metadata = DependencyMetadata {
            producer_block_hash: Vec::new(),
            block_number: None,
            settled_producer_block_hashes: Vec::new(),
        };

        let selected = select_ciphertext_candidate(&candidates, &metadata)
            .expect("branchless dependency should resolve to branchless candidate");
        assert_eq!(selected.ciphertext.as_slice(), b"user input");
    }

    #[test]
    fn select_ciphertext_candidate_rejects_branchless_row_for_branchful_dependency() {
        let candidates = vec![ciphertext_candidate(&[], b"user input")];
        let metadata = DependencyMetadata {
            producer_block_hash: vec![0xAA; 32],
            block_number: Some(10),
            settled_producer_block_hashes: Vec::new(),
        };

        assert!(select_ciphertext_candidate(&candidates, &metadata).is_none());
    }

    #[test]
    fn select_ciphertext_candidate_rejects_other_branch_rows() {
        let candidates = vec![ciphertext_candidate(&[0xBB; 32], b"other branch")];
        let metadata = DependencyMetadata {
            producer_block_hash: vec![0xAA; 32],
            block_number: Some(10),
            settled_producer_block_hashes: Vec::new(),
        };

        assert!(select_ciphertext_candidate(&candidates, &metadata).is_none());
    }

    #[test]
    fn same_block_db_dependency_is_invariant_violation() {
        let batch_context = BatchExecutionContext {
            host_chain_id: 42,
            block_number: Some(10),
            producer_block_hash: vec![0xAA; 32],
        };
        let metadata = DependencyMetadata {
            producer_block_hash: vec![0xAA; 32],
            block_number: Some(10),
            settled_producer_block_hashes: Vec::new(),
        };

        assert!(super::is_same_block_materialized_dependency(
            &metadata,
            &batch_context
        ));

        let err = ensure_not_same_block_db_dependency(&[0x42; 32], &metadata, &batch_context)
            .expect_err("same-block DB fetch must be rejected");
        let msg = err.to_string();
        assert!(msg.contains("same-block DB fetch invariant violation"));
        assert!(msg.contains("same-block dependencies must be selected in the worker component"));
    }

    #[test]
    fn settled_ciphertext_divergence_detects_byte_mismatch() {
        let candidates = vec![
            ciphertext_candidate(&[0xAA; 32], b"settled-a"),
            ciphertext_candidate(&[0xBB; 32], b"settled-b"),
            ciphertext_candidate(&[0xCC; 32], b"live-fork"),
        ];
        let metadata = DependencyMetadata {
            producer_block_hash: vec![0xBB; 32],
            block_number: Some(100),
            settled_producer_block_hashes: vec![vec![0xAA; 32], vec![0xBB; 32]],
        };

        assert!(settled_ciphertext_divergence(&candidates, &metadata).is_some());
    }

    #[test]
    fn settled_ciphertext_divergence_ignores_non_settled_fork_variant() {
        let candidates = vec![
            ciphertext_candidate(&[0xAA; 32], b"settled"),
            ciphertext_candidate(&[0xBB; 32], b"live-fork"),
        ];
        let metadata = DependencyMetadata {
            producer_block_hash: vec![0xAA; 32],
            block_number: Some(100),
            settled_producer_block_hashes: vec![vec![0xAA; 32]],
        };

        assert!(settled_ciphertext_divergence(&candidates, &metadata).is_none());
    }

    fn dedupe_entry(
        handle_byte: u8,
        producer_hash_byte: u8,
        ct_bytes: &[u8],
        ciphertext_type: i16,
        tid_byte: u8,
    ) -> CiphertextDedupeEntry {
        CiphertextDedupeEntry {
            handle: vec![handle_byte; 32],
            producer_block_hash: vec![producer_hash_byte; 32],
            block_number: Some(10),
            ct_bytes: ct_bytes.to_vec(),
            ciphertext_version: 1,
            ciphertext_type,
            transaction_id: vec![tid_byte; 32],
        }
    }

    #[test]
    fn dedupe_ciphertext_inserts_keeps_identical_duplicate() {
        let entry_a = dedupe_entry(0xAA, 0xBB, b"same", 4, 0x01);
        let entry_b = dedupe_entry(0xAA, 0xBB, b"same", 4, 0x02);

        let deduped = dedupe_ciphertext_inserts(vec![entry_a, entry_b])
            .expect("identical duplicates should dedupe");

        assert_eq!(deduped.len(), 1);
        // Bytes must match (they were the same), and the row carries the
        // single retained entry.
        assert_eq!(deduped[0].ct_bytes.as_slice(), b"same");
    }

    #[test]
    fn dedupe_ciphertext_inserts_strict_errors_on_byte_divergence() {
        let error = dedupe_ciphertext_inserts_inner(
            vec![
                dedupe_entry(0xAA, 0xBB, b"left", 4, 0x01),
                dedupe_entry(0xAA, 0xBB, b"right", 4, 0x02),
            ],
            true,
        )
        .expect_err("byte divergence should fail in strict mode");

        let msg = error.to_string();
        assert!(
            msg.contains("multi-producer ciphertext divergence"),
            "unexpected error message: {msg}"
        );
        // Both producer tids must appear in the error for triage.
        assert!(msg.contains(&hex::encode([0x01u8; 32])), "msg: {msg}");
        assert!(msg.contains(&hex::encode([0x02u8; 32])), "msg: {msg}");
    }

    #[test]
    fn dedupe_ciphertext_inserts_strict_errors_on_type_divergence() {
        let error = dedupe_ciphertext_inserts_inner(
            vec![
                dedupe_entry(0xAA, 0xBB, b"same", 4, 0x01),
                dedupe_entry(0xAA, 0xBB, b"same", 5, 0x02),
            ],
            true,
        )
        .expect_err("type divergence should fail in strict mode");

        assert!(error
            .to_string()
            .contains("multi-producer ciphertext divergence"));
    }

    /// In non-strict (production-default) mode a byte divergence between
    /// two producers of the same handle is logged + counted, not raised.
    /// The lex-smallest `transaction_id` bytes are kept — matching the
    /// canonical-producer choice in `scheduler::dfg::DFComponentGraph::build`
    /// so in-memory consumer routing and persisted bytes stay consistent.
    #[test]
    fn dedupe_ciphertext_inserts_non_strict_keeps_lex_smallest_tid_bytes() {
        let deduped = dedupe_ciphertext_inserts_inner(
            vec![
                dedupe_entry(0xAA, 0xBB, b"high-tid-bytes", 4, 0x02),
                dedupe_entry(0xAA, 0xBB, b"low-tid-bytes", 4, 0x01),
            ],
            false,
        )
        .expect("non-strict mode must not fail on byte divergence");

        assert_eq!(deduped.len(), 1);
        assert_eq!(
            deduped[0].ct_bytes.as_slice(),
            b"low-tid-bytes",
            "canonical (lex-smallest tid) bytes must win"
        );
    }

    /// Non-strict type divergence: canonical (lex-smallest tid) type wins.
    #[test]
    fn dedupe_ciphertext_inserts_non_strict_keeps_lex_smallest_tid_type() {
        let deduped = dedupe_ciphertext_inserts_inner(
            vec![
                dedupe_entry(0xAA, 0xBB, b"same", 5, 0x02),
                dedupe_entry(0xAA, 0xBB, b"same", 4, 0x01),
            ],
            false,
        )
        .expect("non-strict mode must not fail on type divergence");

        assert_eq!(deduped.len(), 1);
        assert_eq!(
            deduped[0].ciphertext_type, 4,
            "canonical (lex-smallest tid) type must win"
        );
    }

    /// Canonical selection must not depend on Vec insertion order.
    #[test]
    fn dedupe_ciphertext_inserts_canonical_independent_of_input_order() {
        let forward = dedupe_ciphertext_inserts_inner(
            vec![
                dedupe_entry(0xAA, 0xBB, b"low-bytes", 4, 0x01),
                dedupe_entry(0xAA, 0xBB, b"high-bytes", 4, 0x02),
            ],
            false,
        )
        .expect("forward order");
        let reverse = dedupe_ciphertext_inserts_inner(
            vec![
                dedupe_entry(0xAA, 0xBB, b"high-bytes", 4, 0x02),
                dedupe_entry(0xAA, 0xBB, b"low-bytes", 4, 0x01),
            ],
            false,
        )
        .expect("reverse order");

        let forward_bytes = &forward[0].ct_bytes;
        let reverse_bytes = &reverse[0].ct_bytes;
        assert_eq!(forward_bytes, reverse_bytes);
        assert_eq!(forward_bytes.as_slice(), b"low-bytes");
    }

    /// Different `(handle, producer_block_hash, ciphertext_version)` keys
    /// must coexist — dedupe must not collapse rows with different keys.
    #[test]
    fn dedupe_ciphertext_inserts_preserves_distinct_keys() {
        let deduped = dedupe_ciphertext_inserts_inner(
            vec![
                dedupe_entry(0xAA, 0xBB, b"a", 4, 0x01),
                dedupe_entry(0xAA, 0xCC, b"b", 4, 0x01),
                dedupe_entry(0xBB, 0xBB, b"c", 4, 0x01),
            ],
            false,
        )
        .expect("three distinct keys");
        assert_eq!(deduped.len(), 3);
    }

    #[test]
    fn builds_current_branch_ancestry_from_parent_links() {
        let rows = vec![
            BlockAncestorRow {
                block_hash: vec![3],
                parent_hash: Some(vec![2]),
                block_number: 3,
            },
            BlockAncestorRow {
                block_hash: vec![2],
                parent_hash: Some(vec![1]),
                block_number: 2,
            },
            BlockAncestorRow {
                block_hash: vec![1],
                parent_hash: Some(vec![0]),
                block_number: 1,
            },
        ];

        let ancestry =
            build_current_branch_ancestry(rows, &[3], 3, 1).expect("ancestry should resolve");

        assert_eq!(ancestry.get(&3), Some(&vec![3]));
        assert_eq!(ancestry.get(&2), Some(&vec![2]));
        assert_eq!(ancestry.get(&1), Some(&vec![1]));
    }

    #[test]
    fn errors_when_parent_chain_is_incomplete() {
        let rows = vec![BlockAncestorRow {
            block_hash: vec![3],
            parent_hash: Some(vec![2]),
            block_number: 3,
        }];

        let error =
            build_current_branch_ancestry(rows, &[3], 3, 1).expect_err("ancestry should fail");

        assert!(error.to_string().contains("missing block 2"));
    }

    #[test]
    fn errors_when_parent_hash_is_missing_before_min_block() {
        let rows = vec![
            BlockAncestorRow {
                block_hash: vec![3],
                parent_hash: None,
                block_number: 3,
            },
            BlockAncestorRow {
                block_hash: vec![2],
                parent_hash: Some(vec![1]),
                block_number: 2,
            },
        ];

        let error =
            build_current_branch_ancestry(rows, &[3], 3, 2).expect_err("ancestry should fail");

        assert!(error.to_string().contains("missing parent_hash"));
    }

    #[test]
    fn resolve_current_branch_candidates_matches_only_current_branch_rows() {
        let candidates = vec![
            ProducerCandidate {
                block_number: Some(10),
                producer_block_hash: vec![0xAA],
            },
            ProducerCandidate {
                block_number: Some(10),
                producer_block_hash: vec![0xBB],
            },
        ];
        let ancestry = HashMap::from([(10_i64, vec![0xAA])]);

        let matches = resolve_current_branch_candidates(&candidates, &ancestry);

        assert_eq!(matches.len(), 1);
        assert!(matches.contains(&vec![0xAA]));
    }

    #[test]
    fn resolve_dependency_metadata_prefers_branch_match_over_branchless() {
        let candidates = vec![
            ProducerCandidate {
                block_number: Some(10),
                producer_block_hash: vec![0xAA],
            },
            ProducerCandidate {
                block_number: None,
                producer_block_hash: vec![],
            },
        ];
        let ancestry = HashMap::from([(10_i64, vec![0xAA])]);

        let metadata = resolve_dependency_metadata(&candidates, &ancestry, 0, -1)
            .expect("no ambiguity")
            .expect("resolved");
        assert_eq!(metadata.producer_block_hash, vec![0xAA]);
        assert_eq!(metadata.block_number, Some(10));
    }

    #[test]
    fn resolve_dependency_metadata_resolves_pre_cutover_candidates_as_branchless() {
        // The handle was produced by the legacy pipeline below the cutover:
        // its real block hash never matches branch state, the bytes live in
        // branchless rows or the legacy ciphertexts table.
        let candidates = vec![ProducerCandidate {
            block_number: Some(10),
            producer_block_hash: vec![0xAA],
        }];
        let ancestry = HashMap::new();

        let metadata = resolve_dependency_metadata(&candidates, &ancestry, 100, -1)
            .expect("no ambiguity")
            .expect("resolved");
        assert!(metadata.producer_block_hash.is_empty());
        assert_eq!(metadata.block_number, None);
    }

    #[test]
    fn resolve_dependency_metadata_resolves_branchless_rows_at_any_cutover() {
        // Backfilled / user-input rows carry an empty producer hash and
        // resolve as branchless regardless of their block number.
        let candidates = vec![ProducerCandidate {
            block_number: Some(500),
            producer_block_hash: vec![],
        }];
        let ancestry = HashMap::new();

        let metadata = resolve_dependency_metadata(&candidates, &ancestry, 100, -1)
            .expect("no ambiguity")
            .expect("resolved");
        assert!(metadata.producer_block_hash.is_empty());
        assert_eq!(metadata.block_number, None);
    }

    #[test]
    fn resolve_dependency_metadata_ignores_pre_cutover_candidates_for_ambiguity() {
        // Two pre-cutover rows plus one current-branch row: the pre-cutover
        // rows collapse into the branchless bucket and must not trigger the
        // ambiguity error.
        let candidates = vec![
            ProducerCandidate {
                block_number: Some(10),
                producer_block_hash: vec![0xAA],
            },
            ProducerCandidate {
                block_number: Some(11),
                producer_block_hash: vec![0xBB],
            },
            ProducerCandidate {
                block_number: Some(120),
                producer_block_hash: vec![0xCC],
            },
        ];
        let ancestry = HashMap::from([(120_i64, vec![0xCC])]);

        let metadata = resolve_dependency_metadata(&candidates, &ancestry, 100, -1)
            .expect("no ambiguity")
            .expect("resolved");
        assert_eq!(metadata.producer_block_hash, vec![0xCC]);
        assert_eq!(metadata.block_number, Some(120));
    }

    #[test]
    fn resolve_dependency_metadata_uses_settled_candidate_without_ancestry() {
        let candidates = vec![
            ProducerCandidate {
                block_number: Some(90),
                producer_block_hash: vec![0xAA],
            },
            ProducerCandidate {
                block_number: Some(110),
                producer_block_hash: vec![0xBB],
            },
        ];
        let ancestry = HashMap::new();

        let metadata = resolve_dependency_metadata(&candidates, &ancestry, 0, 100)
            .expect("metadata resolution should succeed")
            .expect("settled candidate should resolve");

        assert_eq!(metadata.producer_block_hash, vec![0xAA]);
        assert_eq!(metadata.block_number, Some(90));
        assert_eq!(metadata.settled_producer_block_hashes, vec![vec![0xAA]]);
    }

    #[test]
    fn resolve_dependency_metadata_prefers_recent_current_branch_over_settled() {
        let candidates = vec![
            ProducerCandidate {
                block_number: Some(90),
                producer_block_hash: vec![0xAA],
            },
            ProducerCandidate {
                block_number: Some(110),
                producer_block_hash: vec![0xBB],
            },
        ];
        let ancestry = HashMap::from([(110_i64, vec![0xBB])]);

        let metadata = resolve_dependency_metadata(&candidates, &ancestry, 0, 100)
            .expect("metadata resolution should succeed")
            .expect("current branch candidate should resolve");

        assert_eq!(metadata.producer_block_hash, vec![0xBB]);
        assert!(metadata.settled_producer_block_hashes.is_empty());
    }

    #[test]
    fn resolve_current_branch_candidates_ignores_rows_without_block_number() {
        let candidates = vec![
            ProducerCandidate {
                block_number: Some(10),
                producer_block_hash: vec![0xAA],
            },
            ProducerCandidate {
                block_number: None,
                producer_block_hash: vec![],
            },
        ];
        let ancestry = HashMap::from([(10_i64, vec![0xAA])]);

        let matches = resolve_current_branch_candidates(&candidates, &ancestry);

        assert_eq!(matches.len(), 1);
        assert!(matches.contains(&vec![0xAA]));
    }

    #[test]
    fn allowed_handle_metadata_keeps_exact_hash() {
        let candidates = vec![ProducerCandidate {
            block_number: Some(10),
            producer_block_hash: vec![0xAA],
        }];
        let ancestry = HashMap::from([(10_i64, vec![0xAA])]);

        let metadata = resolve_dependency_metadata(&candidates, &ancestry, 0, -1)
            .expect("metadata resolution should succeed")
            .expect("allowed handle should resolve");

        assert_eq!(metadata.producer_block_hash, vec![0xAA]);
    }

    #[test]
    fn allowed_handle_metadata_keeps_newest_current_branch_hash() {
        let candidates = vec![
            ProducerCandidate {
                block_number: Some(10),
                producer_block_hash: vec![0xAA],
            },
            ProducerCandidate {
                block_number: Some(11),
                producer_block_hash: vec![0xAB],
            },
        ];
        let ancestry = HashMap::from([(10_i64, vec![0xAA]), (11_i64, vec![0xAB])]);

        let metadata = resolve_dependency_metadata(&candidates, &ancestry, 0, -1)
            .expect("metadata resolution should succeed")
            .expect("allowed handle should resolve");

        assert_eq!(metadata.producer_block_hash, vec![0xAB]);
    }

    #[test]
    fn resolve_dependency_metadata_returns_none_for_non_current_branch() {
        let candidates = vec![ProducerCandidate {
            block_number: Some(10),
            producer_block_hash: vec![0xAA],
        }];
        let ancestry = HashMap::from([(10_i64, vec![0xBB])]);

        let metadata = resolve_dependency_metadata(&candidates, &ancestry, 0, -1)
            .expect("metadata resolution should succeed");

        assert!(metadata.is_none());
    }

    #[test]
    fn branchless_ciphertext_fallback_adds_missing_metadata_only() {
        let branchless_handle = vec![0x11; 32];
        let branchful_handle = vec![0x22; 32];
        let absent_handle = vec![0x33; 32];
        let mut metadata = HashMap::from([(
            branchful_handle.clone(),
            DependencyMetadata {
                producer_block_hash: vec![0xAA; 32],
                block_number: Some(10),
                settled_producer_block_hashes: Vec::new(),
            },
        )]);
        let branchless_ciphertext_rows =
            HashSet::from([branchless_handle.clone(), branchful_handle.clone()]);

        add_branchless_ciphertext_metadata_fallback(
            &mut metadata,
            &[
                branchless_handle.clone(),
                branchful_handle.clone(),
                absent_handle.clone(),
            ],
            &branchless_ciphertext_rows,
            &HashSet::new(),
        );

        let branchless = metadata
            .get(&branchless_handle)
            .expect("branchless ciphertext row should add metadata");
        assert!(branchless.producer_block_hash.is_empty());
        assert_eq!(branchless.block_number, None);

        let branchful = metadata
            .get(&branchful_handle)
            .expect("existing metadata should remain");
        assert_eq!(branchful.producer_block_hash, vec![0xAA; 32]);
        assert!(!metadata.contains_key(&absent_handle));
    }

    #[test]
    fn branchless_ciphertext_fallback_does_not_mask_unresolved_branch_metadata() {
        let unresolved_branch_handle = vec![0x44; 32];
        let branchless_ciphertext_rows = HashSet::from([unresolved_branch_handle.clone()]);
        let handles_with_candidate_metadata = HashSet::from([unresolved_branch_handle.clone()]);
        let mut metadata = HashMap::new();

        add_branchless_ciphertext_metadata_fallback(
            &mut metadata,
            std::slice::from_ref(&unresolved_branch_handle),
            &branchless_ciphertext_rows,
            &handles_with_candidate_metadata,
        );

        assert!(
            !metadata.contains_key(&unresolved_branch_handle),
            "branchless ciphertext fallback must not mask unresolved branch-aware metadata"
        );
    }

    fn ciphertext_insert(
        producer_block_hash: Vec<u8>,
        block_number: Option<i64>,
    ) -> CiphertextInsert {
        CiphertextInsert {
            handle: vec![0x42; 32],
            producer_block_hash,
            block_number,
            ct_bytes: b"ct".to_vec(),
            ciphertext_version: 1,
            ciphertext_type: 4,
        }
    }

    #[test]
    fn write_guard_rejects_branch_ciphertext_at_or_below_settlement() {
        let err =
            ensure_ciphertext_write_above_settled(&ciphertext_insert(vec![0xAA; 32], Some(10)), 10)
                .expect_err("settled branch write should be rejected");

        assert!(err
            .to_string()
            .contains("refusing settled branch ciphertext write"));
    }

    #[test]
    fn write_guard_allows_branchless_and_future_branch_ciphertexts() {
        ensure_ciphertext_write_above_settled(&ciphertext_insert(vec![], None), 10)
            .expect("branchless rows are outside settlement");
        ensure_ciphertext_write_above_settled(&ciphertext_insert(vec![0xAA; 32], Some(11)), 10)
            .expect("future branch row should be accepted");
    }
}
