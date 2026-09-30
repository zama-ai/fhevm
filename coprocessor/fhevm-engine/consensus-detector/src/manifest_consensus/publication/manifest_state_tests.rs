use super::{block_discovery::*, manifest_builder::*, publication_status::*};
use alloy_primitives::{Address, B256};
use serial_test::serial;
use sqlx::{postgres::PgConnectOptions, PgPool, Row};
use std::time::Duration;
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};

const CHAIN_ID: i64 = 137;
const ANVIL_CHAIN_ID: i64 = 31337;

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

async fn stack_pool(db_url: &str, search_path: &str) -> PgPool {
    let options = db_url
        .parse::<PgConnectOptions>()
        .expect("parse stack database URL")
        .options([("search_path", search_path)]);
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(3)
        .connect_with(options)
        .await
        .expect("connect stack-local manifest database")
}

async fn create_green_discovery_schema(pool: &PgPool) {
    sqlx::query("CREATE SCHEMA gcs_manifest_test")
        .execute(pool)
        .await
        .expect("create Green test schema");
    for table in [
        "host_chain_blocks_valid",
        "handle_producer_block",
        "ciphertext_digest",
        "blue_green_consensus_epoch",
    ] {
        sqlx::query(&format!(
            "CREATE TABLE gcs_manifest_test.{table} \
             (LIKE public.{table} INCLUDING ALL)"
        ))
        .execute(pool)
        .await
        .expect("create Green discovery table");
    }
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
        "SELECT consensus_epoch, host_chain_id, block_number, block_hash, parent_block_hash,
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
        consensus_epoch: row.get("consensus_epoch"),
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

async fn insert_producer_block(pool: &PgPool, block_number: i64, block_hash: &[u8], handle: &[u8]) {
    sqlx::query(
        "INSERT INTO handle_producer_block (
             host_chain_id, producer_block_number, producer_block_hash, handle
         ) VALUES ($1, $2, $3, $4)",
    )
    .bind(CHAIN_ID)
    .bind(block_number)
    .bind(block_hash)
    .bind(handle)
    .execute(pool)
    .await
    .expect("insert producer block");
}

async fn insert_completed_sns_digest(
    pool: &PgPool,
    _block_number: i64,
    _block_hash: &[u8],
    handle: &[u8],
) {
    sqlx::query(
        "INSERT INTO ciphertext_digest (
             host_chain_id, key_id_gw, handle, ciphertext, ciphertext128,
             ciphertext128_format
         ) VALUES ($1, $2, $3, $4, $5, 11)",
    )
    .bind(CHAIN_ID)
    .bind(vec![0x11_u8; 32])
    .bind(handle)
    .bind(vec![0x64_u8; 32])
    .bind(vec![0x80_u8; 32])
    .execute(pool)
    .await
    .expect("insert computed ciphertext digests");
}

#[tokio::test]
#[serial(db)]
async fn detector_seeds_candidate_early_but_waits_for_every_allowed_handle() {
    let (_instance, pool) = setup_pool().await;
    let block_number = 42;
    let block_hash = vec![0x42; 32];
    let parent_hash = vec![0x41; 32];
    let first_handle = vec![0x21; 32];
    let second_handle = vec![0x22; 32];

    insert_host_block(&pool, block_number, &block_hash, &parent_hash, "finalized").await;
    insert_producer_block(&pool, block_number, &block_hash, &first_handle).await;
    insert_producer_block(&pool, block_number, &block_hash, &second_handle).await;
    insert_completed_sns_digest(&pool, block_number, &block_hash, &first_handle).await;

    assert_eq!(
        discover_blocks(&pool)
            .await
            .expect("discover first candidate"),
        1
    );
    assert_eq!(
        discover_blocks(&pool)
            .await
            .expect("replay candidate discovery"),
        0
    );

    let block = load_pending_block(&pool, &block_hash).await;
    let mut trx = pool.begin().await.expect("begin readiness check");
    assert!(!is_block_manifest_ready(&mut trx, &block)
        .await
        .expect("check incomplete block"));
    trx.rollback().await.expect("rollback readiness check");

    insert_completed_sns_digest(&pool, block_number, &block_hash, &second_handle).await;

    let mut trx = pool.begin().await.expect("begin final readiness check");
    assert!(is_block_manifest_ready(&mut trx, &block)
        .await
        .expect("check completed block"));
    trx.rollback()
        .await
        .expect("rollback final readiness check");
}

#[tokio::test]
#[serial(db)]
async fn detector_caps_upgrade_discovery_at_the_consensus_epoch_start() {
    let (_instance, pool) = setup_pool().await;
    let old_block_hash = vec![0x52; 32];
    let old_handle = vec![0x32; 32];
    let start_block_hash = vec![0x60; 32];
    let current_block_hash = vec![0x62; 32];
    let current_handle = vec![0x33; 32];

    insert_host_block(&pool, 52, &old_block_hash, &[0x51; 32], "pending").await;
    insert_producer_block(&pool, 52, &old_block_hash, &old_handle).await;
    insert_completed_sns_digest(&pool, 52, &old_block_hash, &old_handle).await;
    insert_host_block(&pool, 62, &current_block_hash, &[0x61; 32], "pending").await;
    insert_producer_block(&pool, 62, &current_block_hash, &current_handle).await;
    insert_completed_sns_digest(&pool, 62, &current_block_hash, &current_handle).await;

    sqlx::query(
        "INSERT INTO consensus_epoch_history (
             consensus_epoch, proposal_id, proposal_block, stack_version, outcome
         ) VALUES ('1', $1, 50, 'test-green', 'pending')",
    )
    .bind(vec![0x91_u8; 32])
    .execute(&pool)
    .await
    .expect("allocate Green consensus_epoch");
    sqlx::query(
        "INSERT INTO consensus_epoch_block_window (
             consensus_epoch, host_chain_id, start_block, consensus_deadline_block
         ) VALUES ('1', $1, 60, 70)",
    )
    .bind(CHAIN_ID)
    .execute(&pool)
    .await
    .expect("store Green consensus_epoch window");
    sqlx::query(
        "UPDATE blue_green_consensus_epoch
            SET consensus_epoch = '1', updated_at = NOW()
          WHERE singleton = TRUE",
    )
    .execute(&pool)
    .await
    .expect("select Green consensus_epoch");

    assert_eq!(
        discover_blocks(&pool)
            .await
            .expect("wait for the consensus_epoch start block"),
        0
    );
    assert!(!manifest_state_exists(&pool, &current_block_hash).await);

    insert_host_block(&pool, 60, &start_block_hash, &[0x59; 32], "pending").await;
    assert_eq!(
        discover_blocks(&pool)
            .await
            .expect("discover in-consensus_epoch catch-up"),
        2
    );
    assert!(!manifest_state_exists(&pool, &old_block_hash).await);
    assert!(manifest_state_exists(&pool, &start_block_hash).await);
    assert!(manifest_state_exists(&pool, &current_block_hash).await);
    let consensus_epoch: String = sqlx::query_scalar(
        "SELECT consensus_epoch FROM block_manifest_state
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(&current_block_hash)
    .fetch_one(&pool)
    .await
    .expect("load selected candidate consensus_epoch");
    assert_eq!(consensus_epoch, "1");
    assert_eq!(
        pending_chain_ids(&pool)
            .await
            .expect("select in-consensus_epoch publication work"),
        vec![CHAIN_ID]
    );
}

