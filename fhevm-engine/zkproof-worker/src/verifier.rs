use sqlx::{postgres::PgListener, PgPool, Row};
use std::str::FromStr;

use crate::ExecutionError;
use anyhow::Result;
use aws_sdk_s3::Client;
use tfhe::safe_serialization::safe_deserialize;

use tfhe::zk::CompactPkeCrs;
use tfhe::{CompactPublicKey, ProvenCompactCiphertextList};
use tokio::io::AsyncReadExt;

use tokio::{select, time::Duration};
use tracing::debug;

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
    // TODO: Should we retrieve S3 bucket info from the tenants table for a specific tenant API key?
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
            _ = tokio::time::sleep(Duration::from_secs(2)) => {
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
        "SELECT zk_proof_id, input, handles, chain_id, contract_address, user_address
            FROM verify_proofs
            WHERE verified = false AND retry_count < 5
            ORDER BY zk_proof_id ASC
            LIMIT 1 FOR UPDATE SKIP LOCKED",
    )
    .fetch_one(&mut *txn)
    .await
    {
        let zk_proof_id: i64 = row.get("zk_proof_id");
        let input: Vec<u8> = row.get("input");
        let handles: Vec<u8> = row.get("handles");
        let chain_id: i32 = row.get("chain_id");
        let contract_address = row.get("contract_address");
        let user_address = row.get("user_address");

        let md = Metadata::new(contract_address, user_address, chain_id);
        match verify_proof_and_sign(zk_proof_id, crs, compact_pubkey, &md, &input, &handles).await {
            Ok(_signature) => {
                // TODO: Should we store the signature in the database for this zk_proof_id?

                // Mark as verified
                sqlx::query(
                    "UPDATE verify_proofs SET verified = true, is_valid = true, verified_at = NOW()
                    WHERE zk_proof_id = $1",
                )
                .bind(zk_proof_id)
                .execute(&mut *txn)
                .await?;
            }
            Err(ExecutionError::InvalidProof(_)) => {
                // TODO: Should we mark the proof as invalid in the database?
                sqlx::query(
                    "UPDATE verify_proofs SET verified = true, is_valid = false, verified_at = NOW()
                    WHERE zk_proof_id = $1",
                )
                .bind(zk_proof_id)
                .execute(&mut *txn)
                .await?;
            }
            Err(_) => {
                // Increment retry count and log error
                sqlx::query(
                    "UPDATE verify_proofs SET retry_count = retry_count + 1, last_retry_at = NOW()
                    WHERE zk_proof_id = $1",
                )
                .bind(zk_proof_id)
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
    proof_id: i64,
    crs: &CompactPkeCrs,
    public_key: &CompactPublicKey,
    md: &Metadata,
    input_bytes: &[u8],
    handles: &[u8],
) -> Result<Vec<u8>, ExecutionError> {
    let md_bytes = md.assemble();

    // TODO: is the input bytes a valid ProvenCompactCiphertextList?
    let mut cursor = std::io::Cursor::new(input_bytes);
    let proven_ct: ProvenCompactCiphertextList = safe_deserialize(&mut cursor, SAFE_SER_SIZE_LIMIT)
        .map_err(ExecutionError::InvalidInputBytes)?;

    if proven_ct.verify(crs, public_key, &md_bytes).is_invalid() {
        return Err(ExecutionError::InvalidProof(proof_id));
    }

    // TODO: How to extract ciphertexts from packed ciphertexts according to data types embedded in packed ciphertexts?
    // proven_ct.iter()

    compute_signature(&proven_ct, handles)
}

pub(crate) fn compute_signature(
    _proven_ct: &ProvenCompactCiphertextList,
    _handles: &[u8],
) -> Result<Vec<u8>, ExecutionError> {
    // TODO: Implement signature computation
    Ok(vec![])
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

pub(crate) struct Metadata {
    chain_id: i32,
    user_address: String,
    contract_address: String,
}

impl Metadata {
    pub fn new(contract_address: String, user_address: String, chain_id: i32) -> Self {
        Self {
            contract_address,
            user_address,
            chain_id,
        }
    }

    /// creates the metadata (auxiliary data) for proving/verifying the input ZKPs from the individual inputs
    ///
    /// metadata is `contract_addr || user_addr  || chain_id` i.e. 92 bytes since chain ID is encoded as a 32 byte big endian integer
    pub fn assemble(&self) -> [u8; 92] {
        let contract_address = alloy_primitives::Address::from_str(&self.user_address).unwrap();
        let client_address = alloy_primitives::Address::from_str(&self.contract_address).unwrap();
        let chain_id = alloy_primitives::U256::from(self.chain_id).to_owned();

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
        .map_err(ExecutionError::InvalidInputBytes)?;

    Ok(pks)
}
