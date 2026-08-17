use super::*;
use crate::manifest_consensus::manifest_archive::{
    manifest_object_key, store_authenticated_manifest, ManifestSource,
};
use alloy::signers::local::PrivateKeySigner;
use alloy_primitives::B256;
use aws_sdk_s3::primitives::ByteStream;
use block_manifest::{
    block_content_digest, canonical_history_scale, detailed_range_digest, dyadic_range_digest,
    BlockCiphertextDescriptor, CiphertextFormat, DetailedRange, HistoricalRange,
    ManifestBlockEntry, ManifestPayload, SignedManifest,
};
use serial_test::serial;
use sqlx::Row;
use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};
use tokio::sync::Barrier;

const TEST_CONTEXT_ID: U256 = U256::ONE;
const TEST_CHAIN_ID: i64 = 7;
const TEST_BLOCK_NUMBER: i64 = 42;
const TEST_WORKERS: usize = 8;
const GENERATION: &str = block_manifest::LEGACY_CONSENSUS_EPOCH;

#[derive(Default)]
struct FakePeerSource {
    objects: Mutex<HashMap<Address, Vec<PeerManifestObject>>>,
    list_calls: Mutex<HashMap<Address, usize>>,
    body_downloads: Mutex<HashMap<Address, usize>>,
    missing_object_keys: Mutex<HashMap<Address, HashSet<String>>>,
    transient_object_keys: Mutex<HashMap<Address, HashSet<String>>>,
}

impl FakePeerSource {
    fn set_manifest(&self, publisher: Address, manifest: &SignedManifest) {
        self.set_manifests(publisher, std::slice::from_ref(manifest));
    }

    fn set_manifests(&self, publisher: Address, manifests: &[SignedManifest]) {
        self.objects.lock().expect("lock fake objects").insert(
            publisher,
            manifests
                .iter()
                .map(|manifest| PeerManifestObject {
                    object_key: manifest_object_key(manifest),
                    signed_bytes: serde_json::to_vec(manifest)
                        .expect("serialize fake peer manifest"),
                })
                .collect(),
        );
    }

    fn corrupt_manifest_body(&self, publisher: Address, manifest: &SignedManifest) {
        let object_key = manifest_object_key(manifest);
        let mut objects = self.objects.lock().expect("lock fake objects");
        let object = objects
            .get_mut(&publisher)
            .and_then(|objects| {
                objects
                    .iter_mut()
                    .find(|object| object.object_key == object_key)
            })
            .expect("fake manifest to corrupt");
        object.signed_bytes = b"not a signed manifest".to_vec();
    }

    fn list_calls(&self, publisher: Address) -> usize {
        self.list_calls
            .lock()
            .expect("lock fake list calls")
            .get(&publisher)
            .copied()
            .unwrap_or_default()
    }

    fn body_downloads(&self, publisher: Address) -> usize {
        self.body_downloads
            .lock()
            .expect("lock fake body downloads")
            .get(&publisher)
            .copied()
            .unwrap_or_default()
    }

    fn list_missing_object(&self, publisher: Address, object_key: String) {
        self.missing_object_keys
            .lock()
            .expect("lock fake missing keys")
            .entry(publisher)
            .or_default()
            .insert(object_key);
    }

    fn fail_object_transiently(&self, publisher: Address, object_key: String) {
        self.transient_object_keys
            .lock()
            .expect("lock fake transient keys")
            .entry(publisher)
            .or_default()
            .insert(object_key);
    }
}

impl PeerManifestSource for FakePeerSource {
    async fn list_manifests(
        &self,
        request: &PeerDownloadRequest,
    ) -> Result<Vec<String>, ExecutionError> {
        *self
            .list_calls
            .lock()
            .expect("lock fake list calls")
            .entry(request.publisher)
            .or_default() += 1;
        let expected_prefix = block_manifest::manifest_object_prefix(
            request.version,
            request.coprocessor_context_id,
            U256::from(request.host_chain_id as u64),
            U256::from(request.publication_block_number as u64),
            request.publication_block_hash,
            &request.generation,
        );
        let listed_revision = |object_key: &str| {
            object_key.starts_with(&expected_prefix)
                && object_key
                    .rsplit('/')
                    .next()
                    .and_then(|revision| revision.parse::<u64>().ok())
                    .is_some_and(|revision| request.considers_object(object_key, revision))
        };
        let mut object_keys = self
            .objects
            .lock()
            .expect("lock fake objects")
            .get(&request.publisher)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter(|object| listed_revision(&object.object_key))
            .map(|object| object.object_key)
            .collect::<Vec<_>>();
        for extra_keys in [&self.missing_object_keys, &self.transient_object_keys] {
            let extras = extra_keys
                .lock()
                .expect("lock fake extra keys")
                .get(&request.publisher)
                .cloned()
                .unwrap_or_default();
            for object_key in extras {
                if listed_revision(&object_key) && !object_keys.contains(&object_key) {
                    object_keys.push(object_key);
                }
            }
        }
        object_keys.sort_unstable_by_key(|key| {
            std::cmp::Reverse(
                key.rsplit('/')
                    .next()
                    .and_then(|revision| revision.parse::<u64>().ok())
                    .unwrap_or_default(),
            )
        });
        Ok(object_keys)
    }

