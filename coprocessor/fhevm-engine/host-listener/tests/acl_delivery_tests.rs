//! Separate deliveries of ACL and TFHE events must converge to runnable work.

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
    ingest_with_options(db, block, logs, catchup, OPTIONS).await
}

async fn ingest_with_options(
    db: &mut Database,
    block: &BlockSummary,
    logs: Vec<Log>,
    catchup: bool,
    options: IngestOptions,
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
        options,
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
async fn acl_only_delivery_upgrades_existing_computation() -> anyhow::Result<()>
{
    let (mut db, pool, _instance) = setup().await?;
    let b = block();
    let tx_hash = FixedBytes::<32>::with_last_byte(0xe1);
    let handle = FixedBytes::<32>::with_last_byte(0x71);
    ingest(
        &mut db,
        &b,
        vec![rpc_log(&trivial_encrypt(handle), &b, tx_hash, 0)],
        false,
    )
    .await?;
    ingest(
        &mut db,
        &b,
        vec![rpc_log(&allowed(handle), &b, tx_hash, 1)],
        true,
    )
    .await?;
    assert_eq!(computation_flags(&pool, handle).await?, (true, false));
    assert_eq!(schedulable(&pool, tx_hash).await?, 1);
    assert_eq!(chain_status(&pool, tx_hash).await?, "updated");
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn tfhe_only_delivery_uses_persisted_acl() -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    let b = block();
    let tx_hash = FixedBytes::<32>::with_last_byte(0xe2);
    let handle = FixedBytes::<32>::with_last_byte(0x72);
    ingest(
        &mut db,
        &b,
        vec![rpc_log(&allowed(handle), &b, tx_hash, 1)],
        false,
    )
    .await?;
    ingest(
        &mut db,
        &b,
        vec![rpc_log(&trivial_encrypt(handle), &b, tx_hash, 0)],
        true,
    )
    .await?;
    assert_eq!(computation_flags(&pool, handle).await?, (true, false));
    assert_eq!(schedulable(&pool, tx_hash).await?, 1);
    assert_eq!(chain_status(&pool, tx_hash).await?, "updated");
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn complete_and_incomplete_concurrent_ingestion_converge(
) -> anyhow::Result<()> {
    let (mut db, pool, instance) = setup().await?;
    let mut db2 =
        Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 128)
            .await?;
    let b = block();
    for byte in 1..=12 {
        let tx_hash = FixedBytes::<32>::with_last_byte(byte);
        let handle = FixedBytes::<32>::with_last_byte(byte + 32);
        let (a, c) = tokio::join!(
            ingest(
                &mut db,
                &b,
                vec![rpc_log(&trivial_encrypt(handle), &b, tx_hash, 0)],
                false
            ),
            ingest(
                &mut db2,
                &b,
                vec![
                    rpc_log(&trivial_encrypt(handle), &b, tx_hash, 0),
                    rpc_log(&allowed(handle), &b, tx_hash, 1)
                ],
                true
            )
        );
        a?;
        c?;
        assert_eq!(computation_flags(&pool, handle).await?, (true, false));
        assert_eq!(chain_status(&pool, tx_hash).await?, "updated");
        // A subsequent incomplete replay must not downgrade the stored row.
        ingest(
            &mut db,
            &b,
            vec![rpc_log(&trivial_encrypt(handle), &b, tx_hash, 0)],
            true,
        )
        .await?;
        assert_eq!(computation_flags(&pool, handle).await?, (true, false));
    }
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn acl_in_later_transaction_recreates_original_chain(
) -> anyhow::Result<()> {
    let (mut db, pool, instance) = setup().await?;
    let producer = block();
    let tx_hash = FixedBytes::<32>::with_last_byte(0xe3);
    let handle = FixedBytes::<32>::with_last_byte(0x73);
    ingest(
        &mut db,
        &producer,
        vec![rpc_log(&trivial_encrypt(handle), &producer, tx_hash, 0)],
        false,
    )
    .await?;
    sqlx::query("DELETE FROM dependence_chain WHERE dependence_chain_id = $1")
        .bind(tx_hash.to_vec())
        .execute(&pool)
        .await?;
    let acl_block = BlockSummary {
        number: producer.number + 1,
        hash: FixedBytes::with_last_byte(0xb2),
        parent_hash: producer.hash,
        timestamp: producer.timestamp + 12,
    };
    // A fresh listener receives permission in a different transaction/block.
    let mut fresh =
        Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 128)
            .await?;
    ingest(
        &mut fresh,
        &acl_block,
        vec![rpc_log(
            &allowed(handle),
            &acl_block,
            FixedBytes::with_last_byte(0xf3),
            0,
        )],
        false,
    )
    .await?;
    assert_eq!(computation_flags(&pool, handle).await?, (true, false));
    assert_eq!(chain_status(&pool, tx_hash).await?, "updated");
    assert_eq!(schedulable(&pool, tx_hash).await?, 1);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn duplicate_acl_only_delivery_recreates_pending_chain(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    let b = block();
    let tx_hash = FixedBytes::<32>::with_last_byte(0xe4);
    let handle = FixedBytes::<32>::with_last_byte(0x74);
    let acl = rpc_log(&allowed(handle), &b, tx_hash, 1);
    ingest(
        &mut db,
        &b,
        vec![
            rpc_log(&trivial_encrypt(handle), &b, tx_hash, 0),
            acl.clone(),
        ],
        false,
    )
    .await?;
    sqlx::query("DELETE FROM dependence_chain WHERE dependence_chain_id = $1")
        .bind(tx_hash.to_vec())
        .execute(&pool)
        .await?;
    ingest(&mut db, &b, vec![acl], true).await?;
    assert_eq!(computation_flags(&pool, handle).await?, (true, false));
    assert_eq!(chain_status(&pool, tx_hash).await?, "updated");
    assert_eq!(schedulable(&pool, tx_hash).await?, 1);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn public_decryption_acl_upgrades_every_listed_handle(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    let b = block();
    let tx_hash = FixedBytes::<32>::with_last_byte(0xe5);
    let handles = [
        FixedBytes::<32>::with_last_byte(0x75),
        FixedBytes::<32>::with_last_byte(0x76),
    ];
    let logs = handles
        .iter()
        .enumerate()
        .map(|(index, handle)| {
            rpc_log(&trivial_encrypt(*handle), &b, tx_hash, index as u64)
        })
        .collect();
    ingest(&mut db, &b, logs, false).await?;
    sqlx::query("UPDATE dependence_chain SET status = 'processed' WHERE dependence_chain_id = $1")
        .bind(tx_hash.to_vec()).execute(&pool).await?;
    let acl = AclContract::AllowedForDecryption {
        caller: Address::ZERO,
        handlesList: handles.to_vec(),
    };
    ingest(&mut db, &b, vec![rpc_log(&acl, &b, tx_hash, 2)], true).await?;
    for handle in handles {
        assert_eq!(computation_flags(&pool, handle).await?, (true, false));
    }
    assert_eq!(schedulable(&pool, tx_hash).await?, 2);
    assert_eq!(chain_status(&pool, tx_hash).await?, "updated");
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn persisted_acl_is_scoped_to_the_host_chain() -> anyhow::Result<()> {
    let (mut db, pool, instance) = setup().await?;
    let b = block();
    let tx_hash = FixedBytes::<32>::with_last_byte(0xe6);
    let handle = FixedBytes::<32>::with_last_byte(0x77);
    ingest(
        &mut db,
        &b,
        vec![rpc_log(&allowed(handle), &b, tx_hash, 1)],
        false,
    )
    .await?;
    let other_chain = ChainId::try_from(43_u64)?;
    let mut other_db =
        Database::new(&instance.db_url, other_chain, 128).await?;
    let block_logs = BlockLogs {
        logs: vec![rpc_log(&trivial_encrypt(handle), &b, tx_hash, 0)],
        summary: b,
        catchup: true,
        finalized: false,
    };
    ingest_block_logs(
        other_chain,
        &mut other_db,
        &block_logs,
        &None,
        &None,
        &None,
        OPTIONS,
    )
    .await?;
    assert_eq!(computation_flags(&pool, handle).await?, (false, true));
    assert_eq!(schedulable(&pool, tx_hash).await?, 0);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn concurrent_separate_acl_and_computation_deliveries_converge(
) -> anyhow::Result<()> {
    let (mut db, pool, instance) = setup().await?;
    let mut acl_db =
        Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 128)
            .await?;
    for byte in 1..=12 {
        let mut b = block();
        b.number += u64::from(byte);
        b.hash = FixedBytes::with_last_byte(byte);
        let tx_hash = FixedBytes::<32>::with_last_byte(byte);
        let handle = FixedBytes::<32>::with_last_byte(byte + 32);
        let (computation, acl) = tokio::join!(
            ingest(
                &mut db,
                &b,
                vec![rpc_log(&trivial_encrypt(handle), &b, tx_hash, 0)],
                false
            ),
            ingest(
                &mut acl_db,
                &b,
                vec![rpc_log(&allowed(handle), &b, tx_hash, 1)],
                true
            )
        );
        computation?;
        acl?;
        assert_eq!(computation_flags(&pool, handle).await?, (true, false));
        assert_eq!(chain_status(&pool, tx_hash).await?, "updated");
    }
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn reversed_duplicate_delivery_order_does_not_deadlock(
) -> anyhow::Result<()> {
    let (mut first, pool, instance) = setup().await?;
    let mut second =
        Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 128)
            .await?;
    let b = block();
    let mut logs = Vec::new();
    for byte in 1..=8 {
        let tx_hash = FixedBytes::<32>::with_last_byte(byte);
        let handle = FixedBytes::<32>::with_last_byte(byte + 32);
        logs.push(rpc_log(
            &trivial_encrypt(handle),
            &b,
            tx_hash,
            u64::from(byte) * 2,
        ));
        logs.push(rpc_log(
            &allowed(handle),
            &b,
            tx_hash,
            u64::from(byte) * 2 + 1,
        ));
    }
    let mut reversed = logs.clone();
    reversed.reverse();
    let (left, right) =
        tokio::time::timeout(std::time::Duration::from_secs(30), async {
            tokio::join!(
                ingest(&mut first, &b, logs, false),
                ingest(&mut second, &b, reversed, true)
            )
        })
        .await?;
    left?;
    right?;
    for byte in 1..=8 {
        assert_eq!(
            computation_flags(&pool, FixedBytes::with_last_byte(byte + 32))
                .await?,
            (true, false)
        );
    }
    Ok(())
}

async fn upgrade_only_replay_marks_original_chain_slow(
    acl_only: bool,
) -> anyhow::Result<()> {
    let (mut db, pool, instance) = setup().await?;
    let parent = block();
    let parent_tx = FixedBytes::<32>::with_last_byte(0xa0);
    let input = FixedBytes::<32>::with_last_byte(0x10);
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
        number: parent.number + 1,
        hash: FixedBytes::with_last_byte(0xb2),
        parent_hash: parent.hash,
        timestamp: parent.timestamp + 12,
    };
    let child_tx = FixedBytes::<32>::with_last_byte(0xa1);
    let outputs = [
        FixedBytes::<32>::with_last_byte(0x20),
        FixedBytes::<32>::with_last_byte(0x21),
        FixedBytes::<32>::with_last_byte(0x22),
    ];
    let mut operations = Vec::new();
    let mut permissions = Vec::new();
    for (index, output) in outputs.iter().enumerate() {
        let operation = TfheContract::FheAdd {
            caller: Address::ZERO,
            lhs: input,
            rhs: input,
            scalarByte: FixedBytes::ZERO,
            result: *output,
        };
        operations.push(rpc_log(
            &operation,
            &child,
            child_tx,
            index as u64 * 2,
        ));
        permissions.push(rpc_log(
            &allowed(*output),
            &child,
            child_tx,
            index as u64 * 2 + 1,
        ));
    }
    // The original warm listener extends the parent chain. All child rows exist
    // but were first ingested without permissions; the lane starts fast.
    ingest(&mut db, &child, operations.clone(), false).await?;
    let priority: i16 = sqlx::query_scalar("SELECT schedule_priority FROM dependence_chain WHERE dependence_chain_id = $1")
        .bind(parent_tx.to_vec()).fetch_one(&pool).await?;
    assert_eq!(priority, 0);
    let stored: Vec<Vec<u8>> = sqlx::query_scalar("SELECT DISTINCT dependence_chain_id FROM computations WHERE transaction_id = $1")
        .bind(child_tx.to_vec()).fetch_all(&pool).await?;
    assert_eq!(stored, vec![parent_tx.to_vec()]);
    let mut replay =
        Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 128)
            .await?;
    let mut logs = if acl_only { Vec::new() } else { operations };
    logs.extend(permissions);
    let options = IngestOptions {
        dependent_ops_max_per_chain: 2,
        ..OPTIONS
    };
    // With TFHE logs a cold replay infers child_tx as the chain; with ACL-only
    // logs it has no newly inferred chain. Both must account for the actual
    // three upgraded rows on the original persisted parent chain.
    ingest_with_options(&mut replay, &child, logs, true, options).await?;
    for output in outputs {
        assert_eq!(computation_flags(&pool, output).await?, (true, false));
    }
    let priority: i16 = sqlx::query_scalar("SELECT schedule_priority FROM dependence_chain WHERE dependence_chain_id = $1")
        .bind(parent_tx.to_vec()).fetch_one(&pool).await?;
    assert_eq!(priority, 1, "upgrade-only work must count against the original chain's slow-lane threshold");
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn cold_upgrade_only_replay_marks_original_chain_slow(
) -> anyhow::Result<()> {
    upgrade_only_replay_marks_original_chain_slow(false).await
}

#[tokio::test]
#[serial(db)]
async fn acl_only_upgrade_marks_original_chain_slow() -> anyhow::Result<()> {
    upgrade_only_replay_marks_original_chain_slow(true).await
}

#[tokio::test]
#[serial(db)]
async fn large_ingestion_batches_bound_locks_and_serialize_overlap(
) -> anyhow::Result<()> {
    let (db, observer, _instance) = setup().await?;
    let handles = (0_u32..16_384)
        .map(|index| {
            let mut handle = vec![0_u8; 32];
            handle[28..].copy_from_slice(&index.to_be_bytes());
            handle
        })
        .collect::<Vec<_>>();
    let mut first = db.new_transaction().await?;
    let first_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *first)
        .await?;
    // Two large calls in the same transaction must retain a bounded lock set,
    // rather than accumulating one PostgreSQL shared-memory lock per handle.
    for batch in handles.chunks(8_192) {
        db.lock_ingestion_handles(&mut first, batch).await?;
        let locks: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM pg_locks WHERE locktype = 'advisory' AND pid = pg_backend_pid()")
            .fetch_one(&mut *first).await?;
        assert!(
            (1..=128).contains(&locks),
            "large batch held {locks} advisory locks"
        );
    }

    let overlapping = vec![handles[8_193].clone()];
    let second_db = db.clone();
    let (pid_sender, pid_receiver) = tokio::sync::oneshot::channel();
    let waiter = tokio::spawn(async move {
        let mut second = second_db.new_transaction().await?;
        let pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
            .fetch_one(&mut *second)
            .await?;
        pid_sender
            .send(pid)
            .map_err(|_| anyhow::anyhow!("lock observer dropped"))?;
        second_db
            .lock_ingestion_handles(&mut second, &overlapping)
            .await?;
        second.commit().await?;
        Ok::<(), anyhow::Error>(())
    });
    let second_pid = pid_receiver.await?;
    // Observe the server's lock wait, not a timing assumption about task
    // scheduling. The overlapping delivery cannot finish while first owns it.
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            let blocked: bool = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM pg_locks WHERE locktype = 'advisory' AND pid = $1 AND NOT granted)")
                .bind(second_pid).fetch_one(&observer).await?;
            if blocked {
                return Ok::<(), anyhow::Error>(());
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    }).await??;
    assert!(!waiter.is_finished());
    first.commit().await?;
    tokio::time::timeout(std::time::Duration::from_secs(5), waiter).await???;
    let remaining: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM pg_locks WHERE locktype = 'advisory' AND pid IN ($1, $2)")
        .bind(first_pid).bind(second_pid).fetch_one(&observer).await?;
    assert_eq!(
        remaining, 0,
        "transaction completion must release every advisory lock"
    );
    Ok(())
}