#[tokio::test]
#[serial(db)]
async fn initial_consensus_epoch_bootstraps_at_the_latest_finalized_block() {
    let (_instance, pool) = setup_pool().await;
    let historical_hash = vec![0x42; 32];
    let historical_handle = vec![0x21; 32];
    let bootstrap_hash = vec![0x45; 32];
    let head_hash = vec![0x50; 32];

    insert_host_block(&pool, 42, &historical_hash, &[0x41; 32], "pending").await;
    insert_producer_block(&pool, 42, &historical_hash, &historical_handle).await;
    insert_completed_sns_digest(&pool, 42, &historical_hash, &historical_handle).await;
    insert_host_block(&pool, 50, &head_hash, &[0x49; 32], "pending").await;

    // A pending head can still be reorged away, and with it every lineage
    // rooted there, so the first pass waits for a finalized seed.
    assert_eq!(discover_blocks(&pool).await.unwrap(), 0);

    insert_host_block(&pool, 45, &bootstrap_hash, &[0x44; 32], "finalized").await;
    assert_eq!(discover_blocks(&pool).await.unwrap(), 1);
    assert!(!manifest_state_exists(&pool, &historical_hash).await);
    assert!(manifest_state_exists(&pool, &bootstrap_hash).await);
    assert!(!manifest_state_exists(&pool, &head_hash).await);

    // The persisted bootstrap block is the consensus_epoch-zero lower bound; a
    // restart must not widen the arbitrary initial history into older blocks.
    assert_eq!(discover_blocks(&pool).await.unwrap(), 0);
    assert!(!manifest_state_exists(&pool, &historical_hash).await);
}

async fn open_upgrade_window(pool: &PgPool, start_block: i64) {
    sqlx::query(
        "INSERT INTO consensus_epoch_history (
             consensus_epoch, proposal_id, proposal_block, stack_version, outcome
         ) VALUES ('1', $1, $2, 'test-green', 'pending')",
    )
    .bind(vec![0x91_u8; 32])
    .bind(start_block - 10)
    .execute(pool)
    .await
    .expect("allocate Green consensus_epoch");
    sqlx::query(
        "INSERT INTO consensus_epoch_block_window (
             consensus_epoch, host_chain_id, start_block, consensus_deadline_block
         ) VALUES ('1', $1, $2, $3)",
    )
    .bind(CHAIN_ID)
    .bind(start_block)
    .bind(start_block + 10)
    .execute(pool)
    .await
    .expect("store Green consensus_epoch window");
    sqlx::query(
        "UPDATE blue_green_consensus_epoch
            SET consensus_epoch = '1', updated_at = NOW()
          WHERE singleton = TRUE",
    )
    .execute(pool)
    .await
    .expect("select Green consensus_epoch");
}

