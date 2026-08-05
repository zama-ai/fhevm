use serde_json::{json, Value};
use sqlx::{types::Json, Postgres, Transaction};

use crate::ExecutionError;

use super::consensus_analysis::{CommitmentScope, QuorumEvaluation};

pub(super) struct VerificationAttemptEvidence<'a> {
    pub(super) task_id: i64,
    pub(super) attempt: i32,
    pub(super) required_quorum: usize,
    pub(super) evaluation: &'a QuorumEvaluation,
    pub(super) localization_complete: bool,
    pub(super) drifted_block_count: Option<i64>,
    pub(super) drifted_handle_count: Option<i64>,
}

pub(super) async fn persist_verification_evidence(
    trx: &mut Transaction<'_, Postgres>,
    evidence: VerificationAttemptEvidence<'_>,
) -> Result<(), ExecutionError> {
    let VerificationAttemptEvidence {
        task_id,
        attempt,
        required_quorum,
        evaluation,
        localization_complete,
        drifted_block_count,
        drifted_handle_count,
    } = evidence;
    sqlx::query!(
        r#"
        INSERT INTO block_manifest_verification_attempt (
            task_id,
            attempt,
            outcome,
            local_quorum_status,
            drifted_block_count,
            drifted_handle_count,
            localization_complete
        ) VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (task_id, attempt) DO NOTHING
        "#,
        task_id,
        attempt,
        evaluation.outcome.as_db_str(),
        evaluation.local_quorum_status.as_db_str(),
        drifted_block_count,
        drifted_handle_count,
        localization_complete,
    )
    .execute(trx.as_mut())
    .await?;

    // Persist only divergent block-range comparisons. Consensus rows repeat data
    // already available in the immutable manifests and add no audit value.
    for (drift_index, scope) in evaluation
        .scopes
        .iter()
        .filter(|scope| scope.groups.len() > 1)
        .enumerate()
    {
        let drift_index = i32::try_from(drift_index)
            .map_err(|_| internal("verification drift range count exceeds INTEGER"))?;
        let (range_kind, first, last, scale, end_block_hash) = match &scope.scope {
            CommitmentScope::Detailed {
                first,
                last,
                end_block_hash,
            } => ("detailed", *first, *last, None, *end_block_hash),
            CommitmentScope::Historical {
                first,
                last,
                scale,
                end_block_hash,
            } => (
                "historical",
                *first,
                *last,
                Some(i32::try_from(*scale).map_err(|_| internal("history scale exceeds INTEGER"))?),
                *end_block_hash,
            ),
        };
        let first = i64_from_u256("scope first block number", first)?;
        let last = i64_from_u256("scope last block number", last)?;
        let local_digest = scope.local_digest.map(|digest| digest.as_slice().to_vec());
        let quorum_digest = scope.quorum_digest.map(|digest| digest.as_slice().to_vec());
        let publisher_groups = Json(Value::Array(
            scope
                .groups
                .iter()
                .map(|group| {
                    json!({
                        "digest": group.digest.to_string(),
                        "publishers": group.publishers.iter().map(ToString::to_string).collect::<Vec<_>>(),
                        "has_quorum": group.publishers.len() >= required_quorum,
                    })
                })
                .collect(),
        ));
        sqlx::query!(
            r#"
            INSERT INTO block_manifest_verification_attempt_drift (
                task_id,
                attempt,
                drift_index,
                range_kind,
                first_block_number,
                last_block_number,
                scale,
                end_block_hash,
                local_digest,
                quorum_digest,
                publisher_groups
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (task_id, attempt, drift_index) DO NOTHING
            "#,
            task_id,
            attempt,
            drift_index,
            range_kind,
            first,
            last,
            scale,
            end_block_hash.as_slice(),
            local_digest,
            quorum_digest,
            publisher_groups as Json<Value>,
        )
        .execute(trx.as_mut())
        .await?;
    }
    Ok(())
}

fn i64_from_u256(field: &str, value: alloy_primitives::U256) -> Result<i64, ExecutionError> {
    i64::try_from(value).map_err(|_| internal(format!("{field} exceeds BIGINT")))
}

fn internal(message: impl Into<String>) -> ExecutionError {
    ExecutionError::InternalError(message.into())
}
