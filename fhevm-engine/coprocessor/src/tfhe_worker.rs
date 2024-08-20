use crate::tfhe_ops::{
    current_ciphertext_version, deserialize_fhe_ciphertext, perform_fhe_operation,
};
use crate::types::{SupportedFheCiphertexts, TfheTenantKeys};
use sqlx::{postgres::PgListener, query, Acquire};
use std::{
    cell::Cell,
    collections::{BTreeSet, HashMap},
    num::NonZeroUsize,
};

pub async fn run_tfhe_worker(
    args: crate::cli::Args,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    loop {
        // here we log the errors and make sure we retry
        if let Err(cycle_error) = tfhe_worker_cycle(&args).await {
            eprintln!(
                "Error in background worker, retrying shortly: {:?}",
                cycle_error
            );
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    }
}

async fn tfhe_worker_cycle(
    args: &crate::cli::Args,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let key_cache_size = 32;
    let tenant_key_cache: std::sync::Arc<tokio::sync::RwLock<lru::LruCache<i32, TfheTenantKeys>>> =
        std::sync::Arc::new(tokio::sync::RwLock::new(lru::LruCache::new(
            NonZeroUsize::new(key_cache_size).unwrap(),
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
                    println!("Received work_available notification from postgres");
                },
                _ = tokio::time::sleep(tokio::time::Duration::from_millis(5000)) => {
                    println!("Polling the database for more work on timer");
                },
            };
        }

        let mut conn = pool.acquire().await?;
        let mut trx = conn.begin().await?;

        // this query locks our work items so other worker doesn't select them
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
                AND ( NOT c.is_scalar OR c.is_scalar AND NOT elems.dep_index = 2 )
            )
            LIMIT $1
            FOR UPDATE SKIP LOCKED
        ",
            args.work_items_batch_size as i32
        )
        .fetch_all(trx.as_mut())
        .await?;

        immedially_poll_more_work = !the_work.is_empty();

        if the_work.is_empty() {
            continue;
        }

        println!("Processing {} work items", the_work.len());

        // make sure we process each tenant sequentially not to
        // load different keys in cache by different tenants
        the_work.sort_by_key(|k| k.tenant_id);

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

        if !keys_to_query.is_empty() {
            let keys = query!(
                "
                SELECT tenant_id, pks_key, sks_key
                FROM tenants
                WHERE tenant_id = ANY($1::INT[])
            ",
                &keys_to_query
            )
            .fetch_all(trx.as_mut())
            .await?;

            assert!(
                keys.len() > 0,
                "We should have keys here, otherwise our database is corrupt"
            );

            let mut key_cache = tenant_key_cache.write().await;

            for key in keys {
                let sks: tfhe::ServerKey = bincode::deserialize(&key.sks_key)
                    .expect("We can't deserialize our own validated sks key");
                let pks: tfhe::CompactPublicKey = bincode::deserialize(&key.pks_key)
                    .expect("We can't deserialize our own validated pks key");
                key_cache.put(key.tenant_id, TfheTenantKeys { sks, pks });
            }
        }

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

        // index ciphertexts in hashmap
        let mut ciphertext_map: HashMap<(i32, &[u8]), _> =
            HashMap::with_capacity(ciphertexts_rows.len());
        for row in &ciphertexts_rows {
            let _ = ciphertext_map.insert((row.tenant_id, &row.handle), row);
        }

        let mut tfhe_work_set = tokio::task::JoinSet::new();
        // process every tenant by tenant id because we must switch keys for each tenant
        for w in the_work {
            let tenant_key_cache = tenant_key_cache.clone();

            let mut work_ciphertexts: Vec<(i16, Vec<u8>)> =
                Vec::with_capacity(w.dependencies.len());
            for (idx, dh) in w.dependencies.iter().enumerate() {
                let is_operand_scalar = w.is_scalar && idx == 1;
                if is_operand_scalar {
                    work_ciphertexts.push((-1, dh.clone()));
                } else {
                    let ct_map_val = ciphertext_map
                        .get(&(w.tenant_id, &dh))
                        .expect("Me must get ciphertext here");
                    work_ciphertexts
                        .push((ct_map_val.ciphertext_type, ct_map_val.ciphertext.clone()));
                }
            }

            // copy for setting error in database
            tfhe_work_set.spawn_blocking(
                move || -> Result<_, (Box<(dyn std::error::Error + Send + Sync)>, i32, Vec<u8>)> {
                    thread_local! {
                        static TFHE_TENANT_ID: Cell<i32> = Cell::new(-1);
                    }

                    // set thread tenant key
                    if w.tenant_id != TFHE_TENANT_ID.get() {
                        let mut rk = tenant_key_cache.blocking_write();
                        let keys = rk
                            .get(&w.tenant_id)
                            .expect("Can't get tenant key from cache");
                        tfhe::set_server_key(keys.sks.clone());
                        TFHE_TENANT_ID.set(w.tenant_id);
                    }

                    let mut deserialized_cts: Vec<SupportedFheCiphertexts> =
                        Vec::with_capacity(work_ciphertexts.len());
                    for (idx, (ct_type, ct_bytes)) in work_ciphertexts.iter().enumerate() {
                        let is_operand_scalar = w.is_scalar && idx == 1;
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
                                deserialize_fhe_ciphertext(*ct_type, ct_bytes.as_slice())
                                    .map_err(|e| (e, w.tenant_id, w.output_handle.clone()))?,
                            );
                        }
                    }

                    let res = perform_fhe_operation(w.fhe_operation, &deserialized_cts)
                        .map_err(|e| (e, w.tenant_id, w.output_handle.clone()))?;
                    let (db_type, db_bytes) = res.serialize();

                    Ok((w, db_type, db_bytes))
                },
            );
        }

        while let Some(output) = tfhe_work_set.join_next().await {
            let finished_work_unit = output?;
            match finished_work_unit {
                Ok((w, db_type, db_bytes)) => {
                    let _ = query!("
                        INSERT INTO ciphertexts(tenant_id, handle, ciphertext, ciphertext_version, ciphertext_type)
                        VALUES($1, $2, $3, $4, $5)
                        ON CONFLICT (tenant_id, handle, ciphertext_version) DO NOTHING
                    ", w.tenant_id, w.output_handle, &db_bytes, current_ciphertext_version(), db_type)
                    .execute(trx.as_mut())
                    .await?;
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
                }
                Err((err, tenant_id, output_handle)) => {
                    let _ = query!(
                        "
                        UPDATE computations
                        SET is_error = true, error_message = $1
                        WHERE tenant_id = $2
                        AND output_handle = $3
                    ",
                        err.to_string(),
                        tenant_id,
                        output_handle
                    )
                    .execute(trx.as_mut())
                    .await?;
                }
            }
        }

        trx.commit().await?;
    }
}
