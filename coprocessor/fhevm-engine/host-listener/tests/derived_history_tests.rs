//! Repairs must not silently change inventory underneath immutable commitments.

mod ingest_fixtures;
use ingest_fixtures::*;

use fhevm_engine_common::chain_recovery::{
    is_halted, lock_history_tx, ListenerSource,
};
use host_listener::database::tfhe_event_propagate::Database;
use serial_test::serial;
use sqlx::PgPool;
use test_harness::instance::DBInstance;
use tokio::time::{timeout, Duration};

async fn partial_block() -> anyhow::Result<(Database, PgPool, DBInstance)> {
    let (mut db, pool, instance) = setup().await?;
    let b = block();
    let tx = computed_handle(0xe1);
    let a = computed_handle(1);
    let b_handle = computed_handle(2);
    let logs = vec![
        rpc_log(&trivial_encrypt(a), &b, tx, 0),
        rpc_log(&allowed(a), &b, tx, 1),
        rpc_log(&trivial_encrypt(b_handle), &b, tx, 2),
    ];
    ingest(&mut db, &b, logs, false).await?;
    sqlx::query(
        "UPDATE host_chain_blocks_valid SET block_status = 'finalized'",
    )
    .execute(&pool)
    .await?;
    sqlx::query(
        "UPDATE computations SET is_completed = TRUE WHERE output_handle = $1",
    )
    .bind(a.to_vec())
    .execute(&pool)
    .await?;
    sqlx::query("INSERT INTO ciphertexts (tenant_id, handle, ciphertext, ciphertext_version, ciphertext_type)
                 VALUES (0, $1, '\\xaabb', 0, 5)")
        .bind(a.to_vec()).execute(&pool).await?;
    assert_eq!(computation_flags(&pool, b_handle).await?, (false, true));
    Ok((db, pool, instance))
}

async fn hash_block(pool: &PgPool, uploaded: bool) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO state_hash (chain_id, block_number, state_hash, s3_uploaded_at)
                 VALUES (42, 100, encode(sha256('\\xaabb'::bytea), 'hex'),
                         CASE WHEN $1 THEN NOW() END)")
        .bind(uploaded).execute(pool).await?;
    Ok(())
}

