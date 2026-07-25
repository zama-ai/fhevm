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

/// A node's two ingestion paths can transiently disagree about canonicality
/// (flapping LB, consumer stream fed from a different backend): the fork side
/// finalizes first and wrongly orphans the canonical row. Once the oracles
/// reconcile (by-number answers canonical again), the orphaned canonical row
/// must RESURRECT — it is positively anchored on its finalized parent — and
/// finalizing it orphans the stale fork branch recursively. Without
/// resurrection the settlement frontier wedges below the height forever.
#[tokio::test]
#[serial(db)]
async fn wrongly_orphaned_canonical_row_resurrects_with_finalized_anchor() {
    let (mut db, _inst) = fresh_db(CHAIN_ID).await;
    let (a1, c2, f2, c3, f3) =
        (b32(0xA1), b32(0xC2), b32(0xF2), b32(0xC3), b32(0xF3));

    seed_block(&db, 1, &a1, &b32(0xA0), "finalized").await;
    // The fork oracle won the race: fork blocks finalized, canonical sibling
    // wrongly orphaned (it was ingested by the second oracle).
    seed_block(&db, 2, &f2, &a1, "finalized").await;
    seed_block(&db, 2, &c2, &a1, "orphaned").await;
    seed_block(&db, 3, &f3, &f2, "finalized").await;
    seed_block(&db, 3, &c3, &c2, "pending").await;
    seed_settled_height(&db, 1).await;

    // Oracles reconciled: by-number now serves the canonical chain.
    let chain = [(2u64, c2.clone()), (3u64, c3.clone())];
    update_finalized_blocks_aux(&mut db, 3, 0, 0, |n| {
        let hash = chain
            .iter()
            .find(|(num, _)| *num == n)
            .map(|(_, h)| alloy::primitives::FixedBytes::<32>::from_slice(h))
            .expect("requested block");
        async move { Ok(hash) }
    })
    .await;

    assert_eq!(
        block_status(&db, &c2).await.as_deref(),
        Some("finalized"),
        "the wrongly-orphaned canonical row must resurrect on its anchor"
    );
    for (hash, what) in [(&f2, "stale fork block"), (&f3, "stale fork child")] {
        assert_eq!(
            block_status(&db, hash).await.as_deref(),
            Some("orphaned"),
            "{what} must be orphaned once the canonical sibling finalizes"
        );
    }
    assert_eq!(
        block_status(&db, &c3).await.as_deref(),
        Some("finalized"),
        "the canonical child finalizes on top of the resurrected parent"
    );
}

/// A height where EVERY row is orphaned (both siblings lost a finalization
/// race during split ingestion) is invisible to the pending queue and has no
/// finalized row for plain revalidation — yet it is not a bare gap, so it
/// holds settlement. The revalidation queue must select such heights so the
/// anchored canonical row can resurrect; otherwise the wedge is terminal.
#[tokio::test]
#[serial(db)]
async fn all_orphaned_height_is_revalidated_and_resurrects() {
    let (mut db, _inst) = fresh_db(CHAIN_ID).await;
    let (a1, c2, f2, c3) = (b32(0xA1), b32(0xC2), b32(0xF2), b32(0xC3));

    seed_block(&db, 1, &a1, &b32(0xA0), "finalized").await;
    seed_block(&db, 2, &c2, &a1, "orphaned").await;
    seed_block(&db, 2, &f2, &a1, "orphaned").await;
    seed_block(&db, 3, &c3, &c2, "pending").await;
    seed_settled_height(&db, 1).await;

    let chain = [(2u64, c2.clone()), (3u64, c3.clone())];
    update_finalized_blocks_aux(&mut db, 3, 0, 0, |n| {
        let hash = chain
            .iter()
            .find(|(num, _)| *num == n)
            .map(|(_, h)| alloy::primitives::FixedBytes::<32>::from_slice(h))
            .expect("requested block");
        async move { Ok(hash) }
    })
    .await;

    assert_eq!(
        block_status(&db, &c2).await.as_deref(),
        Some("finalized"),
        "the canonical row at an all-orphaned height must resurrect"
    );
    assert_eq!(
        block_status(&db, &f2).await.as_deref(),
        Some("orphaned"),
        "the fork sibling stays orphaned"
    );
    assert_eq!(
        block_status(&db, &c3).await.as_deref(),
        Some("finalized"),
        "the child finalizes on top of the resurrected parent"
    );
}

