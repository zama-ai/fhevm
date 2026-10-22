//! Bounded startup inspection and continuity on every finalized admission.
mod ingest_fixtures;

use alloy::primitives::B256;
use alloy::rpc::types::Log;
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::chain_recovery::is_halted;
use host_listener::cmd::block_history::BlockSummary;
use host_listener::database::history_verification::verify_finalized_history;
use host_listener::database::ingest::{
    ingest_block_logs, ingest_with_history_recovery,
    update_finalized_blocks_aux, BlockLogs, HistoryGap, IngestResult,
    HISTORY_RECOVERY_BATCH,
};
use host_listener::database::tfhe_event_propagate::Database;
use ingest_fixtures::{setup, OPTIONS};
use serial_test::serial;

fn hash(n: u64) -> B256 {
    let mut bytes = [0; 32];
    bytes[24..].copy_from_slice(&n.to_be_bytes());
    B256::from(bytes)
}

fn block(n: u64) -> BlockLogs<Log> {
    BlockLogs {
        summary: BlockSummary {
            number: n,
            hash: hash(n),
            parent_hash: hash(n.saturating_sub(1)),
            timestamp: 1_790_000_000 + n,
        },
        logs: vec![],
        catchup: true,
        finalized: true,
    }
}

async fn admit(
    db: &mut Database,
    block: &BlockLogs<Log>,
) -> anyhow::Result<IngestResult> {
    Ok(ingest_block_logs(
        ChainId::try_from(42_u64)?,
        db,
        block,
        &None,
        &None,
        &None,
        &None,
        &None,
        OPTIONS,
    )
    .await?)
}

async fn recover(
    db: &mut Database,
    target: u64,
    fetched: &mut Vec<u64>,
    unavailable: Option<u64>,
) -> anyhow::Result<IngestResult> {
    ingest_with_history_recovery(
        ChainId::try_from(42_u64)?,
        db,
        &block(target),
        &None,
        &None,
        &None,
        &None,
        &None,
        OPTIONS,
        |n| {
            fetched.push(n);
            async move {
                anyhow::ensure!(unavailable != Some(n), "RPC unavailable");
                Ok(block(n))
            }
        },
    )
    .await
}

async fn height(pool: &sqlx::PgPool) -> anyhow::Result<i64> {
    Ok(sqlx::query_scalar(
        "SELECT MAX(block_number) FROM host_chain_blocks_valid
         WHERE chain_id = 42 AND block_status = 'finalized'",
    )
    .fetch_one(pool)
    .await?)
}

