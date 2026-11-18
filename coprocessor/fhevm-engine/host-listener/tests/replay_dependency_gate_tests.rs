//! CFI-0020: ACL replay must gate the persisted consumer chain even when the
//! complete block would now coalesce it with the producer.

mod ingest_fixtures;
use ingest_fixtures::*;

use alloy::primitives::{Address, FixedBytes};
use fhevm_engine_common::chain_id::ChainId;
use host_listener::contracts::TfheContract;
use host_listener::database::tfhe_event_propagate::Database;
use serial_test::serial;

async fn replay_dependency_gate(
    materialized: bool,
    forked: bool,
    legacy_provenance: bool,
) -> anyhow::Result<()> {
    let (mut db, pool, instance) = setup().await?;
    let block = block();
    let parent = FixedBytes::<32>::with_last_byte(0xa0);
    let child = FixedBytes::<32>::with_last_byte(0xa1);
    let sibling = FixedBytes::<32>::with_last_byte(0xa2);
    let input = computed_handle(0x10);
    let output = computed_handle(0x11);
    let add = TfheContract::FheAdd {
        caller: Address::ZERO,
        lhs: input,
        rhs: input,
        scalarByte: FixedBytes::ZERO,
        result: output,
    };
    let logs = |complete| {
        let mut logs = vec![
            rpc_log(&trivial_encrypt(input), &block, parent, 0),
            rpc_log(&add, &block, child, 2),
            rpc_log(&allowed(output), &block, child, 3),
        ];
        if complete {
            logs.push(rpc_log(&allowed(input), &block, parent, 1));
        }
        if forked {
            let output = computed_handle(0x12);
            logs.extend([
                rpc_log(
                    &TfheContract::FheNeg {
                        caller: Address::ZERO,
                        ct: input,
                        result: output,
                    },
                    &block,
                    sibling,
                    4,
                ),
                rpc_log(&allowed(output), &block, sibling, 5),
            ]);
        }
        logs
    };
    ingest(&mut db, &block, logs(false), false).await?;
    let stored_chain: Vec<u8> = sqlx::query_scalar(
        "SELECT dependence_chain_id FROM computations WHERE output_handle = $1",
    )
    .bind(output.as_slice())
    .fetch_one(&pool)
    .await?;
    assert_eq!(stored_chain, child.to_vec());
    assert_eq!(computation_flags(&pool, input).await?, (false, true));

    if legacy_provenance {
        // ACL-only recovery makes the parent eligible but has no computation
        // graph to repair the child's gate. Model retained pre-migration rows
        // before the complete replay: every duplicate will backfill metadata,
        // and no new ACL upgrade will independently trigger gate repair.
        ingest(
            &mut db,
            &block,
            vec![rpc_log(&allowed(input), &block, parent, 1)],
            true,
        )
        .await?;
        sqlx::query("UPDATE computations SET log_index = NULL, log_index_block_hash = NULL, is_executor_minted = NULL")
            .execute(&pool).await?;
    }

    if materialized {
        sqlx::query(
            "INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type)
             VALUES ($1, '\\x00', 0, 5)",
        )
        .bind(input.as_slice())
        .execute(&pool)
        .await?;
    }

    // Restart with no process-local grouping state. Complete replay infers
    // parent and child as one chain, but the duplicate INSERT preserves the
    // original child ID. Both encrypted operands need the same parent once.
    for _ in 0..3 {
        let mut replay =
            Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 128)
                .await?;
        ingest(&mut replay, &block, logs(true), true).await?;
        let count: i32 = sqlx::query_scalar(
            "SELECT dependency_count FROM dependence_chain WHERE dependence_chain_id = $1",
        )
        .bind(child.as_slice())
        .fetch_one(&pool)
        .await?;
        assert_eq!(
            count,
            i32::from(!materialized),
            "the persisted child must wait exactly once for its missing producer ciphertext"
        );
        let dependents: Vec<Vec<u8>> = sqlx::query_scalar(
            "SELECT dependents FROM dependence_chain WHERE dependence_chain_id = $1",
        )
        .bind(parent.as_slice())
        .fetch_one(&pool)
        .await?;
        assert_eq!(
            dependents,
            if materialized {
                vec![]
            } else if forked {
                vec![child.to_vec(), sibling.to_vec()]
            } else {
                vec![child.to_vec()]
            }
        );
        let canonical: (Vec<u8>, Vec<u8>) = sqlx::query_as(
            "SELECT dependence_chain_id, operand_boundary_mask FROM computations WHERE output_handle = $1",
        )
        .bind(output.as_slice())
        .fetch_one(&pool)
        .await?;
        assert_eq!(canonical.0, child.to_vec());
        let mut mask = vec![0; 32];
        mask[31] = 3;
        assert_eq!(canonical.1, mask, "replay preserves operand origins");
        if !materialized {
            assert!(replay.sealed_chains.read().await.peek(&parent).is_some());
        }
    }
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn acl_replay_restores_persisted_chain_gate_once() -> anyhow::Result<()> {
    replay_dependency_gate(false, false, false).await
}

