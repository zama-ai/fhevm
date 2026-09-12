use super::{
    block_discovery::*, manifest_builder::*, manifest_frontier::append_leaf,
    manifest_history::historical_ranges, publication_status::*,
};
use crate::manifest_consensus::lineage::{RangeFrontier, RangeNode};
use crate::manifest_consensus::manifest_archive::ManifestReference;
use crate::manifest_consensus::manifest_archive::{
    manifest_object_key, store_authenticated_manifest, ManifestSource,
};
use alloy::signers::local::PrivateKeySigner;
use alloy_primitives::{Address, B256, U256};
use block_manifest::{
    block_content_digest, detailed_range_digest, CiphertextFormat, ManifestPayload,
    ManifestVersion, SignedManifest,
};
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

pub(super) const SEQUENCE_CONTEXT_ID: U256 = U256::ONE;
pub(super) const SEQUENCE_CHAIN_ID: i64 = 7;
pub(super) const SEQUENCE_CADENCE: i64 = 3;
const SHORT_BLOCK_COUNT: u64 = 7;
const SHORT_WORKER_COUNT: u64 = 2;
const BLOCK_INTERVAL_MS: u64 = 5;

#[derive(Clone, Debug)]
pub(super) struct ExpectedBlock {
    pub number: i64,
    pub block_hash: B256,
    pub parent_block_hash: B256,
    pub descriptors: Vec<CiphertextDescriptor>,
    pub content_digest: B256,
}

#[derive(Debug)]
pub(super) struct ConcurrentPublisherStats {
    pub attempts: AtomicU64,
    pub busy_locks: AtomicU64,
    pub sealed_blocks: AtomicU64,
    pub published_manifests: AtomicU64,
    worker_progress: Vec<AtomicU64>,
    manifest_owners: Mutex<BTreeMap<(i64, B256), usize>>,
    signed_manifests: Mutex<BTreeMap<(i64, B256), SignedManifest>>,
}

