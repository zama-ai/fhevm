//! Minimal Merkle Mountain Range over 32-byte leaf commitments.
//!
//! The live `EncryptedValue` account stores only the MMR *peaks* and the leaf
//! count; full inclusion proofs are reconstructed off-chain and verified here —
//! and, identically, on-chain and in the KMS, since this is the single shared
//! implementation. SHA-256 with domain separation; leaf and node prefixes
//! differ so a leaf can never be reinterpreted as an internal node.

use crate::{sha256, AclError};

const LEAF_PREFIX: &[u8] = b"ZAMA_MMR_LEAF_V1";
const NODE_PREFIX: &[u8] = b"ZAMA_MMR_NODE_V1";

/// Maximum peak entries representable by this MMR.
///
/// `leaf_count` is a `u64`, and an MMR has one peak per set bit in
/// `leaf_count`, so a valid state can never exceed 64 peaks. Reaching 64 peaks
/// requires `u64::MAX` leaves, which normal on-chain operation cannot
/// realistically reach; the explicit cap exists to fail before mutating state
/// at the representation boundary.
pub const MAX_MMR_PEAKS: usize = u64::BITS as usize;

/// An inclusion proof: the authentication path from a leaf up to its mountain's
/// peak. Rides in on a decrypt request. `siblings.len()` equals the mountain height.
#[derive(borsh::BorshSerialize, borsh::BorshDeserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct MmrProof {
    pub leaf_index: u64,
    pub siblings: Vec<[u8; 32]>,
}

/// Hashes a leaf commitment into its MMR leaf node.
pub fn mmr_leaf_node(commitment: &[u8; 32]) -> [u8; 32] {
    sha256(&[LEAF_PREFIX, commitment])
}

/// Hashes two child nodes into their MMR parent node.
pub fn mmr_node(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    sha256(&[NODE_PREFIX, left, right])
}

/// Appends one leaf commitment to the running peaks (oldest mountain first),
/// advancing the leaf count. Carries merge equal-height peaks.
pub fn mmr_append(
    peaks: &mut Vec<[u8; 32]>,
    leaf_count: &mut u64,
    commitment: [u8; 32],
) -> Result<(), AclError> {
    if peaks.len() > MAX_MMR_PEAKS {
        return Err(AclError::MmrPeakCapacityExceeded);
    }
    if peaks.len() != leaf_count.count_ones() as usize {
        return Err(AclError::MmrInconsistent);
    }
    let next_leaf_count = leaf_count
        .checked_add(1)
        .ok_or(AclError::MmrPeakCapacityExceeded)?;
    let mut node = mmr_leaf_node(&commitment);
    let mut carry = *leaf_count;
    while carry & 1 == 1 {
        let left = peaks.pop().ok_or(AclError::MmrInconsistent)?;
        node = mmr_node(&left, &node);
        carry >>= 1;
    }
    peaks.push(node);
    *leaf_count = next_leaf_count;
    Ok(())
}

/// Verifies that `commitment` is the leaf at `proof.leaf_index`, recomputing its
/// mountain peak and matching it against the stored peak. Peaks are not bagged
/// into a single root — they live in the finalized account, so a peak match suffices.
pub fn mmr_verify(
    peaks: &[[u8; 32]],
    leaf_count: u64,
    commitment: [u8; 32],
    proof: &MmrProof,
) -> bool {
    if peaks.len() > MAX_MMR_PEAKS
        || proof.siblings.len() > MAX_MMR_PEAKS
        || proof.leaf_index >= leaf_count
        || peaks.len() != leaf_count.count_ones() as usize
    {
        return false;
    }
    let mut offset: u64 = 0;
    let mut peak_pos: usize = 0;
    for height in (0..64).rev() {
        let bit = 1u64 << height;
        if leaf_count & bit == 0 {
            continue;
        }
        if proof.leaf_index >= offset && proof.leaf_index < offset + bit {
            if proof.siblings.len() != height {
                return false;
            }
            let mut node = mmr_leaf_node(&commitment);
            let mut local = proof.leaf_index - offset;
            for sibling in &proof.siblings {
                node = if local.is_multiple_of(2) {
                    mmr_node(&node, sibling)
                } else {
                    mmr_node(sibling, &node)
                };
                local >>= 1;
            }
            return node == peaks[peak_pos];
        }
        offset += bit;
        peak_pos += 1;
    }
    false
}

/// Off-chain helper: recompute the full peak set for an ordered leaf list, by an
/// independent stack-fold from [`mmr_append`]. Used by clients/proof services/tests.
pub fn mmr_peaks_from_leaves(leaves: &[[u8; 32]]) -> Vec<[u8; 32]> {
    let mut stack: Vec<([u8; 32], u32)> = Vec::new();
    for leaf in leaves {
        let mut node = mmr_leaf_node(leaf);
        let mut height = 0u32;
        while let Some(&(top, top_h)) = stack.last() {
            if top_h != height {
                break;
            }
            stack.pop();
            node = mmr_node(&top, &node);
            height += 1;
        }
        stack.push((node, height));
    }
    stack.into_iter().map(|(node, _)| node).collect()
}

