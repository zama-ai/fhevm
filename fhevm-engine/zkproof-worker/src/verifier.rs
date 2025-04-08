use alloy_primitives::Address;
use fhevm_engine_common::tenant_keys::TfheTenantKeys;
use fhevm_engine_common::tenant_keys::{self, FetchTenantKeyResult};
use fhevm_engine_common::tfhe_ops::{current_ciphertext_version, extract_ct_list};
use fhevm_engine_common::types::SupportedFheCiphertexts;

use fhevm_engine_common::utils::safe_deserialize;
use lru::LruCache;
use sha3::Digest;
use sha3::Keccak256;
use sqlx::postgres::PgPoolOptions;
use sqlx::{postgres::PgListener, PgPool, Row};
use std::num::NonZero;
use std::str::FromStr;
use tokio::sync::RwLock;
use tokio::task::JoinSet;

use crate::{auxiliary, Config, ExecutionError};
use anyhow::Result;

use std::sync::Arc;
use tfhe::set_server_key;

use tokio::{select, time::Duration};
use tracing::{debug, error, info};

const MAX_CACHED_TENANT_KEYS: usize = 100;

pub(crate) struct Ciphertext {
    handle: Vec<u8>,
    compressed: Vec<u8>,
    ct_type: i16,
    ct_version: i16,
}

/// Executes the main loop for handling verify_proofs requests inserted in the database
pub async fn execute_verify_proofs_loop(conf: &Config) -> Result<(), ExecutionError> {
    info!("Starting with config {:?}", conf);

    // Tenants key cache is shared amongst all workers
    let tenant_key_cache = Arc::new(RwLock::new(LruCache::new(
        NonZero::new(MAX_CACHED_TENANT_KEYS).unwrap(),
    )));

    // Each worker needs at least 3 pg connections
    let pool_connections = std::cmp::max(conf.pg_pool_connections, 3 * conf.worker_thread_count);

    // DB Connection pool is shared amongst all workers
    let pool = PgPoolOptions::new()
        .max_connections(pool_connections)
        .connect(&conf.database_url)
        .await
        .expect("valid db pool");

    let mut task_set = JoinSet::new();

    for _ in 0..conf.worker_thread_count {
        let conf = conf.clone();
        let tenant_key_cache = tenant_key_cache.clone();
        let pool = pool.clone();

        // Spawn a ZK-proof worker
        // All workers compete for zk-proof tasks queued in the 'verify_proof' table.
        task_set.spawn(async move {
            if let Err(err) = execute_worker(&conf, &pool, &tenant_key_cache).await {
                error!("executor failed with {}", err);
            }
        });
    }

    // Wait for all tasks to complete
    while let Some(result) = task_set.join_next().await {
        if let Err(err) = result {
            eprintln!("A worker failed: {:?}", err);
        }
    }

    Ok(())
}

async fn execute_worker(
    conf: &Config,
    pool: &sqlx::Pool<sqlx::Postgres>,
    tenant_key_cache: &Arc<RwLock<LruCache<i32, TfheTenantKeys>>>,
) -> Result<(), ExecutionError> {
    let mut listener = PgListener::connect_with(pool).await?;
    listener.listen(&conf.listen_database_channel).await?;
    let idle_poll_interval = Duration::from_secs(conf.pg_polling_interval as u64);

    loop {
        if let Err(e) = execute_verify_proof_routine(pool, tenant_key_cache, conf).await {
            error!(target: "zkpok", "Execution err: {}", e);
        } else {
            let count = get_remaining_tasks(pool).await?;
            if count > 0 {
                info!(target: "zkpok", {count}, "ZkPok tasks available");
                continue;
            }
        }

        select! {
            res = listener.try_recv() => {
                match res {
                    Ok(None) => {
                        error!(target: "zkpok", "DB connection err");
                        return Err(ExecutionError::LostDbConnection)
                    },
                    Ok(_) => info!(target: "zkpok", "Received notification"),
                    Err(err) => {
                        error!(target: "zkpok", "DB connection err {}", err);
                        return Err(ExecutionError::LostDbConnection)
                    },
                };
            },
            _ = tokio::time::sleep(idle_poll_interval) => {
                debug!(target: "zkpok", "Polling timeout, rechecking for tasks");
            }
        }
    }
}

