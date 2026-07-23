use super::*;
use ciphertext_attestation::{
    manifest::{
        block_content_digest, detailed_range_digest, BlockCiphertextDescriptor, DetailedRange,
        HistoricalRange, ManifestBlockEntry, ManifestPayload, ManifestVersion, SignedManifest,
    },
    CiphertextFormat,
};

const CONTEXT: U256 = U256::ONE;
const CHAIN_ID: u64 = 7;

#[test]
fn any_visible_detailed_difference_is_drift_even_when_local_has_quorum() {
    let local = Address::repeat_byte(1);
    let manifests = vec![
        manifest(local, 40, 42, 0xaa, 0x11, vec![]),
        manifest(Address::repeat_byte(2), 40, 42, 0xaa, 0x11, vec![]),
        manifest(Address::repeat_byte(3), 40, 42, 0xaa, 0x22, vec![]),
    ];

    let evaluation = evaluate_quorum(&manifests, local, 2);
    assert_eq!(evaluation.outcome, VerificationOutcome::Drift);
    assert_eq!(evaluation.quorum_scope_count, 1);
    assert_eq!(evaluation.local_drift_scope_count, 1);
}

#[test]
fn concordant_visible_results_below_quorum_are_insufficient_but_not_drift() {
    let local = Address::repeat_byte(1);
    let manifests = vec![
        manifest(local, 40, 42, 0xaa, 0x11, vec![]),
        manifest(Address::repeat_byte(2), 40, 42, 0xaa, 0x11, vec![]),
    ];

    let evaluation = evaluate_quorum(&manifests, local, 3);
    assert_eq!(evaluation.outcome, VerificationOutcome::UnknownEqual);
    assert_eq!(evaluation.quorum_scope_count, 0);
    assert_eq!(evaluation.local_drift_scope_count, 0);
}

#[test]
fn historical_range_quorum_classifies_local_drift() {
    let local = Address::repeat_byte(1);
    let shared_range = |digest| historical_range(0, 31, 5, 0x91, digest);
    let manifests = vec![
        manifest(local, 40, 42, 0xaa, 0x11, vec![shared_range(0x31)]),
        manifest(
            Address::repeat_byte(2),
            41,
            42,
            0xaa,
            0x12,
            vec![shared_range(0x32)],
        ),
        manifest(
            Address::repeat_byte(3),
            42,
            42,
            0xaa,
            0x13,
            vec![shared_range(0x32)],
        ),
    ];

    let evaluation = evaluate_quorum(&manifests, local, 2);
    assert_eq!(evaluation.outcome, VerificationOutcome::Drift);
    assert_eq!(evaluation.quorum_scope_count, 1);
    assert_eq!(evaluation.local_drift_scope_count, 1);
}

#[test]
fn detailed_consensus_does_not_hide_historical_local_drift() {
    let local = Address::repeat_byte(1);
    let shared_range = |digest| historical_range(0, 31, 5, 0x91, digest);
    let manifests = vec![
        manifest(local, 40, 42, 0xaa, 0x11, vec![shared_range(0x31)]),
        manifest(
            Address::repeat_byte(2),
            40,
            42,
            0xaa,
            0x11,
            vec![shared_range(0x32)],
        ),
        manifest(
            Address::repeat_byte(3),
            40,
            42,
            0xaa,
            0x11,
            vec![shared_range(0x32)],
        ),
    ];

    let evaluation = evaluate_quorum(&manifests, local, 2);
    assert_eq!(evaluation.outcome, VerificationOutcome::Drift);
    assert_eq!(evaluation.quorum_scope_count, 2);
    assert_eq!(evaluation.local_drift_scope_count, 1);

    let detailed = evaluation
        .scopes
        .iter()
        .find(|scope| matches!(scope.scope, CommitmentScope::Detailed { .. }))
        .expect("detailed scope");
    assert_eq!(detailed.local_digest, detailed.quorum_digest);

    let historical = evaluation
        .scopes
        .iter()
        .find(|scope| matches!(scope.scope, CommitmentScope::Historical { .. }))
        .expect("historical scope");
    assert_ne!(historical.local_digest, historical.quorum_digest);
}

