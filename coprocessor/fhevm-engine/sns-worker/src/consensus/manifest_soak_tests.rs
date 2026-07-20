use super::*;
use crate::consensus::manifest_archive::{manifest_object_key, store_authenticated_manifest};
use alloy::signers::local::PrivateKeySigner;
use ciphertext_attestation::manifest::SignedManifest;
use serial_test::serial;
use sqlx::{PgPool, Row};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};
use test_harness::instance::{setup_test_db, ImportMode};
use tokio::sync::Barrier;

const SOAK_CONTEXT_ID: U256 = U256::ONE;
const SOAK_CHAIN_ID: i64 = 7;
const SOAK_CADENCE: i64 = 3;
const SIMULATED_BLOCK_COUNT: u64 = 181;
const SIMULATED_BLOCK_INTERVAL_MS: u64 = 5;
const CONCURRENT_WORKER_COUNT: u64 = 8;

#[derive(Clone, Debug)]
struct ExpectedBlock {
    number: i64,
    block_hash: B256,
    parent_block_hash: B256,
    descriptors: Vec<CiphertextDescriptor>,
    content_digest: B256,
}

#[derive(Debug)]
struct MissingPreviousFaultSchedule {
    state: u64,
    eligible_checkpoints: u64,
}

impl MissingPreviousFaultSchedule {
    fn new() -> Self {
        Self {
            state: 0x6d61_6e69_6665_7374,
            eligible_checkpoints: 0,
        }
    }

    fn should_delete(&mut self) -> bool {
        // Deterministic xorshift fault schedule: reproducible, but irregular.
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.eligible_checkpoints += 1;
        self.eligible_checkpoints == 1 || self.state.is_multiple_of(4)
    }
}

#[derive(Debug)]
struct ConcurrentPublisherStats {
    attempts: AtomicU64,
    busy_locks: AtomicU64,
    sealed_blocks: AtomicU64,
    published_manifests: AtomicU64,
    worker_progress: Vec<AtomicU64>,
    manifest_owners: Mutex<BTreeMap<(i64, B256), usize>>,
    signed_manifests: Mutex<BTreeMap<(i64, B256), SignedManifest>>,
}

impl ConcurrentPublisherStats {
    fn new(worker_count: usize) -> Self {
        Self {
            attempts: AtomicU64::new(0),
            busy_locks: AtomicU64::new(0),
            sealed_blocks: AtomicU64::new(0),
            published_manifests: AtomicU64::new(0),
            worker_progress: (0..worker_count).map(|_| AtomicU64::new(0)).collect(),
            manifest_owners: Mutex::new(BTreeMap::new()),
            signed_manifests: Mutex::new(BTreeMap::new()),
        }
    }
}

/// Runs the production DB-backed manifest generator against a synthetic chain
/// producing one block every few milliseconds. S3 is intentionally excluded: this test is
/// about deterministic manifest construction, not object-store availability.
///
/// Default run (181 blocks at 5 ms, 61 manifests):
/// `SQLX_OFFLINE=true cargo test -p sns-worker canonical_manifest_generation_simulation_fast_chain --lib -- --ignored --nocapture`
///
/// Short local check:
/// `SNS_MANIFEST_SIMULATED_BLOCKS=7 SQLX_OFFLINE=true cargo test -p sns-worker canonical_manifest_generation_simulation_fast_chain --lib -- --ignored --nocapture`
#[tokio::test]
#[serial(db)]
#[ignore = "database-backed canonical manifest generation simulation; run explicitly"]
async fn canonical_manifest_generation_simulation_fast_chain() {
    run_concurrent_manifest_simulation("SNS_MANIFEST_SIMULATED_BLOCKS", false).await;
}

/// Runs several simulated sns-worker publishers against the same production
/// chain lock. Blocks are synthetic, while locking, sealing, canonical
/// preparation, signing, and persistence all use the production code and a
/// real PostgreSQL database.
///
/// Default run (181 blocks at 5 ms with eight publishers):
/// `SQLX_OFFLINE=true cargo test -p sns-worker concurrent_manifest_publishers_respect_the_production_db_lock --lib -- --ignored --nocapture`
#[tokio::test]
#[serial(db)]
#[ignore = "database-backed concurrent manifest publisher simulation; run explicitly"]
async fn concurrent_manifest_publishers_respect_the_production_db_lock() {
    run_concurrent_manifest_simulation("SNS_MANIFEST_SIMULATED_BLOCKS", false).await;
}

/// Repeatedly removes the immediate previous signed-manifest row at
/// deterministic-random checkpoints. The successor must reconstruct the
/// frontier from sealed block lineage and publish canonically anyway.
///
/// Default run (181 blocks at 5 ms):
/// `SQLX_OFFLINE=true cargo test -p sns-worker canonical_manifest_simulation_publishes_with_randomly_missing_previous_rows --lib -- --ignored --nocapture`
///
/// Short local check:
/// `SNS_MANIFEST_MISSING_PREVIOUS_SIMULATED_BLOCKS=13 SQLX_OFFLINE=true cargo test -p sns-worker canonical_manifest_simulation_publishes_with_randomly_missing_previous_rows --lib -- --ignored --nocapture`
#[tokio::test]
#[serial(db)]
#[ignore = "database-backed missing-previous-manifest simulation; run explicitly"]
async fn canonical_manifest_simulation_publishes_with_randomly_missing_previous_rows() {
    run_concurrent_manifest_simulation("SNS_MANIFEST_MISSING_PREVIOUS_SIMULATED_BLOCKS", true)
        .await;
}

/// Publishes a shared prefix, an initial branch A, and then a replacement
/// branch B that forks from the prefix and continues beyond A. All simulated
/// publishers race through the production chain lock throughout the reorg.
#[tokio::test]
#[serial(db)]
#[ignore = "database-backed concurrent fork/reorg manifest simulation; run explicitly"]
async fn concurrent_manifest_publishers_keep_histories_isolated_across_reorg() {
    run_concurrent_reorg_simulation().await;
}