    async fn fetch_manifest(
        &self,
        request: &PeerDownloadRequest,
        object_key: &str,
    ) -> Result<PeerManifestObject, ExecutionError> {
        self.body_downloads
            .lock()
            .expect("lock fake body downloads")
            .entry(request.publisher)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        if self
            .transient_object_keys
            .lock()
            .expect("lock fake transient keys")
            .get(&request.publisher)
            .is_some_and(|keys| keys.contains(object_key))
        {
            return Err(ExecutionError::S3TransientError(format!(
                "fake transient fetch for {object_key}"
            )));
        }
        if self
            .missing_object_keys
            .lock()
            .expect("lock fake missing keys")
            .get(&request.publisher)
            .is_some_and(|keys| keys.contains(object_key))
        {
            return Err(ExecutionError::S3ObjectNotFound(format!(
                "fake missing peer object {object_key}"
            )));
        }
        self.objects
            .lock()
            .expect("lock fake objects")
            .get(&request.publisher)
            .and_then(|objects| {
                objects
                    .iter()
                    .find(|object| object.object_key == object_key)
                    .cloned()
            })
            .ok_or_else(|| internal(format!("missing fake peer object {object_key}")))
    }
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn concurrent_downloaders_claim_once_and_reach_manifest_quorum() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers, 2).await;
    let local = sign_payload(&signers[0], payload(signers[0].address(), 1)).await;
    schedule_local(&pool, &local, 1).await;

    let source = Arc::new(FakePeerSource::default());
    let agreeing_zero = sign_payload(&signers[1], payload(signers[1].address(), 1)).await;
    let agreeing_one =
        sign_payload(&signers[1], revision_payload(signers[1].address(), 1, 1)).await;
    let agreeing_two =
        sign_payload(&signers[1], revision_payload(signers[1].address(), 1, 2)).await;
    source.set_manifests(
        signers[1].address(),
        &[agreeing_zero, agreeing_one, agreeing_two],
    );

    let outcomes = concurrent_wave(&pool, &source).await;
    assert_eq!(completed_runs(&outcomes), 1, "{outcomes:?}");
    assert!(outcomes.iter().all(Result::is_ok), "{outcomes:?}");
    let result = outcomes
        .iter()
        .find_map(|outcome| outcome.as_ref().ok().copied().flatten())
        .expect("one worker completes the target");
    assert_eq!(result.outcome, VerificationOutcome::Consensus);
    assert_eq!(source.body_downloads(signers[1].address()), 1);
    assert_eq!(source.body_downloads(signers[2].address()), 0);
    assert_target(&pool, "consensus", "consensus", 1).await;
    assert_eq!(archive_count(&pool).await, 2);
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn concurrent_downloaders_respect_the_durable_initial_delay() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers, 2).await;
    let local = sign_payload(&signers[0], payload(signers[0].address(), 1)).await;
    schedule_local_with_delay(&pool, &local, Duration::from_secs(30), 0).await;

    let source = Arc::new(FakePeerSource::default());
    let agreeing = sign_payload(&signers[1], payload(signers[1].address(), 1)).await;
    source.set_manifest(signers[1].address(), &agreeing);

    let early_wave = concurrent_wave(&pool, &source).await;
    assert!(early_wave.iter().all(|outcome| matches!(outcome, Ok(None))));
    assert_eq!(source.list_calls(signers[1].address()), 0);
    assert_target(&pool, "pending", "unknown", 0).await;

    sqlx::query(
        "UPDATE block_manifest_verification_task SET next_attempt_at = NOW() - INTERVAL '1 second'",
    )
    .execute(&pool)
    .await
    .expect("advance the simulated verification clock");
    let due_wave = concurrent_wave(&pool, &source).await;
    assert_eq!(completed_runs(&due_wave), 1, "{due_wave:?}");
    assert!(due_wave.iter().all(Result::is_ok), "{due_wave:?}");
    assert_eq!(source.body_downloads(signers[1].address()), 1);
    assert_target(&pool, "consensus", "consensus", 1).await;
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn expired_claim_recovers_after_mid_download_crash_without_redownloading() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers, 3).await;
    let local = sign_payload(&signers[0], payload(signers[0].address(), 1)).await;
    schedule_local(&pool, &local, 1).await;

    let source = Arc::new(FakePeerSource::default());
    for signer in &signers[1..] {
        let manifest = sign_payload(signer, payload(signer.address(), 1)).await;
        source.set_manifest(signer.address(), &manifest);
    }

    let claim = claim_due_task(&pool, "crashed-worker", Duration::from_secs(60), GENERATION)
        .await
        .expect("claim target before simulated crash")
        .expect("target is due");
    let first_peer = claim
        .peers
        .iter()
        .find(|peer| peer.publisher == signers[1].address())
        .expect("first peer belongs to claim")
        .clone();
    download_claimed_peer(&pool, source.as_ref(), &claim, &first_peer)
        .await
        .expect("durably download first peer");
    assert_eq!(source.body_downloads(signers[1].address()), 1);

    sqlx::query(
        "UPDATE block_manifest_verification_task SET claim_expires_at = NOW() - INTERVAL '1 second' WHERE id = $1",
    )
    .bind(claim.task_id)
    .execute(&pool)
    .await
    .expect("simulate process death and claim expiry");

    let outcomes = concurrent_wave(&pool, &source).await;
    assert_eq!(completed_runs(&outcomes), 1, "{outcomes:?}");
    assert!(outcomes.iter().all(Result::is_ok), "{outcomes:?}");
    assert_eq!(
        source.body_downloads(signers[1].address()),
        1,
        "the recovered attempt must reuse the first peer body stored before the crash",
    );
    assert_eq!(source.list_calls(signers[1].address()), 1);
    assert_eq!(source.body_downloads(signers[2].address()), 1);
    assert_target(&pool, "consensus", "consensus", 1).await;
    assert_eq!(archive_count(&pool).await, 3);
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn concurrent_retry_rejects_invalid_peer_then_uses_new_valid_revision() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers, 2).await;
    let local = sign_payload(&signers[0], payload(signers[0].address(), 1)).await;
    schedule_local(&pool, &local, 1).await;

    let source = Arc::new(FakePeerSource::default());
    let rogue = PrivateKeySigner::random();
    let invalid = sign_payload(&rogue, payload(rogue.address(), 1)).await;
    source.set_manifest(signers[1].address(), &invalid);

    let first_wave = concurrent_wave(&pool, &source).await;
    assert_eq!(completed_runs(&first_wave), 1, "{first_wave:?}");
    assert!(first_wave.iter().all(Result::is_ok), "{first_wave:?}");
    assert_target(&pool, "pending", "unknown_but_equal", 1).await;
    assert_peer_failure(&pool, "corrupted:").await;
    assert_eq!(archive_count(&pool).await, 1);

    let valid = sign_payload(&signers[1], revision_payload(signers[1].address(), 1, 1)).await;
    source.set_manifest(signers[1].address(), &valid);
    sqlx::query(
        "UPDATE block_manifest_verification_task SET next_attempt_at = NOW() - INTERVAL '1 second'",
    )
    .execute(&pool)
    .await
    .expect("make bounded retry immediately due");

    let second_wave = concurrent_wave(&pool, &source).await;
    assert_eq!(completed_runs(&second_wave), 1, "{second_wave:?}");
    assert!(second_wave.iter().all(Result::is_ok), "{second_wave:?}");
    let result = second_wave
        .iter()
        .find_map(|outcome| outcome.as_ref().ok().copied().flatten())
        .expect("one retry worker completes the target");
    assert_eq!(result.outcome, VerificationOutcome::Consensus);
    assert_target(&pool, "consensus", "consensus", 2).await;
    assert_eq!(archive_count(&pool).await, 2);
    assert_eq!(source.body_downloads(signers[1].address()), 2);
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn downloader_uses_highest_verified_revision() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers[..2], 2).await;
    let local = sign_payload(&signers[0], payload(signers[0].address(), 1)).await;
    schedule_local(&pool, &local, 1).await;

    let source = Arc::new(FakePeerSource::default());
    let rogue = PrivateKeySigner::random();
    let valid_nine = sign_payload(&signers[1], revision_payload(signers[1].address(), 1, 9)).await;
    let invalid_ten = sign_payload(&rogue, revision_payload(rogue.address(), 1, 10)).await;
    source.set_manifests(signers[1].address(), &[valid_nine, invalid_ten]);

    let outcomes = concurrent_wave(&pool, &source).await;
    assert_eq!(completed_runs(&outcomes), 1, "{outcomes:?}");
    assert!(outcomes.iter().all(Result::is_ok), "{outcomes:?}");
    let result = outcomes
        .iter()
        .find_map(|outcome| outcome.as_ref().ok().copied().flatten())
        .expect("one worker completes the target");
    assert_eq!(result.outcome, VerificationOutcome::Consensus);
    assert_eq!(
        source.body_downloads(signers[1].address()),
        2,
        "numeric revision 10 must be rejected before revision 9 is accepted",
    );
    assert_eq!(archive_count(&pool).await, 2);
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn missing_highest_revision_falls_back_to_the_previous_key() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers[..2], 2).await;
    let local = sign_payload(&signers[0], payload(signers[0].address(), 1)).await;
    schedule_local(&pool, &local, 1).await;

    let source = Arc::new(FakePeerSource::default());
    let valid_nine = sign_payload(&signers[1], revision_payload(signers[1].address(), 1, 9)).await;
    let missing_ten =
        sign_payload(&signers[1], revision_payload(signers[1].address(), 1, 10)).await;
    source.set_manifest(signers[1].address(), &valid_nine);
    source.list_missing_object(signers[1].address(), manifest_object_key(&missing_ten));

    let outcomes = concurrent_wave(&pool, &source).await;
    assert_eq!(completed_runs(&outcomes), 1, "{outcomes:?}");
    assert!(outcomes.iter().all(Result::is_ok), "{outcomes:?}");
    let result = outcomes
        .iter()
        .find_map(|outcome| outcome.as_ref().ok().copied().flatten())
        .expect("one worker completes the target");
    assert_eq!(result.outcome, VerificationOutcome::Consensus);
    assert_eq!(
        source.body_downloads(signers[1].address()),
        2,
        "NoSuchKey on revision 10 must continue to revision 9",
    );
    assert_eq!(archive_count(&pool).await, 2);
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn transient_highest_revision_does_not_fall_back() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers[..2], 2).await;
    let local = sign_payload(&signers[0], payload(signers[0].address(), 1)).await;
    schedule_local(&pool, &local, 1).await;

    let source = Arc::new(FakePeerSource::default());
    let valid_nine = sign_payload(&signers[1], revision_payload(signers[1].address(), 1, 9)).await;
    let transient_ten =
        sign_payload(&signers[1], revision_payload(signers[1].address(), 1, 10)).await;
    source.set_manifest(signers[1].address(), &valid_nine);
    source.fail_object_transiently(signers[1].address(), manifest_object_key(&transient_ten));

    let outcomes = concurrent_wave(&pool, &source).await;
    assert_eq!(completed_runs(&outcomes), 1, "{outcomes:?}");
    assert!(outcomes.iter().all(Result::is_ok), "{outcomes:?}");
    assert_peer_failure(&pool, "incomplete:").await;
    assert_eq!(
        source.body_downloads(signers[1].address()),
        1,
        "a transient get of revision 10 must retry later instead of using revision 9",
    );
    assert_eq!(archive_count(&pool).await, 1);
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn archived_revision_does_not_fetch_older_listed_keys() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers[..2], 2).await;
    let local = sign_payload(&signers[0], payload(signers[0].address(), 1)).await;
    schedule_local(&pool, &local, 1).await;

    let source = Arc::new(FakePeerSource::default());
    let archived_nine =
        sign_payload(&signers[1], revision_payload(signers[1].address(), 1, 9)).await;
    let older_eight = sign_payload(&signers[1], revision_payload(signers[1].address(), 1, 8)).await;
    let missing_ten =
        sign_payload(&signers[1], revision_payload(signers[1].address(), 1, 10)).await;
    archive_only(&pool, &archived_nine).await;
    source.set_manifests(signers[1].address(), &[archived_nine, older_eight]);
    source.list_missing_object(signers[1].address(), manifest_object_key(&missing_ten));

    let outcomes = concurrent_wave(&pool, &source).await;
    assert_eq!(completed_runs(&outcomes), 1, "{outcomes:?}");
    assert!(outcomes.iter().all(Result::is_ok), "{outcomes:?}");
    let result = outcomes
        .iter()
        .find_map(|outcome| outcome.as_ref().ok().copied().flatten())
        .expect("one worker completes using the archived revision");
    assert_eq!(result.outcome, VerificationOutcome::Consensus);
    assert_eq!(
        source.body_downloads(signers[1].address()),
        1,
        "only the strictly newer missing head is fetched; older revision 8 is ignored",
    );
    assert_eq!(archive_count(&pool).await, 2);
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
/// Attacker fixture: junk keys in a peer-controlled prefix, not an honest
/// coprocessor publishing six revisions.
async fn downloader_checks_at_most_five_revision_candidates() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers[..2], 2).await;
    let local = sign_payload(&signers[0], payload(signers[0].address(), 2)).await;
    schedule_local(&pool, &local, 1).await;

    let source = Arc::new(FakePeerSource::default());
    let rogue = PrivateKeySigner::random();
    let invalid_revisions = (1..=6)
        .map(|revision| revision_payload(rogue.address(), 2, revision))
        .collect::<Vec<_>>();
    let mut invalid_manifests = Vec::with_capacity(invalid_revisions.len() + 1);
    invalid_manifests
        .push(sign_payload(&signers[1], revision_payload(signers[1].address(), 2, 0)).await);
    for payload in invalid_revisions {
        invalid_manifests.push(sign_payload(&rogue, payload).await);
    }
    source.set_manifests(signers[1].address(), &invalid_manifests);

    let first_wave = concurrent_wave(&pool, &source).await;
    assert_eq!(completed_runs(&first_wave), 1, "{first_wave:?}");
    assert!(first_wave.iter().all(Result::is_ok), "{first_wave:?}");
    assert_eq!(
        source.body_downloads(signers[1].address()),
        MAX_REVISION_CANDIDATES_PER_ATTEMPT,
    );
    assert_peer_failure(&pool, "corrupted:").await;
    assert_eq!(archive_count(&pool).await, 1);

    sqlx::query(
        "UPDATE block_manifest_verification_task SET next_attempt_at = NOW() - INTERVAL '1 second'",
    )
    .execute(&pool)
    .await
    .expect("make bounded retry immediately due");

    let second_wave = concurrent_wave(&pool, &source).await;
    assert_eq!(completed_runs(&second_wave), 1, "{second_wave:?}");
    assert!(second_wave.iter().all(Result::is_ok), "{second_wave:?}");
    let result = second_wave
        .iter()
        .find_map(|outcome| outcome.as_ref().ok().copied().flatten())
        .expect("retry walks below rejected junk to the valid revision");
    assert_eq!(result.outcome, VerificationOutcome::Consensus);
    assert_eq!(
        source.body_downloads(signers[1].address()),
        MAX_REVISION_CANDIDATES_PER_ATTEMPT + 2,
        "second attempt skips the five rejected heads and fetches remaining 1 then 0",
    );
    assert_eq!(archive_count(&pool).await, 2);
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn corrupt_predecessor_completes_the_attempt_as_a_peer_failure() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers[..2], 2).await;

    let current_block_number = TEST_BLOCK_NUMBER + 1;
    let current_block_hash = B256::repeat_byte(0xaa);
    let local_predecessor = sign_payload(
        &signers[0],
        payload_at(
            signers[0].address(),
            2,
            TEST_BLOCK_NUMBER,
            B256::repeat_byte(0xa9),
        ),
    )
    .await;
    archive_local_only(&pool, &local_predecessor).await;
    let local = sign_payload(
        &signers[0],
        payload_with_history(
            signers[0].address(),
            1,
            current_block_number,
            current_block_hash,
            &local_predecessor,
        ),
    )
    .await;
    schedule_local(&pool, &local, 0).await;

    let predecessor = sign_payload(
        &signers[1],
        payload_at(
            signers[1].address(),
            1,
            TEST_BLOCK_NUMBER,
            B256::repeat_byte(0xa9),
        ),
    )
    .await;
    let current = payload_with_history(
        signers[1].address(),
        1,
        current_block_number,
        current_block_hash,
        &predecessor,
    );
    let current = sign_payload(&signers[1], current).await;

    let source = Arc::new(FakePeerSource::default());
    source.set_manifests(signers[1].address(), &[current, predecessor.clone()]);
    source.corrupt_manifest_body(signers[1].address(), &predecessor);

    let outcomes = concurrent_wave(&pool, &source).await;
    assert_eq!(completed_runs(&outcomes), 1, "{outcomes:?}");
    assert!(outcomes.iter().all(Result::is_ok), "{outcomes:?}");
    assert_peer_failure(&pool, "corrupted:").await;
    assert_attempt_completed(&pool).await;
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn rejected_predecessor_does_not_hide_current_revision_on_retry() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers[..2], 2).await;
    let local_chain = predecessor_chain(&signers[0], 40, 2, 0x31).await;
    let peer_chain = predecessor_chain(&signers[1], 40, 2, 0x31).await;
    archive_local_only(&pool, &local_chain[0]).await;
    schedule_local(&pool, &local_chain[1], 1).await;

    let source = Arc::new(FakePeerSource::default());
    source.set_manifest(signers[1].address(), &peer_chain[0]);
    source.corrupt_manifest_body(signers[1].address(), &peer_chain[0]);
    let first = concurrent_wave(&pool, &source).await;
    assert!(first.iter().all(Result::is_ok), "{first:?}");
    assert_eq!(completed_runs(&first), 1);
    let rejected = sqlx::query_scalar::<_, Vec<String>>(
        "SELECT rejected_object_keys FROM block_manifest_peer_download",
    )
    .fetch_one(&pool)
    .await
    .expect("load rejected predecessor");
    assert_eq!(rejected, vec![manifest_object_key(&peer_chain[0])]);
    assert_eq!(source.body_downloads(signers[1].address()), 1);

    // The current object arrives with the same revision as the rejected predecessor.
    assert_eq!(
        peer_chain[0].payload.revision,
        peer_chain[1].payload.revision
    );
    source.set_manifests(signers[1].address(), &peer_chain);
    sqlx::query(
        "UPDATE block_manifest_verification_task
            SET next_attempt_at = NOW() - INTERVAL '1 second'
          WHERE state = 'pending'",
    )
    .execute(&pool)
    .await
    .expect("make retry due");
    let second = concurrent_wave(&pool, &source).await;
    assert!(second.iter().all(Result::is_ok), "{second:?}");
    assert_eq!(completed_runs(&second), 1);
    assert_target(&pool, "consensus", "consensus", 2).await;
    assert_eq!(source.body_downloads(signers[1].address()), 2);
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn concurrent_retry_downloads_new_peer_revision_and_resolves_drift() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers[..2], 2).await;
    let local = sign_payload(&signers[0], payload(signers[0].address(), 1)).await;
    schedule_local(&pool, &local, 1).await;

    let source = Arc::new(FakePeerSource::default());
    let drifting_zero = sign_payload(&signers[1], payload(signers[1].address(), 9)).await;
    source.set_manifest(signers[1].address(), &drifting_zero);

    let first_wave = concurrent_wave(&pool, &source).await;
    assert_eq!(completed_runs(&first_wave), 1, "{first_wave:?}");
    assert!(first_wave.iter().all(Result::is_ok), "{first_wave:?}");
    assert_target(&pool, "pending", "drift", 1).await;
    assert_eq!(source.body_downloads(signers[1].address()), 1);
    assert_eq!(archive_count(&pool).await, 2);
    let unresolved = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM drifted_handle WHERE status = 'unresolved'",
    )
    .fetch_one(&pool)
    .await
    .expect("count unresolved peer-revision drift findings");
    assert!(unresolved > 0);

    let repaired_one =
        sign_payload(&signers[1], revision_payload(signers[1].address(), 1, 1)).await;
    source.set_manifests(signers[1].address(), &[drifting_zero, repaired_one]);
    sqlx::query(
        "UPDATE block_manifest_verification_task
            SET next_attempt_at = NOW() - INTERVAL '1 second'
          WHERE state = 'pending'",
    )
    .execute(&pool)
    .await
    .expect("make peer-revision retry immediately due");

    let second_wave = concurrent_wave(&pool, &source).await;
    assert_eq!(completed_runs(&second_wave), 1, "{second_wave:?}");
    assert!(second_wave.iter().all(Result::is_ok), "{second_wave:?}");
    assert_target(&pool, "consensus", "consensus", 2).await;
    assert_eq!(source.body_downloads(signers[1].address()), 2);
    assert_eq!(archive_count(&pool).await, 3);
    let states = sqlx::query(
        "SELECT COUNT(*) FILTER (WHERE status = 'resolved') AS resolved,
                COUNT(*) FILTER (WHERE status = 'unresolved') AS unresolved
           FROM drifted_handle",
    )
    .fetch_one(&pool)
    .await
    .expect("load peer-revision remission state");
    assert_eq!(states.try_get::<i64, _>("resolved").unwrap(), unresolved);
    assert_eq!(states.try_get::<i64, _>("unresolved").unwrap(), 0);
    let evidence = sqlx::query(
        "SELECT
             (SELECT COUNT(*) FROM block_manifest_verification_attempt) AS attempts,
             (SELECT COUNT(*) FROM block_manifest_verification_attempt_drift) AS drifts,
             (SELECT COUNT(*) FROM block_manifest_verification_attempt
               WHERE outcome = 'drift' AND localization_complete) AS localized_drift",
    )
    .fetch_one(&pool)
    .await
    .expect("load durable verification evidence");
    assert_eq!(evidence.try_get::<i64, _>("attempts").unwrap(), 2);
    assert!(evidence.try_get::<i64, _>("drifts").unwrap() > 0);
    assert_eq!(evidence.try_get::<i64, _>("localized_drift").unwrap(), 1);
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn concurrent_missing_peer_attempt_exhausts_the_bounded_budget_once() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers, 2).await;
    let local = sign_payload(&signers[0], payload(signers[0].address(), 1)).await;
    schedule_local(&pool, &local, 0).await;
    let source = Arc::new(FakePeerSource::default());

    let first_wave = concurrent_wave(&pool, &source).await;
    assert_eq!(completed_runs(&first_wave), 1, "{first_wave:?}");
    assert!(first_wave.iter().all(Result::is_ok), "{first_wave:?}");
    assert_target(&pool, "retry_exhausted", "unknown_but_equal", 1).await;

    let later_wave = concurrent_wave(&pool, &source).await;
    assert!(later_wave.iter().all(|outcome| matches!(outcome, Ok(None))));
    assert_target(&pool, "retry_exhausted", "unknown_but_equal", 1).await;
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn concurrent_workers_cover_five_copro_drift_populations_from_every_origin() {
    struct Scenario {
        name: &'static str,
        block_number: i64,
        block_hash: B256,
        digests: [u8; 5],
        expected_outcomes: [VerificationOutcome; 5],
    }

    let scenarios = [
        Scenario {
            name: "one drifter",
            block_number: 101,
            block_hash: B256::repeat_byte(0xa1),
            digests: [0x11, 0x11, 0x11, 0x11, 0x21],
            expected_outcomes: [VerificationOutcome::Drift; 5],
        },
        Scenario {
            name: "two matching drifters",
            block_number: 102,
            block_hash: B256::repeat_byte(0xa2),
            digests: [0x12, 0x12, 0x12, 0x22, 0x22],
            expected_outcomes: [VerificationOutcome::Drift; 5],
        },
        Scenario {
            name: "three drifters split as pair and singleton",
            block_number: 103,
            block_hash: B256::repeat_byte(0xa3),
            digests: [0x13, 0x13, 0x23, 0x23, 0x33],
            expected_outcomes: [VerificationOutcome::Drift; 5],
        },
        Scenario {
            name: "five drifters split as two pairs and singleton",
            block_number: 104,
            block_hash: B256::repeat_byte(0xa4),
            digests: [0x14, 0x14, 0x24, 0x24, 0x34],
            expected_outcomes: [VerificationOutcome::Drift; 5],
        },
        Scenario {
            name: "all five results differ",
            block_number: 105,
            block_hash: B256::repeat_byte(0xa5),
            digests: [0x15, 0x25, 0x35, 0x45, 0x55],
            expected_outcomes: [VerificationOutcome::Drift; 5],
        },
    ];

    // Each operator owns its local inventory; peer archives may still be shared.
    for local_origin in 0..5 {
        let (_instance, pool) = setup_download_db().await;
        let signers = five_test_signers();
        seed_registry(&pool, &signers, 3).await;
        for scenario in &scenarios {
            for (origin, (signer, digest)) in signers.iter().zip(scenario.digests).enumerate() {
                let manifest = sign_payload(
                    signer,
                    payload_at(
                        signer.address(),
                        digest,
                        scenario.block_number,
                        scenario.block_hash,
                    ),
                )
                .await;
                if origin == local_origin {
                    schedule_local(&pool, &manifest, 0).await;
                } else {
                    archive_only(&pool, &manifest).await;
                }
            }
        }

        let source = Arc::new(FakePeerSource::default());
        let mut completed = 0;
        loop {
            let outcomes = concurrent_wave(&pool, &source).await;
            assert!(outcomes.iter().all(Result::is_ok), "{outcomes:?}");
            let wave_completed = completed_runs(&outcomes);
            if wave_completed == 0 {
                break;
            }
            completed += wave_completed;
        }
        assert_eq!(completed, scenarios.len());

        for scenario in &scenarios {
            for (origin, (signer, expected_outcome)) in signers
                .iter()
                .zip(scenario.expected_outcomes)
                .enumerate()
                .filter(|(origin, _)| *origin == local_origin)
            {
                let row = sqlx::query(
                    r#"
                SELECT task.id,
                       state,
                       latest_outcome,
                       attempt_count
                  FROM block_manifest_verification_task task
                  JOIN block_manifest local_manifest
                    ON local_manifest.id = task.local_manifest_id
                 WHERE local_manifest.publisher = $1
                   AND local_manifest.publication_block_number = $2
                   AND local_manifest.publication_block_hash = $3
                "#,
                )
                .bind(signer.address().as_slice())
                .bind(scenario.block_number)
                .bind(scenario.block_hash.as_slice())
                .fetch_one(&pool)
                .await
                .expect("load five-copro verification result");
                let terminal_state = if expected_outcome == VerificationOutcome::Consensus {
                    "consensus"
                } else {
                    "retry_exhausted"
                };
                assert_eq!(
                    row.try_get::<String, _>("state").unwrap(),
                    terminal_state,
                    "{} from local origin {}",
                    scenario.name,
                    signer.address(),
                );
                assert_eq!(
                    row.try_get::<String, _>("latest_outcome").unwrap(),
                    expected_outcome.as_db_str(),
                    "{} from local origin {}",
                    scenario.name,
                    signer.address(),
                );
                assert_eq!(row.try_get::<i32, _>("attempt_count").unwrap(), 1);
                if expected_outcome == VerificationOutcome::Drift {
                    let local_digest = scenario.digests[origin];
                    let expected_quorum_backed_difference = scenario.digests.iter().any(|digest| {
                        *digest != local_digest
                            && scenario
                                .digests
                                .iter()
                                .filter(|candidate| **candidate == *digest)
                                .count()
                                >= 3
                    });
                    let task_id = row.try_get::<i64, _>("id").unwrap();
                    let persisted = sqlx::query(
                        r#"
                    SELECT COUNT(*) AS finding_count,
                           BOOL_OR(target_ct64_digest IS NOT NULL) AS has_quorum_backed_difference
                      FROM drifted_handle
                     WHERE last_observed_task_id = $1
                    "#,
                    )
                    .bind(task_id)
                    .fetch_one(&pool)
                    .await
                    .expect("load persisted drift explanations");
                    assert!(
                        persisted.try_get::<i64, _>("finding_count").unwrap() > 0,
                        "{} from local origin {} must persist handle differences",
                        scenario.name,
                        signer.address(),
                    );
                    assert_eq!(
                        persisted
                            .try_get::<Option<bool>, _>("has_quorum_backed_difference")
                            .unwrap()
                            .unwrap_or(false),
                        expected_quorum_backed_difference,
                        "{} from local origin {} has an unexpected actionable difference",
                        scenario.name,
                        signer.address(),
                    );
                }
            }
        }

        for signer in &signers {
            assert_eq!(
                source.body_downloads(signer.address()),
                0,
                "the all-origin DB simulation must reuse archived manifest bodies",
            );
        }
    }
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn detailed_consensus_does_not_hide_localized_historical_drift() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers, 2).await;

    let predecessor_block_number = 42;
    let predecessor_block_hash = B256::repeat_byte(0xa9);
    let current_block_number = 43;
    let current_block_hash = B256::repeat_byte(0xaa);
    let local_predecessor = sign_payload(
        &signers[0],
        payload_at(
            signers[0].address(),
            0x31,
            predecessor_block_number,
            predecessor_block_hash,
        ),
    )
    .await;
    let mut quorum_predecessors = Vec::new();
    for signer in &signers[1..] {
        quorum_predecessors.push(
            sign_payload(
                signer,
                payload_at(
                    signer.address(),
                    0x32,
                    predecessor_block_number,
                    predecessor_block_hash,
                ),
            )
            .await,
        );
    }

    archive_local_only(&pool, &local_predecessor).await;

    let local = sign_payload(
        &signers[0],
        payload_with_history(
            signers[0].address(),
            0x41,
            current_block_number,
            current_block_hash,
            &local_predecessor,
        ),
    )
    .await;
    schedule_local(&pool, &local, 0).await;

    let source = Arc::new(FakePeerSource::default());
    for (signer, predecessor) in signers[1..].iter().zip(&quorum_predecessors) {
        let current = sign_payload(
            signer,
            payload_with_history(
                signer.address(),
                0x41,
                current_block_number,
                current_block_hash,
                predecessor,
            ),
        )
        .await;
        source.set_manifests(signer.address(), &[current, predecessor.clone()]);
    }

    let outcomes = concurrent_wave(&pool, &source).await;
    assert!(outcomes.iter().all(Result::is_ok), "{outcomes:?}");
    assert_eq!(completed_runs(&outcomes), 1, "{outcomes:?}");
    let result = outcomes
        .iter()
        .find_map(|outcome| outcome.as_ref().ok().copied().flatten())
        .expect("one completed historical drift verification");
    assert_eq!(result.outcome, VerificationOutcome::Drift);

    let target = sqlx::query(
        r#"
        SELECT task.state,
               task.latest_outcome,
               attempt.local_quorum_status,
               attempt.drifted_block_count,
               attempt.drifted_handle_count
          FROM block_manifest_verification_task task
          JOIN block_manifest_verification_attempt attempt
            ON attempt.task_id = task.id
           AND attempt.attempt = task.attempt_count
        "#,
    )
    .fetch_one(&pool)
    .await
    .expect("load historical drift target");
    assert_eq!(
        target.try_get::<String, _>("state").unwrap(),
        "retry_exhausted"
    );
    assert_eq!(
        target.try_get::<String, _>("latest_outcome").unwrap(),
        "drift"
    );
    assert_eq!(
        target.try_get::<String, _>("local_quorum_status").unwrap(),
        "differs_from_quorum"
    );
    assert_eq!(
        target
            .try_get::<Option<i64>, _>("drifted_block_count")
            .unwrap(),
        Some(1)
    );
    assert_eq!(
        target
            .try_get::<Option<i64>, _>("drifted_handle_count")
            .unwrap(),
        Some(1)
    );

    let finding = sqlx::query(
        r#"
        SELECT block_number,
               block_hash,
               local_ct64_digest,
               observed_ct64_digest,
               target_ct64_digest
          FROM drifted_handle
        "#,
    )
    .fetch_one(&pool)
    .await
    .expect("load localized historical handle finding");
    assert_eq!(
        finding.try_get::<i64, _>("block_number").unwrap(),
        predecessor_block_number
    );
    assert_eq!(
        finding.try_get::<Vec<u8>, _>("block_hash").unwrap(),
        predecessor_block_hash.to_vec()
    );
    assert_eq!(
        finding.try_get::<Vec<u8>, _>("local_ct64_digest").unwrap(),
        B256::repeat_byte(0x31).to_vec()
    );
    assert_eq!(
        finding
            .try_get::<Vec<u8>, _>("observed_ct64_digest")
            .unwrap(),
        B256::repeat_byte(0x32).to_vec()
    );
    assert_eq!(
        finding.try_get::<Vec<u8>, _>("target_ct64_digest").unwrap(),
        B256::repeat_byte(0x32).to_vec()
    );
    for signer in &signers[1..] {
        assert_eq!(source.body_downloads(signer.address()), 2);
    }
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn equal_historical_ranges_do_not_download_predecessors() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers[..2], 2).await;

    let predecessor_block_number = TEST_BLOCK_NUMBER;
    let predecessor_block_hash = B256::repeat_byte(0xa9);
    let current_block_number = TEST_BLOCK_NUMBER + 1;
    let current_block_hash = B256::repeat_byte(0xaa);
    let local_predecessor = sign_payload(
        &signers[0],
        payload_at(
            signers[0].address(),
            0x31,
            predecessor_block_number,
            predecessor_block_hash,
        ),
    )
    .await;
    archive_local_only(&pool, &local_predecessor).await;
    let local = sign_payload(
        &signers[0],
        payload_with_history(
            signers[0].address(),
            0x41,
            current_block_number,
            current_block_hash,
            &local_predecessor,
        ),
    )
    .await;
    schedule_local(&pool, &local, 0).await;

    let peer_predecessor = sign_payload(
        &signers[1],
        payload_at(
            signers[1].address(),
            0x31,
            predecessor_block_number,
            predecessor_block_hash,
        ),
    )
    .await;
    let peer = sign_payload(
        &signers[1],
        payload_with_history(
            signers[1].address(),
            0x41,
            current_block_number,
            current_block_hash,
            &peer_predecessor,
        ),
    )
    .await;
    let source = Arc::new(FakePeerSource::default());
    source.set_manifest(signers[1].address(), &peer);

    let outcomes = concurrent_wave(&pool, &source).await;
    assert!(outcomes.iter().all(Result::is_ok), "{outcomes:?}");
    assert_eq!(completed_runs(&outcomes), 1, "{outcomes:?}");
    assert_eq!(
        outcomes
            .iter()
            .find_map(|outcome| outcome.as_ref().ok().copied().flatten())
            .expect("one completed equal historical verification")
            .outcome,
        VerificationOutcome::Consensus
    );
    assert_eq!(source.body_downloads(signers[1].address()), 1);
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn persistent_historical_drift_reuses_exact_localization_with_quorum() {
    const HISTORY_LENGTH: i64 = 257;

    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers, 2).await;

    let first_block_number = 0;
    let local_history =
        predecessor_chain(&signers[0], first_block_number, HISTORY_LENGTH, 0x31).await;
    let peer_history =
        predecessor_chain(&signers[1], first_block_number, HISTORY_LENGTH, 0x32).await;
    for manifest in &local_history {
        archive_local_only(&pool, manifest).await;
    }

    let current_block_number = first_block_number + HISTORY_LENGTH;
    let current_block_hash = B256::repeat_byte(0xee);
    let local = sign_payload(
        &signers[0],
        payload_with_complete_history(
            signers[0].address(),
            0x41,
            current_block_number,
            current_block_hash,
            local_history.last().expect("local history has a tip"),
            &local_history,
        ),
    )
    .await;
    schedule_local(&pool, &local, 1).await;

    let peer = sign_payload(
        &signers[1],
        payload_with_complete_history(
            signers[1].address(),
            0x41,
            current_block_number,
            current_block_hash,
            peer_history.last().expect("peer history has a tip"),
            &peer_history,
        ),
    )
    .await;
    let source = Arc::new(FakePeerSource::default());
    let mut peer_objects = peer_history;
    peer_objects.push(peer);
    source.set_manifests(signers[1].address(), &peer_objects);
    let mut quorum_objects = Vec::new();
    for object in &peer_objects {
        let mut payload = object.payload.clone();
        payload.publisher = signers[2].address();
        quorum_objects.push(sign_payload(&signers[2], payload).await);
    }
    source.set_manifests(signers[2].address(), &quorum_objects);

    let first_wave = concurrent_wave(&pool, &source).await;
    assert!(first_wave.iter().all(Result::is_ok), "{first_wave:?}");
    assert_eq!(completed_runs(&first_wave), 1, "{first_wave:?}");
    assert_eq!(
        first_wave
            .iter()
            .find_map(|outcome| outcome.as_ref().ok().copied().flatten())
            .expect("one completed historical drift verification")
            .outcome,
        VerificationOutcome::Drift
    );
    let first_scan_downloads = source.body_downloads(signers[1].address());
    assert!(
        first_scan_downloads <= 10,
        "range-directed scan downloaded {first_scan_downloads} manifests"
    );
    assert!(
        first_scan_downloads < usize::try_from(HISTORY_LENGTH).expect("positive history length"),
        "history scan must not walk every predecessor"
    );
    assert!(sqlx::query_scalar::<_, bool>(
        "SELECT localization_cacheable FROM block_manifest_verification_attempt"
    )
    .fetch_one(&pool)
    .await
    .expect("complete quorum-backed localization is reusable"));
    let localized_handles = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM drifted_handle WHERE status = 'unresolved'",
    )
    .fetch_one(&pool)
    .await
    .expect("count localized historical drift handles");
    assert!(localized_handles > 0);

    // The first attempt has fully localized this historical drift. Losing the
    // old local bodies must not make its retry re-localize the same comparison.
    sqlx::query(
        "DELETE FROM block_manifest
          WHERE publisher = $1
            AND publication_block_number < $2",
    )
    .bind(signers[0].address().as_slice())
    .bind(current_block_number)
    .execute(&pool)
    .await
    .expect("remove old local manifests after localization");
    sqlx::query(
        "UPDATE block_manifest_verification_task
            SET next_attempt_at = NOW() - INTERVAL '1 second'
          WHERE state = 'pending'",
    )
    .execute(&pool)
    .await
    .expect("make persistent drift retry due");

    let second_wave = concurrent_wave(&pool, &source).await;
    assert!(second_wave.iter().all(Result::is_ok), "{second_wave:?}");
    assert_eq!(completed_runs(&second_wave), 1, "{second_wave:?}");
    assert_target(&pool, "retry_exhausted", "drift", 2).await;
    assert_eq!(
        source.body_downloads(signers[1].address()),
        first_scan_downloads,
        "persistent drift retry must reuse the archived range-directed scan",
    );
    let complete_attempts = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*)
           FROM block_manifest_verification_attempt
          WHERE localization_complete",
    )
    .fetch_one(&pool)
    .await
    .expect("count complete persistent-drift attempts");
    assert_eq!(complete_attempts, 2);
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM drifted_handle WHERE status = 'unresolved'",
        )
        .fetch_one(&pool)
        .await
        .expect("keep the previously localized drift inventory"),
        localized_handles,
    );
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn concurrent_workers_do_not_invent_findings_when_historical_bodies_are_missing() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers, 2).await;

    let predecessor_block_number = 42;
    let predecessor_block_hash = B256::repeat_byte(0xa9);
    let current_block_number = 43;
    let current_block_hash = B256::repeat_byte(0xaa);
    let local_predecessor = sign_payload(
        &signers[0],
        payload_at(
            signers[0].address(),
            0x31,
            predecessor_block_number,
            predecessor_block_hash,
        ),
    )
    .await;
    let mut quorum_predecessors = Vec::new();
    for signer in &signers[1..] {
        quorum_predecessors.push(
            sign_payload(
                signer,
                payload_at(
                    signer.address(),
                    0x32,
                    predecessor_block_number,
                    predecessor_block_hash,
                ),
            )
            .await,
        );
    }

    // The local covering manifest is archived, but the matching peer bodies are
    // unavailable. The signed historical roots still prove a difference, while
    // localization must remain incomplete rather than inventing handles.
    archive_local_only(&pool, &local_predecessor).await;
    let local = sign_payload(
        &signers[0],
        payload_with_history(
            signers[0].address(),
            0x41,
            current_block_number,
            current_block_hash,
            &local_predecessor,
        ),
    )
    .await;
    schedule_local(&pool, &local, 0).await;

    let source = Arc::new(FakePeerSource::default());
    for (signer, predecessor) in signers[1..].iter().zip(&quorum_predecessors) {
        let current = sign_payload(
            signer,
            payload_with_history(
                signer.address(),
                0x41,
                current_block_number,
                current_block_hash,
                predecessor,
            ),
        )
        .await;
        source.set_manifest(signer.address(), &current);
    }

    let outcomes = concurrent_wave(&pool, &source).await;
    assert!(outcomes.iter().all(Result::is_ok), "{outcomes:?}");
    assert_eq!(completed_runs(&outcomes), 1, "{outcomes:?}");
    let result = outcomes
        .iter()
        .find_map(|outcome| outcome.as_ref().ok().copied().flatten())
        .expect("one worker completed incomplete historical verification");
    assert_eq!(result.outcome, VerificationOutcome::Drift);
    assert_target(&pool, "retry_exhausted", "drift", 1).await;
    assert_peer_failure(&pool, "incomplete:").await;
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM drifted_handle")
            .fetch_one(&pool)
            .await
            .expect("count findings after incomplete localization"),
        0,
        "missing historical bodies must not be interpreted as missing or drifted handles",
    );
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn covering_s3_is_not_retried_on_the_same_task() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers[..2], 2).await;

    let predecessor_block_number = 42;
    let predecessor_block_hash = B256::repeat_byte(0xa9);
    let current_block_number = 43;
    let current_block_hash = B256::repeat_byte(0xaa);
    let local_predecessor = sign_payload(
        &signers[0],
        payload_at(
            signers[0].address(),
            0x31,
            predecessor_block_number,
            predecessor_block_hash,
        ),
    )
    .await;
    archive_local_only(&pool, &local_predecessor).await;
    let local = sign_payload(
        &signers[0],
        payload_with_history(
            signers[0].address(),
            0x41,
            current_block_number,
            current_block_hash,
            &local_predecessor,
        ),
    )
    .await;
    schedule_local(&pool, &local, 1).await;

    let source = Arc::new(FakePeerSource::default());
    let current = sign_payload(
        &signers[1],
        payload_with_history(
            signers[1].address(),
            0x41,
            current_block_number,
            current_block_hash,
            &sign_payload(
                &signers[1],
                payload_at(
                    signers[1].address(),
                    0x32,
                    predecessor_block_number,
                    predecessor_block_hash,
                ),
            )
            .await,
        ),
    )
    .await;
    source.set_manifest(signers[1].address(), &current);

    let first_wave = concurrent_wave(&pool, &source).await;
    assert_eq!(completed_runs(&first_wave), 1, "{first_wave:?}");
    let lists_after_first = source.list_calls(signers[1].address());
    assert!(
        lists_after_first >= 2,
        "first attempt lists the current prefix and the missing covering prefix: {lists_after_first}"
    );

    sqlx::query(
        "UPDATE block_manifest_verification_task SET next_attempt_at = NOW() - INTERVAL '1 second'",
    )
    .execute(&pool)
    .await
    .expect("make same-task retry immediately due");

    let second_wave = concurrent_wave(&pool, &source).await;
    assert_eq!(completed_runs(&second_wave), 1, "{second_wave:?}");
    assert_eq!(
        source.list_calls(signers[1].address()),
        lists_after_first + 1,
        "same-task retry lists the current prefix only; covering S3 is not fetched again",
    );
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn later_publication_retries_missing_covering_manifest() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers[..2], 2).await;

    let predecessor_block_number = 42;
    let predecessor_block_hash = B256::repeat_byte(0xa9);
    let local_predecessor = sign_payload(
        &signers[0],
        payload_at(
            signers[0].address(),
            0x31,
            predecessor_block_number,
            predecessor_block_hash,
        ),
    )
    .await;
    archive_local_only(&pool, &local_predecessor).await;

    let source = Arc::new(FakePeerSource::default());
    let first_current_number = 43;
    let first_current_hash = B256::repeat_byte(0xaa);
    let first_local = sign_payload(
        &signers[0],
        payload_with_history(
            signers[0].address(),
            0x41,
            first_current_number,
            first_current_hash,
            &local_predecessor,
        ),
    )
    .await;
    schedule_local(&pool, &first_local, 0).await;
    let peer_predecessor = sign_payload(
        &signers[1],
        payload_at(
            signers[1].address(),
            0x32,
            predecessor_block_number,
            predecessor_block_hash,
        ),
    )
    .await;
    let first_peer = sign_payload(
        &signers[1],
        payload_with_history(
            signers[1].address(),
            0x41,
            first_current_number,
            first_current_hash,
            &peer_predecessor,
        ),
    )
    .await;
    source.set_manifest(signers[1].address(), &first_peer);

    let first_wave = concurrent_wave(&pool, &source).await;
    assert_eq!(completed_runs(&first_wave), 1, "{first_wave:?}");
    let lists_after_first = source.list_calls(signers[1].address());

    let second_current_number = 44;
    let second_current_hash = B256::repeat_byte(0xab);
    let second_local = sign_payload(
        &signers[0],
        payload_with_complete_history(
            signers[0].address(),
            0x41,
            second_current_number,
            second_current_hash,
            &first_local,
            &[local_predecessor.clone(), first_local.clone()],
        ),
    )
    .await;
    schedule_local(&pool, &second_local, 0).await;
    let second_peer = sign_payload(
        &signers[1],
        payload_with_complete_history(
            signers[1].address(),
            0x41,
            second_current_number,
            second_current_hash,
            &first_peer,
            &[peer_predecessor, first_peer.clone()],
        ),
    )
    .await;
    source.set_manifests(signers[1].address(), &[first_peer, second_peer]);

    let second_wave = concurrent_wave(&pool, &source).await;
    assert_eq!(completed_runs(&second_wave), 1, "{second_wave:?}");
    assert_eq!(
        source.list_calls(signers[1].address()),
        lists_after_first + 2,
        "a later publication lists the new current prefix and retries the still-missing covering prefix",
    );
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn missing_covering_manifest_preserves_findings_from_closest_predecessor() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers, 2).await;

    let local_chain = predecessor_chain(&signers[0], 40, 3, 0x31).await;
    archive_local_only(&pool, &local_chain[0]).await;
    archive_local_only(&pool, &local_chain[1]).await;
    schedule_local(&pool, &local_chain[2], 0).await;

    let source = Arc::new(FakePeerSource::default());
    for signer in &signers[1..] {
        let peer_chain = predecessor_chain(signer, 40, 3, 0x32).await;
        source.set_manifests(
            signer.address(),
            &[peer_chain[0].clone(), peer_chain[2].clone()],
        );
    }

    let outcomes = concurrent_wave(&pool, &source).await;
    assert!(outcomes.iter().all(Result::is_ok), "{outcomes:?}");
    assert_eq!(completed_runs(&outcomes), 1, "{outcomes:?}");
    let result = outcomes
        .iter()
        .find_map(|outcome| outcome.as_ref().ok().copied().flatten())
        .expect("one worker completed predecessor fallback verification");
    assert_eq!(result.outcome, VerificationOutcome::Drift);

    let attempt = sqlx::query(
        "SELECT localization_complete, drifted_block_count, drifted_handle_count
           FROM block_manifest_verification_attempt",
    )
    .fetch_one(&pool)
    .await
    .expect("load predecessor fallback evidence");
    assert!(!attempt.try_get::<bool, _>("localization_complete").unwrap());
    assert_eq!(
        attempt
            .try_get::<Option<i64>, _>("drifted_block_count")
            .unwrap(),
        None,
    );
    assert_eq!(
        attempt
            .try_get::<Option<i64>, _>("drifted_handle_count")
            .unwrap(),
        None,
    );
    assert!(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM drifted_handle WHERE status = 'unresolved'",
        )
        .fetch_one(&pool)
        .await
        .expect("count predecessor-derived handle findings")
            > 0,
        "the known predecessor branch must retain its handle findings",
    );
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn missing_current_manifest_reconstructs_its_history_from_predecessor_ranges() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers, 2).await;

    let local_chain = predecessor_chain(&signers[0], 40, 3, 0x31).await;
    archive_local_only(&pool, &local_chain[0]).await;
    archive_local_only(&pool, &local_chain[1]).await;
    schedule_local(&pool, &local_chain[2], 0).await;

    let source = Arc::new(FakePeerSource::default());
    for signer in &signers[1..] {
        let peer_chain = predecessor_chain(signer, 40, 3, 0x31).await;
        for range in &local_chain[2].payload.historical_ranges {
            let scope = CommitmentScope::Historical {
                first: range.start_block_number,
                last: range.end_block_number,
                scale: range.scale,
                end_block_hash: range.end_block_hash,
            };
            assert_eq!(
                derive_historical_scope_digest(&peer_chain[1].payload, &scope),
                Some(range.digest),
                "the predecessor's refined ranges must reconstruct the current history root",
            );
        }
        source.set_manifests(
            signer.address(),
            &[peer_chain[0].clone(), peer_chain[1].clone()],
        );
    }

    let outcomes = concurrent_wave(&pool, &source).await;
    assert!(outcomes.iter().all(Result::is_ok), "{outcomes:?}");
    assert_eq!(completed_runs(&outcomes), 1, "{outcomes:?}");
    let result = outcomes
        .iter()
        .find_map(|outcome| outcome.as_ref().ok().copied().flatten())
        .expect("one worker completed derived history verification");
    assert_eq!(result.outcome, VerificationOutcome::PartialConsensus);
    assert_target(&pool, "retry_exhausted", "partial_consensus", 1).await;
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM drifted_handle")
            .fetch_one(&pool)
            .await
            .expect("count findings after derived historical consensus"),
        0,
    );
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn concurrent_one_drifter_replay_resolves_exact_handle_findings() {
    struct DriftedBlock {
        number: i64,
        hash: B256,
        handle: B256,
        material: u8,
    }

    let blocks = [
        DriftedBlock {
            number: 201,
            hash: B256::repeat_byte(0xc1),
            handle: B256::repeat_byte(0x41),
            material: 0x51,
        },
        DriftedBlock {
            number: 202,
            hash: B256::repeat_byte(0xc2),
            handle: B256::repeat_byte(0x42),
            material: 0x52,
        },
    ];
    let quorum_keyset_id = U256::from(17);
    let drifted_keyset_id = U256::from(99);
    let local_gateway_key_id = U256::from(77);

    let (_instance, pool) = setup_download_db().await;
    let signers = five_test_signers();
    seed_registry(&pool, &signers, 3).await;
    let representative = signers[1..]
        .iter()
        .min_by_key(|signer| signer.address())
        .expect("four quorum publishers")
        .address();
    let mut quorum_commitment_digests = HashMap::new();
    let mut drifted_manifests = Vec::new();

    for block in &blocks {
        for signer in &signers[1..] {
            let manifest = sign_payload(
                signer,
                descriptor_payload_at(
                    signer.address(),
                    block.material,
                    quorum_keyset_id,
                    Some(U256::from(17)),
                    block.handle,
                    0,
                    block.number,
                    block.hash,
                ),
            )
            .await;
            if signer.address() == representative {
                quorum_commitment_digests
                    .insert(block.number, manifest.payload.detailed_range.digest);
            }
            archive_only(&pool, &manifest).await;
        }
        let local = sign_payload(
            &signers[0],
            descriptor_payload_at(
                signers[0].address(),
                block.material,
                drifted_keyset_id,
                Some(local_gateway_key_id),
                block.handle,
                0,
                block.number,
                block.hash,
            ),
        )
        .await;
        schedule_local(&pool, &local, 0).await;
        drifted_manifests.push(local);
    }

    let source = Arc::new(FakePeerSource::default());
    let drift_wave = concurrent_wave(&pool, &source).await;
    assert!(drift_wave.iter().all(Result::is_ok), "{drift_wave:?}");
    assert_eq!(completed_runs(&drift_wave), blocks.len(), "{drift_wave:?}");
    assert!(drift_wave
        .iter()
        .filter_map(|result| result.as_ref().ok().copied().flatten())
        .all(|result| result.outcome == VerificationOutcome::Drift));

    let rows = sqlx::query(
        r#"
        SELECT finding.version,
               finding.coprocessor_context_id,
               finding.host_chain_id,
               finding.block_number,
               finding.block_hash,
               finding.handle,
               finding.status,
               finding.local_present,
               finding.observed_present,
               finding.local_keyset_id,
               finding.observed_keyset_id,
               finding.local_gateway_key_id,
               finding.local_ct64_digest,
               finding.observed_ct64_digest,
               finding.local_ct128_digest,
               finding.observed_ct128_digest,
               finding.local_ct128_format,
               finding.observed_ct128_format,
               finding.observed_commitment_digest,
               finding.target_ct64_digest,
               finding.last_observed_task_id,
               finding.resolved_task_id,
               finding.healed_at IS NULL AS resolution_missing,
               detected_manifest.revision AS detected_revision,
               detected.latest_outcome AS detected_outcome
          FROM drifted_handle finding
          JOIN block_manifest_verification_task detected
            ON detected.id = finding.last_observed_task_id
          JOIN block_manifest detected_manifest
            ON detected_manifest.id = detected.local_manifest_id
         ORDER BY finding.block_number
        "#,
    )
    .fetch_all(&pool)
    .await
    .expect("load unresolved drift handle rows");
    assert_eq!(rows.len(), blocks.len());
    for (row, block, _local_manifest) in rows
        .iter()
        .zip(&blocks)
        .zip(&drifted_manifests)
        .map(|((row, block), local_manifest)| (row, block, local_manifest))
    {
        assert_eq!(row.try_get::<i16, _>("version").unwrap(), 1);
        assert_eq!(
            row.try_get::<Vec<u8>, _>("coprocessor_context_id").unwrap(),
            TEST_CONTEXT_ID.to_be_bytes::<32>()
        );
        assert_eq!(
            row.try_get::<i64, _>("host_chain_id").unwrap(),
            TEST_CHAIN_ID
        );
        assert_eq!(row.try_get::<i64, _>("block_number").unwrap(), block.number);
        assert_eq!(
            row.try_get::<Vec<u8>, _>("block_hash").unwrap(),
            block.hash.to_vec()
        );
        assert_eq!(
            row.try_get::<Vec<u8>, _>("handle").unwrap(),
            block.handle.to_vec()
        );
        assert_eq!(row.try_get::<String, _>("status").unwrap(), "unresolved");
        assert!(row.try_get::<bool, _>("local_present").unwrap());
        assert!(row.try_get::<bool, _>("observed_present").unwrap());
        assert_eq!(
            row.try_get::<Vec<u8>, _>("local_keyset_id").unwrap(),
            drifted_keyset_id.to_be_bytes::<32>()
        );
        assert_eq!(
            row.try_get::<Vec<u8>, _>("observed_keyset_id").unwrap(),
            quorum_keyset_id.to_be_bytes::<32>()
        );
        assert_eq!(
            row.try_get::<Vec<u8>, _>("local_gateway_key_id").unwrap(),
            local_gateway_key_id.to_be_bytes::<32>()
        );
        assert_eq!(
            row.try_get::<Vec<u8>, _>("local_ct64_digest").unwrap(),
            B256::repeat_byte(block.material).to_vec()
        );
        assert_eq!(
            row.try_get::<Vec<u8>, _>("observed_ct64_digest").unwrap(),
            B256::repeat_byte(block.material).to_vec()
        );
        assert_eq!(
            row.try_get::<Vec<u8>, _>("local_ct128_digest").unwrap(),
            B256::repeat_byte(block.material.wrapping_add(1)).to_vec()
        );
        assert_eq!(
            row.try_get::<Vec<u8>, _>("observed_ct128_digest").unwrap(),
            B256::repeat_byte(block.material.wrapping_add(1)).to_vec()
        );
        assert_eq!(
            row.try_get::<i16, _>("local_ct128_format").unwrap(),
            CiphertextFormat::CompressedOnCpu as u8 as i16
        );
        assert_eq!(
            row.try_get::<i16, _>("observed_ct128_format").unwrap(),
            CiphertextFormat::CompressedOnCpu as u8 as i16
        );
        assert_ne!(
            row.try_get::<Vec<u8>, _>("local_keyset_id").unwrap(),
            row.try_get::<Vec<u8>, _>("observed_keyset_id").unwrap()
        );
        assert_eq!(
            row.try_get::<Vec<u8>, _>("observed_commitment_digest")
                .unwrap(),
            quorum_commitment_digests[&block.number].to_vec()
        );
        assert_eq!(row.try_get::<i64, _>("detected_revision").unwrap(), 0);
        assert_eq!(
            row.try_get::<String, _>("detected_outcome").unwrap(),
            "drift"
        );
        assert!(row
            .try_get::<Option<i64>, _>("resolved_task_id")
            .unwrap()
            .is_none());
        assert!(row.try_get::<bool, _>("resolution_missing").unwrap());
    }

    for block in &blocks {
        let replayed = sign_payload(
            &signers[0],
            descriptor_payload_at(
                signers[0].address(),
                block.material,
                quorum_keyset_id,
                Some(local_gateway_key_id),
                block.handle,
                1,
                block.number,
                block.hash,
            ),
        )
        .await;
        schedule_local(&pool, &replayed, 0).await;
    }

    let replay_wave = concurrent_wave(&pool, &source).await;
    assert!(replay_wave.iter().all(Result::is_ok), "{replay_wave:?}");
    assert_eq!(
        completed_runs(&replay_wave),
        blocks.len(),
        "{replay_wave:?}"
    );
    assert!(replay_wave
        .iter()
        .filter_map(|result| result.as_ref().ok().copied().flatten())
        .all(|result| result.outcome == VerificationOutcome::Consensus));

    let resolved_rows = sqlx::query(
        r#"
        SELECT finding.block_number,
               finding.status,
               finding.last_observed_task_id,
               finding.resolved_task_id,
               finding.healed_at IS NULL AS healing_not_performed,
               resolved_manifest.revision AS resolved_revision,
               resolved.latest_outcome AS resolved_outcome
          FROM drifted_handle finding
          JOIN block_manifest_verification_task resolved
            ON resolved.id = finding.resolved_task_id
          JOIN block_manifest resolved_manifest
            ON resolved_manifest.id = resolved.local_manifest_id
         ORDER BY finding.block_number
        "#,
    )
    .fetch_all(&pool)
    .await
    .expect("load resolved drift handle rows");
    assert_eq!(resolved_rows.len(), blocks.len());
    for row in &resolved_rows {
        assert_eq!(row.try_get::<String, _>("status").unwrap(), "resolved");
        let last_task_id = row.try_get::<i64, _>("last_observed_task_id").unwrap();
        assert_eq!(
            row.try_get::<i64, _>("resolved_task_id").unwrap(),
            last_task_id
        );
        assert_eq!(row.try_get::<i64, _>("resolved_revision").unwrap(), 1);
        assert_eq!(
            row.try_get::<String, _>("resolved_outcome").unwrap(),
            "consensus"
        );
        assert!(row.try_get::<bool, _>("healing_not_performed").unwrap());
    }
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM drifted_handle")
            .fetch_one(&pool)
            .await
            .expect("count drift findings after replay"),
        blocks.len() as i64,
        "rechecking must resolve the existing findings rather than insert replacements",
    );
    for signer in &signers {
        assert_eq!(source.body_downloads(signer.address()), 0);
    }
}

