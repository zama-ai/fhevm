use super::*;
use crate::manifest_consensus::manifest_archive::{
    manifest_object_key, store_authenticated_manifest, ManifestSource,
};
use alloy::signers::local::PrivateKeySigner;
use alloy_primitives::{B256, U256};
use block_manifest::{
    block_content_digest, detailed_range_digest, DetailedRange, ManifestBlockEntry,
    ManifestPayload, ManifestVersion,
};
use serial_test::serial;
use std::time::Duration;
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};

#[path = "case_matrix_tests.rs"]
mod case_matrix;

#[path = "ensure_containment_tests.rs"]
mod ensure_containment_tests;

#[path = "inflight_tests.rs"]
mod inflight;

#[path = "cross_epoch_tests.rs"]
mod cross_epoch;

const TEST_EPOCH: &str = block_manifest::LEGACY_CONSENSUS_EPOCH;

fn bytes(n: u8) -> Vec<u8> {
    vec![n; 32]
}

async fn setup() -> (DBInstance, PgPool) {
    let instance = setup_test_db(ImportMode::None).await.unwrap();
    let pool = PgPool::connect(instance.db_url()).await.unwrap();
    (instance, pool)
}

async fn producer(pool: &PgPool, handle: u8, chain: i64) {
    sqlx::query("INSERT INTO handle_producer_block (host_chain_id, handle, producer_block_number, producer_block_hash) VALUES ($1, $2, $3, $2)")
        .bind(chain).bind(bytes(handle)).bind(i64::from(handle)).execute(pool).await.unwrap();
}

async fn root(pool: &PgPool, handle: u8) -> i64 {
    producer(pool, handle, 1).await;
    insert_root(pool, handle).await
}

async fn insert_root<'a>(executor: impl sqlx::PgExecutor<'a>, handle: u8) -> i64 {
    sqlx::query_scalar("INSERT INTO drifted_handle (consensus_epoch, coprocessor_context_id, host_chain_id, block_number, block_hash, handle, detection_kind, reason, local_present, quorum_present) SELECT consensus_epoch, $1, 1, $2, $3, $3, 'inferred', 'ct64_mismatch', TRUE, FALSE FROM blue_green_consensus_epoch RETURNING id")
        .bind(bytes(1)).bind(i64::from(handle)).bind(bytes(handle)).fetch_one(executor).await.unwrap()
}

pub(crate) async fn direct_root(pool: &PgPool, handle: u8, reason: &str) -> i64 {
    let id = root(pool, handle).await;
    // Store a real signed local manifest to back the finding's required task FK.
    // This test exercises containment of recorded findings, not peer comparison.
    let signer = PrivateKeySigner::random();
    let number = U256::from(handle);
    let hash = B256::repeat_byte(handle);
    let context = U256::from_be_slice(&bytes(1));
    let digest =
        block_content_digest(ManifestVersion::V1, context, U256::ONE, number, hash, &[]).unwrap();
    let consensus_epoch: String =
        sqlx::query_scalar("SELECT consensus_epoch FROM blue_green_consensus_epoch")
            .fetch_one(pool)
            .await
            .unwrap();
    let manifest = ManifestPayload {
        version: ManifestVersion::V1,
        consensus_epoch: consensus_epoch.clone(),
        publisher: signer.address(),
        coprocessor_context_id: context,
        host_chain_id: U256::ONE,
        publication_block_number: number,
        publication_block_hash: hash,
        publication_parent_block_hash: B256::ZERO,
        revision: 0,
        detailed_range: DetailedRange {
            first_block_number: number,
            last_block_number: number,
            digest: detailed_range_digest(
                ManifestVersion::V1,
                context,
                U256::ONE,
                number,
                number,
                &[digest],
            ),
            blocks: vec![ManifestBlockEntry {
                block_number: number,
                block_hash: hash,
                parent_block_hash: B256::ZERO,
                block_content_digest: digest,
                ciphertexts: vec![],
            }],
        },
        historical_ranges: vec![],
    }
    .sign(&signer)
    .await
    .unwrap();
    let mut trx = pool.begin().await.unwrap();
    let archive = store_authenticated_manifest(
        &mut trx,
        signer.address(),
        &manifest_object_key(&manifest),
        &serde_json::to_vec(&manifest).unwrap(),
        ManifestSource::Local,
    )
    .await
    .unwrap();
    let task: i64 = sqlx::query_scalar("INSERT INTO block_manifest_verification_task (consensus_epoch, local_manifest_id, eligible_at, retry_delay_secs, max_attempts) VALUES ($1, $2, NOW(), 0, 5) RETURNING id")
        .bind(consensus_epoch).bind(archive.id).fetch_one(trx.as_mut()).await.unwrap();
    sqlx::query("UPDATE drifted_handle SET detection_kind = 'verified', reason = $2, local_keyset_id = $3, local_ct64_digest = $3, local_ct128_digest = $3, local_ct128_format = 0, quorum_present = TRUE, quorum_keyset_id = $3, quorum_ct64_digest = $4, quorum_ct128_digest = $4, quorum_ct128_format = 0, last_quorum_task_id = $5 WHERE id = $1")
        .bind(id).bind(reason).bind(bytes(1)).bind(bytes(2)).bind(task).execute(trx.as_mut()).await.unwrap();
    if reason == "ct128_mismatch" {
        sqlx::query(
            "UPDATE drifted_handle SET quorum_ct64_digest = local_ct64_digest WHERE id = $1",
        )
        .bind(id)
        .execute(trx.as_mut())
        .await
        .unwrap();
    }
    trx.commit().await.unwrap();
    id
}

