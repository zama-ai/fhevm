use alloy_primitives::{Address, U256};
use fhevm_engine_common::tfhe_ops::{current_ciphertext_version, extract_ct_list};
use fhevm_engine_common::types::{FhevmError, SupportedFheCiphertexts};

use sha3::Digest;
use sha3::Keccak256;
use sqlx::{postgres::PgListener, PgPool, Row};

use std::str::FromStr;

use crate::ExecutionError;
use anyhow::Result;
use aws_sdk_s3::Client;
use tfhe::safe_serialization::safe_deserialize;

use tfhe::zk::CompactPkeCrs;
use tfhe::{set_server_key, CompactPublicKey};
use tokio::io::AsyncReadExt;

use tokio::{select, time::Duration};
use tracing::{debug, info};

pub const SAFE_SER_SIZE_LIMIT: u64 = 1024 * 1024 * 1024 * 2;

#[derive(Default)]
pub struct Config {
    database_url: String,
    listen_database_channel: String,

    crs_bucket_name: String,
    crs_object_key: String,
}

/// Executes the main loop for handling verify_proofs requests inserted in the database
pub async fn execute_verify_proofs_loop(conf: &Config) -> Result<(), ExecutionError> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&conf.database_url)
        .await?;

    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen(&conf.listen_database_channel).await?;

    // Fetch CRS
    let crs = download_crs(&conf.crs_bucket_name, &conf.crs_object_key).await?;
    let compact_pubkey = fetch_compact_pubkey()?;

    loop {
        if let Err(e) = execute_verify_proof_routine(&pool, &crs, &compact_pubkey).await {
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
    crs: &CompactPkeCrs,
    compact_pubkey: &CompactPublicKey,
) -> Result<(), ExecutionError> {
    let mut txn: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await?;
    if let Ok(row) = sqlx::query(
        "SELECT zk_proof_id, input, chain_id, contract_address, user_address
            FROM verify_proofs
            WHERE verified = false AND retry_count < 5
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

        let md: AuxiliaryData = AuxiliaryData {
            contract_address,
            user_address,
            chain_id,
        };

        match verify_proof_and_sign(request_id, crs, compact_pubkey, &md, &input).await {
            Ok(handles) => {
                let handles_bytes = handles.iter().fold(Vec::new(), |mut acc, h| {
                    acc.extend_from_slice(h.as_ref());
                    acc
                });

                // Mark as verified and set handles
                sqlx::query(
                    "UPDATE verify_proofs SET handles = $2, verified = true, is_valid = true, verified_at = NOW()
                    WHERE zk_proof_id = $1",
                )
                .bind(request_id)
                .bind(handles_bytes)
                .execute(&mut *txn)
                .await?;
            }
            Err(_) => {
                sqlx::query(
                    "UPDATE verify_proofs SET verified = true, is_valid = false, verified_at = NOW()
                    WHERE zk_proof_id = $1",
                )
                .bind(request_id)
                .execute(&mut *txn)
                .await?;
            }
        }

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
    crs: &CompactPkeCrs,
    public_key: &CompactPublicKey,
    aux_data: &AuxiliaryData,
    raw_ct: &[u8],
) -> Result<Vec<Vec<u8>>, ExecutionError> {
    let cts: Vec<SupportedFheCiphertexts> =
        try_verify_and_expand_ciphertext_list(request_id, raw_ct, crs, public_key, aux_data)?;

    // TODO: set_server_key(self.keys.server_key.clone());
    let handles = cts
        .iter()
        .map(|ct: &SupportedFheCiphertexts| {
            ciphertext_handle(&ct.compress().1, ct.type_num() as u8) // TODO: double check
        })
        .collect();

    Ok(handles)
}

fn try_verify_and_expand_ciphertext_list(
    request_id: i64,
    raw_ct: &[u8],
    crs: &CompactPkeCrs,
    public_key: &CompactPublicKey,
    aux_data: &AuxiliaryData,
) -> Result<Vec<SupportedFheCiphertexts>, ExecutionError> {
    let aux_data_bytes = aux_data.assemble();

    let mut cursor = std::io::Cursor::new(raw_ct);
    let the_list: tfhe::ProvenCompactCiphertextList =
        safe_deserialize(&mut cursor, SAFE_SER_SIZE_LIMIT)
            .map_err(ExecutionError::InvalidCiphertextBytes)?;

    let expanded = the_list
        .verify_and_expand(crs, public_key, &aux_data_bytes)
        .map_err(|_| ExecutionError::InvalidProof(request_id))?;

    Ok(extract_ct_list(&expanded)?)
}

pub fn ciphertext_handle(ciphertext: &[u8], ct_type: u8) -> Vec<u8> {
    let mut handle: Vec<u8> = Keccak256::digest(ciphertext).to_vec();
    handle[30] = ct_type;
    handle[31] = current_ciphertext_version() as u8;
    handle
}

/// Retrieves the CRS from an S3 bucket
async fn download_crs(
    bucket_name: &str,
    object_key: &str,
) -> Result<CompactPkeCrs, ExecutionError> {
    let bytes = download_s3_binary(bucket_name, object_key).await?;
    let mut cursor = std::io::Cursor::new(bytes);
    let crs: CompactPkeCrs = safe_deserialize(&mut cursor, SAFE_SER_SIZE_LIMIT)
        .map_err(ExecutionError::InvalidCrsBytes)?;
    Ok(crs)
}

/// Downloads a binary file from an S3 bucket and returns it as a Vec<u8>
pub async fn download_s3_binary(
    bucket_name: &str,
    object_key: &str,
) -> Result<Vec<u8>, ExecutionError> {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let s3_client = Client::new(&config);

    // Fetch the object from S3
    let resp = s3_client
        .get_object()
        .bucket(bucket_name)
        .key(object_key)
        .send()
        .await?;

    // Read the binary data into a buffer
    let mut stream = resp.body.into_async_read();
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await?;

    Ok(buffer)
}

pub(crate) struct AuxiliaryData {
    chain_id: i32,
    user_address: String,
    contract_address: String,
}

impl AuxiliaryData {
    /// creates the metadata (auxiliary data) for proving/verifying the input ZKPs from the individual inputs
    ///
    /// metadata is `contract_addr || user_addr  || chain_id` i.e. 92 bytes since chain ID is encoded as a 32 byte big endian integer
    pub fn assemble(&self) -> [u8; 92] {
        let contract_address = alloy_primitives::Address::from_str(&self.user_address).unwrap();
        let client_address = alloy_primitives::Address::from_str(&self.contract_address).unwrap();
        let chain_id = alloy_primitives::U256::from(self.chain_id).to_owned();
        // TODO: ACL_ADDRESS from Tenants table

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

pub fn fetch_compact_pubkey() -> Result<CompactPublicKey, ExecutionError> {
    // TODO: Should we fetch pks bytes (compact public key) from `tenants` table?
    let bytes = vec![]; // Fetch from database

    let mut cursor = std::io::Cursor::new(bytes);
    let pks: tfhe::CompactPublicKey = safe_deserialize(&mut cursor, SAFE_SER_SIZE_LIMIT)
        .map_err(ExecutionError::InvalidPkBytes)?;

    Ok(pks)
}
