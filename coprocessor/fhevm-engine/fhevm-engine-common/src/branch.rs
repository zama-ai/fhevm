//! Branch-context helpers shared by the services that read the `*_branch`
//! tables (tfhe-worker, sns-worker).
//!
//! Rows in `ciphertexts_branch` / `ciphertexts128_branch` are keyed by the
//! hash of the block that produced them. Ciphertexts that are not derived
//! from any block — ZK-verified user inputs written by the zkproof-worker —
//! are stored with an empty `producer_block_hash` ("branchless"): they are
//! valid on every branch and must survive reorg cleanup, which only targets
//! rows keyed by real block hashes.

use sqlx::{Postgres, Transaction};
use std::collections::HashSet;

/// `producer_block_hash` value marking a row as branchless (valid on every
/// branch). This matches the column default in the branch-table migrations.
pub const BRANCHLESS_PRODUCER_BLOCK_HASH: &[u8] = &[];

/// Conservative default before the settlement process has advanced any host
/// block. Ethereum block numbers are non-negative, so this disables the write
/// guard until the first explicit settlement row is created.
pub const INITIAL_SETTLED_HEIGHT: i64 = -1;

/// Validate the coordinated wave-1 activation and wave-2 cutover heights.
/// A cutover below activation creates a block range that neither the legacy
/// nor branch worker will execute.
pub fn validate_branch_rollout_bounds(
    activation_block: u64,
    cutover_block: i64,
) -> Result<(), String> {
    if cutover_block < 0 {
        return Err(format!(
            "FHEVM_BRANCH_CUTOVER_BLOCK must be non-negative, got {cutover_block}"
        ));
    }
    if (cutover_block as u64) < activation_block {
        return Err(format!(
            "FHEVM_BRANCH_CUTOVER_BLOCK ({cutover_block}) must be greater than or equal to \
             FHEVM_BRANCH_ACTIVATION_BLOCK ({activation_block}); otherwise blocks in the gap \
             have neither legacy nor branch work"
        ));
    }
    Ok(())
}

/// A candidate row carrying the `producer_block_hash` it was stored under.
pub trait ProducerBlockHashed {
    fn producer_block_hash(&self) -> &[u8];
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct S3CanonicalPublicationTarget {
    pub handle: Vec<u8>,
    pub producer_block_hash: Vec<u8>,
    pub block_hash: Vec<u8>,
    pub block_number: Option<i64>,
}

/// Selects the ciphertext row to use for a dependency resolved to
/// `producer_block_hash`.
///
/// A branchful dependency must match its exact producer block. Branchless rows
/// are valid only when the dependency itself is branchless; using them as a
/// fallback for a missing branchful row can silently cross fork contexts.
pub fn select_producer_candidate<'a, T: ProducerBlockHashed>(
    candidates: &'a [T],
    producer_block_hash: &[u8],
) -> Option<&'a T> {
    candidates
        .iter()
        .find(|candidate| candidate.producer_block_hash() == producer_block_hash)
}

pub fn is_branchless_producer(producer_block_hash: &[u8]) -> bool {
    producer_block_hash == BRANCHLESS_PRODUCER_BLOCK_HASH
}

pub async fn read_settled_height(
    tx: &mut Transaction<'_, Postgres>,
    chain_id: i64,
) -> Result<i64, sqlx::Error> {
    Ok(sqlx::query_scalar!(
        r#"
        SELECT settled_height AS "settled_height!"
         FROM coprocessor_settlement
         WHERE chain_id = $1
        "#,
        chain_id,
    )
    .fetch_optional(tx.as_mut())
    .await?
    .unwrap_or(INITIAL_SETTLED_HEIGHT))
}

/// Unix epoch seconds of the last settlement-frontier advance, or `None`
/// until the chain's first explicit settlement row exists. `updated_at` is a
/// timezone-naive column written with `CURRENT_TIMESTAMP`; deployments run
/// UTC, matching the assumption made for every other timestamp column.
pub async fn read_settled_height_updated_epoch(
    tx: &mut Transaction<'_, Postgres>,
    chain_id: i64,
) -> Result<Option<i64>, sqlx::Error> {
    Ok(sqlx::query_scalar!(
        r#"
        SELECT updated_at AS "updated_at!"
         FROM coprocessor_settlement
         WHERE chain_id = $1
        "#,
        chain_id,
    )
    .fetch_optional(tx.as_mut())
    .await?
    .map(|updated_at| updated_at.assume_utc().unix_timestamp()))
}