async fn run_concurrent_manifest_simulation(block_count_env: &str, inject_missing_previous: bool) {
    let block_count = positive_env_u64(block_count_env, SIMULATED_BLOCK_COUNT);
    let block_interval_ms = positive_env_u64(
        "SNS_MANIFEST_SIMULATED_BLOCK_INTERVAL_MS",
        SIMULATED_BLOCK_INTERVAL_MS,
    );
    let worker_count = positive_env_u64("SNS_MANIFEST_SIMULATED_WORKERS", CONCURRENT_WORKER_COUNT);
    assert!(
        worker_count >= 2,
        "concurrency test requires at least two workers"
    );
    let worker_count = usize::try_from(worker_count).expect("worker count fits usize");
    let cadence = positive_env_i64("SNS_MANIFEST_SOAK_CADENCE", SOAK_CADENCE);
    let last_block_number = i64::try_from(block_count - 1).expect("block count fits BIGINT");

    let test_instance = setup_test_db(ImportMode::None)
        .await
        .expect("create concurrent manifest database");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(u32::try_from(worker_count + 4).expect("pool size fits u32"))
        .connect(test_instance.db_url())
        .await
        .expect("connect concurrent manifest database");
    let signer = Arc::new(PrivateKeySigner::random());
    let publisher = signer.address();
    let producer_done = Arc::new(AtomicBool::new(false));
    let stats = Arc::new(ConcurrentPublisherStats::new(worker_count));
    let start_barrier = Arc::new(Barrier::new(worker_count + 1));
    let mut workers = Vec::with_capacity(worker_count);
    let mut fault_schedule = MissingPreviousFaultSchedule::new();
    let mut deleted_manifest_digests = BTreeSet::new();

    for worker_id in 0..worker_count {
        workers.push(tokio::spawn(concurrent_manifest_worker(
            worker_id,
            pool.clone(),
            cadence,
            Arc::clone(&signer),
            Arc::clone(&producer_done),
            Arc::clone(&start_barrier),
            Arc::clone(&stats),
        )));
    }

    start_barrier.wait().await;
    let started_at = tokio::time::Instant::now();
    let mut expected_blocks = Vec::with_capacity(usize::try_from(block_count).unwrap());
    let mut parent_block_hash = B256::repeat_byte(0xff);
    for block_number in 0..=last_block_number {
        tokio::time::sleep_until(
            started_at
                + Duration::from_millis(u64::try_from(block_number).unwrap() * block_interval_ms),
        )
        .await;

        let block_hash = soak_value(*b"soakblok", block_number, 0);
        let mut predecessor_was_deleted = false;
        if inject_missing_previous
            && block_number > 0
            && block_number.rem_euclid(cadence) == 0
            && fault_schedule.should_delete()
        {
            let predecessor_number = block_number - cadence;
            let predecessor_hash = expected_block(&expected_blocks, predecessor_number).block_hash;
            let predecessor =
                wait_for_published_manifest_reference(&pool, predecessor_number, predecessor_hash)
                    .await;
            let deleted_digest = delete_previous_manifest(&pool, &predecessor).await;
            assert!(deleted_manifest_digests.insert(deleted_digest));
            predecessor_was_deleted = true;
        }

        // Seed all producer outputs before making the consensus block visible
        // to concurrent publishers; they must never seal a partially seeded
        // synthetic block.
        let descriptors = seed_block_descriptors(&pool, block_number, block_hash).await;
        let content_digest = block_content_digest(
            ManifestVersion::V1,
            SOAK_CONTEXT_ID,
            U256::from(SOAK_CHAIN_ID as u64),
            U256::from(block_number as u64),
            block_hash,
            &descriptors,
        )
        .expect("compute expected concurrent block digest");
        insert_consensus_block(&pool, block_number, block_hash, parent_block_hash).await;
        expected_blocks.push(ExpectedBlock {
            number: block_number,
            block_hash,
            parent_block_hash,
            descriptors,
            content_digest,
        });
        if predecessor_was_deleted {
            let successor =
                wait_for_published_manifest_reference(&pool, block_number, block_hash).await;
            assert_successor_was_published(&pool, block_hash, successor.manifest_digest).await;
        }
        parent_block_hash = block_hash;
    }
    producer_done.store(true, Ordering::Release);

    tokio::time::timeout(Duration::from_secs(60), async {
        for worker in workers {
            worker.await.expect("concurrent manifest worker panicked");
        }
    })
    .await
    .expect("concurrent manifest workers did not drain the chain");

    let expected_manifest_count = u64::try_from(last_block_number.div_euclid(cadence) + 1).unwrap();
    assert_eq!(
        stats.sealed_blocks.load(Ordering::Acquire),
        block_count,
        "each block must be sealed exactly once"
    );
    assert_eq!(
        stats.published_manifests.load(Ordering::Acquire),
        expected_manifest_count,
        "each cadence manifest must be published exactly once"
    );
    assert!(
        stats.busy_locks.load(Ordering::Acquire) > 0,
        "concurrent workers never observed the production chain lock as busy"
    );
    if inject_missing_previous {
        assert!(
            !deleted_manifest_digests.is_empty(),
            "missing-predecessor simulation injected no faults"
        );
    }

    let distinct_manifest_workers = {
        let owners = stats.manifest_owners.lock().expect("manifest owners lock");
        assert_eq!(
            owners.len(),
            usize::try_from(expected_manifest_count).unwrap()
        );
        for block_number in (0..=last_block_number).step_by(usize::try_from(cadence).unwrap()) {
            let block_hash = expected_block(&expected_blocks, block_number).block_hash;
            assert!(
                owners.contains_key(&(block_number, block_hash)),
                "manifest block {block_number} ({block_hash}) has no unique worker owner"
            );
        }
        owners.values().copied().collect::<BTreeSet<_>>().len()
    };

    let generated_manifests = stats
        .signed_manifests
        .lock()
        .expect("signed manifests lock")
        .values()
        .cloned()
        .collect::<Vec<_>>();
    assert_generated_manifests_are_canonical(
        &generated_manifests,
        &expected_blocks,
        publisher,
        expected_manifest_count,
    );
    assert_persisted_canonical_state(
        &pool,
        cadence,
        &expected_blocks,
        &generated_manifests,
        &deleted_manifest_digests,
    )
    .await;

    let progressing_workers = stats
        .worker_progress
        .iter()
        .filter(|progress| progress.load(Ordering::Acquire) > 0)
        .count();
    println!(
        "concurrent manifest simulation complete: blocks={block_count} manifests={expected_manifest_count} workers={worker_count} progressing_workers={progressing_workers} manifest_workers={distinct_manifest_workers} attempts={} busy_locks={} missing_previous_faults={} interval_ms={block_interval_ms} elapsed={:?}",
        stats.attempts.load(Ordering::Acquire),
        stats.busy_locks.load(Ordering::Acquire),
        deleted_manifest_digests.len(),
        started_at.elapsed(),
    );
}