/// Resurrection demands a POSITIVE finalized-parent anchor — an orphaned row
/// whose parent is missing (or not finalized) stays refused even when the
/// by-number answer names it: the vacuous pass that fresh finalization
/// enjoys must not extend to resurrection, or a poisoned RPC could revive
/// arbitrary orphaned fork blocks behind a gap.
#[tokio::test]
#[serial(db)]
async fn orphaned_row_without_anchor_is_not_resurrected() {
    let (mut db, _inst) = fresh_db(CHAIN_ID).await;
    let (o3, s3, x2) = (b32(0xD3), b32(0xE3), b32(0x0F));

    // Orphaned row at height 3 whose recorded parent (x2) has no stored row,
    // plus a stale finalized sibling so the revalidation queue selects the
    // height and actually calls finalization with the orphaned row's hash.
    seed_block(&db, 3, &o3, &x2, "orphaned").await;
    seed_block(&db, 3, &s3, &b32(0x0E), "finalized").await;
    seed_settled_height(&db, 1).await;

    let served = [(3u64, o3.clone())];
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
        block_status(&db, &o3).await.as_deref(),
        Some("orphaned"),
        "no vacuous resurrection without a finalized parent anchor"
    );
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

/// A height can transiently carry TWO finalized rows: a refused pass inserts
/// the canonical row as finalized without orphaning the stale sibling. The
/// stale sibling must not veto a child anchored on the matching finalized
/// parent — otherwise every ancestor above the double height refuses forever
/// (orphaning only runs on success, so the contradiction never clears).
#[tokio::test]
#[serial(db)]
async fn stale_finalized_sibling_does_not_veto_anchored_child() {
    let (mut db, _inst) = fresh_db(CHAIN_ID).await;
    let (a1, a2, s2, a3) = (b32(0xA1), b32(0xA2), b32(0x52), b32(0xA3));

    seed_block(&db, 1, &a1, &b32(0xA0), "finalized").await;
    // Both the canonical block and a stale branch sibling are finalized at
    // height 2 (the stale one from a pre-switch chain view).
    seed_block(&db, 2, &a2, &a1, "finalized").await;
    seed_block(&db, 2, &s2, &a1, "finalized").await;
    // Canonical child anchored on the matching finalized parent a2.
    seed_block(&db, 3, &a3, &a2, "pending").await;
    seed_settled_height(&db, 1).await;

    // Settlement lag 2 keeps height 2 out of the revalidation window, so the
    // child at height 3 is finalized while BOTH height-2 rows are still
    // finalized — the exact double-row state the veto exception is for.
    let chain = [(2u64, a2.clone()), (3u64, a3.clone())];
    update_finalized_blocks_aux(&mut db, 3, 0, 2, |n| {
        let hash = chain
            .iter()
            .find(|(num, _)| *num == n)
            .map(|(_, h)| alloy::primitives::FixedBytes::<32>::from_slice(h))
            .expect("requested block");
        async move { Ok(hash) }
    })
    .await;

    assert_eq!(
        block_status(&db, &a3).await.as_deref(),
        Some("finalized"),
        "a child anchored on the matching finalized parent must not be \
         vetoed by the stale sibling"
    );
    assert_eq!(
        block_status(&db, &s2).await.as_deref(),
        Some("finalized"),
        "the stale sibling is cleaned up when its height is revalidated, \
         not by the child's pass"
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

/// The row AT the settlement frontier is the parent-linkage anchor for
/// finalizing (and resurrecting) height frontier+1. When settlement wedges
/// far below the finalized head, retention-based pruning must not delete
/// it — otherwise the linkage check degrades to a vacuous pass and a
/// wrongly-orphaned frontier+1 row can never resurrect.
#[tokio::test]
#[serial(db)]
async fn prune_keeps_the_settlement_frontier_anchor_row() {
    let (db, _inst) = fresh_db(CHAIN_ID).await;
    let (below_frontier, at_frontier, above_frontier) =
        (b32(0x11), b32(0x12), b32(0x13));

    seed_block(&db, 499, &below_frontier, &b32(0), "finalized").await;
    seed_block(&db, 500, &at_frontier, &b32(0), "finalized").await;
    seed_block(&db, 501, &above_frontier, &b32(0), "finalized").await;
    seed_settled_height(&db, 500).await;

    // Finalized head far ahead: plain retention would prune everything here.
    let pruned = db
        .prune_finalized_block_history(50_000)
        .await
        .expect("prune");
    assert_eq!(pruned, 1, "only the row strictly below the frontier goes");

    assert_eq!(block_status(&db, &below_frontier).await, None);
    assert_eq!(
        block_status(&db, &at_frontier).await.as_deref(),
        Some("finalized"),
        "the frontier row is the linkage anchor for frontier+1 and must survive"
    );
    assert_eq!(
        block_status(&db, &above_frontier).await.as_deref(),
        Some("finalized"),
        "unsettled rows above the frontier are live revalidation evidence"
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