// Boundary inputs use canonical ct64. Raw intermediate rows have no stored bytes
// despite being born is_completed=true, just as listener ingestion records them.
async fn computation(
    pool: &PgPool,
    output: u8,
    input: u8,
    transaction: u8,
    stored: bool,
    boundary: bool,
) {
    let mut mask = vec![0u8; 32];
    if boundary {
        mask[31] = 1;
    }
    sqlx::query("INSERT INTO computations (output_handle, dependencies, fhe_operation, is_scalar, transaction_id, host_chain_id, block_number, is_completed, is_allowed, operand_boundary_mask) VALUES ($1, ARRAY[$2, $3]::bytea[], 0, TRUE, $4, 1, $5, $6, $6, $7)")
        .bind(bytes(output)).bind(bytes(input)).bind(vec![0u8; 32]).bind(bytes(transaction)).bind(i64::from(output)).bind(stored).bind(mask)
        .execute(pool).await.unwrap();
    if stored {
        producer(pool, output, 1).await;
        sqlx::query("INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type) VALUES ($1, $2, 0, 0)")
            .bind(bytes(output)).bind(vec![1u8]).execute(pool).await.unwrap();
    }
}

async fn flags(pool: &PgPool) -> Vec<(Vec<u8>, bool)> {
    sqlx::query_as("SELECT handle, is_contained FROM drifted_handle ORDER BY handle")
        .fetch_all(pool)
        .await
        .unwrap()
}

#[tokio::test]
#[serial(db)]
async fn optimistic_then_guaranteed_pass_catches_late_outputs_and_is_idempotent() {
    let (_db, pool) = setup().await;
    root(&pool, 1).await;
    computation(&pool, 2, 1, 2, true, true).await;
    computation(&pool, 3, 2, 3, true, true).await;
    computation(&pool, 4, 3, 4, false, true).await; // frozen, no output
    let mut trx = pool.begin().await.unwrap();
    assert_eq!(
        propagate_for_test(&mut trx, false).await.unwrap(),
        PropagationResult {
            inferred_handles: 2,
            contained_findings: 0
        }
    );
    trx.commit().await.unwrap();
    assert_eq!(
        flags(&pool).await,
        vec![(bytes(1), false), (bytes(2), false), (bytes(3), false)]
    );
    // A batch escaping the optimistic pass commits another computed descendant.
    computation(&pool, 5, 3, 5, true, true).await;
    assert_eq!(
        enforce_guaranteed_containment(&pool).await.unwrap(),
        PropagationResult {
            inferred_handles: 1,
            contained_findings: 4
        }
    );
    assert_eq!(
        flags(&pool).await,
        vec![
            (bytes(1), true),
            (bytes(2), true),
            (bytes(3), true),
            (bytes(5), true)
        ]
    );
    assert_eq!(
        enforce_guaranteed_containment(&pool).await.unwrap(),
        PropagationResult::default()
    );
    // An ordinary pending insertion needs no inferred record or new pass.
    computation(&pool, 6, 5, 6, false, true).await;
    assert_eq!(
        enforce_guaranteed_containment(&pool).await.unwrap(),
        PropagationResult::default()
    );
}