#[tokio::test]
#[serial(db)]
async fn empty_gap_blocks_are_recovered_before_the_original_delivery(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    admit(&mut db, &block(10)).await?.require_complete()?;
    assert_eq!(
        admit(&mut db, &block(13)).await?,
        IngestResult::NeedsHistory(HistoryGap { block_number: 11 })
    );
    assert_eq!(height(&pool).await?, 10);
    assert!(!is_halted(&pool).await?);
    let mut fetched = vec![];
    recover(&mut db, 13, &mut fetched, None)
        .await?
        .require_complete()?;
    assert_eq!(fetched, [11, 12]);
    assert_eq!(height(&pool).await?, 13);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn historical_backfill_attaches_to_a_known_successor(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    admit(&mut db, &block(15)).await?.require_complete()?;
    let mut fetched = vec![];
    recover(&mut db, 10, &mut fetched, None)
        .await?
        .require_complete()?;
    assert_eq!(fetched, [14, 13, 12, 11]);
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM host_chain_blocks_valid WHERE block_status='finalized'",
    ).fetch_one(&pool).await?;
    assert_eq!(count, 6);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn large_gaps_yield_without_acknowledging_the_original_block(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    admit(&mut db, &block(1)).await?.require_complete()?;
    let mut fetched = vec![];
    let first = recover(&mut db, 100, &mut fetched, None).await?;
    assert!(matches!(first, IngestResult::NeedsHistory(_)));
    assert_eq!(fetched.len(), HISTORY_RECOVERY_BATCH);
    assert_eq!(height(&pool).await?, 1 + HISTORY_RECOVERY_BATCH as i64);
    loop {
        let mut batch = vec![];
        let result = recover(&mut db, 100, &mut batch, None).await?;
        assert!(batch.len() <= HISTORY_RECOVERY_BATCH);
        fetched.extend(batch);
        if result == IngestResult::Complete {
            break;
        }
    }
    assert_eq!(fetched, (2..100).collect::<Vec<_>>());
    assert_eq!(height(&pool).await?, 100);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn unavailable_gap_header_preserves_progress_and_retries(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    admit(&mut db, &block(10)).await?.require_complete()?;
    let mut fetched = vec![];
    assert!(recover(&mut db, 14, &mut fetched, Some(12)).await.is_err());
    assert_eq!(height(&pool).await?, 11);
    assert!(!is_halted(&pool).await?);
    fetched.clear();
    recover(&mut db, 14, &mut fetched, None)
        .await?
        .require_complete()?;
    assert_eq!(fetched, [12, 13]);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn a_canonical_gap_cannot_hide_a_committed_fork() -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    let mut old = block(10);
    old.summary.hash = hash(999);
    admit(&mut db, &old).await?.require_complete()?;
    let mut fetched = vec![];
    let error = recover(&mut db, 13, &mut fetched, None).await.unwrap_err();
    assert!(error.to_string().contains("committed canonical history"));
    assert_eq!(fetched, [11]);
    assert_eq!(height(&pool).await?, 10);
    assert!(is_halted(&pool).await?);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn concurrent_first_deliveries_cannot_create_disconnected_anchors(
) -> anyhow::Result<()> {
    let (db, pool, _instance) = setup().await?;
    let mut a = db.clone();
    let mut b = db;
    let ten = block(10);
    let twenty = block(20);
    let (a, b) = tokio::join!(admit(&mut a, &ten), admit(&mut b, &twenty));
    let results = [a?, b?];
    assert_eq!(
        results
            .iter()
            .filter(|r| **r == IngestResult::Complete)
            .count(),
        1
    );
    assert_eq!(
        results
            .iter()
            .filter(|r| matches!(r, IngestResult::NeedsHistory(_)))
            .count(),
        1
    );
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM host_chain_blocks_valid")
            .fetch_one(&pool)
            .await?;
    assert_eq!(count, 1);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn deferred_finalization_recovers_links_before_advancing(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    admit(&mut db, &block(10)).await?.require_complete()?;
    let mut pending = block(13);
    pending.finalized = false;
    admit(&mut db, &pending).await?.require_complete()?;
    let result =
        update_finalized_blocks_aux(
            &mut db,
            30,
            5,
            |n| async move { Ok(hash(n)) },
        )
        .await?;
    assert_eq!(result.missing_canonical_block, Some(11));
    assert_eq!(height(&pool).await?, 10);
    let mut fetched = vec![];
    recover(&mut db, 13, &mut fetched, None)
        .await?
        .require_complete()?;
    assert_eq!(fetched, [11, 12]);
    assert_eq!(height(&pool).await?, 13);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn backfill_repairs_a_legacy_successors_unknown_parent(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    admit(&mut db, &block(15)).await?.require_complete()?;
    sqlx::query("UPDATE host_chain_blocks_valid SET parent_hash=NULL")
        .execute(&pool)
        .await?;
    let mut fetched = vec![];
    recover(&mut db, 14, &mut fetched, None)
        .await?
        .require_complete()?;
    assert_eq!(fetched, [15]);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn initial_audit_is_bounded_by_heights_and_later_passes_only_check_the_tip(
) -> anyhow::Result<()> {
    let (db, pool, _instance) = setup().await?;
    sqlx::query(
        "INSERT INTO host_chain_blocks_valid
         (chain_id, block_number, block_hash, parent_hash, block_status)
         SELECT 42, n, decode(lpad(to_hex(n),64,'0'),'hex'),
                decode(lpad(to_hex(n-1),64,'0'),'hex'), 'finalized'
         FROM generate_series(2::bigint,12000,2) n",
    )
    .execute(&pool)
    .await?;
    let mut checked = vec![];
    verify_finalized_history(&db, |n| {
        checked.push(n);
        async move { Ok(if n < 11001 { hash(0) } else { hash(n) }) }
    })
    .await?;
    assert_eq!(
        checked,
        (5501_u64..6001).rev().map(|n| n * 2).collect::<Vec<_>>()
    );
    checked.clear();
    verify_finalized_history(&db, |n| {
        checked.push(n);
        async move { Ok(hash(n)) }
    })
    .await?;
    assert_eq!(checked, [12000]);
    // Older history remains a trusted baseline, even if a historical RPC
    // would disagree. This deliberate scope limit must not silently expand.
    assert!(!is_halted(&pool).await?);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn initial_audit_checks_the_inclusive_lower_boundary(
) -> anyhow::Result<()> {
    let (db, pool, _instance) = setup().await?;
    sqlx::query(
        "INSERT INTO host_chain_blocks_valid
         (chain_id, block_number, block_hash, parent_hash, block_status)
         SELECT 42, n, decode(lpad(to_hex(n),64,'0'),'hex'),
                decode(lpad(to_hex(n-1),64,'0'),'hex'), 'finalized'
         FROM unnest(ARRAY[1000::bigint,1001,2000]) n",
    )
    .execute(&pool)
    .await?;
    let mut checked = vec![];
    let error = verify_finalized_history(&db, |n| {
        checked.push(n);
        async move { Ok(if n == 1001 { hash(999) } else { hash(n) }) }
    })
    .await
    .unwrap_err();
    assert!(error.to_string().contains("committed canonical history"));
    assert_eq!(checked, [2000, 1001]);
    assert!(is_halted(&pool).await?);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn old_exact_replay_does_not_force_a_whole_history_walk(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    admit(&mut db, &block(10)).await?.require_complete()?;
    // A pre-upgrade disconnected segment, outside the bounded audit.
    sqlx::query(
        "INSERT INTO host_chain_blocks_valid
         (chain_id, block_number, block_hash, parent_hash, block_status)
         VALUES (42, 5000, $1, $2, 'finalized')",
    )
    .bind(hash(5000).to_vec())
    .bind(hash(4999).to_vec())
    .execute(&pool)
    .await?;
    let mut fetched = vec![];
    recover(&mut db, 10, &mut fetched, None)
        .await?
        .require_complete()?;
    assert!(fetched.is_empty());
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn backfill_uses_the_nearby_successor_instead_of_an_ancient_anchor(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    admit(&mut db, &block(1)).await?.require_complete()?;
    sqlx::query(
        "INSERT INTO host_chain_blocks_valid
         (chain_id, block_number, block_hash, parent_hash, block_status)
         VALUES (42, 10000, $1, $2, 'finalized')",
    )
    .bind(hash(10000).to_vec())
    .bind(hash(9999).to_vec())
    .execute(&pool)
    .await?;
    let mut fetched = vec![];
    recover(&mut db, 9998, &mut fetched, None)
        .await?
        .require_complete()?;
    assert_eq!(fetched, [9999]);
    Ok(())
}
