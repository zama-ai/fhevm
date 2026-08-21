use std::time::Duration;

use sqlx::{PgPool, Postgres, Transaction};

use crate::manifest_consensus::ExecutionError;

/// A generation-scoped host block tracked by the manifest publication state
/// machine.
#[derive(Clone, Debug)]
pub(crate) struct PendingBlock {
    pub generation: i64,
    pub host_chain_id: i64,
    pub block_number: i64,
    pub block_hash: Vec<u8>,
    pub parent_block_hash: Vec<u8>,
    pub publication_cadence: i64,
    pub block_content_digest: Option<Vec<u8>>,
    pub block_handle_count: Option<i64>,
    pub manifest_revision: i64,
    pub manifest_publisher: Option<Vec<u8>>,
    pub manifest_digest: Option<Vec<u8>>,
    pub manifest_published: bool,
}

/// Stable pagination position while a publisher scans competing block
/// lineages without repeatedly selecting the same blocked candidate.
#[derive(Debug)]
pub(crate) struct ManifestProgressCursor {
    block_number: i64,
    block_hash: Vec<u8>,
}

impl ManifestProgressCursor {
    pub(crate) fn start() -> Self {
        Self {
            block_number: -1,
            block_hash: Vec::new(),
        }
    }

    pub(crate) fn advance_to(&mut self, block: &PendingBlock) {
        self.block_number = block.block_number;
        self.block_hash.clone_from(&block.block_hash);
    }
}

// This query protects lineage ordering only; it must never monopolize a
// database connection while a large recovery backlog is present.
const MANIFEST_WORK_SELECTION_TIMEOUT: Duration = Duration::from_secs(5);
const MANIFEST_DISCOVERY_BLOCK_OVERLAP: i64 = 5;

/// Seeds manifest processing from the stack-local host-chain view and the
/// immutable handle-to-producer-block associations written by its listener.
///
/// Generation ownership is established by stack routing, not by a block-number
/// predicate on this association. Blue reads the public table; Green reads the
/// GCS table through its pool's `search_path`. Green remains parked until
/// pre-`start_block` work has been pruned, and Blue follows the same rule until
/// its stack-version gate retires it at cutover.
///
/// `block_manifest_state` is also the durable discovery frontier. The first
/// generation starts at the latest block already known on each chain because
/// its historical boundary is intentionally arbitrary. Upgrade generations
/// start at their configured per-chain `start_block`. Once a frontier exists,
/// each pass revisits a small block window to tolerate recently-arrived rows,
/// without ever crossing an upgrade generation's start boundary.
#[cfg(test)]
pub(crate) async fn discover_completed_sns_blocks(pool: &PgPool) -> Result<u64, ExecutionError> {
    let generation = crate::manifest_consensus::storage::active::load_generation(pool).await?;
    discover_completed_sns_blocks_for_generation(pool, generation).await
}

