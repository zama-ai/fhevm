use super::*;
use serial_test::serial;
use sqlx::{PgPool, Row};
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};

const CHAIN_ID: i64 = 137;

async fn setup_pool() -> (DBInstance, PgPool) {
    let instance = setup_test_db(ImportMode::None)
        .await
        .expect("create manifest state database");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(3)
        .connect(instance.db_url())
        .await
        .expect("connect manifest state database");
    (instance, pool)
}

async fn insert_host_block(
    pool: &PgPool,
    block_number: i64,
    block_hash: &[u8],
    parent_hash: &[u8],
    status: &str,
) {
    sqlx::query(
        "INSERT INTO host_chain_blocks_valid \
         (chain_id, block_hash, parent_hash, block_number, block_status) \
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(CHAIN_ID)
    .bind(block_hash)
    .bind(parent_hash)
    .bind(block_number)
    .bind(status)
    .execute(pool)
    .await
    .expect("insert host block");
}

async fn insert_manifest_state(
    pool: &PgPool,
    block_number: i64,
    block_hash: &[u8],
    parent_hash: &[u8],
) {
    sqlx::query(
        "INSERT INTO block_manifest_state \
         (host_chain_id, block_number, block_hash, parent_block_hash, publication_cadence) \
         VALUES ($1, $2, $3, $4, 30)",
    )
    .bind(CHAIN_ID)
    .bind(block_number)
    .bind(block_hash)
    .bind(parent_hash)
    .execute(pool)
    .await
    .expect("insert manifest state");
}

async fn load_pending_block(pool: &PgPool, block_hash: &[u8]) -> PendingBlock {
    let row = sqlx::query(
        "SELECT host_chain_id, block_number, block_hash, parent_block_hash,
                publication_cadence, block_content_digest, block_handle_count,
                manifest_revision, manifest_publisher, manifest_digest, manifest_published
           FROM block_manifest_state
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(block_hash)
    .fetch_one(pool)
    .await
    .expect("load pending manifest block");
    PendingBlock {
        host_chain_id: row.get("host_chain_id"),
        block_number: row.get("block_number"),
        block_hash: row.get("block_hash"),
        parent_block_hash: row.get("parent_block_hash"),
        publication_cadence: row.get("publication_cadence"),
        block_content_digest: row.get("block_content_digest"),
        block_handle_count: row.get("block_handle_count"),
        manifest_revision: row.get("manifest_revision"),
        manifest_publisher: row.get("manifest_publisher"),
        manifest_digest: row.get("manifest_digest"),
        manifest_published: row.get("manifest_published"),
    }
}

async fn manifest_state_exists(pool: &PgPool, block_hash: &[u8]) -> bool {
    sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(
             SELECT 1 FROM block_manifest_state
              WHERE host_chain_id = $1 AND block_hash = $2
         )",
    )
    .bind(CHAIN_ID)
    .bind(block_hash)
    .fetch_one(pool)
    .await
    .expect("check manifest state")
}

#[tokio::test]
#[serial(db)]
async fn discovers_live_descendants_and_closes_finalized_parent() {
    let (_instance, pool) = setup_pool().await;
    let parent = vec![0x40; 32];
    let child = vec![0x41; 32];
    let grandchild = vec![0x42; 32];
    let orphan = vec![0x43; 32];

    insert_host_block(&pool, 40, &parent, &[0x3f; 32], "finalized").await;
    insert_host_block(&pool, 41, &child, &parent, "pending").await;
    insert_host_block(&pool, 42, &grandchild, &child, "pending").await;
    insert_host_block(&pool, 41, &orphan, &parent, "orphaned").await;
    insert_manifest_state(&pool, 40, &parent, &[0x3f; 32]).await;

    assert_eq!(
        discover_known_children(&pool)
            .await
            .expect("discover direct children"),
        1
    );
    let parent_closed = sqlx::query_scalar::<_, bool>(
        "SELECT child_block_discovery_closed
           FROM block_manifest_state
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(&parent)
    .fetch_one(&pool)
    .await
    .expect("load parent discovery state");
    assert!(
        parent_closed,
        "finalized parent discovery must close after copying children"
    );
    assert!(manifest_state_exists(&pool, &child).await);
    assert!(!manifest_state_exists(&pool, &grandchild).await);
    assert!(!manifest_state_exists(&pool, &orphan).await);

    let parent = load_pending_block(&pool, &parent).await;
    let mut trx = pool.begin().await.expect("begin descendant discovery");
    assert_eq!(
        discover_block_children(&mut trx, &parent)
            .await
            .expect("discover recursive descendants"),
        1
    );
    trx.commit().await.expect("commit descendant discovery");

    assert!(manifest_state_exists(&pool, &grandchild).await);
    assert!(!manifest_state_exists(&pool, &orphan).await);
}