/// Off-chain helper: build the inclusion proof for the leaf at `leaf_index`.
pub fn mmr_build_proof(leaves: &[[u8; 32]], leaf_index: u64) -> Option<MmrProof> {
    let count = leaves.len() as u64;
    if leaf_index >= count {
        return None;
    }
    let mut offset: u64 = 0;
    for height in (0..64).rev() {
        let bit = 1u64 << height;
        if count & bit == 0 {
            continue;
        }
        if leaf_index >= offset && leaf_index < offset + bit {
            let mut level: Vec<[u8; 32]> = (offset..offset + bit)
                .map(|i| mmr_leaf_node(&leaves[i as usize]))
                .collect();
            let mut local = (leaf_index - offset) as usize;
            let mut siblings = Vec::new();
            while level.len() > 1 {
                let sibling = if local.is_multiple_of(2) {
                    local + 1
                } else {
                    local - 1
                };
                siblings.push(level[sibling]);
                level = level
                    .chunks(2)
                    .map(|pair| mmr_node(&pair[0], &pair[1]))
                    .collect();
                local /= 2;
            }
            return Some(MmrProof {
                leaf_index,
                siblings,
            });
        }
        offset += bit;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn leaf(i: u64) -> [u8; 32] {
        let mut l = [0u8; 32];
        l[..8].copy_from_slice(&i.to_be_bytes());
        l
    }

    fn append_all(n: u64) -> (Vec<[u8; 32]>, u64) {
        let mut peaks = Vec::new();
        let mut count = 0u64;
        for i in 0..n {
            mmr_append(&mut peaks, &mut count, leaf(i)).unwrap();
        }
        (peaks, count)
    }

    #[test]
    fn append_matches_independent_reference() {
        for n in 0u64..=33 {
            let (peaks, count) = append_all(n);
            assert_eq!(count, n);
            let leaves: Vec<_> = (0..n).map(leaf).collect();
            assert_eq!(peaks, mmr_peaks_from_leaves(&leaves));
        }
    }

    #[test]
    fn every_leaf_verifies_and_tampering_fails() {
        for n in 1u64..=33 {
            let (peaks, count) = append_all(n);
            let leaves: Vec<_> = (0..n).map(leaf).collect();
            for i in 0..n {
                let proof = mmr_build_proof(&leaves, i).unwrap();
                assert!(mmr_verify(&peaks, count, leaves[i as usize], &proof));
                assert!(!mmr_verify(&peaks, count, leaf(999), &proof));
            }
        }
    }

    #[test]
    fn leaf_and_node_domains_are_separated() {
        assert_ne!(mmr_leaf_node(&[1; 32]), mmr_node(&[1; 32], &[2; 32]));
    }

    #[test]
    fn hash_is_sha256() {
        // Pin the hash function: a leaf node is SHA-256(LEAF_PREFIX ‖ commitment).
        use sha2::{Digest as _, Sha256};
        let commitment = [7u8; 32];
        let mut hasher = Sha256::new();
        hasher.update(b"ZAMA_MMR_LEAF_V1");
        hasher.update(commitment);
        let expected: [u8; 32] = hasher.finalize().into();
        assert_eq!(mmr_leaf_node(&commitment), expected);
    }

    #[test]
    fn out_of_range_and_short_path_fail() {
        let (peaks, count) = append_all(8);
        assert!(!mmr_verify(
            &peaks,
            count,
            leaf(0),
            &MmrProof {
                leaf_index: 8,
                siblings: vec![]
            }
        ));
        let leaves: Vec<_> = (0..8).map(leaf).collect();
        let mut proof = mmr_build_proof(&leaves, 0).unwrap();
        proof.siblings.pop();
        assert!(!mmr_verify(&peaks, count, leaves[0], &proof));
    }

    #[test]
    fn append_at_peak_cap_fails_without_mutating() {
        let mut peaks: Vec<_> = (0..MAX_MMR_PEAKS)
            .map(|i| {
                let mut peak = [0u8; 32];
                peak[0] = i as u8;
                peak
            })
            .collect();
        let original_peaks = peaks.clone();
        let mut count = u64::MAX;

        let err = mmr_append(&mut peaks, &mut count, leaf(99)).unwrap_err();

        assert_eq!(err, AclError::MmrPeakCapacityExceeded);
        assert_eq!(peaks, original_peaks);
        assert_eq!(count, u64::MAX);
    }

    #[test]
    fn below_peak_cap_append_and_verify_are_unaffected() {
        let mut peaks: Vec<_> = (0..MAX_MMR_PEAKS - 1)
            .map(|i| {
                let mut peak = [0u8; 32];
                peak[0] = i as u8;
                peak
            })
            .collect();
        let mut count = u64::MAX - 1;

        mmr_append(&mut peaks, &mut count, leaf(100)).unwrap();

        assert_eq!(count, u64::MAX);
        assert_eq!(peaks.len(), MAX_MMR_PEAKS);

        let (mut over_cap_peaks, over_cap_count) = append_all(1);
        over_cap_peaks.resize(MAX_MMR_PEAKS + 1, [0u8; 32]);
        assert!(!mmr_verify(
            &over_cap_peaks,
            over_cap_count,
            leaf(0),
            &MmrProof {
                leaf_index: 0,
                siblings: vec![],
            }
        ));
    }
}