/// Fetch, verify a single proof and then compute signature
async fn execute_verify_proof_routine(
    pool: &PgPool,
    tenant_key_cache: &Arc<RwLock<LruCache<i32, TfheTenantKeys>>>,
    conf: &Config,
) -> Result<(), ExecutionError> {
    let mut txn: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await?;
    if let Ok(row) = sqlx::query(
        "SELECT zk_proof_id, input, chain_id, contract_address, user_address
            FROM verify_proofs
            WHERE verified IS NULL
            ORDER BY zk_proof_id ASC
            LIMIT 1 FOR UPDATE SKIP LOCKED",
    )
    .fetch_one(&mut *txn)
    .await
    {
        let request_id: i64 = row.get("zk_proof_id");
        let input: Vec<u8> = row.get("input");
        let chain_id: i32 = row.get("chain_id");
        let contract_address = row.get("contract_address");
        let user_address = row.get("user_address");

        info!(
            message = "Process zk-verify request",
            request_id,
            chain_id,
            user_address,
            contract_address,
            input_len = format!("{}", input.len()),
        );

        let keys = tenant_keys::fetch_tenant_server_key(chain_id, pool, tenant_key_cache, false)
            .await
            .map_err(|err| ExecutionError::ServerKeysNotFound(err.to_string()))?;

        let tenant_id = keys.tenant_id;
        info!(message = "Keys retrieved", request_id, chain_id);

        let res = tokio::task::spawn_blocking(move || {
            let aux_data = auxiliary::ZkData {
                contract_address,
                user_address,
                chain_id: keys.chain_id,
                acl_contract_address: keys.acl_contract_address.clone(),
            };

            verify_proof(request_id, &keys, &aux_data, &input)
        })
        .await?;

        let mut verified = false;
        let mut handles_bytes = vec![];
        match res {
            Ok((cts, blob_hash)) => {
                info!(
                    message = "Proof verification successful",
                    request_id,
                    cts = format!("{}", cts.len()),
                );

                handles_bytes = cts.iter().fold(Vec::new(), |mut acc, ct| {
                    acc.extend_from_slice(ct.handle.as_ref());
                    acc
                });
                verified = true;

                insert_ciphertexts(pool, tenant_id, cts, blob_hash).await?;

                info!(message = "Ciphertexts inserted", request_id);
            }
            Err(err) => {
                error!(
                    message = "Failed to verify proof",
                    request_id,
                    err = err.to_string()
                );
            }
        }

        // Mark as verified=true/false and set handles, if computed
        sqlx::query(
            "UPDATE verify_proofs SET handles = $1, verified = $2, verified_at = NOW()
            WHERE zk_proof_id = $3",
        )
        .bind(handles_bytes)
        .bind(verified)
        .bind(request_id)
        .execute(&mut *txn)
        .await?;

        // Notify
        sqlx::query("SELECT pg_notify($1, '')")
            .bind(conf.notify_database_channel.clone())
            .execute(&mut *txn)
            .await?;

        txn.commit().await?;

        info!(message = "Completed", request_id);
    }

    Ok(())
}

pub(crate) fn verify_proof(
    request_id: i64,
    keys: &FetchTenantKeyResult,
    aux_data: &auxiliary::ZkData,
    raw_ct: &[u8],
) -> Result<(Vec<Ciphertext>, Vec<u8>), ExecutionError> {
    set_server_key(keys.server_key.clone());

    let cts: Vec<SupportedFheCiphertexts> =
        try_verify_and_expand_ciphertext_list(request_id, raw_ct, keys, aux_data)?;

    let mut h = Keccak256::new();
    h.update(raw_ct);
    let blob_hash = h.finalize().to_vec();

    let cts = cts
        .iter()
        .enumerate()
        .map(|(idx, ct)| create_ciphertext(&blob_hash, idx, ct, aux_data))
        .collect();

    Ok((cts, blob_hash))
}

