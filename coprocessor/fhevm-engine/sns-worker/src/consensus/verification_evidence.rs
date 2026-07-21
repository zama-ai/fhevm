use sqlx::{Postgres, Transaction};

use crate::ExecutionError;

use super::consensus_analysis::{CommitmentScope, QuorumEvaluation};

pub(super) async fn persist_verification_evidence(
    trx: &mut Transaction<'_, Postgres>,
    target_id: i64,
    attempt: i32,
    required_quorum: usize,
    evaluation: &QuorumEvaluation,
    localization_complete: bool,
) -> Result<(), ExecutionError> {
    sqlx::query!(
        r#"
        INSERT INTO block_consensus_verification_attempt (
            target_id,
            attempt,
            outcome,
            quorum_scope_count,
            local_drift_scope_count,
            localization_complete
        ) VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (target_id, attempt) DO NOTHING
        "#,
        target_id,
        attempt,
        evaluation.outcome.as_db_str(),
        evaluation.quorum_scope_count,
        evaluation.local_drift_scope_count,
        localization_complete,
    )
    .execute(trx.as_mut())
    .await?;

    for (scope_index, scope) in evaluation.scopes.iter().enumerate() {
        let scope_index = i32::try_from(scope_index)
            .map_err(|_| internal("verification scope count exceeds INTEGER"))?;
        let (scope_kind, first, last, scale, end_block_hash) = match &scope.scope {
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
        sqlx::query!(
            r#"
            INSERT INTO block_consensus_verification_scope (
                target_id,
                attempt,
                scope_index,
                scope_kind,
                first_block_number,
                last_block_number,
                scale,
                end_block_hash,
                local_digest,
                quorum_digest
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (target_id, attempt, scope_index) DO NOTHING
            "#,
            target_id,
            attempt,
            scope_index,
            scope_kind,
            first,
            last,
            scale,
            end_block_hash.as_slice(),
            local_digest,
            quorum_digest,
        )
        .execute(trx.as_mut())
        .await?;

        for group in &scope.groups {
            let group_has_quorum = group.publishers.len() >= required_quorum;
            for publisher in &group.publishers {
                sqlx::query!(
                    r#"
                    INSERT INTO block_consensus_verification_scope_member (
                        target_id,
                        attempt,
                        scope_index,
                        digest,
                        publisher,
                        group_has_quorum
                    ) VALUES ($1, $2, $3, $4, $5, $6)
                    ON CONFLICT DO NOTHING
                    "#,
                    target_id,
                    attempt,
                    scope_index,
                    group.digest.as_slice(),
                    publisher.as_slice(),
                    group_has_quorum,
                )
                .execute(trx.as_mut())
                .await?;
            }
        }
    }
    Ok(())
}

fn i64_from_u256(field: &str, value: alloy_primitives::U256) -> Result<i64, ExecutionError> {
    i64::try_from(value).map_err(|_| internal(format!("{field} exceeds BIGINT")))
}

fn internal(message: impl Into<String>) -> ExecutionError {
    ExecutionError::InternalError(message.into())
}
