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
    manifest_range_start: i64,
    manifest_range_digest: B256,
    manifest_digest: B256,
) -> Result<(), ExecutionError> {
    let result = sqlx::query!(
        r#"
        UPDATE block_manifest_state
           SET manifest_range_start = $3,
               manifest_range_digest = $4,
               manifest_digest = $5,
               manifest_publisher = $7,
               manifest_published = TRUE,
               manifest_published_at = NOW(),
               publication_error_count = 0,
               publication_last_error = NULL,
               publication_next_retry_at = NULL,
               updated_at = NOW()
         WHERE host_chain_id = $1
           AND block_hash = $2
           AND generation = $8
           AND manifest_revision = $6
           AND manifest_digest IS NULL
           AND manifest_published = FALSE
        "#,
        target.host_chain_id,
        &target.block_hash,
        manifest_range_start,
        manifest_range_digest.as_slice(),
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
