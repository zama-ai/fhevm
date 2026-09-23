//! Coverage is a contiguous, authenticated suffix ending at the local detailed range.
//! Older missing evidence must not invalidate that suffix or be silently included in it.
use alloy_primitives::U256;

use super::consensus_analysis::{CommitmentScope, ScopeEvaluation};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct QuorumCoverage {
    pub quorum_from_block: Option<U256>,
    pub quorum_through_block: Option<U256>,
    /// This prefix is not fully verified; it can contain isolated matching scopes.
    pub unverified_prefix_from_block: Option<U256>,
    pub unverified_prefix_through_block: Option<U256>,
}

fn bounds(scope: &CommitmentScope) -> (U256, U256) {
    match scope {
        CommitmentScope::Detailed { first, last, .. }
        | CommitmentScope::Historical { first, last, .. } => (*first, *last),
    }
}

pub(crate) fn quorum_coverage(
    scopes: &[ScopeEvaluation],
    detailed_scope: Option<&CommitmentScope>,
) -> QuorumCoverage {
    let mut local = scopes
        .iter()
        .filter(|s| s.local_digest.is_some())
        .collect::<Vec<_>>();
    local.sort_unstable_by_key(|s| bounds(&s.scope));
    let Some(first) = local.first().map(|s| bounds(&s.scope).0) else {
        return QuorumCoverage::default();
    };
    let Some(detailed) = local.iter().find(|s| Some(&s.scope) == detailed_scope) else {
        return QuorumCoverage::default();
    };
    let (mut start, end) = bounds(&detailed.scope);
    if detailed.local_digest != detailed.quorum_digest {
        return QuorumCoverage {
            unverified_prefix_from_block: Some(first),
            unverified_prefix_through_block: Some(end),
            ..QuorumCoverage::default()
        };
    }
    // Compare whole committed scopes only. Never split a digest at a peer's start
    // height, bridge a gap, or carry coverage across a mismatch/unknown scope.
    for scope in local.iter().rev() {
        if Some(&scope.scope) == detailed_scope {
            continue;
        }
        let (range_start, range_end) = bounds(&scope.scope);
        if range_end.checked_add(U256::ONE) != Some(start)
            || scope.local_digest != scope.quorum_digest
        {
            break;
        }
        start = range_start;
    }
    QuorumCoverage {
        quorum_from_block: Some(start),
        quorum_through_block: Some(end),
        unverified_prefix_from_block: (first < start).then_some(first),
        unverified_prefix_through_block: (first < start).then(|| start - U256::ONE),
    }
}
