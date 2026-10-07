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
    is_protocol_config_listener: false,
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
    let branch_flags: (bool, bool) = sqlx::query_as(
        "SELECT is_allowed, is_completed FROM computations_branch WHERE output_handle = $1 AND producer_block_hash = $2",
    )
    .bind(handle.to_vec())
    .bind(block.hash.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(
        branch_flags,
        (true, false),
        "branch mirror must be promoted too"
    );
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
    sqlx::query(
        "INSERT INTO ciphertexts_branch (handle, ciphertext, ciphertext_version, ciphertext_type, producer_block_hash, block_number)
         VALUES ($1, '\\x00', 0, 5, $2, $3)",
    )
    .bind(handle.to_vec())
    .bind(block.hash.to_vec())
    .bind(block.number as i64)
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
    let branch_flags: (bool, bool) = sqlx::query_as(
        "SELECT is_allowed, is_completed FROM computations_branch WHERE output_handle = $1 AND producer_block_hash = $2",
    )
    .bind(handle.to_vec())
    .bind(block.hash.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(branch_flags, (false, true));
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

async fn replay_with_different_chain(cleanup: bool) -> anyhow::Result<()> {
    let (mut db, pool, instance) = setup().await?;
    let parent = block();
    let parent_tx = FixedBytes::<32>::with_last_byte(0xa0);
    let child_tx = FixedBytes::<32>::with_last_byte(0xa1);
    let input = FixedBytes::<32>::with_last_byte(0x10);
    let output = FixedBytes::<32>::with_last_byte(0x11);
    ingest(
        &mut db,
        &parent,
        vec![
            rpc_log(&trivial_encrypt(input), &parent, parent_tx, 0),
            rpc_log(&allowed(input), &parent, parent_tx, 1),
        ],
        false,
    )
    .await?;
    let child = BlockSummary {
        number: 101,
        hash: FixedBytes::with_last_byte(0xb2),
        parent_hash: parent.hash,
        timestamp: parent.timestamp + 1,
    };
    let op = TfheContract::FheAdd {
        caller: Address::ZERO,
        lhs: input,
        rhs: input,
        scalarByte: FixedBytes::ZERO,
        result: output,
    };
    ingest(
        &mut db,
        &child,
        vec![rpc_log(&op, &child, child_tx, 0)],
        false,
    )
    .await?;
    let original: Vec<u8> = sqlx::query_scalar(
        "SELECT dependence_chain_id FROM computations WHERE output_handle = $1",
    )
    .bind(output.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(original, parent_tx.to_vec());
    sqlx::query(
        "UPDATE computations SET is_completed = TRUE WHERE output_handle = $1",
    )
    .bind(input.to_vec())
    .execute(&pool)
    .await?;
    assert_eq!(computation_flags(&pool, output).await?, (false, true));
    sqlx::query("UPDATE dependence_chain SET status = 'processed'")
        .execute(&pool)
        .await?;
    // Simulate normal worker cleanup of an old processed chain.
    if cleanup {
        sqlx::query(
            "DELETE FROM dependence_chain WHERE dependence_chain_id = $1",
        )
        .bind(original)
        .execute(&pool)
        .await?;
    }
    // A fresh catch-up process no longer has the original in-memory chain cache.
    let mut replay_db =
        Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 128)
            .await?;
    ingest(
        &mut replay_db,
        &child,
        vec![
            rpc_log(&op, &child, child_tx, 0),
            rpc_log(&allowed(output), &child, child_tx, 1),
        ],
        true,
    )
    .await?;
    assert_eq!(computation_flags(&pool, output).await?, (true, false));
    let persisted_chain: Vec<u8> = sqlx::query_scalar(
        "SELECT dependence_chain_id FROM computations WHERE output_handle = $1",
    )
    .bind(output.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(persisted_chain, parent_tx.to_vec());
    assert_eq!(chain_status(&pool, parent_tx).await?, "updated");
    // The cold replay inferred the child's transaction as a separate chain.
    assert_eq!(chain_status(&pool, child_tx).await?, "updated");
    let reachable: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM computations c JOIN dependence_chain dc USING (dependence_chain_id)
         WHERE c.output_handle = $1 AND c.is_allowed AND NOT c.is_completed
         AND NOT c.is_error AND dc.status = 'updated'
         AND dc.dependency_count = 0 AND dc.worker_id IS NULL"
    ).bind(output.to_vec()).fetch_one(&pool).await?;
    assert_eq!(reachable, 1, "upgraded work must have a schedulable chain");
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn computation_upgrade_requeues_original_chain_with_different_replay_chain(
) -> anyhow::Result<()> {
    replay_with_different_chain(false).await
}

#[tokio::test]
#[serial(db)]
async fn computation_upgrade_recreates_cleaned_up_original_chain(
) -> anyhow::Result<()> {
    replay_with_different_chain(true).await
}
