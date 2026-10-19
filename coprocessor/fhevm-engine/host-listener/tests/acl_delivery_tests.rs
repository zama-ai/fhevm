//! Separate deliveries of ACL and TFHE events must converge to runnable work.

mod ingest_fixtures;
use ingest_fixtures::*;

use alloy::primitives::{Address, FixedBytes};
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::chain_recovery::{lock_history_tx, ListenerSource};
use host_listener::cmd::block_history::BlockSummary;
use host_listener::contracts::{AclContract, TfheContract};
use host_listener::database::ingest::{
    ingest_block_logs, BlockLogs, IngestOptions,
};
use host_listener::database::tfhe_event_propagate::Database;
use serial_test::serial;

#[tokio::test]
#[serial(db)]
async fn acl_only_delivery_upgrades_existing_computation() -> anyhow::Result<()>
{
    let (mut db, pool, _instance) = setup().await?;
    let b = block();
    let tx_hash = computed_handle(0xe1);
    let handle = computed_handle(0x71);
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
    let producer_hash: Vec<u8> = sqlx::query_scalar(
        "SELECT producer_block_hash FROM handle_producer_block WHERE handle = $1",
    ).bind(handle.as_slice()).fetch_one(&pool).await?;
    assert_eq!(producer_hash, b.hash.to_vec());

    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn tfhe_only_delivery_uses_persisted_acl() -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    let b = block();
    let tx_hash = computed_handle(0xe2);
    let handle = computed_handle(0x72);
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
    let producer_hash: Vec<u8> = sqlx::query_scalar(
        "SELECT producer_block_hash FROM handle_producer_block WHERE handle = $1",
    ).bind(handle.as_slice()).fetch_one(&pool).await?;
    assert_eq!(producer_hash, b.hash.to_vec());
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
        let tx_hash = computed_handle(byte);
        let handle = computed_handle(byte + 32);
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
    let tx_hash = computed_handle(0xe3);
    let handle = computed_handle(0x73);
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
        hash: computed_handle(0xb2),
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
            computed_handle(0xf3),
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
    let tx_hash = computed_handle(0xe4);
    let handle = computed_handle(0x74);
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
    let tx_hash = computed_handle(0xe5);
    let handles = [computed_handle(0x75), computed_handle(0x76)];
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
    let (mut db, pool, _instance) = setup().await?;
    let b = block();
    let tx_hash = computed_handle(0xe6);
    let handle = computed_handle(0x77);
    ingest(
        &mut db,
        &b,
        vec![rpc_log(&allowed(handle), &b, tx_hash, 1)],
        false,
    )
    .await?;
    // Seed legacy ACL evidence tagged with a different host chain. Modern
    // event validation itself rejects emitting this handle on that chain.
    sqlx::query(
        "UPDATE allowed_handles SET host_chain_id = 43 WHERE handle = $1",
    )
    .bind(handle.as_slice())
    .execute(&pool)
    .await?;
    ingest(
        &mut db,
        &b,
        vec![rpc_log(&trivial_encrypt(handle), &b, tx_hash, 0)],
        true,
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
        b.hash = computed_handle(byte);
        let tx_hash = computed_handle(byte);
        let handle = computed_handle(byte + 32);
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
        let tx_hash = computed_handle(byte);
        let handle = computed_handle(byte + 32);
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
            computation_flags(&pool, computed_handle(byte + 32)).await?,
            (true, false)
        );
    }
    Ok(())
}

