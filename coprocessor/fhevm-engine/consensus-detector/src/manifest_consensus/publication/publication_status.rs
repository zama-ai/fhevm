use alloy_primitives::{Address, B256};
use sqlx::{PgPool, Postgres, Transaction};

use crate::manifest_consensus::{
    publication::{block_discovery::PendingBlock, manifest_builder::internal},
    ExecutionError,
};

pub(crate) async fn mark_manifest_published(
    trx: &mut Transaction<'_, Postgres>,
    target: &PendingBlock,
    publisher: Address,
    manifest_digest: B256,
) -> Result<(), ExecutionError> {
    let result = sqlx::query!(
        r#"
        UPDATE block_manifest_state
           SET manifest_digest = $3,
               manifest_publisher = $5,
               manifest_published = TRUE,
               manifest_published_at = NOW(),
               publication_error_count = 0,
               publication_last_error = NULL,
               publication_next_retry_at = NULL,
               updated_at = NOW()
         WHERE host_chain_id = $1
           AND block_hash = $2
           AND generation = $6
           AND manifest_revision = $4
           AND manifest_digest IS NULL
           AND manifest_published = FALSE
        "#,
        target.host_chain_id,
        &target.block_hash,
        manifest_digest.as_slice(),
        target.manifest_revision,
        publisher.as_slice(),
        target.generation,
    )
    .execute(trx.as_mut())
    .await?;
    if result.rows_affected() != 1 {
        return Err(internal(format!(
            "manifest publication updated no row for chain {} block {}",
            target.host_chain_id, target.block_number,
        )));
    }
    Ok(())
}

/// Records a failed publication for the current revision, unless another
/// publisher has already advanced that row. Once the finite attempt limit is
/// reached, the row is left exhausted instead of scheduling another retry.
pub(crate) async fn record_manifest_publication_error(
    pool: &PgPool,
    target: &PendingBlock,
    error: &str,
    max_attempts: i64,
    retry_delay_micros: i64,
) -> Result<(), ExecutionError> {
    sqlx::query!(
        r#"
        UPDATE block_manifest_state
           SET publication_error_count = publication_error_count + 1,
               publication_last_error = $4,
               publication_next_retry_at = CASE
                   WHEN publication_error_count + 1 < $5
                   THEN NOW() + $6::BIGINT * INTERVAL '1 microsecond'
                   ELSE NULL
               END,
               updated_at = NOW()
         WHERE host_chain_id = $1
           AND block_hash = $2
           AND generation = $7
           AND manifest_revision = $3
           AND manifest_required
           AND NOT manifest_published
           AND block_content_digest IS NOT NULL
        "#,
        target.host_chain_id,
        &target.block_hash,
        target.manifest_revision,
        error,
        max_attempts,
        retry_delay_micros,
        target.generation,
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Marks a sealed cadence identity exhausted without leaving a peelable skip.
///
/// `publication_error_count` is raised to at least `max_attempts + 1` so a later
/// successful descendant does not grant the extra skip attempt used for
/// transient S3 exhaustion.
pub(crate) async fn exhaust_manifest_publication_error(
    pool: &PgPool,
    target: &PendingBlock,
    error: &str,
    max_attempts: i64,
) -> Result<(), ExecutionError> {
    let exhausted_count = max_attempts.saturating_add(1);
    sqlx::query!(
        r#"
        UPDATE block_manifest_state
           SET publication_error_count = GREATEST(publication_error_count + 1, $4),
               publication_last_error = $3,
               publication_next_retry_at = NULL,
               updated_at = NOW()
         WHERE host_chain_id = $1
           AND block_hash = $2
           AND generation = $5
           AND manifest_revision = $6
           AND manifest_required
           AND NOT manifest_published
           AND block_content_digest IS NOT NULL
        "#,
        target.host_chain_id,
        &target.block_hash,
        error,
        exhausted_count,
        target.generation,
        target.manifest_revision,
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// After a cadence object is published, allow one extra attempt (`max_attempts + 1`)
/// of the nearest skipped ancestor on this lineage. If that retry succeeds, its
/// own publication peels the next older skip. A skip that already used the extra
/// attempt (`publication_error_count > max_attempts`) is not reopened.
pub(crate) async fn retry_skipped_predecessor_once(
    trx: &mut Transaction<'_, Postgres>,
    published: &PendingBlock,
    max_attempts: i64,
) -> Result<(), ExecutionError> {
    sqlx::query!(
        r#"
        WITH RECURSIVE lineage AS (
            SELECT generation,
                   host_chain_id,
                   block_hash,
                   parent_block_hash,
                   block_number
              FROM block_manifest_state
             WHERE generation = $1
               AND host_chain_id = $2
               AND block_hash = $3
            UNION ALL
            SELECT parent.generation,
                   parent.host_chain_id,
                   parent.block_hash,
                   parent.parent_block_hash,
                   parent.block_number
              FROM block_manifest_state parent
              JOIN lineage child
                ON parent.generation = child.generation
               AND parent.host_chain_id = child.host_chain_id
               AND parent.block_hash = child.parent_block_hash
             WHERE parent.block_number < child.block_number
        ),
        nearest AS (
            SELECT skipped.generation,
                   skipped.host_chain_id,
                   skipped.block_hash
              FROM lineage
              JOIN block_manifest_state skipped
                ON skipped.generation = lineage.generation
               AND skipped.host_chain_id = lineage.host_chain_id
               AND skipped.block_hash = lineage.block_hash
             WHERE skipped.block_number < $4
               AND skipped.manifest_required
               AND NOT skipped.manifest_published
               AND skipped.publication_error_count > 0
               AND skipped.publication_error_count <= $5
               AND skipped.publication_next_retry_at IS NULL
               AND skipped.block_content_digest IS NOT NULL
             ORDER BY skipped.block_number DESC
             LIMIT 1
        )
        UPDATE block_manifest_state skipped
           SET publication_next_retry_at = NOW(),
               updated_at = NOW()
          FROM nearest
         WHERE skipped.generation = nearest.generation
           AND skipped.host_chain_id = nearest.host_chain_id
           AND skipped.block_hash = nearest.block_hash
        "#,
        published.generation,
        published.host_chain_id,
        &published.block_hash,
        published.block_number,
        max_attempts,
    )
    .execute(trx.as_mut())
    .await?;
    Ok(())
}