async fn run_concurrent_reorg_simulation() {
    let block_interval_ms = positive_env_u64(
        "SNS_MANIFEST_SIMULATED_BLOCK_INTERVAL_MS",
        SIMULATED_BLOCK_INTERVAL_MS,
    );
    let worker_count = positive_env_u64("SNS_MANIFEST_SIMULATED_WORKERS", CONCURRENT_WORKER_COUNT);
    assert!(
        worker_count >= 2,
        "reorg test requires at least two workers"
    );
    let worker_count = usize::try_from(worker_count).expect("worker count fits usize");
    let cadence = positive_env_i64("SNS_MANIFEST_SOAK_CADENCE", SOAK_CADENCE);

    let test_instance = setup_test_db(ImportMode::None)
        .await
        .expect("create concurrent reorg manifest database");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(u32::try_from(worker_count + 4).expect("pool size fits u32"))
        .connect(test_instance.db_url())
        .await
        .expect("connect concurrent reorg manifest database");
    let signer = Arc::new(PrivateKeySigner::random());
    let producer_done = Arc::new(AtomicBool::new(false));
    let stats = Arc::new(ConcurrentPublisherStats::new(worker_count));
    let start_barrier = Arc::new(Barrier::new(worker_count + 1));
    let mut workers = Vec::with_capacity(worker_count);
    for worker_id in 0..worker_count {
        workers.push(tokio::spawn(concurrent_manifest_worker(
            worker_id,
            pool.clone(),
            cadence,
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
        tokio::time::sleep(Duration::from_millis(block_interval_ms)).await;
    }
    let fork_parent = parent;
    wait_for_published_manifest_reference(&pool, 6, fork_parent).await;

    let mut branch_a_parent = fork_parent;
    for number in 7..=12 {
        branch_a_parent =
            insert_simulated_history_block(&pool, &mut expected_blocks, number, branch_a_parent, 1)
                .await;
        tokio::time::sleep(Duration::from_millis(block_interval_ms)).await;
    }
    wait_for_published_manifest_reference(&pool, 12, branch_a_parent).await;

    // The replacement branch appears only after branch A was fully published.
    // Both histories remain in the database while branch B catches up.
    let mut branch_b_parent = fork_parent;
    for number in 7..=12 {
        branch_b_parent =
            insert_simulated_history_block(&pool, &mut expected_blocks, number, branch_b_parent, 2)
                .await;
        tokio::time::sleep(Duration::from_millis(block_interval_ms)).await;
    }
    wait_for_published_manifest_reference(&pool, 12, branch_b_parent).await;

    // Continue the replacement history beyond the old head to model the
    // settled post-reorg chain while the old manifests still coexist.
    for number in 13..=18 {
        branch_b_parent =
            insert_simulated_history_block(&pool, &mut expected_blocks, number, branch_b_parent, 2)
                .await;
        tokio::time::sleep(Duration::from_millis(block_interval_ms)).await;
    }
    wait_for_published_manifest_reference(&pool, 18, branch_b_parent).await;
    producer_done.store(true, Ordering::Release);

    tokio::time::timeout(Duration::from_secs(60), async {
        for worker in workers {
            worker.await.expect("concurrent reorg worker panicked");
        }
    })
    .await
    .expect("concurrent reorg workers did not drain all histories");

    let expected_manifest_keys = expected_blocks
        .iter()
        .filter(|block| block.number.rem_euclid(cadence) == 0)
        .map(|block| (block.number, block.block_hash))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        stats.sealed_blocks.load(Ordering::Acquire),
        u64::try_from(expected_blocks.len()).unwrap()
    );
    assert_eq!(
        stats.published_manifests.load(Ordering::Acquire),
        u64::try_from(expected_manifest_keys.len()).unwrap()
    );
    assert!(stats.busy_locks.load(Ordering::Acquire) > 0);

    let distinct_manifest_workers = {
        let owners = stats.manifest_owners.lock().expect("manifest owners lock");
        assert_eq!(
            owners.keys().copied().collect::<BTreeSet<_>>(),
            expected_manifest_keys
        );
        owners.values().copied().collect::<BTreeSet<_>>().len()
    };

    let generated = stats
        .signed_manifests
        .lock()
        .expect("signed manifests lock")
        .clone();
    assert_eq!(
        generated.keys().copied().collect::<BTreeSet<_>>(),
        expected_manifest_keys
    );
    assert_reorg_manifests_are_canonical(&generated, &expected_blocks, signer.address(), cadence);
    assert_persisted_reorg_state(&pool, &generated, &expected_blocks).await;

    println!(
        "concurrent reorg simulation complete: blocks={} manifests={} workers={} manifest_workers={} attempts={} busy_locks={} elapsed={:?}",
        expected_blocks.len(),
        expected_manifest_keys.len(),
        worker_count,
        distinct_manifest_workers,
        stats.attempts.load(Ordering::Acquire),
        stats.busy_locks.load(Ordering::Acquire),
        started_at.elapsed(),
    );
}

async fn concurrent_manifest_worker(
    worker_id: usize,
    pool: PgPool,
    cadence: i64,
    signer: Arc<PrivateKeySigner>,
    producer_done: Arc<AtomicBool>,
    start_barrier: Arc<Barrier>,
    stats: Arc<ConcurrentPublisherStats>,
) {
    start_barrier.wait().await;
    loop {
        let chains = pending_chain_ids(&pool, cadence)
            .await
            .expect("list pending chains from concurrent worker");
        if chains.is_empty() {
            if producer_done.load(Ordering::Acquire) {
                return;
            }
            tokio::time::sleep(Duration::from_millis(1)).await;
            continue;
        }

        for host_chain_id in chains {
            stats.attempts.fetch_add(1, Ordering::AcqRel);
            let mut trx = pool
                .begin()
                .await
                .expect("begin concurrent publisher transaction");
            let cursor = ManifestProgressCursor::start();
            let Some(block) =
                lock_next_block_to_progress(&mut trx, host_chain_id, cadence, &cursor)
                    .await
                    .expect("call production manifest chain lock")
            else {
                stats.busy_locks.fetch_add(1, Ordering::AcqRel);
                trx.rollback().await.expect("rollback busy publisher");
                continue;
            };

            // Real publication holds this production transaction lock while it
            // prepares/signs/uploads. A small delay makes that contention
            // deterministic without introducing any test-only lock.
            tokio::time::sleep(Duration::from_millis(2)).await;
            if block.block_content_digest.is_none() {
                assert!(is_block_manifest_ready(&mut trx, &block)
                    .await
                    .expect("check concurrent block readiness"));
                let descriptors = load_manifest_descriptors(&mut trx, &block)
                    .await
                    .expect("load concurrent block descriptors");
                seal_block_content(&mut trx, &block, SOAK_CONTEXT_ID, &descriptors)
                    .await
                    .expect("seal concurrent block");
                trx.commit().await.expect("commit concurrent block seal");
                stats.sealed_blocks.fetch_add(1, Ordering::AcqRel);
            } else {
                let block_hash = B256::from_slice(&block.block_hash);
                {
                    let mut owners = stats.manifest_owners.lock().expect("manifest owners lock");
                    assert!(
                        owners
                            .insert((block.block_number, block_hash), worker_id)
                            .is_none(),
                        "more than one worker acquired manifest block {} ({block_hash})",
                        block.block_number,
                    );
                }
                let prepared =
                    prepare_manifest(&mut trx, &block, SOAK_CONTEXT_ID, signer.address())
                        .await
                        .expect("prepare concurrent manifest");
                let detailed_range_start =
                    i64::try_from(prepared.payload.detailed_range.first_block_number)
                        .expect("detailed range start fits BIGINT");
                let detailed_range_digest = prepared.payload.detailed_range.digest;
                let signed = prepared
                    .payload
                    .sign(signer.as_ref())
                    .await
                    .expect("sign concurrent manifest");
                signed.verify().expect("verify concurrent manifest");
                let manifest_digest = signed.digest().expect("digest concurrent manifest");
                let body = serde_json::to_vec(&signed).expect("serialize concurrent manifest");
                let object_key = manifest_object_key(&signed);
                store_authenticated_manifest(&mut trx, signer.address(), &object_key, &body)
                    .await
                    .expect("archive concurrent manifest");
                mark_manifest_published(
                    &mut trx,
                    &block,
                    detailed_range_start,
                    detailed_range_digest,
                    manifest_digest,
                )
                .await
                .expect("persist concurrent manifest");
                trx.commit().await.expect("commit concurrent manifest");
                let mut manifests = stats
                    .signed_manifests
                    .lock()
                    .expect("signed manifests lock");
                assert!(
                    manifests
                        .insert((block.block_number, block_hash), signed)
                        .is_none(),
                    "more than one signed manifest was produced for block {} ({block_hash})",
                    block.block_number,
                );
                stats.published_manifests.fetch_add(1, Ordering::AcqRel);
            }
            stats.worker_progress[worker_id].fetch_add(1, Ordering::AcqRel);
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    }
}

fn assert_generated_manifests_are_canonical(
    manifests: &[SignedManifest],
    expected_blocks: &[ExpectedBlock],
    publisher: alloy_primitives::Address,
    expected_manifest_count: u64,
) {
    assert_eq!(
        manifests.len(),
        usize::try_from(expected_manifest_count).unwrap()
    );

    let mut previous = None;
    for signed in manifests {
        signed.verify().expect("verify stored concurrent manifest");
        assert_eq!(signed.payload.publisher, publisher);
        assert_manifest_is_canonical(&signed.payload, expected_blocks, previous.as_ref());
        let digest = signed.digest().expect("digest stored concurrent manifest");
        previous = Some(ManifestReference {
            block_number: signed.payload.publication_block_number,
            block_hash: signed.payload.publication_block_hash,
            revision: signed.payload.revision,
            manifest_digest: digest,
        });
    }
}

async fn wait_for_published_manifest_reference(
    pool: &PgPool,
    block_number: i64,
    block_hash: B256,
) -> ManifestReference {
    tokio::time::timeout(Duration::from_secs(30), async {
        loop {
            let row = sqlx::query(
                "SELECT manifest_revision, manifest_digest, manifest_published
                   FROM block_consensus
                  WHERE host_chain_id = $1
                    AND block_number = $2
                    AND block_hash = $3",
            )
            .bind(SOAK_CHAIN_ID)
            .bind(block_number)
            .bind(block_hash.as_slice())
            .fetch_optional(pool)
            .await
            .expect("load predecessor publication state");
            if let Some(row) = row {
                let published: bool = row.get("manifest_published");
                let digest: Option<Vec<u8>> = row.get("manifest_digest");
                if published {
                    let digest = digest.expect("published predecessor has a manifest digest");
                    return ManifestReference {
                        block_number: U256::from(block_number as u64),
                        block_hash,
                        revision: u64::try_from(row.get::<i64, _>("manifest_revision"))
                            .expect("manifest revision is non-negative"),
                        manifest_digest: B256::from_slice(&digest),
                    };
                }
            }
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    })
    .await
    .expect("predecessor manifest was not published in time")
}

async fn delete_previous_manifest(pool: &PgPool, previous: &ManifestReference) -> B256 {
    let revision = i64::try_from(previous.revision).expect("manifest revision fits BIGINT");
    let row = sqlx::query(
        "DELETE FROM block_consensus_manifest
          WHERE host_chain_id = $1
            AND publication_block_hash = $2
            AND revision = $3
        RETURNING publication_block_number, publication_block_hash, revision,
                  manifest_digest, object_key, signed_manifest",
    )
    .bind(SOAK_CHAIN_ID)
    .bind(previous.block_hash.as_slice())
    .bind(revision)
    .fetch_one(pool)
    .await
    .expect("delete previous signed manifest for fault injection");
    let publication_block_number: i64 = row.get("publication_block_number");
    let publication_block_hash: Vec<u8> = row.get("publication_block_hash");
    let stored_revision: i64 = row.get("revision");
    let manifest_digest: Vec<u8> = row.get("manifest_digest");
    let object_key: String = row.get("object_key");
    let signed_manifest: Vec<u8> = row.get("signed_manifest");
    assert_eq!(
        publication_block_number,
        i64::try_from(previous.block_number).unwrap()
    );
    assert_eq!(publication_block_hash, previous.block_hash.as_slice());
    assert_eq!(stored_revision, revision);
    assert_eq!(manifest_digest, previous.manifest_digest.as_slice());
    let signed: SignedManifest =
        serde_json::from_slice(&signed_manifest).expect("decode deleted signed manifest");
    signed.verify().expect("verify deleted signed manifest");
    assert_eq!(signed.digest().unwrap(), previous.manifest_digest);
    assert_eq!(object_key, manifest_object_key(&signed));
    previous.manifest_digest
}

async fn assert_successor_was_published(pool: &PgPool, block_hash: B256, manifest_digest: B256) {
    let state = sqlx::query(
        "SELECT b.manifest_published, b.manifest_digest,
                COUNT(m.*) AS manifest_body_count
           FROM block_consensus b
           LEFT JOIN block_consensus_manifest m
             ON m.host_chain_id = b.host_chain_id
            AND m.publication_block_hash = b.block_hash
          WHERE b.host_chain_id = $1 AND b.block_hash = $2
          GROUP BY b.manifest_published, b.manifest_digest",
    )
    .bind(SOAK_CHAIN_ID)
    .bind(block_hash.as_slice())
    .fetch_one(pool)
    .await
    .expect("load successor publication after missing-predecessor reconstruction");
    assert!(state.get::<bool, _>("manifest_published"));
    assert_eq!(
        state
            .get::<Option<Vec<u8>>, _>("manifest_digest")
            .as_deref(),
        Some(manifest_digest.as_slice())
    );
    assert_eq!(state.get::<i64, _>("manifest_body_count"), 1);
}

fn assert_manifest_is_canonical(
    payload: &ManifestPayload,
    expected_blocks: &[ExpectedBlock],
    previous_manifest: Option<&ManifestReference>,
) {
    payload.validate().expect("manifest payload validates");
    assert_eq!(payload.version, ManifestVersion::V1);
    assert_eq!(payload.coprocessor_context_id, SOAK_CONTEXT_ID);
    assert_eq!(payload.host_chain_id, U256::from(SOAK_CHAIN_ID as u64));
    assert_eq!(payload.previous_manifest.as_ref(), previous_manifest);

    let publication_number =
        i64::try_from(payload.publication_block_number).expect("publication number fits BIGINT");
    let expected_publication = expected_block(expected_blocks, publication_number);
    assert_eq!(
        payload.publication_block_hash,
        expected_publication.block_hash
    );
    assert_eq!(
        payload.publication_parent_block_hash,
        expected_publication.parent_block_hash
    );

    let first = i64::try_from(payload.detailed_range.first_block_number)
        .expect("detailed range start fits BIGINT");
    let expected_first = previous_manifest
        .map(|previous| i64::try_from(previous.block_number).unwrap() + 1)
        .unwrap_or(0);
    assert_eq!(first, expected_first);
    assert_eq!(
        i64::try_from(payload.detailed_range.last_block_number).unwrap(),
        publication_number
    );

    let expected_detailed = &expected_blocks
        [usize::try_from(first).unwrap()..=usize::try_from(publication_number).unwrap()];
    assert_eq!(payload.detailed_range.blocks.len(), expected_detailed.len());
    let mut detailed_digests = Vec::with_capacity(expected_detailed.len());
    for (actual, expected) in payload.detailed_range.blocks.iter().zip(expected_detailed) {
        assert_eq!(actual.block_number, U256::from(expected.number as u64));
        assert_eq!(actual.block_hash, expected.block_hash);
        assert_eq!(actual.parent_block_hash, expected.parent_block_hash);
        assert_eq!(actual.ciphertexts, expected.descriptors);
        let recomputed = block_content_digest(
            ManifestVersion::V1,
            SOAK_CONTEXT_ID,
            U256::from(SOAK_CHAIN_ID as u64),
            actual.block_number,
            actual.block_hash,
            &actual.ciphertexts,
        )
        .expect("recompute manifest block content digest");
        assert_eq!(recomputed, expected.content_digest);
        assert_eq!(actual.block_content_digest, recomputed);
        detailed_digests.push(recomputed);
    }
    assert_eq!(
        payload.detailed_range.digest,
        detailed_range_digest(
            ManifestVersion::V1,
            SOAK_CONTEXT_ID,
            U256::from(SOAK_CHAIN_ID as u64),
            U256::from(first as u64),
            U256::from(publication_number as u64),
            &detailed_digests,
        )
    );

    let expected_history = independent_canonical_cover(first);
    assert_eq!(payload.historical_ranges.len(), expected_history.len());
    for (actual, (start, end, scale)) in payload.historical_ranges.iter().zip(expected_history) {
        assert_eq!(actual.start_block_number, U256::from(start as u64));
        assert_eq!(actual.end_block_number, U256::from(end as u64));
        assert_eq!(actual.scale, scale);
        assert_eq!(
            actual.end_block_hash,
            expected_block(expected_blocks, end).block_hash
        );
        assert_eq!(
            actual.digest,
            independently_compute_range_digest(expected_blocks, start, end, scale)
        );
    }
}

fn assert_reorg_manifests_are_canonical(
    manifests: &BTreeMap<(i64, B256), SignedManifest>,
    expected_blocks: &[ExpectedBlock],
    publisher: alloy_primitives::Address,
    cadence: i64,
) {
    let expected_by_hash = expected_blocks
        .iter()
        .map(|block| (block.block_hash, block))
        .collect::<BTreeMap<_, _>>();

    for ((publication_number, publication_hash), signed) in manifests {
        signed.verify().expect("verify reorg manifest");
        signed.payload.validate().expect("validate reorg manifest");
        assert_eq!(signed.payload.publisher, publisher);
        assert_eq!(
            signed.payload.publication_block_number,
            U256::from(*publication_number as u64)
        );
        assert_eq!(signed.payload.publication_block_hash, *publication_hash);
        let publication = expected_by_hash
            .get(publication_hash)
            .expect("reorg publication block exists");
        assert_eq!(publication.number, *publication_number);

        let previous =
            expected_previous_manifest(publication, manifests, &expected_by_hash, cadence);
        let expected_previous_reference = previous.map(|manifest| ManifestReference {
            block_number: manifest.payload.publication_block_number,
            block_hash: manifest.payload.publication_block_hash,
            revision: manifest.payload.revision,
            manifest_digest: manifest.digest().expect("digest previous reorg manifest"),
        });
        assert_eq!(
            signed.payload.previous_manifest, expected_previous_reference,
            "reorg manifest at block {} ({publication_hash}) references the wrong history",
            publication.number,
        );

        let detailed = expected_detailed_branch(
            publication,
            expected_previous_reference.as_ref(),
            &expected_by_hash,
        );
        assert_eq!(signed.payload.detailed_range.blocks.len(), detailed.len());
        let mut block_digests = Vec::with_capacity(detailed.len());
        for (actual, expected) in signed.payload.detailed_range.blocks.iter().zip(&detailed) {
            assert_eq!(actual.block_number, U256::from(expected.number as u64));
            assert_eq!(actual.block_hash, expected.block_hash);
            assert_eq!(actual.parent_block_hash, expected.parent_block_hash);
            assert_eq!(actual.ciphertexts, expected.descriptors);
            let digest = block_content_digest(
                ManifestVersion::V1,
                SOAK_CONTEXT_ID,
                U256::from(SOAK_CHAIN_ID as u64),
                actual.block_number,
                actual.block_hash,
                &actual.ciphertexts,
            )
            .expect("recompute reorg block digest");
            assert_eq!(digest, expected.content_digest);
            assert_eq!(actual.block_content_digest, digest);
            block_digests.push(digest);
        }
        let first = detailed.first().expect("non-empty reorg detailed range");
        let last = detailed.last().expect("non-empty reorg detailed range");
        assert_eq!(
            signed.payload.detailed_range.digest,
            detailed_range_digest(
                ManifestVersion::V1,
                SOAK_CONTEXT_ID,
                U256::from(SOAK_CHAIN_ID as u64),
                U256::from(first.number as u64),
                U256::from(last.number as u64),
                &block_digests,
            )
        );

        let expected_history = independent_canonical_cover(first.number);
        assert_eq!(
            signed.payload.historical_ranges.len(),
            expected_history.len()
        );
        for (actual, (start, end, scale)) in signed
            .payload
            .historical_ranges
            .iter()
            .zip(expected_history)
        {
            assert_eq!(actual.start_block_number, U256::from(start as u64));
            assert_eq!(actual.end_block_number, U256::from(end as u64));
            assert_eq!(actual.scale, scale);
            assert_eq!(
                actual.digest,
                independently_compute_history_range_digest(
                    &expected_by_hash,
                    start,
                    end,
                    scale,
                    actual.end_block_hash,
                )
            );
        }
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

fn expected_previous_manifest<'a>(
    publication: &ExpectedBlock,
    manifests: &'a BTreeMap<(i64, B256), SignedManifest>,
    expected_by_hash: &BTreeMap<B256, &ExpectedBlock>,
    cadence: i64,
) -> Option<&'a SignedManifest> {
    let mut parent_hash = publication.parent_block_hash;
    while let Some(parent) = expected_by_hash.get(&parent_hash) {
        if parent.number.rem_euclid(cadence) == 0 {
            return Some(
                manifests
                    .get(&(parent.number, parent.block_hash))
                    .expect("published reorg ancestor manifest exists"),
            );
        }
        parent_hash = parent.parent_block_hash;
    }
    None
}

fn expected_detailed_branch<'a>(
    publication: &'a ExpectedBlock,
    previous: Option<&ManifestReference>,
    expected_by_hash: &BTreeMap<B256, &'a ExpectedBlock>,
) -> Vec<&'a ExpectedBlock> {
    let mut reverse = Vec::new();
    let mut current = publication;
    loop {
        reverse.push(current);
        if previous.is_some_and(|reference| current.parent_block_hash == reference.block_hash) {
            break;
        }
        let Some(parent) = expected_by_hash.get(&current.parent_block_hash) else {
            assert!(previous.is_none());
            break;
        };
        current = parent;
    }
    reverse.reverse();
    reverse
}

fn independently_compute_history_range_digest(
    expected_by_hash: &BTreeMap<B256, &ExpectedBlock>,
    start: i64,
    end: i64,
    scale: u32,
    end_block_hash: B256,
) -> B256 {
    let mut reverse = Vec::new();
    let mut current = expected_by_hash
        .get(&end_block_hash)
        .expect("historical range end belongs to a known reorg history");
    assert_eq!(current.number, end);
    loop {
        reverse.push(*current);
        if current.number == start {
            break;
        }
        current = expected_by_hash
            .get(&current.parent_block_hash)
            .expect("historical range is contiguous on one reorg history");
    }
    reverse.reverse();
    assert_eq!(reverse.len(), 1_usize << scale);
    compute_history_range_digest_from_blocks(&reverse, scale)
}

fn compute_history_range_digest_from_blocks(blocks: &[&ExpectedBlock], scale: u32) -> B256 {
    if scale == 0 {
        assert_eq!(blocks.len(), 1);
        return blocks[0].content_digest;
    }
    let middle = blocks.len() / 2;
    let left = compute_history_range_digest_from_blocks(&blocks[..middle], scale - 1);
    let right = compute_history_range_digest_from_blocks(&blocks[middle..], scale - 1);
    let first = blocks.first().unwrap();
    let last = blocks.last().unwrap();
    dyadic_range_digest(
        ManifestVersion::V1,
        SOAK_CONTEXT_ID,
        U256::from(SOAK_CHAIN_ID as u64),
        U256::from(first.number as u64),
        U256::from(last.number as u64),
        scale,
        last.block_hash,
        left,
        right,
    )
}

async fn assert_persisted_reorg_state(
    pool: &PgPool,
    generated: &BTreeMap<(i64, B256), SignedManifest>,
    expected_blocks: &[ExpectedBlock],
) {
    let expected_by_hash = expected_blocks
        .iter()
        .map(|block| (block.block_hash, block))
        .collect::<BTreeMap<_, _>>();
    let unsealed: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM block_consensus
          WHERE host_chain_id = $1
            AND (block_content_digest IS NULL OR descriptor_count IS NULL)",
    )
    .bind(SOAK_CHAIN_ID)
    .fetch_one(pool)
    .await
    .expect("count unsealed reorg blocks");
    assert_eq!(unsealed, 0);

    let rows = sqlx::query(
        "SELECT publication_block_number, publication_block_hash, signed_manifest
           FROM block_consensus_manifest
          WHERE host_chain_id = $1",
    )
    .bind(SOAK_CHAIN_ID)
    .fetch_all(pool)
    .await
    .expect("load persisted reorg manifests");
    assert_eq!(rows.len(), generated.len());
    for row in rows {
        let number: i64 = row.get("publication_block_number");
        let hash_bytes: Vec<u8> = row.get("publication_block_hash");
        let hash = B256::from_slice(&hash_bytes);
        let body: Vec<u8> = row.get("signed_manifest");
        let signed: SignedManifest =
            serde_json::from_slice(&body).expect("decode persisted reorg manifest");
        assert_eq!(generated.get(&(number, hash)), Some(&signed));
    }

    let range_rows = sqlx::query(
        "SELECT range_start, range_end, scale,
                range_start_block_hash, range_start_parent_block_hash,
                range_end_block_hash, range_digest
           FROM block_consensus_range
          WHERE host_chain_id = $1",
    )
    .bind(SOAK_CHAIN_ID)
    .fetch_all(pool)
    .await
    .expect("load persisted reorg ranges");
    assert!(!range_rows.is_empty());
    for row in range_rows {
        let start: i64 = row.get("range_start");
        let end: i64 = row.get("range_end");
        let scale = u32::try_from(row.get::<i32, _>("scale")).unwrap();
        let start_hash: Vec<u8> = row.get("range_start_block_hash");
        let start_parent_hash: Vec<u8> = row.get("range_start_parent_block_hash");
        let end_hash_bytes: Vec<u8> = row.get("range_end_block_hash");
        let end_hash = B256::from_slice(&end_hash_bytes);
        let digest: Vec<u8> = row.get("range_digest");
        let range_digest = independently_compute_history_range_digest(
            &expected_by_hash,
            start,
            end,
            scale,
            end_hash,
        );
        let lineage_start = expected_by_hash
            .get(&B256::from_slice(&start_hash))
            .expect("persisted reorg range starts on a known history");
        assert_eq!(lineage_start.number, start);
        assert_eq!(
            lineage_start.parent_block_hash.as_slice(),
            start_parent_hash
        );
        assert_eq!(range_digest.as_slice(), digest);
    }
}

async fn assert_persisted_canonical_state(
    pool: &PgPool,
    cadence: i64,
    expected_blocks: &[ExpectedBlock],
    expected_manifests: &[SignedManifest],
    deleted_manifest_digests: &BTreeSet<B256>,
) {
    let unsealed: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM block_consensus
          WHERE host_chain_id = $1
            AND (block_content_digest IS NULL OR descriptor_count IS NULL)",
    )
    .bind(SOAK_CHAIN_ID)
    .fetch_one(pool)
    .await
    .expect("count unsealed consensus blocks");
    assert_eq!(unsealed, 0, "every generated block must be sealed");

    let incorrectly_published: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM block_consensus
          WHERE host_chain_id = $1
            AND manifest_published
            AND MOD(block_number, $2) <> 0",
    )
    .bind(SOAK_CHAIN_ID)
    .bind(cadence)
    .fetch_one(pool)
    .await
    .expect("count off-cadence manifests");
    assert_eq!(incorrectly_published, 0);

    let published_blocks: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM block_consensus
          WHERE host_chain_id = $1 AND manifest_published",
    )
    .bind(SOAK_CHAIN_ID)
    .fetch_one(pool)
    .await
    .expect("count published manifest blocks");
    assert_eq!(
        usize::try_from(published_blocks).unwrap(),
        expected_manifests.len(),
        "every cadence block must publish even after predecessor deletion"
    );

    let rows = sqlx::query(
        "SELECT publication_block_number, manifest_digest, object_key, signed_manifest
           FROM block_consensus_manifest
          WHERE host_chain_id = $1
          ORDER BY publication_block_number",
    )
    .bind(SOAK_CHAIN_ID)
    .fetch_all(pool)
    .await
    .expect("load persisted manifests");
    let expected_persisted = expected_manifests
        .iter()
        .filter(|manifest| {
            !deleted_manifest_digests.contains(&manifest.digest().expect("digest manifest"))
        })
        .collect::<Vec<_>>();
    assert_eq!(rows.len(), expected_persisted.len());
    for (row, expected) in rows.iter().zip(expected_persisted) {
        let publication_number: i64 = row.get("publication_block_number");
        let stored_digest: Vec<u8> = row.get("manifest_digest");
        let stored_key: String = row.get("object_key");
        let stored_body: Vec<u8> = row.get("signed_manifest");
        let decoded: SignedManifest =
            serde_json::from_slice(&stored_body).expect("decode stored signed manifest");
        assert_eq!(decoded, *expected);
        decoded.verify().expect("verify stored signed manifest");
        assert_eq!(
            publication_number,
            i64::try_from(expected.payload.publication_block_number).unwrap()
        );
        assert_eq!(stored_digest, expected.digest().unwrap().as_slice());
        assert_eq!(stored_key, manifest_object_key(expected));
    }
    for deleted_digest in deleted_manifest_digests {
        let stored: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM block_consensus_manifest
              WHERE host_chain_id = $1 AND manifest_digest = $2",
        )
        .bind(SOAK_CHAIN_ID)
        .bind(deleted_digest.as_slice())
        .fetch_one(pool)
        .await
        .expect("confirm deleted predecessor remains absent");
        assert_eq!(stored, 0);
    }

    let range_rows = sqlx::query(
        "SELECT range_start, range_end, scale,
                range_start_block_hash, range_start_parent_block_hash,
                range_end_block_hash, range_digest
           FROM block_consensus_range
          WHERE host_chain_id = $1
          ORDER BY scale, range_start, range_end",
    )
    .bind(SOAK_CHAIN_ID)
    .fetch_all(pool)
    .await
    .expect("load persisted canonical ranges");
    assert!(!range_rows.is_empty(), "the soak must persist range roots");
    for row in range_rows {
        let start: i64 = row.get("range_start");
        let end: i64 = row.get("range_end");
        let scale: i32 = row.get("scale");
        let start_hash: Vec<u8> = row.get("range_start_block_hash");
        let start_parent_hash: Vec<u8> = row.get("range_start_parent_block_hash");
        let end_hash: Vec<u8> = row.get("range_end_block_hash");
        let digest: Vec<u8> = row.get("range_digest");
        let scale = u32::try_from(scale).expect("stored range scale is non-negative");
        let size = 1_i64
            .checked_shl(scale)
            .expect("stored range scale fits BIGINT");
        assert_eq!(end - start + 1, size);
        assert_eq!(start.rem_euclid(size), 0);
        assert_eq!(
            start_hash,
            expected_block(expected_blocks, start).block_hash.as_slice()
        );
        assert_eq!(
            start_parent_hash,
            expected_block(expected_blocks, start)
                .parent_block_hash
                .as_slice()
        );
        assert_eq!(
            end_hash,
            expected_block(expected_blocks, end).block_hash.as_slice()
        );
        assert_eq!(
            digest,
            independently_compute_range_digest(expected_blocks, start, end, scale).as_slice()
        );
    }
}