pub(crate) async fn discover_completed_sns_blocks_for_generation(
    pool: &PgPool,
    generation: i64,
) -> Result<u64, ExecutionError> {
    let inserted = sqlx::query_scalar!(
        r#"
        WITH existing_frontier AS MATERIALIZED (
            SELECT host_chain_id,
                   MIN(block_number) AS first_block,
                   MAX(block_number) AS block_number
              FROM block_manifest_state
             WHERE generation = $1
             GROUP BY host_chain_id
        ), latest_valid AS MATERIALIZED (
            SELECT chain_id AS host_chain_id,
                   MAX(block_number) AS block_number
              FROM host_chain_blocks_valid
             WHERE block_status <> 'orphaned'
               AND OCTET_LENGTH(parent_hash) = 32
             GROUP BY chain_id
        ), discovery_bounds AS MATERIALIZED (
            SELECT latest.host_chain_id,
                   CASE
                       WHEN frontier.block_number IS NOT NULL THEN GREATEST(
                           frontier.block_number - $2,
                           COALESCE(gen_window.start_block, frontier.first_block)
                       )
                       WHEN $1 = 0 THEN latest.block_number
                       ELSE gen_window.start_block
                   END AS first_block,
                   frontier.block_number IS NULL AS needs_anchor,
                   CASE latest.host_chain_id
                       WHEN 1 THEN 5
                       WHEN 11155111 THEN 5
                       WHEN 137 THEN 30
                       WHEN 80002 THEN 30
                       WHEN 8453 THEN 30
                       WHEN 84532 THEN 30
                       ELSE 30
                   END AS publication_cadence
              FROM latest_valid latest
              LEFT JOIN existing_frontier frontier
                ON frontier.host_chain_id = latest.host_chain_id
              LEFT JOIN generation_block_window gen_window
                ON gen_window.generation = $1
               AND gen_window.host_chain_id = latest.host_chain_id
             WHERE $1 = 0 OR gen_window.host_chain_id IS NOT NULL
        ), anchor AS MATERIALIZED (
            SELECT $1::BIGINT AS generation,
                   bounds.host_chain_id,
                   host.block_number,
                   host.block_hash,
                   host.parent_hash,
                   bounds.publication_cadence
              FROM discovery_bounds bounds
              JOIN host_chain_blocks_valid host
                ON host.chain_id = bounds.host_chain_id
               AND host.block_number = bounds.first_block
             WHERE bounds.needs_anchor
               AND host.block_status <> 'orphaned'
               AND OCTET_LENGTH(host.parent_hash) = 32
        ), source AS (
            SELECT generation,
                   host_chain_id,
                   block_number,
                   block_hash,
                   parent_hash,
                   publication_cadence
              FROM anchor

            UNION

            SELECT $1::BIGINT AS generation,
                   producer.host_chain_id,
                   host.block_number,
                   host.block_hash,
                   host.parent_hash,
                   bounds.publication_cadence
              FROM discovery_bounds bounds
              JOIN handle_producer_block producer
                ON producer.host_chain_id = bounds.host_chain_id
               AND producer.producer_block_number >= bounds.first_block
              JOIN host_chain_blocks_valid host
                ON host.chain_id = producer.host_chain_id
               AND host.block_number = producer.producer_block_number
               AND host.block_hash = producer.producer_block_hash
             WHERE OCTET_LENGTH(host.parent_hash) = 32
               AND host.block_status <> 'orphaned'
               AND (
                   NOT bounds.needs_anchor
                   OR EXISTS (
                       SELECT 1
                         FROM anchor
                        WHERE anchor.host_chain_id = bounds.host_chain_id
                   )
               )
        ), inserted AS (
            INSERT INTO block_manifest_state (
                generation,
                host_chain_id,
                block_number,
                block_hash,
                parent_block_hash,
                publication_cadence
            )
            SELECT generation,
                   host_chain_id,
                   block_number,
                   block_hash,
                   parent_hash,
                   publication_cadence
              FROM source
            ON CONFLICT (generation, host_chain_id, block_hash) DO NOTHING
            RETURNING 1
        )
        SELECT COUNT(*)::BIGINT AS "inserted!" FROM inserted
        "#,
        generation,
        MANIFEST_DISCOVERY_BLOCK_OVERLAP,
    )
    .fetch_one(pool)
    .await?;
    Ok(u64::try_from(inserted).expect("insert count is non-negative"))
}

#[cfg(test)]
pub(crate) async fn pending_chain_ids(pool: &PgPool) -> Result<Vec<i64>, ExecutionError> {
    let generation = crate::manifest_consensus::storage::active::load_generation(pool).await?;
    pending_chain_ids_for_generation(pool, generation).await
}

pub(crate) async fn pending_chain_ids_for_generation(
    pool: &PgPool,
    generation: i64,
) -> Result<Vec<i64>, ExecutionError> {
    let rows = sqlx::query!(
        r#"
        SELECT DISTINCT host_chain_id
         FROM block_manifest_state candidate
         WHERE candidate.generation = $1
           AND (block_content_digest IS NULL
            OR (
                manifest_required
                AND manifest_published = FALSE
                AND (
                    publication_error_count = 0
                    OR publication_next_retry_at <= NOW()
                )
            ))
         ORDER BY host_chain_id
        "#,
        generation,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|row| row.host_chain_id).collect())
}