async fn seal_block(pool: &PgPool, published: bool) -> anyhow::Result<()> {
    let b = block();
    sqlx::query("INSERT INTO block_manifest_state
        (host_chain_id, block_number, block_hash, parent_block_hash, publication_cadence,
         block_content_digest, block_handle_count, manifest_published, manifest_digest,
         manifest_publisher, manifest_published_at)
        VALUES (42, 100, $1, $2, 1, $3, 1, $4,
                CASE WHEN $4 THEN $3 END,
                CASE WHEN $4 THEN decode(repeat('11', 20), 'hex') END,
                CASE WHEN $4 THEN NOW() END)")
        .bind(b.hash.to_vec()).bind(b.parent_hash.to_vec())
        .bind(vec![0x33u8; 32]).bind(published).execute(pool).await?;
    Ok(())
}

async fn repair(db: &mut Database, later_acl: bool) -> anyhow::Result<()> {
    let mut b = block();
    if later_acl {
        b.number += 1;
        b.parent_hash = b.hash;
        b.hash = computed_handle(0xb2);
    }
    ingest(
        db,
        &b,
        vec![rpc_log(
            &allowed(computed_handle(2)),
            &b,
            computed_handle(0xe1),
            3,
        )],
        true,
    )
    .await
}

async fn assert_contained(pool: &PgPool) -> anyhow::Result<()> {
    assert!(is_halted(pool).await?);
    let (height, reason): (i64, String) = sqlx::query_as(
        "SELECT block_number, reason FROM public.chain_recovery_halts WHERE resolved_at IS NULL"
    ).fetch_one(pool).await?;
    assert_eq!(
        height, 100,
        "record the producer height, even for a later ACL"
    );
    assert!(reason.contains("inventory already committed"));
    assert_eq!(
        computation_flags(pool, computed_handle(2)).await?,
        (false, true)
    );
    let producers: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM handle_producer_block")
            .fetch_one(pool)
            .await?;
    assert_eq!(producers, 1, "roll back the producer inventory repair too");
    Ok(())
}

async fn unchanged_replay(
    db: &mut Database,
    pool: &PgPool,
) -> anyhow::Result<()> {
    let b = block();
    let tx = computed_handle(0xe1);
    ingest(
        db,
        &b,
        vec![
            rpc_log(&trivial_encrypt(computed_handle(1)), &b, tx, 0),
            rpc_log(&allowed(computed_handle(1)), &b, tx, 1),
            rpc_log(&trivial_encrypt(computed_handle(2)), &b, tx, 2),
        ],
        true,
    )
    .await?;
    assert!(!is_halted(pool).await?, "an identical replay is harmless");
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn repair_before_commitments_remains_automatic() -> anyhow::Result<()> {
    let (mut db, pool, _instance) = partial_block().await?;
    repair(&mut db, false).await?;
    assert_eq!(
        computation_flags(&pool, computed_handle(2)).await?,
        (true, false)
    );
    assert!(!is_halted(&pool).await?);
    Ok(())
}

async fn hashed_repair(uploaded: bool, later_acl: bool) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = partial_block().await?;
    hash_block(&pool, uploaded).await?;
    unchanged_replay(&mut db, &pool).await?;
    let err = repair(&mut db, later_acl).await.unwrap_err();
    assert!(err.to_string().contains("inventory already committed"));
    assert_contained(&pool).await?;
    let unchanged: bool = sqlx::query_scalar("SELECT state_hash = encode(sha256('\\xaabb'::bytea), 'hex')
        AND (s3_uploaded_at IS NOT NULL) = $1 FROM state_hash WHERE chain_id = 42 AND block_number = 100")
        .bind(uploaded).fetch_one(&pool).await?;
    assert!(
        unchanged,
        "preserve the original commitment and its upload status"
    );
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn repair_after_hash_creation_is_contained() -> anyhow::Result<()> {
    hashed_repair(false, false).await
}

#[tokio::test]
#[serial(db)]
async fn repair_after_hash_upload_is_contained() -> anyhow::Result<()> {
    hashed_repair(true, false).await
}

#[tokio::test]
#[serial(db)]
async fn later_acl_cannot_change_an_older_hashed_block() -> anyhow::Result<()> {
    hashed_repair(true, true).await
}

async fn sealed_repair(
    published: bool,
    descendant: bool,
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = partial_block().await?;
    seal_block(&pool, published).await?;
    if descendant {
        // Block 100 is committed through 101's detailed history, without its
        // own publication. A published-flag-only check would miss it.
        sqlx::query("INSERT INTO block_manifest_state
            (host_chain_id, block_number, block_hash, parent_block_hash, publication_cadence,
             block_content_digest, block_handle_count, manifest_published, manifest_digest,
             manifest_publisher, manifest_published_at)
            VALUES (42, 101, $1, $2, 1, $3, 0, TRUE, $3,
                    decode(repeat('11', 20), 'hex'), NOW())")
            .bind(computed_handle(0xb2).to_vec()).bind(block().hash.to_vec())
            .bind(vec![0x44u8; 32]).execute(&pool).await?;
        // Older binaries cleared both fields when refreshing an unpublished
        // ancestor's seal. Its published descendant still commits the history.
        sqlx::query(
            "UPDATE block_manifest_state SET block_content_digest = NULL,
            block_handle_count = NULL WHERE block_number = 100",
        )
        .execute(&pool)
        .await?;
    }
    unchanged_replay(&mut db, &pool).await?;
    assert!(repair(&mut db, false).await.is_err());
    assert_contained(&pool).await?;
    let (count, still_published): (Option<i64>, bool) = sqlx::query_as(
        "SELECT block_handle_count, manifest_published FROM block_manifest_state WHERE block_number = 100"
    ).fetch_one(&pool).await?;
    assert_eq!(
        (count, still_published),
        (if descendant { None } else { Some(1) }, published)
    );
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn repair_after_manifest_sealing_is_contained() -> anyhow::Result<()> {
    sealed_repair(false, false).await
}

#[tokio::test]
#[serial(db)]
async fn repair_during_manifest_resealing_is_contained() -> anyhow::Result<()> {
    let (mut db, pool, _instance) = partial_block().await?;
    seal_block(&pool, false).await?;
    // Normal completion can invalidate a digest; the retained descriptor count
    // must keep the inventory frozen until the publisher reseals it.
    sqlx::query("UPDATE block_manifest_state SET block_content_digest = NULL")
        .execute(&pool)
        .await?;
    assert!(repair(&mut db, false).await.is_err());
    assert_contained(&pool).await
}

#[tokio::test]
#[serial(db)]
async fn repair_after_manifest_publication_is_contained() -> anyhow::Result<()>
{
    sealed_repair(true, false).await
}

#[tokio::test]
#[serial(db)]
async fn repair_of_history_in_a_published_descendant_is_contained(
) -> anyhow::Result<()> {
    sealed_repair(false, true).await
}

#[tokio::test]
#[serial(db)]
async fn new_computation_cannot_extend_a_hashed_block() -> anyhow::Result<()> {
    let (mut db, pool, _instance) = partial_block().await?;
    hash_block(&pool, true).await?;
    let b = block();
    let new = computed_handle(3);
    assert!(ingest(
        &mut db,
        &b,
        vec![
            rpc_log(&trivial_encrypt(new), &b, computed_handle(0xe2), 4),
            rpc_log(&allowed(new), &b, computed_handle(0xe2), 5),
        ],
        true
    )
    .await
    .is_err());
    assert_contained(&pool).await?;
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM computations WHERE output_handle = $1",
    )
    .bind(new.to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(count, 0);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn inventory_only_repair_of_an_existing_ciphertext_is_contained(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = partial_block().await?;
    sqlx::query("DELETE FROM handle_producer_block")
        .execute(&pool)
        .await?;
    seal_block(&pool, true).await?;
    sqlx::query("UPDATE block_manifest_state SET block_handle_count = 0")
        .execute(&pool)
        .await?;
    let b = block();
    let a = computed_handle(1);
    let tx = computed_handle(0xe1);
    assert!(ingest(
        &mut db,
        &b,
        vec![
            rpc_log(&trivial_encrypt(a), &b, tx, 0),
            rpc_log(&allowed(a), &b, tx, 1),
        ],
        true
    )
    .await
    .is_err());
    assert!(is_halted(&pool).await?);
    assert_eq!(computation_flags(&pool, a).await?, (true, true));
    let producers: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM handle_producer_block")
            .fetch_one(&pool)
            .await?;
    assert_eq!(producers, 0);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn scheduling_only_repair_after_a_seal_remains_allowed(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = partial_block().await?;
    repair(&mut db, false).await?;
    seal_block(&pool, true).await?;
    sqlx::query("UPDATE block_manifest_state SET block_handle_count = 2")
        .execute(&pool)
        .await?;
    sqlx::query("UPDATE dependence_chain SET status = 'processed'")
        .execute(&pool)
        .await?;
    repair(&mut db, false).await?;
    assert!(!is_halted(&pool).await?);
    assert_eq!(chain_status(&pool, computed_handle(0xe1)).await?, "updated");
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn green_commitments_do_not_freeze_blue_inventory() -> anyhow::Result<()>
{
    use fhevm_engine_common::database::GCS_SCHEMA_QUOTED;
    let (mut db, pool, _instance) = partial_block().await?;
    sqlx::query(&format!("CREATE SCHEMA {GCS_SCHEMA_QUOTED}"))
        .execute(&pool)
        .await?;
    sqlx::query(&format!("CREATE TABLE {GCS_SCHEMA_QUOTED}.state_hash (LIKE public.state_hash INCLUDING ALL)"))
        .execute(&pool).await?;
    sqlx::query(&format!(
        "INSERT INTO {GCS_SCHEMA_QUOTED}.state_hash
        (chain_id, block_number, state_hash) VALUES (42, 100, 'green')"
    ))
    .execute(&pool)
    .await?;
    repair(&mut db, false).await?;
    assert_eq!(
        computation_flags(&pool, computed_handle(2)).await?,
        (true, false)
    );
    assert!(!is_halted(&pool).await?);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn sealed_sibling_does_not_prevent_ordinary_reorg_ingestion(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = partial_block().await?;
    sqlx::query("UPDATE host_chain_blocks_valid SET block_status = 'pending'")
        .execute(&pool)
        .await?;
    seal_block(&pool, true).await?;
    let mut fork = block();
    fork.hash = computed_handle(0xb3);
    let out = computed_handle(3);
    ingest(
        &mut db,
        &fork,
        vec![
            rpc_log(&trivial_encrypt(out), &fork, computed_handle(0xe2), 0),
            rpc_log(&allowed(out), &fork, computed_handle(0xe2), 1),
        ],
        false,
    )
    .await?;
    assert!(!is_halted(&pool).await?);
    assert_eq!(computation_flags(&pool, out).await?, (true, false));
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn replay_observes_a_commitment_that_was_in_flight() -> anyhow::Result<()>
{
    let (mut db, pool, _instance) = partial_block().await?;
    let writer_pool = db.pool().await;
    let mut writer = writer_pool.begin().await?;
    lock_history_tx(&mut writer, ListenerSource::Host, false, 42).await?;
    sqlx::query("INSERT INTO state_hash (chain_id, block_number, state_hash) VALUES (42, 100, 'old')")
        .execute(&mut *writer).await?;
    let repair_task = tokio::spawn(async move { repair(&mut db, false).await });
    timeout(Duration::from_secs(5), async {
        loop {
            let waiting: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_locks
                WHERE locktype = 'advisory' AND NOT granted
                  AND database = (SELECT oid FROM pg_database WHERE datname = current_database()))")
                .fetch_one(&pool).await.unwrap();
            if waiting { break; }
            tokio::task::yield_now().await;
        }
    }).await?;
    writer.commit().await?;
    assert!(timeout(Duration::from_secs(5), repair_task)
        .await??
        .is_err());
    assert_contained(&pool).await?;
    Ok(())
}