#[test]
fn parses_path_style_and_virtual_host_bucket_urls() {
    assert_eq!(
        s3_bucket_location("http://localhost:4566/peer-ct128/operator-1").unwrap(),
        S3BucketLocation {
            bucket: "peer-ct128".into(),
            key_prefix: "operator-1".into(),
        },
    );
    assert_eq!(
        s3_bucket_location("https://peer-ct128.s3.eu-west-1.amazonaws.com/operator-1").unwrap(),
        S3BucketLocation {
            bucket: "peer-ct128".into(),
            key_prefix: "operator-1".into(),
        },
    );
}

#[tokio::test]
#[serial]
async fn s3_source_lists_all_numbered_revisions_and_skips_known_bodies() {
    let localstack = test_harness::localstack::start_localstack()
        .await
        .expect("start LocalStack for peer manifest download");
    let client =
        Arc::new(test_harness::localstack::create_localstack_s3_client(localstack.host_port).await);
    let bucket = "peer-manifest-download";
    client
        .create_bucket()
        .bucket(bucket)
        .send()
        .await
        .expect("create peer manifest bucket");

    let signer = PrivateKeySigner::random();
    let epoch = "v0.15/.././release candidate/~2E/%2E%2E/rc?#\\é";
    let epoch_payload = |revision| {
        let mut payload = revision_payload(signer.address(), 1, revision);
        payload.consensus_epoch = epoch.to_owned();
        payload
    };
    let revision_zero = sign_payload(&signer, epoch_payload(0)).await;
    let revision_one = sign_payload(&signer, epoch_payload(1)).await;
    let revision_two = sign_payload(&signer, epoch_payload(2)).await;
    let escaped_epoch = "v0.15/~2E~2E/~2E/release~20candidate/~7E2E/~252E~252E/rc~3F~23~5C~C3~A9";
    assert!(manifest_object_key(&revision_two).contains(escaped_epoch));
    for manifest in [&revision_zero, &revision_one, &revision_two] {
        let canonical_key = manifest_object_key(manifest);
        client
            .put_object()
            .bucket(bucket)
            .key(format!("operator-1/{canonical_key}"))
            .body(ByteStream::from(
                serde_json::to_vec(manifest).expect("serialize S3 peer manifest"),
            ))
            .send()
            .await
            .expect("upload numbered peer manifest");
    }

    let source = S3PeerManifestSource::new(client);
    let request = PeerDownloadRequest {
        publisher: signer.address(),
        s3_bucket_url: format!(
            "http://localhost:{}/{bucket}/operator-1",
            localstack.host_port
        ),
        version: ManifestVersion::V1,
        generation: epoch.to_owned(),
        coprocessor_context_id: TEST_CONTEXT_ID,
        host_chain_id: TEST_CHAIN_ID,
        publication_block_number: TEST_BLOCK_NUMBER,
        publication_block_hash: test_block_hash(),
        highest_archived_revision: Some(1),
        rejected_object_keys: HashSet::new(),
    };
    let object_keys = source
        .list_manifests(&request)
        .await
        .expect("list unknown peer revisions");
    let mut downloaded = Vec::new();
    for object_key in object_keys {
        downloaded.push(
            source
                .fetch_manifest(&request, &object_key)
                .await
                .expect("download peer revision"),
        );
    }

    assert_eq!(
        downloaded
            .iter()
            .map(|object| object.object_key.rsplit('/').next().unwrap())
            .collect::<Vec<_>>(),
        ["2"],
    );
    assert!(downloaded
        .iter()
        .all(|object| object.object_key.starts_with("manifests/")));

    let authenticated = crate::manifest_consensus::manifest_archive::authenticate_manifest_object(
        signer.address(),
        &downloaded[0].object_key,
        &downloaded[0].signed_bytes,
    )
    .expect("escaped S3 key authenticates against the original signed epoch");
    assert_eq!(authenticated.signed.payload.consensus_epoch, epoch);

    let mut request = request;
    let mut other_publication = revision_two.clone();
    other_publication.payload.publication_block_hash = B256::repeat_byte(0xfe);
    request
        .rejected_object_keys
        .insert(manifest_object_key(&other_publication));
    assert_eq!(
        source
            .list_manifests(&request)
            .await
            .expect("list with unrelated rejection"),
        vec![manifest_object_key(&revision_two)],
    );
    request
        .rejected_object_keys
        .insert(manifest_object_key(&revision_two));
    assert!(source
        .list_manifests(&request)
        .await
        .expect("skip exact rejected key beneath bucket prefix")
        .is_empty());
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn transient_verification_db_error_does_not_charge_the_attempt_budget() {
    let claim = claimed_local_task(1).await;
    apply_claimed_db_error(&claim.pool, &claim.claim, &sqlx::Error::PoolTimedOut)
        .await
        .expect("release uncharged after a transient database error");
    assert_verification_task(&claim.pool, "pending", 0).await;
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn integrity_verification_db_error_charges_the_attempt_budget() {
    let claim = claimed_local_task(2).await;
    fail_claimed_task(
        &claim.pool,
        &claim.claim,
        "unique constraint violation",
        false,
    )
    .await
    .expect("charge the attempt after an integrity error");
    assert_verification_task(&claim.pool, "pending", 1).await;
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn definitive_verification_db_error_exhausts_the_task() {
    let claim = claimed_local_task(5).await;
    apply_claimed_db_error(&claim.pool, &claim.claim, &sqlx::Error::RowNotFound)
        .await
        .expect("exhaust after a definitive database error");
    assert_verification_task(&claim.pool, "retry_exhausted", 1).await;
}

struct ClaimedLocalTask {
    _instance: DBInstance,
    pool: PgPool,
    claim: VerificationClaim,
}

async fn claimed_local_task(retry_count: u32) -> ClaimedLocalTask {
    let (instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers[..2], 2).await;
    let local = sign_payload(&signers[0], payload(signers[0].address(), 1)).await;
    schedule_local(&pool, &local, retry_count).await;
    let worker_id = "db-error-worker";
    bind_one_unbound_pending_task(&pool, GENERATION)
        .await
        .expect("bind verification task");
    let claim = claim_due_task(&pool, worker_id, Duration::from_secs(60), GENERATION)
        .await
        .expect("claim verification task")
        .expect("a due verification task exists");
    ClaimedLocalTask {
        _instance: instance,
        pool,
        claim,
    }
}

async fn assert_verification_task(pool: &PgPool, state: &str, attempt_count: i32) {
    let row = sqlx::query("SELECT state, attempt_count FROM block_manifest_verification_task")
        .fetch_one(pool)
        .await
        .expect("load verification task");
    assert_eq!(row.try_get::<String, _>("state").unwrap(), state);
    assert_eq!(
        row.try_get::<i32, _>("attempt_count").unwrap(),
        attempt_count
    );
}

async fn concurrent_wave(
    pool: &PgPool,
    source: &Arc<FakePeerSource>,
) -> Vec<Result<Option<VerificationRunResult>, String>> {
    let barrier = Arc::new(Barrier::new(TEST_WORKERS));
    let mut workers = Vec::with_capacity(TEST_WORKERS);
    for worker_id in 0..TEST_WORKERS {
        let pool = pool.clone();
        let source = Arc::clone(source);
        let barrier = Arc::clone(&barrier);
        workers.push(tokio::spawn(async move {
            barrier.wait().await;
            run_peer_manifest_download_once(
                &pool,
                source.as_ref(),
                &format!("test-worker-{worker_id}"),
                Duration::from_secs(30),
                GENERATION,
            )
            .await
            .map_err(|err| err.to_string())
        }));
    }
    let mut outcomes = Vec::with_capacity(TEST_WORKERS);
    for worker in workers {
        outcomes.push(worker.await.expect("download worker panicked"));
    }
    outcomes
}

fn completed_runs(outcomes: &[Result<Option<VerificationRunResult>, String>]) -> usize {
    outcomes
        .iter()
        .filter(|outcome| matches!(outcome, Ok(Some(_))))
        .count()
}

async fn setup_download_db() -> (DBInstance, PgPool) {
    let instance = setup_test_db(ImportMode::None)
        .await
        .expect("create peer download database");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(24)
        .connect(instance.db_url())
        .await
        .expect("connect peer download database");
    (instance, pool)
}

fn test_signers() -> [PrivateKeySigner; 3] {
    [
        PrivateKeySigner::random(),
        PrivateKeySigner::random(),
        PrivateKeySigner::random(),
    ]
}

fn five_test_signers() -> [PrivateKeySigner; 5] {
    std::array::from_fn(|_| PrivateKeySigner::random())
}

async fn seed_registry(pool: &PgPool, signers: &[PrivateKeySigner], threshold: i64) {
    for (index, signer) in signers.iter().enumerate() {
        sqlx::query(
            r#"
            INSERT INTO gateway_config_coprocessors (
                tx_sender_address,
                signer_address,
                s3_bucket_url,
                coprocessor_threshold,
                gateway_chain_id,
                gateway_config_address,
                snapshot_block_number,
                snapshot_block_hash
            )
            VALUES ($1, $2, $3, $4, 54321, $5, 100, $6)
            "#,
        )
        .bind(Address::repeat_byte(0x30 + index as u8).as_slice())
        .bind(signer.address().as_slice())
        .bind(format!("http://localhost:4566/peer-{index}"))
        .bind(threshold)
        .bind(Address::repeat_byte(0x10).as_slice())
        .bind(B256::repeat_byte(0x20).as_slice())
        .execute(pool)
        .await
        .expect("insert registry peer");
    }
}

async fn schedule_local(pool: &PgPool, manifest: &SignedManifest, retry_count: u32) {
    schedule_local_with_delay(pool, manifest, Duration::ZERO, retry_count).await;
}

async fn schedule_local_with_delay(
    pool: &PgPool,
    manifest: &SignedManifest,
    verification_delay: Duration,
    retry_count: u32,
) {
    let key = manifest_object_key(manifest);
    let body = serde_json::to_vec(manifest).expect("serialize local manifest");
    let mut trx = pool.begin().await.expect("begin local manifest schedule");
    let local = store_authenticated_manifest(
        &mut trx,
        manifest.payload.publisher,
        &key,
        &body,
        ManifestSource::Local,
    )
    .await
    .expect("archive local manifest");
    schedule_manifest_verification(
        &mut trx,
        local.id,
        verification_delay,
        Duration::from_secs(30),
        retry_count,
    )
    .await
    .expect("schedule local manifest verification");
    trx.commit().await.expect("commit local verification task");
}

async fn archive_only(pool: &PgPool, manifest: &SignedManifest) {
    archive_manifest(pool, manifest, ManifestSource::Peer).await;
}

async fn archive_local_only(pool: &PgPool, manifest: &SignedManifest) {
    archive_manifest(pool, manifest, ManifestSource::Local).await;
}

async fn archive_manifest(pool: &PgPool, manifest: &SignedManifest, source: ManifestSource) {
    let key = manifest_object_key(manifest);
    let body = serde_json::to_vec(manifest).expect("serialize archived manifest");
    let mut trx = pool.begin().await.expect("begin manifest archive");
    store_authenticated_manifest(&mut trx, manifest.payload.publisher, &key, &body, source)
        .await
        .expect("archive peer manifest");
    trx.commit().await.expect("commit peer manifest archive");
}

async fn assert_target(pool: &PgPool, state: &str, outcome: &str, attempt_count: i32) {
    let row = sqlx::query(
        r#"
        SELECT state, latest_outcome, attempt_count
          FROM block_manifest_verification_task
        "#,
    )
    .fetch_one(pool)
    .await
    .expect("load verification task");
    assert_eq!(row.try_get::<String, _>("state").unwrap(), state);
    assert_eq!(row.try_get::<String, _>("latest_outcome").unwrap(), outcome,);
    assert_eq!(
        row.try_get::<i32, _>("attempt_count").unwrap(),
        attempt_count,
    );
}

async fn assert_peer_failure(pool: &PgPool, prefix: &str) {
    let error = sqlx::query_scalar::<_, Option<String>>(
        "SELECT last_error
           FROM block_manifest_peer_download
          WHERE last_error IS NOT NULL
          LIMIT 1",
    )
    .fetch_one(pool)
    .await
    .expect("load peer download failure")
    .expect("peer failure is recorded");
    assert!(
        error.starts_with(prefix),
        "unexpected peer failure: {error}"
    );
}

async fn assert_attempt_completed(pool: &PgPool) {
    let row = sqlx::query("SELECT state, attempt_count FROM block_manifest_verification_task")
        .fetch_one(pool)
        .await
        .expect("load completed verification task");
    assert_eq!(row.try_get::<i32, _>("attempt_count").unwrap(), 1);
    assert!(
        matches!(
            row.try_get::<String, _>("state").unwrap().as_str(),
            "consensus" | "retry_exhausted"
        ),
        "peer failures must complete rather than leave the task claimed"
    );
}

async fn archive_count(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT COUNT(*) FROM block_manifest")
        .fetch_one(pool)
        .await
        .expect("count archived manifests")
}

async fn sign_payload(signer: &PrivateKeySigner, payload: ManifestPayload) -> SignedManifest {
    payload.sign(signer).await.expect("sign peer manifest")
}

fn payload(publisher: Address, material: u8) -> ManifestPayload {
    revision_payload(publisher, material, 0)
}

fn revision_payload(publisher: Address, material: u8, revision: u64) -> ManifestPayload {
    revision_payload_at(
        publisher,
        material,
        revision,
        TEST_BLOCK_NUMBER,
        test_block_hash(),
    )
}

fn payload_at(
    publisher: Address,
    material: u8,
    block_number: i64,
    block_hash: B256,
) -> ManifestPayload {
    revision_payload_at(publisher, material, 0, block_number, block_hash)
}

fn payload_with_history(
    publisher: Address,
    material: u8,
    block_number: i64,
    block_hash: B256,
    predecessor: &SignedManifest,
) -> ManifestPayload {
    payload_with_historical_range(
        publisher,
        material,
        block_number,
        block_hash,
        predecessor,
        predecessor,
    )
}

fn payload_with_historical_range(
    publisher: Address,
    material: u8,
    block_number: i64,
    block_hash: B256,
    predecessor: &SignedManifest,
    historical_start: &SignedManifest,
) -> ManifestPayload {
    let mut payload = payload_at(publisher, material, block_number, block_hash);
    link_predecessor(&mut payload, predecessor);
    payload.historical_ranges = vec![HistoricalRange {
        start_block_number: historical_start.payload.publication_block_number,
        end_block_number: historical_start.payload.publication_block_number,
        scale: 0,
        end_block_hash: historical_start.payload.publication_block_hash,
        digest: historical_start
            .payload
            .detailed_range
            .blocks
            .last()
            .expect("predecessor block")
            .block_content_digest,
    }];
    payload
}

fn payload_with_complete_history(
    publisher: Address,
    material: u8,
    block_number: i64,
    block_hash: B256,
    predecessor: &SignedManifest,
    history: &[SignedManifest],
) -> ManifestPayload {
    let mut payload = payload_at(publisher, material, block_number, block_hash);
    link_predecessor(&mut payload, predecessor);
    payload.historical_ranges = canonical_historical_ranges(history);
    payload
}

fn canonical_historical_ranges(history: &[SignedManifest]) -> Vec<HistoricalRange> {
    let mut upper = history.len();
    let mut previous_scale = 0;
    let mut ranges = Vec::new();
    while upper > 0 {
        let scale = canonical_history_scale(U256::from(upper), previous_scale)
            .expect("canonical test history scale");
        let size = 1_usize
            .checked_shl(scale)
            .expect("canonical test history range size");
        let start = upper
            .checked_sub(size)
            .expect("canonical test history range starts at zero or later");
        ranges.push(historical_range(&history[start..upper], scale));
        upper = start;
        previous_scale = scale;
    }
    ranges
}

fn historical_range(history: &[SignedManifest], scale: u32) -> HistoricalRange {
    let first = history.first().expect("historical range has a first block");
    let last = history.last().expect("historical range has a last block");
    HistoricalRange {
        start_block_number: first.payload.publication_block_number,
        end_block_number: last.payload.publication_block_number,
        scale,
        end_block_hash: last.payload.publication_block_hash,
        digest: historical_range_digest(history, scale),
    }
}

fn historical_range_digest(history: &[SignedManifest], scale: u32) -> B256 {
    if scale == 0 {
        return history
            .first()
            .expect("single-block historical range")
            .payload
            .detailed_range
            .blocks[0]
            .block_content_digest;
    }
    let (left, right) = history.split_at(history.len() / 2);
    let first = left
        .first()
        .expect("left historical range has a first block");
    let last = right
        .last()
        .expect("right historical range has a last block");
    dyadic_range_digest(
        ManifestVersion::V1,
        TEST_CONTEXT_ID,
        U256::from(TEST_CHAIN_ID),
        first.payload.publication_block_number,
        last.payload.publication_block_number,
        scale,
        last.payload.publication_block_hash,
        historical_range_digest(left, scale - 1),
        historical_range_digest(right, scale - 1),
    )
}

fn link_predecessor(payload: &mut ManifestPayload, predecessor: &SignedManifest) {
    payload.publication_parent_block_hash = predecessor.payload.publication_block_hash;
    payload
        .detailed_range
        .blocks
        .iter_mut()
        .for_each(|block| block.parent_block_hash = predecessor.payload.publication_block_hash);
}

async fn predecessor_chain(
    signer: &PrivateKeySigner,
    first_block_number: i64,
    length: i64,
    first_material: u8,
) -> Vec<SignedManifest> {
    let mut history = Vec::new();
    for offset in 0..length {
        let block_number = first_block_number + offset;
        let block_hash = B256::repeat_byte(
            u8::try_from(block_number % i64::from(u8::MAX)).expect("test block byte"),
        );
        let mut payload = payload_at(
            signer.address(),
            if offset == 0 { first_material } else { 0x41 },
            block_number,
            block_hash,
        );
        if let Some(previous) = history.last() {
            link_predecessor(&mut payload, previous);
            payload.historical_ranges = canonical_historical_ranges(&history);
        }
        history.push(sign_payload(signer, payload).await);
    }
    history
}

fn revision_payload_at(
    publisher: Address,
    material: u8,
    revision: u64,
    block_number: i64,
    block_hash: B256,
) -> ManifestPayload {
    descriptor_payload_at(
        publisher,
        material,
        U256::from(17),
        Some(U256::from(17)),
        B256::repeat_byte(1),
        revision,
        block_number,
        block_hash,
    )
}

#[allow(clippy::too_many_arguments)]
fn descriptor_payload_at(
    publisher: Address,
    material: u8,
    keyset_id: U256,
    gateway_key_id: Option<U256>,
    handle: B256,
    revision: u64,
    block_number: i64,
    block_hash: B256,
) -> ManifestPayload {
    let block_number = U256::from(u64::try_from(block_number).expect("positive test block"));
    let parent_block_hash = B256::repeat_byte(0xa9);
    let descriptors = vec![BlockCiphertextDescriptor::computed(
        handle,
        keyset_id,
        gateway_key_id,
        B256::repeat_byte(material),
        B256::repeat_byte(material.wrapping_add(1)),
        CiphertextFormat::CompressedOnCpu,
    )];
    let block_digest = block_content_digest(
        ManifestVersion::V1,
        TEST_CONTEXT_ID,
        U256::from(TEST_CHAIN_ID),
        block_number,
        block_hash,
        &descriptors,
    )
    .expect("compute peer block digest");
    let detailed_digest = detailed_range_digest(
        ManifestVersion::V1,
        TEST_CONTEXT_ID,
        U256::from(TEST_CHAIN_ID),
        block_number,
        block_number,
        &[block_digest],
    );
    ManifestPayload {
        version: ManifestVersion::V1,
        consensus_epoch: block_manifest::LEGACY_CONSENSUS_EPOCH.to_owned(),
        publisher,
        coprocessor_context_id: TEST_CONTEXT_ID,
        host_chain_id: U256::from(TEST_CHAIN_ID),
        publication_block_number: block_number,
        publication_block_hash: block_hash,
        publication_parent_block_hash: parent_block_hash,
        revision,
        detailed_range: DetailedRange {
            first_block_number: block_number,
            last_block_number: block_number,
            digest: detailed_digest,
            blocks: vec![ManifestBlockEntry {
                block_number,
                block_hash,
                parent_block_hash,
                block_content_digest: block_digest,
                ciphertexts: descriptors,
            }],
        },
        historical_ranges: vec![],
    }
}

fn test_block_hash() -> B256 {
    B256::repeat_byte(0xaa)
}

#[path = "localization_cache_tests.rs"]
mod localization_cache_tests;

#[path = "lease_renewal_tests.rs"]
mod lease_renewal_tests;

#[tokio::test]
#[serial]
async fn oversized_s3_head_is_rejected_and_falls_back_to_valid_older_revision() {
    let (_instance, pool) = setup_download_db().await;
    let localstack = test_harness::localstack::start_localstack().await.unwrap();
    let client =
        Arc::new(test_harness::localstack::create_localstack_s3_client(localstack.host_port).await);
    let bucket = "review-oversized-manifest";
    client.create_bucket().bucket(bucket).send().await.unwrap();
    let signers = test_signers();
    seed_registry(&pool, &signers[..2], 2).await;
    sqlx::query("UPDATE gateway_config_coprocessors SET s3_bucket_url = $1")
        .bind(format!(
            "http://localhost:{}/{bucket}",
            localstack.host_port
        ))
        .execute(&pool)
        .await
        .unwrap();
    let local = sign_payload(&signers[0], payload(signers[0].address(), 1)).await;
    schedule_local(&pool, &local, 0).await;
    let valid = sign_payload(&signers[1], payload(signers[1].address(), 1)).await;
    let higher = sign_payload(&signers[1], revision_payload(signers[1].address(), 1, 1)).await;
    for (key, body) in [
        (
            manifest_object_key(&valid),
            serde_json::to_vec(&valid).unwrap(),
        ),
        (
            manifest_object_key(&higher),
            vec![b'x'; block_manifest::MAX_MANIFEST_BYTES + 1],
        ),
    ] {
        client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(ByteStream::from(body))
            .send()
            .await
            .unwrap();
    }
    let result = run_peer_manifest_download_once(
        &pool,
        &S3PeerManifestSource::new(client),
        "oversize",
        Duration::from_secs(60),
        GENERATION,
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(result.outcome, VerificationOutcome::Consensus);

    assert_eq!(
        archive_count(&pool).await,
        2,
        "valid peer revision zero must be archived"
    );
    let rejected: Vec<String> =
        sqlx::query_scalar("SELECT rejected_object_keys FROM block_manifest_peer_download")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(rejected, vec![manifest_object_key(&higher)]);
}

#[tokio::test]
async fn healing_state_is_independent_of_later_manifest_agreement() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers, 2).await;
    let source = FakePeerSource::default();
    for signer in &signers[1..] {
        source.set_manifest(
            signer.address(),
            &sign_payload(signer, payload(signer.address(), 9)).await,
        );
    }
    let local = sign_payload(&signers[0], payload(signers[0].address(), 1)).await;
    schedule_local(&pool, &local, 0).await;
    run_peer_manifest_download_once(
        &pool,
        &source,
        "healing-schema",
        Duration::from_secs(60),
        GENERATION,
    )
    .await
    .unwrap()
    .unwrap();
    let id: i64 =
        sqlx::query_scalar("SELECT id FROM drifted_handle WHERE detection_kind = 'verified'")
            .fetch_one(&pool)
            .await
            .unwrap();
    // Inferred findings do not invent peer evidence or require completed SNS.
    let inferred: i64 = sqlx::query_scalar("INSERT INTO drifted_handle (generation, version, coprocessor_context_id, host_chain_id, block_number, block_hash, handle, detection_kind, reason, local_present, observed_present)
        SELECT generation, version, coprocessor_context_id, host_chain_id, block_number, block_hash, $2, 'inferred', 'ct64_mismatch', TRUE, FALSE FROM drifted_handle WHERE id = $1 RETURNING id")
        .bind(id).bind(B256::repeat_byte(0xee).as_slice()).fetch_one(&pool).await.unwrap();
    let defaults = sqlx::query("SELECT demand_count, can_be_healed, healed_at IS NULL AS pending, next_retry_at IS NULL AS no_retry, claimed_by IS NULL AS unclaimed, target_evidence IS NULL AS no_evidence, peer_sources FROM drifted_handle WHERE id = $1")
        .bind(inferred).fetch_one(&pool).await.unwrap();
    assert_eq!(defaults.get::<i64, _>("demand_count"), 0);
    assert!(!defaults.get::<bool, _>("can_be_healed"));
    for name in ["pending", "no_retry", "unclaimed", "no_evidence"] {
        assert!(defaults.get::<bool, _>(name));
    }
    assert_eq!(
        defaults.get::<serde_json::Value, _>("peer_sources"),
        serde_json::json!([])
    );
    for sql in [
        "UPDATE drifted_handle SET demand_count = -1 WHERE id = $1",
        "UPDATE drifted_handle SET claimed_by = 'worker' WHERE id = $1",
        "UPDATE drifted_handle SET healed_at = NOW() WHERE id = $1",
        "UPDATE drifted_handle SET target_evidence = '[]'::jsonb WHERE id = $1",
    ] {
        let error = sqlx::query(sql)
            .bind(inferred)
            .execute(&pool)
            .await
            .unwrap_err();
        assert_eq!(
            error.as_database_error().unwrap().code().as_deref(),
            Some("23514")
        );
    }
    sqlx::query("UPDATE drifted_handle SET demand_count = 3, claimed_by = 'worker', claim_expires_at = NOW() + INTERVAL '1 minute' WHERE id = $1")
        .bind(id).execute(&pool).await.unwrap();
    let matching = sign_payload(&signers[0], revision_payload(signers[0].address(), 9, 1)).await;
    schedule_local(&pool, &matching, 0).await;
    let result = run_peer_manifest_download_once(
        &pool,
        &source,
        "healing-schema",
        Duration::from_secs(60),
        GENERATION,
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(result.outcome, VerificationOutcome::Consensus);
    let row = sqlx::query("SELECT detection_kind, can_be_healed, healed_at IS NULL AS pending, demand_count, claimed_by, target_ct64_digest FROM drifted_handle WHERE id = $1")
        .bind(id).fetch_one(&pool).await.unwrap();
    assert_eq!(row.get::<String, _>("detection_kind"), "verified");
    assert!(row.get::<bool, _>("can_be_healed"));
    assert!(row.get::<bool, _>("pending"));
    assert_eq!(row.get::<i64, _>("demand_count"), 3);
    assert_eq!(row.get::<String, _>("claimed_by"), "worker");
    assert_eq!(
        row.get::<Vec<u8>, _>("target_ct64_digest"),
        B256::repeat_byte(9).to_vec()
    );
    sqlx::query("UPDATE drifted_handle SET target_ct64_digest = $2 WHERE id = $1")
        .bind(inferred)
        .bind(B256::repeat_byte(9).as_slice())
        .execute(&pool)
        .await
        .unwrap();
    let inferred_ready =
        sqlx::query("SELECT detection_kind, can_be_healed FROM drifted_handle WHERE id = $1")
            .bind(inferred)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(
        inferred_ready.get::<String, _>("detection_kind"),
        "inferred"
    );
    assert!(inferred_ready.get::<bool, _>("can_be_healed"));
    // Model installation's state transition, not ciphertext replacement itself.
    sqlx::query("UPDATE drifted_handle SET healed_at = NOW(), claimed_by = NULL, claim_expires_at = NULL WHERE id = $1")
        .bind(id).execute(&pool).await.unwrap();
}
