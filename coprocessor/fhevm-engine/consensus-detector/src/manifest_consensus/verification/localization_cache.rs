//! Reuse exact completed historical comparisons from the durable audit evidence.
//! A height alone cannot identify either a lineage or the commitments localized.

use std::collections::HashSet;

use sqlx::{Postgres, Transaction};

use crate::manifest_consensus::ExecutionError;

use super::consensus_analysis::{CommitmentScope, QuorumEvaluation, VerificationOutcome};
use super::verification_evidence::publisher_groups;
use super::verification_queue::VerificationClaim;
use super::verification_utils::internal;

pub(super) fn localization_is_cacheable(
    evaluation: &QuorumEvaluation,
    localization_complete: bool,
) -> bool {
    evaluation.outcome == VerificationOutcome::Drift
        && localization_complete
        && evaluation
            .scopes
            .iter()
            .any(|scope| scope.local_digest.is_some())
        && evaluation
            .scopes
            .iter()
            .filter(|scope| scope.local_digest.is_some())
            .all(|scope| scope.quorum_digest.is_some())
}

pub(super) async fn load_completed_history(
    trx: &mut Transaction<'_, Postgres>,
    claim: &VerificationClaim,
    evaluation: &QuorumEvaluation,
) -> Result<HashSet<CommitmentScope>, ExecutionError> {
    let mut completed = HashSet::new();
    // Missing quorum must not turn any historical work into a cache hit.
    if !localization_is_cacheable(evaluation, true) {
        return Ok(completed);
    }
    let context = claim.scope.coprocessor_context_id.to_be_bytes::<32>();
    let required_quorum = i32::try_from(claim.required_quorum)
        .map_err(|_| internal("required quorum exceeds INTEGER"))?;
    for scope in &evaluation.scopes {
        let CommitmentScope::Historical {
            first,
            last,
            scale,
            end_block_hash,
        } = &scope.scope
        else {
            continue;
        };
        let Some(local_digest) = scope.local_digest else {
            continue;
        };
        if scope.groups.len() <= 1 {
            continue;
        }
        let first = i64::try_from(*first).map_err(|_| internal("range start exceeds BIGINT"))?;
        let last = i64::try_from(*last).map_err(|_| internal("range end exceeds BIGINT"))?;
        let scale = i32::try_from(*scale).map_err(|_| internal("range scale exceeds INTEGER"))?;
        let groups = publisher_groups(scope, claim.required_quorum);
        let cached = sqlx::query_scalar!(
            r#"
            SELECT EXISTS (
                SELECT 1
                  FROM block_manifest_verification_attempt_drift drift
                  JOIN block_manifest_verification_attempt attempt
                    USING (generation, task_id, attempt)
                  JOIN block_manifest_verification_task task
                    ON task.id = drift.task_id AND task.generation = drift.generation
                  JOIN block_manifest manifest
                    ON manifest.id = task.local_manifest_id AND manifest.generation = task.generation
                 WHERE drift.generation = $1
                   AND drift.range_kind = 'historical'
                   AND drift.first_block_number = $2
                   AND drift.last_block_number = $3
                   AND drift.scale = $4
                   AND drift.end_block_hash = $5
                   AND drift.local_digest = $6
                   AND drift.publisher_groups = $7
                   AND attempt.localization_cacheable
                   AND task.required_quorum = $8
                   AND manifest.publisher = $9
                   AND manifest.version = $10
                   AND manifest.coprocessor_context_id = $11
                   AND manifest.host_chain_id = $12
            ) AS "cached!"
            "#,
            claim.scope.generation,
            first,
            last,
            scale,
            end_block_hash.as_slice(),
            local_digest.as_slice(),
            groups as sqlx::types::Json<serde_json::Value>,
            required_quorum,
            claim.scope.local_publisher.as_slice(),
            i16::from(u8::from(claim.scope.version)),
            context.as_slice(),
            claim.scope.host_chain_id,
        )
        .fetch_one(trx.as_mut())
        .await?;
        if cached {
            completed.insert(scope.scope.clone());
        }
    }
    Ok(completed)
}

/// Filter only this evaluation. Nested comparisons can have different evidence
/// at the same range, so the cache-hit set must not leak into recursive descent.
pub(super) fn without_completed_history(
    evaluation: &QuorumEvaluation,
    completed: &HashSet<CommitmentScope>,
) -> QuorumEvaluation {
    let mut pending = evaluation.clone();
    pending
        .scopes
        .retain(|scope| !completed.contains(&scope.scope));
    pending
}