#[tokio::test]
#[serial(db)]
async fn inferred_insert_notifies_healing() {
    let (_db, pool) = setup().await;
    root(&pool, 1).await;
    computation(&pool, 2, 1, 2, true, true).await;
    let mut listener = sqlx::postgres::PgListener::connect_with(&pool)
        .await
        .unwrap();
    listener
        .listen(crate::manifest_consensus::healing::EVENT_HEALING_WORK)
        .await
        .unwrap();
    assert!(
        enforce_guaranteed_containment(&pool)
            .await
            .unwrap()
            .inferred_handles
            > 0
    );
    tokio::time::timeout(Duration::from_secs(2), listener.recv())
        .await
        .expect("inferred drifted_handle insert should NOTIFY event_healing_work")
        .unwrap();
}

#[tokio::test]
#[serial(db)]
async fn same_block_consumer_listed_before_producer_is_inferred() {
    let (_db, pool) = setup().await;
    root(&pool, 1).await;
    // Consumer first in schedule_order, same block as the producer it depends on.
    sqlx::query("INSERT INTO computations (output_handle, dependencies, fhe_operation, is_scalar, transaction_id, host_chain_id, block_number, is_completed, is_allowed) VALUES ($1, ARRAY[$2, $3]::bytea[], 0, TRUE, $1, 1, 10, TRUE, TRUE)")
        .bind(bytes(3))
        .bind(bytes(2))
        .bind(vec![0u8; 32])
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO computations (output_handle, dependencies, fhe_operation, is_scalar, transaction_id, host_chain_id, block_number, is_completed, is_allowed) VALUES ($1, ARRAY[$2, $3]::bytea[], 0, TRUE, $1, 1, 10, TRUE, TRUE)")
        .bind(bytes(2))
        .bind(bytes(1))
        .bind(vec![0u8; 32])
        .execute(&pool)
        .await
        .unwrap();
    producer(&pool, 2, 1).await;
    producer(&pool, 3, 1).await;
    sqlx::query("INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type) VALUES ($1, $2, 0, 0), ($3, $2, 0, 0)")
        .bind(bytes(2))
        .bind(vec![1u8])
        .bind(bytes(3))
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        enforce_guaranteed_containment(&pool)
            .await
            .unwrap()
            .inferred_handles,
        2
    );
}

#[tokio::test]
#[serial(db)]
async fn raw_and_canonical_are_conservative_but_scalars_and_chains_are_distinct() {
    let (_db, pool) = setup().await;
    root(&pool, 1).await;
    computation(&pool, 2, 1, 2, false, true).await; // raw intermediate consumed drift
    sqlx::query("UPDATE computations SET is_completed = TRUE WHERE output_handle = $1")
        .bind(bytes(2))
        .execute(&pool)
        .await
        .unwrap();
    computation(&pool, 3, 2, 2, true, false).await; // follows the contaminated raw path
    computation(&pool, 4, 1, 4, true, false).await; // conservatively include raw consumers
    computation(&pool, 5, 2, 5, true, false).await; // conservatively include ambiguous raw consumers
    computation(&pool, 6, 9, 6, true, true).await;
    sqlx::query(
        "UPDATE computations SET dependencies = ARRAY[$1, $2]::bytea[] WHERE output_handle = $3",
    )
    .bind(bytes(9))
    .bind(bytes(1))
    .bind(bytes(6))
    .execute(&pool)
    .await
    .unwrap(); // scalar equals root
    computation(&pool, 7, 1, 7, true, true).await;
    sqlx::query("UPDATE computations SET host_chain_id = 2 WHERE output_handle = $1")
        .bind(bytes(7))
        .execute(&pool)
        .await
        .unwrap();
    let result = enforce_guaranteed_containment(&pool).await.unwrap();
    assert_eq!(result.inferred_handles, 3);
    assert_eq!(
        flags(&pool).await,
        vec![
            (bytes(1), true),
            (bytes(3), true),
            (bytes(4), true),
            (bytes(5), true)
        ]
    );
}