/// Locks the earliest eligible work row after `cursor`. `SKIP LOCKED` lets
/// another worker progress an independent lineage while this transaction holds
/// the selected row. `cursor` is only this caller's local scan position.
#[cfg(test)]
pub(crate) async fn lock_next_block_to_progress(
    trx: &mut Transaction<'_, Postgres>,
    host_chain_id: i64,
    cursor: &ManifestProgressCursor,
) -> Result<Option<PendingBlock>, ExecutionError> {
    let generation = sqlx::query_scalar::<_, i64>(
        "SELECT generation FROM blue_green_generation WHERE singleton = TRUE",
    )
    .fetch_one(trx.as_mut())
    .await?;
    lock_next_block_to_progress_for_generation(trx, host_chain_id, cursor, generation).await
}

pub(crate) async fn lock_next_block_to_progress_for_generation(
    trx: &mut Transaction<'_, Postgres>,
    host_chain_id: i64,
    cursor: &ManifestProgressCursor,
    generation: i64,
) -> Result<Option<PendingBlock>, ExecutionError> {
    set_local_statement_timeout(trx, MANIFEST_WORK_SELECTION_TIMEOUT).await?;
    let result = sqlx::query!(
        r#"
        WITH RECURSIVE blocked_descendants AS (
            SELECT child.generation,
                   child.host_chain_id,
                   child.block_hash
              FROM block_manifest_state blocker
              JOIN block_manifest_state child
                ON child.generation = blocker.generation
               AND child.host_chain_id = blocker.host_chain_id
               AND child.parent_block_hash = blocker.block_hash
             WHERE blocker.host_chain_id = $1
               AND blocker.generation = $4
               AND (
                    blocker.block_content_digest IS NULL
                    OR (
                        blocker.manifest_required
                        AND blocker.manifest_published = FALSE
                        AND (
                            blocker.publication_error_count = 0
                            OR blocker.publication_next_retry_at IS NOT NULL
                        )
                    )
               )
            UNION
            SELECT child.generation,
                   child.host_chain_id,
                   child.block_hash
              FROM blocked_descendants blocked
              JOIN block_manifest_state child
                ON child.generation = blocked.generation
               AND child.host_chain_id = blocked.host_chain_id
               AND child.parent_block_hash = blocked.block_hash
        )
        SELECT candidate.generation,
               candidate.host_chain_id,
               candidate.block_number,
               candidate.block_hash,
               candidate.parent_block_hash,
               candidate.publication_cadence,
               candidate.block_content_digest,
               candidate.block_handle_count,
               candidate.manifest_revision,
               candidate.manifest_publisher,
               candidate.manifest_digest,
               candidate.manifest_published
          FROM block_manifest_state candidate
         WHERE candidate.host_chain_id = $1
           AND candidate.generation = $4
           AND (
                candidate.block_content_digest IS NULL
                OR (
                    candidate.manifest_required
                    AND candidate.manifest_published = FALSE
                    AND (
                        candidate.publication_error_count = 0
                        OR candidate.publication_next_retry_at <= NOW()
                    )
                )
           )
           AND NOT EXISTS (
                SELECT 1
                  FROM blocked_descendants blocked
                 WHERE blocked.host_chain_id = candidate.host_chain_id
                   AND blocked.generation = candidate.generation
                   AND blocked.block_hash = candidate.block_hash
           )
           AND (
                candidate.block_number > $2
                OR (
                    candidate.block_number = $2
                    AND candidate.block_hash > $3
                )
           )
         ORDER BY candidate.block_number, candidate.block_hash
         LIMIT 1
           FOR UPDATE SKIP LOCKED
        "#,
        host_chain_id,
        cursor.block_number,
        &cursor.block_hash,
        generation,
    )
    .fetch_optional(trx.as_mut())
    .await;

    // The bound is only for the recursive selector. Later manifest preparation
    // may legitimately issue several ordinary queries in this transaction.
    if result.is_ok() {
        set_local_statement_timeout(trx, Duration::ZERO).await?;
    }
    let row = result?;

    Ok(row.map(|row| PendingBlock {
        generation: row.generation,
        host_chain_id: row.host_chain_id,
        block_number: row.block_number,
        block_hash: row.block_hash,
        parent_block_hash: row.parent_block_hash,
        publication_cadence: row.publication_cadence,
        block_content_digest: row.block_content_digest,
        block_handle_count: row.block_handle_count,
        manifest_revision: row.manifest_revision,
        manifest_publisher: row.manifest_publisher,
        manifest_digest: row.manifest_digest,
        manifest_published: row.manifest_published,
    }))
}

