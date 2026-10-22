//! Ingestion validates block provenance and publishes chain-cache changes only
//! after the database transaction commits.

mod ingest_fixtures;
use ingest_fixtures::*;

use alloy::primitives::{Address, FixedBytes};
use fhevm_engine_common::chain_id::ChainId;
use host_listener::cmd::block_history::BlockSummary;
use host_listener::contracts::TfheContract;
use host_listener::database::tfhe_event_propagate::Database;
use serial_test::serial;
use tokio::time::{timeout, Duration};

fn add(input: FixedBytes<32>, result: FixedBytes<32>) -> TfheContract::FheAdd {
    TfheContract::FheAdd {
        caller: Address::ZERO,
        lhs: input,
        rhs: input,
        scalarByte: FixedBytes::ZERO,
        result,
    }
}

async fn assert_empty(
    pool: &sqlx::PgPool,
    db: &Database,
) -> anyhow::Result<()> {
    let counts: (i64, i64, i64, i64) = sqlx::query_as(
        "SELECT (SELECT COUNT(*) FROM computations), (SELECT COUNT(*) FROM allowed_handles), (SELECT COUNT(*) FROM dependence_chain), (SELECT COUNT(*) FROM host_chain_blocks_valid)",
    ).fetch_one(pool).await?;
    assert_eq!(
        counts,
        (0, 0, 0, 0),
        "a rejected block must not publish partial database state"
    );
    assert!(
        db.dependence_chain.read().await.is_empty(),
        "a rejected block must not publish cache state"
    );
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn invalid_log_provenance_rejects_the_entire_block() -> anyhow::Result<()>
{
    let (mut db, pool, _instance) = setup().await?;
    let b = block();
    let tx_hash = computed_handle(0xa1);
    let handle = computed_handle(0x11);
    let valid = rpc_log(&trivial_encrypt(handle), &b, tx_hash, 0);
    for case in [
        "foreign_hash",
        "foreign_number",
        "missing_hash",
        "missing_number",
        "removed",
        "missing_transaction",
        "missing_index",
    ] {
        let mut invalid = rpc_log(&allowed(handle), &b, tx_hash, 1);
        match case {
            "foreign_hash" => invalid.block_hash = Some(computed_handle(0xff)),
            "foreign_number" => invalid.block_number = Some(b.number + 1),
            "missing_hash" => invalid.block_hash = None,
            "missing_number" => invalid.block_number = None,
            "removed" => invalid.removed = true,
            "missing_transaction" => invalid.transaction_hash = None,
            "missing_index" => invalid.log_index = None,
            _ => unreachable!(),
        }
        let result =
            ingest(&mut db, &b, vec![valid.clone(), invalid], false).await;
        assert!(result.is_err(), "{case} must reject the block");
        assert_empty(&pool, &db).await?;
    }
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn failed_transaction_does_not_publish_chain_cache_changes(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    let parent = block();
    let parent_tx = computed_handle(0xa3);
    let parent_handle = computed_handle(0x13);
    ingest(
        &mut db,
        &parent,
        vec![
            rpc_log(&trivial_encrypt(parent_handle), &parent, parent_tx, 0),
            rpc_log(&allowed(parent_handle), &parent, parent_tx, 1),
        ],
        false,
    )
    .await?;
    let observer = db.clone();
    let before = db
        .dependence_chain
        .read()
        .await
        .iter()
        .map(|(handle, chain)| (*handle, *chain))
        .collect::<Vec<_>>();
    let child = BlockSummary {
        number: parent.number + 1,
        hash: computed_handle(0xb2),
        parent_hash: parent.hash,
        timestamp: parent.timestamp + 12,
    };
    let child_tx = computed_handle(0xa4);
    let child_handle = computed_handle(0x14);
    let logs = vec![
        rpc_log(&add(parent_handle, child_handle), &child, child_tx, 0),
        rpc_log(&allowed(child_handle), &child, child_tx, 1),
    ];
    // Existing rows are exempt, but every new/updated chain row now fails. This
    // forces rollback after grouping and computation insertion have occurred.
    sqlx::query("ALTER TABLE dependence_chain ADD CONSTRAINT reject_atomicity_test CHECK (false) NOT VALID").execute(&pool).await?;
    let result = ingest(&mut db, &child, logs.clone(), false).await;
    sqlx::query(
        "ALTER TABLE dependence_chain DROP CONSTRAINT reject_atomicity_test",
    )
    .execute(&pool)
    .await?;
    assert!(
        result.is_err(),
        "the injected database failure must reach ingestion"
    );
    let after = observer
        .dependence_chain
        .read()
        .await
        .iter()
        .map(|(handle, chain)| (*handle, *chain))
        .collect::<Vec<_>>();
    assert_eq!(
        after, before,
        "rolled-back grouping must not change the shared cache"
    );
    let rows: (i64, i64, i64) = sqlx::query_as("SELECT (SELECT COUNT(*) FROM computations WHERE output_handle = $1), (SELECT COUNT(*) FROM allowed_handles WHERE handle = $1), (SELECT COUNT(*) FROM host_chain_blocks_valid WHERE block_hash = $2)")
        .bind(child_handle.to_vec()).bind(child.hash.to_vec()).fetch_one(&pool).await?;
    assert_eq!(rows, (0, 0, 0));
    // The same data succeeds after the failure is removed and becomes visible
    // through an already-created Database clone only once it has committed.
    ingest(&mut db, &child, logs, true).await?;
    assert_eq!(
        observer
            .dependence_chain
            .read()
            .await
            .peek(&child_handle)
            .copied(),
        Some(parent_tx.into())
    );
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn concurrent_database_clones_keep_both_committed_cache_publications(
) -> anyhow::Result<()> {
    let (mut first, _pool, _instance) = setup().await?;
    let mut second = first.clone();
    let observer = first.clone();
    for byte in 1..=8 {
        let mut a = block();
        a.number += u64::from(byte) * 2;
        a.hash = computed_handle(byte);
        let b = BlockSummary {
            number: a.number + 1,
            hash: computed_handle(byte + 32),
            parent_hash: a.hash,
            timestamp: a.timestamp + 12,
        };
        let tx_a = computed_handle(byte);
        let tx_b = computed_handle(byte + 32);
        let out_a = computed_handle(byte + 64);
        let out_b = computed_handle(byte + 96);
        let (left, right) = tokio::join!(
            ingest(
                &mut first,
                &a,
                vec![
                    rpc_log(&trivial_encrypt(out_a), &a, tx_a, 0),
                    rpc_log(&allowed(out_a), &a, tx_a, 1)
                ],
                false
            ),
            ingest(
                &mut second,
                &b,
                vec![
                    rpc_log(&trivial_encrypt(out_b), &b, tx_b, 0),
                    rpc_log(&allowed(out_b), &b, tx_b, 1)
                ],
                false
            )
        );
        left?;
        right?;
        let cache = observer.dependence_chain.read().await;
        assert_eq!(cache.peek(&out_a).copied(), Some(tx_a.into()));
        assert_eq!(cache.peek(&out_b).copied(), Some(tx_b.into()));
    }
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn cold_replay_publishes_persisted_chain_for_subsequent_consumers(
) -> anyhow::Result<()> {
    let (mut original, pool, instance) = setup().await?;
    let parent = block();
    let parent_tx = computed_handle(0xa5);
    let parent_output = computed_handle(0x15);
    ingest(
        &mut original,
        &parent,
        vec![
            rpc_log(&trivial_encrypt(parent_output), &parent, parent_tx, 0),
            rpc_log(&allowed(parent_output), &parent, parent_tx, 1),
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
    let child_tx = computed_handle(0xa6);
    let child_output = computed_handle(0x16);
    let child_logs = vec![
        rpc_log(&add(parent_output, child_output), &child, child_tx, 0),
        rpc_log(&allowed(child_output), &child, child_tx, 1),
    ];
    ingest(&mut original, &child, child_logs.clone(), false).await?;
    let stored: Vec<u8> = sqlx::query_scalar(
        "SELECT dependence_chain_id FROM computations WHERE output_handle = $1",
    )
    .bind(child_output.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(stored, parent_tx.to_vec());
    let mut replay =
        Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 128)
            .await?;
    let observer = replay.clone();
    ingest(&mut replay, &child, child_logs, true).await?;
    assert_eq!(observer.dependence_chain.read().await.peek(&child_output).copied(), Some(parent_tx.into()), "replay must publish the stored chain, not the cold cache's inferred child chain");
    let grandchild = BlockSummary {
        number: child.number + 1,
        hash: computed_handle(0xb3),
        parent_hash: child.hash,
        timestamp: child.timestamp + 12,
    };
    let grandchild_tx = computed_handle(0xa7);
    let grandchild_output = computed_handle(0x17);
    ingest(
        &mut replay,
        &grandchild,
        vec![
            rpc_log(
                &add(child_output, grandchild_output),
                &grandchild,
                grandchild_tx,
                0,
            ),
            rpc_log(&allowed(grandchild_output), &grandchild, grandchild_tx, 1),
        ],
        false,
    )
    .await?;
    let stored: Vec<u8> = sqlx::query_scalar(
        "SELECT dependence_chain_id FROM computations WHERE output_handle = $1",
    )
    .bind(grandchild_output.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(
        stored,
        parent_tx.to_vec(),
        "later consumers must join the producer's persisted chain"
    );
    Ok(())
}

async fn wait_for_chain_writers(
    pool: &sqlx::PgPool,
    count: i64,
) -> anyhow::Result<()> {
    timeout(Duration::from_secs(5), async {
        loop {
            let waiting: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM pg_stat_activity
                 WHERE datname = current_database() AND wait_event_type = 'Lock'
                   AND (query LIKE '%INSERT INTO dependence_chain%' OR query LIKE '%pg_advisory_xact_lock(hashtextextended%')",
            )
            .fetch_one(pool)
            .await?;
            if waiting == count {
                return Ok::<_, sqlx::Error>(());
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await??;
    Ok(())
}

async fn concurrent_divergent_grouping_repair(
    missing_chains: bool,
) -> anyhow::Result<()> {
    let (mut warm, pool, instance) = setup().await?;
    let mut cold =
        Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 128)
            .await?;
    let parent = block();
    let a = computed_handle(0xa1);
    let b = computed_handle(0xa2);
    let input = computed_handle(0x11);
    let first = computed_handle(0x12);
    let second = computed_handle(0x13);
    // The history admission fence serializes concurrent repair deliveries
    // before either can publish the ordered union of inferred/stored chains.
    ingest(
        &mut warm,
        &parent,
        vec![
            rpc_log(&trivial_encrypt(input), &parent, a, 0),
            rpc_log(&allowed(input), &parent, a, 1),
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
    let sibling = BlockSummary {
        hash: computed_handle(0xb3),
        ..child
    };
    // Use competing block observations: v0.14 updates ancestry on the block
    // marker, so same-block deliveries would serialize before chain writes.
    // Two partial deliveries of transaction B omit its ACLs. Only the warm
    // listener knows input -> A, so the computations persist different chains.
    ingest(
        &mut warm,
        &child,
        vec![rpc_log(&add(input, first), &child, b, 0)],
        false,
    )
    .await?;
    ingest(
        &mut cold,
        &sibling,
        vec![rpc_log(&add(input, second), &sibling, b, 1)],
        false,
    )
    .await?;
    let stored: Vec<(Vec<u8>, bool, bool)> = sqlx::query_as(
        "SELECT dependence_chain_id, is_allowed, is_completed FROM computations
         WHERE output_handle = ANY($1::bytea[]) ORDER BY output_handle",
    )
    .bind(vec![first.to_vec(), second.to_vec()])
    .fetch_all(&pool)
    .await?;
    assert_eq!(
        stored,
        vec![(a.to_vec(), false, true), (b.to_vec(), false, true)]
    );
    assert_eq!(
        warm.dependence_chain.read().await.peek(&input).copied(),
        Some(a.into())
    );
    assert!(cold.dependence_chain.read().await.peek(&input).is_none());
    // Both non-finalized block markers already exist. Also cover cleanup:
    // pre-locking only existing rows would leave that variant vulnerable.
    if missing_chains {
        sqlx::query("DELETE FROM dependence_chain")
            .execute(&pool)
            .await?;
    } else {
        sqlx::query("UPDATE dependence_chain SET status = 'processed'")
            .execute(&pool)
            .await?;
    }
    sqlx::raw_sql(
        "CREATE FUNCTION pause_test_chain_write() RETURNS trigger LANGUAGE plpgsql AS $$
         BEGIN
             PERFORM pg_advisory_xact_lock_shared(4278, 2);
             RETURN NEW;
         END;
         $$;
         CREATE TRIGGER pause_test_chain_write AFTER INSERT OR UPDATE ON dependence_chain
         FOR EACH ROW EXECUTE FUNCTION pause_test_chain_write();",
    ).execute(&pool).await?;
    // Pause AFTER the first chain write has taken its row/unique-index lock.
    // The shared gate lets both writers proceed together when released. With
    // two separately sorted passes they own opposite rows and then deadlock;
    // with one common order the second writer waits for the first chain.
    let mut gate = warm.pool().await.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock(4278, 2)")
        .execute(&mut *gate)
        .await?;
    let cold_observer = cold.clone();
    let warm_observer = warm.clone();
    let cold_repair = tokio::spawn(async move {
        ingest(
            &mut cold,
            &child,
            vec![
                rpc_log(&add(input, first), &child, b, 0),
                rpc_log(&allowed(first), &child, b, 2),
            ],
            true,
        )
        .await
    });
    wait_for_chain_writers(&pool, 1).await?;
    let warm_repair = tokio::spawn(async move {
        ingest(
            &mut warm,
            &sibling,
            vec![
                rpc_log(&add(input, second), &sibling, b, 1),
                rpc_log(&allowed(second), &sibling, b, 3),
            ],
            true,
        )
        .await
    });
    wait_for_chain_writers(&pool, 2).await?;
    gate.commit().await?;
    // Exercise the complete ingestion path once each, without caller retries.
    let (cold_result, warm_result) = timeout(Duration::from_secs(5), async {
        tokio::join!(cold_repair, warm_repair)
    })
    .await?;
    cold_result??;
    warm_result??;
    assert_eq!(
        cold_observer
            .dependence_chain
            .read()
            .await
            .peek(&first)
            .copied(),
        Some(a.into())
    );
    assert_eq!(
        warm_observer
            .dependence_chain
            .read()
            .await
            .peek(&second)
            .copied(),
        Some(b.into())
    );
    let repaired: Vec<(Vec<u8>, bool, bool, String)> = sqlx::query_as(
        "SELECT c.dependence_chain_id, c.is_allowed, c.is_completed, d.status
         FROM computations c JOIN dependence_chain d USING (dependence_chain_id)
         WHERE c.output_handle = ANY($1::bytea[]) ORDER BY c.output_handle",
    )
    .bind(vec![first.to_vec(), second.to_vec()])
    .fetch_all(&pool)
    .await?;
    assert_eq!(
        repaired,
        vec![
            (a.to_vec(), true, false, "updated".into()),
            (b.to_vec(), true, false, "updated".into()),
        ]
    );
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn concurrent_divergent_grouping_repairs_existing_chains(
) -> anyhow::Result<()> {
    concurrent_divergent_grouping_repair(false).await
}

#[tokio::test]
#[serial(db)]
async fn concurrent_divergent_grouping_recreates_missing_chains(
) -> anyhow::Result<()> {
    concurrent_divergent_grouping_repair(true).await
}
