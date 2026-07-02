//! RFC-029 host-listener ingestion: the compressed-key material stays
//! staged until the cutover schedule is applied, the schedule is
//! single-assignment with loud conflict handling, and orphaned events
//! are cancelled.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use serial_test::serial;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Row};
use tokio_util::bytes::Bytes;

use fhevm_engine_common::chain_id::ChainId;
use host_listener::kms_generation::aws_s3::AwsS3Interface;
use host_listener::kms_generation::digest::digest_key;
use host_listener::kms_generation::process_kms_generation_activations;
use test_harness::instance::ImportMode;

const TEST_CHAIN_ID: u64 = 31888;
const COMPRESSED_BYTES: &[u8] = b"rfc029-compressed-xof-keyset";

#[derive(Clone)]
struct SingleBlobS3 {
    objects: Arc<HashMap<String, Bytes>>,
}

#[async_trait]
impl AwsS3Interface for SingleBlobS3 {
    async fn get_bucket_key(
        &self,
        _url: &str,
        _bucket: &str,
        key_suffix: &str,
    ) -> anyhow::Result<Bytes> {
        self.objects
            .iter()
            .find(|(key, _)| key.ends_with(key_suffix))
            .map(|(_, bytes)| bytes.clone())
            .ok_or_else(|| anyhow::anyhow!("NoSuchKey: {key_suffix}"))
    }
}

struct Env {
    _instance: test_harness::instance::DBInstance,
    pool: Pool<Postgres>,
    key_id: Vec<u8>,
}

async fn setup() -> anyhow::Result<Env> {
    let instance =
        test_harness::instance::setup_test_db(ImportMode::WithKeysNoSns)
            .await
            .expect("valid db instance");
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(instance.db_url())
        .await?;

    // The fixture key row is the "existing live key" being migrated.
    let key_id: Vec<u8> = sqlx::query(
        "SELECT key_id_gw FROM keys ORDER BY sequence_number DESC LIMIT 1",
    )
    .fetch_one(&pool)
    .await?
    .try_get("key_id_gw")?;
    // Reset it to the pre-migration shape: legacy material only.
    sqlx::query(
        "UPDATE keys SET compressed_xof_keyset = NULL WHERE key_id_gw = $1",
    )
    .bind(&key_id)
    .execute(&pool)
    .await?;
    for table in [
        "compressed_key_material_events",
        "compressed_key_cutover_events",
        "compressed_key_cutover_hosts",
        "compressed_key_cutover",
    ] {
        sqlx::query(&format!("DELETE FROM {table}"))
            .execute(&pool)
            .await?;
    }
    sqlx::query("DELETE FROM host_chain_blocks_valid WHERE chain_id = $1")
        .bind(ChainId::try_from(TEST_CHAIN_ID)?.as_i64())
        .execute(&pool)
        .await?;

    Ok(Env {
        _instance: instance,
        pool,
        key_id,
    })
}

async fn insert_block(
    pool: &Pool<Postgres>,
    block_hash: &[u8],
    block_number: i64,
    status: &str,
) -> anyhow::Result<()> {
    sqlx::query(
        "INSERT INTO host_chain_blocks_valid (chain_id, block_hash, block_number, block_status)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (chain_id, block_hash) DO UPDATE SET block_status = EXCLUDED.block_status",
    )
    .bind(ChainId::try_from(TEST_CHAIN_ID)?.as_i64())
    .bind(block_hash)
    .bind(block_number)
    .bind(status)
    .execute(pool)
    .await?;
    Ok(())
}

async fn insert_material_event(
    pool: &Pool<Postgres>,
    key_id: &[u8],
    block_hash: &[u8],
    block_number: i64,
) -> anyhow::Result<()> {
    sqlx::query(
        "INSERT INTO compressed_key_material_events (
            chain_id, block_hash, block_number, key_id, key_digest_server, storage_urls
        ) VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(ChainId::try_from(TEST_CHAIN_ID)?.as_i64())
    .bind(block_hash)
    .bind(block_number)
    .bind(key_id)
    .bind(digest_key(COMPRESSED_BYTES).to_vec())
    .bind(vec!["https://s3.region.amazonaws.com/test-bucket".to_owned()])
    .execute(pool)
    .await?;
    Ok(())
}