fn independent_canonical_cover(mut upper: i64) -> Vec<(i64, i64, u32)> {
    let mut newest_to_oldest = Vec::new();
    let mut previous_scale = 0_u32;
    while upper > 0 {
        let larger_scale = previous_scale + 1;
        let larger_size = 1_i64 << larger_scale;
        let scale = if upper.rem_euclid(larger_size) == 0 {
            larger_scale
        } else {
            previous_scale
        };
        let size = 1_i64 << scale;
        let start = upper - size;
        newest_to_oldest.push((start, upper - 1, scale));
        upper = start;
        previous_scale = scale;
    }
    newest_to_oldest
}

fn independently_compute_range_digest(
    blocks: &[ExpectedBlock],
    start: i64,
    end: i64,
    scale: u32,
) -> B256 {
    if scale == 0 {
        assert_eq!(start, end);
        return expected_block(blocks, start).content_digest;
    }

    let child_size = 1_i64 << (scale - 1);
    let left_end = start + child_size - 1;
    let right_start = left_end + 1;
    let left = independently_compute_range_digest(blocks, start, left_end, scale - 1);
    let right = independently_compute_range_digest(blocks, right_start, end, scale - 1);
    dyadic_range_digest(
        ManifestVersion::V1,
        SOAK_CONTEXT_ID,
        U256::from(SOAK_CHAIN_ID as u64),
        U256::from(start as u64),
        U256::from(end as u64),
        scale,
        expected_block(blocks, end).block_hash,
        left,
        right,
    )
}

