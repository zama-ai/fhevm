use crate::dependence_chain::{self};
use crate::types::CoprocessorError;
use crate::{db_queries::populate_cache_with_tenant_keys, types::TfheTenantKeys};
use fhevm_engine_common::tfhe_ops::check_fhe_operand_types;
use fhevm_engine_common::types::{FhevmError, Handle, SupportedFheCiphertexts};
use fhevm_engine_common::{tfhe_ops::current_ciphertext_version, types::SupportedFheOperations};
use itertools::Itertools;
use lazy_static::lazy_static;
use opentelemetry::trace::{Span, TraceContextExt, Tracer};
use opentelemetry::KeyValue;
use prometheus::{register_histogram, register_int_counter, Histogram, IntCounter};
use scheduler::dfg::types::{DFGTxInput, SchedulerError};
use scheduler::dfg::{build_component_nodes, ComponentNode, DFComponentGraph, DFGOp};
use scheduler::dfg::{scheduler::Scheduler, types::DFGTaskInput};
use sqlx::types::Uuid;
use sqlx::Postgres;
use sqlx::{postgres::PgListener, query, Acquire};
use std::time::SystemTime;
use std::{
    collections::{BTreeSet, HashMap},
    num::NonZeroUsize,
};
use tracing::{debug, error, info, warn};

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
            error!(target: "tfhe_worker", { error = cycle_error }, "Error in background worker, retrying shortly");
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    }
}

