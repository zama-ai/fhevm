//! Regression coverage for sparse finality and replay takeover.
mod ingest_fixtures;
use alloy::primitives::FixedBytes;
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::chain_recovery::{
    is_halted, pending_host_replay, pending_replay, prepare_host_replay,
    readiness_blocked, HostReplayStartup, ListenerSource,
};
use host_listener::cmd::block_history::BlockSummary;
use host_listener::database::history_verification::verify_finalized_history;
use host_listener::database::ingest::{
    ingest_block_logs, update_finalized_blocks_aux, BlockLogs,
};
use host_listener::database::tfhe_event_propagate::Database;
use ingest_fixtures::*;
use serial_test::serial;

fn header(number: u64, canonical: bool) -> BlockSummary {
    let hash = |n| {
        FixedBytes::from(
            [n as u8 + if canonical && n >= 75 { 100 } else { 0 }; 32],
        )
    };
    BlockSummary {
        number,
        hash: hash(number),
        parent_hash: hash(number - 1),
        timestamp: 1_790_000_000 + number,
    }
}

async fn admit(
    db: &mut Database,
    summary: BlockSummary,
    finalized: bool,
) -> anyhow::Result<()> {
    let logs = if summary.hash == header(82, true).hash {
        let output = computed_handle(0x51);
        let transaction = computed_handle(0xE1);
        vec![
            rpc_log(&trivial_encrypt(output), &summary, transaction, 0),
            rpc_log(&allowed(output), &summary, transaction, 1),
        ]
    } else {
        vec![]
    };
    ingest_block_logs(
        ChainId::try_from(42_u64)?,
        db,
        &BlockLogs {
            logs,
            summary,
            catchup: true,
            finalized,
        },
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

#[tokio::test]
#[serial(db)]
async fn sparse_catchup_repair_contains_a_contradiction_instead_of_repeating(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    for number in 74..=80 {
        admit(&mut db, header(number, false), true).await?;
    }
    admit(&mut db, header(81, false), false).await?;
    // Sparse historical delivery can retain disconnected finalized segments.
    // The latest segment alone does not prove that the older one is canonical.
    admit(&mut db, header(82, true), true).await?;
    let error = update_finalized_blocks_aux(&mut db, 100, 10, |n| async move {
        Ok(header(n, true).hash)
    })
    .await
    .unwrap_err();
    // Verify the disconnected older segment before queuing any gap repair.
    assert!(error.to_string().contains("committed canonical history"));
    assert!(is_halted(&pool).await?);
    let repaired: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM host_chain_blocks_valid WHERE block_hash=$1",
    )
    .bind(header(81, true).hash.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(repaired, 0, "contradictory work must not be admitted");
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn surviving_poller_adopts_dead_starters_generation() -> anyhow::Result<()>
{
    let (db, pool, _instance) = setup().await?;
    db.poller_set_last_caught_up_block(ChainId::try_from(42_u64)?, 100)
        .await?;
    let HostReplayStartup::Ready {
        replay: Some(first),
        ..
    } = prepare_host_replay(&pool, false, 42, 10, None).await?
    else {
        panic!("first replay")
    };
    let HostReplayStartup::Ready {
        replay: Some(second),
        ..
    } = prepare_host_replay(&pool, false, 42, 10, None).await?
    else {
        panic!("second replay")
    };
    let generation = second.progress.generation;
    assert!(!first.complete(&pool, 100).await?);
    assert!(readiness_blocked(&pool).await?);
    let adopted = pending_host_replay(&pool, false, 42).await?.unwrap();
    assert_eq!(adopted.progress.generation, generation);
    assert!(!first.same_chunk(&adopted));
    assert!(!adopted.complete(&pool, 100).await?);
    assert!(!readiness_blocked(&pool).await?);
    assert!(pending_host_replay(&pool, false, 42).await?.is_none());
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn long_abandoned_replay_is_adopted_in_bounded_chunks_without_loss(
) -> anyhow::Result<()> {
    let (db, pool, _instance) = setup().await?;
    db.poller_set_last_caught_up_block(ChainId::try_from(42_u64)?, 100)
        .await?;
    prepare_host_replay(&pool, false, 42, 10, None).await?;
    db.poller_set_last_caught_up_block(ChainId::try_from(42_u64)?, 20_150)
        .await?;
    let HostReplayStartup::Ready {
        replay: Some(first_chunk),
        ..
    } = prepare_host_replay(&pool, false, 42, 10, None).await?
    else {
        panic!("adopt replay")
    };
    assert_eq!(
        (
            first_chunk.progress.from_block,
            first_chunk.progress.to_block
        ),
        (91, 10_090)
    );
    let generation = first_chunk.progress.generation;
    assert!(first_chunk.complete(&pool, 10_090).await?);
    let durable = pending_replay(&pool, ListenerSource::Host, false, 42)
        .await?
        .unwrap();
    assert_eq!((durable.from_block, durable.to_block), (10_091, 20_150));
    assert!(readiness_blocked(&pool).await?);
    let second = pending_host_replay(&pool, false, 42).await?.unwrap();
    assert_eq!(second.progress.generation, generation);
    assert_eq!(second.progress.to_block, 20_090);
    assert!(second.complete(&pool, 20_090).await?);
    let last = pending_host_replay(&pool, false, 42).await?.unwrap();
    assert_eq!(
        (last.progress.from_block, last.progress.to_block),
        (20_091, 20_150)
    );
    assert!(!last.complete(&pool, 20_150).await?);
    assert!(!readiness_blocked(&pool).await?);
    Ok(())
}

// A range response need not include an empty height at all.
#[tokio::test]
#[serial(db)]
async fn sparse_finalized_gap_without_pending_rows_is_detected(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    for number in 74..=80 {
        admit(&mut db, header(number, false), true).await?;
    }
    // No row for 81: it has never carried an event and the old listener was
    // disconnected. Range catchup jumps straight to the first logged block.
    admit(&mut db, header(82, true), true).await?;
    let error = update_finalized_blocks_aux(&mut db, 100, 10, |n| async move {
        Ok(header(n, true).hash)
    })
    .await
    .unwrap_err();
    assert!(is_halted(&pool).await?);
    assert!(error.to_string().contains("committed canonical history"));
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn verified_sparse_history_only_rechecks_the_newest_segment(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    for n in [74, 76, 78] {
        admit(&mut db, header(n, true), true).await?;
    }
    let mut checked = vec![];
    verify_finalized_history(&db, |n| {
        checked.push(n);
        async move { Ok(header(n, true).hash) }
    })
    .await?;
    assert_eq!(checked, [78, 76, 74]);
    checked.clear();
    verify_finalized_history(&db, |n| {
        checked.push(n);
        async move { Ok(header(n, true).hash) }
    })
    .await?;
    assert_eq!(checked, [78], "stable older segments need no repeated RPC");
    admit(&mut db, header(80, true), true).await?;
    checked.clear();
    verify_finalized_history(&db, |n| {
        checked.push(n);
        async move { Ok(header(n, true).hash) }
    })
    .await?;
    assert_eq!(
        checked,
        [80, 78],
        "a previous tip must not become trusted across a new gap"
    );
    assert!(!is_halted(&pool).await?);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn a_verified_tip_is_rechecked_after_a_sparse_reorg() -> anyhow::Result<()>
{
    let (mut db, pool, _instance) = setup().await?;
    for n in 74..=80 {
        admit(&mut db, header(n, false), true).await?;
    }
    verify_finalized_history(&db, |n| async move { Ok(header(n, false).hash) })
        .await?;
    admit(&mut db, header(82, true), true).await?;
    let error =
        verify_finalized_history(
            &db,
            |n| async move { Ok(header(n, true).hash) },
        )
        .await
        .unwrap_err();
    assert!(error.to_string().contains("committed canonical history"));
    assert!(is_halted(&pool).await?);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn removing_the_conflicting_anchor_during_rpc_invalidates_the_observation(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    admit(&mut db, header(80, false), true).await?;
    let error = verify_finalized_history(&db, |n| {
        let pool = pool.clone();
        async move {
            // Simulate pruning/recovery removing the observed identity while
            // RPC is in flight. It no longer justifies containing this history.
            sqlx::query(
                "DELETE FROM host_chain_blocks_valid WHERE block_number = $1",
            )
            .bind(n as i64)
            .execute(&pool)
            .await?;
            Ok(header(n, true).hash)
        }
    })
    .await
    .unwrap_err();
    assert!(error.to_string().contains("changed during verification"));
    assert!(!is_halted(&pool).await?);
    verify_finalized_history(&db, |n| async move { Ok(header(n, false).hash) })
        .await?;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn promotion_during_rpc_invalidates_a_conflicting_observation(
) -> anyhow::Result<()> {
    use fhevm_engine_common::database::{GCS_SCHEMA_QUOTED, GCS_SEARCH_PATH};
    use fhevm_engine_common::versioning::CUTOVER_LOCK_ID;
    use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

    let (mut db, pool, instance) = setup().await?;
    admit(&mut db, header(80, false), true).await?;
    // Resolve the initial role through the green cursor namespace. History is
    // deliberately identical across the role change: only the role invalidates
    // the old observation.
    sqlx::query(&format!("CREATE SCHEMA {GCS_SCHEMA_QUOTED}"))
        .execute(&pool)
        .await?;
    sqlx::query(&format!(
        "CREATE TABLE {GCS_SCHEMA_QUOTED}.host_listener_poller_state
         (LIKE public.host_listener_poller_state INCLUDING ALL)"
    ))
    .execute(&pool)
    .await?;
    let green_pool = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(
            instance
                .db_url()
                .parse::<PgConnectOptions>()?
                .options([("search_path", GCS_SEARCH_PATH)]),
        )
        .await?;
    *db.pool.write().await = green_pool;
    let error = verify_finalized_history(&db, |n| {
        let pool = pool.clone();
        async move {
            let mut tx = pool.begin().await?;
            sqlx::query("SELECT pg_advisory_xact_lock($1)")
                .bind(CUTOVER_LOCK_ID).execute(&mut *tx).await?;
            sqlx::query(&format!(
                "ALTER SCHEMA {GCS_SCHEMA_QUOTED} RENAME TO host_before_promotion"
            ))
            .execute(&mut *tx).await?;
            tx.commit().await?;
            Ok(header(n, true).hash)
        }
    }).await.unwrap_err();
    assert!(error.to_string().contains("changed during verification"));
    assert!(!is_halted(&pool).await?);
    verify_finalized_history(&db, |n| async move { Ok(header(n, false).hash) })
        .await?;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn concurrent_extensions_do_not_mask_a_retained_history_conflict(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    admit(&mut db, header(80, false), true).await?;
    let other = db.clone();
    let error = verify_finalized_history(&db, |n| {
        let mut other = other.clone();
        async move {
            // Production admission by another replica, while the verifier has
            // no database lock open. The old tip becomes an interior block
            // and a newer disconnected segment is added; neither removes it.
            admit(&mut other, header(81, false), true).await?;
            admit(&mut other, header(84, true), true).await?;
            Ok(header(n, true).hash)
        }
    })
    .await
    .unwrap_err();
    assert!(error.to_string().contains("committed canonical history"));
    assert!(is_halted(&pool).await?);
    let halt: (i64, Vec<u8>, Vec<u8>) = sqlx::query_as(
        "SELECT block_number, expected_hash, observed_hash
         FROM public.chain_recovery_halts WHERE resolved_at IS NULL",
    )
    .fetch_one(&pool)
    .await?;
    assert_eq!(
        halt,
        (
            80,
            header(80, false).hash.to_vec(),
            header(80, true).hash.to_vec()
        )
    );
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn removing_an_unrelated_tip_does_not_mask_a_retained_conflict(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    for n in [80, 84] {
        admit(&mut db, header(n, false), true).await?;
    }
    let error = verify_finalized_history(&db, |n| {
        let pool = pool.clone();
        async move {
            if n == 80 {
                // The positive prefix cannot be checkpointed against its old
                // newest tip anymore, but the conflicting 80 is still present.
                sqlx::query("DELETE FROM host_chain_blocks_valid WHERE block_number = 84")
                    .execute(&pool).await?;
            }
            Ok(header(n, n == 80).hash)
        }
    })
    .await
    .unwrap_err();
    assert!(error.to_string().contains("committed canonical history"));
    assert!(is_halted(&pool).await?);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn an_unavailable_segment_does_not_get_cached() -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    for n in [74, 80, 82] {
        admit(&mut db, header(n, false), true).await?;
    }
    let result = verify_finalized_history(&db, |n| async move {
        anyhow::ensure!(n != 80, "RPC unavailable");
        Ok(header(n, false).hash)
    })
    .await;
    assert!(result.is_err());
    assert!(!is_halted(&pool).await?);
    let mut checked = vec![];
    verify_finalized_history(&db, |n| {
        checked.push(n);
        async move { Ok(header(n, false).hash) }
    })
    .await?;
    assert_eq!(checked, [82, 80, 74]);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn losing_the_verified_tip_invalidates_cached_segments(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    for n in [74, 80, 82] {
        admit(&mut db, header(n, false), true).await?;
    }
    verify_finalized_history(&db, |n| async move { Ok(header(n, false).hash) })
        .await?;
    sqlx::query("DELETE FROM host_chain_blocks_valid WHERE block_number = 82")
        .execute(&pool)
        .await?;
    admit(&mut db, header(84, true), true).await?;
    let result =
        verify_finalized_history(
            &db,
            |n| async move { Ok(header(n, true).hash) },
        )
        .await;
    assert!(result.is_err());
    assert!(
        is_halted(&pool).await?,
        "a removed checkpoint cannot justify skipping older history"
    );
    Ok(())
}

fn numbered_hash(number: u64) -> FixedBytes<32> {
    let mut bytes = [0; 32];
    bytes[24..].copy_from_slice(&number.to_be_bytes());
    FixedBytes::from(bytes)
}

async fn seed_verification_history(
    pool: &sqlx::PgPool,
    count: i64,
    stride: i64,
    legacy: bool,
) -> anyhow::Result<()> {
    sqlx::query(
        "INSERT INTO host_chain_blocks_valid
             (chain_id, block_number, block_hash, parent_hash, block_status)
         SELECT 42, n * $2, decode(lpad(to_hex(n * $2), 64, '0'), 'hex'),
                CASE WHEN NOT $3 THEN decode(lpad(to_hex(n * $2 - 1), 64, '0'), 'hex')
                     WHEN n % 2 = 0 THEN NULL
                     ELSE decode('', 'hex') END,
                'finalized'
         FROM generate_series(1::bigint, $1) n",
    )
    .bind(count)
    .bind(stride)
    .bind(legacy)
    .execute(pool)
    .await?;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn legacy_null_and_empty_parents_do_not_split_contiguous_history(
) -> anyhow::Result<()> {
    let (db, pool, _instance) = setup().await?;
    seed_verification_history(&pool, 6000, 1, true).await?;
    let mut checked = vec![];
    verify_finalized_history(&db, |n| {
        checked.push(n);
        async move { Ok(numbered_hash(n)) }
    })
    .await?;
    assert_eq!(
        checked,
        [6000],
        "cold upgrades must not fetch every legacy header"
    );
    assert!(!is_halted(&pool).await?);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn legacy_unknown_parents_preserve_gaps_and_known_mismatches(
) -> anyhow::Result<()> {
    let (db, pool, _instance) = setup().await?;
    seed_verification_history(&pool, 10, 1, true).await?;
    sqlx::query("DELETE FROM host_chain_blocks_valid WHERE block_number = 5")
        .execute(&pool)
        .await?;
    sqlx::query("UPDATE host_chain_blocks_valid SET parent_hash = $1 WHERE block_number = 8")
        .bind(numbered_hash(99).to_vec())
        .execute(&pool)
        .await?;
    let mut checked = vec![];
    let error = verify_finalized_history(&db, |n| {
        checked.push(n);
        async move { Ok(numbered_hash(if n == 4 { 44 } else { n })) }
    })
    .await
    .unwrap_err();
    assert_eq!(checked, [10, 7, 4]);
    assert!(error.to_string().contains("committed canonical history"));
    assert!(is_halted(&pool).await?);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn sparse_verification_keeps_successful_work_after_a_late_rpc_error(
) -> anyhow::Result<()> {
    let (db, pool, _instance) = setup().await?;
    seed_verification_history(&pool, 257, 2, false).await?;
    let error = verify_finalized_history(&db, |n| async move {
        anyhow::ensure!(n != 2, "injected RPC failure");
        Ok(numbered_hash(n))
    })
    .await
    .unwrap_err();
    assert!(error.to_string().contains("injected RPC failure"));
    let mut checked = vec![];
    verify_finalized_history(&db, |n| {
        checked.push(n);
        async move { Ok(numbered_hash(n)) }
    })
    .await?;
    assert_eq!(
        checked,
        [514, 2],
        "retry only the newest tip and unfinished work"
    );
    assert!(!is_halted(&pool).await?);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn sparse_verification_keeps_progress_when_another_replica_extends_history(
) -> anyhow::Result<()> {
    let (db, pool, _instance) = setup().await?;
    seed_verification_history(&pool, 257, 2, false).await?;
    let error = verify_finalized_history(&db, |n| {
        let pool = pool.clone();
        async move {
            if n == 2 {
                sqlx::query(
                    "INSERT INTO host_chain_blocks_valid
                     (chain_id, block_number, block_hash, parent_hash, block_status)
                     VALUES (42, 516, $1, $2, 'finalized')",
                )
                .bind(numbered_hash(516).to_vec())
                .bind(numbered_hash(515).to_vec())
                .execute(&pool)
                .await?;
            }
            Ok(numbered_hash(n))
        }
    })
    .await
    .unwrap_err();
    assert!(error.to_string().contains("changed during verification"));
    let mut checked = vec![];
    verify_finalized_history(&db, |n| {
        checked.push(n);
        async move { Ok(numbered_hash(n)) }
    })
    .await?;
    assert_eq!(
        checked,
        [516, 514],
        "the former newest tip must still be rechecked"
    );
    assert!(!is_halted(&pool).await?);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn interrupted_sparse_verification_keeps_completed_batches(
) -> anyhow::Result<()> {
    let (db, pool, _instance) = setup().await?;
    seed_verification_history(&pool, 257, 2, false).await?;
    let reached = tokio::sync::Notify::new();
    {
        let pending = verify_finalized_history(&db, |n| {
            let reached = &reached;
            async move {
                if n == 114 {
                    reached.notify_one();
                    std::future::pending::<()>().await;
                }
                Ok(numbered_hash(n))
            }
        });
        tokio::pin!(pending);
        tokio::select! {
            result = &mut pending => panic!("verification unexpectedly finished: {result:?}"),
            result = tokio::time::timeout(std::time::Duration::from_secs(10), reached.notified()) => result?,
        }
        // Drop the verification future mid-scan, without returning an RPC error.
    }
    let mut checked = vec![];
    verify_finalized_history(&db, |n| {
        checked.push(n);
        async move { Ok(numbered_hash(n)) }
    })
    .await?;
    assert_eq!(checked[0], 514, "the newest tip is never cached");
    assert!(
        checked.contains(&114),
        "unfinished work must still be checked"
    );
    assert!(
        checked.len() < 100,
        "completed batches must survive cancellation: {} RPCs",
        checked.len()
    );
    assert!(!is_halted(&pool).await?);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn removing_a_partially_verified_tip_invalidates_cached_work(
) -> anyhow::Result<()> {
    let (db, pool, _instance) = setup().await?;
    seed_verification_history(&pool, 257, 2, false).await?;
    assert!(verify_finalized_history(&db, |n| async move {
        anyhow::ensure!(n != 2, "injected RPC failure");
        Ok(numbered_hash(n))
    })
    .await
    .is_err());
    sqlx::query("DELETE FROM host_chain_blocks_valid WHERE block_number = 514")
        .execute(&pool)
        .await?;
    let mut checked = vec![];
    let error = verify_finalized_history(&db, |n| {
        checked.push(n);
        async move { Ok(numbered_hash(if n == 510 { 99 } else { n })) }
    })
    .await
    .unwrap_err();
    assert_eq!(checked, [512, 510]);
    assert!(error.to_string().contains("committed canonical history"));
    assert!(is_halted(&pool).await?);
    Ok(())
}
