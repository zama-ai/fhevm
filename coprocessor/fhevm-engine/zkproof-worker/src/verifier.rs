use alloy_primitives::Address;
use fhevm_engine_common::pg_pool::{PostgresPoolManager, ServiceError};
use fhevm_engine_common::telemetry::{self, gen_buckets};
use fhevm_engine_common::tenant_keys::TfheTenantKeys;
use fhevm_engine_common::tenant_keys::{self, FetchTenantKeyResult};
use fhevm_engine_common::tfhe_ops::{current_ciphertext_version, extract_ct_list};
use fhevm_engine_common::types::SupportedFheCiphertexts;

use fhevm_engine_common::utils::safe_deserialize_conformant;
use hex::encode;
use lru::LruCache;
use prometheus::{register_histogram, Histogram};
use sha3::Digest;
use sha3::Keccak256;
use sqlx::{postgres::PgListener, PgPool, Row};
use sqlx::{Postgres, Transaction};
use std::num::NonZero;
use std::str::FromStr;
use tfhe::integer::ciphertext::IntegerProvenCompactCiphertextListConformanceParams;
use tokio::sync::RwLock;
use tokio::task::JoinSet;

use crate::{auxiliary, Config, ExecutionError, MAX_INPUT_INDEX};
use anyhow::Result;

use std::sync::{Arc, LazyLock};
use std::time::SystemTime;
use tfhe::set_server_key;

use fhevm_engine_common::healthz_server::{HealthCheckService, HealthStatus, Version};
use tokio::time::interval;
use tokio::{select, time::Duration};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

const MAX_CACHED_TENANT_KEYS: usize = 100;
const EVENT_CIPHERTEXT_COMPUTED: &str = "event_ciphertext_computed";

const RAW_CT_HASH_DOMAIN_SEPARATOR: [u8; 8] = *b"ZK-w_rct";
const HANDLE_HASH_DOMAIN_SEPARATOR: [u8; 8] = *b"ZK-w_hdl";

pub(crate) static ZKPROOF_LATENCY_HISTOGRAM: LazyLock<Histogram> = LazyLock::new(|| {
    let buckets = gen_buckets(0.01, 10.0);

    register_histogram!(
        "coprocessor_zkverify_latency_seconds",
        "ZKProof verification latencies in seconds",
        buckets
    )
    .unwrap()
});

pub(crate) struct Ciphertext {
    handle: Vec<u8>,
    compressed: Vec<u8>,
    ct_type: i16,
    ct_version: i16,
}

pub struct ZkProofService {
    pool_mngr: PostgresPoolManager,
    conf: Config,

    // Timestamp of the last moment the service was active
    last_active_at: Arc<RwLock<SystemTime>>,
}
impl HealthCheckService for ZkProofService {
    async fn health_check(&self) -> HealthStatus {
        let mut status = HealthStatus::default();
        status.set_db_connected(&self.pool_mngr.pool()).await;
        status
    }

    async fn is_alive(&self) -> bool {
        let last_active_at = *self.last_active_at.read().await;
        let threshold = self.conf.pg_polling_interval + 10;

        (SystemTime::now()
            .duration_since(last_active_at)
            .map(|d| d.as_secs())
            .unwrap_or(u64::MAX) as u32)
            < threshold
    }

    fn get_version(&self) -> Version {
        // Later, the unknowns will be initialized from build.rs
        Version {
            name: "zkproof-worker",
            version: "unknown",
            build: "unknown",
        }
    }
}

impl ZkProofService {
    pub async fn create(conf: Config, token: CancellationToken) -> Option<ZkProofService> {
        // Each worker needs at least 3 pg connections
        let max_pool_connections =
            std::cmp::max(conf.pg_pool_connections, 3 * conf.worker_thread_count);
        let t = telemetry::tracer("init_service", &None);
        let _s = t.child_span("pg_connect");

        let Some(pool_mngr) = PostgresPoolManager::connect_pool(
            token.child_token(),
            conf.database_url.as_str(),
            conf.pg_timeout,
            max_pool_connections,
            Duration::from_secs(2),
            conf.pg_auto_explain_with_min_duration,
        )
        .await
        else {
            error!("Service was cancelled during Postgres pool initialization");
            return None;
        };

        Some(ZkProofService {
            pool_mngr,
            conf,
            last_active_at: Arc::new(RwLock::new(SystemTime::UNIX_EPOCH)),
        })
    }