fn try_verify_and_expand_ciphertext_list(
    request_id: i64,
    raw_ct: &[u8],
    keys: &FetchTenantKeyResult,
    aux_data: &auxiliary::ZkData,
) -> Result<Vec<SupportedFheCiphertexts>, ExecutionError> {
    let aux_data_bytes = aux_data
        .assemble()
        .map_err(|e| ExecutionError::InvalidAuxData(e.to_string()))?;

    let the_list: tfhe::ProvenCompactCiphertextList = safe_deserialize(raw_ct)?;

    let expanded: tfhe::CompactCiphertextListExpander = the_list
        .verify_and_expand(&keys.public_params, &keys.pks, &aux_data_bytes)
        .map_err(|_| ExecutionError::InvalidProof(request_id))?;

    Ok(extract_ct_list(&expanded)?)
}

/// Creates a ciphertext
fn create_ciphertext(
    blob_hash: &[u8],
    ct_idx: usize,
    the_ct: &SupportedFheCiphertexts,
    aux_data: &auxiliary::ZkData,
) -> Ciphertext {
    let (serialized_type, compressed) = the_ct.compress();
    let chain_id_bytes: [u8; 32] = alloy_primitives::U256::from(aux_data.chain_id)
        .to_owned()
        .to_be_bytes();

    let mut handle_hash = Keccak256::new();
    handle_hash.update(blob_hash);
    handle_hash.update([ct_idx as u8]);
    handle_hash.update(
        Address::from_str(&aux_data.acl_contract_address)
            .expect("valid acl_contract_address")
            .into_array(),
    );
    handle_hash.update(chain_id_bytes);
    let mut handle = handle_hash.finalize().to_vec();

    assert_eq!(handle.len(), 32);
    // idx cast to u8 must succeed because we don't allow
    // more handles than u8 size
    handle[21] = ct_idx as u8;
    // TODO: change chain ID to be u64
    handle[22..30].copy_from_slice(&(aux_data.chain_id as u64).to_be_bytes());
    handle[30] = serialized_type as u8;
    handle[31] = current_ciphertext_version() as u8;

    Ciphertext {
        handle,
        compressed,
        ct_type: serialized_type,
        ct_version: current_ciphertext_version(),
    }
}

/// Returns the number of remaining tasks in the database.
async fn get_remaining_tasks(pool: &PgPool) -> Result<i64, ExecutionError> {
    let row = sqlx::query(
        "
        SELECT COUNT(*)
        FROM (
            SELECT 1
            FROM verify_proofs
            WHERE verified IS NULL
            ORDER BY zk_proof_id ASC
            FOR UPDATE SKIP LOCKED
        ) AS unlocked_rows;
        ",
    )
    .fetch_one(pool)
    .await?;

    let count: i64 = row.get("count");

    Ok(count)
}

pub(crate) async fn insert_ciphertexts(
    pool: &PgPool,
    tenant_id: i32,
    cts: Vec<Ciphertext>,
    blob_hash: Vec<u8>,
) -> Result<(), ExecutionError> {
    let mut tx = pool.begin().await?;

    for (i, ct) in cts.iter().enumerate() {
        sqlx::query!(
            r#"
            INSERT INTO ciphertexts (
                tenant_id, handle, ciphertext, ciphertext_version, ciphertext_type, 
                input_blob_hash, input_blob_index, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
            ON CONFLICT (tenant_id, handle, ciphertext_version) DO NOTHING;
            "#,
            tenant_id,
            &ct.handle,
            &ct.compressed,
            ct.ct_version,
            ct.ct_type,
            &blob_hash,
            i as i32,
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}