pub async fn advance_settled_height(
    tx: &mut Transaction<'_, Postgres>,
    chain_id: i64,
    candidate_height: i64,
    branch_cutover_block: i64,
) -> Result<i64, sqlx::Error> {
    let current = read_settled_height(tx, chain_id).await?;
    if candidate_height <= current {
        return Ok(current);
    }

    let first_post_cutover_block = branch_cutover_block.max(0);
    let first_block_to_check = current.saturating_add(1).max(first_post_cutover_block);
    // The block table is sparse after range catch-up: only blocks carrying
    // relevant logs are necessarily materialized. Find the first *stored*
    // checkpoint which is pending or has undrained work; absent empty heights
    // do not freeze the frontier. Terminal PBS errors are settled just like
    // terminal computation errors: they remain auditable and recoverable, but
    // cannot freeze the frontier.
    let first_blocked_height = sqlx::query_scalar!(
        r#"
        SELECT MIN(b.block_number) AS "block_number?"
         FROM host_chain_blocks_valid b
         WHERE b.chain_id = $1
           AND b.block_number >= $2
           AND b.block_number <= $3
           AND (
             b.block_status = 'pending'
             OR (
               b.block_status = 'finalized'
               AND (
                 -- Two finalized rows at one height (a refused pass inserts
                 -- the canonical row as finalized without orphaning the stale
                 -- sibling) must be resolved by revalidation before the
                 -- frontier crosses: a stale finalized row left below the
                 -- frontier would poison settled producer resolution.
                 EXISTS (
               SELECT 1
               FROM host_chain_blocks_valid sib
               WHERE sib.chain_id = b.chain_id
                 AND sib.block_number = b.block_number
                 AND sib.block_status = 'finalized'
                 AND sib.block_hash <> b.block_hash
                 )
                 OR EXISTS (
               SELECT 1
               FROM computations_branch c
               WHERE c.host_chain_id = b.chain_id
                 AND c.block_number = b.block_number
                 AND c.producer_block_hash = b.block_hash
                 AND c.is_completed = FALSE
                 AND c.is_error = FALSE
                 )
                 OR EXISTS (
               SELECT 1
               FROM pbs_computations_branch p
               WHERE p.host_chain_id = b.chain_id
                 AND p.block_number = b.block_number
                 AND p.block_hash = b.block_hash
                 AND p.is_completed = FALSE
                 AND p.is_error = FALSE
                 AND NOT EXISTS (
                     SELECT 1
                     FROM computations_branch pc
                     WHERE pc.host_chain_id = p.host_chain_id
                       AND pc.output_handle = p.handle
                       AND pc.producer_block_hash = p.producer_block_hash
                       AND pc.is_error = TRUE
                 )
                 )
                 OR EXISTS (
               SELECT 1
               FROM s3_canonical_repair_queue q
               JOIN ciphertext_digest_branch d
                 ON d.host_chain_id = q.host_chain_id
                AND d.handle = q.handle
                AND d.producer_block_hash = q.target_producer_block_hash
                AND d.block_hash = q.target_block_hash
               WHERE q.host_chain_id = b.chain_id
                 AND q.target_block_number = b.block_number
                 )
                 OR EXISTS (
               SELECT 1
               FROM ciphertext_digest_branch d
               WHERE d.host_chain_id = b.chain_id
                 AND d.block_number = b.block_number
                 AND d.block_hash = b.block_hash
                 AND (
                      d.ciphertext IS NULL
                      OR (
                          d.ciphertext128 IS NULL
                          AND EXISTS (
                              SELECT 1
                              FROM ciphertexts128_branch c
                              WHERE c.handle = d.handle
                                AND c.producer_block_hash = d.producer_block_hash
                                AND c.ciphertext IS NOT NULL
                          )
                      )
                      OR d.s3_publication_verified_at IS NULL
                      OR d.s3_publication_verified_digest IS DISTINCT FROM d.ciphertext
                      OR d.s3_publication_verified_producer_block_hash IS DISTINCT FROM d.producer_block_hash
                 )
                 )
               )
             )
           )
        "#,
        chain_id,
        first_block_to_check,
        candidate_height,
    )
    .fetch_one(tx.as_mut())
    .await?;

    // Incomplete orphan cleanup blocks settlement on its own, NOT via a
    // stored block row: settled dependency resolution trusts cleanup
    // completion, and with a sparse block table there may be no stored
    // finalized row at or above the job's height to carry the check.
    // Quarantined cleanup has not removed orphan rows either, so it keeps
    // blocking until an operator resolves it.
    let first_pending_cleanup_height = sqlx::query_scalar!(
        r#"
        SELECT MIN(finalized_block_number) AS "block_number?"
         FROM branch_cleanup_jobs
         WHERE chain_id = $1
           AND status IN ('pending', 'quarantined')
        "#,
        chain_id,
    )
    .fetch_one(tx.as_mut())
    .await?;

    let first_blocked_height = match (first_blocked_height, first_pending_cleanup_height) {
        (Some(row), Some(job)) => Some(row.min(job)),
        (row, None) => row,
        (None, job) => job,
    };

    let settled_height = next_settled_height(
        current,
        candidate_height,
        branch_cutover_block,
        first_blocked_height,
    );

    if settled_height > current {
        sqlx::query!(
            r#"
            INSERT INTO coprocessor_settlement(chain_id, settled_height, updated_at)
             VALUES($1, $2, CURRENT_TIMESTAMP)
             ON CONFLICT (chain_id) DO UPDATE
             SET settled_height = GREATEST(coprocessor_settlement.settled_height, EXCLUDED.settled_height),
                 updated_at = CASE
                     WHEN EXCLUDED.settled_height > coprocessor_settlement.settled_height
                     THEN CURRENT_TIMESTAMP
                     ELSE coprocessor_settlement.updated_at
                 END
            "#,
            chain_id,
            settled_height,
        )
        .execute(tx.as_mut())
        .await?;
    }

    Ok(settled_height)
}