    pub async fn run(&self) -> Result<(), ExecutionError> {
        execute_verify_proofs_loop(
            self.pool_mngr.clone(),
            self.conf.clone(),
            self.last_active_at.clone(),
        )
        .await
    }
}
/// Executes the main loop for handling verify_proofs requests inserted in the
/// database
pub async fn execute_verify_proofs_loop(
    pool_mngr: PostgresPoolManager,
    conf: Config,
    last_active_at: Arc<RwLock<SystemTime>>,
) -> Result<(), ExecutionError> {
    info!(conf = %conf, "Starting with config");

    // Tenants key cache is shared amongst all workers
    let tenant_key_cache = Arc::new(RwLock::new(LruCache::new(
        NonZero::new(MAX_CACHED_TENANT_KEYS).unwrap(),
    )));

    let t = telemetry::tracer("init_workers", &None);
    let mut s = t.child_span("start_workers");
    telemetry::attribute(&mut s, "count", conf.worker_thread_count.to_string());
    let mut task_set = JoinSet::new();

    for index in 0..conf.worker_thread_count {
        let conf = conf.clone();
        let tenant_key_cache = tenant_key_cache.clone();
        let last_active_at = last_active_at.clone();

        // Spawn a ZK-proof worker
        // All workers compete for zk-proof tasks queued in the 'verify_proof' table.
        let op = move |pool: PgPool, ct: CancellationToken| {
            let tenant_key_cache = tenant_key_cache.clone();
            let last_active_at = last_active_at.clone();
            let conf = conf.clone();
            async move {
                execute_worker(conf, pool, ct, tenant_key_cache, last_active_at)
                    .await
                    .map_err(ServiceError::from)
            }
        };

        pool_mngr
            .spawn_join_set_with_db_retry(op, &mut task_set, format!("worker_{}", index).as_str())
            .await;
    }

    telemetry::end_span(s);

    // Wait for all tasks to complete
    while let Some(result) = task_set.join_next().await {
        if let Err(err) = result {
            error!(error = %err, "A worker failed");
        }
    }

    Ok(())
}

async fn execute_worker(
    conf: Config,
    pool: sqlx::Pool<sqlx::Postgres>,
    token: CancellationToken,
    tenant_key_cache: Arc<RwLock<LruCache<i64, TfheTenantKeys>>>,
    last_active_at: Arc<RwLock<SystemTime>>,
) -> Result<(), ExecutionError> {
    update_last_active(last_active_at.clone()).await;

    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen(&conf.listen_database_channel).await?;

    let mut idle_event = interval(Duration::from_secs(conf.pg_polling_interval as u64));

    loop {
        update_last_active(last_active_at.clone()).await;

        execute_verify_proof_routine(&pool, &tenant_key_cache, &conf).await?;
        let count = get_remaining_tasks(&pool).await?;
        if count > 0 {
            info!({ count }, "zkproof requests available");
            continue;
        }

        select! {
            res = listener.try_recv() => {
                let res = res?;
                match res {
                    Some(notification) => info!( src = %notification.process_id(), "Received notification"),
                    None => {
                        error!("Connection lost");
                        continue;
                    },
                };
            },
            _ = idle_event.tick() => {
                debug!("Polling timeout, rechecking for requests");
            },
            _ = token.cancelled() => {
                info!("Cancellation requested, stopping worker");
                return Ok(());
            }
        }
    }
}

