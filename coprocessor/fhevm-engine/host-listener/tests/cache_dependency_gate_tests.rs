//! CFI-0025: evicting a producer from the bounded cache must not lose its gate.

mod ingest_fixtures;
use ingest_fixtures::*;

use alloy::primitives::{Address, FixedBytes};
use host_listener::contracts::TfheContract;
use serial_test::serial;
use std::num::NonZeroUsize;

#[tokio::test]
#[serial(db)]
async fn evicted_producer_keeps_its_cross_block_join_gate() -> anyhow::Result<()>
{
    let (mut db, pool, _instance) = setup().await?;
    db.dependence_chain
        .write()
        .await
        .resize(NonZeroUsize::new(16).unwrap());
    let producer_block = block();
    let mut consumer_block = producer_block;
    consumer_block.number += 1;
    consumer_block.parent_hash = producer_block.hash;
    consumer_block.hash = computed_handle(0xb2);
    consumer_block.timestamp += 1;

    let parents = (1..=17).map(computed_handle).collect::<Vec<_>>();
    let parent_chains = (1..=17)
        .map(|id| FixedBytes::<32>::with_last_byte(0x20 + id))
        .collect::<Vec<_>>();
    let mut producer_logs = Vec::new();
    for (index, (handle, chain)) in
        parents.iter().zip(&parent_chains).enumerate()
    {
        producer_logs.push(rpc_log(
            &trivial_encrypt(*handle),
            &producer_block,
            *chain,
            2 * index as u64,
        ));
        producer_logs.push(rpc_log(
            &allowed(*handle),
            &producer_block,
            *chain,
            2 * index as u64 + 1,
        ));
    }
    ingest(&mut db, &producer_block, producer_logs, false).await?;
    {
        let cache = db.dependence_chain.read().await;
        assert_eq!(cache.len(), 16);
        assert_eq!(
            parents
                .iter()
                .filter(|handle| !cache.contains(*handle))
                .count(),
            1
        );
    }

    let child = FixedBytes::<32>::with_last_byte(0xf0);
    let intermediate = computed_handle(0x70);
    let output = computed_handle(0x71);
    let mut inputs = parents.clone();
    inputs.extend([intermediate, parents[0], intermediate]);
    let consumer_logs = vec![
        rpc_log(
            &host_listener::contracts::TfheContract::FheAdd {
                caller: Address::ZERO,
                lhs: parents[0],
                rhs: parents[1],
                scalarByte: FixedBytes::ZERO,
                result: intermediate,
            },
            &consumer_block,
            child,
            0,
        ),
        rpc_log(
            &host_listener::contracts::TfheContract::FheSum {
                caller: Address::ZERO,
                values: inputs,
                result: output,
            },
            &consumer_block,
            child,
            1,
        ),
        rpc_log(&allowed(output), &consumer_block, child, 2),
    ];

    for replay in 0..3 {
        ingest(&mut db, &consumer_block, consumer_logs.clone(), replay != 0)
            .await?;
        let (stored_child, gate_count): (Vec<u8>, i32) = sqlx::query_as(
            "SELECT c.dependence_chain_id, dc.dependency_count FROM computations c
             JOIN dependence_chain dc USING (dependence_chain_id) WHERE c.output_handle = $1",
        ).bind(output.as_slice()).fetch_one(&pool).await?;
        assert_eq!(stored_child, child.to_vec());
        assert_eq!(gate_count, 17, "all distinct pending parents must gate the join, including the evicted producer");
        for parent in &parent_chains {
            let children: Vec<Vec<u8>> = sqlx::query_scalar(
                "SELECT dependents FROM dependence_chain WHERE dependence_chain_id = $1",
            ).bind(parent.as_slice()).fetch_one(&pool).await?;
            assert_eq!(
                children,
                vec![child.to_vec()],
                "each parent owes exactly one discharge after replay {replay}"
            );
        }
    }
    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum Producer {
    Pending,
    EarlierInSameBlock,
    Materialized,
    Processed,
    Busy,
    Unallowed,
    Error,
    OtherHostChain,
    FutureBlock,
    LaterInSameBlock,
    SameBlockUnknownIndex,
    Unknown,
    CrossBlockDisabled,
}

/// Cold-cache recovery must preserve provenance and the gate registration
/// contract: count exactly the eligible pending parents actually registered.
#[tokio::test]
#[serial(db)]
async fn cold_cache_checks_provenance_and_parent_readiness(
) -> anyhow::Result<()> {
    for state in [
        Producer::Pending,
        Producer::EarlierInSameBlock,
        Producer::Materialized,
        Producer::Processed,
        Producer::Busy,
        Producer::Unallowed,
        Producer::Error,
        Producer::OtherHostChain,
        Producer::FutureBlock,
        Producer::LaterInSameBlock,
        Producer::SameBlockUnknownIndex,
        Producer::Unknown,
        Producer::CrossBlockDisabled,
    ] {
        let (mut db, pool, _instance) = setup().await?;
        let producer_block = block();
        let mut consumer_block = producer_block;
        consumer_block.number += 1;
        consumer_block.parent_hash = producer_block.hash;
        consumer_block.hash = computed_handle(0xb2);
        consumer_block.timestamp += 1;
        let parent = FixedBytes::<32>::with_last_byte(0xa0);
        let child = FixedBytes::<32>::with_last_byte(0xa1);
        let input = computed_handle(0x10);
        let output = computed_handle(0x11);
        ingest(
            &mut db,
            &producer_block,
            vec![
                rpc_log(&trivial_encrypt(input), &producer_block, parent, 0),
                rpc_log(&allowed(input), &producer_block, parent, 1),
            ],
            false,
        )
        .await?;
        db.dependence_chain.write().await.clear();

        // Model durable producer states while leaving the cache empty. Same-
        // block cases exercise delivery splitting with explicit log order.
        let mutation = match state {
            Producer::Unallowed => {
                Some("UPDATE computations SET is_allowed = false")
            }
            Producer::Error => Some("UPDATE computations SET is_error = true"),
            Producer::OtherHostChain => {
                Some("UPDATE computations SET host_chain_id = 43")
            }
            Producer::FutureBlock => {
                Some("UPDATE computations SET block_number = 102")
            }
            Producer::EarlierInSameBlock => Some(
                "UPDATE computations SET block_number = 101, log_index = 0",
            ),
            Producer::LaterInSameBlock => Some(
                "UPDATE computations SET block_number = 101, log_index = 4",
            ),
            Producer::SameBlockUnknownIndex => Some(
                "UPDATE computations SET block_number = 101, log_index = NULL, is_executor_minted = NULL",
            ),
            Producer::Unknown => Some("DELETE FROM computations"),
            Producer::Processed => {
                Some("UPDATE dependence_chain SET status = 'processed'")
            }
            _ => None,
        };
        if let Some(mutation) = mutation {
            sqlx::query(mutation).execute(&pool).await?;
        }
        if matches!(state, Producer::Materialized) {
            sqlx::query(
                "INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type)
                 VALUES ($1, '\\x00', 0, 5)",
            ).bind(input.as_slice()).execute(&pool).await?;
        }
        let mut busy = if matches!(state, Producer::Busy) {
            let mut tx = pool.begin().await?;
            sqlx::query("SELECT 1 FROM dependence_chain WHERE dependence_chain_id = $1 FOR UPDATE")
                .bind(parent.as_slice()).execute(&mut *tx).await?;
            Some(tx)
        } else {
            None
        };
        let logs = vec![
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
        ];
        let mut options = OPTIONS;
        options.dependence_cross_block =
            !matches!(state, Producer::CrossBlockDisabled);
        tokio::time::timeout(
            std::time::Duration::from_secs(5),
            ingest_with_options(&mut db, &consumer_block, logs, false, options),
        )
        .await??;
        if let Some(tx) = busy.take() {
            tx.rollback().await?;
        }
        let (stored_child, count): (Vec<u8>, i32) = sqlx::query_as(
            "SELECT c.dependence_chain_id, dc.dependency_count FROM computations c
             JOIN dependence_chain dc USING (dependence_chain_id) WHERE c.output_handle = $1",
        ).bind(output.as_slice()).fetch_one(&pool).await?;
        assert_eq!(
            stored_child,
            child.to_vec(),
            "cold-cache parent must not be extended: {state:?}"
        );
        let gated =
            matches!(state, Producer::Pending | Producer::EarlierInSameBlock);
        assert_eq!(count, i32::from(gated), "{state:?}");
        let children: Vec<Vec<u8>> = sqlx::query_scalar(
            "SELECT dependents FROM dependence_chain WHERE dependence_chain_id = $1",
        ).bind(parent.as_slice()).fetch_one(&pool).await?;
        assert_eq!(
            children,
            if gated { vec![child.to_vec()] } else { vec![] },
            "{state:?}"
        );
    }
    Ok(())
}
