//! Repairs must not silently change inventory underneath immutable commitments.

mod ingest_fixtures;
use ingest_fixtures::*;

use alloy::rpc::types::Log;
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::chain_recovery::{
    is_halted, lock_history_tx, ListenerSource,
};
use host_listener::database::ingest::{ingest_block_logs, BlockLogs};
use host_listener::database::tfhe_event_propagate::Database;
use serial_test::serial;
use sqlx::PgPool;
use test_harness::db_utils::revert_coprocessor_db_state_sql;
use test_harness::instance::DBInstance;
use tokio::time::{timeout, Duration};

fn partial_logs() -> Vec<Log> {
    let b = block();
    let tx = computed_handle(0xe1);
    vec![
        rpc_log(&trivial_encrypt(computed_handle(1)), &b, tx, 0),
        rpc_log(&allowed(computed_handle(1)), &b, tx, 1),
        rpc_log(&trivial_encrypt(computed_handle(2)), &b, tx, 2),
    ]
}

async fn partial_block() -> anyhow::Result<(Database, PgPool, DBInstance)> {
    let (mut db, pool, instance) = setup().await?;
    let a = computed_handle(1);
    ingest(&mut db, &block(), partial_logs(), false).await?;
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
    assert_eq!(
        computation_flags(&pool, computed_handle(2)).await?,
        (false, true)
    );
    Ok((db, pool, instance))
}

async fn finalized_replay(
    db: &mut Database,
    logs: Vec<Log>,
) -> anyhow::Result<()> {
    ingest_block_logs(
        ChainId::try_from(42u64)?,
        db,
        &BlockLogs {
            logs,
            summary: block(),
            catchup: true,
            finalized: true,
        },
        &None,
        &None,
        &None,
        &None,
        &None,
        OPTIONS,
    )
    .await?
    .require_complete()?;
    Ok(())
}

async fn prune_header(db: &Database, pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query("UPDATE host_chain_blocks_valid SET created_at = NOW() - INTERVAL '8 days'")
        .execute(pool).await?;
    assert_eq!(db.prune_finalized_block_history(20_000).await?, 1);
    let remaining: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM host_chain_blocks_valid")
            .fetch_one(pool)
            .await?;
    assert_eq!(remaining, 0);
    Ok(())
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
    ingest(db, &block(), partial_logs(), true).await?;
    assert!(!is_halted(pool).await?, "an identical replay is harmless");
    Ok(())
}

async fn rollback_preserves_committed_inventory(
    manifest: bool,
    published: bool,
    automatic: bool,
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = partial_block().await?;
    sqlx::query("INSERT INTO host_chains (chain_id, name, acl_contract_address) VALUES (42, 'test', '0xACL')")
        .execute(&pool).await?;
    if manifest {
        seal_block(&pool, published).await?;
    } else {
        hash_block(&pool, published).await?;
    }
    let commitments_before: serde_json::Value = sqlx::query_scalar(
        "SELECT jsonb_build_object(
            'hashes', (SELECT jsonb_agg(to_jsonb(h)) FROM state_hash h),
            'manifests', (SELECT jsonb_agg(to_jsonb(m)) FROM block_manifest_state m),
            'producers', (SELECT jsonb_agg(to_jsonb(p)) FROM handle_producer_block p))",
    )
    .fetch_one(&pool)
    .await?;
    let metadata_before: Vec<serde_json::Value> = sqlx::query_scalar(
        "SELECT to_jsonb(c) - ARRAY['is_completed', 'is_error', 'error_message', 'error_retry_count']
         FROM computations c ORDER BY output_handle",
    )
    .fetch_all(&pool)
    .await?;
    // Cover both a retired chain and a chain already removed by cleanup.
    if published {
        sqlx::query("DELETE FROM dependence_chain")
            .execute(&pool)
            .await?;
    } else {
        sqlx::query("UPDATE dependence_chain SET status = 'processed'")
            .execute(&pool)
            .await?;
    }
    sqlx::query("UPDATE computations SET is_error = TRUE, error_message = 'previous execution', error_retry_count = 3 WHERE is_allowed")
        .execute(&pool).await?;

    if automatic {
        fhevm_engine_common::drift_revert::execute_revert(&pool, 42, 100)
            .await?;
    } else {
        sqlx::raw_sql(&revert_coprocessor_db_state_sql(42, 99))
            .execute(&pool)
            .await?;
    }
    assert_eq!(
        computation_flags(&pool, computed_handle(1)).await?,
        (true, false),
        "the allowed producer keeps its metadata but must run again"
    );
    assert_eq!(
        computation_flags(&pool, computed_handle(2)).await?,
        (false, true),
        "rollback must not invent a missing permission"
    );
    let remaining: (i64, i64, i64) = sqlx::query_as(
        "SELECT (SELECT COUNT(*) FROM ciphertexts),
                (SELECT COUNT(*) FROM computations WHERE is_error OR error_message IS NOT NULL OR error_retry_count <> 0),
                (SELECT COUNT(*) FROM host_chain_blocks_valid)",
    )
    .fetch_one(&pool)
    .await?;
    assert_eq!(remaining, (0, 0, 0));

    finalized_replay(&mut db, partial_logs()).await?;
    assert!(!is_halted(&pool).await?);
    let metadata_after: Vec<serde_json::Value> = sqlx::query_scalar(
        "SELECT to_jsonb(c) - ARRAY['is_completed', 'is_error', 'error_message', 'error_retry_count']
         FROM computations c ORDER BY output_handle",
    )
    .fetch_all(&pool)
    .await?;
    assert_eq!(metadata_after, metadata_before);
    let reachable: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM computations c JOIN dependence_chain d
            ON d.dependence_chain_id = c.dependence_chain_id
            WHERE c.output_handle = $1 AND d.status = 'updated')",
    )
    .bind(computed_handle(1).to_vec())
    .fetch_one(&pool)
    .await?;
    assert!(
        reachable,
        "replay must re-arm the retained producer's chain"
    );
    let commitments_after: serde_json::Value = sqlx::query_scalar(
        "SELECT jsonb_build_object(
            'hashes', (SELECT jsonb_agg(to_jsonb(h)) FROM state_hash h),
            'manifests', (SELECT jsonb_agg(to_jsonb(m)) FROM block_manifest_state m),
            'producers', (SELECT jsonb_agg(to_jsonb(p)) FROM handle_producer_block p))",
    )
    .fetch_one(&pool)
    .await?;
    assert_eq!(commitments_after, commitments_before);

    // Recovery is not an exemption from the inventory guard. A genuinely new
    // permission must still be rejected against the retained original metadata.
    let error = repair(&mut db, false).await.unwrap_err();
    assert!(error.to_string().contains("inventory already committed"));
    assert_contained(&pool).await?;
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn rollback_replays_a_hashed_block() -> anyhow::Result<()> {
    rollback_preserves_committed_inventory(false, false, false).await
}