async fn insert_simulated_history_block(
    pool: &PgPool,
    expected_blocks: &mut Vec<ExpectedBlock>,
    block_number: i64,
    parent_block_hash: B256,
    history_variant: u32,
) -> B256 {
    let block_hash = soak_value(*b"soakblok", block_number, history_variant);
    let descriptors =
        seed_block_descriptors_for_history(pool, block_number, block_hash, history_variant).await;
    let content_digest = block_content_digest(
        ManifestVersion::V1,
        SOAK_CONTEXT_ID,
        U256::from(SOAK_CHAIN_ID as u64),
        U256::from(block_number as u64),
        block_hash,
        &descriptors,
    )
    .expect("compute simulated history block digest");
    insert_consensus_block(pool, block_number, block_hash, parent_block_hash).await;
    expected_blocks.push(ExpectedBlock {
        number: block_number,
        block_hash,
        parent_block_hash,
        descriptors,
        content_digest,
    });
    block_hash
}

async fn insert_consensus_block(
    pool: &PgPool,
    block_number: i64,
    block_hash: B256,
    parent_block_hash: B256,
) {
    sqlx::query(
        "INSERT INTO block_consensus (
             host_chain_id, block_number, block_hash, parent_block_hash
         ) VALUES ($1, $2, $3, $4)",
    )
    .bind(SOAK_CHAIN_ID)
    .bind(block_number)
    .bind(block_hash.as_slice())
    .bind(parent_block_hash.as_slice())
    .execute(pool)
    .await
    .expect("insert synthetic consensus block");
}

