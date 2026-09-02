use super::sequence_tests::{
    assert_payload_matches_production_cover, assert_persisted_ranges, concurrent_manifest_worker,
    index_blocks, insert_simulated_history_block, join_workers, positive_env_u64,
    wait_for_published_manifest_reference, ConcurrentPublisherStats, ExpectedBlock,
    SEQUENCE_CADENCE, SEQUENCE_CHAIN_ID,
};
use alloy::signers::local::PrivateKeySigner;
use alloy_primitives::{Address, B256, U256};
use block_manifest::SignedManifest;
use serial_test::serial;
use sqlx::Row;
use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use test_harness::instance::{setup_test_db, ImportMode};
use tokio::sync::Barrier;

/// Concurrent publishers keep prefix/A/B histories isolated across a reorg.
/// Cover oracles are production `append_leaf` via the shared sequence helpers.
#[tokio::test]
#[serial(db)]
async fn concurrent_manifest_publishers_keep_histories_isolated_across_reorg() {
    let worker_count = usize::try_from(positive_env_u64("SNS_MANIFEST_SIMULATED_WORKERS", 2))
        .expect("worker count fits usize");
    assert!(worker_count >= 2);

    let test_instance = setup_test_db(ImportMode::None)
        .await
        .expect("create concurrent reorg database");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(u32::try_from(worker_count + 4).unwrap())
        .connect(test_instance.db_url())
        .await
        .expect("connect concurrent reorg database");
    let signer = Arc::new(PrivateKeySigner::random());
    let producer_done = Arc::new(AtomicBool::new(false));
    let stats = Arc::new(ConcurrentPublisherStats::new(worker_count));
    let start_barrier = Arc::new(Barrier::new(worker_count + 1));
    let mut workers = Vec::with_capacity(worker_count);
    for worker_id in 0..worker_count {
        workers.push(tokio::spawn(concurrent_manifest_worker(
            worker_id,
            pool.clone(),
            Arc::clone(&signer),
            Arc::clone(&producer_done),
            Arc::clone(&start_barrier),
            Arc::clone(&stats),
        )));
    }
    start_barrier.wait().await;
    let started_at = tokio::time::Instant::now();
    let mut expected_blocks = Vec::new();

    let mut parent = B256::repeat_byte(0xff);
    for number in 0..=6 {
        parent =
            insert_simulated_history_block(&pool, &mut expected_blocks, number, parent, 0).await;
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
    let fork_parent = parent;
    wait_for_published_manifest_reference(&pool, 6, fork_parent).await;

    let mut branch_a_parent = fork_parent;
    for number in 7..=12 {
        branch_a_parent =
            insert_simulated_history_block(&pool, &mut expected_blocks, number, branch_a_parent, 1)
                .await;
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
    wait_for_published_manifest_reference(&pool, 12, branch_a_parent).await;

    let mut branch_b_parent = fork_parent;
    for number in 7..=12 {
        branch_b_parent =
            insert_simulated_history_block(&pool, &mut expected_blocks, number, branch_b_parent, 2)
                .await;
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
    wait_for_published_manifest_reference(&pool, 12, branch_b_parent).await;

    for number in 13..=18 {
        branch_b_parent =
            insert_simulated_history_block(&pool, &mut expected_blocks, number, branch_b_parent, 2)
                .await;
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
    wait_for_published_manifest_reference(&pool, 18, branch_b_parent).await;
    producer_done.store(true, Ordering::Release);
    join_workers(workers).await;

    let expected_manifest_keys = expected_blocks
        .iter()
        .filter(|block| block.number.rem_euclid(SEQUENCE_CADENCE) == 0)
        .map(|block| (block.number, block.block_hash))
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        stats.sealed_blocks.load(Ordering::Acquire),
        u64::try_from(expected_blocks.len()).unwrap()
    );
    assert_eq!(
        stats.published_manifests.load(Ordering::Acquire),
        u64::try_from(expected_manifest_keys.len()).unwrap()
    );
    assert!(stats.busy_locks.load(Ordering::Acquire) > 0);
    assert_eq!(stats.owner_keys(), expected_manifest_keys);

    let generated = stats.signed_manifests();
    assert_eq!(
        generated
            .keys()
            .copied()
            .collect::<std::collections::BTreeSet<_>>(),
        expected_manifest_keys
    );
    assert_reorg_manifests_use_production_cover(&generated, &expected_blocks, signer.address());
    assert_persisted_reorg_state(&pool, &generated, &expected_blocks).await;
    println!(
        "reorg sequence complete: blocks={} manifests={} workers={} elapsed={:?}",
        expected_blocks.len(),
        expected_manifest_keys.len(),
        worker_count,
        started_at.elapsed(),
    );
}

fn assert_reorg_manifests_use_production_cover(
    manifests: &BTreeMap<(i64, B256), SignedManifest>,
    expected_blocks: &[ExpectedBlock],
    publisher: Address,
) {
    let by_hash = index_blocks(expected_blocks);
    for ((publication_number, publication_hash), signed) in manifests {
        signed.verify().expect("verify reorg manifest");
        assert_eq!(signed.payload.publisher, publisher);
        assert_eq!(
            signed.payload.publication_block_number,
            U256::from(*publication_number as u64)
        );
        assert_eq!(signed.payload.publication_block_hash, *publication_hash);
        let previous = previous_publication(*publication_hash, manifests, &by_hash);
        assert_payload_matches_production_cover(&signed.payload, &by_hash, previous.as_ref());
    }
    for height in [9_i64, 12_i64] {
        assert_eq!(
            manifests
                .keys()
                .filter(|(number, _)| *number == height)
                .count(),
            2,
            "both histories must publish independently at reorg height {height}"
        );
    }
}

fn previous_publication(
    publication_hash: B256,
    manifests: &BTreeMap<(i64, B256), SignedManifest>,
    by_hash: &BTreeMap<B256, &ExpectedBlock>,
) -> Option<crate::manifest_consensus::manifest_archive::ManifestReference> {
    let publication = *by_hash.get(&publication_hash).expect("publication exists");
    let mut parent_hash = publication.parent_block_hash;
    while let Some(parent) = by_hash.get(&parent_hash) {
        if parent.number.rem_euclid(SEQUENCE_CADENCE) == 0 {
            let signed = manifests
                .get(&(parent.number, parent.block_hash))
                .expect("published reorg ancestor manifest exists");
            return Some(
                crate::manifest_consensus::manifest_archive::ManifestReference {
                    generation: signed.payload.consensus_epoch.clone(),
                    publisher: signed.payload.publisher,
                    block_number: signed.payload.publication_block_number,
                    block_hash: signed.payload.publication_block_hash,
                    revision: signed.payload.revision,
                    manifest_digest: signed.digest().expect("digest previous reorg manifest"),
                },
            );
        }
        parent_hash = parent.parent_block_hash;
    }
    None
}

async fn assert_persisted_reorg_state(
    pool: &sqlx::PgPool,
    generated: &BTreeMap<(i64, B256), SignedManifest>,
    expected_blocks: &[ExpectedBlock],
) {
    let unsealed: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM block_manifest_state
          WHERE host_chain_id = $1
            AND (block_content_digest IS NULL OR block_handle_count IS NULL)",
    )
    .bind(SEQUENCE_CHAIN_ID)
    .fetch_one(pool)
    .await
    .expect("count unsealed reorg blocks");
    assert_eq!(unsealed, 0);

    let rows = sqlx::query(
        "SELECT publication_block_number, publication_block_hash, signed_manifest
           FROM block_manifest
          WHERE host_chain_id = $1",
    )
    .bind(SEQUENCE_CHAIN_ID)
    .fetch_all(pool)
    .await
    .expect("load persisted reorg manifests");
    assert_eq!(rows.len(), generated.len());
    for row in rows {
        let number: i64 = row.get("publication_block_number");
        let hash = B256::from_slice(&row.get::<Vec<u8>, _>("publication_block_hash"));
        let signed: SignedManifest =
            serde_json::from_slice(&row.get::<Vec<u8>, _>("signed_manifest"))
                .expect("decode persisted reorg manifest");
        assert_eq!(generated.get(&(number, hash)), Some(&signed));
    }
    assert_persisted_ranges(pool, expected_blocks).await;
}