async fn upgrade_only_replay_marks_original_chain_slow(
    acl_only: bool,
    limit: u32,
) -> anyhow::Result<()> {
    let (mut db, pool, instance) = setup().await?;
    let parent = block();
    let parent_tx = computed_handle(0xa0);
    let input = computed_handle(0x10);
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
        hash: computed_handle(0xb2),
        parent_hash: parent.hash,
        timestamp: parent.timestamp + 12,
    };
    let child_tx = computed_handle(0xa1);
    let outputs = [
        computed_handle(0x20),
        computed_handle(0x21),
        computed_handle(0x22),
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
        dependent_ops_max_per_chain: limit,
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
    assert_eq!(priority, i16::from(limit < 3), "upgrade-only work must count against the original chain's slow-lane threshold");
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn cold_upgrade_only_replay_marks_original_chain_slow(
) -> anyhow::Result<()> {
    upgrade_only_replay_marks_original_chain_slow(false, 2).await
}

#[tokio::test]
#[serial(db)]
async fn acl_only_upgrade_marks_original_chain_slow() -> anyhow::Result<()> {
    upgrade_only_replay_marks_original_chain_slow(true, 2).await
}

#[tokio::test]
#[serial(db)]
async fn history_fence_bounds_locks_and_serializes_deliveries(
) -> anyhow::Result<()> {
    let (db, observer, _instance) = setup().await?;
    let mut first = db.new_transaction().await?.expect("active stack");
    let first_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *first)
        .await?;
    // The shared cutover guard is acquired before the history fence.
    let initial_locks: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM pg_locks WHERE locktype = 'advisory' AND pid = pg_backend_pid()")
        .fetch_one(&mut *first).await?;
    // Lock use is independent of the number of handles or blocks admitted.
    for _ in 0..2 {
        lock_history_tx(&mut first, ListenerSource::Host, false, 42).await?;
        let locks: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM pg_locks WHERE locktype = 'advisory' AND pid = pg_backend_pid()")
            .fetch_one(&mut *first).await?;
        assert_eq!(locks - initial_locks, 1);
    }

    let second_db = db.clone();
    let (pid_sender, pid_receiver) = tokio::sync::oneshot::channel();
    let waiter = tokio::spawn(async move {
        let mut second =
            second_db.new_transaction().await?.expect("active stack");
        let pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
            .fetch_one(&mut *second)
            .await?;
        pid_sender
            .send(pid)
            .map_err(|_| anyhow::anyhow!("lock observer dropped"))?;
        lock_history_tx(&mut second, ListenerSource::Host, false, 42).await?;
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

#[tokio::test]
#[serial(db)]
async fn bridge_replay_preserves_transient_materialization_permission(
) -> anyhow::Result<()> {
    use host_listener::contracts::BridgeContract;
    let (mut db, pool, _instance) = setup().await?;
    let b = block();
    let tx_hash = computed_handle(0xe7);
    let handle = computed_handle(0x78);
    let producer = rpc_log(&trivial_encrypt(handle), &b, tx_hash, 0);
    ingest(&mut db, &b, vec![producer.clone()], false).await?;
    assert_eq!(computation_flags(&pool, handle).await?, (false, true));
    let bridge = BridgeContract::BridgeHandle {
        senderDapp: Address::with_last_byte(1),
        srcHandle: handle,
        dstChainId: 43,
        guid: FixedBytes::with_last_byte(2),
    };
    ingest_block_logs(
        ChainId::try_from(42_u64)?,
        &mut db,
        &BlockLogs {
            logs: vec![producer, rpc_log(&bridge, &b, tx_hash, 1)],
            summary: b,
            catchup: true,
            finalized: false,
        },
        &None,
        &None,
        &None,
        &None,
        &Some(Address::ZERO),
        OPTIONS,
    )
    .await?;
    assert_eq!(computation_flags(&pool, handle).await?, (true, false));
    assert_eq!(schedulable(&pool, tx_hash).await?, 1);
    let acl_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM allowed_handles")
            .fetch_one(&pool)
            .await?;
    assert_eq!(
        acl_count, 0,
        "bridge permission does not invent persistent ACL events"
    );
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn producer_inventory_repair_does_not_double_count_slow_lane_work(
) -> anyhow::Result<()> {
    upgrade_only_replay_marks_original_chain_slow(false, 3).await
}