async fn seed_block_descriptors(
    pool: &PgPool,
    block_number: i64,
    block_hash: B256,
) -> Vec<CiphertextDescriptor> {
    seed_block_descriptors_for_history(pool, block_number, block_hash, 0).await
}

async fn seed_block_descriptors_for_history(
    pool: &PgPool,
    block_number: i64,
    block_hash: B256,
    history_variant: u32,
) -> Vec<CiphertextDescriptor> {
    // Include regular empty blocks and vary the non-empty cardinality so the
    // simulation repeatedly exercises canonical ordering and descriptor counts.
    let descriptor_count = if block_number.rem_euclid(5) == 0 {
        0
    } else {
        usize::try_from(block_number.rem_euclid(3) + 1).unwrap()
    };
    let mut descriptors = Vec::with_capacity(descriptor_count);
    for ordinal in 0..descriptor_count {
        let ordinal = history_variant
            .checked_shl(16)
            .and_then(|prefix| prefix.checked_add(u32::try_from(ordinal).unwrap()))
            .expect("history descriptor ordinal fits u32");
        let handle = soak_value(*b"soakhand", block_number, ordinal);
        let transaction_id = soak_value(*b"soaktxid", block_number, ordinal);
        let dependence_chain_id = soak_value(*b"soakdcid", block_number, ordinal);
        let gateway_key_id = soak_value(*b"soakkey_", block_number, ordinal);
        let ct64_digest = soak_value(*b"soakct64", block_number, ordinal);
        let ct128_digest = soak_value(*b"soak128_", block_number, ordinal);

        sqlx::query(
            "INSERT INTO computations_branch (
                 output_handle, dependencies, fhe_operation, is_scalar,
                 dependence_chain_id, transaction_id, is_allowed,
                 schedule_order, is_completed, is_error, host_chain_id,
                 block_number, producer_block_hash
             ) VALUES (
                 $1, ARRAY[]::BYTEA[], 0, FALSE, $2, $3, TRUE,
                 NOW(), TRUE, FALSE, $4, $5, $6
             )",
        )
        .bind(handle.as_slice())
        .bind(dependence_chain_id.as_slice())
        .bind(transaction_id.as_slice())
        .bind(SOAK_CHAIN_ID)
        .bind(block_number)
        .bind(block_hash.as_slice())
        .execute(pool)
        .await
        .expect("insert completed manifest computation");

        sqlx::query(
            "INSERT INTO pbs_computations_branch (
                 handle, host_chain_id, block_number, producer_block_hash,
                 block_hash, is_completed, is_error
             ) VALUES ($1, $2, $3, $4, $5, TRUE, FALSE)",
        )
        .bind(handle.as_slice())
        .bind(SOAK_CHAIN_ID)
        .bind(block_number)
        .bind(block_hash.as_slice())
        .bind(block_hash.as_slice())
        .execute(pool)
        .await
        .expect("insert completed manifest PBS witness");

        sqlx::query(
            "INSERT INTO keys (
                 key_id_gw, key_id, pks_key, sks_key, chain_id, block_hash
             ) VALUES ($1, $1, ''::BYTEA, ''::BYTEA, $2, $3)
             ON CONFLICT (chain_id, block_hash, key_id_gw) DO NOTHING",
        )
        .bind(gateway_key_id.as_slice())
        .bind(SOAK_CHAIN_ID)
        .bind(block_hash.as_slice())
        .execute(pool)
        .await
        .expect("insert manifest keyset identity");

        sqlx::query(
            "INSERT INTO ciphertext_digest_branch (
                 host_chain_id, key_id_gw, handle, producer_block_hash,
                 block_hash, block_number, ciphertext, ciphertext128,
                 ciphertext128_format
             ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 11)",
        )
        .bind(SOAK_CHAIN_ID)
        .bind(gateway_key_id.as_slice())
        .bind(handle.as_slice())
        .bind(block_hash.as_slice())
        .bind(block_hash.as_slice())
        .bind(block_number)
        .bind(ct64_digest.as_slice())
        .bind(ct128_digest.as_slice())
        .execute(pool)
        .await
        .expect("insert complete manifest digest row");

        descriptors.push(CiphertextDescriptor {
            handle,
            keyset_id: U256::from_be_slice(gateway_key_id.as_slice()),
            gateway_key_id: Some(U256::from_be_slice(gateway_key_id.as_slice())),
            ct64_digest,
            ct128_digest,
            ct128_format: CiphertextFormat::CompressedOnCpu,
        });
    }
    descriptors.sort_by(|left, right| left.handle.as_slice().cmp(right.handle.as_slice()));
    descriptors
}

