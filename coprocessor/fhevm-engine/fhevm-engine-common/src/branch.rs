//! Branch-context helpers shared by the services that read the `*_branch`
//! tables (tfhe-worker, sns-worker).
//!
//! Rows in `ciphertexts_branch` / `ciphertexts128_branch` are keyed by the
//! hash of the block that produced them. Ciphertexts that are not derived
//! from any block — ZK-verified user inputs written by the zkproof-worker —
//! are stored with an empty `producer_block_hash` ("branchless"): they are
//! valid on every branch and must survive reorg cleanup, which only targets
//! rows keyed by real block hashes.

/// `producer_block_hash` value marking a row as branchless (valid on every
/// branch). This matches the column default in the branch-table migrations.
pub const BRANCHLESS_PRODUCER_BLOCK_HASH: &[u8] = &[];

/// A candidate row carrying the `producer_block_hash` it was stored under.
pub trait ProducerBlockHashed {
    fn producer_block_hash(&self) -> &[u8];
}

/// Selects the ciphertext row to use for a dependency resolved to
/// `producer_block_hash`: an exact branch match wins, otherwise fall back to
/// a branchless row (e.g. a ZK-verified user input).
pub fn select_producer_candidate<'a, T: ProducerBlockHashed>(
    candidates: &'a [T],
    producer_block_hash: &[u8],
) -> Option<&'a T> {
    candidates
        .iter()
        .find(|candidate| candidate.producer_block_hash() == producer_block_hash)
        .or_else(|| {
            candidates
                .iter()
                .find(|candidate| candidate.producer_block_hash() == BRANCHLESS_PRODUCER_BLOCK_HASH)
        })
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
    fn falls_back_to_branchless_when_no_exact_match() {
        let candidates = vec![candidate(&[0xaa; 32], 1), candidate(&[], 2)];
        let selected = select_producer_candidate(&candidates, &[0xbb; 32]).unwrap();
        assert_eq!(selected.tag, 2);
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
}
