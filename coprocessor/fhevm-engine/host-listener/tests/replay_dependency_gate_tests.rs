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
    replay_dependency_gate(false, false).await
}

#[tokio::test]
#[serial(db)]
async fn acl_replay_does_not_gate_materialized_inputs() -> anyhow::Result<()> {
    replay_dependency_gate(true, false).await
}

#[tokio::test]
#[serial(db)]
async fn acl_replay_counts_edges_also_present_in_inferred_forks(
) -> anyhow::Result<()> {
    replay_dependency_gate(false, true).await
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
