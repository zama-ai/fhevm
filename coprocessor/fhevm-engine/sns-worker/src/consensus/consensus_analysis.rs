use std::collections::{HashMap, HashSet};

use alloy_primitives::{Address, B256, U256};
use ciphertext_attestation::{manifest::DetailedRange, CiphertextFormat};

use super::manifest_archive::AuthenticatedManifest;

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
    pub quorum_scope_count: i32,
    pub local_drift_scope_count: i32,
    pub scopes: Vec<ScopeEvaluation>,
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

pub(crate) fn evaluate_quorum(
    manifests: &[AuthenticatedManifest],
    local_publisher: Address,
    required_quorum: usize,
) -> QuorumEvaluation {
    let mut groups = HashMap::<CommitmentScope, HashMap<B256, HashSet<Address>>>::new();
    let mut local = HashMap::<CommitmentScope, B256>::new();
    let mut local_detailed_scope = None;

    for manifest in manifests {
        let payload = &manifest.signed.payload;
        let detailed = &payload.detailed_range;
        if let Some(end) = detailed.blocks.last() {
            let scope = CommitmentScope::Detailed {
                first: detailed.first_block_number,
                last: detailed.last_block_number,
                end_block_hash: end.block_hash,
            };
            groups
                .entry(scope.clone())
                .or_default()
                .entry(detailed.digest)
                .or_default()
                .insert(payload.publisher);
            if payload.publisher == local_publisher {
                local.insert(scope.clone(), detailed.digest);
                local_detailed_scope = Some(scope);
            }
        }
        for range in &payload.historical_ranges {
            let scope = CommitmentScope::Historical {
                first: range.start_block_number,
                last: range.end_block_number,
                scale: range.scale,
                end_block_hash: range.end_block_hash,
            };
            groups
                .entry(scope.clone())
                .or_default()
                .entry(range.digest)
                .or_default()
                .insert(payload.publisher);
            if payload.publisher == local_publisher {
                local.insert(scope, range.digest);
            }
        }
    }

    let mut quorum_scope_count = 0_i32;
    let mut comparable_scope_count = 0_i32;
    let mut local_drift_scope_count = 0_i32;
    let mut detailed_match = None;
    let mut detailed_group_count = None;
    let mut drift_detected = false;
    let mut scopes = Vec::with_capacity(groups.len());
    for (scope, commitments) in groups {
        let mut commitment_groups = commitments
            .into_iter()
            .map(|(digest, publishers)| {
                let mut publishers = publishers.into_iter().collect::<Vec<_>>();
                publishers.sort_unstable();
                CommitmentGroup { digest, publishers }
            })
            .collect::<Vec<_>>();
        commitment_groups.sort_unstable_by_key(|group| group.digest);
        let winners = commitment_groups
            .iter()
            .filter(|group| group.publishers.len() >= required_quorum)
            .map(|group| group.digest)
            .collect::<Vec<_>>();
        let quorum_digest = if winners.len() == 1 {
            winners.first().copied()
        } else {
            None
        };
        let local_digest = local.get(&scope).copied();
        if commitment_groups.len() > 1 {
            drift_detected = true;
            if local_digest.is_some() {
                local_drift_scope_count += 1;
            }
        }
        let local_disagreements = detailed_disagreements(
            manifests,
            local_publisher,
            &scope,
            local_digest,
            &commitment_groups,
        );
        if local_detailed_scope.as_ref() == Some(&scope) {
            detailed_group_count = Some(commitment_groups.len());
        }
        if let Some(winner) = quorum_digest {
            quorum_scope_count += 1;
            if let Some(local_digest) = local_digest {
                comparable_scope_count += 1;
                let matches = local_digest == winner;
                if local_detailed_scope.as_ref() == Some(&scope) {
                    detailed_match = Some(matches);
                }
            }
        }
        scopes.push(ScopeEvaluation {
            scope,
            local_digest,
            groups: commitment_groups,
            quorum_digest,
            local_disagreements,
        });
    }
    scopes.sort_unstable_by(|left, right| left.scope.cmp(&right.scope));

    let outcome = if drift_detected {
        // Drift is an observed content divergence, not the result of losing a
        // quorum vote. Quorum only determines whether one observed value has
        // enough support to be used as an attributed remediation reference.
        VerificationOutcome::Drift
    } else if detailed_match == Some(true) {
        VerificationOutcome::Consensus
    } else if comparable_scope_count > 0 {
        VerificationOutcome::PartialConsensus
    } else if detailed_group_count == Some(1) {
        VerificationOutcome::UnknownEqual
    } else {
        VerificationOutcome::Unknown
    };
    QuorumEvaluation {
        outcome,
        quorum_scope_count,
        local_drift_scope_count,
        scopes,
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

fn detailed_scope(detailed: &DetailedRange) -> Option<CommitmentScope> {
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
                    let handle = local_descriptor.handle;
                    if local_descriptor.keyset_id != observed_descriptor.keyset_id {
                        explanations.push(DriftExplanation::KeysetMismatch {
                            block_number,
                            handle,
                            local: local_descriptor.keyset_id,
                            observed: observed_descriptor.keyset_id,
                        });
                    }
                    if local_descriptor.ct64_digest != observed_descriptor.ct64_digest {
                        explanations.push(DriftExplanation::Ct64DigestMismatch {
                            block_number,
                            handle,
                            local: local_descriptor.ct64_digest,
                            observed: observed_descriptor.ct64_digest,
                        });
                    }
                    if local_descriptor.ct128_digest != observed_descriptor.ct128_digest {
                        explanations.push(DriftExplanation::Ct128DigestMismatch {
                            block_number,
                            handle,
                            local: local_descriptor.ct128_digest,
                            observed: observed_descriptor.ct128_digest,
                        });
                    }
                    if local_descriptor.ct128_format != observed_descriptor.ct128_format {
                        explanations.push(DriftExplanation::Ct128FormatMismatch {
                            block_number,
                            handle,
                            local: local_descriptor.ct128_format,
                            observed: observed_descriptor.ct128_format,
                        });
                    }
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

#[cfg(test)]
#[path = "consensus_analysis_tests.rs"]
mod tests;
