//! Read-only MMR proof construction from a SQL snapshot + on-chain peak check.
//!
//! Unlike the embedded relayer path, this never triggers catch-up ingestion.
//! Background completed-block ingest owns all writes; a lagging snapshot returns
//! retryable `Lagging`, and equal-count peak divergence fails closed.

use zama_solana_acl::mmr::{mmr_build_proof, mmr_peaks_from_leaves, MmrProof};

use crate::chain::{ChainFetcher, OnChainLineageState};
use solana_proof_store::{ProofSnapshot, SqlProofStore, StoreError};

#[derive(Debug, Clone, PartialEq)]
pub struct MmrProofResult {
    pub mmr_proof: Option<MmrProof>,
    pub leaf_count: u64,
    /// Backwards-compatible wire name for the lineage `leaf_count` the proof was built against.
    pub proof_slot: u64,
    pub verified: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum ProofError {
    #[error("chain error: {0}")]
    Chain(#[from] crate::chain::ChainError),
    #[error("store error: {0}")]
    Store(#[from] StoreError),
    #[error("no on-chain account found for lineage")]
    LineageNotFound,
    #[error("lineage proof data is lagging chain state (leaf_count {leaf_count})")]
    Lagging { leaf_count: u64 },
    #[error(
        "lineage proof store diverged from chain (leaf_count {leaf_count}); integrity rebuild required"
    )]
    CorruptStore { leaf_count: u64 },
    #[error("leaf index {leaf_index} out of range for leaf_count {leaf_count}")]
    LeafIndexOutOfRange { leaf_index: u64, leaf_count: u64 },
}

/// Builds a verified proof for `(lineage, leaf_index)` using one SQL snapshot and
/// the confirmed on-chain account. Read-only: never writes to the store.
pub async fn build_proof<C: ChainFetcher>(
    fetcher: &C,
    store: &SqlProofStore,
    lineage: [u8; 32],
    leaf_index: u64,
) -> Result<MmrProofResult, ProofError> {
    let on_chain = fetcher
        .get_lineage_state(lineage)
        .await?
        .ok_or(ProofError::LineageNotFound)?;

    if leaf_index >= on_chain.leaf_count {
        return Err(ProofError::Lagging {
            leaf_count: on_chain.leaf_count,
        });
    }

    let snapshot = store.proof_snapshot(lineage).await?;
    let Some(snapshot) = snapshot else {
        // Chain has the lineage but the store has not ingested it yet.
        return Err(ProofError::Lagging {
            leaf_count: on_chain.leaf_count,
        });
    };

    try_build_from_snapshot(&snapshot, &on_chain, leaf_index)
}

fn try_build_from_snapshot(
    snapshot: &ProofSnapshot,
    on_chain: &OnChainLineageState,
    leaf_index: u64,
) -> Result<MmrProofResult, ProofError> {
    // Internal consistency: recomputed peaks must match the persisted peaks.
    let recomputed = mmr_peaks_from_leaves(&snapshot.leaves);
    if recomputed != snapshot.peaks || snapshot.leaf_count != snapshot.leaves.len() as u64 {
        return Err(ProofError::CorruptStore {
            leaf_count: on_chain.leaf_count,
        });
    }

    match snapshot.leaf_count.cmp(&on_chain.leaf_count) {
        std::cmp::Ordering::Less => {
            return Err(ProofError::Lagging {
                leaf_count: on_chain.leaf_count,
            });
        }
        std::cmp::Ordering::Greater => {
            // Store ahead of confirmed chain is not a recoverable lag signal.
            return Err(ProofError::CorruptStore {
                leaf_count: on_chain.leaf_count,
            });
        }
        std::cmp::Ordering::Equal => {}
    }

    if snapshot.peaks != on_chain.peaks {
        return Err(ProofError::CorruptStore {
            leaf_count: on_chain.leaf_count,
        });
    }

    let proof =
        mmr_build_proof(&snapshot.leaves, leaf_index).ok_or(ProofError::LeafIndexOutOfRange {
            leaf_index,
            leaf_count: snapshot.leaf_count,
        })?;

    Ok(MmrProofResult {
        mmr_proof: Some(proof),
        leaf_count: on_chain.leaf_count,
        proof_slot: on_chain.leaf_count,
        verified: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use zama_solana_acl::mmr::mmr_append;

    fn pk(tag: u8) -> [u8; 32] {
        [tag; 32]
    }

    fn snapshot_with_leaves(lineage: [u8; 32], leaves: Vec<[u8; 32]>) -> ProofSnapshot {
        let peaks = mmr_peaks_from_leaves(&leaves);
        ProofSnapshot {
            lineage,
            current_handle: None,
            subjects: vec![],
            leaf_count: leaves.len() as u64,
            peaks,
            leaves,
            last_slot: 1,
        }
    }

    #[test]
    fn builds_verified_proof_when_snapshot_matches_chain() {
        let lineage = pk(0x01);
        let leaf =
            zama_solana_acl::historical_access_leaf_commitment(lineage, 0, pk(0x10), pk(0x30));
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, leaf).unwrap();

        let snapshot = snapshot_with_leaves(lineage, vec![leaf]);
        let on_chain = OnChainLineageState { peaks, leaf_count };
        let result = try_build_from_snapshot(&snapshot, &on_chain, 0).unwrap();
        assert!(result.verified);
        assert_eq!(result.leaf_count, 1);
        assert!(result.mmr_proof.is_some());
    }

    #[test]
    fn returns_lagging_when_snapshot_leaf_count_is_behind() {
        let lineage = pk(0x02);
        let leaf0 = zama_solana_acl::public_decrypt_leaf_commitment(lineage, 0, pk(0x10));
        let leaf1 = zama_solana_acl::public_decrypt_leaf_commitment(lineage, 1, pk(0x11));
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, leaf0).unwrap();
        mmr_append(&mut peaks, &mut leaf_count, leaf1).unwrap();

        let snapshot = snapshot_with_leaves(lineage, vec![leaf0]);
        let on_chain = OnChainLineageState { peaks, leaf_count };
        let err = try_build_from_snapshot(&snapshot, &on_chain, 0).unwrap_err();
        assert!(matches!(err, ProofError::Lagging { leaf_count: 2 }));
    }

    #[test]
    fn returns_corrupt_when_equal_count_peaks_diverge() {
        let lineage = pk(0x03);
        let leaf = zama_solana_acl::public_decrypt_leaf_commitment(lineage, 0, pk(0x10));
        let other = zama_solana_acl::public_decrypt_leaf_commitment(lineage, 0, pk(0xAA));
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, other).unwrap();

        let snapshot = snapshot_with_leaves(lineage, vec![leaf]);
        let on_chain = OnChainLineageState { peaks, leaf_count };
        let err = try_build_from_snapshot(&snapshot, &on_chain, 0).unwrap_err();
        assert!(matches!(err, ProofError::CorruptStore { leaf_count: 1 }));
    }

    #[test]
    fn returns_corrupt_when_persisted_peaks_do_not_match_leaves() {
        let lineage = pk(0x04);
        let leaf = zama_solana_acl::public_decrypt_leaf_commitment(lineage, 0, pk(0x10));
        let mut peaks = Vec::new();
        let mut leaf_count = 0u64;
        mmr_append(&mut peaks, &mut leaf_count, leaf).unwrap();

        let mut snapshot = snapshot_with_leaves(lineage, vec![leaf]);
        snapshot.peaks = vec![pk(0xFF)];
        let on_chain = OnChainLineageState {
            peaks: peaks.clone(),
            leaf_count,
        };
        let err = try_build_from_snapshot(&snapshot, &on_chain, 0).unwrap_err();
        assert!(matches!(err, ProofError::CorruptStore { leaf_count: 1 }));
    }
}