#[tokio::test]
#[serial(db)]
async fn automatic_rollback_replays_an_uploaded_hash() -> anyhow::Result<()> {
    rollback_preserves_committed_inventory(false, true, true).await
}

#[tokio::test]
#[serial(db)]
async fn rollback_replays_a_sealed_manifest() -> anyhow::Result<()> {
    rollback_preserves_committed_inventory(true, false, false).await
}

#[tokio::test]
#[serial(db)]
async fn automatic_rollback_replays_a_published_manifest() -> anyhow::Result<()>
{
    rollback_preserves_committed_inventory(true, true, true).await
}

#[tokio::test]
#[serial(db)]
async fn rollback_keeps_a_retained_computation_complete_when_its_ciphertext_is_shared(
) -> anyhow::Result<()> {
    let (_db, pool, _instance) = partial_block().await?;
    sqlx::query("INSERT INTO host_chains (chain_id, name, acl_contract_address) VALUES (42, 'test', '0xACL')")
        .execute(&pool).await?;
    hash_block(&pool, false).await?;
    // A pre-boundary computation still needs these bytes. Rollback must keep
    // both its ciphertext and the completion of the committed later producer.
    sqlx::query("INSERT INTO computations
        (output_handle, dependencies, fhe_operation, is_scalar, transaction_id,
         host_chain_id, dependence_chain_id, block_number, is_allowed, is_completed)
        SELECT output_handle, dependencies, fhe_operation, is_scalar, $2,
               host_chain_id, dependence_chain_id, 99, is_allowed, is_completed
        FROM computations WHERE output_handle = $1")
        .bind(computed_handle(1).to_vec())
        .bind(computed_handle(0xe0).to_vec())
        .execute(&pool).await?;
    sqlx::raw_sql(&revert_coprocessor_db_state_sql(42, 99))
        .execute(&pool)
        .await?;
    let (rows, completed): (i64, bool) = sqlx::query_as(
        "SELECT COUNT(*), BOOL_AND(is_completed) FROM computations WHERE output_handle = $1",
    )
    .bind(computed_handle(1).to_vec())
    .fetch_one(&pool).await?;
    assert_eq!((rows, completed), (2, true));
    let ciphertext: Vec<u8> = sqlx::query_scalar(
        "SELECT ciphertext FROM ciphertexts WHERE handle = $1",
    )
    .bind(computed_handle(1).to_vec())
    .fetch_one(&pool)
    .await?;
    assert_eq!(ciphertext, vec![0xaa, 0xbb]);
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

#[tokio::test]
#[serial(db)]
async fn identical_replay_restores_pruned_hashed_header() -> anyhow::Result<()>
{
    let (mut db, pool, _instance) = partial_block().await?;
    hash_block(&pool, true).await?;
    finalized_replay(&mut db, partial_logs()).await?;
    let before: Vec<serde_json::Value> = sqlx::query_scalar(
        "SELECT to_jsonb(c) FROM computations c ORDER BY output_handle",
    )
    .fetch_all(&pool)
    .await?;
    let hash_before: serde_json::Value =
        sqlx::query_scalar("SELECT to_jsonb(sh) FROM state_hash sh")
            .fetch_one(&pool)
            .await?;
    for acl_only_first in [false, true] {
        prune_header(&db, &pool).await?;
        if acl_only_first {
            // A partial delivery can restore a finalized header with count 0.
            // It must not erase the retained hash's evidence of FHE activity.
            finalized_replay(&mut db, vec![partial_logs()[1].clone()]).await?;
        }
        finalized_replay(&mut db, partial_logs()).await?;
        assert!(!is_halted(&pool).await?);
        let header: (String, i32) = sqlx::query_as(
            "SELECT block_status, fhe_event_count FROM host_chain_blocks_valid",
        )
        .fetch_one(&pool)
        .await?;
        assert_eq!(header, ("finalized".into(), 2));
        let after: Vec<serde_json::Value> = sqlx::query_scalar(
            "SELECT to_jsonb(c) FROM computations c ORDER BY output_handle",
        )
        .fetch_all(&pool)
        .await?;
        assert_eq!(before, after, "only the header needed restoration");
        let hash_after: serde_json::Value =
            sqlx::query_scalar("SELECT to_jsonb(sh) FROM state_hash sh")
                .fetch_one(&pool)
                .await?;
        assert_eq!(
            hash_before, hash_after,
            "preserve the hash and upload status"
        );
    }
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn identical_empty_replay_restores_pruned_hashed_header(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = setup().await?;
    finalized_replay(&mut db, vec![]).await?;
    sqlx::query(
        "INSERT INTO state_hash (chain_id, block_number, state_hash)
        VALUES (42, 100, encode(sha256(''::bytea), 'hex'))",
    )
    .execute(&pool)
    .await?;
    prune_header(&db, &pool).await?;
    finalized_replay(&mut db, vec![]).await?;
    assert!(!is_halted(&pool).await?);
    let unchanged: bool = sqlx::query_scalar(
        "SELECT sh.state_hash = encode(sha256(''::bytea), 'hex') AND b.fhe_event_count = 0
         FROM state_hash sh JOIN host_chain_blocks_valid b
           ON b.chain_id = sh.chain_id AND b.block_number = sh.block_number
         WHERE b.block_status = 'finalized'",
    ).fetch_one(&pool).await?;
    assert!(unchanged);
    Ok(())
}

async fn empty_hash_event_repair(pruned: bool) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = partial_block().await?;
    // A prior partial delivery stored the computations but marked the block
    // empty. Replaying their events changes only the header's event count.
    sqlx::query("UPDATE host_chain_blocks_valid SET fhe_event_count = 0")
        .execute(&pool)
        .await?;
    sqlx::query(
        "INSERT INTO state_hash (chain_id, block_number, state_hash)
        VALUES (42, 100, encode(sha256(''::bytea), 'hex'))",
    )
    .execute(&pool)
    .await?;
    if pruned {
        prune_header(&db, &pool).await?;
    }
    let error = finalized_replay(&mut db, partial_logs()).await.unwrap_err();
    assert!(
        error.to_string().contains("inventory already committed"),
        "{error}"
    );
    assert_contained(&pool).await?;
    let unchanged: bool = sqlx::query_scalar(
        "SELECT state_hash = encode(sha256(''::bytea), 'hex') FROM state_hash",
    )
    .fetch_one(&pool)
    .await?;
    assert!(unchanged);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn event_count_repair_of_a_known_empty_commitment_is_contained(
) -> anyhow::Result<()> {
    empty_hash_event_repair(false).await
}

#[tokio::test]
#[serial(db)]
async fn event_count_repair_of_a_pruned_empty_commitment_is_contained(
) -> anyhow::Result<()> {
    empty_hash_event_repair(true).await
}

#[tokio::test]
#[serial(db)]
async fn acl_repair_after_header_pruning_is_still_contained(
) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = partial_block().await?;
    hash_block(&pool, true).await?;
    prune_header(&db, &pool).await?;
    assert!(repair(&mut db, false).await.is_err());
    assert_contained(&pool).await
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

/// Rollback drops the green schema and marks the epoch failed together. The
/// seal stays in `public`, and must not freeze inventory that is gone.
#[tokio::test]
#[serial(db)]
async fn a_failed_epoch_seal_does_not_contain_the_repair() -> anyhow::Result<()>
{
    let (mut db, pool, _instance) = partial_block().await?;
    seal_block(&pool, true).await?;
    sqlx::query(
        "INSERT INTO consensus_epoch_history (consensus_epoch, outcome, completed_at)
         VALUES ('2', 'failed', NOW())",
    )
    .execute(&pool)
    .await?;
    sqlx::query("UPDATE block_manifest_state SET consensus_epoch = '2'")
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

async fn extend_hashed_block(pruned: bool) -> anyhow::Result<()> {
    let (mut db, pool, _instance) = partial_block().await?;
    hash_block(&pool, true).await?;
    if pruned {
        prune_header(&db, &pool).await?;
    }
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
async fn new_computation_cannot_extend_a_hashed_block() -> anyhow::Result<()> {
    extend_hashed_block(false).await
}

#[tokio::test]
#[serial(db)]
async fn new_computation_cannot_extend_a_pruned_hashed_block(
) -> anyhow::Result<()> {
    extend_hashed_block(true).await
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
