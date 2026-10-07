//! A computation first ingested without its ACL allow event (e.g. a partial
//! get_logs while recovering from a reorg) must become schedulable once the
//! same block is re-ingested with the allow event.

use alloy::primitives::{Address, FixedBytes, U256};
use alloy::rpc::types::Log;
use alloy::sol_types::SolEvent;
use fhevm_engine_common::chain_id::ChainId;
use host_listener::cmd::block_history::BlockSummary;
use host_listener::contracts::{AclContract, TfheContract};
use host_listener::database::ingest::{
    ingest_block_logs, BlockLogs, IngestOptions,
};
use host_listener::database::tfhe_event_propagate::Database;
use serial_test::serial;
use sqlx::postgres::PgPoolOptions;
use test_harness::instance::ImportMode;

const OPTIONS: IngestOptions = IngestOptions {
    dependence_by_connexity: false,
    dependence_cross_block: true,
    dependent_ops_max_per_chain: 0,
};

fn rpc_log<E: SolEvent>(
    event: &E,
    block: &BlockSummary,
    tx_hash: FixedBytes<32>,
    log_index: u64,
) -> Log {
    Log {
        inner: alloy::primitives::Log {
            address: Address::ZERO,
            data: event.encode_log_data(),
        },
        block_hash: Some(block.hash),
        block_number: Some(block.number),
        block_timestamp: None,
        transaction_hash: Some(tx_hash),
        transaction_index: Some(0),
        log_index: Some(log_index),
        removed: false,
    }
}

fn trivial_encrypt(result: FixedBytes<32>) -> TfheContract::TrivialEncrypt {
    TfheContract::TrivialEncrypt {
        caller: Address::ZERO,
        pt: U256::from(7),
        toType: 5,
        result,
    }
}

fn allowed(handle: FixedBytes<32>) -> AclContract::Allowed {
    AclContract::Allowed {
        caller: Address::ZERO,
        account: Address::with_last_byte(1),
        handle,
    }
}

async fn ingest(
    db: &mut Database,
    block: &BlockSummary,
    logs: Vec<Log>,
    catchup: bool,
) -> anyhow::Result<()> {
    let block_logs = BlockLogs {
        logs,
        summary: *block,
        catchup,
        finalized: false,
    };
    ingest_block_logs(
        ChainId::try_from(42_u64)?,
        db,
        &block_logs,
        &None,
        &None,
        &None,
        OPTIONS,
    )
    .await?;
    Ok(())
}

async fn computation_flags(
    pool: &sqlx::PgPool,
    handle: FixedBytes<32>,
) -> anyhow::Result<(bool, bool)> {
    let row: (bool, bool) = sqlx::query_as(
        "SELECT is_allowed, is_completed FROM computations WHERE output_handle = $1",
    )
    .bind(handle.to_vec())
    .fetch_one(pool)
    .await?;
    Ok(row)
}

async fn chain_status(
    pool: &sqlx::PgPool,
    dcid: FixedBytes<32>,
) -> anyhow::Result<String> {
    Ok(sqlx::query_scalar(
        "SELECT status FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(dcid.to_vec())
    .fetch_one(pool)
    .await?)
}

/// Rows the tfhe-worker would pick up for this dependence chain.
async fn schedulable(
    pool: &sqlx::PgPool,
    dcid: FixedBytes<32>,
) -> anyhow::Result<i64> {
    Ok(sqlx::query_scalar(
        "SELECT count(*) FROM computations
         WHERE is_completed = FALSE AND is_error = FALSE AND is_allowed = TRUE
           AND dependence_chain_id = $1",
    )
    .bind(dcid.to_vec())
    .fetch_one(pool)
    .await?)
}

async fn setup(
) -> anyhow::Result<(Database, sqlx::PgPool, test_harness::instance::DBInstance)>
{
    let db_instance = test_harness::instance::setup_test_db(ImportMode::None)
        .await
        .map_err(|e| anyhow::anyhow!("{e}"))?;
    let db =
        Database::new(&db_instance.db_url, ChainId::try_from(42_u64)?, 128)
            .await?;
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(db_instance.db_url())
        .await?;
    Ok((db, pool, db_instance))
}

fn block() -> BlockSummary {
    BlockSummary {
        number: 100,
        hash: FixedBytes::with_last_byte(0xb1),
        parent_hash: FixedBytes::with_last_byte(0xb0),
        timestamp: 1_790_000_000,
    }
}

#[tokio::test]
#[serial(db)]
async fn computation_upgraded_when_allow_event_arrives_later(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    let block = block();
    let tx_hash = FixedBytes::<32>::with_last_byte(0x9f);
    let handle = FixedBytes::<32>::with_last_byte(0x01);

    // First ingestion misses the ACL allow event.
    ingest(
        &mut db,
        &block,
        vec![rpc_log(&trivial_encrypt(handle), &block, tx_hash, 0)],
        false,
    )
    .await?;
    assert_eq!(computation_flags(&pool, handle).await?, (false, true));
    // The worker finds no work and marks the chain processed.
    sqlx::query("UPDATE dependence_chain SET status = 'processed'")
        .execute(&pool)
        .await?;
    assert_eq!(schedulable(&pool, tx_hash).await?, 0);

    // Re-ingestion of the same block, now with the allow event.
    ingest(
        &mut db,
        &block,
        vec![
            rpc_log(&trivial_encrypt(handle), &block, tx_hash, 0),
            rpc_log(&allowed(handle), &block, tx_hash, 1),
        ],
        true,
    )
    .await?;
    assert_eq!(computation_flags(&pool, handle).await?, (true, false));
    assert_eq!(chain_status(&pool, tx_hash).await?, "updated");
    assert_eq!(schedulable(&pool, tx_hash).await?, 1);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn computation_not_upgraded_when_ciphertext_exists() -> anyhow::Result<()>
{
    let (mut db, pool, _instance) = setup().await?;
    let block = block();
    let tx_hash = FixedBytes::<32>::with_last_byte(0x9e);
    let handle = FixedBytes::<32>::with_last_byte(0x02);

    ingest(
        &mut db,
        &block,
        vec![rpc_log(&trivial_encrypt(handle), &block, tx_hash, 0)],
        false,
    )
    .await?;
    sqlx::query(
        "INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type)
         VALUES ($1, '\\x00', 0, 5)",
    )
    .bind(handle.to_vec())
    .execute(&pool)
    .await?;

    ingest(
        &mut db,
        &block,
        vec![
            rpc_log(&trivial_encrypt(handle), &block, tx_hash, 0),
            rpc_log(&allowed(handle), &block, tx_hash, 1),
        ],
        true,
    )
    .await?;
    assert_eq!(computation_flags(&pool, handle).await?, (false, true));
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn allowed_first_insert_unchanged() -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    let block = block();
    let tx_hash = FixedBytes::<32>::with_last_byte(0x9d);
    let handle = FixedBytes::<32>::with_last_byte(0x03);
    let logs = || {
        vec![
            rpc_log(&trivial_encrypt(handle), &block, tx_hash, 0),
            rpc_log(&allowed(handle), &block, tx_hash, 1),
        ]
    };

    ingest(&mut db, &block, logs(), false).await?;
    assert_eq!(computation_flags(&pool, handle).await?, (true, false));
    // A duplicate ingestion is still a no-op.
    ingest(&mut db, &block, logs(), true).await?;
    assert_eq!(computation_flags(&pool, handle).await?, (true, false));
    assert_eq!(schedulable(&pool, tx_hash).await?, 1);
    Ok(())
}
