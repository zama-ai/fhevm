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

/// `producer_block_hash` value marking a row as branchless (valid on every
/// branch). This matches the column default in the branch-table migrations.
pub const BRANCHLESS_PRODUCER_BLOCK_HASH: &[u8] = &[];

/// Conservative default before the settlement process has advanced any host
/// block. Ethereum block numbers are non-negative, so this disables the write
/// guard until the first explicit settlement row is created.
pub const INITIAL_SETTLED_HEIGHT: i64 = -1;

/// A candidate row carrying the `producer_block_hash` it was stored under.
pub trait ProducerBlockHashed {
    fn producer_block_hash(&self) -> &[u8];
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
    Ok(sqlx::query_scalar::<_, i64>(
        "SELECT settled_height
         FROM coprocessor_settlement
         WHERE chain_id = $1",
    )
    .bind(chain_id)
    .fetch_optional(tx.as_mut())
    .await?
    .unwrap_or(INITIAL_SETTLED_HEIGHT))
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
    // `pbs_computations_branch` has no `is_error` terminal state; incomplete
    // PBS rows are the only PBS rows that can block settlement.
    let rows = sqlx::query_scalar::<_, i64>(
        "SELECT b.block_number
         FROM host_chain_blocks_valid b
         WHERE b.chain_id = $1
           AND b.block_status = 'finalized'
           AND b.block_number >= $2
           AND b.block_number <= $3
           AND NOT EXISTS (
               SELECT 1
               FROM computations_branch c
               WHERE c.host_chain_id = b.chain_id
                 AND c.block_number = b.block_number
                 AND c.producer_block_hash = b.block_hash
                 AND c.is_completed = FALSE
                 AND c.is_error = FALSE
           )
           AND NOT EXISTS (
               SELECT 1
               FROM pbs_computations_branch p
               WHERE p.host_chain_id = b.chain_id
                 AND p.block_number = b.block_number
                 AND p.block_hash = b.block_hash
                 AND p.is_completed = FALSE
                 AND NOT EXISTS (
                     SELECT 1
                     FROM computations_branch pc
                     WHERE pc.host_chain_id = p.host_chain_id
                       AND pc.output_handle = p.handle
                       AND pc.producer_block_hash = p.producer_block_hash
                       AND pc.is_error = TRUE
                 )
           )
         ORDER BY b.block_number ASC",
    )
    .bind(chain_id)
    .bind(first_block_to_check)
    .bind(candidate_height)
    .fetch_all(tx.as_mut())
    .await?;

    let settled_height = next_settled_height(current, candidate_height, branch_cutover_block, rows);

    if settled_height > current {
        sqlx::query(
            "INSERT INTO coprocessor_settlement(chain_id, settled_height, updated_at)
             VALUES($1, $2, CURRENT_TIMESTAMP)
             ON CONFLICT (chain_id) DO UPDATE
             SET settled_height = GREATEST(coprocessor_settlement.settled_height, EXCLUDED.settled_height),
                 updated_at = CASE
                     WHEN EXCLUDED.settled_height > coprocessor_settlement.settled_height
                     THEN CURRENT_TIMESTAMP
                     ELSE coprocessor_settlement.updated_at
                 END",
        )
        .bind(chain_id)
        .bind(settled_height)
        .execute(tx.as_mut())
        .await?;
    }

    Ok(settled_height)
}

fn next_settled_height(
    current: i64,
    candidate_height: i64,
    branch_cutover_block: i64,
    drained_finalized_blocks: impl IntoIterator<Item = i64>,
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

    let mut expected_block = settled_height
        .saturating_add(1)
        .max(first_post_cutover_block);
    for block_number in drained_finalized_blocks {
        if block_number < expected_block {
            continue;
        }
        if block_number > expected_block || block_number > candidate_height {
            break;
        }
        settled_height = block_number;
        if settled_height >= candidate_height {
            break;
        }
        expected_block = expected_block.saturating_add(1);
    }

    settled_height
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
    fn settlement_skips_pre_cutover_history_without_branch_work() {
        let settled = next_settled_height(-1, 9, 10, []);
        assert_eq!(settled, 9);
    }

    #[test]
    fn settlement_does_not_leapfrog_first_post_cutover_gap() {
        let settled = next_settled_height(-1, 15, 10, [12, 13, 14, 15]);
        assert_eq!(settled, 9);
    }

    #[test]
    fn settlement_advances_contiguously_after_cutover() {
        let settled = next_settled_height(9, 13, 10, [10, 11, 12, 13]);
        assert_eq!(settled, 13);
    }

    #[test]
    fn settlement_stops_at_lowest_undrained_post_cutover_block() {
        let settled = next_settled_height(9, 15, 10, [10, 11, 13, 14, 15]);
        assert_eq!(settled, 11);
    }

    #[test]
    fn settlement_cutover_zero_requires_contiguous_chain_from_zero() {
        let settled = next_settled_height(-1, 2, 0, [1, 2]);
        assert_eq!(settled, -1);
    }
}