#[tokio::test]
#[serial(db)]
async fn exclusive_pass_waits_for_shared_batches_and_blocks_new_batches() {
    let (_db, pool) = setup().await;
    root(&pool, 1).await;
    computation(&pool, 2, 1, 2, true, true).await;
    let mut batch = pool.begin().await.unwrap();
    sqlx::query("SELECT pg_advisory_xact_lock_shared($1)")
        .bind(DRIFT_CONTAINMENT_BARRIER)
        .execute(batch.as_mut())
        .await
        .unwrap();
    let p = pool.clone();
    let pass = tokio::spawn(async move { enforce_guaranteed_containment(&p).await });
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let waiting: bool = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM pg_locks WHERE locktype = 'advisory' AND mode = 'ExclusiveLock' AND NOT granted AND objid = ($1::bigint & 4294967295)::oid)")
                .bind(DRIFT_CONTAINMENT_BARRIER).fetch_one(&pool).await.unwrap();
            if waiting { break; }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }).await.unwrap();
    assert!(!pass.is_finished());
    // The optimistic pass has committed before waiting for the worker barrier.
    assert_eq!(
        flags(&pool).await,
        vec![(bytes(1), false), (bytes(2), false)]
    );
    sqlx::query("SELECT id FROM drifted_handle WHERE handle = $1 FOR UPDATE NOWAIT")
        .bind(bytes(2))
        .fetch_one(batch.as_mut())
        .await
        .unwrap();
    // Commit escaped output before the shared batch releases its barrier.
    computation(&pool, 3, 2, 3, true, true).await;
    batch.commit().await.unwrap();
    let result = tokio::time::timeout(Duration::from_secs(10), pass)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(
        result,
        PropagationResult {
            inferred_handles: 2,
            contained_findings: 3
        }
    );
    let mut exclusive = pool.begin().await.unwrap();
    sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(DRIFT_CONTAINMENT_BARRIER)
        .execute(exclusive.as_mut())
        .await
        .unwrap();
    let mut next_batch = pool.begin().await.unwrap();
    let entered: bool = sqlx::query_scalar("SELECT pg_try_advisory_xact_lock_shared($1)")
        .bind(DRIFT_CONTAINMENT_BARRIER)
        .fetch_one(next_batch.as_mut())
        .await
        .unwrap();
    assert!(!entered);
    exclusive.commit().await.unwrap();
    let entered: bool = sqlx::query_scalar("SELECT pg_try_advisory_xact_lock_shared($1)")
        .bind(DRIFT_CONTAINMENT_BARRIER)
        .fetch_one(next_batch.as_mut())
        .await
        .unwrap();
    assert!(entered);
}

#[tokio::test]
#[serial(db)]
async fn caller_holds_the_barrier_and_rollback_reverts_findings() {
    let (_db, pool) = setup().await;
    root(&pool, 1).await;
    computation(&pool, 2, 1, 2, true, true).await;
    let mut barrier = pool.begin().await.unwrap();
    sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(DRIFT_CONTAINMENT_BARRIER)
        .execute(barrier.as_mut())
        .await
        .unwrap();
    let mut trx = pool.begin().await.unwrap();
    // Optimistic propagation proceeds while another session holds the barrier.
    let fast = tokio::time::timeout(Duration::from_secs(10), propagate_for_test(&mut trx, false))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        fast,
        PropagationResult {
            inferred_handles: 1,
            contained_findings: 0
        }
    );
    trx.rollback().await.unwrap();
    barrier.rollback().await.unwrap();

    let mut trx = pool.begin().await.unwrap();
    assert_eq!(
        propagate_for_test(&mut trx, true)
            .await
            .unwrap()
            .contained_findings,
        2
    );
    // The test caller holds the exclusive barrier through the transaction end.
    let entered: bool = sqlx::query_scalar("SELECT pg_try_advisory_xact_lock_shared($1)")
        .bind(DRIFT_CONTAINMENT_BARRIER)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(!entered);
    trx.rollback().await.unwrap();
    assert_eq!(flags(&pool).await, vec![(bytes(1), false)]);
    let entered: bool = sqlx::query_scalar("SELECT pg_try_advisory_xact_lock_shared($1)")
        .bind(DRIFT_CONTAINMENT_BARRIER)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(entered);
}