fn expected_block(blocks: &[ExpectedBlock], number: i64) -> &ExpectedBlock {
    let block = &blocks[usize::try_from(number).expect("block number is non-negative")];
    assert_eq!(block.number, number);
    block
}

fn soak_value(tag: [u8; 8], block_number: i64, ordinal: u32) -> B256 {
    let mut bytes = [0_u8; 32];
    bytes[..8].copy_from_slice(&tag);
    bytes[8..16].copy_from_slice(&block_number.to_be_bytes());
    bytes[28..].copy_from_slice(&ordinal.to_be_bytes());
    B256::from(bytes)
}

fn positive_env_u64(name: &str, default: u64) -> u64 {
    match std::env::var(name) {
        Ok(value) => value
            .parse::<u64>()
            .ok()
            .filter(|value| *value > 0)
            .unwrap_or_else(|| panic!("{name} must be a positive integer")),
        Err(std::env::VarError::NotPresent) => default,
        Err(err) => panic!("failed to read {name}: {err}"),
    }
}

fn positive_env_i64(name: &str, default: i64) -> i64 {
    match std::env::var(name) {
        Ok(value) => value
            .parse::<i64>()
            .ok()
            .filter(|value| *value > 0)
            .unwrap_or_else(|| panic!("{name} must be a positive integer")),
        Err(std::env::VarError::NotPresent) => default,
        Err(err) => panic!("failed to read {name}: {err}"),
    }
}
