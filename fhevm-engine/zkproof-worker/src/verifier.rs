use alloy_primitives::Address;
use fhevm_engine_common::tenant_keys::TfheTenantKeys;
use fhevm_engine_common::tenant_keys::{self, FetchTenantKeyResult};
use fhevm_engine_common::tfhe_ops::{current_ciphertext_version, extract_ct_list};
use fhevm_engine_common::types::SupportedFheCiphertexts;

use fhevm_engine_common::utils::safe_deserialize;
use lru::LruCache;
use sha3::Digest;
use sha3::Keccak256;
use sqlx::{postgres::PgListener, PgPool, Row};
use std::num::NonZero;
use std::str::FromStr;
use tokio::sync::RwLock;

use crate::{auxiliary, ExecutionError};
use anyhow::Result;

use std::sync::Arc;
use tfhe::set_server_key;

use tokio::{select, time::Duration};
use tracing::{debug, error, info};

pub const SAFE_SER_SIZE_LIMIT: u64 = 1024 * 1024 * 1024 * 2;
const MAX_CACHED_TENANT_KEYS: usize = 100;

pub(crate) struct Ciphertext {
    handle: Vec<u8>,
    compressed: Vec<u8>,
}

#[derive(Default)]
pub struct Config {
    pub database_url: String,
    pub listen_database_channel: String,
}

/// Executes the main loop for handling verify_proofs requests inserted in the database
pub async fn execute_verify_proofs_loop(conf: &Config) -> Result<(), ExecutionError> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&conf.database_url)
        .await?;

    info!("Starting verify_proofs loop");

    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen(&conf.listen_database_channel).await?;

    let tenant_key_cache = Arc::new(RwLock::new(LruCache::new(
        NonZero::new(MAX_CACHED_TENANT_KEYS).unwrap(),
    )));

    loop {
        if let Err(e) = execute_verify_proof_routine(&pool, &tenant_key_cache).await {
            debug!(target: "worker", "Error executing verify_proof_routine: {:?}", e);
        }

        select! {
            _ = listener.try_recv() => {
                info!(target: "worker", "Received notification");
            },
            _ = tokio::time::sleep(Duration::from_secs(10)) => {
                debug!(target: "worker", "Polling timeout, rechecking for tasks");
            }
        }
    }
}

/// Fetch, verify a single proof and then compute signature
async fn execute_verify_proof_routine(
    pool: &PgPool,
    tenant_key_cache: &Arc<RwLock<LruCache<i32, TfheTenantKeys>>>,
) -> Result<(), ExecutionError> {
    let mut txn: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await?;
    if let Ok(row) = sqlx::query(
        "SELECT zk_proof_id, input, tenant_id, contract_address, user_address
            FROM verify_proofs
            WHERE verified IS NULL
            ORDER BY zk_proof_id ASC
            LIMIT 1 FOR UPDATE SKIP LOCKED",
    )
    .fetch_one(&mut *txn)
    .await
    {
        let request_id: i64 = row.get("zk_proof_id");
        info!(message = "Processing proof", request_id);
        let input: Vec<u8> = row.get("input");
        let tenant_id: i32 = row.get("tenant_id");
        let contract_address = row.get("contract_address");
        let user_address = row.get("user_address");

        // TODO: fetch tenant keys by tenant id
        let keys = tenant_keys::fetch_tenant_server_key(tenant_id, pool, tenant_key_cache)
            .await
            .map_err(|err| ExecutionError::ServerKeysNotFound(err.to_string()))?;

        let aux_data = auxiliary::ZkData {
            contract_address,
            user_address,
            chain_id: keys.chain_id,
            acl_contract_address: keys.acl_contract_address.clone(),
        };

        let mut verified = false;
        let mut handles_bytes = vec![];
        match verify_proof_and_sign(request_id, &keys, &aux_data, &input).await {
            Ok(handles) => {
                handles_bytes = handles.iter().fold(Vec::new(), |mut acc, ct| {
                    acc.extend_from_slice(ct.handle.as_ref());
                    acc
                });
                verified = true;
                // TODO: Insert ciphertexts into the database

                info!(message = "Check valid proof", request_id);
            }
            Err(err) => {
                error!(
                    message = "Failed to verify proof",
                    request_id,
                    err = err.to_string()
                );
            }
        }

        // Mark as verified and set handles
        sqlx::query(
            "UPDATE verify_proofs SET handles = $1, verified = $2, verified_at = NOW()
            WHERE zk_proof_id = $3",
        )
        .bind(handles_bytes)
        .bind(verified)
        .bind(request_id)
        .execute(&mut *txn)
        .await?;

        // Notify verify_proof_responses
        sqlx::query("SELECT pg_notify($1, '')")
            .bind("verify_proof_responses")
            .execute(&mut *txn)
            .await?;

        txn.commit().await?;
    }

    Ok(())
}

pub(crate) async fn verify_proof_and_sign(
    request_id: i64,
    keys: &FetchTenantKeyResult,
    aux_data: &auxiliary::ZkData,
    raw_ct: &[u8],
) -> Result<Vec<Ciphertext>, ExecutionError> {
    set_server_key(keys.server_key.clone());

    let cts: Vec<SupportedFheCiphertexts> =
        try_verify_and_expand_ciphertext_list(request_id, raw_ct, keys, aux_data)?;

    let mut h = Keccak256::new();
    h.update(raw_ct);
    let blob_hash = h.finalize().to_vec();

    let handles = cts
        .iter()
        .enumerate()
        .map(|(idx, ct)| ciphertext_handle(&blob_hash, idx, ct, aux_data))
        .collect();

    Ok(handles)
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

    let the_list: tfhe::ProvenCompactCiphertextList =
        safe_deserialize(raw_ct).expect(" deserialization failed");

    let expanded: tfhe::CompactCiphertextListExpander = the_list
        .verify_and_expand(&keys.public_params, &keys.pks, &aux_data_bytes)
        .map_err(|_| ExecutionError::InvalidProof(request_id))?;

    Ok(extract_ct_list(&expanded)?)
}

/// Computes the handle for a ciphertext
fn ciphertext_handle(
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
    handle[29] = ct_idx as u8;
    handle[30] = serialized_type as u8;
    handle[31] = current_ciphertext_version() as u8;

    Ciphertext { handle, compressed }
}