async fn tfhe_worker_cycle(
    args: &crate::daemon_cli::Args,
    worker_id: Uuid,
    health_check: crate::health_check::HealthCheck,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let tracer = opentelemetry::global::tracer("tfhe_worker");
    let tenant_key_cache: std::sync::Arc<tokio::sync::RwLock<lru::LruCache<i32, TfheTenantKeys>>> =
        std::sync::Arc::new(tokio::sync::RwLock::new(lru::LruCache::new(
            NonZeroUsize::new(args.tenant_key_cache_size as usize).unwrap(),
        )));
    let db_url = args.database_url.clone().unwrap_or_default();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(args.pg_pool_max_connections)
        .connect(db_url.as_str())
        .await?;
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
    populate_cache_with_tenant_keys(vec![1i32], &pool, &tenant_key_cache).await?;
    let mut immedially_poll_more_work = false;
    loop {
        // only if previous iteration had no work done do the wait
        if !immedially_poll_more_work {
            tokio::select! {
                _ = listener.try_recv() => {
                    WORK_ITEMS_NOTIFICATIONS_COUNTER.inc();
                    info!(target: "tfhe_worker", "Received work_available notification from postgres");
                },
                _ = tokio::time::sleep(tokio::time::Duration::from_millis(args.worker_polling_interval_ms)) => {
                    WORK_ITEMS_POLL_COUNTER.inc();
                    debug!(target: "tfhe_worker", "Polling the database for more work on timer");
                },
            };
        }

        #[cfg(feature = "bench")]
        let now = std::time::SystemTime::now();
        let loop_span = tracer.start("worker_iteration");
        let loop_ctx = opentelemetry::Context::current_with_span(loop_span);
        let mut s = tracer.start_with_context("acquire_connection", &loop_ctx);
        let mut conn = pool.acquire().await?;
        s.end();
        let mut s = tracer.start_with_context("begin_transaction", &loop_ctx);
        let mut trx = conn.begin().await?;
        s.end();

        // Query for transactions to execute, and if relevant the associated keys
        let (mut transactions, mut unneeded_handles, has_more_work) = query_for_work(
            args,
            &health_check,
            &mut trx,
            &mut dcid_mngr,
            &tracer,
            &loop_ctx,
        )
        .await?;
        if has_more_work {
            // We've fetched work, so we'll poll again without waiting
            // for a notification after this cycle.
            immedially_poll_more_work = true;
        } else {
            dcid_mngr.release_current_lock(true).await?;
            dcid_mngr.do_cleanup().await?;

            // Lock another dependence chain if available and
            // continue processing without waiting for notification
            let mut s = tracer.start_with_context("query_dependence_chain", &loop_ctx);

            let (dependence_chain_id, _) = dcid_mngr.acquire_next_lock().await?;
            immedially_poll_more_work = dependence_chain_id.is_some();

            s.set_attribute(KeyValue::new(
                "dependence_chain_id",
                format!("{:?}", dependence_chain_id.as_ref().map(hex::encode)),
            ));
            s.end();

            continue;
        }
        query_tenants_and_keys(
            &transactions,
            &tenant_key_cache,
            &mut trx,
            &tracer,
            &loop_ctx,
        )
        .await?;

        // Execute transactions segregated by tenant
        for (tenant_id, ref mut tenant_txs) in transactions.iter_mut() {
            if dcid_mngr
                .extend_or_release_current_lock(false)
                .await?
                .is_none()
            {
                // best-effort attempt to extend the lock and prevent other replicas from trying to lock the same DCID.
                // Worst-case scenario, it returns None if the lock has expired.
                // However, the worker has already secured exclusive access to the txn computations in the Computations table.
                if dcid_mngr.enabled() {
                    warn!(target: "tfhe_worker", tenant_id = %tenant_id, "Lost dcid lock while processing transactions, but continuing since computations are locked");
                }
            }

            let mut tx_graph = build_transaction_graph_and_execute(
                tenant_id,
                tenant_txs,
                &tenant_key_cache,
                &health_check,
                &mut trx,
                &tracer,
                &loop_ctx,
            )
            .await?;
            upload_transaction_graph_results(
                tenant_id,
                &mut tx_graph,
                &mut unneeded_handles,
                &mut trx,
                &mut dcid_mngr,
                &tracer,
                &loop_ctx,
            )
            .await?;
        }
        s.end();
        trx.commit().await?;
        let _guard = loop_ctx.attach();
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

async fn query_tenants_and_keys<'a>(
    transactions: &[(i32, Vec<ComponentNode>)],
    tenant_key_cache: &std::sync::Arc<tokio::sync::RwLock<lru::LruCache<i32, TfheTenantKeys>>>,
    trx: &mut sqlx::Transaction<'a, Postgres>,
    tracer: &opentelemetry::global::BoxedTracer,
    loop_ctx: &opentelemetry::Context,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut s = tracer.start_with_context("populate_key_cache", loop_ctx);
    let mut tenants_to_query: BTreeSet<i32> = BTreeSet::new();
    let mut keys_to_query: BTreeSet<i32> = BTreeSet::new();
    let key_cache = tenant_key_cache.read().await;
    for (tenant_id, _) in transactions.iter() {
        let _ = tenants_to_query.insert(*tenant_id);
        if !key_cache.contains(tenant_id) {
            let _ = keys_to_query.insert(*tenant_id);
        }
    }
    drop(key_cache);
    let tenants_to_query = tenants_to_query.into_iter().collect::<Vec<_>>();
    let keys_to_query = keys_to_query.into_iter().collect::<Vec<_>>();
    s.set_attribute(KeyValue::new("keys_to_query", keys_to_query.len() as i64));
    s.set_attribute(KeyValue::new(
        "tenants_to_query",
        tenants_to_query.len() as i64,
    ));
    populate_cache_with_tenant_keys(keys_to_query, trx.as_mut(), tenant_key_cache).await?;
    s.end();
    Ok(())
}

async fn query_ciphertexts<'a>(
    cts_to_query: &[Vec<u8>],
    tenant_id: i32,
    trx: &mut sqlx::Transaction<'a, Postgres>,
    tracer: &opentelemetry::global::BoxedTracer,
    loop_ctx: &opentelemetry::Context,
) -> Result<HashMap<Vec<u8>, (i16, Vec<u8>)>, Box<dyn std::error::Error + Send + Sync>> {
    let mut s = tracer.start_with_context("query_ciphertext_batch", loop_ctx);
    s.set_attribute(KeyValue::new("cts_to_query", cts_to_query.len() as i64));
    // TODO: select all the ciphertexts where they're contained in the tuples
    let ciphertexts_rows = query!(
        "
                SELECT tenant_id, handle, ciphertext, ciphertext_type
                FROM ciphertexts
                WHERE tenant_id = $1
                AND handle = ANY($2::BYTEA[])
            ",
        &tenant_id,
        &cts_to_query
    )
    .fetch_all(trx.as_mut())
    .await
    .map_err(|err| {
        error!(target: "tfhe_worker", { error = %err }, "error while querying ciphertexts");
        err
    })?;

    s.end();
    // index ciphertexts in hashmap
    let mut ciphertext_map: HashMap<Vec<u8>, (i16, Vec<u8>)> =
        HashMap::with_capacity(ciphertexts_rows.len());
    for row in &ciphertexts_rows {
        let _ = ciphertext_map.insert(
            row.handle.clone(),
            (row.ciphertext_type, row.ciphertext.clone()),
        );
    }
    Ok(ciphertext_map)
}