async fn insert_cutover_event(
    pool: &Pool<Postgres>,
    key_id: &[u8],
    block_hash: &[u8],
    block_number: i64,
    gateway_cutover_block: i64,
) -> anyhow::Result<()> {
    sqlx::query(
        "INSERT INTO compressed_key_cutover_events (
            chain_id, block_hash, block_number, key_id, gateway_cutover_block, host_cutovers
        ) VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(ChainId::try_from(TEST_CHAIN_ID)?.as_i64())
    .bind(block_hash)
    .bind(block_number)
    .bind(key_id)
    .bind(gateway_cutover_block)
    .bind(format!(
        r#"[{{"chain_id": "{TEST_CHAIN_ID}", "cutover_block": 1000}}]"#
    ))
    .execute(pool)
    .await?;
    Ok(())
}

fn s3_with_material(key_id: &[u8]) -> SingleBlobS3 {
    SingleBlobS3 {
        objects: Arc::new(HashMap::from([(
            format!("/CompressedXofKeySet/{}", alloy::hex::encode(key_id)),
            Bytes::from_static(COMPRESSED_BYTES),
        )])),
    }
}

async fn keys_compressed_blob(
    pool: &Pool<Postgres>,
    key_id: &[u8],
) -> anyhow::Result<Option<Vec<u8>>> {
    Ok(sqlx::query(
        "SELECT compressed_xof_keyset FROM keys WHERE key_id_gw = $1",
    )
    .bind(key_id)
    .fetch_one(pool)
    .await?
    .try_get("compressed_xof_keyset")?)
}

/// Material published (finalized, verified, staged) but not scheduled
/// must never reach the keys table; once the schedule is applied it
/// must — in that order, so the selection policy is always visible
/// before the material can influence reads.
#[tokio::test]
#[serial(db)]
async fn material_stays_staged_until_cutover_is_scheduled() -> anyhow::Result<()>
{
    let env = setup().await?;
    let s3 = s3_with_material(&env.key_id);

    let material_block = vec![0x0A_u8; 32];
    insert_block(&env.pool, &material_block, 100, "finalized").await?;
    insert_material_event(&env.pool, &env.key_id, &material_block, 100).await?;

    // Two passes: one downloads/stages, one would apply if allowed.
    process_kms_generation_activations(env.pool.clone(), s3.clone()).await?;
    process_kms_generation_activations(env.pool.clone(), s3.clone()).await?;

    let status: String = sqlx::query(
        "SELECT status FROM compressed_key_material_events WHERE key_id = $1",
    )
    .bind(&env.key_id)
    .fetch_one(&env.pool)
    .await?
    .try_get("status")?;
    assert_eq!(status, "ready", "material must be staged after download");
    assert_eq!(
        keys_compressed_blob(&env.pool, &env.key_id).await?,
        None,
        "staged material must not reach the keys table before the cutover is scheduled"
    );

    // Schedule arrives (finalized) -> schedule applied, then material.
    let schedule_block = vec![0x0B_u8; 32];
    insert_block(&env.pool, &schedule_block, 110, "finalized").await?;
    insert_cutover_event(&env.pool, &env.key_id, &schedule_block, 110, 500)
        .await?;
    process_kms_generation_activations(env.pool.clone(), s3.clone()).await?;

    let gateway_block: i64 = sqlx::query(
        "SELECT gateway_cutover_block FROM compressed_key_cutover WHERE key_id = $1",
    )
    .bind(&env.key_id)
    .fetch_one(&env.pool)
    .await?
    .try_get("gateway_cutover_block")?;
    assert_eq!(gateway_block, 500);

    let host_block: i64 = sqlx::query(
        "SELECT cutover_block FROM compressed_key_cutover_hosts WHERE key_id = $1 AND chain_id = $2",
    )
    .bind(&env.key_id)
    .bind(ChainId::try_from(TEST_CHAIN_ID)?.as_i64())
    .fetch_one(&env.pool)
    .await?
    .try_get("cutover_block")?;
    assert_eq!(host_block, 1000);

    assert_eq!(
        keys_compressed_blob(&env.pool, &env.key_id).await?,
        Some(COMPRESSED_BYTES.to_vec()),
        "material must land in keys once the schedule is applied"
    );
    Ok(())
}

