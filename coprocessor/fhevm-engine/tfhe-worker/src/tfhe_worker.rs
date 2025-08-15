use crate::types::CoprocessorError;
use crate::{db_queries::populate_cache_with_tenant_keys, types::TfheTenantKeys};
use fhevm_engine_common::tfhe_ops::check_fhe_operand_types;
use fhevm_engine_common::types::{FhevmError, Handle, SupportedFheCiphertexts};
use fhevm_engine_common::{tfhe_ops::current_ciphertext_version, types::SupportedFheOperations};
use itertools::Itertools;
use lazy_static::lazy_static;
use opentelemetry::trace::{Span, TraceContextExt, Tracer};
use opentelemetry::KeyValue;
use prometheus::{register_int_counter, IntCounter};
use scheduler::dfg::types::SchedulerError;
use scheduler::dfg::{scheduler::Scheduler, types::DFGTaskInput, DFGraph};
use sqlx::{postgres::PgListener, query, Acquire};
use std::{
    collections::{BTreeSet, HashMap},
    num::NonZeroUsize,
};
use tracing::{debug, error, info};

const EVENT_CIPHERTEXT_COMPUTED: &str = "event_ciphertext_computed";

#[cfg(feature = "bench")]
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
}

pub async fn run_tfhe_worker(
    args: crate::daemon_cli::Args,
    health_check: crate::health_check::HealthCheck,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    loop {
        // here we log the errors and make sure we retry
        if let Err(cycle_error) = tfhe_worker_cycle(&args, health_check.clone()).await {
            WORKER_ERRORS_COUNTER.inc();
            error!(target: "tfhe_worker", { error = cycle_error }, "Error in background worker, retrying shortly");
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    }
}

async fn tfhe_worker_cycle(
    args: &crate::daemon_cli::Args,
    health_check: crate::health_check::HealthCheck,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let tracer = opentelemetry::global::tracer("tfhe_worker");

    let tenant_key_cache: std::sync::Arc<tokio::sync::RwLock<lru::LruCache<i32, TfheTenantKeys>>> =
        std::sync::Arc::new(tokio::sync::RwLock::new(lru::LruCache::new(
            NonZeroUsize::new(args.tenant_key_cache_size as usize).unwrap(),
        )));

    let db_url = crate::utils::db_url(args);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(args.pg_pool_max_connections)
        .connect(&db_url)
        .await?;

    #[cfg(feature = "bench")]
    populate_cache_with_tenant_keys(vec![1i32], &pool, &tenant_key_cache).await?;

    let mut listener = PgListener::connect_with(&pool).await.unwrap();
    listener.listen("event_allowed_handle").await?;

    let mut immedially_poll_more_work = false;
    loop {
        // only if previous iteration had no work done do the wait
        if !immedially_poll_more_work {
            tokio::select! {
                _ = listener.try_recv() => {
                    WORK_ITEMS_NOTIFICATIONS_COUNTER.inc();
                    info!(target: "tfhe_worker", "Received event_allowed_handle notification from postgres");
                },
                _ = tokio::time::sleep(tokio::time::Duration::from_millis(args.worker_polling_interval_ms)) => {
                    WORK_ITEMS_POLL_COUNTER.inc();
                    debug!(target: "tfhe_worker", "Polling the database for more work on timer");
                },
            };
        }
        immedially_poll_more_work = false;
        let loop_span = tracer.start("worker_iteration");
        let loop_ctx = opentelemetry::Context::current_with_span(loop_span);
        let mut s = tracer.start_with_context("acquire_connection", &loop_ctx);
        let mut conn = pool.acquire().await?;
        s.end();
        let mut s = tracer.start_with_context("begin_transaction", &loop_ctx);
        let mut trx = conn.begin().await?;
        s.end();
        // This query locks our work items so other worker doesn't select them.
        let mut s = tracer.start_with_context("query_work_items", &loop_ctx);
        #[cfg(feature = "bench")]
        let now = std::time::SystemTime::now();
        let the_work = query!(
            "
WITH selected_computations AS (
  -- Get all computations from such transactions
  (
    SELECT 
      c.tenant_id, 
      c.output_handle,
      c.transaction_id,
      ah.handle, 
      ah.is_computed
    FROM computations c
    LEFT JOIN allowed_handles ah
      ON c.output_handle = ah.handle
    WHERE c.transaction_id IN (
      -- Select transaction IDs with uncomputed handles
      -- out of the dependence buckets
      SELECT transaction_id 
      FROM computations
      WHERE is_completed = FALSE
        AND is_error = FALSE
        AND dependence_chain_id IN (
          WITH dep_chains AS (
            -- Find oldest uncomputed allowed handles and
            -- get their dependence buckets
            SELECT dependence_chain_id
            FROM computations
            WHERE (tenant_id, output_handle) IN (
              SELECT tenant_id, handle
              FROM allowed_handles
              WHERE is_computed = FALSE
              ORDER BY allowed_at
              LIMIT $2
            )
          )
          SELECT DISTINCT dependence_chain_id FROM dep_chains
        )
      -- In case an ACL event is missed and we get a late allow event
      UNION
      (
        SELECT transaction_id
        FROM computations
        WHERE (tenant_id, output_handle) IN (
          SELECT tenant_id, handle
          FROM allowed_handles
          WHERE is_computed = FALSE
          ORDER BY allowed_at
          LIMIT $2
        )
      )
      LIMIT $1
    )
  )
  -- For legacy reasons (or whenever no transaction ID is
  -- available) we need to also select unsorted
  -- computations
  UNION ALL
  (
    SELECT 
      tenant_id, 
      output_handle,
      transaction_id,
      NULL AS handle, 
      NULL AS is_computed
    FROM computations 
    WHERE is_completed = FALSE
      AND is_error = FALSE
      AND transaction_id IS NULL  
    ORDER BY schedule_order
    LIMIT $1
  )
)
-- Acquire all computations from this transaction set
SELECT 
  c.tenant_id, 
  c.output_handle, 
  c.dependencies, 
  c.fhe_operation, 
  c.is_scalar,
  sc.handle IS NOT NULL AS is_allowed, 
  c.dependence_chain_id,
  COALESCE(c.transaction_id) AS transaction_id, 
  c.is_completed, 
  sc.is_computed
FROM computations c
JOIN selected_computations sc
  ON c.tenant_id = sc.tenant_id
  AND c.output_handle = sc.output_handle
  AND (c.transaction_id = sc.transaction_id OR c.transaction_id IS NULL)
FOR UPDATE SKIP LOCKED            ",
            args.work_items_batch_size as i32,
            args.dependence_chains_per_batch as i32,
        )
        .fetch_all(trx.as_mut())
        .await?;
        s.set_attribute(KeyValue::new("count", the_work.len() as i64));
        s.end();
        health_check.update_db_access();
        if the_work.is_empty() {
            health_check.update_activity();
            continue;
        }
        WORK_ITEMS_FOUND_COUNTER.inc_by(the_work.len() as u64);
        info!(target: "tfhe_worker", { count = the_work.len() }, "Processing work items");
        // Make sure we process each tenant independently to avoid
        // setting different keys from different tenants in the worker
        // threads
        let mut work_by_tenant = the_work.into_iter().into_group_map_by(|k| k.tenant_id);

        let mut s = tracer.start_with_context("populate_key_cache", &loop_ctx);
        let mut cts_to_query: BTreeSet<&[u8]> = BTreeSet::new();
        let mut tenants_to_query: BTreeSet<i32> = BTreeSet::new();
        let mut keys_to_query: BTreeSet<i32> = BTreeSet::new();
        let key_cache = tenant_key_cache.read().await;
        // Clear unneeded work items
        for (_, work) in work_by_tenant.iter_mut() {
            let mut work_to_remove = vec![];
            for (idx, w) in work.iter_mut().enumerate() {
                // If this handle is already marked as computed in
                // allowed_handles, we don't need to re-compute it
                // (nor any other intermediate handles it depends on,
                // which will be marked as unneeded during compute
                // graph finalization).  Remove it from the work set.
                if w.is_computed.unwrap_or(false) {
                    work_to_remove.push(idx);
                }
            }
            for idx in work_to_remove.iter().rev() {
                work.remove(*idx);
            }
        }
        for (tenant_id, work) in work_by_tenant.iter_mut() {
            let _ = tenants_to_query.insert(*tenant_id);
            if !key_cache.contains(tenant_id) {
                let _ = keys_to_query.insert(*tenant_id);
            }
            for w in work.iter_mut() {
                for dh in &w.dependencies {
                    let _ = cts_to_query.insert(dh);
                }
                // If this operation is not part of a labelled
                // transaction, we need to treat it as if its output
                // is allowed (meaning that we will consider it needed
                // and will save its output in the DB) as otherwise we
                // cannot reasonably keep track of all needed
                // operations within the set of computations that are
                // not part of a transaction
                if w.transaction_id.is_none() {
                    w.is_allowed = Some(true);
                }
            }
        }
        drop(key_cache);
        let cts_to_query = cts_to_query
            .into_iter()
            .map(|i| i.to_vec())
            .collect::<Vec<_>>();
        let tenants_to_query = tenants_to_query.into_iter().collect::<Vec<_>>();
        let keys_to_query = keys_to_query.into_iter().collect::<Vec<_>>();
        s.set_attribute(KeyValue::new("keys_to_query", keys_to_query.len() as i64));
        s.set_attribute(KeyValue::new(
            "tenants_to_query",
            tenants_to_query.len() as i64,
        ));
        populate_cache_with_tenant_keys(keys_to_query, trx.as_mut(), &tenant_key_cache).await?;
        s.end();
        let mut s = tracer.start_with_context("query_ciphertext_batch", &loop_ctx);
        s.set_attribute(KeyValue::new("cts_to_query", cts_to_query.len() as i64));
        // TODO: select all the ciphertexts where they're contained in the tuples
        let ciphertexts_rows = query!(
            "
                SELECT tenant_id, handle, ciphertext, ciphertext_type
                FROM ciphertexts
                WHERE tenant_id = ANY($1::INT[])
                AND handle = ANY($2::BYTEA[])
            ",
            &tenants_to_query,
            &cts_to_query
        )
        .fetch_all(trx.as_mut())
        .await?;
        s.end();
        // index ciphertexts in hashmap
        let mut ciphertext_map: HashMap<(i32, &[u8]), _> =
            HashMap::with_capacity(ciphertexts_rows.len());
        for row in &ciphertexts_rows {
            let _ = ciphertext_map.insert((row.tenant_id, &row.handle), row);
        }

        // Process tenants in sequence to avoid switching keys during execution
        for (tenant_id, work) in work_by_tenant.iter() {
            let mut s_schedule = tracer.start_with_context("schedule_fhe_work", &loop_ctx);
            s_schedule.set_attribute(KeyValue::new("work_items", work.len() as i64));
            s_schedule.set_attribute(KeyValue::new("tenant_id", *tenant_id as i64));
            // We need to ensure that no handles are missing from
            // either DB inputs or values produced within this batch
            // before this batch is scheduled.
            let mut produced_handles: HashMap<&Handle, ()> = HashMap::new();
            for w in work.iter() {
                produced_handles.insert(&w.output_handle, ());
            }
            let mut produced_handles_count = produced_handles.len();
            loop {
                'work_items: for w in work.iter() {
                    let fhe_op: SupportedFheOperations = w
                        .fhe_operation
                        .try_into()
                        .expect("only valid fhe ops must have been put in db");
                    for (idx, dh) in w.dependencies.iter().enumerate() {
                        let is_operand_scalar =
                            w.is_scalar && idx == 1 || fhe_op.does_have_more_than_one_scalar();
                        if !is_operand_scalar
                            && !ciphertext_map.contains_key(&(w.tenant_id, dh))
                            && !produced_handles.contains_key(dh)
                        {
                            // As this operation is not computable, remove
                            // the output handle from those produced in
                            // this batch.
                            produced_handles.remove(&w.output_handle);
                            continue 'work_items;
                        }
                    }
                }
                // Test if we've reached the fixpoint
                if produced_handles_count == produced_handles.len() {
                    break;
                }
                produced_handles_count = produced_handles.len();
            }

            // Now build the DF graph for the computations that can
            // proceed and record those that can't
            let mut graph = DFGraph::default();
            let mut uncomputable: HashMap<usize, Handle> = HashMap::new();
            let mut producer_indexes: HashMap<&Handle, usize> = HashMap::new();
            let mut consumer_indexes: HashMap<usize, usize> = HashMap::new();
            'work_items: for (widx, w) in work.iter().enumerate() {
                let mut s = tracer.start_with_context("tfhe_computation", &loop_ctx);
                let fhe_op: SupportedFheOperations = w
                    .fhe_operation
                    .try_into()
                    .expect("only valid fhe ops must have been put in db");
                let mut input_ciphertexts: Vec<DFGTaskInput> =
                    Vec::with_capacity(w.dependencies.len());
                let mut this_comp_inputs: Vec<Vec<u8>> = Vec::with_capacity(w.dependencies.len());
                let mut is_scalar_op_vec: Vec<bool> = Vec::with_capacity(w.dependencies.len());
                for (idx, dh) in w.dependencies.iter().enumerate() {
                    let is_operand_scalar =
                        w.is_scalar && idx == 1 || fhe_op.does_have_more_than_one_scalar();
                    is_scalar_op_vec.push(is_operand_scalar);
                    this_comp_inputs.push(dh.clone());
                    if is_operand_scalar {
                        input_ciphertexts.push(DFGTaskInput::Value(
                            SupportedFheCiphertexts::Scalar(dh.clone()),
                        ));
                    } else if let Some(ct_map_val) = ciphertext_map.get(&(w.tenant_id, dh)) {
                        input_ciphertexts.push(DFGTaskInput::Compressed((
                            ct_map_val.ciphertext_type,
                            ct_map_val.ciphertext.clone().to_vec(),
                        )));
                    } else if produced_handles.contains_key(dh) {
                        input_ciphertexts.push(DFGTaskInput::Dependence(None));
                    } else {
                        // If this cannot be computed, we need to
                        // exclude it from the DF graph.
                        uncomputable.insert(widx, w.output_handle.clone());
                        continue 'work_items;
                    }
                }

                check_fhe_operand_types(
                    w.fhe_operation.into(),
                    &this_comp_inputs,
                    &is_scalar_op_vec,
                )
                .map_err(CoprocessorError::FhevmError)?;

                let n = graph.add_node(
                    w.output_handle.clone(),
                    w.fhe_operation.into(),
                    input_ciphertexts.clone(),
                    w.is_allowed.unwrap_or(true),
                    widx,
                )?;
                producer_indexes.insert(&w.output_handle, n.index());
                consumer_indexes.insert(widx, n.index());

                s.set_attribute(KeyValue::new("fhe_operation", w.fhe_operation as i64));
                s.set_attribute(KeyValue::new(
                    "handle",
                    format!("0x{}", hex::encode(&w.output_handle)),
                ));
                let input_types = input_ciphertexts
                    .iter()
                    .map(|i| match i {
                        DFGTaskInput::Value(i) => i.type_num().to_string(),
                        DFGTaskInput::Compressed(_) => "Compressed value".to_string(),
                        DFGTaskInput::Dependence(_) => "Temporary value".to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join(",");
                s.set_attribute(KeyValue::new("input_types", input_types));
                s.end();
            }
            s.end();
            // Traverse computations and add dependences/edges as required
            for (index, w) in work.iter().enumerate() {
                if uncomputable.contains_key(&index) {
                    continue;
                }
                for (input_idx, input) in w.dependencies.iter().enumerate() {
                    let fhe_op: SupportedFheOperations = w
                        .fhe_operation
                        .try_into()
                        .expect("only valid fhe ops must have been put in db");
                    let is_operand_scalar =
                        w.is_scalar && input_idx == 1 || fhe_op.does_have_more_than_one_scalar();
                    if !is_operand_scalar && !ciphertext_map.contains_key(&(w.tenant_id, input)) {
                        if let Some(producer_index) = producer_indexes.get(input) {
                            let consumer_index = consumer_indexes.get(&index).unwrap();
                            graph.add_dependence(*producer_index, *consumer_index, input_idx)?;
                        }
                    }
                }
            }
            graph.finalize();
            s_schedule.end();

            // Execute the DFG with the current tenant's keys
            let mut s_outer = tracer.start_with_context("wait_and_update_fhe_work", &loop_ctx);
            {
                let mut rk = tenant_key_cache.write().await;
                let keys = rk.get(tenant_id).expect("Can't get tenant key from cache");

                // Schedule computations in parallel as dependences allow
                tfhe::set_server_key(keys.sks.clone());
                let mut sched = Scheduler::new(
                    &mut graph.graph,
                    keys.sks.clone(),
                    #[cfg(feature = "gpu")]
                    keys.gpu_sks.clone(),
                    health_check.activity_heartbeat.clone(),
                );
                sched.schedule().await?;
            }
            // Extract the results from the graph
            let mut graph_results = graph.get_results();

            // Update uncomputable ops schedule orders
            {
                let mut s =
                    tracer.start_with_context("update_unschedulable_computations", &loop_ctx);
                s.set_attribute(KeyValue::new("tenant_id", *tenant_id as i64));
                s.set_attributes(
                    uncomputable
                        .values()
                        .map(|h| KeyValue::new("handle", format!("0x{}", hex::encode(h)))),
                );
                let _ = query!(
                    "
                            UPDATE computations
                            SET schedule_order = CURRENT_TIMESTAMP + INTERVAL '1 second' * uncomputable_counter,
                                uncomputable_counter = uncomputable_counter * 2 
                            WHERE tenant_id = $1
                            AND output_handle = ANY($2::BYTEA[])
                        ",
                    *tenant_id,
                    &uncomputable.into_values().collect::<Vec<_>>()
                )
                .execute(trx.as_mut())
                .await?;
                s.end();
            }
            // Traverse computations that have been scheduled and
            // upload their results/errors
            let mut handles_to_udate = vec![];
            let mut intermediate_handles_to_udate = vec![];
            let mut cts_to_insert = vec![];
            for result in graph_results.iter_mut() {
                let idx = result.work_index;
                let result = &mut result.result;

                let finished_work_unit: Result<
                    _,
                    (Box<dyn std::error::Error + Send + Sync>, i32, Vec<u8>),
                > = result
                    .as_mut()
                    .map(|rok| {
                        if let Some((ct_type, ref mut ct_bytes)) = rok {
                            (&work[idx], Some((ct_type, std::mem::take(ct_bytes))))
                        } else {
                            (&work[idx], None)
                        }
                    })
                    .map_err(|rerr| {
                        if rerr.downcast_ref::<FhevmError>().is_some() {
                            let mut swap_val = FhevmError::BadInputs;
                            std::mem::swap(
                                &mut *rerr.downcast_mut::<FhevmError>().unwrap(),
                                &mut swap_val,
                            );
                            (
                                CoprocessorError::FhevmError(swap_val).into(),
                                work[idx].tenant_id,
                                work[idx].output_handle.clone(),
                            )
                        } else {
                            (
                                CoprocessorError::SchedulerError(
                                    *rerr
                                        .downcast_ref::<SchedulerError>()
                                        .unwrap_or(&SchedulerError::SchedulerError),
                                )
                                .into(),
                                work[idx].tenant_id,
                                work[idx].output_handle.clone(),
                            )
                        }
                    });
                match finished_work_unit {
                    Ok((w, Some((db_type, db_bytes)))) => {
                        cts_to_insert.push((
                            w.tenant_id,
                            (
                                w.output_handle.clone(),
                                (db_bytes, (current_ciphertext_version(), *db_type)),
                            ),
                        ));
                        handles_to_udate.push(w.output_handle.clone());
                        // As we've completed useful computation on an
                        // allowed handle, we poll for work again
                        // without waiting for a notification.
                        immedially_poll_more_work = true;
                        WORK_ITEMS_PROCESSED_COUNTER.inc();
                    }
                    Ok((w, None)) => {
                        // Non allowed handles are still marked as
                        // complete but we don't upload the CT
                        intermediate_handles_to_udate.push(w.output_handle.clone());
                        WORK_ITEMS_PROCESSED_COUNTER.inc();
                    }
                    Err((err, tenant_id, output_handle)) => {
                        WORKER_ERRORS_COUNTER.inc();
                        error!(target: "tfhe_worker",
                            { tenant_id = tenant_id, error = err, output_handle = format!("0x{}", hex::encode(&output_handle)) },
                            "error while processing work item"
                        );
                        let mut s =
                            tracer.start_with_context("set_computation_error_in_db", &loop_ctx);
                        s.set_attribute(KeyValue::new("tenant_id", tenant_id as i64));
                        s.set_attribute(KeyValue::new(
                            "handle",
                            format!("0x{}", hex::encode(&output_handle)),
                        ));
                        let err_string = err.to_string();
                        s.set_status(opentelemetry::trace::Status::Error {
                            description: err_string.clone().into(),
                        });
                        let _ = query!(
                            "
                            UPDATE computations
                            SET is_error = true, error_message = $1
                            WHERE tenant_id = $2
                            AND output_handle = $3
                        ",
                            err_string,
                            tenant_id,
                            output_handle
                        )
                        .execute(trx.as_mut())
                        .await?;
                        s.end();
                    }
                }
            }
            let mut s = tracer.start_with_context("insert_ct_into_db", &loop_ctx);
            s.set_attribute(KeyValue::new("tenant_id", *tenant_id as i64));
            s.set_attributes(cts_to_insert.iter().map(|(_, (h, (_, (_, _))))| {
                KeyValue::new("handle", format!("0x{}", hex::encode(h)))
            }));
            s.set_attributes(cts_to_insert.iter().map(|(_, (_, (_, (_, db_type))))| {
                KeyValue::new("ciphertext_type", *db_type as i64)
            }));
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
			.await?;
            // Notify all workers that new ciphertext is inserted
            // For now, it's only the SnS workers that are listening for these events
            let _ = sqlx::query!("SELECT pg_notify($1, '')", EVENT_CIPHERTEXT_COMPUTED)
                .execute(trx.as_mut())
                .await?;
            s.end();

            let mut s = tracer.start_with_context("update_computation", &loop_ctx);
            s.set_attribute(KeyValue::new("tenant_id", *tenant_id as i64));
            s.set_attributes(
                handles_to_udate
                    .iter()
                    .map(|h| KeyValue::new("handle", format!("0x{}", hex::encode(h)))),
            );
            let _ = query!(
                "
                UPDATE computations
                SET is_completed = true, completed_at = CURRENT_TIMESTAMP
                WHERE tenant_id = $1
                AND output_handle = ANY($2::BYTEA[])
            ",
                *tenant_id,
                &handles_to_udate
            )
            .execute(trx.as_mut())
            .await?;
            s.end();
            let mut s = tracer.start_with_context("update_allowed_handles_is_computed", &loop_ctx);
            s.set_attribute(KeyValue::new("tenant_id", *tenant_id as i64));
            s.set_attributes(
                handles_to_udate
                    .iter()
                    .map(|h| KeyValue::new("handle", format!("0x{}", hex::encode(h)))),
            );
            let _ = query!(
                "
                UPDATE allowed_handles
                SET is_computed = TRUE
                WHERE tenant_id = $1
                AND handle = ANY($2::BYTEA[])
            ",
                *tenant_id,
                &handles_to_udate
            )
            .execute(trx.as_mut())
            .await?;
            s.end();
            let mut s = tracer.start_with_context("update_intermediate_computation", &loop_ctx);
            s.set_attribute(KeyValue::new("tenant_id", *tenant_id as i64));
            s.set_attributes(
                intermediate_handles_to_udate
                    .iter()
                    .map(|h| KeyValue::new("handle", format!("0x{}", hex::encode(h)))),
            );
            let _ = query!(
                "
                UPDATE computations
                SET is_completed = true, completed_at = CURRENT_TIMESTAMP
                WHERE tenant_id = $1
                AND output_handle = ANY($2::BYTEA[])
            ",
                *tenant_id,
                &intermediate_handles_to_udate
            )
            .execute(trx.as_mut())
            .await?;
            s.end();

            s_outer.end();
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
