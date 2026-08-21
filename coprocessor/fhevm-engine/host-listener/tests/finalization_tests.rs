//! Finalization safety: the by-number RPC hash is only trusted when the
//! recorded block's parent linkage does not contradict the finalized chain,
//! because finalization destructively cleans up the orphaned siblings.

use fhevm_engine_common::chain_id::ChainId;
use host_listener::database::ingest::update_finalized_blocks_aux;
use host_listener::database::tfhe_event_propagate::Database;
use serial_test::serial;
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};

const CHAIN_ID: u64 = 4242;

async fn fresh_db(chain_id: u64) -> (Database, DBInstance) {
    let inst = setup_test_db(ImportMode::None).await.expect("test db");
    let db =
        Database::new(&inst.db_url, ChainId::try_from(chain_id).unwrap(), 16)
            .await
            .expect("database");
    (db, inst)
}

async fn seed_block(
    db: &Database,
    number: i64,
    hash: &[u8],
    parent: &[u8],
    status: &str,
) {
    let pool = db.pool().await;
    sqlx::query(
        "INSERT INTO host_chain_blocks_valid
             (chain_id, block_hash, block_number, parent_hash, block_status)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(CHAIN_ID as i64)
    .bind(hash)
    .bind(number)
    .bind(parent)
    .bind(status)
    .execute(&pool)
    .await
    .expect("seed host_chain_blocks_valid");
}

async fn block_status(db: &Database, hash: &[u8]) -> Option<String> {
    let pool = db.pool().await;
    sqlx::query_scalar::<_, String>(
        "SELECT block_status FROM host_chain_blocks_valid
         WHERE chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID as i64)
    .bind(hash)
    .fetch_optional(&pool)
    .await
    .expect("status query")
}

fn b32(seed: u8) -> Vec<u8> {
    vec![seed; 32]
}

/// A stale/poisoned RPC answers block 2 with the fork sibling whose recorded
/// parent contradicts the finalized block 1: nothing may be finalized or
/// orphaned, so the true sibling stays available for a later, honest pass.
/// The batch must also STOP at the refusal — block 3's linkage check would
/// pass vacuously (no finalized predecessor at 2), letting the same poisoned
/// RPC finalize the fork right behind the refusal and orphan the true chain.
#[tokio::test]
#[serial(db)]
async fn finalization_refuses_mismatched_parent_and_stops_batch() {
    let (mut db, _inst) = fresh_db(CHAIN_ID).await;
    let (a1, b2, c2, x0) = (b32(0xA1), b32(0xB2), b32(0xC2), b32(0x0F));
    let (b3, c3) = (b32(0xB3), b32(0xC3));

    seed_block(&db, 1, &a1, &b32(0xA0), "finalized").await;
    seed_block(&db, 2, &b2, &a1, "pending").await; // true chain
    seed_block(&db, 2, &c2, &x0, "pending").await; // fork sibling
    seed_block(&db, 3, &b3, &b2, "pending").await; // true chain
    seed_block(&db, 3, &c3, &c2, "pending").await; // fork child

    // The poisoned RPC serves the fork chain for both heights.
    let fork = [(2u64, c2.clone()), (3u64, c3.clone())];
    update_finalized_blocks_aux(&mut db, 3, 0, |n| {
        let hash = fork
            .iter()
            .find(|(num, _)| *num == n)
            .map(|(_, h)| alloy::primitives::FixedBytes::<32>::from_slice(h))
            .expect("requested block");
        async move { Ok(hash) }
    })
    .await;

    for (hash, what) in [
        (&c2, "contradicting sibling"),
        (&b2, "true sibling"),
        (&c3, "fork child behind the refusal"),
        (&b3, "true child"),
    ] {
        assert_eq!(
            block_status(&db, hash).await.as_deref(),
            Some("pending"),
            "{what} must stay pending after a refused finalization"
        );
    }
}

/// An RPC fetch failure mid-batch must STOP the batch, not skip the height:
/// behind the gap the parent-linkage check has no finalized predecessor and
/// passes vacuously, so a poisoned RPC that errors at height 3 and serves a
/// fork child at height 4 would finalize the fork and orphan the true chain.
/// The prefix fetched before the gap is still safe and finalizes.
#[tokio::test]
#[serial(db)]
async fn finalization_stops_batch_at_fetch_failure() {
    let (mut db, _inst) = fresh_db(CHAIN_ID).await;
    let (a1, b2) = (b32(0xA1), b32(0xB2));
    let (b3, c3, b4, c4) = (b32(0xB3), b32(0xC3), b32(0xB4), b32(0xC4));

    seed_block(&db, 1, &a1, &b32(0xA0), "finalized").await;
    seed_block(&db, 2, &b2, &a1, "pending").await; // true chain
    seed_block(&db, 3, &b3, &b2, "pending").await; // true chain
    seed_block(&db, 3, &c3, &b32(0x0F), "pending").await; // fork sibling
    seed_block(&db, 4, &b4, &b3, "pending").await; // true chain
    seed_block(&db, 4, &c4, &c3, "pending").await; // fork child

    // Height 2 answers honestly, height 3 errors, height 4 serves the fork.
    let served = [(2u64, b2.clone()), (4u64, c4.clone())];
    update_finalized_blocks_aux(&mut db, 4, 0, |n| {
        let hash = served
            .iter()
            .find(|(num, _)| *num == n)
            .map(|(_, h)| alloy::primitives::FixedBytes::<32>::from_slice(h));
        async move {
            hash.ok_or_else(|| anyhow::anyhow!("rpc unavailable for {n}"))
        }
    })
    .await;

    assert_eq!(
        block_status(&db, &b2).await.as_deref(),
        Some("finalized"),
        "prefix before the gap must still finalize"
    );
    for (hash, what) in [
        (&b3, "true block at the gap"),
        (&c3, "fork sibling at the gap"),
        (&b4, "true child behind the gap"),
        (&c4, "fork child behind the gap"),
    ] {
        assert_eq!(
            block_status(&db, hash).await.as_deref(),
            Some("pending"),
            "{what} must stay pending after a fetch failure"
        );
    }
}

/// Pruning removes only old finalized rows that nothing references: rows
/// referenced by bridge/fallback state, orphaned markers, and everything
/// within the retention window stay.
#[tokio::test]
#[serial(db)]
async fn prune_keeps_referenced_orphaned_and_recent_rows() {
    let (db, _inst) = fresh_db(CHAIN_ID).await;
    let (old_unref, old_ref, old_orphaned, recent) =
        (b32(0x01), b32(0x02), b32(0x03), b32(0x04));

    seed_block(&db, 100, &old_unref, &b32(0), "finalized").await;
    seed_block(&db, 200, &old_ref, &b32(0), "finalized").await;
    seed_block(&db, 300, &old_orphaned, &b32(0), "orphaned").await;
    seed_block(&db, 19_000, &recent, &b32(0), "finalized").await;

    // Fallback-grant observation referencing block 200 by hash.
    let pool = db.pool().await;
    sqlx::query(
        "INSERT INTO fallback_granted_events
             (dst_chain_id, dst_handle, plaintext, block_number, block_hash,
              transaction_id)
         VALUES ($1, $2, $3, 200, $4, NULL)",
    )
    .bind(CHAIN_ID as i64)
    .bind(vec![0x55u8; 32])
    .bind(vec![0u8; 32])
    .bind(&old_ref)
    .execute(&pool)
    .await
    .expect("seed fallback_granted_events");

    // Retention window is 10_000 blocks below the finalized head (20_000):
    // rows below 10_000 are candidates.
    let pruned = db
        .prune_finalized_block_history(20_000)
        .await
        .expect("prune");
    assert_eq!(pruned, 1, "exactly the unreferenced old row is pruned");

    assert_eq!(block_status(&db, &old_unref).await, None);
    assert_eq!(
        block_status(&db, &old_ref).await.as_deref(),
        Some("finalized"),
        "rows referenced by bridge/fallback state must survive"
    );
    assert_eq!(
        block_status(&db, &old_orphaned).await.as_deref(),
        Some("orphaned"),
        "orphan markers are never pruned"
    );
    assert_eq!(
        block_status(&db, &recent).await.as_deref(),
        Some("finalized"),
        "rows within the retention window must survive"
    );
}

/// The honest answer links to the finalized predecessor: it finalizes and the
/// fork sibling is orphaned. Also covers multi-block batches finalizing in
/// ascending order (block 3's linkage check needs block 2 finalized first
/// within the same transaction).
#[tokio::test]
#[serial(db)]
async fn finalization_accepts_linked_chain_and_orphans_sibling() {
    let (mut db, _inst) = fresh_db(CHAIN_ID).await;
    let (a1, b2, c2, b3) = (b32(0xA1), b32(0xB2), b32(0xC2), b32(0xB3));

    seed_block(&db, 1, &a1, &b32(0xA0), "finalized").await;
    seed_block(&db, 2, &b2, &a1, "pending").await;
    seed_block(&db, 2, &c2, &b32(0x0F), "pending").await;
    seed_block(&db, 3, &b3, &b2, "pending").await;

    let chain = [(2u64, b2.clone()), (3u64, b3.clone())];
    update_finalized_blocks_aux(&mut db, 3, 0, |n| {
        let hash = chain
            .iter()
            .find(|(num, _)| *num == n)
            .map(|(_, h)| alloy::primitives::FixedBytes::<32>::from_slice(h))
            .expect("requested block");
        async move { Ok(hash) }
    })
    .await;

    assert_eq!(block_status(&db, &b2).await.as_deref(), Some("finalized"));
    assert_eq!(block_status(&db, &b3).await.as_deref(), Some("finalized"));
    assert_eq!(
        block_status(&db, &c2).await.as_deref(),
        Some("orphaned"),
        "fork sibling of a finalized block is orphaned"
    );
}

/// A persistent allow observed AFTER the block that computed the handle must
/// reach the earlier computation row — but only through finality: the fork
/// sibling's identical allow is retracted with its branch. Application also
/// re-arms the owning chain and repairs a legacy completed-but-unpersisted
/// row so the recompute actually persists.
#[tokio::test]
#[serial(db)]
async fn late_allow_propagates_at_finality_and_retracts_on_orphan() {
    use host_listener::cmd::block_history::BlockSummary;

    let (mut db, _inst) = fresh_db(CHAIN_ID).await;
    let (a1, b2, c2) = (b32(0xA1), b32(0xB2), b32(0xC2));
    let handle = b32(0x77);
    let chain = b32(0xD1);
    let pool = db.pool().await;

    // Block 1 computed `handle` without an allow: the row is pending,
    // non-allowed, and its chain has settled. Also simulate the pre-mask
    // legacy state this repair targets: completed with no persisted
    // ciphertext.
    sqlx::query(
        "INSERT INTO computations
             (output_handle, dependencies, fhe_operation, is_scalar,
              transaction_id, dependence_chain_id, is_allowed, is_completed,
              host_chain_id, block_number, operand_boundary_mask)
         VALUES ($1, '{}', 0, false, $2, $3, false, true, $4, 1, $5)",
    )
    .bind(&handle)
    .bind(b32(0xAA))
    .bind(&chain)
    .bind(CHAIN_ID as i64)
    .bind(vec![0u8; 32])
    .execute(&pool)
    .await
    .expect("seed computation");
    sqlx::query(
        "INSERT INTO dependence_chain (dependence_chain_id, status)
         VALUES ($1, 'processed')",
    )
    .bind(&chain)
    .execute(&pool)
    .await
    .expect("seed chain");

    seed_block(&db, 1, &a1, &b32(0xA0), "finalized").await;
    seed_block(&db, 2, &b2, &a1, "pending").await; // true chain, allows handle
    seed_block(&db, 2, &c2, &a1, "pending").await; // fork sibling, same allow

    // Both height-2 blocks observe the allow; neither may apply it yet.
    for hash in [&b2, &c2] {
        let summary = BlockSummary {
            number: 2,
            hash: alloy::primitives::FixedBytes::from_slice(hash),
            parent_hash: alloy::primitives::FixedBytes::from_slice(&a1),
            timestamp: 0,
        };
        let mut tx =
            db.new_transaction().await.expect("tx").expect("live stack");
        let recorded = db
            .record_late_allows(
                &mut tx,
                &summary,
                std::slice::from_ref(&handle),
            )
            .await
            .expect("record");
        tx.commit().await.expect("commit");
        assert_eq!(recorded, 1, "late allow recorded for {hash:?}");
    }
    let is_allowed: bool = sqlx::query_scalar(
        "SELECT is_allowed FROM computations WHERE output_handle = $1",
    )
    .bind(&handle)
    .fetch_one(&pool)
    .await
    .expect("pre-finality row");
    assert!(!is_allowed, "recording must not apply the allow");

    // Finalize height 2 on the true chain: b2 applies, c2 retracts.
    update_finalized_blocks_aux(&mut db, 2, 0, |n| {
        let (a1, b2) = (a1.clone(), b2.clone());
        async move {
            let hash = match n {
                1 => a1,
                2 => b2,
                _ => panic!("unexpected block {n}"),
            };
            Ok(alloy::primitives::FixedBytes::from_slice(&hash))
        }
    })
    .await;

    assert_eq!(block_status(&db, &b2).await.as_deref(), Some("finalized"));
    assert_eq!(block_status(&db, &c2).await.as_deref(), Some("orphaned"));

    let (is_allowed, is_completed): (bool, bool) = sqlx::query_as(
        "SELECT is_allowed, is_completed FROM computations WHERE output_handle = $1",
    )
    .bind(&handle)
    .fetch_one(&pool)
    .await
    .expect("post-finality row");
    assert!(is_allowed, "finalized allow reaches the earlier row");
    assert!(
        !is_completed,
        "completed-but-unpersisted legacy row resets for recompute"
    );

    let chain_status: String = sqlx::query_scalar(
        "SELECT status FROM dependence_chain WHERE dependence_chain_id = $1",
    )
    .bind(&chain)
    .fetch_one(&pool)
    .await
    .expect("chain row");
    assert_eq!(chain_status, "updated", "owning chain re-armed");

    let leftovers: i64 =
        sqlx::query_scalar("SELECT count(*) FROM late_allow_propagation")
            .fetch_one(&pool)
            .await
            .expect("leftover count");
    assert_eq!(leftovers, 0, "applied and orphaned records both cleaned up");
}

/// A same-block allow is already stamped on the row at ingest; it must not
/// be recorded as a late allow.
#[tokio::test]
#[serial(db)]
async fn same_block_allow_is_not_recorded_as_late() {
    use host_listener::cmd::block_history::BlockSummary;

    let (db, _inst) = fresh_db(CHAIN_ID).await;
    let handle = b32(0x78);
    let pool = db.pool().await;
    sqlx::query(
        "INSERT INTO computations
             (output_handle, dependencies, fhe_operation, is_scalar,
              transaction_id, is_allowed, host_chain_id, block_number,
              operand_boundary_mask)
         VALUES ($1, '{}', 0, false, $2, true, $3, 1, $4)",
    )
    .bind(&handle)
    .bind(b32(0xAB))
    .bind(CHAIN_ID as i64)
    .bind(vec![0u8; 32])
    .execute(&pool)
    .await
    .expect("seed allowed computation");

    let summary = BlockSummary {
        number: 1,
        hash: alloy::primitives::FixedBytes::from_slice(&b32(0xB1)),
        parent_hash: alloy::primitives::FixedBytes::from_slice(&b32(0xA0)),
        timestamp: 0,
    };
    let mut tx = db.new_transaction().await.expect("tx").expect("live stack");
    let recorded = db
        .record_late_allows(&mut tx, &summary, &[handle])
        .await
        .expect("record");
    tx.commit().await.expect("commit");
    assert_eq!(recorded, 0);
}