pub(super) async fn set_local_statement_timeout(
    trx: &mut Transaction<'_, Postgres>,
    timeout: Duration,
) -> Result<(), ExecutionError> {
    let timeout = format!("{}ms", timeout.as_millis());
    sqlx::query!("SELECT set_config('statement_timeout', $1, TRUE)", timeout)
        .fetch_one(trx.as_mut())
        .await?;
    Ok(())
}

/// Discovers children only below host blocks whose lineage can still change.
/// Finalized and orphaned parents are closed after their currently visible
/// children have been copied, keeping polling cost bounded by the finality window.
#[cfg(test)]
pub(crate) async fn discover_known_children(pool: &PgPool) -> Result<u64, ExecutionError> {
    let generation = crate::manifest_consensus::storage::active::load_generation(pool).await?;
    discover_known_children_for_generation(pool, generation).await
}

pub(crate) async fn discover_known_children_for_generation(
    pool: &PgPool,
    generation: i64,
) -> Result<u64, ExecutionError> {
    let row = sqlx::query!(
        r#"
        WITH inserted AS (
            INSERT INTO block_manifest_state (
                generation,
                host_chain_id,
                block_number,
                block_hash,
                parent_block_hash,
                publication_cadence
            )
            SELECT parent.generation,
                   child.chain_id,
                   child.block_number,
                   child.block_hash,
                   child.parent_hash,
                   parent.publication_cadence
              FROM block_manifest_state parent
              JOIN host_chain_blocks_valid parent_host
                ON parent_host.chain_id = parent.host_chain_id
               AND parent_host.block_hash = parent.block_hash
              JOIN host_chain_blocks_valid child
                ON child.chain_id = parent.host_chain_id
               AND child.parent_hash = parent.block_hash
             WHERE NOT parent.child_block_discovery_closed
               AND parent.generation = $1
               AND parent_host.block_status <> 'orphaned'
               AND child.block_status <> 'orphaned'
               AND OCTET_LENGTH(child.parent_hash) = 32
            ON CONFLICT (generation, host_chain_id, block_hash) DO NOTHING
            RETURNING 1
        ), closed AS (
            UPDATE block_manifest_state block
               SET child_block_discovery_closed = TRUE,
                   updated_at = NOW()
              FROM host_chain_blocks_valid host
             WHERE NOT block.child_block_discovery_closed
               AND block.generation = $1
               AND host.chain_id = block.host_chain_id
               AND host.block_hash = block.block_hash
               AND host.block_status IN ('finalized', 'orphaned')
            RETURNING 1
        )
        SELECT COUNT(*) AS "inserted!" FROM inserted
        "#,
        generation,
    )
    .fetch_one(pool)
    .await?;
    Ok(u64::try_from(row.inserted).expect("insert count is non-negative"))
}

pub(crate) async fn discover_block_children(
    trx: &mut Transaction<'_, Postgres>,
    block: &PendingBlock,
) -> Result<u64, ExecutionError> {
    let row = sqlx::query!(
        r#"
        WITH inserted AS (
            INSERT INTO block_manifest_state (
                generation,
                host_chain_id,
                block_number,
                block_hash,
                parent_block_hash,
                publication_cadence
            )
            SELECT $4,
                   child.chain_id,
                   child.block_number,
                   child.block_hash,
                   child.parent_hash,
                   $3
              FROM host_chain_blocks_valid child
             WHERE child.chain_id = $1
               AND child.parent_hash = $2
               AND child.block_status <> 'orphaned'
               AND OCTET_LENGTH(child.parent_hash) = 32
            ON CONFLICT (generation, host_chain_id, block_hash) DO NOTHING
            RETURNING 1
        )
        SELECT COUNT(*) AS "inserted!" FROM inserted
        "#,
        block.host_chain_id,
        &block.block_hash,
        block.publication_cadence,
        block.generation,
    )
    .fetch_one(trx.as_mut())
    .await?;
    Ok(u64::try_from(row.inserted).expect("insert count is non-negative"))
}
