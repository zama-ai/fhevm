use super::*;
use crate::consensus::manifest_archive::{manifest_object_key, store_authenticated_manifest};
use alloy::signers::local::PrivateKeySigner;
use aws_sdk_s3::primitives::ByteStream;
use ciphertext_attestation::{
    manifest::{
        block_content_digest, detailed_range_digest, BlockCiphertextDescriptor, DetailedRange,
        HistoricalRange, ManifestBlockEntry, ManifestPayload,
    },
    CiphertextFormat,
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

#[derive(Default)]
struct FakePeerSource {
    objects: Mutex<HashMap<Address, Vec<PeerManifestObject>>>,
    list_calls: Mutex<HashMap<Address, usize>>,
    body_downloads: Mutex<HashMap<Address, usize>>,
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
}

impl PeerManifestSource for FakePeerSource {
    async fn fetch_manifests(
        &self,
        request: &PeerDownloadRequest,
    ) -> Result<Vec<PeerManifestObject>, ExecutionError> {
        *self
            .list_calls
            .lock()
            .expect("lock fake list calls")
            .entry(request.publisher)
            .or_default() += 1;
        let objects = self
            .objects
            .lock()
            .expect("lock fake objects")
            .get(&request.publisher)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter(|object| {
                object
                    .object_key
                    .rsplit('/')
                    .next()
                    .and_then(|revision| revision.parse::<u64>().ok())
                    .is_some_and(|revision| !request.known_revisions.contains(&revision))
            })
            .collect::<Vec<_>>();
        *self
            .body_downloads
            .lock()
            .expect("lock fake body downloads")
            .entry(request.publisher)
            .or_default() += objects.len();
        Ok(objects)
    }
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "database-backed concurrent peer manifest download simulation"]
#[serial]
async fn concurrent_downloaders_claim_once_and_reach_manifest_quorum() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers, 2).await;
    let local = sign_payload(&signers[0], payload(signers[0].address(), 1)).await;
    schedule_local(&pool, &local, 1).await;

    let source = Arc::new(FakePeerSource::default());
    let agreeing_zero = sign_payload(&signers[1], payload(signers[1].address(), 1)).await;
    let agreeing_one = sign_payload(
        &signers[1],
        revision_payload(
            signers[1].address(),
            1,
            1,
            Some(manifest_reference(&agreeing_zero)),
        ),
    )
    .await;
    let agreeing_two = sign_payload(
        &signers[1],
        revision_payload(
            signers[1].address(),
            1,
            2,
            Some(manifest_reference(&agreeing_one)),
        ),
    )
    .await;
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
    assert_eq!(source.body_downloads(signers[1].address()), 3);
    assert_eq!(source.body_downloads(signers[2].address()), 0);
    assert_target(&pool, "complete", "consensus", 1).await;
    assert_eq!(archive_count(&pool).await, 4);
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "database-backed delayed concurrent peer manifest download simulation"]
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
        "UPDATE block_consensus_verification_target SET next_attempt_at = NOW() - INTERVAL '1 second'",
    )
    .execute(&pool)
    .await
    .expect("advance the simulated verification clock");
    let due_wave = concurrent_wave(&pool, &source).await;
    assert_eq!(completed_runs(&due_wave), 1, "{due_wave:?}");
    assert!(due_wave.iter().all(Result::is_ok), "{due_wave:?}");
    assert_eq!(source.body_downloads(signers[1].address()), 1);
    assert_target(&pool, "complete", "consensus", 1).await;
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "database-backed lease recovery after simulated downloader crash"]
#[serial]
async fn expired_lease_recovers_after_mid_download_crash_without_redownloading() {
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

    let claim = claim_due_target(&pool, "crashed-worker", Duration::from_secs(60))
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
        "UPDATE block_consensus_verification_target SET lease_expires_at = NOW() - INTERVAL '1 second' WHERE id = $1",
    )
    .bind(claim.target_id)
    .execute(&pool)
    .await
    .expect("simulate process death and lease expiry");

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
    assert_target(&pool, "complete", "consensus", 1).await;
    assert_eq!(archive_count(&pool).await, 3);
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "database-backed concurrent invalid-download retry simulation"]
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
    assert_eq!(archive_count(&pool).await, 1);

    let valid = sign_payload(&signers[1], payload(signers[1].address(), 1)).await;
    source.set_manifest(signers[1].address(), &valid);
    sqlx::query(
        "UPDATE block_consensus_verification_target SET next_attempt_at = NOW() - INTERVAL '1 second'",
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
    assert_target(&pool, "complete", "consensus", 2).await;
    assert_eq!(archive_count(&pool).await, 2);
    assert_eq!(source.body_downloads(signers[1].address()), 2);
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "database-backed concurrent peer revision remission simulation"]
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
        "SELECT COUNT(*) FROM block_consensus_drift_handle WHERE status = 'unresolved'",
    )
    .fetch_one(&pool)
    .await
    .expect("count unresolved peer-revision drift findings");
    assert!(unresolved > 0);

    let repaired_one = sign_payload(
        &signers[1],
        revision_payload(
            signers[1].address(),
            1,
            1,
            Some(manifest_reference(&drifting_zero)),
        ),
    )
    .await;
    source.set_manifests(signers[1].address(), &[drifting_zero, repaired_one]);
    sqlx::query(
        "UPDATE block_consensus_verification_target
            SET next_attempt_at = NOW() - INTERVAL '1 second'
          WHERE state = 'pending'",
    )
    .execute(&pool)
    .await
    .expect("make peer-revision retry immediately due");

    let second_wave = concurrent_wave(&pool, &source).await;
    assert_eq!(completed_runs(&second_wave), 1, "{second_wave:?}");
    assert!(second_wave.iter().all(Result::is_ok), "{second_wave:?}");
    assert_target(&pool, "complete", "consensus", 2).await;
    assert_eq!(source.body_downloads(signers[1].address()), 2);
    assert_eq!(archive_count(&pool).await, 3);
    let states = sqlx::query(
        "SELECT COUNT(*) FILTER (WHERE status = 'resolved') AS resolved,
                COUNT(*) FILTER (WHERE status = 'unresolved') AS unresolved
           FROM block_consensus_drift_handle",
    )
    .fetch_one(&pool)
    .await
    .expect("load peer-revision remission state");
    assert_eq!(states.try_get::<i64, _>("resolved").unwrap(), unresolved);
    assert_eq!(states.try_get::<i64, _>("unresolved").unwrap(), 0);
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "database-backed concurrent bounded peer retry exhaustion simulation"]
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
    assert_target(&pool, "exhausted", "unknown_but_equal", 1).await;

    let later_wave = concurrent_wave(&pool, &source).await;
    assert!(later_wave.iter().all(|outcome| matches!(outcome, Ok(None))));
    assert_target(&pool, "exhausted", "unknown_but_equal", 1).await;
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "database-backed five-copro all-origin consensus matrix"]
#[serial]
async fn concurrent_workers_cover_five_copro_drift_populations_from_every_origin() {
    struct Scenario {
        name: &'static str,
        block_number: i64,
        block_hash: B256,
        digests: [u8; 5],
        expected_outcomes: [VerificationOutcome; 5],
        expected_quorum_scope_count: i32,
    }

    let scenarios = [
        Scenario {
            name: "one drifter",
            block_number: 101,
            block_hash: B256::repeat_byte(0xa1),
            digests: [0x11, 0x11, 0x11, 0x11, 0x21],
            expected_outcomes: [VerificationOutcome::Drift; 5],
            expected_quorum_scope_count: 1,
        },
        Scenario {
            name: "two matching drifters",
            block_number: 102,
            block_hash: B256::repeat_byte(0xa2),
            digests: [0x12, 0x12, 0x12, 0x22, 0x22],
            expected_outcomes: [VerificationOutcome::Drift; 5],
            expected_quorum_scope_count: 1,
        },
        Scenario {
            name: "three drifters split as pair and singleton",
            block_number: 103,
            block_hash: B256::repeat_byte(0xa3),
            digests: [0x13, 0x13, 0x23, 0x23, 0x33],
            expected_outcomes: [VerificationOutcome::Drift; 5],
            expected_quorum_scope_count: 0,
        },
        Scenario {
            name: "five drifters split as two pairs and singleton",
            block_number: 104,
            block_hash: B256::repeat_byte(0xa4),
            digests: [0x14, 0x14, 0x24, 0x24, 0x34],
            expected_outcomes: [VerificationOutcome::Drift; 5],
            expected_quorum_scope_count: 0,
        },
        Scenario {
            name: "all five results differ",
            block_number: 105,
            block_hash: B256::repeat_byte(0xa5),
            digests: [0x15, 0x25, 0x35, 0x45, 0x55],
            expected_outcomes: [VerificationOutcome::Drift; 5],
            expected_quorum_scope_count: 0,
        },
    ];

    let (_instance, pool) = setup_download_db().await;
    let signers = five_test_signers();
    seed_registry(&pool, &signers, 3).await;
    for scenario in &scenarios {
        for (signer, digest) in signers.iter().zip(scenario.digests) {
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
            schedule_local(&pool, &manifest, 0).await;
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
    assert_eq!(completed, scenarios.len() * signers.len());

    for scenario in &scenarios {
        for (signer, expected_outcome) in signers.iter().zip(scenario.expected_outcomes) {
            let row = sqlx::query(
                r#"
                SELECT id,
                       state,
                       latest_outcome,
                       attempt_count,
                       quorum_scope_count,
                       local_drift_scope_count
                  FROM block_consensus_verification_target
                 WHERE local_publisher = $1
                   AND publication_block_number = $2
                   AND publication_block_hash = $3
                "#,
            )
            .bind(signer.address().as_slice())
            .bind(scenario.block_number)
            .bind(scenario.block_hash.as_slice())
            .fetch_one(&pool)
            .await
            .expect("load five-copro verification result");
            let terminal_state = if expected_outcome == VerificationOutcome::Consensus {
                "complete"
            } else {
                "exhausted"
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
            assert_eq!(
                row.try_get::<i32, _>("quorum_scope_count").unwrap(),
                scenario.expected_quorum_scope_count,
            );
            assert_eq!(row.try_get::<i32, _>("local_drift_scope_count").unwrap(), 1,);
            if scenario.expected_quorum_scope_count == 0 {
                let target_id = row.try_get::<i64, _>("id").unwrap();
                let persisted = sqlx::query(
                    r#"
                    SELECT COUNT(*) AS finding_count,
                           BOOL_AND(NOT observed_has_quorum) AS all_below_quorum
                      FROM block_consensus_drift_handle
                     WHERE last_observed_target_id = $1
                    "#,
                )
                .bind(target_id)
                .fetch_one(&pool)
                .await
                .expect("load persisted no-quorum drift explanations");
                assert!(
                    persisted.try_get::<i64, _>("finding_count").unwrap() > 0,
                    "{} from local origin {} must persist handle differences",
                    scenario.name,
                    signer.address(),
                );
                assert!(
                    persisted
                        .try_get::<Option<bool>, _>("all_below_quorum")
                        .unwrap()
                        .unwrap_or(false),
                    "{} from local origin {} must not mark no-quorum evidence actionable",
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

#[tokio::test(flavor = "multi_thread")]
#[ignore = "database-backed historical drift localization through archived manifests"]
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

    archive_only(&pool, &local_predecessor).await;
    for predecessor in &quorum_predecessors {
        archive_only(&pool, predecessor).await;
    }

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
        .expect("one completed historical drift verification");
    assert_eq!(result.outcome, VerificationOutcome::Drift);

    let target = sqlx::query(
        r#"
        SELECT state, latest_outcome, quorum_scope_count, local_drift_scope_count
          FROM block_consensus_verification_target
        "#,
    )
    .fetch_one(&pool)
    .await
    .expect("load historical drift target");
    assert_eq!(target.try_get::<String, _>("state").unwrap(), "exhausted");
    assert_eq!(
        target.try_get::<String, _>("latest_outcome").unwrap(),
        "drift"
    );
    assert_eq!(target.try_get::<i32, _>("quorum_scope_count").unwrap(), 2);
    assert_eq!(
        target.try_get::<i32, _>("local_drift_scope_count").unwrap(),
        1
    );

    let finding = sqlx::query(
        r#"
        SELECT block_number,
               block_hash,
               finding_kind,
               local_ct64_digest,
               observed_ct64_digest,
               observed_has_quorum,
               ct64_digest_mismatch
          FROM block_consensus_drift_handle
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
        finding.try_get::<String, _>("finding_kind").unwrap(),
        "descriptor_mismatch"
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
    assert!(finding.try_get::<bool, _>("observed_has_quorum").unwrap());
    assert!(finding.try_get::<bool, _>("ct64_digest_mismatch").unwrap());
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "database-backed one-drifter replay and remission simulation"]
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
    let mut quorum_manifest_digests = HashMap::new();
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
                    None,
                    block.number,
                    block.hash,
                ),
            )
            .await;
            if signer.address() == representative {
                quorum_manifest_digests.insert(
                    block.number,
                    manifest.digest().expect("digest quorum manifest"),
                );
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
                None,
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
        SELECT finding.local_publisher,
               finding.version,
               finding.coprocessor_context_id,
               finding.host_chain_id,
               finding.block_number,
               finding.block_hash,
               finding.handle,
               finding.status,
               finding.finding_kind,
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
               finding.keyset_mismatch,
               finding.ct64_digest_mismatch,
               finding.ct128_digest_mismatch,
               finding.ct128_format_mismatch,
               finding.observed_publisher,
               finding.observed_manifest_digest,
               finding.observed_commitment_digest,
               finding.observed_has_quorum,
               finding.first_detected_target_id,
               finding.last_observed_target_id,
               finding.first_detected_local_manifest_digest,
               finding.last_observed_local_manifest_digest,
               finding.last_local_manifest_revision,
               finding.resolved_target_id,
               finding.resolved_local_manifest_digest,
               finding.resolved_local_manifest_revision,
               finding.resolved_at IS NULL AS resolution_missing,
               detected.revision AS detected_revision,
               detected.latest_outcome AS detected_outcome
          FROM block_consensus_drift_handle finding
          JOIN block_consensus_verification_target detected
            ON detected.id = finding.first_detected_target_id
         ORDER BY finding.block_number
        "#,
    )
    .fetch_all(&pool)
    .await
    .expect("load unresolved drift handle rows");
    assert_eq!(rows.len(), blocks.len());
    for (row, block, local_manifest) in rows
        .iter()
        .zip(&blocks)
        .zip(&drifted_manifests)
        .map(|((row, block), local_manifest)| (row, block, local_manifest))
    {
        assert_eq!(
            row.try_get::<Vec<u8>, _>("local_publisher").unwrap(),
            signers[0].address().to_vec()
        );
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
        assert_eq!(
            row.try_get::<String, _>("finding_kind").unwrap(),
            "descriptor_mismatch"
        );
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
        assert!(row.try_get::<bool, _>("keyset_mismatch").unwrap());
        assert!(!row.try_get::<bool, _>("ct64_digest_mismatch").unwrap());
        assert!(!row.try_get::<bool, _>("ct128_digest_mismatch").unwrap());
        assert!(!row.try_get::<bool, _>("ct128_format_mismatch").unwrap());
        assert_eq!(
            row.try_get::<Vec<u8>, _>("observed_publisher").unwrap(),
            representative.to_vec()
        );
        assert_eq!(
            row.try_get::<Vec<u8>, _>("observed_manifest_digest")
                .unwrap(),
            quorum_manifest_digests[&block.number].to_vec()
        );
        assert_eq!(
            row.try_get::<Vec<u8>, _>("observed_commitment_digest")
                .unwrap(),
            quorum_commitment_digests[&block.number].to_vec()
        );
        assert_eq!(
            row.try_get::<i64, _>("first_detected_target_id").unwrap(),
            row.try_get::<i64, _>("last_observed_target_id").unwrap()
        );
        let local_digest = local_manifest.digest().expect("digest drifting manifest");
        assert_eq!(
            row.try_get::<Vec<u8>, _>("first_detected_local_manifest_digest")
                .unwrap(),
            local_digest.to_vec()
        );
        assert_eq!(
            row.try_get::<Vec<u8>, _>("last_observed_local_manifest_digest")
                .unwrap(),
            local_digest.to_vec()
        );
        assert_eq!(
            row.try_get::<i64, _>("last_local_manifest_revision")
                .unwrap(),
            0
        );
        assert_eq!(row.try_get::<i64, _>("detected_revision").unwrap(), 0);
        assert_eq!(
            row.try_get::<String, _>("detected_outcome").unwrap(),
            "drift"
        );
        assert!(row
            .try_get::<Option<i64>, _>("resolved_target_id")
            .unwrap()
            .is_none());
        assert!(row
            .try_get::<Option<Vec<u8>>, _>("resolved_local_manifest_digest")
            .unwrap()
            .is_none());
        assert!(row
            .try_get::<Option<i64>, _>("resolved_local_manifest_revision")
            .unwrap()
            .is_none());
        assert!(row.try_get::<bool, _>("resolution_missing").unwrap());
    }

    let mut replayed_manifests = Vec::new();
    for (block, drifted) in blocks.iter().zip(&drifted_manifests) {
        let replayed = sign_payload(
            &signers[0],
            descriptor_payload_at(
                signers[0].address(),
                block.material,
                quorum_keyset_id,
                Some(local_gateway_key_id),
                block.handle,
                1,
                Some(manifest_reference(drifted)),
                block.number,
                block.hash,
            ),
        )
        .await;
        schedule_local(&pool, &replayed, 0).await;
        replayed_manifests.push(replayed);
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
               finding.first_detected_target_id,
               finding.last_observed_target_id,
               finding.resolved_target_id,
               finding.first_detected_local_manifest_digest,
               finding.last_observed_local_manifest_digest,
               finding.resolved_local_manifest_digest,
               finding.last_local_manifest_revision,
               finding.resolved_local_manifest_revision,
               finding.resolved_at IS NOT NULL AS resolution_present,
               resolved.revision AS resolved_revision,
               resolved.latest_outcome AS resolved_outcome
          FROM block_consensus_drift_handle finding
          JOIN block_consensus_verification_target resolved
            ON resolved.id = finding.resolved_target_id
         ORDER BY finding.block_number
        "#,
    )
    .fetch_all(&pool)
    .await
    .expect("load resolved drift handle rows");
    assert_eq!(resolved_rows.len(), blocks.len());
    for ((row, drifted), replayed) in resolved_rows
        .iter()
        .zip(&drifted_manifests)
        .zip(&replayed_manifests)
    {
        assert_eq!(row.try_get::<String, _>("status").unwrap(), "resolved");
        let first_target_id = row.try_get::<i64, _>("first_detected_target_id").unwrap();
        let last_target_id = row.try_get::<i64, _>("last_observed_target_id").unwrap();
        assert_ne!(first_target_id, last_target_id);
        assert_eq!(
            row.try_get::<i64, _>("resolved_target_id").unwrap(),
            last_target_id
        );
        assert_eq!(
            row.try_get::<Vec<u8>, _>("first_detected_local_manifest_digest")
                .unwrap(),
            drifted.digest().expect("digest drifting manifest").to_vec()
        );
        let replayed_digest = replayed.digest().expect("digest replayed manifest");
        assert_eq!(
            row.try_get::<Vec<u8>, _>("last_observed_local_manifest_digest")
                .unwrap(),
            replayed_digest.to_vec()
        );
        assert_eq!(
            row.try_get::<Vec<u8>, _>("resolved_local_manifest_digest")
                .unwrap(),
            replayed_digest.to_vec()
        );
        assert_eq!(
            row.try_get::<i64, _>("last_local_manifest_revision")
                .unwrap(),
            1
        );
        assert_eq!(
            row.try_get::<i64, _>("resolved_local_manifest_revision")
                .unwrap(),
            1
        );
        assert_eq!(row.try_get::<i64, _>("resolved_revision").unwrap(), 1);
        assert_eq!(
            row.try_get::<String, _>("resolved_outcome").unwrap(),
            "consensus"
        );
        assert!(row.try_get::<bool, _>("resolution_present").unwrap());
    }
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM block_consensus_drift_handle")
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
#[ignore = "LocalStack-backed numbered peer manifest S3 download"]
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
    let revision_zero = sign_payload(&signer, payload(signer.address(), 1)).await;
    let revision_one = sign_payload(
        &signer,
        revision_payload(
            signer.address(),
            1,
            1,
            Some(manifest_reference(&revision_zero)),
        ),
    )
    .await;
    let revision_two = sign_payload(
        &signer,
        revision_payload(
            signer.address(),
            1,
            2,
            Some(manifest_reference(&revision_one)),
        ),
    )
    .await;
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
    let downloaded = source
        .fetch_manifests(&PeerDownloadRequest {
            publisher: signer.address(),
            s3_bucket_url: format!(
                "http://localhost:{}/{bucket}/operator-1",
                localstack.host_port
            ),
            version: ManifestVersion::V1,
            coprocessor_context_id: TEST_CONTEXT_ID,
            host_chain_id: TEST_CHAIN_ID,
            publication_block_number: TEST_BLOCK_NUMBER,
            publication_block_hash: test_block_hash(),
            known_revisions: HashSet::from([1]),
        })
        .await
        .expect("list and download unknown peer revisions");

    assert_eq!(
        downloaded
            .iter()
            .map(|object| object.object_key.rsplit('/').next().unwrap())
            .collect::<Vec<_>>(),
        ["0", "2"],
    );
    assert!(downloaded
        .iter()
        .all(|object| object.object_key.starts_with("manifests/")));
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
    let local = store_authenticated_manifest(&mut trx, manifest.payload.publisher, &key, &body)
        .await
        .expect("archive local manifest");
    schedule_manifest_verification(
        &mut trx,
        &local.manifest,
        verification_delay,
        Duration::from_secs(30),
        retry_count,
    )
    .await
    .expect("schedule local manifest verification");
    trx.commit()
        .await
        .expect("commit local verification target");
}

async fn archive_only(pool: &PgPool, manifest: &SignedManifest) {
    let key = manifest_object_key(manifest);
    let body = serde_json::to_vec(manifest).expect("serialize archived manifest");
    let mut trx = pool.begin().await.expect("begin manifest archive");
    store_authenticated_manifest(&mut trx, manifest.payload.publisher, &key, &body)
        .await
        .expect("archive peer manifest");
    trx.commit().await.expect("commit peer manifest archive");
}

async fn assert_target(pool: &PgPool, state: &str, outcome: &str, attempt_count: i32) {
    let row = sqlx::query(
        r#"
        SELECT state, latest_outcome, attempt_count
          FROM block_consensus_verification_target
        "#,
    )
    .fetch_one(pool)
    .await
    .expect("load verification target");
    assert_eq!(row.try_get::<String, _>("state").unwrap(), state);
    assert_eq!(row.try_get::<String, _>("latest_outcome").unwrap(), outcome,);
    assert_eq!(
        row.try_get::<i32, _>("attempt_count").unwrap(),
        attempt_count,
    );
}

async fn archive_count(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT COUNT(*) FROM block_consensus_manifest")
        .fetch_one(pool)
        .await
        .expect("count archived manifests")
}

async fn sign_payload(signer: &PrivateKeySigner, payload: ManifestPayload) -> SignedManifest {
    payload.sign(signer).await.expect("sign peer manifest")
}

fn payload(publisher: Address, material: u8) -> ManifestPayload {
    revision_payload(publisher, material, 0, None)
}

fn revision_payload(
    publisher: Address,
    material: u8,
    revision: u64,
    supersedes: Option<ManifestReference>,
) -> ManifestPayload {
    revision_payload_at(
        publisher,
        material,
        revision,
        supersedes,
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
    revision_payload_at(publisher, material, 0, None, block_number, block_hash)
}

fn payload_with_history(
    publisher: Address,
    material: u8,
    block_number: i64,
    block_hash: B256,
    predecessor: &SignedManifest,
) -> ManifestPayload {
    let mut payload = payload_at(publisher, material, block_number, block_hash);
    payload.previous_manifest = Some(manifest_reference(predecessor));
    payload.historical_ranges = vec![HistoricalRange {
        start_block_number: predecessor.payload.publication_block_number,
        end_block_number: predecessor.payload.publication_block_number,
        scale: 0,
        end_block_hash: predecessor.payload.publication_block_hash,
        digest: predecessor
            .payload
            .detailed_range
            .blocks
            .last()
            .expect("predecessor block")
            .block_content_digest,
    }];
    payload
}

fn revision_payload_at(
    publisher: Address,
    material: u8,
    revision: u64,
    supersedes: Option<ManifestReference>,
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
        supersedes,
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
    supersedes: Option<ManifestReference>,
    block_number: i64,
    block_hash: B256,
) -> ManifestPayload {
    let block_number = U256::from(u64::try_from(block_number).expect("positive test block"));
    let parent_block_hash = B256::repeat_byte(0xa9);
    let descriptors = vec![BlockCiphertextDescriptor {
        handle,
        keyset_id,
        gateway_key_id,
        ct64_digest: B256::repeat_byte(material),
        ct128_digest: B256::repeat_byte(material.wrapping_add(1)),
        ct128_format: CiphertextFormat::CompressedOnCpu,
    }];
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
        publisher,
        coprocessor_context_id: TEST_CONTEXT_ID,
        host_chain_id: U256::from(TEST_CHAIN_ID),
        publication_block_number: block_number,
        publication_block_hash: block_hash,
        publication_parent_block_hash: parent_block_hash,
        revision,
        supersedes,
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
        full_consensus_checkpoint: None,
        previous_manifest: None,
    }
}

fn manifest_reference(manifest: &SignedManifest) -> ManifestReference {
    ManifestReference {
        block_number: manifest.payload.publication_block_number,
        block_hash: manifest.payload.publication_block_hash,
        revision: manifest.payload.revision,
        manifest_digest: manifest.digest().expect("digest peer manifest"),
    }
}

fn test_block_hash() -> B256 {
    B256::repeat_byte(0xaa)
}