#[test]
fn reorged_historical_ranges_never_vote_in_the_same_scope() {
    let local = Address::repeat_byte(1);
    let canonical = historical_range(0, 31, 5, 0x91, 0x31);
    let reorged = historical_range(0, 31, 5, 0x92, 0x32);
    let manifests = vec![
        manifest(local, 40, 42, 0xaa, 0x11, vec![canonical.clone()]),
        manifest(Address::repeat_byte(2), 41, 42, 0xaa, 0x12, vec![canonical]),
        manifest(Address::repeat_byte(3), 42, 42, 0xaa, 0x13, vec![reorged]),
    ];

    let evaluation = evaluate_quorum(&manifests, local, 2);
    assert_eq!(evaluation.outcome, VerificationOutcome::PartialConsensus);
    assert_eq!(evaluation.quorum_scope_count, 1);
    assert_eq!(evaluation.local_drift_scope_count, 0);
}

#[test]
fn five_copro_consensus_summary_covers_every_local_origin_and_drift_population() {
    struct Scenario {
        name: &'static str,
        materials: [(u8, u8); 5],
        expected_outcomes: [VerificationOutcome; 5],
        quorum_group_origin: Option<usize>,
        expected_group_sizes: &'static [usize],
    }

    let scenarios = [
        Scenario {
            name: "one drifter",
            materials: [(1, 0x11), (1, 0x11), (1, 0x11), (1, 0x11), (2, 0x11)],
            expected_outcomes: [VerificationOutcome::Drift; 5],
            quorum_group_origin: Some(0),
            expected_group_sizes: &[1, 4],
        },
        Scenario {
            name: "two matching drifters",
            materials: [(1, 0x12), (1, 0x12), (1, 0x12), (2, 0x12), (2, 0x12)],
            expected_outcomes: [VerificationOutcome::Drift; 5],
            quorum_group_origin: Some(0),
            expected_group_sizes: &[2, 3],
        },
        Scenario {
            name: "three drifters split as pair and singleton",
            materials: [(1, 0x13), (1, 0x13), (2, 0x23), (2, 0x23), (3, 0x33)],
            expected_outcomes: [VerificationOutcome::Drift; 5],
            quorum_group_origin: None,
            expected_group_sizes: &[1, 2, 2],
        },
        Scenario {
            name: "five drifters split as two pairs and singleton",
            materials: [(4, 0x14), (4, 0x14), (5, 0x24), (5, 0x24), (6, 0x34)],
            expected_outcomes: [VerificationOutcome::Drift; 5],
            quorum_group_origin: None,
            expected_group_sizes: &[1, 2, 2],
        },
        Scenario {
            name: "all five results differ",
            materials: [(1, 0x15), (2, 0x25), (3, 0x35), (4, 0x45), (5, 0x55)],
            expected_outcomes: [VerificationOutcome::Drift; 5],
            quorum_group_origin: None,
            expected_group_sizes: &[1, 1, 1, 1, 1],
        },
    ];

    let publishers = std::array::from_fn::<_, 5, _>(|index| {
        Address::repeat_byte(u8::try_from(index + 1).unwrap())
    });
    for scenario in scenarios {
        let manifests = publishers
            .iter()
            .zip(scenario.materials)
            .map(|(publisher, (keyset_id, ct64_digest))| {
                material_manifest(*publisher, keyset_id, ct64_digest)
            })
            .collect::<Vec<_>>();

        for (origin_index, (origin, expected_outcome)) in publishers
            .iter()
            .zip(scenario.expected_outcomes)
            .enumerate()
        {
            let evaluation = evaluate_quorum(&manifests, *origin, 3);
            assert_eq!(
                evaluation.outcome, expected_outcome,
                "{} from local origin {origin}",
                scenario.name,
            );
            assert_eq!(evaluation.scopes.len(), 1, "{}", scenario.name);
            let summary = &evaluation.scopes[0];
            assert_eq!(
                summary.local_digest,
                Some(manifests[origin_index].signed.payload.detailed_range.digest),
                "{} from local origin {origin}",
                scenario.name,
            );
            assert_eq!(
                summary.quorum_digest,
                scenario.quorum_group_origin.map(|index| manifests[index]
                    .signed
                    .payload
                    .detailed_range
                    .digest),
                "{} from local origin {origin}",
                scenario.name,
            );
            let mut group_sizes = summary
                .groups
                .iter()
                .map(|group| group.publishers.len())
                .collect::<Vec<_>>();
            group_sizes.sort_unstable();
            assert_eq!(
                group_sizes, scenario.expected_group_sizes,
                "{} from local origin {origin}",
                scenario.name,
            );
            assert!(
                !summary.local_disagreements.is_empty(),
                "{} from local origin {origin} must retain diagnostic disagreement groups",
                scenario.name,
            );
            assert!(
                summary.local_disagreements.iter().any(|disagreement| {
                    disagreement.explanations.iter().any(|explanation| {
                        matches!(explanation, DriftExplanation::KeysetMismatch { .. })
                    })
                }),
                "{} from local origin {origin} must identify the keyset mismatch",
                scenario.name,
            );
        }
    }
}

