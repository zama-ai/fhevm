use fhevm_engine_common::tenant_keys::TfheTenantKeys;
use fhevm_engine_common::tenant_keys::{self, FetchTenantKeyResult};
use fhevm_engine_common::tfhe_ops::{current_ciphertext_version, extract_ct_list};
use fhevm_engine_common::types::SupportedFheCiphertexts;

use lru::LruCache;
use sha3::Digest;
use sha3::Keccak256;
use sqlx::{postgres::PgListener, PgPool, Row};
use std::num::NonZero;
use std::str::FromStr;
use tokio::sync::RwLock;

use crate::ExecutionError;
use anyhow::Result;
use tfhe::safe_serialization::safe_deserialize;

use std::sync::Arc;
use tfhe::set_server_key;

use tokio::{select, time::Duration};
use tracing::{debug, error};

pub const SAFE_SER_SIZE_LIMIT: u64 = 1024 * 1024 * 1024 * 2;

#[derive(Default)]
pub struct Config {
    database_url: String,
    listen_database_channel: String,
}

/// Executes the main loop for handling verify_proofs requests inserted in the database
pub async fn execute_verify_proofs_loop(conf: &Config) -> Result<(), ExecutionError> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&conf.database_url)
        .await?;

    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen(&conf.listen_database_channel).await?;

    let tenant_key_cache = Arc::new(RwLock::new(LruCache::new(NonZero::new(100).unwrap())));
    tenant_keys::fetch_tenant_server_key(1, &pool, &tenant_key_cache)
        .await
        .map_err(|err| ExecutionError::ServerKeysNotFound(err.to_string()))?;

    loop {
        if let Err(e) = execute_verify_proof_routine(&pool, &tenant_key_cache).await {
            debug!(target: "worker", "Error executing verify_proof_routine: {:?}", e);
        }

        select! {
            _ = listener.try_recv() => {
                debug!(target: "worker", "Received notification");
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
        "SELECT zk_proof_id, input, chain_id, contract_address, user_address
            FROM verify_proofs
            WHERE verified = NULL
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

        // TODO: fetch tenant keys by tenant id
        let keys = tenant_keys::fetch_tenant_server_key(chain_id, pool, tenant_key_cache)
            .await
            .map_err(|err| ExecutionError::ServerKeysNotFound(err.to_string()))?;

        let aux_data: AuxiliaryData = AuxiliaryData {
            contract_address,
            user_address,
            chain_id,
            acl_contract_address: keys.acl_contract_address.clone(),
        };

        let mut verified = false;
        let mut handles_bytes = vec![];
        match verify_proof_and_sign(request_id, &keys, &aux_data, &input).await {
            Ok(handles) => {
                handles_bytes = handles.iter().fold(Vec::new(), |mut acc, h| {
                    acc.extend_from_slice(h.as_ref());
                    acc
                });
                verified = true;
            }
            Err(err) => {
                error!("Failed to verify proof: {}, err: {}", request_id, err);
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
    aux_data: &AuxiliaryData,
    raw_ct: &[u8],
) -> Result<Vec<Vec<u8>>, ExecutionError> {
    let cts: Vec<SupportedFheCiphertexts> =
        try_verify_and_expand_ciphertext_list(request_id, raw_ct, keys, aux_data)?;

    set_server_key(keys.server_key.clone());
    let handles = cts
        .iter()
        .enumerate()
        .map(|(idx, ct)| {
            ciphertext_handle(&ct.compress().1, idx as u8, ct.type_num() as u8, aux_data)
        })
        .collect();

    // TODO: Insert ciphertexts into the database

    Ok(handles)
}

fn try_verify_and_expand_ciphertext_list(
    request_id: i64,
    raw_ct: &[u8],
    keys: &FetchTenantKeyResult,
    aux_data: &AuxiliaryData,
) -> Result<Vec<SupportedFheCiphertexts>, ExecutionError> {
    let aux_data_bytes = aux_data.assemble();

    let mut cursor = std::io::Cursor::new(raw_ct);
    let the_list: tfhe::ProvenCompactCiphertextList =
        safe_deserialize(&mut cursor, SAFE_SER_SIZE_LIMIT)
            .map_err(ExecutionError::InvalidCiphertextBytes)?;

    let expanded: tfhe::CompactCiphertextListExpander = the_list
        .verify_and_expand(&keys.public_params, &keys.pks, &aux_data_bytes)
        .map_err(|_| ExecutionError::InvalidProof(request_id))?;

    Ok(extract_ct_list(&expanded)?)
}

/// Computes the handle for a ciphertext
/// handle = hash(individual ciphertext, zkpok, type, user_address, contract_address, keyID, chain id)
fn ciphertext_handle(
    compress_ct: &[u8],
    ct_idx: u8,
    ct_type: u8,
    aux_data: &AuxiliaryData,
) -> Vec<u8> {
    let mut handle_hash = Keccak256::new();
    handle_hash.update(compress_ct); // individual ciphertext
    handle_hash.update(aux_data.user_address.as_bytes());
    handle_hash.update(aux_data.contract_address.as_bytes());
    handle_hash.update([ct_idx]);
    // handle_hash.update(acl_contract_address.as_slice());
    // TODO: handle_hash.update(&chain_id_be);
    let mut handle = handle_hash.finalize().to_vec();
    assert_eq!(handle.len(), 32);
    // idx cast to u8 must succeed because we don't allow
    // more handles than u8 size
    handle[29] = ct_idx;
    handle[30] = ct_type;
    handle[31] = current_ciphertext_version() as u8;

    handle.to_vec()
}

pub(crate) struct AuxiliaryData {
    chain_id: i32,
    user_address: String,
    contract_address: String,
    acl_contract_address: String,
}

impl AuxiliaryData {
    /// creates the metadata (auxiliary data) for proving/verifying the input ZKPs from the individual inputs
    ///
    /// metadata is `contract_addr || user_addr  || chain_id` i.e. 92 bytes since chain ID is encoded as a 32 byte big endian integer
    pub fn assemble(&self) -> [u8; 92] {
        let contract_address = alloy_primitives::Address::from_str(&self.user_address).unwrap();
        let client_address = alloy_primitives::Address::from_str(&self.contract_address).unwrap();
        let chain_id = alloy_primitives::U256::from(self.chain_id).to_owned();
        // TODO: acl_contract_address

        let mut metadata = [0_u8; 92];
        let contract_bytes = contract_address.into_array();
        let client_bytes = client_address.into_array();

        let chain_id_bytes: [u8; 32] = chain_id.to_be_bytes();
        let front = [contract_bytes, client_bytes].concat();
        metadata[..60].copy_from_slice(front.as_slice());
        metadata[60..].copy_from_slice(&chain_id_bytes);

        // TODO: How to build metadata for the proof?
        metadata
    }
}
