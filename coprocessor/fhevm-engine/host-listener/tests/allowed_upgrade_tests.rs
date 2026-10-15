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

// ---------------------------------------------------------------------------
// Pending computations whose chain became unreachable (re-arm on re-ingestion)
// ---------------------------------------------------------------------------

async fn set_chain_status(
    pool: &sqlx::PgPool,
    dcid: FixedBytes<32>,
    status: &str,
) -> anyhow::Result<()> {
    sqlx::query(
        "UPDATE dependence_chain SET status = $2 WHERE dependence_chain_id = $1",
    )
    .bind(dcid.to_vec())
    .bind(status)
    .execute(pool)
    .await?;
    Ok(())
}

/// Ingest one allowed computation, then put its chain in `prepare`'s state and
/// re-ingest the identical block. Returns the chain status afterwards (None if
/// the chain row does not exist) and the number of schedulable rows.
async fn reingest_after(
    prepare: &str,
    tx_byte: u8,
    handle_byte: u8,
) -> anyhow::Result<(Option<String>, i64, bool)> {
    let (mut db, pool, _instance) = setup().await?;
    let block = block();
    let tx_hash = FixedBytes::<32>::with_last_byte(tx_byte);
    let handle = FixedBytes::<32>::with_last_byte(handle_byte);
    let logs = || {
        vec![
            rpc_log(&trivial_encrypt(handle), &block, tx_hash, 0),
            rpc_log(&allowed(handle), &block, tx_hash, 1),
        ]
    };
    ingest(&mut db, &block, logs(), false).await?;
    assert_eq!(computation_flags(&pool, handle).await?, (true, false));
    match prepare {
        "processed" => set_chain_status(&pool, tx_hash, "processed").await?,
        "missing" => {
            sqlx::query(
                "DELETE FROM dependence_chain WHERE dependence_chain_id = $1",
            )
            .bind(tx_hash.to_vec())
            .execute(&pool)
            .await?;
        }
        "processing" => {
            sqlx::query(
                "UPDATE dependence_chain SET status = 'processing',
                   worker_id = gen_random_uuid(), lock_acquired_at = NOW(),
                   lock_expires_at = NOW() + INTERVAL '2 minutes'
                 WHERE dependence_chain_id = $1",
            )
            .bind(tx_hash.to_vec())
            .execute(&pool)
            .await?;
        }
        "computed" => {
            sqlx::query(
                "INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type)
                 VALUES ($1, '\\x00', 0, 5)",
            )
            .bind(handle.to_vec())
            .execute(&pool)
            .await?;
            sqlx::query("UPDATE computations SET is_completed = TRUE WHERE output_handle = $1")
                .bind(handle.to_vec())
                .execute(&pool)
                .await?;
            set_chain_status(&pool, tx_hash, "processed").await?;
        }
        other => anyhow::bail!("unknown preparation {other}"),
    }
    // A fresh process re-ingests the same block (e.g. catch-up from an earlier
    // block after the fix is deployed).
    let mut replay_db =
        Database::new(&_instance.db_url, ChainId::try_from(42_u64)?, 128)
            .await?;
    ingest(&mut replay_db, &block, logs(), true).await?;
    let status: Option<String> = sqlx::query_scalar(
        "SELECT status FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(tx_hash.to_vec())
    .fetch_optional(&pool)
    .await?;
    let locked: Option<bool> = sqlx::query_scalar(
        "SELECT worker_id IS NOT NULL FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(tx_hash.to_vec())
    .fetch_optional(&pool)
    .await?;
    Ok((
        status,
        schedulable(&pool, tx_hash).await?,
        locked.unwrap_or(false),
    ))
}

#[tokio::test]
#[serial(db)]
async fn pending_computation_rearms_processed_chain() -> anyhow::Result<()> {
    let (status, pending, _) = reingest_after("processed", 0xc0, 0x20).await?;
    assert_eq!(status.as_deref(), Some("updated"));
    assert_eq!(pending, 1);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn pending_computation_recreates_missing_chain() -> anyhow::Result<()> {
    let (status, pending, _) = reingest_after("missing", 0xc1, 0x21).await?;
    assert_eq!(status.as_deref(), Some("updated"));
    assert_eq!(pending, 1);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn pending_computation_leaves_processing_chain_alone(
) -> anyhow::Result<()> {
    let (status, pending, locked) =
        reingest_after("processing", 0xc2, 0x22).await?;
    // The re-arm path only touches processed or missing chains: the owning
    // worker keeps its lock (the block-level upsert may still flag the chain
    // as updated, which the worker's release preserves).
    assert!(locked, "the processing worker must keep its lock");
    assert!(status.is_some());
    assert_eq!(pending, 1);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn computed_computation_does_not_rearm() -> anyhow::Result<()> {
    let (status, pending, _) = reingest_after("computed", 0xc3, 0x23).await?;
    // The block-level upsert may mark the chain updated, but there is no
    // schedulable work: nothing is recomputed.
    assert!(status.is_some());
    assert_eq!(pending, 0);
    Ok(())
}

/// 2026-10-14 testnet (artifact): a 1-block reorg replaced block N. The worker
/// ran the producer transaction's chain for the orphan version and marked it
/// processed; the canonical version of the same transaction then arrived with
/// different (block-dependent) output handles. Those must be computed.
#[tokio::test]
#[serial(db)]
async fn canonical_version_after_orphan_chain_processed() -> anyhow::Result<()>
{
    let (mut db, pool, instance) = setup().await?;
    let orphan = block();
    let canonical = BlockSummary {
        hash: FixedBytes::with_last_byte(0xbf),
        ..orphan
    };
    let producer_tx = FixedBytes::<32>::with_last_byte(0xd0);
    let orphan_out = FixedBytes::<32>::with_last_byte(0x30);
    let canonical_out = FixedBytes::<32>::with_last_byte(0x31);

    ingest(
        &mut db,
        &orphan,
        vec![
            rpc_log(&trivial_encrypt(orphan_out), &orphan, producer_tx, 0),
            rpc_log(&allowed(orphan_out), &orphan, producer_tx, 1),
        ],
        false,
    )
    .await?;
    // The worker computes the orphan version and marks the chain processed.
    sqlx::query(
        "INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type)
         VALUES ($1, '\\x00', 0, 5)",
    )
    .bind(orphan_out.to_vec())
    .execute(&pool)
    .await?;
    sqlx::query(
        "UPDATE computations SET is_completed = TRUE WHERE output_handle = $1",
    )
    .bind(orphan_out.to_vec())
    .execute(&pool)
    .await?;
    set_chain_status(&pool, producer_tx, "processed").await?;

    // The canonical version of the block arrives.
    ingest(
        &mut db,
        &canonical,
        vec![
            rpc_log(
                &trivial_encrypt(canonical_out),
                &canonical,
                producer_tx,
                0,
            ),
            rpc_log(&allowed(canonical_out), &canonical, producer_tx, 1),
        ],
        false,
    )
    .await?;
    assert_eq!(
        computation_flags(&pool, canonical_out).await?,
        (true, false)
    );
    let reachable = |out: FixedBytes<32>| {
        let pool = pool.clone();
        async move {
            sqlx::query_scalar::<_, i64>(
                "SELECT count(*) FROM computations c JOIN dependence_chain dc USING (dependence_chain_id)
                 WHERE c.output_handle = $1 AND c.is_allowed AND NOT c.is_completed
                   AND NOT c.is_error AND dc.status = 'updated' AND dc.worker_id IS NULL",
            )
            .bind(out.to_vec())
            .fetch_one(&pool)
            .await
        }
    };
    assert_eq!(reachable(canonical_out).await?, 1);

    // A stale processed status must also be repaired when the canonical
    // block is replayed; this models recovery, not the origin of that status.
    set_chain_status(&pool, producer_tx, "processed").await?;
    assert_eq!(reachable(canonical_out).await?, 0);
    let mut replay_db =
        Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 128)
            .await?;
    ingest(
        &mut replay_db,
        &canonical,
        vec![
            rpc_log(
                &trivial_encrypt(canonical_out),
                &canonical,
                producer_tx,
                0,
            ),
            rpc_log(&allowed(canonical_out), &canonical, producer_tx, 1),
        ],
        true,
    )
    .await?;
    assert_eq!(reachable(canonical_out).await?, 1);
    Ok(())
}