// Update uncomputable ops schedule orders
async fn update_uncomputable_handles<'a>(
    uncomputable: Vec<(Handle, Handle)>,
    tenant_id: i32,
    trx: &mut sqlx::Transaction<'a, Postgres>,
    tracer: &opentelemetry::global::BoxedTracer,
    loop_ctx: &opentelemetry::Context,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut s = tracer.start_with_context("update_unschedulable_computations", loop_ctx);
    let (handles, transactions): (Vec<_>, Vec<_>) = uncomputable.into_iter().unzip();

    s.set_attribute(KeyValue::new("tenant_id", tenant_id as i64));
    s.set_attributes(
        handles
            .iter()
            .map(|h| KeyValue::new("handle", format!("0x{}", hex::encode(h)))),
    );
    s.set_attributes(
        transactions
            .iter()
            .map(|tid| KeyValue::new("transaction_id", format!("0x{}", hex::encode(tid)))),
    );
    let _ = query!(
        "
        UPDATE computations
           SET schedule_order = CURRENT_TIMESTAMP + INTERVAL '1 second' * uncomputable_counter,
               uncomputable_counter = LEAST(uncomputable_counter * 2, 32000)::SMALLINT
         WHERE tenant_id = $1
           AND (output_handle, transaction_id) IN (
              SELECT * FROM unnest($2::BYTEA[], $3::BYTEA[])
           )
        ",
        tenant_id,
        &handles,
	&transactions
    )
        .execute(trx.as_mut())
        .await.map_err(|err| {
            error!(target: "tfhe_worker", { tenant_id = tenant_id, error = %err }, "error while marking computations as unschedulable");
            err
        })?;
    s.end();
    Ok(())
}

async fn query_for_work<'a>(
    args: &crate::daemon_cli::Args,
    health_check: &crate::health_check::HealthCheck,
    trx: &mut sqlx::Transaction<'a, Postgres>,
    deps_chain_mngr: &mut dependence_chain::LockMngr,
    tracer: &opentelemetry::global::BoxedTracer,
    loop_ctx: &opentelemetry::Context,
) -> Result<
    (Vec<(i32, Vec<ComponentNode>)>, Vec<(Handle, Handle)>, bool),
    Box<dyn std::error::Error + Send + Sync>,