fn material_manifest(publisher: Address, keyset_id: u8, ct64_digest: u8) -> AuthenticatedManifest {
    let block_number = U256::from(42);
    let block_hash = B256::repeat_byte(0xaa);
    let parent_block_hash = B256::repeat_byte(0xa9);
    let descriptors = vec![BlockCiphertextDescriptor {
        handle: B256::repeat_byte(1),
        keyset_id: U256::from(keyset_id),
        gateway_key_id: None,
        ct64_digest: B256::repeat_byte(ct64_digest),
        ct128_digest: B256::repeat_byte(ct64_digest.wrapping_add(1)),
        ct128_format: CiphertextFormat::CompressedOnCpu,
    }];
    let block_digest = block_content_digest(
        ManifestVersion::V1,
        CONTEXT,
        U256::from(CHAIN_ID),
        block_number,
        block_hash,
        &descriptors,
    )
    .unwrap();
    let detailed_digest = detailed_range_digest(
        ManifestVersion::V1,
        CONTEXT,
        U256::from(CHAIN_ID),
        block_number,
        block_number,
        &[block_digest],
    );
    let payload = ManifestPayload {
        version: ManifestVersion::V1,
        publisher,
        coprocessor_context_id: CONTEXT,
        host_chain_id: U256::from(CHAIN_ID),
        publication_block_number: block_number,
        publication_block_hash: block_hash,
        publication_parent_block_hash: parent_block_hash,
        revision: 0,
        supersedes: None,
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
    };
    AuthenticatedManifest {
        signed: SignedManifest {
            payload,
            signature: vec![],
        },
        digest: B256::ZERO,
    }
}

fn manifest(
    publisher: Address,
    detailed_start: u64,
    detailed_end: u64,
    detailed_end_hash: u8,
    detailed_digest: u8,
    historical_ranges: Vec<HistoricalRange>,
) -> AuthenticatedManifest {
    let end_hash = B256::repeat_byte(detailed_end_hash);
    let parent_hash = B256::repeat_byte(detailed_end_hash.wrapping_sub(1));
    let payload = ManifestPayload {
        version: ManifestVersion::V1,
        publisher,
        coprocessor_context_id: CONTEXT,
        host_chain_id: U256::from(CHAIN_ID),
        publication_block_number: U256::from(detailed_end),
        publication_block_hash: end_hash,
        publication_parent_block_hash: parent_hash,
        revision: 0,
        supersedes: None,
        detailed_range: DetailedRange {
            first_block_number: U256::from(detailed_start),
            last_block_number: U256::from(detailed_end),
            digest: B256::repeat_byte(detailed_digest),
            blocks: vec![ManifestBlockEntry {
                block_number: U256::from(detailed_end),
                block_hash: end_hash,
                parent_block_hash: parent_hash,
                block_content_digest: B256::repeat_byte(detailed_digest),
                ciphertexts: vec![],
            }],
        },
        historical_ranges,
        full_consensus_checkpoint: None,
        previous_manifest: None,
    };
    AuthenticatedManifest {
        signed: SignedManifest {
            payload,
            signature: vec![],
        },
        digest: B256::ZERO,
    }
}

fn historical_range(start: u64, end: u64, scale: u32, end_hash: u8, digest: u8) -> HistoricalRange {
    HistoricalRange {
        start_block_number: U256::from(start),
        end_block_number: U256::from(end),
        scale,
        end_block_hash: B256::repeat_byte(end_hash),
        digest: B256::repeat_byte(digest),
    }
}
