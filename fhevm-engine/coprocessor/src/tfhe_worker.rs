use crate::{db_queries::populate_cache_with_tenant_keys, types::TfheTenantKeys};
use fhevm_engine_common::types::{Handle, SupportedFheCiphertexts};
use fhevm_engine_common::{tfhe_ops::current_ciphertext_version, types::SupportedFheOperations};
use itertools::Itertools;
use lazy_static::lazy_static;
use opentelemetry::trace::{Span, TraceContextExt, Tracer};
use opentelemetry::KeyValue;
use prometheus::{register_int_counter, IntCounter};
use scheduler::dfg::{scheduler::Scheduler, types::DFGTaskInput, DFGraph};
use sqlx::{postgres::PgListener, query, Acquire};
use std::{
    collections::{BTreeSet, HashMap},
    num::NonZeroUsize,
};
use tracing::{debug, error, info};

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
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    loop {
        // here we log the errors and make sure we retry
        if let Err(cycle_error) = tfhe_worker_cycle(&args).await {
            WORKER_ERRORS_COUNTER.inc();
            error!(target: "tfhe_worker", { error = cycle_error }, "Error in background worker, retrying shortly");
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    }
}

async fn tfhe_worker_cycle(
    args: &crate::daemon_cli::Args,
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

    let mut listener = PgListener::connect_with(&pool).await.unwrap();
    listener.listen("work_available").await?;

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
        let the_work = query!(
            "
            WITH RECURSIVE dependent_computations(tenant_id, output_handle, dependencies, fhe_operation, is_scalar, produced_handles) AS (
                SELECT c.tenant_id, c.output_handle, c.dependencies, c.fhe_operation, c.is_scalar, ARRAY[ROW(c.tenant_id, c.output_handle)]
                FROM computations c
                WHERE is_completed = false
                AND is_error = false
                AND NOT EXISTS (
                    SELECT 1
                    FROM unnest(c.dependencies) WITH ORDINALITY AS elems(v, dep_index)
                    WHERE (c.tenant_id, elems.v) NOT IN ( SELECT tenant_id, handle FROM ciphertexts )
                    -- don't select scalar operands
                    AND (
                        NOT c.is_scalar
                        OR c.is_scalar AND NOT elems.dep_index = 2
                    )
                    -- ignore fhe random, trivial encrypt operations, all inputs are scalars
                    AND NOT c.fhe_operation = ANY(ARRAY[24, 26, 27])
                )
              UNION ALL
                SELECT c.tenant_id, c.output_handle, c.dependencies, c.fhe_operation, c.is_scalar, dc.produced_handles || ROW(c.tenant_id, c.output_handle)
                FROM dependent_computations dc, computations c
                WHERE is_completed = false
                AND is_error = false
                AND NOT EXISTS (
                    SELECT 1
                    FROM unnest(c.dependencies) WITH ORDINALITY AS elems(v, dep_index)
                    WHERE (c.tenant_id, elems.v) NOT IN ( SELECT tenant_id, handle FROM ciphertexts )
                    AND NOT ROW(c.tenant_id, elems.v) = ANY(dc.produced_handles)
                    -- don't select scalar operands
                    AND (
                        NOT c.is_scalar
                        OR c.is_scalar AND NOT elems.dep_index = 2
                    )
                    -- ignore fhe random, trivial encrypt operations, all inputs are scalars
                    AND NOT c.fhe_operation = ANY(ARRAY[24, 26, 27])
                )
                AND dc.output_handle = ANY(c.dependencies)
                AND dc.tenant_id = c.tenant_id
            ) SEARCH DEPTH FIRST BY output_handle SET computation_order,
           limited_computations AS (
              SELECT tenant_id, output_handle
              FROM dependent_computations
              GROUP BY tenant_id, output_handle
              ORDER BY min(computation_order)
              LIMIT $1
            )
            SELECT tenant_id, output_handle, dependencies, fhe_operation, is_scalar
            FROM computations
            WHERE (tenant_id, output_handle) IN (
              SELECT tenant_id, output_handle FROM limited_computations
            )
            FOR UPDATE SKIP LOCKED
        ",
            args.work_items_batch_size as i32
        )
        .fetch_all(trx.as_mut())
        .await?;
        s.set_attribute(KeyValue::new("count", the_work.len() as i64));
        s.end();
        immedially_poll_more_work = !the_work.is_empty();
        if the_work.is_empty() {
            continue;
        }
        WORK_ITEMS_FOUND_COUNTER.inc_by(the_work.len() as u64);
        info!(target: "tfhe_worker", { count = the_work.len() }, "Processing work items");
        // Make sure we process each tenant independently to avoid
        // setting different keys from different tenants in the worker
        // threads
        let work_by_tenant = the_work.into_iter().into_group_map_by(|k| k.tenant_id);

        let mut s = tracer.start_with_context("populate_key_cache", &loop_ctx);
        let mut cts_to_query: BTreeSet<&[u8]> = BTreeSet::new();
        let mut tenants_to_query: BTreeSet<i32> = BTreeSet::new();
        let mut keys_to_query: BTreeSet<i32> = BTreeSet::new();
        let key_cache = tenant_key_cache.read().await;
        for (tenant_id, work) in work_by_tenant.iter() {
            let _ = tenants_to_query.insert(*tenant_id);
            if !key_cache.contains(tenant_id) {
                let _ = keys_to_query.insert(*tenant_id);
            }
            for w in work.iter() {
                for dh in &w.dependencies {
                    let _ = cts_to_query.insert(dh);
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
            let mut uncomputable: HashMap<usize, ()> = HashMap::new();
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
                for (idx, dh) in w.dependencies.iter().enumerate() {
                    let is_operand_scalar =
                        w.is_scalar && idx == 1 || fhe_op.does_have_more_than_one_scalar();
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
                        uncomputable.insert(widx, ());
                        continue 'work_items;
                    }
                }

                let n = graph.add_node(
                    w.output_handle.clone(),
                    w.fhe_operation.into(),
                    input_ciphertexts.clone(),
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
            s_schedule.end();

            // Execute the DFG with the current tenant's keys
            let mut s_outer = tracer.start_with_context("wait_and_update_fhe_work", &loop_ctx);
            {
                let mut rk = tenant_key_cache.write().await;
                let keys = rk.get(tenant_id).expect("Can't get tenant key from cache");

                // Schedule computations in parallel as dependences allow
                let mut sched = Scheduler::new(&mut graph.graph, args.coprocessor_fhe_threads);
                sched.schedule(keys.sks.clone()).await?;
            }
            // Extract the results from the graph
            let res = graph.get_results().unwrap();

            for (idx, w) in work.iter().enumerate() {
                // Filter out computations that could not complete
                if uncomputable.contains_key(&idx) {
                    continue;
                }
                let r = res.iter().find(|(h, _)| *h == w.output_handle).unwrap();
                {
                    let mut rk = tenant_key_cache.write().await;
                    let keys = rk
                        .get(&w.tenant_id)
                        .expect("Can't get tenant key from cache");
                    tfhe::set_server_key(keys.sks.clone());
                }
                let finished_work_unit: Result<
                    _,
                    (Box<(dyn std::error::Error + Send + Sync)>, i32, Vec<u8>),
                > = Ok((w, r.1 .1, &r.1 .2));
                match finished_work_unit {
                    Ok((w, db_type, db_bytes)) => {
                        let mut s = tracer.start_with_context("insert_ct_into_db", &loop_ctx);
                        s.set_attribute(KeyValue::new("tenant_id", w.tenant_id as i64));
                        s.set_attribute(KeyValue::new(
                            "handle",
                            format!("0x{}", hex::encode(&w.output_handle)),
                        ));
                        s.set_attribute(KeyValue::new("ciphertext_type", db_type as i64));
                        let _ = query!("
                        INSERT INTO ciphertexts(tenant_id, handle, ciphertext, ciphertext_version, ciphertext_type)
                        VALUES($1, $2, $3, $4, $5)
                        ON CONFLICT (tenant_id, handle, ciphertext_version) DO NOTHING
                    ", w.tenant_id, w.output_handle, &db_bytes, current_ciphertext_version(), db_type)
                    .execute(trx.as_mut())
                    .await?;
                        s.end();
                        let mut s = tracer.start_with_context("update_computation", &loop_ctx);
                        s.set_attribute(KeyValue::new("tenant_id", w.tenant_id as i64));
                        s.set_attribute(KeyValue::new(
                            "handle",
                            format!("0x{}", hex::encode(&w.output_handle)),
                        ));
                        s.set_attribute(KeyValue::new("ciphertext_type", db_type as i64));
                        let _ = query!(
                            "
                            UPDATE computations
                            SET is_completed = true
                            WHERE tenant_id = $1
                            AND output_handle = $2
                        ",
                            w.tenant_id,
                            w.output_handle
                        )
                        .execute(trx.as_mut())
                        .await?;
                        s.end();
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
            s_outer.end();
        }
        s.end();

        trx.commit().await?;

        let _guard = loop_ctx.attach();
    }
}