pub async fn resolve_s3_canonical_publication_target(
    tx: &mut Transaction<'_, Postgres>,
    chain_id: i64,
    handle: &[u8],
) -> Result<Option<S3CanonicalPublicationTarget>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT d.handle AS "handle!",
               d.producer_block_hash AS "producer_block_hash!",
               d.block_hash AS "block_hash!",
               d.block_number
          FROM ciphertext_digest_branch d
          LEFT JOIN host_chain_blocks_valid producer
            ON producer.chain_id = d.host_chain_id
           AND producer.block_hash = d.producer_block_hash
           AND d.producer_block_hash <> ''::BYTEA
          LEFT JOIN host_chain_blocks_valid event_block
            ON event_block.chain_id = d.host_chain_id
           AND event_block.block_hash = d.block_hash
           AND d.block_hash <> ''::BYTEA
         WHERE d.host_chain_id = $1
           AND d.handle = $2
           AND (
                d.producer_block_hash = ''::BYTEA
                OR COALESCE(producer.block_status, 'pending') <> 'orphaned'
           )
           AND (
                d.block_hash = ''::BYTEA
                OR COALESCE(event_block.block_status, 'pending') <> 'orphaned'
           )
         ORDER BY COALESCE(d.block_number, -1) DESC,
                  CASE WHEN d.producer_block_hash = ''::BYTEA THEN 1 ELSE 0 END ASC,
                  d.created_at DESC
         LIMIT 1
        "#,
        chain_id,
        handle,
    )
    .fetch_optional(tx.as_mut())
    .await?;

    Ok(row.map(|row| S3CanonicalPublicationTarget {
        handle: row.handle,
        producer_block_hash: row.producer_block_hash,
        block_hash: row.block_hash,
        block_number: row.block_number,
    }))
}