#[tokio::test]
#[serial(db)]
async fn two_producer_hashes_are_both_inferred() {
    let (_db, pool) = setup().await;
    root(&pool, 1).await;
    computation(&pool, 2, 1, 2, true, true).await;
    sqlx::query("INSERT INTO handle_producer_block (host_chain_id, handle, producer_block_number, producer_block_hash) VALUES (1, $1, 2, $2)")
        .bind(bytes(2)).bind(bytes(9)).execute(&pool).await.unwrap();
    enforce_guaranteed_containment(&pool).await.unwrap();
    let rows: Vec<(Vec<u8>, Vec<u8>, bool)> = sqlx::query_as(
        "SELECT handle, block_hash, is_contained FROM drifted_handle ORDER BY handle, block_hash",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        rows,
        vec![
            (bytes(1), bytes(1), true),
            (bytes(2), bytes(2), true),
            (bytes(2), bytes(9), true),
        ]
    );
}

#[tokio::test]
#[serial(db)]
async fn roots_arriving_during_a_pass_remain_uncontained() {
    let (_db, pool) = setup().await;
    root(&pool, 1).await;
    computation(&pool, 2, 1, 2, true, true).await;
    producer(&pool, 9, 1).await;
    let mut blocker = pool.begin().await.unwrap();
    sqlx::query("LOCK TABLE drifted_handle IN SHARE MODE")
        .execute(blocker.as_mut())
        .await
        .unwrap();
    let p = pool.clone();
    // Exercise an arrival during the protected pass specifically; an arrival
    // during the wrapper's optimistic pass can be covered by its second pass.
    let pass = tokio::spawn(async move {
        let mut trx = p.begin().await?;
        let result = propagate_for_test(&mut trx, true).await?;
        trx.commit().await?;
        Ok::<_, anyhow::Error>(result)
    });
    // Freeze the pass after its root/graph reads, at the inferred INSERT.
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let waiting: bool = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM pg_locks WHERE relation = 'drifted_handle'::regclass AND mode = 'RowExclusiveLock' AND NOT granted)")
                .fetch_one(&pool).await.unwrap();
            if waiting { break; }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }).await.unwrap();
    insert_root(blocker.as_mut(), 9).await;
    blocker.commit().await.unwrap();
    let result = tokio::time::timeout(Duration::from_secs(10), pass)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(result.contained_findings, 2);
    assert_eq!(
        flags(&pool).await,
        vec![(bytes(1), true), (bytes(2), true), (bytes(9), false)]
    );
    assert_eq!(
        enforce_guaranteed_containment(&pool)
            .await
            .unwrap()
            .contained_findings,
        1
    );
}

#[tokio::test]
#[serial(db)]
async fn empty_inventory_and_consensus_epoch_selection() {
    let (_db, pool) = setup().await;
    assert_eq!(
        enforce_guaranteed_containment(&pool).await.unwrap(),
        PropagationResult::default()
    );
    let id = root(&pool, 1).await;
    sqlx::query("UPDATE drifted_handle SET consensus_epoch = 'other-epoch' WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        enforce_guaranteed_containment(&pool).await.unwrap(),
        PropagationResult {
            inferred_handles: 0,
            contained_findings: 1
        }
    );
    assert_eq!(flags(&pool).await, vec![(bytes(1), true)]);
    let mut trx = pool.begin().await.unwrap();
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ")
        .execute(trx.as_mut())
        .await
        .unwrap();
    // Wrong isolation is logged at the caller; the pass still runs.
    assert!(propagate_for_test(&mut trx, false).await.is_ok());
}

