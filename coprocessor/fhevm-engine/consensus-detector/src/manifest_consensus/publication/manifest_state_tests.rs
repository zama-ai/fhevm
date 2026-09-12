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
        "blue_green_generation",
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
        "SELECT generation, host_chain_id, block_number, block_hash, parent_block_hash,
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
        generation: row.get("generation"),
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

    insert_host_block(&pool, block_number, &block_hash, &parent_hash, "pending").await;
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
async fn detector_caps_upgrade_discovery_at_the_generation_start() {
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
        "INSERT INTO generation_history (
             generation, proposal_id, proposal_block, stack_version, outcome
         ) VALUES ('1', $1, 50, 'test-green', 'pending')",
    )
    .bind(vec![0x91_u8; 32])
    .execute(&pool)
    .await
    .expect("allocate Green generation");
    sqlx::query(
        "INSERT INTO generation_block_window (
             generation, host_chain_id, start_block, consensus_deadline_block
         ) VALUES ('1', $1, 60, 70)",
    )
    .bind(CHAIN_ID)
    .execute(&pool)
    .await
    .expect("store Green generation window");
    sqlx::query(
        "UPDATE blue_green_generation
            SET generation = '1', updated_at = NOW()
          WHERE singleton = TRUE",
    )
    .execute(&pool)
    .await
    .expect("select Green generation");

    assert_eq!(
        discover_blocks(&pool)
            .await
            .expect("wait for the generation start block"),
        0
    );
    assert!(!manifest_state_exists(&pool, &current_block_hash).await);

    insert_host_block(&pool, 60, &start_block_hash, &[0x59; 32], "pending").await;
    assert_eq!(
        discover_blocks(&pool)
            .await
            .expect("discover in-generation catch-up"),
        2
    );
    assert!(!manifest_state_exists(&pool, &old_block_hash).await);
    assert!(manifest_state_exists(&pool, &start_block_hash).await);
    assert!(manifest_state_exists(&pool, &current_block_hash).await);
    let generation: String = sqlx::query_scalar(
        "SELECT generation FROM block_manifest_state
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(CHAIN_ID)
    .bind(&current_block_hash)
    .fetch_one(&pool)
    .await
    .expect("load selected candidate generation");
    assert_eq!(generation, "1");
    assert_eq!(
        pending_chain_ids(&pool)
            .await
            .expect("select in-generation publication work"),
        vec![CHAIN_ID]
    );
}

#[tokio::test]
#[serial(db)]
async fn initial_generation_bootstraps_at_the_latest_known_block() {
    let (_instance, pool) = setup_pool().await;
    let historical_hash = vec![0x42; 32];
    let historical_handle = vec![0x21; 32];
    let bootstrap_hash = vec![0x50; 32];

    insert_host_block(&pool, 42, &historical_hash, &[0x41; 32], "pending").await;
    insert_producer_block(&pool, 42, &historical_hash, &historical_handle).await;
    insert_completed_sns_digest(&pool, 42, &historical_hash, &historical_handle).await;
    insert_host_block(&pool, 50, &bootstrap_hash, &[0x49; 32], "pending").await;

    assert_eq!(discover_blocks(&pool).await.unwrap(), 1);
    assert!(!manifest_state_exists(&pool, &historical_hash).await);
    assert!(manifest_state_exists(&pool, &bootstrap_hash).await);

    // The persisted bootstrap block is the generation-zero lower bound; a
    // restart must not widen the arbitrary initial history into older blocks.
    assert_eq!(discover_blocks(&pool).await.unwrap(), 0);
    assert!(!manifest_state_exists(&pool, &historical_hash).await);
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
    .bind("pending")
    .execute(&pool)
    .await
    .expect("insert anvil host block");

    let generation = crate::manifest_consensus::storage::active::load_generation(&pool)
        .await
        .expect("load generation");
    let overrides = publication_cadence_overrides([(ANVIL_CHAIN_ID, 1)]).unwrap();
    assert_eq!(
        discover_blocks_for_generation(&pool, &generation, &overrides)
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
async fn blue_and_green_discover_the_same_block_without_generation_collision() {
    let (instance, admin_pool) = setup_pool().await;
    create_green_discovery_schema(&admin_pool).await;
    let blue = stack_pool(instance.db_url(), "public").await;
    let green = stack_pool(instance.db_url(), "gcs_manifest_test,public").await;
    let block_number = 62;
    let block_hash = vec![0x62; 32];
    let parent_hash = vec![0x61; 32];
    let handle = vec![0x42; 32];

    sqlx::query(
        "INSERT INTO generation_history (
             generation, proposal_id, proposal_block, stack_version, outcome
         ) VALUES ('1', $1, 60, 'test-green', 'pending')",
    )
    .bind(vec![0xa1_u8; 32])
    .execute(&admin_pool)
    .await
    .expect("allocate Green generation");
    sqlx::query(
        "INSERT INTO generation_block_window (
             generation, host_chain_id, start_block, consensus_deadline_block
         ) VALUES ('1', $1, 62, 70)",
    )
    .bind(CHAIN_ID)
    .execute(&admin_pool)
    .await
    .expect("store Green generation window");
    sqlx::query(
        "INSERT INTO gcs_manifest_test.blue_green_generation
             (singleton, generation) VALUES (TRUE, '1')",
    )
    .execute(&admin_pool)
    .await
    .expect("select Green generation");

    for stack in [&blue, &green] {
        insert_host_block(stack, block_number, &block_hash, &parent_hash, "pending").await;
        insert_producer_block(stack, block_number, &block_hash, &handle).await;
        insert_completed_sns_digest(stack, block_number, &block_hash, &handle).await;
    }

    assert_eq!(discover_blocks(&blue).await.unwrap(), 1);
    assert_eq!(discover_blocks(&green).await.unwrap(), 1);
    assert_eq!(discover_blocks(&blue).await.unwrap(), 0);
    assert_eq!(discover_blocks(&green).await.unwrap(), 0);

    let generations = sqlx::query_scalar::<_, String>(
        "SELECT generation
           FROM public.block_manifest_state
          WHERE host_chain_id = $1 AND block_hash = $2
          ORDER BY generation",
    )
    .bind(CHAIN_ID)
    .bind(&block_hash)
    .fetch_all(&admin_pool)
    .await
    .expect("load both stack candidates");
    assert_eq!(generations, vec!["1".to_string(), "legacy".to_string()]);
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
    assert!(error.to_string().contains("statement timeout"));
    trx.rollback()
        .await
        .expect("roll back cancelled transaction");
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
        parent_closed,
        "finalized parent discovery must close after copying children"
    );
    assert!(manifest_state_exists(&pool, &grandchild).await);
    assert!(!manifest_state_exists(&pool, &orphan).await);
}