impl ConcurrentPublisherStats {
    pub(super) fn new(worker_count: usize) -> Self {
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

    pub(super) fn signed_manifests(&self) -> BTreeMap<(i64, B256), SignedManifest> {
        self.signed_manifests
            .lock()
            .expect("signed manifests lock")
            .clone()
    }

    pub(super) fn owner_keys(&self) -> BTreeSet<(i64, B256)> {
        self.manifest_owners
            .lock()
            .expect("manifest owners lock")
            .keys()
            .copied()
            .collect()
    }
}

/// Short concurrent path: two workers race the production lock over a handful
/// of blocks. Cover/digest oracles are the production frontier, not a second
/// implementation. Longer cover trees live in `manifest_frontier` tests.
///
/// `SQLX_OFFLINE=true cargo test -p consensus-detector canonical_manifest_generation_simulation_fast_chain --lib -- --nocapture`
#[tokio::test]
#[serial(db)]
async fn canonical_manifest_generation_simulation_fast_chain() {
    let block_count = positive_env_u64("SNS_MANIFEST_SIMULATED_BLOCKS", SHORT_BLOCK_COUNT);
    let worker_count = usize::try_from(positive_env_u64(
        "SNS_MANIFEST_SIMULATED_WORKERS",
        SHORT_WORKER_COUNT,
    ))
    .unwrap();
    assert!(worker_count >= 2);
    let cadence = positive_env_i64("SNS_MANIFEST_SEQUENCE_CADENCE", SEQUENCE_CADENCE);
    let last_block_number = i64::try_from(block_count - 1).unwrap();

    let test_instance = setup_test_db(ImportMode::None)
        .await
        .expect("create sequence database");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(u32::try_from(worker_count + 4).unwrap())
        .connect(test_instance.db_url())
        .await
        .expect("connect sequence database");
    let signer = Arc::new(PrivateKeySigner::random());
    let publisher = signer.address();
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
    let mut parent_block_hash = B256::repeat_byte(0xff);
    let mut deleted_manifest_digests = BTreeSet::new();
    for block_number in 0..=last_block_number {
        tokio::time::sleep(Duration::from_millis(BLOCK_INTERVAL_MS)).await;
        let block_hash = sequence_value(*b"seqblock", block_number, 0);
        if block_number == cadence {
            let predecessor = expected_block(&expected_blocks, 0);
            let published = wait_for_published_manifest_reference(
                &pool,
                predecessor.number,
                predecessor.block_hash,
            )
            .await;
            deleted_manifest_digests
                .insert(delete_last_published_manifest(&pool, &published).await);
        }
        let descriptors = seed_block_descriptors(&pool, block_number, block_hash, 0).await;
        let content_digest = content_digest(block_number, block_hash, &descriptors);
        insert_consensus_block(&pool, block_number, block_hash, parent_block_hash).await;
        expected_blocks.push(ExpectedBlock {
            number: block_number,
            block_hash,
            parent_block_hash,
            descriptors,
            content_digest,
        });
        if block_number == cadence {
            let successor =
                wait_for_published_manifest_reference(&pool, block_number, block_hash).await;
            assert_successor_was_published(&pool, block_hash, successor.manifest_digest).await;
        }
        parent_block_hash = block_hash;
    }
    producer_done.store(true, Ordering::Release);
    join_workers(workers).await;

    let expected_manifest_count = u64::try_from(last_block_number.div_euclid(cadence) + 1).unwrap();
    assert_eq!(stats.sealed_blocks.load(Ordering::Acquire), block_count);
    assert_eq!(
        stats.published_manifests.load(Ordering::Acquire),
        expected_manifest_count
    );
    assert!(stats.busy_locks.load(Ordering::Acquire) > 0);
    assert!(!deleted_manifest_digests.is_empty());

    let generated = stats
        .signed_manifests
        .lock()
        .expect("signed manifests lock")
        .values()
        .cloned()
        .collect::<Vec<_>>();
    assert_generated_manifests_are_canonical(&generated, &expected_blocks, publisher);
    assert_persisted_canonical_state(
        &pool,
        cadence,
        &expected_blocks,
        &generated,
        &deleted_manifest_digests,
    )
    .await;
    println!(
        "short sequence complete: blocks={block_count} manifests={expected_manifest_count} workers={worker_count} attempts={} busy_locks={} elapsed={:?}",
        stats.attempts.load(Ordering::Acquire),
        stats.busy_locks.load(Ordering::Acquire),
        started_at.elapsed(),
    );
}

pub(super) async fn concurrent_manifest_worker(
    worker_id: usize,
    pool: PgPool,
    signer: Arc<PrivateKeySigner>,
    producer_done: Arc<AtomicBool>,
    start_barrier: Arc<Barrier>,
    stats: Arc<ConcurrentPublisherStats>,
) {
    start_barrier.wait().await;
    loop {
        let chains = pending_chain_ids(&pool)
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
            let Some(block) = lock_next_block_to_progress(&mut trx, host_chain_id, &cursor)
                .await
                .expect("lock production manifest work row")
            else {
                stats.busy_locks.fetch_add(1, Ordering::AcqRel);
                trx.rollback().await.expect("rollback busy publisher");
                continue;
            };

            tokio::time::sleep(Duration::from_millis(2)).await;
            if block.block_content_digest.is_none() {
                assert!(is_block_manifest_ready(&mut trx, &block)
                    .await
                    .expect("check concurrent block readiness"));
                let descriptors = load_manifest_descriptors(&mut trx, &block, false)
                    .await
                    .expect("load concurrent block descriptors");
                seal_block_content(&mut trx, &block, SEQUENCE_CONTEXT_ID, &descriptors)
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
                    prepare_manifest(&mut trx, &block, SEQUENCE_CONTEXT_ID, signer.address())
                        .await
                        .expect("prepare concurrent manifest");
                let signed = prepared
                    .payload
                    .sign(signer.as_ref())
                    .await
                    .expect("sign concurrent manifest");
                signed.verify().expect("verify concurrent manifest");
                let manifest_digest = signed.digest().expect("digest concurrent manifest");
                let body = serde_json::to_vec(&signed).expect("serialize concurrent manifest");
                let object_key = manifest_object_key(&signed);
                store_authenticated_manifest(
                    &mut trx,
                    signer.address(),
                    &object_key,
                    &body,
                    ManifestSource::Local,
                )
                .await
                .expect("archive concurrent manifest");
                mark_manifest_published(&mut trx, &block, signer.address(), manifest_digest)
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

pub(super) async fn join_workers(workers: Vec<tokio::task::JoinHandle<()>>) {
    tokio::time::timeout(Duration::from_secs(60), async {
        for worker in workers {
            worker.await.expect("concurrent manifest worker panicked");
        }
    })
    .await
    .expect("concurrent manifest workers did not drain the chain");
}

fn assert_generated_manifests_are_canonical(
    manifests: &[SignedManifest],
    expected_blocks: &[ExpectedBlock],
    publisher: Address,
) {
    let by_hash = index_blocks(expected_blocks);
    let mut previous = None;
    for signed in manifests {
        signed.verify().expect("verify stored concurrent manifest");
        assert_eq!(signed.payload.publisher, publisher);
        assert_payload_matches_production_cover(&signed.payload, &by_hash, previous.as_ref());
        previous = Some(manifest_reference(signed));
    }
}

pub(super) fn assert_payload_matches_production_cover(
    payload: &ManifestPayload,
    by_hash: &BTreeMap<B256, &ExpectedBlock>,
    last_published: Option<&ManifestReference>,
) {
    payload.validate().expect("manifest payload validates");
    assert_eq!(payload.version, ManifestVersion::V1);
    assert_eq!(
        payload.consensus_epoch,
        block_manifest::LEGACY_CONSENSUS_EPOCH
    );
    assert_eq!(payload.coprocessor_context_id, SEQUENCE_CONTEXT_ID);
    assert_eq!(payload.host_chain_id, U256::from(SEQUENCE_CHAIN_ID as u64));

    let publication = *by_hash
        .get(&payload.publication_block_hash)
        .expect("publication block is in the simulated chain");
    assert_eq!(
        payload.publication_block_number,
        U256::from(publication.number as u64)
    );
    assert_eq!(
        payload.publication_parent_block_hash,
        publication.parent_block_hash
    );

    let detailed = lineage_blocks(
        publication,
        last_published.map(|prev| prev.block_hash),
        by_hash,
    );
    assert_eq!(payload.detailed_range.blocks.len(), detailed.len());
    let mut block_digests = Vec::with_capacity(detailed.len());
    for (actual, expected) in payload.detailed_range.blocks.iter().zip(&detailed) {
        assert_eq!(actual.block_number, U256::from(expected.number as u64));
        assert_eq!(actual.block_hash, expected.block_hash);
        assert_eq!(actual.parent_block_hash, expected.parent_block_hash);
        assert_eq!(actual.ciphertexts, expected.descriptors);
        let recomputed = content_digest(expected.number, expected.block_hash, &actual.ciphertexts);
        assert_eq!(recomputed, expected.content_digest);
        assert_eq!(actual.block_content_digest, recomputed);
        block_digests.push(recomputed);
    }
    let first = detailed.first().expect("non-empty detailed range");
    let last = detailed.last().expect("non-empty detailed range");
    assert_eq!(
        payload.detailed_range.digest,
        detailed_range_digest(
            ManifestVersion::V1,
            SEQUENCE_CONTEXT_ID,
            U256::from(SEQUENCE_CHAIN_ID as u64),
            U256::from(first.number as u64),
            U256::from(last.number as u64),
            &block_digests,
        )
    );

    let history = match by_hash.get(&first.parent_block_hash) {
        Some(parent) => lineage_blocks(parent, None, by_hash),
        None => Vec::new(),
    };
    assert_eq!(
        payload.historical_ranges,
        historical_ranges(&frontier_after(&history))
    );
}

pub(super) fn index_blocks(blocks: &[ExpectedBlock]) -> BTreeMap<B256, &ExpectedBlock> {
    blocks
        .iter()
        .map(|block| (block.block_hash, block))
        .collect()
}

pub(super) fn lineage_blocks<'a>(
    tip: &'a ExpectedBlock,
    stop_at_parent: Option<B256>,
    by_hash: &BTreeMap<B256, &'a ExpectedBlock>,
) -> Vec<&'a ExpectedBlock> {
    let mut reverse = vec![tip];
    let mut current = tip;
    while Some(current.parent_block_hash) != stop_at_parent {
        let Some(parent) = by_hash.get(&current.parent_block_hash) else {
            break;
        };
        reverse.push(*parent);
        current = parent;
    }
    reverse.reverse();
    reverse
}

pub(super) fn frontier_after(blocks: &[&ExpectedBlock]) -> RangeFrontier {
    let mut frontier = RangeFrontier::default();
    for block in blocks {
        append_leaf(
            SEQUENCE_CHAIN_ID,
            SEQUENCE_CONTEXT_ID,
            &mut frontier,
            range_leaf(block),
        )
        .expect("production cover accepts simulated lineage");
    }
    frontier
}

fn range_leaf(block: &ExpectedBlock) -> RangeNode {
    RangeNode {
        start_block_number: block.number,
        end: block.number,
        scale: 0,
        start_block_hash: block.block_hash,
        start_parent_block_hash: block.parent_block_hash,
        end_block_hash: block.block_hash,
        digest: block.content_digest,
    }
}

pub(super) fn committed_ranges(blocks: &[&ExpectedBlock]) -> Vec<RangeNode> {
    let mut frontier = RangeFrontier::default();
    let mut committed = Vec::new();
    for block in blocks {
        let leaf = range_leaf(block);
        committed.push(leaf.clone());
        committed.extend(
            append_leaf(SEQUENCE_CHAIN_ID, SEQUENCE_CONTEXT_ID, &mut frontier, leaf)
                .expect("production cover accepts simulated lineage"),
        );
    }
    committed
}

fn manifest_reference(signed: &SignedManifest) -> ManifestReference {
    ManifestReference {
        generation: signed.payload.consensus_epoch.clone(),
        publisher: signed.payload.publisher,
        block_number: signed.payload.publication_block_number,
        block_hash: signed.payload.publication_block_hash,
        revision: signed.payload.revision,
        manifest_digest: signed.digest().expect("digest stored concurrent manifest"),
    }
}

pub(super) async fn wait_for_published_manifest_reference(
    pool: &PgPool,
    block_number: i64,
    block_hash: B256,
) -> ManifestReference {
    tokio::time::timeout(Duration::from_secs(30), async {
        loop {
            let row = sqlx::query(
                "SELECT manifest_revision, manifest_digest, manifest_publisher,
                        manifest_published
                   FROM block_manifest_state
                  WHERE host_chain_id = $1
                    AND block_number = $2
                    AND block_hash = $3",
            )
            .bind(SEQUENCE_CHAIN_ID)
            .bind(block_number)
            .bind(block_hash.as_slice())
            .fetch_optional(pool)
            .await
            .expect("load predecessor publication state");
            if let Some(row) = row {
                if row.get::<bool, _>("manifest_published") {
                    let digest: Vec<u8> = row
                        .get::<Option<Vec<u8>>, _>("manifest_digest")
                        .expect("published predecessor has a manifest digest");
                    let publisher: Vec<u8> = row.get("manifest_publisher");
                    return ManifestReference {
                        generation: block_manifest::LEGACY_CONSENSUS_EPOCH.to_owned(),
                        publisher: Address::from_slice(&publisher),
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

async fn delete_last_published_manifest(pool: &PgPool, previous: &ManifestReference) -> B256 {
    let revision = i64::try_from(previous.revision).expect("manifest revision fits BIGINT");
    let row = sqlx::query(
        "DELETE FROM block_manifest
          WHERE host_chain_id = $1
            AND publication_block_hash = $2
            AND revision = $3
        RETURNING manifest_digest, object_key, signed_manifest",
    )
    .bind(SEQUENCE_CHAIN_ID)
    .bind(previous.block_hash.as_slice())
    .bind(revision)
    .fetch_one(pool)
    .await
    .expect("delete previous signed manifest for fault injection");
    let manifest_digest: Vec<u8> = row.get("manifest_digest");
    let object_key: String = row.get("object_key");
    let signed_manifest: Vec<u8> = row.get("signed_manifest");
    assert_eq!(manifest_digest, previous.manifest_digest.as_slice());
    let signed: SignedManifest =
        serde_json::from_slice(&signed_manifest).expect("decode deleted signed manifest");
    signed.verify().expect("verify deleted signed manifest");
    assert_eq!(object_key, manifest_object_key(&signed));
    previous.manifest_digest
}

async fn assert_successor_was_published(pool: &PgPool, block_hash: B256, manifest_digest: B256) {
    let state = sqlx::query(
        "SELECT b.manifest_published, b.manifest_digest,
                COUNT(m.*) AS manifest_body_count
           FROM block_manifest_state b
           LEFT JOIN block_manifest m
             ON m.host_chain_id = b.host_chain_id
            AND m.publication_block_hash = b.block_hash
          WHERE b.host_chain_id = $1 AND b.block_hash = $2
          GROUP BY b.manifest_published, b.manifest_digest",
    )
    .bind(SEQUENCE_CHAIN_ID)
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

async fn assert_persisted_canonical_state(
    pool: &PgPool,
    cadence: i64,
    expected_blocks: &[ExpectedBlock],
    expected_manifests: &[SignedManifest],
    deleted_manifest_digests: &BTreeSet<B256>,
) {
    let unsealed: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM block_manifest_state
          WHERE host_chain_id = $1
            AND (block_content_digest IS NULL OR block_handle_count IS NULL)",
    )
    .bind(SEQUENCE_CHAIN_ID)
    .fetch_one(pool)
    .await
    .expect("count unsealed consensus blocks");
    assert_eq!(unsealed, 0);

    let incorrectly_published: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM block_manifest_state
          WHERE host_chain_id = $1
            AND manifest_published
            AND MOD(block_number, $2) <> 0",
    )
    .bind(SEQUENCE_CHAIN_ID)
    .bind(cadence)
    .fetch_one(pool)
    .await
    .expect("count off-cadence manifests");
    assert_eq!(incorrectly_published, 0);

    let rows = sqlx::query(
        "SELECT publication_block_number, manifest_digest, object_key, signed_manifest
           FROM block_manifest
          WHERE host_chain_id = $1
          ORDER BY publication_block_number",
    )
    .bind(SEQUENCE_CHAIN_ID)
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
        let stored_body: Vec<u8> = row.get("signed_manifest");
        let decoded: SignedManifest =
            serde_json::from_slice(&stored_body).expect("decode stored signed manifest");
        assert_eq!(decoded, *expected);
        assert_eq!(
            row.get::<Vec<u8>, _>("manifest_digest"),
            expected.digest().unwrap().as_slice()
        );
        assert_eq!(
            row.get::<String, _>("object_key"),
            manifest_object_key(expected)
        );
    }

    assert_persisted_ranges(pool, expected_blocks).await;
}

pub(super) async fn assert_persisted_ranges(pool: &PgPool, expected_blocks: &[ExpectedBlock]) {
    let by_hash = index_blocks(expected_blocks);
    let mut expected = BTreeSet::new();
    for block in expected_blocks {
        let lineage = lineage_blocks(block, None, &by_hash);
        for range in committed_ranges(&lineage) {
            expected.insert((
                range.start_block_number,
                range.end,
                range.start_block_hash,
                range.end_block_hash,
                range.digest,
            ));
        }
    }

    let range_rows = sqlx::query(
        "SELECT range_start, range_end,
                range_start_block_hash, range_end_block_hash, range_digest
           FROM block_range_commitment
          WHERE host_chain_id = $1",
    )
    .bind(SEQUENCE_CHAIN_ID)
    .fetch_all(pool)
    .await
    .expect("load persisted canonical ranges");
    assert!(
        !range_rows.is_empty(),
        "the sequence must persist range roots"
    );
    let actual = range_rows
        .into_iter()
        .map(|row| {
            (
                row.get::<i64, _>("range_start"),
                row.get::<i64, _>("range_end"),
                B256::from_slice(&row.get::<Vec<u8>, _>("range_start_block_hash")),
                B256::from_slice(&row.get::<Vec<u8>, _>("range_end_block_hash")),
                B256::from_slice(&row.get::<Vec<u8>, _>("range_digest")),
            )
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
}

pub(super) async fn insert_simulated_history_block(
    pool: &PgPool,
    expected_blocks: &mut Vec<ExpectedBlock>,
    block_number: i64,
    parent_block_hash: B256,
    history_variant: u32,
) -> B256 {
    let block_hash = sequence_value(*b"seqblock", block_number, history_variant);
    let descriptors = seed_block_descriptors(pool, block_number, block_hash, history_variant).await;
    let content_digest = content_digest(block_number, block_hash, &descriptors);
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

pub(super) async fn insert_consensus_block(
    pool: &PgPool,
    block_number: i64,
    block_hash: B256,
    parent_block_hash: B256,
) {
    sqlx::query(
        "INSERT INTO block_manifest_state (
             host_chain_id, block_number, block_hash, parent_block_hash,
             publication_cadence
         ) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(SEQUENCE_CHAIN_ID)
    .bind(block_number)
    .bind(block_hash.as_slice())
    .bind(parent_block_hash.as_slice())
    .bind(SEQUENCE_CADENCE)
    .execute(pool)
    .await
    .expect("insert synthetic consensus block");
}

async fn seed_block_descriptors(
    pool: &PgPool,
    block_number: i64,
    block_hash: B256,
    history_variant: u32,
) -> Vec<CiphertextDescriptor> {
    let block_handle_count = if block_number.rem_euclid(5) == 0 {
        0
    } else {
        usize::try_from(block_number.rem_euclid(3) + 1).unwrap()
    };
    let mut descriptors = Vec::with_capacity(block_handle_count);
    for ordinal in 0..block_handle_count {
        let ordinal = history_variant
            .checked_shl(16)
            .and_then(|prefix| prefix.checked_add(u32::try_from(ordinal).unwrap()))
            .expect("history descriptor ordinal fits u32");
        let handle = sequence_value(*b"seq_hand", block_number, ordinal);
        let gateway_key_id = sequence_value(*b"seq_key_", block_number, ordinal);
        let ct64_digest = sequence_value(*b"seq_ct64", block_number, ordinal);
        let ct128_digest = sequence_value(*b"seq_128_", block_number, ordinal);

        sqlx::query(
            "INSERT INTO handle_producer_block (
                 host_chain_id, producer_block_number, producer_block_hash, handle
             ) VALUES ($1, $2, $3, $4)",
        )
        .bind(SEQUENCE_CHAIN_ID)
        .bind(block_number)
        .bind(block_hash.as_slice())
        .bind(handle.as_slice())
        .execute(pool)
        .await
        .expect("insert manifest producer block");

        sqlx::query(
            "INSERT INTO keys (
                 key_id_gw, key_id, pks_key, sks_key, chain_id, block_hash
             ) VALUES ($1, $1, ''::BYTEA, ''::BYTEA, $2, $3)
             ON CONFLICT (chain_id, block_hash, key_id_gw) DO NOTHING",
        )
        .bind(gateway_key_id.as_slice())
        .bind(SEQUENCE_CHAIN_ID)
        .bind(block_hash.as_slice())
        .execute(pool)
        .await
        .expect("insert manifest keyset identity");

        sqlx::query(
            "INSERT INTO ciphertext_digest (
                 host_chain_id, key_id_gw, handle, ciphertext, ciphertext128,
                 ciphertext128_format
             ) VALUES ($1, $2, $3, $4, $5, 11)",
        )
        .bind(SEQUENCE_CHAIN_ID)
        .bind(gateway_key_id.as_slice())
        .bind(handle.as_slice())
        .bind(ct64_digest.as_slice())
        .bind(ct128_digest.as_slice())
        .execute(pool)
        .await
        .expect("insert complete manifest digest row");

        descriptors.push(CiphertextDescriptor::computed(
            handle,
            U256::from_be_slice(gateway_key_id.as_slice()),
            Some(U256::from_be_slice(gateway_key_id.as_slice())),
            ct64_digest,
            ct128_digest,
            CiphertextFormat::CompressedOnCpu,
        ));
    }
    descriptors.sort_by(|left, right| left.handle.as_slice().cmp(right.handle.as_slice()));
    descriptors
}

fn expected_block(blocks: &[ExpectedBlock], number: i64) -> &ExpectedBlock {
    let block = &blocks[usize::try_from(number).expect("block number is non-negative")];
    assert_eq!(block.number, number);
    block
}

pub(super) fn content_digest(
    block_number: i64,
    block_hash: B256,
    descriptors: &[CiphertextDescriptor],
) -> B256 {
    block_content_digest(
        ManifestVersion::V1,
        SEQUENCE_CONTEXT_ID,
        U256::from(SEQUENCE_CHAIN_ID as u64),
        U256::from(block_number as u64),
        block_hash,
        descriptors,
    )
    .expect("compute expected block digest")
}

pub(super) fn sequence_value(tag: [u8; 8], block_number: i64, ordinal: u32) -> B256 {
    let mut bytes = [0_u8; 32];
    bytes[..8].copy_from_slice(&tag);
    bytes[8..16].copy_from_slice(&block_number.to_be_bytes());
    bytes[28..].copy_from_slice(&ordinal.to_be_bytes());
    B256::from(bytes)
}

pub(super) fn positive_env_u64(name: &str, default: u64) -> u64 {
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
