use std::collections::{HashMap, HashSet};

use alloy_primitives::{Address, B256, U256};
use ciphertext_attestation::{
    manifest::{BlockCiphertextDescriptor, DetailedRange},
    CiphertextFormat,
};

use crate::consensus::manifest_archive::AuthenticatedManifest;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum VerificationOutcome {
    Unknown,
    UnknownEqual,
    Consensus,
    Drift,
    PartialConsensus,
}

impl VerificationOutcome {
    pub(crate) fn as_db_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::UnknownEqual => "unknown_but_equal",
            Self::Consensus => "consensus",
            Self::Drift => "drift",
            Self::PartialConsensus => "partial_consensus",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct QuorumEvaluation {
    pub outcome: VerificationOutcome,
    pub local_quorum_status: LocalQuorumStatus,
    pub scopes: Vec<ScopeEvaluation>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LocalQuorumStatus {
    MatchesQuorum,
    DiffersFromQuorum,
    Inconclusive,
}

impl LocalQuorumStatus {
    pub(crate) const fn as_db_str(self) -> &'static str {
        match self {
            Self::MatchesQuorum => "matches_quorum",
            Self::DiffersFromQuorum => "differs_from_quorum",
            Self::Inconclusive => "inconclusive",
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum CommitmentScope {
    Detailed {
        first: U256,
        last: U256,
        end_block_hash: B256,
    },
    Historical {
        first: U256,
        last: U256,
        scale: u32,
        end_block_hash: B256,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CommitmentGroup {
    pub digest: B256,
    pub publishers: Vec<Address>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScopeEvaluation {
    pub scope: CommitmentScope,
    pub local_digest: Option<B256>,
    pub groups: Vec<CommitmentGroup>,
    pub quorum_digest: Option<B256>,
    pub local_disagreements: Vec<CommitmentDisagreement>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CommitmentDisagreement {
    pub digest: B256,
    pub publishers: Vec<Address>,
    pub explanations: Vec<DriftExplanation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum DriftExplanation {
    CommitmentDigestMismatch,
    LocalOnlyHandle {
        block_number: U256,
        handle: B256,
    },
    ObservedOnlyHandle {
        block_number: U256,
        handle: B256,
    },
    KeysetMismatch {
        block_number: U256,
        handle: B256,
        local: U256,
        observed: U256,
    },
    Ct64DigestMismatch {
        block_number: U256,
        handle: B256,
        local: B256,
        observed: B256,
    },
    Ct128DigestMismatch {
        block_number: U256,
        handle: B256,
        local: B256,
        observed: B256,
    },
    Ct128FormatMismatch {
        block_number: U256,
        handle: B256,
        local: CiphertextFormat,
        observed: CiphertextFormat,
    },
}

type PublishersByDigest = HashMap<B256, HashSet<Address>>;

#[derive(Default)]
struct CollectedCommitments {
    by_scope: HashMap<CommitmentScope, PublishersByDigest>,
    local_by_scope: HashMap<CommitmentScope, B256>,
    local_detailed_scope: Option<CommitmentScope>,
}

#[derive(Default)]
struct EvaluationSummary {
    local_scope_count: i32,
    local_scope_without_quorum: bool,
    local_quorum_mismatch: bool,
    comparable_scope_count: i32,
    local_detailed_match: Option<bool>,
    local_detailed_group_count: Option<usize>,
    drift_detected: bool,
}

impl EvaluationSummary {
    fn observe(
        &mut self,
        evaluation: &ScopeEvaluation,
        local_detailed_scope: Option<&CommitmentScope>,
    ) {
        let is_local_detailed_scope = local_detailed_scope == Some(&evaluation.scope);
        if evaluation.groups.len() > 1 {
            self.drift_detected = true;
        }
        if is_local_detailed_scope {
            self.local_detailed_group_count = Some(evaluation.groups.len());
        }
        if let Some(local_digest) = evaluation.local_digest {
            self.local_scope_count += 1;
            if let Some(quorum_digest) = evaluation.quorum_digest {
                self.comparable_scope_count += 1;
                self.local_quorum_mismatch |= local_digest != quorum_digest;
                if is_local_detailed_scope {
                    self.local_detailed_match = Some(local_digest == quorum_digest);
                }
            } else {
                self.local_scope_without_quorum = true;
            }
        }
    }

    fn outcome(&self) -> VerificationOutcome {
        if self.drift_detected {
            // Drift is an observed content divergence, not the result of
            // losing a quorum vote. Quorum only selects an attributable
            // remediation reference.
            VerificationOutcome::Drift
        } else if self.local_detailed_match == Some(true) {
            VerificationOutcome::Consensus
        } else if self.comparable_scope_count > 0 {
            VerificationOutcome::PartialConsensus
        } else if self.local_detailed_group_count == Some(1) {
            VerificationOutcome::UnknownEqual
        } else {
            VerificationOutcome::Unknown
        }
    }

    fn local_quorum_status(&self) -> LocalQuorumStatus {
        if self.local_quorum_mismatch {
            LocalQuorumStatus::DiffersFromQuorum
        } else if self.local_scope_count > 0 && !self.local_scope_without_quorum {
            LocalQuorumStatus::MatchesQuorum
        } else {
            LocalQuorumStatus::Inconclusive
        }
    }
}

pub(crate) fn evaluate_quorum(
    manifests: &[AuthenticatedManifest],
    local_publisher: Address,
    required_quorum: usize,
) -> QuorumEvaluation {
    let collected = collect_commitments(manifests, local_publisher);
    let mut summary = EvaluationSummary::default();
    let mut scopes = Vec::with_capacity(collected.by_scope.len());
    for (scope, publishers_by_digest) in collected.by_scope {
        let local_digest = collected.local_by_scope.get(&scope).copied();
        let evaluation = evaluate_scope(
            manifests,
            local_publisher,
            scope,
            publishers_by_digest,
            local_digest,
            required_quorum,
        );
        summary.observe(&evaluation, collected.local_detailed_scope.as_ref());
        scopes.push(evaluation);
    }
    scopes.sort_unstable_by(|left, right| left.scope.cmp(&right.scope));

    QuorumEvaluation {
        outcome: summary.outcome(),
        local_quorum_status: summary.local_quorum_status(),
        scopes,
    }
}

fn collect_commitments(
    manifests: &[AuthenticatedManifest],
    local_publisher: Address,
) -> CollectedCommitments {
    let mut collected = CollectedCommitments::default();
    for manifest in manifests {
        let payload = &manifest.signed.payload;
        if let Some(scope) = detailed_scope(&payload.detailed_range) {
            collect_commitment(
                &mut collected,
                scope.clone(),
                payload.detailed_range.digest,
                payload.publisher,
                local_publisher,
            );
            if payload.publisher == local_publisher {
                collected.local_detailed_scope = Some(scope);
            }
        }
        for range in &payload.historical_ranges {
            collect_commitment(
                &mut collected,
                CommitmentScope::Historical {
                    first: range.start_block_number,
                    last: range.end_block_number,
                    scale: range.scale,
                    end_block_hash: range.end_block_hash,
                },
                range.digest,
                payload.publisher,
                local_publisher,
            );
        }
    }
    collected
}

fn collect_commitment(
    collected: &mut CollectedCommitments,
    scope: CommitmentScope,
    digest: B256,
    publisher: Address,
    local_publisher: Address,
) {
    collected
        .by_scope
        .entry(scope.clone())
        .or_default()
        .entry(digest)
        .or_default()
        .insert(publisher);
    if publisher == local_publisher {
        collected.local_by_scope.insert(scope, digest);
    }
}

fn evaluate_scope(
    manifests: &[AuthenticatedManifest],
    local_publisher: Address,
    scope: CommitmentScope,
    publishers_by_digest: PublishersByDigest,
    local_digest: Option<B256>,
    required_quorum: usize,
) -> ScopeEvaluation {
    let mut groups = publishers_by_digest
        .into_iter()
        .map(|(digest, publishers)| {
            let mut publishers = publishers.into_iter().collect::<Vec<_>>();
            publishers.sort_unstable();
            CommitmentGroup { digest, publishers }
        })
        .collect::<Vec<_>>();
    groups.sort_unstable_by_key(|group| group.digest);
    let mut quorum_digests = groups
        .iter()
        .filter(|group| group.publishers.len() >= required_quorum)
        .map(|group| group.digest);
    let first_quorum_digest = quorum_digests.next();
    let quorum_digest = first_quorum_digest.filter(|_| quorum_digests.next().is_none());
    let local_disagreements =
        detailed_disagreements(manifests, local_publisher, &scope, local_digest, &groups);
    ScopeEvaluation {
        scope,
        local_digest,
        groups,
        quorum_digest,
        local_disagreements,
    }
}

fn detailed_disagreements(
    manifests: &[AuthenticatedManifest],
    local_publisher: Address,
    scope: &CommitmentScope,
    local_digest: Option<B256>,
    groups: &[CommitmentGroup],
) -> Vec<CommitmentDisagreement> {
    let CommitmentScope::Detailed { .. } = scope else {
        return vec![];
    };
    let Some(local_digest) = local_digest else {
        return vec![];
    };
    let Some(local_range) = detailed_range(manifests, local_publisher, scope, local_digest) else {
        return vec![];
    };

    groups
        .iter()
        .filter(|group| group.digest != local_digest)
        .filter_map(|group| {
            let publisher = group.publishers.first().copied()?;
            let observed = detailed_range(manifests, publisher, scope, group.digest)?;
            let mut explanations = compare_detailed_ranges(local_range, observed);
            if explanations.is_empty() {
                explanations.push(DriftExplanation::CommitmentDigestMismatch);
            }
            Some(CommitmentDisagreement {
                digest: group.digest,
                publishers: group.publishers.clone(),
                explanations,
            })
        })
        .collect()
}

fn detailed_range<'a>(
    manifests: &'a [AuthenticatedManifest],
    publisher: Address,
    scope: &CommitmentScope,
    digest: B256,
) -> Option<&'a DetailedRange> {
    manifests.iter().find_map(|manifest| {
        let payload = &manifest.signed.payload;
        let detailed = &payload.detailed_range;
        (payload.publisher == publisher
            && detailed.digest == digest
            && detailed_scope(detailed) == Some(scope.clone()))
        .then_some(detailed)
    })
}

pub(crate) fn detailed_scope(detailed: &DetailedRange) -> Option<CommitmentScope> {
    detailed.blocks.last().map(|end| CommitmentScope::Detailed {
        first: detailed.first_block_number,
        last: detailed.last_block_number,
        end_block_hash: end.block_hash,
    })
}

fn compare_detailed_ranges(
    local: &DetailedRange,
    observed: &DetailedRange,
) -> Vec<DriftExplanation> {
    let mut explanations = Vec::new();
    for (local_block, observed_block) in local.blocks.iter().zip(&observed.blocks) {
        if local_block.block_number != observed_block.block_number
            || local_block.block_hash != observed_block.block_hash
        {
            explanations.push(DriftExplanation::CommitmentDigestMismatch);
            continue;
        }
        let block_number = local_block.block_number;
        let mut local_index = 0;
        let mut observed_index = 0;
        while local_index < local_block.ciphertexts.len()
            || observed_index < observed_block.ciphertexts.len()
        {
            match (
                local_block.ciphertexts.get(local_index),
                observed_block.ciphertexts.get(observed_index),
            ) {
                (Some(local_descriptor), Some(observed_descriptor))
                    if local_descriptor.handle == observed_descriptor.handle =>
                {
                    record_descriptor_differences(
                        &mut explanations,
                        block_number,
                        local_descriptor,
                        observed_descriptor,
                    );
                    local_index += 1;
                    observed_index += 1;
                }
                (Some(local_descriptor), Some(observed_descriptor))
                    if local_descriptor.handle < observed_descriptor.handle =>
                {
                    explanations.push(DriftExplanation::LocalOnlyHandle {
                        block_number,
                        handle: local_descriptor.handle,
                    });
                    local_index += 1;
                }
                (Some(_), Some(observed_descriptor)) => {
                    explanations.push(DriftExplanation::ObservedOnlyHandle {
                        block_number,
                        handle: observed_descriptor.handle,
                    });
                    observed_index += 1;
                }
                (Some(local_descriptor), None) => {
                    explanations.push(DriftExplanation::LocalOnlyHandle {
                        block_number,
                        handle: local_descriptor.handle,
                    });
                    local_index += 1;
                }
                (None, Some(observed_descriptor)) => {
                    explanations.push(DriftExplanation::ObservedOnlyHandle {
                        block_number,
                        handle: observed_descriptor.handle,
                    });
                    observed_index += 1;
                }
                (None, None) => break,
            }
        }
    }
    if local.blocks.len() != observed.blocks.len() {
        explanations.push(DriftExplanation::CommitmentDigestMismatch);
    }
    explanations
}

fn record_descriptor_differences(
    explanations: &mut Vec<DriftExplanation>,
    block_number: U256,
    local: &BlockCiphertextDescriptor,
    observed: &BlockCiphertextDescriptor,
) {
    let handle = local.handle;
    if local.keyset_id != observed.keyset_id {
        explanations.push(DriftExplanation::KeysetMismatch {
            block_number,
            handle,
            local: local.keyset_id,
            observed: observed.keyset_id,
        });
    }
    if local.ct64_digest != observed.ct64_digest {
        explanations.push(DriftExplanation::Ct64DigestMismatch {
            block_number,
            handle,
            local: local.ct64_digest,
            observed: observed.ct64_digest,
        });
    }
    if local.ct128_digest != observed.ct128_digest {
        explanations.push(DriftExplanation::Ct128DigestMismatch {
            block_number,
            handle,
            local: local.ct128_digest,
            observed: observed.ct128_digest,
        });
    }
    if local.ct128_format != observed.ct128_format {
        explanations.push(DriftExplanation::Ct128FormatMismatch {
            block_number,
            handle,
            local: local.ct128_format,
            observed: observed.ct128_format,
        });
    }
}

#[cfg(test)]
#[path = "consensus_analysis_tests.rs"]
mod tests;