/// Fetch, verify a single proof and then compute signature
async fn execute_verify_proof_routine(
    pool: &PgPool,
    tenant_key_cache: &Arc<RwLock<LruCache<i64, TfheTenantKeys>>>,
    conf: &Config,
) -> Result<(), ExecutionError> {
    let mut txn: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await?;
    if let Ok(row) = sqlx::query(
        "SELECT zk_proof_id, input, chain_id, contract_address, user_address, transaction_id
            FROM verify_proofs
            WHERE verified IS NULL
            ORDER BY zk_proof_id ASC
            LIMIT 1 FOR UPDATE SKIP LOCKED",
    )
    .fetch_one(&mut *txn)
    .await
    {
        let started_at = SystemTime::now();
        let request_id: i64 = row.get("zk_proof_id");
        let input: Vec<u8> = row.get("input");
        let chain_id: i64 = row.get("chain_id");
        let contract_address = row.get("contract_address");
        let user_address = row.get("user_address");
        let transaction_id: Option<Vec<u8>> = row.get("transaction_id");

        info!(
            message = "Process zk-verify request",
            request_id,
            chain_id,
            user_address,
            contract_address,
            input_len = format!("{}", input.len()),
        );

        let t: telemetry::OtelTracer = telemetry::tracer("verify_task", &transaction_id);
        t.set_attribute("request_id", request_id.to_string());

        let s = t.child_span("fetch_keys");
        let keys = tenant_keys::fetch_tenant_server_key(chain_id, pool, tenant_key_cache, false)
            .await
            .map_err(|err| ExecutionError::ServerKeysNotFound(err.to_string()))?;
        telemetry::end_span(s);

        let tenant_id = keys.tenant_id;
        info!(message = "Keys retrieved", request_id, chain_id);

        let res = tokio::task::spawn_blocking(move || {
            let aux_data = auxiliary::ZkData {
                contract_address,
                user_address,
                chain_id: keys.chain_id,
                acl_contract_address: keys.acl_contract_address.clone(),
            };

            verify_proof(request_id, &keys, &aux_data, &input, t)
        })
        .await?;

        let t = telemetry::tracer("db_insert", &transaction_id);
        t.set_attribute("request_id", request_id.to_string());

        let mut verified = false;
        let mut handles_bytes = vec![];
        match res.as_ref() {
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
                let count = cts.len();
                insert_ciphertexts(&mut txn, tenant_id, cts, blob_hash).await?;

                info!(message = "Ciphertexts inserted", request_id);
                t.set_attribute("count", count.to_string());
            }
            Err(err) => {
                error!(
                    message = "Failed to verify proof",
                    request_id,
                    err = err.to_string()
                );
            }
        }

        t.set_attribute("valid", verified.to_string());

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

        if res.is_ok() {
            let elapsed = started_at.elapsed().unwrap_or_default().as_secs_f64();
            if elapsed > 0.0 {
                ZKPROOF_LATENCY_HISTOGRAM.observe(elapsed);
            }
        }

        info!(message = "Completed", request_id);
    }

    Ok(())
}

pub(crate) fn verify_proof(
    request_id: i64,
    keys: &FetchTenantKeyResult,
    aux_data: &auxiliary::ZkData,
    raw_ct: &[u8],
    span: telemetry::OtelTracer,
) -> Result<(Vec<Ciphertext>, Vec<u8>), ExecutionError> {
    set_server_key(keys.server_key.clone());

    let mut s = span.child_span("verify_and_expand");
    let mut cts = match try_verify_and_expand_ciphertext_list(request_id, raw_ct, keys, aux_data) {
        Ok(cts) => {
            telemetry::attribute(&mut s, "count", cts.len().to_string());
            telemetry::end_span(s);
            info!(message = "Ciphertext list expanded", request_id);
            cts
        }
        Err(err) => {
            telemetry::end_span_with_err(s, err.to_string());
            return Err(err);
        }
    };

    let _s = span.child_span("create_ciphertext");

    let mut h = Keccak256::new();
    h.update(RAW_CT_HASH_DOMAIN_SEPARATOR);
    h.update(raw_ct);
    let blob_hash = h.finalize().to_vec();

    let cts = cts
        .iter_mut()
        .enumerate()
        .map(|(idx, ct)| create_ciphertext(request_id, &blob_hash, idx, ct, aux_data, &span))
        .collect::<Result<Vec<Ciphertext>, ExecutionError>>()?;

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

    let the_list: tfhe::ProvenCompactCiphertextList = safe_deserialize_conformant(raw_ct,
        &IntegerProvenCompactCiphertextListConformanceParams::from_public_key_encryption_parameters_and_crs_parameters(
            keys.pks.parameters(), &keys.public_params,
        ))?;

    info!(
        message = "Input list deserialized",
        len = format!("{}", the_list.len()),
        request_id,
    );

    // TODO: Make sure we don't try to verify and expand an empty list as it would panic with the current version of tfhe-rs.
    // Could be removed in the future if tfhe-rs is updated to handle empty lists gracefully.
    if the_list.is_empty() {
        return Ok(vec![]);
    }

    if the_list.len() > (MAX_INPUT_INDEX + 1) as usize {
        return Err(ExecutionError::TooManyInputs(the_list.len()));
    }

    let expanded: tfhe::CompactCiphertextListExpander = the_list
        .verify_and_expand(&keys.public_params, &keys.pks, &aux_data_bytes)
        .map_err(|err| ExecutionError::InvalidProof(request_id, err.to_string()))?;

    Ok(extract_ct_list(&expanded)?)
}