> {
    let mut s = tracer.start_with_context("query_dependence_chain", loop_ctx);

    // Lock dependence chain
    let (dependence_chain_id, locking_reason) =
        match deps_chain_mngr.extend_or_release_current_lock(true).await? {
            // If there is a current lock, we extend it and use its dependence_chain_id
            Some((id, reason)) => (Some(id), reason),
            None => deps_chain_mngr.acquire_next_lock().await?,
        };

    if deps_chain_mngr.enabled() && dependence_chain_id.is_none() {
        // No dependence chain to lock, so no work to do
        health_check.update_db_access();
        health_check.update_activity();
        info!(target: "tfhe_worker", "No dcid found to process");
        return Ok((vec![], vec![], false));
    }

    s.set_attribute(KeyValue::new(
        "dependence_chain_id",
        format!("{:?}", dependence_chain_id.as_ref().map(hex::encode)),
    ));
    s.end();

    // This query locks our work items so other worker doesn't select them.
    let mut s = tracer.start_with_context("query_work_items", loop_ctx);

    let transaction_batch_size = args.work_items_batch_size;

    let started_at = SystemTime::now();
    let the_work = query!(
        "
-- Acquire all computations from a transaction set
SELECT
  c.tenant_id, 
  c.output_handle, 
  c.dependencies, 
  c.fhe_operation, 
  c.is_scalar,
  c.is_allowed, 
  c.dependence_chain_id,
  c.transaction_id
FROM computations c
WHERE c.transaction_id IN (
    SELECT DISTINCT
      c_creation_order.transaction_id
    FROM (
      SELECT transaction_id
      FROM computations 
      WHERE is_completed = FALSE
        AND is_error = FALSE
        AND is_allowed = TRUE
        AND ($1::bytea IS NULL OR dependence_chain_id = $1)
      ORDER BY created_at
      LIMIT $2
    ) as c_creation_order
   UNION
    SELECT DISTINCT
      c_schedule_order.transaction_id
    FROM (
      SELECT transaction_id
      FROM computations 
      WHERE is_completed = FALSE
        AND is_error = FALSE
        AND is_allowed = TRUE
        AND ($1::bytea IS NULL OR dependence_chain_id = $1)
      ORDER BY schedule_order
      LIMIT $2
    ) as c_schedule_order
  )
FOR UPDATE SKIP LOCKED            ",
        dependence_chain_id,
        transaction_batch_size as i32,
    )
    .fetch_all(trx.as_mut())
    .await
    .map_err(|err| {
        error!(target: "tfhe_worker", { error = %err }, "error while querying work items");
        err
    })?;

    WORK_ITEMS_QUERY_HISTOGRAM.observe(started_at.elapsed().unwrap_or_default().as_secs_f64());
    s.set_attribute(KeyValue::new("count", the_work.len() as i64));
    s.end();
    health_check.update_db_access();
    if the_work.is_empty() {
        if let Some(dependence_chain_id) = &dependence_chain_id {
            info!(target: "tfhe_worker", dcid = %hex::encode(dependence_chain_id), locking = ?locking_reason, "No work items found to process");
        }
        health_check.update_activity();
        return Ok((vec![], vec![], false));
    }
    WORK_ITEMS_FOUND_COUNTER.inc_by(the_work.len() as u64);
    info!(target: "tfhe_worker", { count = the_work.len(), dcid = ?dependence_chain_id.as_ref().map(hex::encode),
         locking = ?locking_reason }, "Processing work items");
    // Make sure we process each tenant independently to avoid
    // setting different keys from different tenants in the worker
    // threads
    let mut s_prep: opentelemetry::global::BoxedSpan =
        tracer.start_with_context("prepare_dataflow_graphs", loop_ctx);
    s_prep.set_attribute(KeyValue::new("work_items", the_work.len() as i64));
    // Partition work by tenant
    let work_by_tenant = the_work.into_iter().into_group_map_by(|k| k.tenant_id);
    // Partition the work by transaction
    let mut work_by_tenant_by_transaction: HashMap<i32, HashMap<Handle, Vec<_>>> = HashMap::new();
    for (tenant_id, work) in work_by_tenant.into_iter() {
        work_by_tenant_by_transaction.insert(
            tenant_id,
            work.into_iter()
                .into_group_map_by(|k| k.transaction_id.clone()),
        );
    }
    // Traverse transactions and build transaction nodes
    let mut transactions: Vec<(i32, Vec<ComponentNode>)> = vec![];
    let mut unneeded_handles: Vec<(Handle, Handle)> = vec![];
    for (tenant_id, work_by_transaction) in work_by_tenant_by_transaction.iter() {
        let mut tenant_transactions: Vec<ComponentNode> = vec![];
        for (transaction_id, txwork) in work_by_transaction.iter() {
            let mut ops = vec![];
            for w in txwork {
                let fhe_op: SupportedFheOperations = match w.fhe_operation.try_into() {
                    Ok(op) => op,
                    Err(e) => {
                        error!(target: "tfhe_worker", { output_handle = ?w.output_handle, transaction_id = ?hex::encode(transaction_id), error = %e, }, "invalid FHE operation ");
                        set_computation_error(
                            &w.output_handle,
                            transaction_id,
                            tenant_id,
                            &e,
                            trx,
                            deps_chain_mngr,
                            tracer,
                            loop_ctx,
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
                        w.is_scalar && idx == 1 || fhe_op.does_have_more_than_one_scalar();
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
                check_fhe_operand_types(
                    w.fhe_operation.into(),
                    &this_comp_inputs,
                    &is_scalar_op_vec,
                )
                .map_err(CoprocessorError::FhevmError)?;
                ops.push(DFGOp {
                    output_handle: w.output_handle.clone(),
                    fhe_op,
                    inputs,
                    is_allowed: w.is_allowed,
                });
            }
            let (mut components, mut unneeded) = build_component_nodes(ops, transaction_id)?;
            tenant_transactions.append(&mut components);
            unneeded_handles.append(&mut unneeded);
        }
        transactions.push((*tenant_id, tenant_transactions));
    }
    s_prep.end();
    Ok((transactions, unneeded_handles, true))
}

async fn build_transaction_graph_and_execute<'a>(
    tenant_id: &i32,
    tenant_txs: &mut Vec<ComponentNode>,
    tenant_key_cache: &std::sync::Arc<tokio::sync::RwLock<lru::LruCache<i32, TfheTenantKeys>>>,
    health_check: &crate::health_check::HealthCheck,
    trx: &mut sqlx::Transaction<'a, Postgres>,
    tracer: &opentelemetry::global::BoxedTracer,
    loop_ctx: &opentelemetry::Context,
) -> Result<DFComponentGraph, Box<dyn std::error::Error + Send + Sync>> {
    let mut tx_graph = DFComponentGraph::default();
    if let Err(e) = tx_graph.build(tenant_txs) {
        // If we had an error while building the graph, we don't
        // execute anything and return to allow any set results
        // (essentially errors) to be set in DB.
        warn!(target: "tfhe_worker", { error = %e }, "error while building transaction graph");
        return Ok(tx_graph);
    }
    let cts_to_query = tx_graph.needed_map.keys().cloned().collect::<Vec<_>>();
    let ciphertext_map =
        query_ciphertexts(&cts_to_query, *tenant_id, trx, tracer, loop_ctx).await?;
    for (handle, (ct_type, mut ct)) in ciphertext_map.into_iter() {
        tx_graph.add_input(
            &handle,
            &DFGTxInput::Compressed(((ct_type, std::mem::take(&mut ct)), true)),
        )?;
    }
    // Execute the DFG with the current tenant's keys
    let mut s_compute = tracer.start_with_context("compute_fhe_ops", loop_ctx);
    {
        let mut rk = tenant_key_cache.write().await;
        let keys = match rk.get(tenant_id) {
            Some(k) => k,
            None => {
                // If the keys can't be located in the cache, skip
                // executing for this tenant, retry next iteration
                let err = CoprocessorError::MissingKeys {
                    tenant_id: *tenant_id,
                };
                error!(target: "tfhe_worker", { error = %err }, "no keys found for tenant");
                s_compute.set_status(opentelemetry::trace::Status::Error {
                    description: err.to_string().clone().into(),
                });
                WORKER_ERRORS_COUNTER.inc();
                return Err(err.into());
            }
        };

        // Schedule computations in parallel as dependences allow
        tfhe::set_server_key(keys.sks.clone());
        let mut sched = Scheduler::new(
            &mut tx_graph,
            keys.sks.clone(),
            keys.pks.clone(),
            #[cfg(feature = "gpu")]
            keys.gpu_sks.clone(),
            health_check.activity_heartbeat.clone(),
        );
        sched.schedule(loop_ctx).await?;
    }
    s_compute.end();
    Ok(tx_graph)
}

async fn upload_transaction_graph_results<'a>(
    tenant_id: &i32,
    tx_graph: &mut DFComponentGraph,
    unneeded_handles: &mut Vec<(Handle, Handle)>,
    trx: &mut sqlx::Transaction<'a, Postgres>,
    deps_mngr: &mut dependence_chain::LockMngr,
    tracer: &opentelemetry::global::BoxedTracer,
    loop_ctx: &opentelemetry::Context,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Get computation results
    let graph_results = tx_graph.get_results();
    let mut handles_to_update = tx_graph.get_intermediate_handles();
    handles_to_update.append(unneeded_handles);

    // Traverse computations that have been scheduled and
    // upload their results/errors.
    let mut cts_to_insert = vec![];
    let mut uncomputable = vec![];
    for result in graph_results.into_iter() {
        match result.compressed_ct {
            Ok((db_type, db_bytes)) => {
                cts_to_insert.push((
                    *tenant_id,
                    (
                        result.handle.clone(),
                        (db_bytes, (current_ciphertext_version(), db_type)),
                    ),
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
                            *err.downcast_ref::<SchedulerError>()
                                .unwrap_or(&SchedulerError::SchedulerError),
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
                                          { tenant_id = tenant_id, error = cerr,
                        output_handle = format!("0x{}", hex::encode(&result.handle)) },
                                        "scheduler encountered an error while processing work item"
                                    );
                        continue;
                    }
                    if matches!(
                        err,
                        CoprocessorError::SchedulerError(SchedulerError::MissingInputs)
                    ) {
                        uncomputable.push((result.handle.clone(), result.transaction_id.clone()));
                        // Make sure we don't mark this as an error since this simply means that the
                        // inputs weren't available when we tried scheduling these operations.
                        // Setting them as uncomputable will postpone them with an exponential backoff
                        // and they will be retried later.
                        continue;
                    }
                }
                set_computation_error(
                    &result.handle,
                    &result.transaction_id,
                    tenant_id,
                    &*cerr,
                    trx,
                    deps_mngr,
                    tracer,
                    loop_ctx,
                )
                .await?;
            }
        }
    }
    let mut s = tracer.start_with_context("insert_ct_into_db", loop_ctx);
    s.set_attribute(KeyValue::new("tenant_id", *tenant_id as i64));
    s.set_attributes(
        cts_to_insert
            .iter()
            .map(|(_, (h, (_, (_, _))))| KeyValue::new("handle", format!("0x{}", hex::encode(h)))),
    );
    s.set_attributes(
        cts_to_insert
            .iter()
            .map(|(_, (_, (_, (_, db_type))))| KeyValue::new("ciphertext_type", *db_type as i64)),
    );
    #[allow(clippy::type_complexity)]
    let (tenant_ids, (handles, (ciphertexts, (ciphertext_versions, ciphertext_types)))): (
        Vec<_>,
        (Vec<_>, (Vec<_>, (Vec<_>, Vec<_>))),
    ) = cts_to_insert.into_iter().unzip();
    let _ = query!(
			"
                    INSERT INTO ciphertexts(tenant_id, handle, ciphertext, ciphertext_version, ciphertext_type)
                    SELECT * FROM UNNEST($1::INTEGER[], $2::BYTEA[], $3::BYTEA[], $4::SMALLINT[], $5::SMALLINT[])
                    ON CONFLICT (tenant_id, handle, ciphertext_version) DO NOTHING
                    ",
		&tenant_ids, &handles, &ciphertexts, &ciphertext_versions, &ciphertext_types)
			.execute(trx.as_mut())
			.await.map_err(|err| {
                    error!(target: "tfhe_worker", { tenant_id = *tenant_id, error = %err }, "error while inserting new ciphertexts");
                    err
                })?;
    // Notify all workers that new ciphertext is inserted
    // For now, it's only the SnS workers that are listening for these events
    let _ = sqlx::query!("SELECT pg_notify($1, '')", EVENT_CIPHERTEXT_COMPUTED)
        .execute(trx.as_mut())
        .await?;
    s.end();

    let mut s = tracer.start_with_context("update_computation", loop_ctx);
    s.set_attribute(KeyValue::new("tenant_id", *tenant_id as i64));
    s.set_attributes(
        handles_to_update
            .iter()
            .map(|(h, _)| KeyValue::new("handle", format!("0x{}", hex::encode(h)))),
    );

    let (handles_vec, txn_ids_vec): (Vec<_>, Vec<_>) = handles_to_update.into_iter().unzip();

    let _ = query!(
                "
                UPDATE computations
                SET is_completed = true, completed_at = CURRENT_TIMESTAMP
                WHERE tenant_id = $1
                AND is_completed = false
                AND (output_handle, transaction_id) IN (
                    SELECT * FROM unnest($2::BYTEA[], $3::BYTEA[])
                )
                ",
                *tenant_id,
                &handles_vec,
                &txn_ids_vec
            )
            .execute(trx.as_mut())
            .await.map_err(|err| {
                    error!(target: "tfhe_worker", { tenant_id = *tenant_id, error = %err }, "error while updating computations as completed");
                    err
                })?;

    s.end();

    update_uncomputable_handles(uncomputable, *tenant_id, trx, tracer, loop_ctx).await?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn set_computation_error<'a>(
    output_handle: &[u8],
    transaction_id: &[u8],
    tenant_id: &i32,
    cerr: &(dyn std::error::Error + Send + Sync),
    trx: &mut sqlx::Transaction<'a, Postgres>,
    deps_mngr: &mut dependence_chain::LockMngr,
    tracer: &opentelemetry::global::BoxedTracer,
    loop_ctx: &opentelemetry::Context,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    WORKER_ERRORS_COUNTER.inc();
    error!(target: "tfhe_worker", { tenant_id = tenant_id, error = cerr.to_string(), output_handle = format!("0x{}", hex::encode(output_handle)) }, "error while processing work item");
    let mut s = tracer.start_with_context("set_computation_error_in_db", loop_ctx);
    s.set_attribute(KeyValue::new("tenant_id", *tenant_id as i64));
    s.set_attribute(KeyValue::new(
        "handle",
        format!("0x{}", hex::encode(output_handle)),
    ));
    let err_string = cerr.to_string();
    s.set_status(opentelemetry::trace::Status::Error {
        description: err_string.clone().into(),
    });

    let _ = query!(
        "
           UPDATE computations
           SET is_error = true, error_message = $1
           WHERE tenant_id = $2
           AND output_handle = $3
           AND transaction_id = $4
        ",
        err_string,
        *tenant_id,
        output_handle,
        transaction_id
    )
    .execute(trx.as_mut())
    .await?;

    deps_mngr.set_processing_error(Some(err_string)).await?;
    s.end();
    Ok(())
}