#[tokio::test]
#[serial(db)]
async fn verified_ct64_is_contained_but_ct128_and_healed_roots_are_not_propagated() {
    let (_db, pool) = setup().await;
    direct_root(&pool, 1, "ct64_mismatch").await;
    direct_root(&pool, 4, "ct128_mismatch").await;
    let healed = root(&pool, 7).await;
    sqlx::query(
        "UPDATE drifted_handle SET quorum_ct64_digest = $2, healed_at = NOW() WHERE id = $1",
    )
    .bind(healed)
    .bind(bytes(2))
    .execute(&pool)
    .await
    .unwrap();
    computation(&pool, 2, 1, 2, true, true).await;
    computation(&pool, 5, 4, 5, true, true).await;
    computation(&pool, 8, 7, 8, true, true).await;
    assert_eq!(
        enforce_guaranteed_containment(&pool).await.unwrap(),
        PropagationResult {
            inferred_handles: 1,
            contained_findings: 2
        }
    );
    assert_eq!(
        flags(&pool).await,
        vec![
            (bytes(1), true),
            (bytes(2), true),
            (bytes(4), false),
            (bytes(7), false)
        ]
    );
    // Containment leaves the quorum descriptor untouched: the root stays
    // healable toward the quorum ct64.
    let (target, healable): (Option<Vec<u8>>, bool) = sqlx::query_as(
        "SELECT quorum_ct64_digest, can_be_healed FROM drifted_handle WHERE handle = $1",
    )
    .bind(bytes(1))
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(target, Some(bytes(2)));
    assert!(healable);
}

#[tokio::test]
#[serial(db)]
async fn retained_computations_connect_descendants_after_intermediate_bytes_are_removed() {
    let (_db, pool) = setup().await;
    root(&pool, 1).await;
    computation(&pool, 2, 1, 2, true, true).await;
    computation(&pool, 3, 2, 3, true, true).await;
    sqlx::query("DELETE FROM ciphertexts WHERE handle = $1")
        .bind(bytes(2))
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        enforce_guaranteed_containment(&pool)
            .await
            .unwrap()
            .inferred_handles,
        1
    );
    assert_eq!(flags(&pool).await, vec![(bytes(1), true), (bytes(3), true)]);
}

async fn epoch_flags(pool: &PgPool) -> Vec<(Vec<u8>, String, bool)> {
    sqlx::query_as(
        "SELECT handle, consensus_epoch, is_contained FROM drifted_handle
         ORDER BY handle, consensus_epoch",
    )
    .fetch_all(pool)
    .await
    .unwrap()
}

async fn store_ciphertext(pool: &PgPool, handle: u8, payload: u8) {
    sqlx::query(
        "INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type)
         VALUES ($1, $2, 0, 0)",
    )
    .bind(bytes(handle))
    .bind(vec![payload])
    .execute(pool)
    .await
    .unwrap();
}

#[tokio::test]
#[serial(db)]
async fn consumed_foreign_epoch_ct64_marks_descendants_in_this_epoch() {
    let (_db, pool) = setup().await;
    let foreign = root(&pool, 1).await;
    computation(&pool, 2, 1, 2, true, true).await;
    sqlx::query("UPDATE drifted_handle SET consensus_epoch = 'other-epoch' WHERE id = $1")
        .bind(foreign)
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        enforce_guaranteed_containment(&pool).await.unwrap(),
        PropagationResult {
            inferred_handles: 1,
            contained_findings: 2
        }
    );
    assert_eq!(
        epoch_flags(&pool).await,
        vec![
            (bytes(1), "other-epoch".into(), true),
            (bytes(2), TEST_EPOCH.into(), true),
        ]
    );
    assert_eq!(
        enforce_guaranteed_containment(&pool).await.unwrap(),
        PropagationResult::default()
    );
}

#[tokio::test]
#[serial(db)]
async fn independent_local_copy_is_not_contaminated_by_foreign_drift() {
    let (_db, pool) = setup().await;
    let foreign = root(&pool, 1).await;
    store_ciphertext(&pool, 1, 9).await;
    computation(&pool, 2, 1, 2, true, true).await;
    sqlx::query("UPDATE drifted_handle SET consensus_epoch = 'other-epoch' WHERE id = $1")
        .bind(foreign)
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        enforce_guaranteed_containment(&pool).await.unwrap(),
        PropagationResult {
            inferred_handles: 1,
            contained_findings: 2
        }
    );
    assert_eq!(
        epoch_flags(&pool).await,
        vec![
            (bytes(1), "other-epoch".into(), true),
            (bytes(2), TEST_EPOCH.into(), true),
        ]
    );
}

async fn propagate_for_test(
    trx: &mut Transaction<'_, Postgres>,
    is_lock_protected: bool,
) -> Result<PropagationResult> {
    log_unless_read_committed(trx).await?;
    lock_cutover(trx).await?;
    if is_lock_protected {
        fhevm_engine_common::drift_containment::block_ct_computations(trx).await?;
    }
    propagate_drift(trx, is_lock_protected).await
}