/// A conflicting second schedule must never overwrite the stored one,
/// and must be marked as an error (never silently swallowed).
#[tokio::test]
#[serial(db)]
async fn conflicting_cutover_schedule_is_rejected_loudly() -> anyhow::Result<()>
{
    let env = setup().await?;
    let s3 = s3_with_material(&env.key_id);

    let first = vec![0x0C_u8; 32];
    insert_block(&env.pool, &first, 120, "finalized").await?;
    insert_cutover_event(&env.pool, &env.key_id, &first, 120, 500).await?;
    process_kms_generation_activations(env.pool.clone(), s3.clone()).await?;

    let second = vec![0x0D_u8; 32];
    insert_block(&env.pool, &second, 130, "finalized").await?;
    insert_cutover_event(&env.pool, &env.key_id, &second, 130, 999).await?;
    process_kms_generation_activations(env.pool.clone(), s3.clone()).await?;

    let stored: i64 = sqlx::query(
        "SELECT gateway_cutover_block FROM compressed_key_cutover WHERE key_id = $1",
    )
    .bind(&env.key_id)
    .fetch_one(&env.pool)
    .await?
    .try_get("gateway_cutover_block")?;
    assert_eq!(stored, 500, "the stored schedule must never change");

    let status: String = sqlx::query(
        "SELECT status FROM compressed_key_cutover_events WHERE block_hash = $1",
    )
    .bind(&second)
    .fetch_one(&env.pool)
    .await?
    .try_get("status")?;
    assert_eq!(status, "error");
    Ok(())
}

/// Migration events on orphaned blocks are cancelled like activations.
#[tokio::test]
#[serial(db)]
async fn orphaned_migration_events_are_cancelled() -> anyhow::Result<()> {
    let env = setup().await?;
    let s3 = s3_with_material(&env.key_id);

    let orphaned = vec![0x0E_u8; 32];
    insert_block(&env.pool, &orphaned, 140, "orphaned").await?;
    insert_material_event(&env.pool, &env.key_id, &orphaned, 140).await?;
    insert_cutover_event(&env.pool, &env.key_id, &orphaned, 140, 500).await?;

    process_kms_generation_activations(env.pool.clone(), s3).await?;

    for table in [
        "compressed_key_material_events",
        "compressed_key_cutover_events",
    ] {
        let status: String = sqlx::query(&format!(
            "SELECT status FROM {table} WHERE block_hash = $1"
        ))
        .bind(&orphaned)
        .fetch_one(&env.pool)
        .await?
        .try_get("status")?;
        assert_eq!(status, "cancelled", "{table} event must be cancelled");
    }
    let cutover_rows: i64 =
        sqlx::query("SELECT COUNT(*) FROM compressed_key_cutover")
            .fetch_one(&env.pool)
            .await?
            .try_get(0)?;
    assert_eq!(cutover_rows, 0);
    Ok(())
}

/// A staged material event whose key has no keys row must stay 'ready'
/// (retryable), never be consumed as 'applied' with nothing updated.
#[tokio::test]
#[serial(db)]
async fn material_for_unknown_key_is_not_consumed() -> anyhow::Result<()> {
    let env = setup().await?;
    let unknown_key = vec![0x77u8; 32];
    let s3 = s3_with_material(&unknown_key);

    let block_a = vec![0x1A_u8; 32];
    insert_block(&env.pool, &block_a, 150, "finalized").await?;
    insert_material_event(&env.pool, &unknown_key, &block_a, 150).await?;
    let block_b = vec![0x1B_u8; 32];
    insert_block(&env.pool, &block_b, 151, "finalized").await?;
    insert_cutover_event(&env.pool, &unknown_key, &block_b, 151, 500).await?;

    // Several passes: download+stage, apply schedule, attempt material apply.
    for _ in 0..3 {
        process_kms_generation_activations(env.pool.clone(), s3.clone())
            .await?;
    }

    let status: String = sqlx::query(
        "SELECT status FROM compressed_key_material_events WHERE key_id = $1",
    )
    .bind(&unknown_key)
    .fetch_one(&env.pool)
    .await?
    .try_get("status")?;
    assert_eq!(
        status, "ready",
        "material without a matching keys row must stay staged, not be consumed"
    );
    Ok(())
}