#[tokio::test]
#[serial(db)]
async fn partial_replay_keeps_a_parent_outside_the_delivery(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    let block = block();
    let parent = FixedBytes::<32>::with_last_byte(0xb0);
    let older_parent = FixedBytes::<32>::with_last_byte(0xb1);
    let child = FixedBytes::<32>::with_last_byte(0xb2);
    let input = computed_handle(0x20);
    let output = computed_handle(0x21);
    let mut logs = vec![
        rpc_log(&trivial_encrypt(input), &block, parent, 0),
        rpc_log(
            &TfheContract::FheNeg {
                caller: Address::ZERO,
                ct: input,
                result: output,
            },
            &block,
            child,
            2,
        ),
        rpc_log(&allowed(output), &block, child, 3),
    ];
    ingest(&mut db, &block, logs.clone(), false).await?;
    // A previous transaction in this stored child chain already waits on B.
    // This replay only covers the transaction that needs A; its logs cannot
    // reconstruct B, but B's durable edge and charged gate must survive.
    sqlx::query("INSERT INTO dependence_chain (dependence_chain_id, status, dependents) VALUES ($1, 'updated', ARRAY[$2::bytea])")
        .bind(older_parent.as_slice()).bind(child.as_slice()).execute(&pool).await?;
    sqlx::query("UPDATE dependence_chain SET dependency_count = 1 WHERE dependence_chain_id = $1")
        .bind(child.as_slice()).execute(&pool).await?;
    logs.push(rpc_log(&allowed(input), &block, parent, 1));

    let mut worker_lock = db.pool().await.begin().await?;
    sqlx::query("SELECT 1 FROM dependence_chain WHERE dependence_chain_id = $1 FOR NO KEY UPDATE")
        .bind(older_parent.as_slice()).fetch_one(&mut *worker_lock).await?;
    for _ in 0..2 {
        tokio::time::timeout(
            std::time::Duration::from_secs(3),
            ingest(&mut db, &block, logs.clone(), true),
        )
        .await??;
        let count: i32 = sqlx::query_scalar("SELECT dependency_count FROM dependence_chain WHERE dependence_chain_id = $1")
            .bind(child.as_slice()).fetch_one(&pool).await?;
        assert_eq!(count, 2, "A and the already registered B must each gate the child, even while B is locked");
    }
    worker_lock.rollback().await?;
    // Model the worker retiring A and decrementing its dependent once.
    let mut retirement = pool.begin().await?;
    sqlx::query("UPDATE dependence_chain SET status = 'processed' WHERE dependence_chain_id = $1")
        .bind(parent.as_slice()).execute(&mut *retirement).await?;
    let remaining: i32 = sqlx::query_scalar("UPDATE dependence_chain SET dependency_count = dependency_count - 1 WHERE dependence_chain_id = $1 RETURNING dependency_count")
        .bind(child.as_slice()).fetch_one(&mut *retirement).await?;
    retirement.commit().await?;
    assert_eq!(
        remaining, 1,
        "completion of A must leave the child waiting for B"
    );
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn acl_replay_does_not_gate_materialized_inputs() -> anyhow::Result<()> {
    replay_dependency_gate(true, false, false).await
}

#[tokio::test]
#[serial(db)]
async fn acl_replay_counts_edges_also_present_in_inferred_forks(
) -> anyhow::Result<()> {
    replay_dependency_gate(false, true, false).await
}

#[tokio::test]
#[serial(db)]
async fn provenance_backfill_remains_a_replay_and_repairs_the_stored_chain(
) -> anyhow::Result<()> {
    replay_dependency_gate(false, false, true).await
}

#[tokio::test]
#[serial(db)]
async fn fresh_block_with_late_acl_preserves_unrelated_dependency_gates(
) -> anyhow::Result<()> {
    let mut snapshots = Vec::new();
    for late_grant in [false, true] {
        let (mut db, pool, _instance) = setup().await?;
        let first = block();
        let old_tx = FixedBytes::<32>::with_last_byte(0x90);
        let old_output = computed_handle(0x01);
        ingest(
            &mut db,
            &first,
            vec![rpc_log(&trivial_encrypt(old_output), &first, old_tx, 0)],
            false,
        )
        .await?;
        sqlx::query("UPDATE dependence_chain SET status = 'processed'")
            .execute(&pool)
            .await?;

        let fresh = host_listener::cmd::block_history::BlockSummary {
            number: first.number + 1,
            hash: FixedBytes::with_last_byte(0xb2),
            parent_hash: first.hash,
            timestamp: first.timestamp + 12,
        };
        let parent = FixedBytes::<32>::with_last_byte(0xa0);
        let input = computed_handle(0x10);
        let mut logs = vec![
            rpc_log(&trivial_encrypt(input), &fresh, parent, 0),
            rpc_log(&allowed(input), &fresh, parent, 1),
        ];
        // Two consumers force the ordinary grouping path to account for a
        // fork. A late grant for old_output must not change these new chains.
        for (offset, id) in [(2, 0xa1), (4, 0xa2)] {
            let child = FixedBytes::<32>::with_last_byte(id);
            let output = computed_handle(id);
            logs.push(rpc_log(
                &TfheContract::FheNeg {
                    caller: Address::ZERO,
                    ct: input,
                    result: output,
                },
                &fresh,
                child,
                offset,
            ));
            logs.push(rpc_log(&allowed(output), &fresh, child, offset + 1));
        }
        if late_grant {
            logs.push(rpc_log(
                &allowed(old_output),
                &fresh,
                FixedBytes::with_last_byte(0xa3),
                6,
            ));
        }
        ingest(&mut db, &fresh, logs, false).await?;
        assert_eq!(
            computation_flags(&pool, old_output).await?,
            if late_grant {
                (true, false)
            } else {
                (false, true)
            }
        );
        let snapshot: Vec<(Vec<u8>, i32, Vec<Vec<u8>>)> = sqlx::query_as(
            "SELECT dependence_chain_id, dependency_count, ARRAY(SELECT d FROM unnest(dependents) d ORDER BY d)
             FROM dependence_chain WHERE dependence_chain_id <> $1 ORDER BY dependence_chain_id"
        ).bind(old_tx.as_slice()).fetch_all(&pool).await?;
        assert!(
            snapshot.iter().any(|(_, count, _)| *count > 0),
            "the fixture must exercise actual gates"
        );
        snapshots.push(snapshot);
    }
    assert_eq!(snapshots[0], snapshots[1]);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn replay_skips_a_busy_parent_without_arming_an_unmatched_gate(
) -> anyhow::Result<()> {
    let (mut db, pool, instance) = setup().await?;
    let producer_block = block();
    let parent = FixedBytes::<32>::with_last_byte(0xa0);
    let child = FixedBytes::<32>::with_last_byte(0xa1);
    let input = computed_handle(0x10);
    let output = computed_handle(0x11);
    let producer_logs = |complete| {
        let mut logs =
            vec![rpc_log(&trivial_encrypt(input), &producer_block, parent, 0)];
        if complete {
            logs.push(rpc_log(&allowed(input), &producer_block, parent, 1));
        }
        logs
    };
    ingest(&mut db, &producer_block, producer_logs(false), false).await?;
    // Recover the two transactions in separate deliveries of the same block.
    // The consumer-only replay need not write the producer row, so a worker
    // can hold that row while the listener repairs the consumer.
    let consumer_block = producer_block;
    let consumer_logs = || {
        vec![
            rpc_log(
                &TfheContract::FheAdd {
                    caller: Address::ZERO,
                    lhs: input,
                    rhs: input,
                    scalarByte: FixedBytes::ZERO,
                    result: output,
                },
                &consumer_block,
                child,
                2,
            ),
            rpc_log(&allowed(output), &consumer_block, child, 3),
        ]
    };
    ingest(&mut db, &consumer_block, consumer_logs(), false).await?;
    ingest(&mut db, &producer_block, producer_logs(true), true).await?;
    let mut replay =
        Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 128)
            .await?;
    let assert_gate = |count: i32| {
        let pool = &pool;
        async move {
            let actual: (i32, bool) = sqlx::query_as(
                "SELECT child.dependency_count, $2 = ANY(parent.dependents)
                 FROM dependence_chain parent, dependence_chain child
                 WHERE parent.dependence_chain_id = $1 AND child.dependence_chain_id = $2",
            )
            .bind(parent.as_slice())
            .bind(child.as_slice())
            .fetch_one(pool)
            .await?;
            assert_eq!(actual, (count, count != 0));
            anyhow::Ok(())
        }
    };
    let mut locked = pool.begin().await?;
    sqlx::query(
        "SELECT 1 FROM dependence_chain WHERE dependence_chain_id = $1 FOR NO KEY UPDATE",
    )
    .bind(parent.as_slice())
    .fetch_one(&mut *locked)
    .await?;
    tokio::time::timeout(
        std::time::Duration::from_secs(3),
        ingest(&mut replay, &consumer_block, consumer_logs(), true),
    )
    .await??;
    locked.rollback().await?;
    assert_gate(0).await?;
    ingest(&mut replay, &consumer_block, consumer_logs(), true).await?;
    assert_gate(1).await?;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn replay_uses_an_earlier_producer_instead_of_a_later_same_block_remint(
) -> anyhow::Result<()> {
    let (mut db, pool, instance) = setup().await?;
    let b = block();
    let parent = FixedBytes::<32>::with_last_byte(0xa0);
    let child = FixedBytes::<32>::with_last_byte(0xa1);
    let remint = FixedBytes::<32>::with_last_byte(0xa2);
    let input = computed_handle(0x10);
    let output = computed_handle(0x11);
    let descendant = computed_handle(0x12);
    let logs = |complete| {
        let mut logs = vec![
            rpc_log(&trivial_encrypt(input), &b, parent, 0),
            rpc_log(
                &TfheContract::FheNeg {
                    caller: Address::ZERO,
                    ct: input,
                    result: output,
                },
                &b,
                child,
                2,
            ),
            rpc_log(&allowed(output), &b, child, 3),
            rpc_log(
                &TfheContract::FheNeg {
                    caller: Address::ZERO,
                    ct: output,
                    result: descendant,
                },
                &b,
                remint,
                4,
            ),
            rpc_log(&allowed(descendant), &b, remint, 5),
            rpc_log(&trivial_encrypt(input), &b, remint, 6),
        ];
        if complete {
            logs.push(rpc_log(&allowed(input), &b, parent, 1));
        }
        logs
    };
    ingest(&mut db, &b, logs(false), false).await?;
    let remint_chain: Vec<u8> = sqlx::query_scalar("SELECT dependence_chain_id FROM computations WHERE transaction_id = $1 AND output_handle = $2")
        .bind(remint.as_slice()).bind(input.as_slice()).fetch_one(&pool).await?;
    assert_eq!(remint_chain, child.to_vec());
    // Scheduling priority is not host log order: a long earlier chain can
    // rank after a short later one. Force that order in this small fixture.
    sqlx::query("UPDATE computations SET schedule_order = TIMESTAMP '1970-01-01' WHERE transaction_id = $1")
        .bind(remint.as_slice()).execute(&pool).await?;
    // A sibling observation must not erase this block's original producer
    // index. The repair delivery below omits that original producer log.
    let mut fork = b;
    fork.hash = FixedBytes::repeat_byte(0xf1);
    ingest(
        &mut db,
        &fork,
        vec![rpc_log(&trivial_encrypt(input), &fork, parent, 9)],
        false,
    )
    .await?;
    let mut replay =
        Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 128)
            .await?;
    let mut repair = logs(true);
    repair.retain(|log| log.log_index != Some(0));
    ingest(&mut replay, &b, repair, true).await?;
    let count: i32 = sqlx::query_scalar("SELECT dependency_count FROM dependence_chain WHERE dependence_chain_id = $1")
        .bind(child.as_slice()).fetch_one(&pool).await?;
    assert_eq!(
        count, 1,
        "a later remint in the child's own chain must not hide the real parent"
    );
    let dependents: Vec<Vec<u8>> = sqlx::query_scalar("SELECT dependents FROM dependence_chain WHERE dependence_chain_id = $1")
        .bind(parent.as_slice()).fetch_one(&pool).await?;
    assert_eq!(dependents, vec![child.to_vec()]);
    Ok(())
}
