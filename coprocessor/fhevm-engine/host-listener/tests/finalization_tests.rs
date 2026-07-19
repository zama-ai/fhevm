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

async fn seed_settled_height(db: &Database, height: i64) {
    let pool = db.pool().await;
    sqlx::query(
        "INSERT INTO coprocessor_settlement(chain_id, settled_height) \
         VALUES ($1, $2)",
    )
    .bind(CHAIN_ID as i64)
    .bind(height)
    .execute(&pool)
    .await
    .expect("seed coprocessor_settlement");
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
    seed_settled_height(&db, 1).await;

    // The poisoned RPC serves the fork chain for both heights.
    let fork = [(2u64, c2.clone()), (3u64, c3.clone())];
    update_finalized_blocks_aux(&mut db, 3, 0, 0, |n| {
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
    seed_settled_height(&db, 1).await;

    // Height 2 answers honestly, height 3 errors, height 4 serves the fork.
    let served = [(2u64, b2.clone()), (4u64, c4.clone())];
    update_finalized_blocks_aux(&mut db, 4, 0, 0, |n| {
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

/// The revalidation and pending finalization queues stop independently, and
/// the safety of that split rests on stored statuses: a pending block
/// immediately above a refused height anchors its parent-linkage check on the
/// stale finalized row still sitting there, so it must refuse on the
/// contradiction rather than finalize behind the refusal.
#[tokio::test]
#[serial(db)]
async fn pending_block_above_refused_height_fails_its_own_linkage_check() {
    let (mut db, _inst) = fresh_db(CHAIN_ID).await;
    let (a1, f2, c2, c3) = (b32(0xA1), b32(0xF2), b32(0xC2), b32(0xC3));

    seed_block(&db, 1, &a1, &b32(0xA0), "finalized").await;
    // Stale fork row finalized on the old branch; its canonical sibling c2
    // was never ingested (sparse catch-up skipped the empty canonical block).
    seed_block(&db, 2, &f2, &a1, "finalized").await;
    // Canonical child of the unstored c2, ingested pending.
    seed_block(&db, 3, &c3, &c2, "pending").await;
    seed_settled_height(&db, 1).await;

    // The RPC serves the canonical chain: height 2 refuses in the
    // revalidation queue (no stored row for c2), and height 3 sits in the
    // pending queue right above the refusal.
    let served = [(2u64, c2.clone()), (3u64, c3.clone())];
    update_finalized_blocks_aux(&mut db, 3, 0, 0, |n| {
        let hash = served
            .iter()
            .find(|(num, _)| *num == n)
            .map(|(_, h)| alloy::primitives::FixedBytes::<32>::from_slice(h))
            .expect("requested block");
        async move { Ok(hash) }
    })
    .await;

    assert_eq!(
        block_status(&db, &f2).await.as_deref(),
        Some("finalized"),
        "the stale row stays until its canonical sibling is re-ingested"
    );
    assert_eq!(
        block_status(&db, &c3).await.as_deref(),
        Some("pending"),
        "the pending block above the refusal must fail its own linkage \
         check against the stale finalized predecessor"
    );
}

/// Pruning removes only old finalized rows that nothing references: rows
/// referenced by branch state, orphaned markers, and everything within the
/// retention window stay.
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

    // Branch state referencing block 200 as producer.
    let pool = db.pool().await;
    sqlx::query(
        "INSERT INTO computations_branch
             (output_handle, dependencies, fhe_operation, is_scalar,
              dependence_chain_id, transaction_id, is_allowed, created_at,
              schedule_order, is_completed, host_chain_id, block_number,
              producer_block_hash)
         VALUES ($1, '{}', 0, FALSE, '\\x01', '\\x02', TRUE, NOW(), NOW(),
                 FALSE, $2, 200, $3)",
    )
    .bind(vec![0x55u8; 32])
    .bind(CHAIN_ID as i64)
    .bind(&old_ref)
    .execute(&pool)
    .await
    .expect("seed computations_branch");

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
        "rows referenced by branch state must survive"
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
    seed_settled_height(&db, 1).await;

    let chain = [(2u64, b2.clone()), (3u64, b3.clone())];
    update_finalized_blocks_aux(&mut db, 3, 0, 0, |n| {
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
