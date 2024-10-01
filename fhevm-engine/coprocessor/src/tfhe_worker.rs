use crate::{db_queries::populate_cache_with_tenant_keys, types::TfheTenantKeys};
use fhevm_engine_common::types::SupportedFheCiphertexts;
use fhevm_engine_common::{
    tfhe_ops::{current_ciphertext_version, perform_fhe_operation},
    types::SupportedFheOperations,
};
use lazy_static::lazy_static;
use opentelemetry::trace::{Span, TraceContextExt, Tracer};
use opentelemetry::KeyValue;
use prometheus::{register_int_counter, IntCounter};
use sqlx::{postgres::PgListener, query, Acquire};
use std::{
    collections::{BTreeSet, HashMap},
    num::NonZeroUsize,
};
use tracing::{error, info};

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
    args: crate::cli::Args,
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
    args: &crate::cli::Args,
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
                _ = tokio::time::sleep(tokio::time::Duration::from_millis(5000)) => {
                    WORK_ITEMS_POLL_COUNTER.inc();
                    info!(target: "tfhe_worker", "Polling the database for more work on timer");
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

        // this query locks our work items so other worker doesn't select them
        let mut s = tracer.start_with_context("query_work_items", &loop_ctx);
        let mut the_work = query!(
            "
            SELECT tenant_id, output_handle, dependencies, fhe_operation, is_scalar
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
            LIMIT $1
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

        // make sure we process each tenant sequentially not to
        // load different keys in cache by different tenants
        the_work.sort_by_key(|k| k.tenant_id);

        let mut s = tracer.start_with_context("populate_key_cache", &loop_ctx);

        let mut cts_to_query: BTreeSet<&[u8]> = BTreeSet::new();
        let mut tenants_to_query: BTreeSet<i32> = BTreeSet::new();
        let mut keys_to_query: BTreeSet<i32> = BTreeSet::new();
        let key_cache = tenant_key_cache.read().await;
        for w in &the_work {
            let _ = tenants_to_query.insert(w.tenant_id);
            if !key_cache.contains(&w.tenant_id) {
                let _ = keys_to_query.insert(w.tenant_id);
            }
            for dh in &w.dependencies {
                let _ = cts_to_query.insert(&dh);
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

        let mut s = tracer.start_with_context("schedule_fhe_work", &loop_ctx);
        s.set_attribute(KeyValue::new("work_items", the_work.len() as i64));

        let mut tfhe_work_set = tokio::task::JoinSet::new();
        // process every tenant by tenant id because we must switch keys for each tenant
        for w in the_work {
            let tenant_key_cache = tenant_key_cache.clone();
            let fhe_op: SupportedFheOperations = w
                .fhe_operation
                .try_into()
                .expect("only valid fhe ops must have been put in db");

            let mut work_ciphertexts: Vec<(i16, Vec<u8>)> =
                Vec::with_capacity(w.dependencies.len());
            for (idx, dh) in w.dependencies.iter().enumerate() {
                let is_operand_scalar =
                    w.is_scalar && idx == 1 || fhe_op.does_have_more_than_one_scalar();
                if is_operand_scalar {
                    work_ciphertexts.push((-1, dh.clone()));
                } else {
                    let ct_map_val = ciphertext_map.get(&(w.tenant_id, &dh)).expect(
                        "must get ciphertext here, we should have selected all dependencies",
                    );
                    work_ciphertexts
                        .push((ct_map_val.ciphertext_type, ct_map_val.ciphertext.clone()));
                }
            }

            // copy for setting error in database
            let mut s = tracer.start_with_context("tfhe_computation", &loop_ctx);
            tfhe_work_set.spawn_blocking(
                move || -> Result<_, (Box<(dyn std::error::Error + Send + Sync)>, i32, Vec<u8>)> {
                    // set the server key if not set
                    {
                        let mut rk = tenant_key_cache.blocking_write();
                        let keys = rk
                            .get(&w.tenant_id)
                            .expect("Can't get tenant key from cache");
                        tfhe::set_server_key(keys.sks.clone());
                    }

                    let mut deserialized_cts: Vec<SupportedFheCiphertexts> =
                        Vec::with_capacity(work_ciphertexts.len());
                    for (idx, (ct_type, ct_bytes)) in work_ciphertexts.iter().enumerate() {
                        let is_operand_scalar =
                            w.is_scalar && idx == 1 || fhe_op.does_have_more_than_one_scalar();
                        if is_operand_scalar {
                            let mut the_int = tfhe::integer::U256::default();
                            assert!(
                                ct_bytes.len() <= 32,
                                "we don't support larger numbers than 32 bytes"
                            );
                            let mut padded: Vec<u8> = Vec::with_capacity(32);
                            for byte in ct_bytes.iter().rev() {
                                padded.push(*byte);
                            }
                            while padded.len() < 32 {
                                padded.push(0x00);
                            }
                            the_int.copy_from_le_byte_slice(&padded);
                            deserialized_cts.push(SupportedFheCiphertexts::Scalar(the_int));
                        } else {
                            deserialized_cts.push(
                                SupportedFheCiphertexts::decompress(*ct_type, ct_bytes.as_slice())
                                    .map_err(|e| {
                                        let err: Box<dyn std::error::Error + Send + Sync> =
                                            e.into();
                                        (err, w.tenant_id, w.output_handle.clone())
                                    })?,
                            );
                        }
                    }

                    let res =
                        perform_fhe_operation(w.fhe_operation, &deserialized_cts).map_err(|e| {
                            let err: Box<dyn std::error::Error + Send + Sync> = Box::new(e);
                            (err, w.tenant_id, w.output_handle.clone())
                        })?;
                    let (db_type, db_bytes) = res.compress();

                    s.set_attribute(KeyValue::new("fhe_operation", w.fhe_operation as i64));
                    s.set_attribute(KeyValue::new(
                        "handle",
                        format!("0x{}", hex::encode(&w.output_handle)),
                    ));
                    s.set_attribute(KeyValue::new("output_type", db_type as i64));
                    let input_types = deserialized_cts
                        .iter()
                        .map(|i| i.type_num().to_string())
                        .collect::<Vec<_>>()
                        .join(",");
                    s.set_attribute(KeyValue::new("input_types", input_types));
                    s.end();
                    Ok((w, db_type, db_bytes))
                },
            );
        }
        s.end();

        let mut s_outer = tracer.start_with_context("wait_and_update_fhe_work", &loop_ctx);
        while let Some(output) = tfhe_work_set.join_next().await {
            let finished_work_unit = output?;
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
                    let mut s = tracer.start_with_context("set_computation_error_in_db", &loop_ctx);
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

        trx.commit().await?;

        let _guard = loop_ctx.attach();
    }
}