async fn set_host_status(pool: &PgPool, block_hash: &[u8], status: &str) {
    sqlx::query(
        "UPDATE host_chain_blocks_valid SET block_status = $3
          WHERE chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(block_hash)
    .bind(status)
    .execute(pool)
    .await
    .expect("update host block status");
}

/// A reorg of the upgrade `start_block` leaves no tracked parent for the
/// replacement root, so parent-to-child discovery alone never reaches the
/// winning fork. Each pass re-inserts the root height until it is finalized.
#[tokio::test]
#[serial(db)]
async fn upgrade_discovery_reroots_after_the_start_block_is_reorged() {
    let (_instance, pool) = setup_pool().await;
    let lost_root = vec![0x60; 32];
    let lost_child = vec![0x61; 32];
    let new_root = vec![0x6a; 32];
    let new_child = vec![0x6b; 32];

    open_upgrade_window(&pool, 60).await;
    insert_host_block(&pool, 60, &lost_root, &[0x59; 32], "pending").await;
    insert_host_block(&pool, 61, &lost_child, &lost_root, "pending").await;
    assert_eq!(discover_blocks(&pool).await.unwrap(), 1);
    assert_eq!(discover_children(&pool).await.unwrap(), 1);

    // The winning fork forks below the start block and carries no handles in
    // the producer catch-up window.
    set_host_status(&pool, &lost_root, "orphaned").await;
    set_host_status(&pool, &lost_child, "orphaned").await;
    insert_host_block(&pool, 60, &new_root, &[0x5a; 32], "pending").await;
    insert_host_block(&pool, 61, &new_child, &new_root, "pending").await;
    assert_eq!(discover_children(&pool).await.unwrap(), 0);

    assert_eq!(discover_blocks(&pool).await.unwrap(), 1);
    assert!(manifest_state_exists(&pool, &new_root).await);
    assert_eq!(discover_children(&pool).await.unwrap(), 1);
    assert!(manifest_state_exists(&pool, &new_child).await);

    // Once a tracked root is finalized, no reorg can replace it and the root
    // height is no longer rechecked.
    set_host_status(&pool, &new_root, "finalized").await;
    let stray = vec![0x6c; 32];
    insert_host_block(&pool, 60, &stray, &[0x5b; 32], "pending").await;
    assert_eq!(discover_blocks(&pool).await.unwrap(), 0);
    assert!(!manifest_state_exists(&pool, &stray).await);
}

#[tokio::test]
#[serial(db)]
async fn anvil_publication_cadence_overlay_is_stored_on_insert() {
    let (_instance, pool) = setup_pool().await;
    let block_hash = vec![0x31_u8; 32];
    sqlx::query(
        "INSERT INTO host_chain_blocks_valid \
         (chain_id, block_hash, parent_hash, block_number, block_status) \
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(ANVIL_CHAIN_ID)
    .bind(&block_hash)
    .bind(vec![0x30_u8; 32])
    .bind(50_i64)
    .bind("finalized")
    .execute(&pool)
    .await
    .expect("insert anvil host block");

    let consensus_epoch = crate::manifest_consensus::storage::active::load_consensus_epoch(&pool)
        .await
        .expect("load consensus_epoch");
    let overrides = publication_cadence_overrides([(ANVIL_CHAIN_ID, 1)]).unwrap();
    assert_eq!(
        discover_blocks_for_consensus_epoch(&pool, &consensus_epoch, &overrides)
            .await
            .expect("discover anvil candidate"),
        1
    );
    let cadence: i64 = sqlx::query_scalar(
        "SELECT publication_cadence FROM block_manifest_state
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(ANVIL_CHAIN_ID)
    .bind(&block_hash)
    .fetch_one(&pool)
    .await
    .expect("load stored anvil cadence");
    assert_eq!(cadence, 1);
}

#[tokio::test]
#[serial(db)]
async fn established_discovery_frontier_revisits_only_five_blocks() {
    let (_instance, pool) = setup_pool().await;
    let before_overlap_hash = vec![0x64; 32];
    let overlap_hash = vec![0x65; 32];
    let after_frontier_hash = vec![0x6f; 32];

    insert_manifest_state(&pool, 100, &[0x60; 32], &[0x59; 32]).await;
    insert_manifest_state(&pool, 110, &[0x6e; 32], &[0x6d; 32]).await;
    insert_host_block(&pool, 104, &before_overlap_hash, &[0x63; 32], "pending").await;
    insert_host_block(&pool, 105, &overlap_hash, &[0x64; 32], "pending").await;
    insert_host_block(&pool, 111, &after_frontier_hash, &[0x6e; 32], "pending").await;
    insert_producer_block(&pool, 104, &before_overlap_hash, &[0x24; 32]).await;
    insert_producer_block(&pool, 105, &overlap_hash, &[0x25; 32]).await;
    insert_producer_block(&pool, 111, &after_frontier_hash, &[0x26; 32]).await;

    assert_eq!(discover_blocks(&pool).await.unwrap(), 1);
    assert!(!manifest_state_exists(&pool, &before_overlap_hash).await);
    assert!(manifest_state_exists(&pool, &overlap_hash).await);
    assert!(!manifest_state_exists(&pool, &after_frontier_hash).await);
}

#[tokio::test]
#[serial(db)]
async fn blue_and_green_discover_the_same_block_without_consensus_epoch_collision() {
    let (instance, admin_pool) = setup_pool().await;
    create_green_discovery_schema(&admin_pool).await;
    let blue = stack_pool(instance.db_url(), "public").await;
    let green = stack_pool(instance.db_url(), "gcs_manifest_test,public").await;
    let block_number = 62;
    let block_hash = vec![0x62; 32];
    let parent_hash = vec![0x61; 32];
    let handle = vec![0x42; 32];

    sqlx::query(
        "INSERT INTO consensus_epoch_history (
             consensus_epoch, proposal_id, proposal_block, stack_version, outcome
         ) VALUES ('1', $1, 60, 'test-green', 'pending')",
    )
    .bind(vec![0xa1_u8; 32])
    .execute(&admin_pool)
    .await
    .expect("allocate Green consensus_epoch");
    sqlx::query(
        "INSERT INTO consensus_epoch_block_window (
             consensus_epoch, host_chain_id, start_block, consensus_deadline_block
         ) VALUES ('1', $1, 62, 70)",
    )
    .bind(CHAIN_ID)
    .execute(&admin_pool)
    .await
    .expect("store Green consensus_epoch window");
    sqlx::query(
        "INSERT INTO gcs_manifest_test.blue_green_consensus_epoch
             (singleton, consensus_epoch) VALUES (TRUE, '1')",
    )
    .execute(&admin_pool)
    .await
    .expect("select Green consensus_epoch");

    for stack in [&blue, &green] {
        insert_host_block(stack, block_number, &block_hash, &parent_hash, "finalized").await;
        insert_producer_block(stack, block_number, &block_hash, &handle).await;
        insert_completed_sns_digest(stack, block_number, &block_hash, &handle).await;
    }

    assert_eq!(discover_blocks(&blue).await.unwrap(), 1);
    assert_eq!(discover_blocks(&green).await.unwrap(), 1);
    assert_eq!(discover_blocks(&blue).await.unwrap(), 0);
    assert_eq!(discover_blocks(&green).await.unwrap(), 0);

    let consensus_epochs = sqlx::query_scalar::<_, String>(
        "SELECT consensus_epoch
           FROM public.block_manifest_state
          WHERE host_chain_id = $1 AND block_hash = $2
          ORDER BY consensus_epoch",
    )
    .bind(CHAIN_ID)
    .bind(&block_hash)
    .fetch_all(&admin_pool)
    .await
    .expect("load both stack candidates");
    assert_eq!(
        consensus_epochs,
        vec!["1".to_string(), "legacy".to_string()]
    );
    assert_eq!(pending_chain_ids(&blue).await.unwrap(), vec![CHAIN_ID]);
    assert_eq!(pending_chain_ids(&green).await.unwrap(), vec![CHAIN_ID]);
}

#[tokio::test]
#[serial(db)]
async fn exhausts_and_clears_current_manifest_publication_retries() {
    let (_instance, pool) = setup_pool().await;
    let block_hash = vec![0x61; 32];
    insert_manifest_state(&pool, 30, &block_hash, &[0x60; 32]).await;
    sqlx::query(
        "UPDATE block_manifest_state \
            SET block_content_digest = $3, block_handle_count = 0 \
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(&block_hash)
    .bind(vec![0x62_u8; 32])
    .execute(&pool)
    .await
    .expect("seal manifest state");

    let block = load_pending_block(&pool, &block_hash).await;
    record_manifest_publication_error(&pool, &block, "S3 object rejected", 2, 1_000_000)
        .await
        .expect("record publication error");
    let error = sqlx::query(
        "SELECT publication_error_count, publication_last_error, \
                publication_next_retry_at IS NULL AS retry_exhausted \
           FROM block_manifest_state \
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(&block_hash)
    .fetch_one(&pool)
    .await
    .expect("load recorded publication error");
    assert_eq!(error.get::<i64, _>("publication_error_count"), 1);
    assert_eq!(
        error
            .get::<Option<String>, _>("publication_last_error")
            .as_deref(),
        Some("S3 object rejected")
    );
    assert!(!error.get::<bool, _>("retry_exhausted"));

    record_manifest_publication_error(&pool, &block, "S3 object rejected", 2, 1_000_000)
        .await
        .expect("exhaust publication retries");
    let exhausted = sqlx::query(
        "SELECT publication_error_count, publication_next_retry_at IS NULL AS retry_exhausted \
           FROM block_manifest_state \
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(&block_hash)
    .fetch_one(&pool)
    .await
    .expect("load exhausted publication retries");
    assert_eq!(exhausted.get::<i64, _>("publication_error_count"), 2);
    assert!(exhausted.get::<bool, _>("retry_exhausted"));
    assert!(pending_chain_ids(&pool)
        .await
        .expect("list retryable manifest chains")
        .is_empty());

    let later_hash = vec![0x68; 32];
    insert_manifest_state(&pool, 60, &later_hash, &block_hash).await;
    sqlx::query(
        "UPDATE block_manifest_state \
            SET block_content_digest = $3, block_handle_count = 0 \
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(&later_hash)
    .bind(vec![0x69_u8; 32])
    .execute(&pool)
    .await
    .expect("seal later publication point");
    let mut selection = pool.begin().await.expect("begin later selection");
    let selected =
        lock_next_block_to_progress(&mut selection, CHAIN_ID, &ManifestProgressCursor::start())
            .await
            .expect("select after exhausted publication")
            .expect("later publication remains selectable");
    assert_eq!(selected.block_hash, later_hash);
    selection
        .rollback()
        .await
        .expect("rollback later selection");

    let mut trx = pool.begin().await.expect("begin manifest publication");
    mark_manifest_published(
        &mut trx,
        &block,
        Address::repeat_byte(0x63),
        B256::repeat_byte(0x65),
    )
    .await
    .expect("mark manifest published");
    trx.commit().await.expect("commit manifest publication");

    let state = sqlx::query(
        "SELECT publication_error_count, publication_last_error \
           FROM block_manifest_state \
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(&block_hash)
    .fetch_one(&pool)
    .await
    .expect("load published manifest state");
    assert_eq!(state.get::<i64, _>("publication_error_count"), 0);
    assert!(state
        .get::<Option<String>, _>("publication_last_error")
        .is_none());
}

#[tokio::test]
#[serial(db)]
async fn successful_publication_retries_the_skipped_predecessor_once() {
    let (_instance, pool) = setup_pool().await;
    let skipped_hash = vec![0x61; 32];
    let published_hash = vec![0x68; 32];
    insert_manifest_state(&pool, 30, &skipped_hash, &[0x60; 32]).await;
    insert_manifest_state(&pool, 60, &published_hash, &skipped_hash).await;
    sqlx::query(
        "UPDATE block_manifest_state
            SET block_content_digest = $3, block_handle_count = 0
          WHERE host_chain_id = $1 AND block_hash IN ($2, $4)",
    )
    .bind(CHAIN_ID)
    .bind(&skipped_hash)
    .bind(vec![0x62_u8; 32])
    .bind(&published_hash)
    .execute(&pool)
    .await
    .expect("seal cadence points");
    let skipped = load_pending_block(&pool, &skipped_hash).await;
    record_manifest_publication_error(&pool, &skipped, "S3 object rejected", 1, 1_000_000)
        .await
        .expect("exhaust skipped predecessor");

    let published = load_pending_block(&pool, &published_hash).await;
    let mut trx = pool.begin().await.expect("begin successor publication");
    mark_manifest_published(
        &mut trx,
        &published,
        Address::repeat_byte(0x63),
        B256::repeat_byte(0x65),
    )
    .await
    .expect("mark successor published");
    retry_skipped_predecessor_once(&mut trx, &published, 1)
        .await
        .expect("retry skipped predecessor");
    trx.commit().await.expect("commit successor publication");

    let retry_scheduled = sqlx::query_scalar::<_, bool>(
        "SELECT publication_next_retry_at IS NOT NULL
           FROM block_manifest_state
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(&skipped_hash)
    .fetch_one(&pool)
    .await
    .expect("load skipped predecessor retry");
    assert!(retry_scheduled);
}

#[tokio::test]
#[serial(db)]
async fn later_success_retries_the_nearest_skip_then_older_ones_after_it_works() {
    let (_instance, pool) = setup_pool().await;
    let skip_5 = vec![0x05; 32];
    let skip_10 = vec![0x0a; 32];
    let published_15 = vec![0x0f; 32];
    insert_manifest_state(&pool, 30, &skip_5, &[0x04; 32]).await;
    insert_manifest_state(&pool, 60, &skip_10, &skip_5).await;
    insert_manifest_state(&pool, 90, &published_15, &skip_10).await;
    sqlx::query(
        "UPDATE block_manifest_state
            SET block_content_digest = $2, block_handle_count = 0
          WHERE host_chain_id = $1",
    )
    .bind(CHAIN_ID)
    .bind(vec![0x11_u8; 32])
    .execute(&pool)
    .await
    .expect("seal cadence points");
    for hash in [&skip_5, &skip_10] {
        let skipped = load_pending_block(&pool, hash).await;
        record_manifest_publication_error(&pool, &skipped, "S3 object rejected", 1, 1_000_000)
            .await
            .expect("exhaust skip");
    }

    let published = load_pending_block(&pool, &published_15).await;
    let mut trx = pool.begin().await.expect("begin peel nearest skip");
    mark_manifest_published(
        &mut trx,
        &published,
        Address::repeat_byte(0x63),
        B256::repeat_byte(0x65),
    )
    .await
    .expect("mark 15 published");
    retry_skipped_predecessor_once(&mut trx, &published, 1)
        .await
        .expect("retry nearest skip");
    trx.commit().await.expect("commit peel nearest skip");

    let retry_10 = sqlx::query_scalar::<_, bool>(
        "SELECT publication_next_retry_at IS NOT NULL
           FROM block_manifest_state
          WHERE block_hash = $1",
    )
    .bind(&skip_10)
    .fetch_one(&pool)
    .await
    .expect("load nearest skip retry");
    let retry_5 = sqlx::query_scalar::<_, bool>(
        "SELECT publication_next_retry_at IS NOT NULL
           FROM block_manifest_state
          WHERE block_hash = $1",
    )
    .bind(&skip_5)
    .fetch_one(&pool)
    .await
    .expect("load older skip retry");
    assert!(retry_10, "15 should peel the nearest skip (10)");
    assert!(!retry_5, "15 should not peel 5 until 10 succeeds");

    let recovered_10 = load_pending_block(&pool, &skip_10).await;
    let mut trx = pool.begin().await.expect("begin recovered 10");
    mark_manifest_published(
        &mut trx,
        &recovered_10,
        Address::repeat_byte(0x63),
        B256::repeat_byte(0x75),
    )
    .await
    .expect("mark recovered 10 published");
    retry_skipped_predecessor_once(&mut trx, &recovered_10, 1)
        .await
        .expect("peel older skip after 10 works");
    trx.commit().await.expect("commit recovered 10");

    let retry_5_after = sqlx::query_scalar::<_, bool>(
        "SELECT publication_next_retry_at IS NOT NULL
           FROM block_manifest_state
          WHERE block_hash = $1",
    )
    .bind(&skip_5)
    .fetch_one(&pool)
    .await
    .expect("load older skip after 10 worked");
    assert!(retry_5_after, "recovered 10 should peel 5");
}

#[tokio::test]
#[serial(db)]
async fn extra_skip_attempt_is_not_granted_again_after_bound_plus_one() {
    let (_instance, pool) = setup_pool().await;
    let skipped_hash = vec![0x61; 32];
    let published_hash = vec![0x68; 32];
    insert_manifest_state(&pool, 30, &skipped_hash, &[0x60; 32]).await;
    insert_manifest_state(&pool, 60, &published_hash, &skipped_hash).await;
    sqlx::query(
        "UPDATE block_manifest_state
            SET block_content_digest = $3, block_handle_count = 0
          WHERE host_chain_id = $1 AND block_hash IN ($2, $4)",
    )
    .bind(CHAIN_ID)
    .bind(&skipped_hash)
    .bind(vec![0x62_u8; 32])
    .bind(&published_hash)
    .execute(&pool)
    .await
    .expect("seal cadence points");
    let skipped = load_pending_block(&pool, &skipped_hash).await;
    record_manifest_publication_error(&pool, &skipped, "S3 object rejected", 1, 1_000_000)
        .await
        .expect("exhaust skip");
    record_manifest_publication_error(&pool, &skipped, "S3 object rejected", 1, 1_000_000)
        .await
        .expect("consume extra attempt");

    let published = load_pending_block(&pool, &published_hash).await;
    let mut trx = pool.begin().await.expect("begin later success");
    mark_manifest_published(
        &mut trx,
        &published,
        Address::repeat_byte(0x63),
        B256::repeat_byte(0x65),
    )
    .await
    .expect("mark successor published");
    retry_skipped_predecessor_once(&mut trx, &published, 1)
        .await
        .expect("refuse a second extra attempt");
    trx.commit().await.expect("commit later success");

    let retry_scheduled = sqlx::query_scalar::<_, bool>(
        "SELECT publication_next_retry_at IS NOT NULL
           FROM block_manifest_state
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(&skipped_hash)
    .fetch_one(&pool)
    .await
    .expect("load skip after bound plus one");
    assert!(!retry_scheduled);
}

#[tokio::test]
#[serial(db)]
async fn definitive_publication_error_exhausts_past_the_skip_peel() {
    let (_instance, pool) = setup_pool().await;
    let skipped_hash = vec![0x61; 32];
    let published_hash = vec![0x68; 32];
    insert_manifest_state(&pool, 30, &skipped_hash, &[0x60; 32]).await;
    insert_manifest_state(&pool, 60, &published_hash, &skipped_hash).await;
    sqlx::query(
        "UPDATE block_manifest_state
            SET block_content_digest = $3, block_handle_count = 0
          WHERE host_chain_id = $1 AND block_hash IN ($2, $4)",
    )
    .bind(CHAIN_ID)
    .bind(&skipped_hash)
    .bind(vec![0x62_u8; 32])
    .bind(&published_hash)
    .execute(&pool)
    .await
    .expect("seal cadence points");
    let skipped = load_pending_block(&pool, &skipped_hash).await;
    exhaust_manifest_publication_error(&pool, &skipped, "column decode failed", 1)
        .await
        .expect("exhaust definitive error");

    let exhausted = sqlx::query(
        "SELECT publication_error_count, publication_next_retry_at IS NULL AS retry_exhausted
           FROM block_manifest_state
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(&skipped_hash)
    .fetch_one(&pool)
    .await
    .expect("load definitive exhaustion");
    assert_eq!(exhausted.get::<i64, _>("publication_error_count"), 2);
    assert!(exhausted.get::<bool, _>("retry_exhausted"));

    let published = load_pending_block(&pool, &published_hash).await;
    let mut trx = pool.begin().await.expect("begin later success");
    mark_manifest_published(
        &mut trx,
        &published,
        Address::repeat_byte(0x63),
        B256::repeat_byte(0x65),
    )
    .await
    .expect("mark successor published");
    retry_skipped_predecessor_once(&mut trx, &published, 1)
        .await
        .expect("definitive skip must not be peeled");
    trx.commit().await.expect("commit later success");

    let retry_scheduled = sqlx::query_scalar::<_, bool>(
        "SELECT publication_next_retry_at IS NOT NULL
           FROM block_manifest_state
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(&skipped_hash)
    .fetch_one(&pool)
    .await
    .expect("load skip after definitive exhaust");
    assert!(!retry_scheduled);
}

/// Seals the given rows and exhausts the skips (one attempt, `max_attempts = 1`).
async fn seal_and_skip(pool: &PgPool, sealed: &[&[u8]], skipped: &[&[u8]]) {
    for hash in sealed {
        sqlx::query(
            "UPDATE block_manifest_state
                SET block_content_digest = $3, block_handle_count = 0
              WHERE host_chain_id = $1 AND block_hash = $2",
        )
        .bind(CHAIN_ID)
        .bind(*hash)
        .bind(vec![0x11_u8; 32])
        .execute(pool)
        .await
        .expect("seal cadence point");
    }
    for hash in skipped {
        let block = load_pending_block(pool, hash).await;
        record_manifest_publication_error(pool, &block, "S3 object rejected", 1, 1_000_000)
            .await
            .expect("exhaust skip");
    }
}

async fn publish_and_peel(pool: &PgPool, block_hash: &[u8]) {
    let published = load_pending_block(pool, block_hash).await;
    let mut trx = pool.begin().await.expect("begin publication");
    mark_manifest_published(
        &mut trx,
        &published,
        Address::repeat_byte(0x63),
        B256::repeat_byte(0x65),
    )
    .await
    .expect("mark published");
    retry_skipped_predecessor_once(&mut trx, &published, 1)
        .await
        .expect("peel a skip");
    trx.commit().await.expect("commit publication");
}

async fn skip_state(pool: &PgPool, block_hash: &[u8]) -> (bool, i64) {
    let row = sqlx::query(
        "SELECT publication_next_retry_at IS NOT NULL AS retry, publication_error_count
           FROM block_manifest_state
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(block_hash)
    .fetch_one(pool)
    .await
    .expect("load skip state");
    (row.get("retry"), row.get("publication_error_count"))
}

#[tokio::test]
#[serial(db)]
async fn fork_skip_below_the_published_block_gets_its_extra_attempt() {
    let (_instance, pool) = setup_pool().await;
    let fork_60 = vec![0x3c; 32];
    let canonical_60 = vec![0x4c; 32];
    let published_90 = vec![0x5a; 32];
    insert_manifest_state(&pool, 60, &fork_60, &[0x01; 32]).await;
    insert_manifest_state(&pool, 60, &canonical_60, &[0x02; 32]).await;
    insert_manifest_state(&pool, 90, &published_90, &canonical_60).await;
    seal_and_skip(
        &pool,
        &[&fork_60, &canonical_60, &published_90],
        &[&fork_60],
    )
    .await;
    let canonical = load_pending_block(&pool, &canonical_60).await;
    let mut trx = pool.begin().await.expect("begin canonical 60");
    mark_manifest_published(
        &mut trx,
        &canonical,
        Address::repeat_byte(0x63),
        B256::repeat_byte(0x66),
    )
    .await
    .expect("mark canonical 60 published");
    trx.commit().await.expect("commit canonical 60");

    publish_and_peel(&pool, &published_90).await;

    assert_eq!(skip_state(&pool, &fork_60).await, (true, 1));
}

#[tokio::test]
#[serial(db)]
async fn orphaned_skip_is_exhausted_and_the_next_skip_is_retried() {
    let (_instance, pool) = setup_pool().await;
    let skip_30 = vec![0x1e; 32];
    let orphan_60 = vec![0x3c; 32];
    let canonical_60 = vec![0x4c; 32];
    let published_90 = vec![0x5a; 32];
    insert_manifest_state(&pool, 30, &skip_30, &[0x01; 32]).await;
    insert_manifest_state(&pool, 60, &orphan_60, &skip_30).await;
    insert_manifest_state(&pool, 60, &canonical_60, &skip_30).await;
    insert_manifest_state(&pool, 90, &published_90, &canonical_60).await;
    insert_host_block(&pool, 60, &orphan_60, &skip_30, "orphaned").await;
    seal_and_skip(
        &pool,
        &[&skip_30, &orphan_60, &canonical_60, &published_90],
        &[&skip_30, &orphan_60, &canonical_60],
    )
    .await;
    sqlx::query(
        "UPDATE block_manifest_state SET publication_error_count = 2
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(&canonical_60)
    .execute(&pool)
    .await
    .expect("canonical 60 already used its extra attempt");

    publish_and_peel(&pool, &published_90).await;

    assert_eq!(skip_state(&pool, &orphan_60).await, (false, 2));
    assert_eq!(skip_state(&pool, &skip_30).await, (true, 1));
}

#[tokio::test]
#[serial(db)]
async fn skip_locked_by_another_worker_is_not_retried() {
    let (_instance, pool) = setup_pool().await;
    let skip_30 = vec![0x1e; 32];
    let published_60 = vec![0x3c; 32];
    insert_manifest_state(&pool, 30, &skip_30, &[0x01; 32]).await;
    insert_manifest_state(&pool, 60, &published_60, &skip_30).await;
    seal_and_skip(&pool, &[&skip_30, &published_60], &[&skip_30]).await;
    let mut holder = pool.begin().await.expect("begin lock holder");
    sqlx::query(
        "SELECT 1 FROM block_manifest_state
          WHERE host_chain_id = $1 AND block_hash = $2 FOR UPDATE",
    )
    .bind(CHAIN_ID)
    .bind(&skip_30)
    .execute(holder.as_mut())
    .await
    .expect("lock the skip");

    tokio::time::timeout(
        Duration::from_secs(5),
        publish_and_peel(&pool, &published_60),
    )
    .await
    .expect("peeling must not wait for a locked skip");
    holder.rollback().await.expect("release lock holder");

    assert_eq!(skip_state(&pool, &skip_30).await, (false, 1));
}

#[tokio::test]
#[serial(db)]
async fn transient_manifest_publication_errors_exhaust_the_finite_retry_budget() {
    let (_instance, pool) = setup_pool().await;
    let block_hash = vec![0x66; 32];
    insert_manifest_state(&pool, 60, &block_hash, &[0x65; 32]).await;
    sqlx::query(
        "UPDATE block_manifest_state \
            SET block_content_digest = $3, block_handle_count = 0 \
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(&block_hash)
    .bind(vec![0x67_u8; 32])
    .execute(&pool)
    .await
    .expect("seal manifest state");

    let block = load_pending_block(&pool, &block_hash).await;
    // Thirty retries after the initial failure means 31 total attempts.
    for attempt in 1..=31 {
        record_manifest_publication_error(
            &pool,
            &block,
            "manifest S3 operation timed out",
            31,
            1_000_000,
        )
        .await
        .expect("record retryable publication error");

        let retry_scheduled = sqlx::query_scalar::<_, bool>(
            "SELECT publication_next_retry_at IS NOT NULL \
               FROM block_manifest_state \
              WHERE host_chain_id = $1 AND block_hash = $2",
        )
        .bind(CHAIN_ID)
        .bind(&block_hash)
        .fetch_one(&pool)
        .await
        .expect("load publication retry state");
        assert_eq!(retry_scheduled, attempt < 31);
    }

    let error = sqlx::query(
        "SELECT publication_error_count, publication_next_retry_at IS NOT NULL AS retry_scheduled \
           FROM block_manifest_state \
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(&block_hash)
    .fetch_one(&pool)
    .await
    .expect("load retryable publication error");
    assert_eq!(error.get::<i64, _>("publication_error_count"), 31);
    assert!(!error.get::<bool, _>("retry_scheduled"));
}

#[tokio::test]
#[serial(db)]
async fn local_statement_timeout_bounds_manifest_work_selection() {
    let (_instance, pool) = setup_pool().await;
    let mut trx = pool
        .begin()
        .await
        .expect("begin bounded selector transaction");
    set_local_statement_timeout(&mut trx, Duration::from_millis(1))
        .await
        .expect("set short statement timeout");

    let error = sqlx::query("SELECT pg_sleep(0.01)")
        .execute(trx.as_mut())
        .await
        .expect_err("statement exceeding the local timeout is cancelled");
    // SQLSTATE, not the message: server messages follow lc_messages.
    assert_eq!(
        error
            .as_database_error()
            .and_then(|error| error.code())
            .as_deref(),
        Some("57014"),
        "{error}"
    );
    trx.rollback()
        .await
        .expect("roll back cancelled transaction");
}

#[tokio::test]
#[serial(db)]
async fn discovery_retries_finalized_parent_when_child_arrives_after_first_poll() {
    let (_instance, pool) = setup_pool().await;
    let parent = [0x64; 32];
    let child = [0x65; 32];

    // The listener has persisted a finalized anchor, but not its successor yet.
    insert_host_block(&pool, 100, &parent, &[0x63; 32], "finalized").await;
    assert_eq!(discover_blocks(&pool).await.expect("bootstrap anchor"), 1);
    assert_eq!(
        discover_children(&pool)
            .await
            .expect("poll before child arrives"),
        0
    );

    // A later listener transaction supplies the child and real computation work.
    insert_host_block(&pool, 101, &child, &parent, "pending").await;
    insert_producer_block(&pool, 101, &child, &[0x71; 32]).await;
    for _ in 0..3 {
        discover_blocks(&pool)
            .await
            .expect("poll producer discovery");
        discover_children(&pool)
            .await
            .expect("poll child discovery");
    }

    let parent_closed = sqlx::query_scalar::<_, bool>(
        "SELECT child_block_discovery_closed FROM block_manifest_state
         WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(parent.as_slice())
    .fetch_one(&pool)
    .await
    .expect("read anchor discovery state");
    assert!(
        manifest_state_exists(&pool, &child).await,
        "three discovery polls missed child 101 after finalized anchor 100; parent discovery closed={parent_closed}"
    );
}

#[tokio::test]
#[serial(db)]
async fn discovers_all_direct_children_before_advancing_the_global_frontier() {
    let (_instance, pool) = setup_pool().await;
    let parent = vec![0x40; 32];
    let first_child = vec![0x41; 32];
    let grandchild = vec![0x42; 32];
    let orphan = vec![0x43; 32];
    let second_child = vec![0x44; 32];

    insert_host_block(&pool, 40, &parent, &[0x3f; 32], "finalized").await;
    insert_host_block(&pool, 41, &first_child, &parent, "pending").await;
    insert_host_block(&pool, 42, &grandchild, &first_child, "pending").await;
    insert_host_block(&pool, 41, &orphan, &parent, "orphaned").await;
    insert_host_block(&pool, 41, &second_child, &parent, "pending").await;
    insert_manifest_state(&pool, 40, &parent, &[0x3f; 32]).await;

    let parent_block = load_pending_block(&pool, &parent).await;
    let mut trx = pool.begin().await.expect("begin direct-child discovery");
    assert_eq!(
        discover_children_of(&mut trx, &parent_block)
            .await
            .expect("discover every direct child"),
        2
    );
    trx.commit().await.expect("commit direct-child discovery");

    assert!(manifest_state_exists(&pool, &first_child).await);
    assert!(manifest_state_exists(&pool, &second_child).await);
    assert!(!manifest_state_exists(&pool, &grandchild).await);
    assert!(!manifest_state_exists(&pool, &orphan).await);

    assert_eq!(
        discover_children(&pool)
            .await
            .expect("advance all known parents by one level"),
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
        !parent_closed,
        "pending children must not close discovery of a finalized parent"
    );
    assert!(manifest_state_exists(&pool, &grandchild).await);
    assert!(!manifest_state_exists(&pool, &orphan).await);
}

#[tokio::test]
#[serial(db)]
async fn discovery_waits_for_finalized_successor_after_pending_child_replacement() {
    let (_instance, pool) = setup_pool().await;
    let parent = [0x80; 32];
    let pending = [0x81; 32];
    let canonical = [0x82; 32];
    insert_host_block(&pool, 100, &parent, &[0x79; 32], "finalized").await;
    insert_manifest_state(&pool, 100, &parent, &[0x79; 32]).await;
    insert_host_block(&pool, 101, &pending, &parent, "pending").await;
    assert_eq!(discover_children(&pool).await.unwrap(), 1);
    discover_children(&pool).await.unwrap();

    // The first child is replaced after discovery; its finalized replacement
    // must still be discovered even though the parent was finalized throughout.
    sqlx::query("UPDATE host_chain_blocks_valid SET block_status = 'orphaned' WHERE chain_id = $1 AND block_hash = $2")
        .bind(CHAIN_ID).bind(pending.as_slice()).execute(&pool).await.unwrap();
    insert_host_block(&pool, 101, &canonical, &parent, "finalized").await;
    assert_eq!(discover_children(&pool).await.unwrap(), 1);
    assert!(manifest_state_exists(&pool, &canonical).await);
    discover_children(&pool).await.unwrap();
    let closed = sqlx::query_scalar::<_, bool>(
        "SELECT child_block_discovery_closed FROM block_manifest_state WHERE host_chain_id = $1 AND block_hash = $2")
        .bind(CHAIN_ID).bind(parent.as_slice()).fetch_one(&pool).await.unwrap();
    assert!(
        closed,
        "discovered finalized successor allows retiring the parent"
    );
    assert_eq!(discover_children(&pool).await.unwrap(), 0);
}

const SOURCE_CHAIN_ID: i64 = 1;

async fn insert_bridged_event(
    pool: &PgPool,
    block_number: i64,
    block_hash: &[u8],
    src_handle: &[u8],
    dst_handle: &[u8],
) {
    sqlx::query(
        "INSERT INTO handle_bridged_events (
             src_handle, dst_handle, dst_chain_id, receiver_dapp, guid,
             block_number, block_hash
         ) VALUES ($1, $2, $3, '\\xdb'::bytea, '\\x02'::bytea, $4, $5)",
    )
    .bind(src_handle)
    .bind(dst_handle)
    .bind(CHAIN_ID)
    .bind(block_number)
    .bind(block_hash)
    .execute(pool)
    .await
    .expect("insert HandleBridged observation");
}

/// The source chain's `BridgeHandle` approval for `src_handle`, in a source
/// block with `status`.
async fn insert_bridge_approval(pool: &PgPool, src_handle: &[u8], status: &str) {
    let approval_block = vec![0x07_u8; 32];
    sqlx::query(
        "INSERT INTO bridge_handle_events (
             src_handle, dst_chain_id, src_chain_id, sender_dapp, guid,
             block_number, block_hash
         ) VALUES ($1, $2, $3, '\\xda'::bytea, '\\x02'::bytea, 7, $4)",
    )
    .bind(src_handle)
    .bind(CHAIN_ID)
    .bind(SOURCE_CHAIN_ID)
    .bind(&approval_block)
    .execute(pool)
    .await
    .expect("insert BridgeHandle approval");
    sqlx::query(
        "INSERT INTO host_chain_blocks_valid \
         (chain_id, block_hash, parent_hash, block_number, block_status) \
         VALUES ($1, $2, $3, 7, $4)",
    )
    .bind(SOURCE_CHAIN_ID)
    .bind(&approval_block)
    .bind(vec![0x06_u8; 32])
    .bind(status)
    .execute(pool)
    .await
    .expect("insert source approval block");
}

async fn insert_digest(pool: &PgPool, host_chain_id: i64, handle: &[u8], ct64: u8) {
    sqlx::query(
        "INSERT INTO ciphertext_digest (
             host_chain_id, key_id_gw, handle, ciphertext, ciphertext128,
             ciphertext128_format
         ) VALUES ($1, $2, $3, $4, $5, 11)",
    )
    .bind(host_chain_id)
    .bind(vec![0x11_u8; 32])
    .bind(handle)
    .bind(vec![ct64; 32])
    .bind(vec![0x80_u8; 32])
    .execute(pool)
    .await
    .expect("insert ciphertext digests");
    // The descriptor needs a keyset for the digest's Gateway key.
    sqlx::query(
        "INSERT INTO keys (key_id_gw, key_id, pks_key, sks_key, chain_id, block_hash)
         SELECT $1, $2, ''::BYTEA, ''::BYTEA, $3, $2
          WHERE NOT EXISTS (SELECT 1 FROM keys WHERE key_id_gw = $1)",
    )
    .bind(vec![0x11_u8; 32])
    .bind(vec![0x12_u8; 32])
    .bind(host_chain_id)
    .execute(pool)
    .await
    .expect("insert the digest's keyset");
}

async fn ready_descriptors(
    pool: &PgPool,
    block: &PendingBlock,
) -> Option<Vec<CiphertextDescriptor>> {
    let mut trx = pool.begin().await.expect("begin readiness check");
    let ready = is_block_manifest_ready(&mut trx, block)
        .await
        .expect("check readiness");
    let descriptors = if ready {
        Some(
            load_manifest_descriptors(&mut trx, block, false)
                .await
                .expect("load descriptors"),
        )
    } else {
        None
    };
    trx.rollback().await.expect("rollback readiness check");
    descriptors
}

/// A bridged destination is sealed from its source digests as soon as the
/// source is computed, without waiting for the copy, which needs source
/// finality. The copy then carries the same digests, so the seal stays valid.
#[tokio::test]
#[serial(db)]
async fn bridged_destination_seals_with_the_source_digests_before_its_copy() {
    let (_instance, pool) = setup_pool().await;
    let block_number = 42;
    let block_hash = vec![0x42; 32];
    let src_handle = vec![0x51; 32];
    let dst_handle = vec![0x52; 32];

    insert_host_block(&pool, block_number, &block_hash, &[0x41; 32], "finalized").await;
    insert_producer_block(&pool, block_number, &block_hash, &dst_handle).await;
    insert_bridged_event(&pool, block_number, &block_hash, &src_handle, &dst_handle).await;
    insert_bridge_approval(&pool, &src_handle, "pending").await;
    discover_blocks(&pool)
        .await
        .expect("discover bridging block");
    let block = load_pending_block(&pool, &block_hash).await;

    assert!(
        ready_descriptors(&pool, &block).await.is_none(),
        "the destination waits for its source like any consumer"
    );

    insert_digest(&pool, SOURCE_CHAIN_ID, &src_handle, 0x64).await;
    let from_source = ready_descriptors(&pool, &block)
        .await
        .expect("ready once the source is computed");
    assert_eq!(from_source.len(), 1);

    // The bridge worker copies the source digest row onto the destination.
    insert_digest(&pool, CHAIN_ID, &dst_handle, 0x64).await;
    assert_eq!(
        ready_descriptors(&pool, &block).await,
        Some(from_source),
        "the copy describes the destination exactly like its source did"
    );
}

/// Once this node materialized the destination, its own digest describes it,
/// even when it differs from the source: the manifest reports what the node
/// holds, and verification settles the difference.
#[tokio::test]
#[serial(db)]
async fn bridged_destination_prefers_its_own_digest_over_the_source() {
    let (_instance, pool) = setup_pool().await;
    let block_number = 42;
    let block_hash = vec![0x42; 32];
    let src_handle = vec![0x51; 32];
    let dst_handle = vec![0x52; 32];
    let plain_handle = vec![0x53; 32];

    insert_host_block(&pool, block_number, &block_hash, &[0x41; 32], "pending").await;
    insert_producer_block(&pool, block_number, &block_hash, &dst_handle).await;
    insert_bridged_event(&pool, block_number, &block_hash, &src_handle, &dst_handle).await;
    insert_digest(&pool, SOURCE_CHAIN_ID, &src_handle, 0x64).await;
    insert_digest(&pool, CHAIN_ID, &dst_handle, 0x65).await;
    insert_manifest_state(&pool, block_number, &block_hash, &[0x41; 32]).await;
    let bridged = ready_descriptors(&pool, &load_pending_block(&pool, &block_hash).await)
        .await
        .expect("bridged block ready");

    // The same digests on a plain handle give the expected descriptor bytes.
    let other_hash = vec![0x43; 32];
    insert_host_block(&pool, block_number + 1, &other_hash, &block_hash, "pending").await;
    insert_producer_block(&pool, block_number + 1, &other_hash, &plain_handle).await;
    insert_digest(&pool, CHAIN_ID, &plain_handle, 0x65).await;
    insert_manifest_state(&pool, block_number + 1, &other_hash, &block_hash).await;
    let plain = ready_descriptors(&pool, &load_pending_block(&pool, &other_hash).await)
        .await
        .expect("plain block ready");

    assert_eq!(
        format!("{:?}", bridged[0]).replace(&hex::encode(&dst_handle), "H"),
        format!("{:?}", plain[0]).replace(&hex::encode(&plain_handle), "H"),
        "the destination is described by its own ct64 digest (0x65), not the source's (0x64)"
    );
}

/// Once the source's `BridgeHandle` block is orphaned the copy can never
/// happen, so the source digests no longer describe the destination: it waits
/// for its own bytes, which a fallback grant supplies.
#[tokio::test]
#[serial(db)]
async fn bridged_destination_ignores_the_source_once_its_approval_is_orphaned() {
    let (_instance, pool) = setup_pool().await;
    let block_number = 42;
    let block_hash = vec![0x42; 32];
    let src_handle = vec![0x51; 32];
    let dst_handle = vec![0x52; 32];

    insert_host_block(&pool, block_number, &block_hash, &[0x41; 32], "pending").await;
    insert_producer_block(&pool, block_number, &block_hash, &dst_handle).await;
    insert_bridged_event(&pool, block_number, &block_hash, &src_handle, &dst_handle).await;
    insert_bridge_approval(&pool, &src_handle, "orphaned").await;
    insert_digest(&pool, SOURCE_CHAIN_ID, &src_handle, 0x64).await;
    insert_manifest_state(&pool, block_number, &block_hash, &[0x41; 32]).await;
    let block = load_pending_block(&pool, &block_hash).await;

    assert!(
        ready_descriptors(&pool, &block).await.is_none(),
        "an orphaned approval never yields the copy the source digests predict"
    );

    insert_digest(&pool, CHAIN_ID, &dst_handle, 0x65).await;
    assert!(
        ready_descriptors(&pool, &block).await.is_some(),
        "the grant's own bytes make the block ready"
    );
}