/// Creates a ciphertext
fn create_ciphertext(
    request_id: i64,
    blob_hash: &[u8],
    ct_idx: usize,
    the_ct: &mut SupportedFheCiphertexts,
    aux_data: &auxiliary::ZkData,
    span: &telemetry::OtelTracer,
) -> Result<Ciphertext, ExecutionError> {
    if ct_idx > MAX_INPUT_INDEX as usize {
        return Err(ExecutionError::TooManyInputs(ct_idx));
    }

    let chain_id_bytes: [u8; 32] = alloy_primitives::U256::from(aux_data.chain_id)
        .to_owned()
        .to_be_bytes();
    let mut handle_hash = Keccak256::new();
    handle_hash.update(HANDLE_HASH_DOMAIN_SEPARATOR);
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

    // Add the full 256bit hash as re-randomization metadata, NOT the
    // truncated hash of the handle
    the_ct.add_re_randomization_metadata(&handle);
    let (serialized_type, compressed) = the_ct.compress();

    // idx cast to u8 must succeed because we don't allow
    // more handles than u8 size
    handle[21] = ct_idx as u8;
    // TODO: change chain ID to be u64
    handle[22..30].copy_from_slice(&(aux_data.chain_id as u64).to_be_bytes());
    handle[30] = serialized_type as u8;
    handle[31] = current_ciphertext_version() as u8;

    let t = &mut span.child_span("create_handle");
    telemetry::attribute(t, "request_id", request_id.to_string());
    telemetry::attribute(t, "handle", hex::encode(handle.clone()));
    telemetry::attribute(t, "chain_id", aux_data.chain_id.to_string());
    telemetry::attribute(t, "ct_idx", ct_idx.to_string());
    telemetry::attribute(t, "user_address", aux_data.user_address.clone());
    telemetry::attribute(t, "contract_address", aux_data.user_address.clone());
    telemetry::attribute(t, "version", current_ciphertext_version().to_string());
    telemetry::attribute(t, "type", serialized_type.to_string());
    telemetry::attribute(
        t,
        "acl_contract_address",
        aux_data.acl_contract_address.clone(),
    );

    info!(handle = ?encode(&handle), "Create new handle");

    Ok(Ciphertext {
        handle,
        compressed,
        ct_type: serialized_type,
        ct_version: current_ciphertext_version(),
    })
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
    db_txn: &mut Transaction<'_, Postgres>,
    tenant_id: i32,
    cts: &[Ciphertext],
    blob_hash: &Vec<u8>,
) -> Result<(), ExecutionError> {
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
        .execute(db_txn.as_mut())
        .await?;
    }

    // Notify all workers that new ciphertext is inserted
    // For now, it's only the SnS workers that are listening for these events
    let _ = sqlx::query!(
        "SELECT pg_notify($1, 'zk-worker')",
        EVENT_CIPHERTEXT_COMPUTED
    )
    .execute(db_txn.as_mut())
    .await?;
    Ok(())
}

async fn update_last_active(last_active_at: Arc<RwLock<SystemTime>>) {
    let mut value = last_active_at.write().await;
    *value = SystemTime::now();
}
