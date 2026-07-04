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
#[tokio::test]
#[serial(db)]
async fn finalization_refuses_hash_with_mismatched_parent() {
    let (mut db, _inst) = fresh_db(CHAIN_ID).await;
    let (a1, b2, c2, x0) = (b32(0xA1), b32(0xB2), b32(0xC2), b32(0x0F));

    seed_block(&db, 1, &a1, &b32(0xA0), "finalized").await;
    seed_block(&db, 2, &b2, &a1, "pending").await; // true chain
    seed_block(&db, 2, &c2, &x0, "pending").await; // fork sibling

    let evil = c2.clone();
    update_finalized_blocks_aux(&mut db, 2, 0, |_n| {
        let h = alloy::primitives::FixedBytes::<32>::from_slice(&evil);
        async move { Ok(h) }
    })
    .await;

    assert_eq!(
        block_status(&db, &c2).await.as_deref(),
        Some("pending"),
        "contradicting sibling must not be finalized"
    );
    assert_eq!(
        block_status(&db, &b2).await.as_deref(),
        Some("pending"),
        "true sibling must not be orphaned by a refused finalization"
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