pub async fn enqueue_s3_canonical_repair(
    tx: &mut Transaction<'_, Postgres>,
    chain_id: i64,
    handle: &[u8],
    reason: &str,
) -> Result<bool, sqlx::Error> {
    let Some(target) = resolve_s3_canonical_publication_target(tx, chain_id, handle).await? else {
        sqlx::query!(
            r#"
            DELETE FROM s3_canonical_repair_queue
             WHERE host_chain_id = $1
               AND handle = $2
            "#,
            chain_id,
            handle,
        )
        .execute(tx.as_mut())
        .await?;
        return Ok(false);
    };

    sqlx::query!(
        r#"
        INSERT INTO s3_canonical_repair_queue (
            host_chain_id,
            handle,
            target_producer_block_hash,
            target_block_hash,
            target_block_number,
            reason
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (host_chain_id, handle) DO UPDATE
        SET target_producer_block_hash = EXCLUDED.target_producer_block_hash,
            target_block_hash = EXCLUDED.target_block_hash,
            target_block_number = EXCLUDED.target_block_number,
            reason = EXCLUDED.reason,
            attempts = CASE
                WHEN s3_canonical_repair_queue.target_producer_block_hash IS DISTINCT FROM EXCLUDED.target_producer_block_hash
                  OR s3_canonical_repair_queue.target_block_hash IS DISTINCT FROM EXCLUDED.target_block_hash
                THEN 0
                ELSE s3_canonical_repair_queue.attempts
            END,
            status = CASE
                WHEN s3_canonical_repair_queue.target_producer_block_hash IS DISTINCT FROM EXCLUDED.target_producer_block_hash
                  OR s3_canonical_repair_queue.target_block_hash IS DISTINCT FROM EXCLUDED.target_block_hash
                THEN 'pending'
                ELSE s3_canonical_repair_queue.status
            END,
            last_error = CASE
                WHEN s3_canonical_repair_queue.target_producer_block_hash IS DISTINCT FROM EXCLUDED.target_producer_block_hash
                  OR s3_canonical_repair_queue.target_block_hash IS DISTINCT FROM EXCLUDED.target_block_hash
                THEN NULL
                ELSE s3_canonical_repair_queue.last_error
            END,
            last_error_at = CASE
                WHEN s3_canonical_repair_queue.target_producer_block_hash IS DISTINCT FROM EXCLUDED.target_producer_block_hash
                  OR s3_canonical_repair_queue.target_block_hash IS DISTINCT FROM EXCLUDED.target_block_hash
                THEN NULL
                ELSE s3_canonical_repair_queue.last_error_at
            END,
            locked_at = NULL,
            updated_at = NOW()
        "#,
        chain_id,
        &target.handle,
        &target.producer_block_hash,
        &target.block_hash,
        target.block_number,
        reason,
    )
    .execute(tx.as_mut())
    .await?;

    Ok(true)
}

pub async fn enqueue_s3_canonical_repairs(
    tx: &mut Transaction<'_, Postgres>,
    chain_id: i64,
    handles: impl IntoIterator<Item = Vec<u8>>,
    reason: &str,
) -> Result<u64, sqlx::Error> {
    let mut seen = HashSet::new();
    let mut enqueued = 0;
    for handle in handles {
        if !seen.insert(handle.clone()) {
            continue;
        }
        if enqueue_s3_canonical_repair(tx, chain_id, &handle, reason).await? {
            enqueued += 1;
        }
    }
    Ok(enqueued)
}

fn next_settled_height(
    current: i64,
    candidate_height: i64,
    branch_cutover_block: i64,
    first_blocked_height: Option<i64>,
) -> i64 {
    if candidate_height <= current {
        return current;
    }

    let first_post_cutover_block = branch_cutover_block.max(0);
    let pre_cutover_ceiling = first_post_cutover_block.saturating_sub(1);

    let mut settled_height = current;
    if settled_height < pre_cutover_ceiling {
        settled_height = candidate_height.min(pre_cutover_ceiling);
    }
    if settled_height >= candidate_height || candidate_height < first_post_cutover_block {
        return settled_height;
    }

    if let Some(blocked_height) = first_blocked_height {
        // A blocker below the post-cutover range (an unfinished cleanup job
        // can sit at any height) must still hold the frontier: clamping to
        // `settled_height` via `max` keeps the pre-cutover skip but forbids
        // crossing the blocker. Dropping it instead would fail open past
        // every higher blocker it shadowed through the MIN merge.
        if blocked_height <= candidate_height {
            return candidate_height
                .min(blocked_height.saturating_sub(1))
                .max(settled_height);
        }
    }
    candidate_height
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Candidate {
        producer_block_hash: Vec<u8>,
        tag: u8,
    }

    impl ProducerBlockHashed for Candidate {
        fn producer_block_hash(&self) -> &[u8] {
            &self.producer_block_hash
        }
    }

    fn candidate(hash: &[u8], tag: u8) -> Candidate {
        Candidate {
            producer_block_hash: hash.to_vec(),
            tag,
        }
    }

    #[test]
    fn exact_branch_match_wins_over_branchless() {
        let candidates = vec![
            candidate(&[], 1),
            candidate(&[0xaa; 32], 2),
            candidate(&[0xbb; 32], 3),
        ];
        let selected = select_producer_candidate(&candidates, &[0xbb; 32]).unwrap();
        assert_eq!(selected.tag, 3);
    }

    #[test]
    fn branchful_request_does_not_fall_back_to_branchless() {
        let candidates = vec![candidate(&[0xaa; 32], 1), candidate(&[], 2)];
        assert!(select_producer_candidate(&candidates, &[0xbb; 32]).is_none());
    }

    #[test]
    fn no_match_without_exact_or_branchless_candidate() {
        let candidates = vec![candidate(&[0xaa; 32], 1)];
        assert!(select_producer_candidate(&candidates, &[0xbb; 32]).is_none());
    }

    #[test]
    fn empty_request_hash_selects_branchless_row() {
        let candidates = vec![candidate(&[0xaa; 32], 1), candidate(&[], 2)];
        let selected = select_producer_candidate(&candidates, &[]).unwrap();
        assert_eq!(selected.tag, 2);
    }

    #[test]
    fn rollout_bounds_reject_cutover_before_activation() {
        assert!(validate_branch_rollout_bounds(100, 99).is_err());
        assert!(validate_branch_rollout_bounds(100, -1).is_err());
        assert!(validate_branch_rollout_bounds(100, 100).is_ok());
        assert!(validate_branch_rollout_bounds(100, 101).is_ok());
    }

    #[test]
    fn settlement_skips_pre_cutover_history_without_branch_work() {
        let settled = next_settled_height(-1, 9, 10, None);
        assert_eq!(settled, 9);
    }

    #[test]
    fn settlement_stops_before_first_stored_blocker() {
        let settled = next_settled_height(-1, 15, 10, Some(12));
        assert_eq!(settled, 11);
    }

    #[test]
    fn settlement_advances_across_sparse_drained_history() {
        let settled = next_settled_height(9, 13, 10, None);
        assert_eq!(settled, 13);
    }

    #[test]
    fn settlement_stops_at_lowest_undrained_post_cutover_block() {
        let settled = next_settled_height(9, 15, 10, Some(12));
        assert_eq!(settled, 11);
    }

    #[test]
    fn settlement_cutover_zero_allows_sparse_drained_history() {
        let settled = next_settled_height(-1, 2, 0, None);
        assert_eq!(settled, 2);
    }

    #[test]
    fn settlement_freezes_on_blocker_below_cutover() {
        // A pre-cutover blocker (an unfinished cleanup job) keeps the
        // pre-cutover skip but must not be discarded: the frontier stays at
        // the cutover ceiling instead of failing open to the candidate.
        let settled = next_settled_height(-1, 15, 10, Some(4));
        assert_eq!(settled, 9);
    }

    #[test]
    fn settlement_freezes_on_blocker_below_current() {
        // A re-activated cleanup job at or below the frontier holds it.
        let settled = next_settled_height(7, 15, 0, Some(5));
        assert_eq!(settled, 7);
    }
}
