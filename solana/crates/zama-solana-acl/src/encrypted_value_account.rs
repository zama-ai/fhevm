//! Off-chain reconstruction of an encrypted value account's full leaf list.
//!
//! The on-chain account stores only the MMR peaks and leaf count, never the ordered leaves a
//! decrypt proof needs. This module rebuilds that leaf list from the account's chronological
//! allow/make-public record and builds inclusion proofs from it, reusing the shared leaf
//! commitments and MMR exactly as the host program appends them — so a reconstructed account's
//! peaks match the chain byte-for-byte.
//!
//! Pure data transform: no I/O, no async, no chain access. The coprocessor decodes the host's
//! instructions into these events and stores the leaves; here the caller supplies the events.
//!
//! Public API surface: proof builders outside this repository. The coprocessor and the KMS
//! connector call these to mint and check inclusion proofs, and the peak/leaf helpers are exported
//! so a third-party verifier can reproduce the chain's bytes without reimplementing the MMR.

use crate::{
    historical_access_leaf_commitment, mmr_build_proof, mmr_peaks_from_leaves,
    public_decrypt_leaf_commitment, MmrProof,
};

/// Why a reconstruction or proof-build could not be trusted against chain state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EncryptedValueAccountError {
    /// The reconstructed `(peaks, leaf_count)` diverge from the on-chain account's.
    /// The record is incomplete or reordered; any proof built from it would be rejected by
    /// the KMS at verify time.
    PeaksDiverged,
    /// `leaf_index` is outside the reconstructed leaf list.
    LeafIndexOutOfRange,
}

/// One leaf-appending operation in an encrypted value account's history, in chronological
/// order. Each is one leaf, decodable from the host instruction that sealed it with no prior
/// state: a write names the handle it installs and the keys it allows on it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EncryptedValueAccountEvent {
    /// One `allow` of `key` on `handle`, sealed by the write that installed `handle`.
    Allowed { handle: [u8; 32], key: [u8; 32] },
    /// `handle` was made publicly decryptable.
    MarkedPublic { handle: [u8; 32] },
}

/// The full ordered leaf list of an encrypted value account plus the MMR state it implies.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReconstructedEncryptedValueAccount {
    pub leaves: Vec<[u8; 32]>,
    pub leaf_count: u64,
    pub peaks: Vec<[u8; 32]>,
}

/// Rebuilds the full ordered leaf list from an account's chronological events.
///
/// Mirrors the host program's append order exactly: one commitment per event, the leaf index
/// bound into each from a single running counter — exactly as the on-chain handler uses
/// `leaf_count` before each append — so indices can never desynchronize from event order.
pub fn reconstruct(
    encrypted_value_account: [u8; 32],
    events: &[EncryptedValueAccountEvent],
) -> ReconstructedEncryptedValueAccount {
    let leaves: Vec<[u8; 32]> = events
        .iter()
        .zip(0u64..)
        .map(|(event, leaf_index)| match event {
            EncryptedValueAccountEvent::Allowed { handle, key } => {
                historical_access_leaf_commitment(
                    encrypted_value_account,
                    leaf_index,
                    *handle,
                    *key,
                )
            }
            EncryptedValueAccountEvent::MarkedPublic { handle } => {
                public_decrypt_leaf_commitment(encrypted_value_account, leaf_index, *handle)
            }
        })
        .collect();
    let peaks = mmr_peaks_from_leaves(&leaves);
    ReconstructedEncryptedValueAccount {
        leaf_count: leaves.len() as u64,
        leaves,
        peaks,
    }
}

impl ReconstructedEncryptedValueAccount {
    /// Builds the inclusion proof for the leaf at `leaf_index`, or `None` if out of range.
    pub fn build_proof(&self, leaf_index: u64) -> Option<MmrProof> {
        mmr_build_proof(&self.leaves, leaf_index)
    }

    /// Cross-checks the reconstruction against the on-chain `(peaks, leaf_count)`:
    /// a missed or reordered event yields a different leaf list whose peaks diverge.
    pub fn peaks_match(&self, on_chain_peaks: &[[u8; 32]], on_chain_leaf_count: u64) -> bool {
        self.leaf_count == on_chain_leaf_count && self.peaks == on_chain_peaks
    }

    /// Builds a proof only after confirming the reconstruction matches chain state, so a
    /// wrong or incomplete record surfaces as [`EncryptedValueAccountError::PeaksDiverged`]
    /// here rather than as a silent KMS rejection later.
    pub fn build_verified_proof(
        &self,
        on_chain_peaks: &[[u8; 32]],
        on_chain_leaf_count: u64,
        leaf_index: u64,
    ) -> Result<MmrProof, EncryptedValueAccountError> {
        if !self.peaks_match(on_chain_peaks, on_chain_leaf_count) {
            return Err(EncryptedValueAccountError::PeaksDiverged);
        }
        self.build_proof(leaf_index)
            .ok_or(EncryptedValueAccountError::LeafIndexOutOfRange)
    }
}

/// One-shot reconstruction + proof build for the leaf at `leaf_index`. Does NOT cross-check
/// against chain state; for that use [`build_verified_proof_from_events`].
pub fn build_proof_from_events(
    encrypted_value_account: [u8; 32],
    events: &[EncryptedValueAccountEvent],
    leaf_index: u64,
) -> Option<MmrProof> {
    reconstruct(encrypted_value_account, events).build_proof(leaf_index)
}

/// One-shot reconstruction + chain-verified proof build for the leaf at `leaf_index`.
/// See [`ReconstructedEncryptedValueAccount::build_verified_proof`].
pub fn build_verified_proof_from_events(
    encrypted_value_account: [u8; 32],
    events: &[EncryptedValueAccountEvent],
    on_chain_peaks: &[[u8; 32]],
    on_chain_leaf_count: u64,
    leaf_index: u64,
) -> Result<MmrProof, EncryptedValueAccountError> {
    reconstruct(encrypted_value_account, events).build_verified_proof(
        on_chain_peaks,
        on_chain_leaf_count,
        leaf_index,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{authorize_historical, authorize_public, mmr_append, mmr_verify, EncryptedValue};
    use EncryptedValueAccountEvent::{Allowed, MarkedPublic};

    fn h(tag: u8) -> [u8; 32] {
        [tag; 32]
    }

    /// Recomputes peaks by an independent append loop over the same leaves.
    fn peaks_via_append(leaves: &[[u8; 32]]) -> (Vec<[u8; 32]>, u64) {
        let mut peaks = Vec::new();
        let mut count = 0u64;
        for leaf in leaves {
            mmr_append(&mut peaks, &mut count, *leaf).unwrap();
        }
        (peaks, count)
    }

    fn assert_every_leaf_proves(account: &ReconstructedEncryptedValueAccount) {
        for i in 0..account.leaf_count {
            let proof = account.build_proof(i).unwrap();
            assert!(mmr_verify(
                &account.peaks,
                account.leaf_count,
                account.leaves[i as usize],
                &proof
            ));
        }
        assert!(account.build_proof(account.leaf_count).is_none());
    }

    #[test]
    fn empty_events_produce_no_leaves() {
        let acct = h(0xAC);
        let account = reconstruct(acct, &[]);
        assert!(account.leaves.is_empty());
        assert_eq!(account.leaf_count, 0);
        assert!(account.peaks.is_empty());
        assert!(build_proof_from_events(acct, &[], 0).is_none());
        assert!(account.peaks_match(&[], 0));
    }

    /// Create with two allows, an update that allows one key and goes public, then an update
    /// with no allows at all (legal: nobody can decrypt that handle). Pins the leaf indices
    /// and the "one event, one leaf, in order" rule; swapping two allows changes the leaves.
    #[test]
    fn realistic_sequence_indices_and_proofs() {
        let acct = h(0xAC);
        let events = [
            Allowed {
                handle: h(10),
                key: h(1),
            },
            Allowed {
                handle: h(10),
                key: h(2),
            },
            Allowed {
                handle: h(11),
                key: h(1),
            },
            MarkedPublic { handle: h(11) },
        ];
        let account = reconstruct(acct, &events);
        assert_eq!(
            account.leaves,
            vec![
                historical_access_leaf_commitment(acct, 0, h(10), h(1)),
                historical_access_leaf_commitment(acct, 1, h(10), h(2)),
                historical_access_leaf_commitment(acct, 2, h(11), h(1)),
                public_decrypt_leaf_commitment(acct, 3, h(11)),
            ]
        );
        let (peaks, count) = peaks_via_append(&account.leaves);
        assert!(account.peaks_match(&peaks, count));
        assert_every_leaf_proves(&account);

        let mut swapped = events.clone();
        swapped.swap(0, 1);
        assert_ne!(reconstruct(acct, &swapped).leaves, account.leaves);
    }

    /// A dropped event is the realistic divergence: the shorter record's peaks fail to match
    /// the chain's, and `build_verified_proof` refuses before handing back a doomed proof.
    #[test]
    fn build_verified_proof_guards_divergence() {
        let acct = h(0xAC);
        let events = [
            Allowed {
                handle: h(10),
                key: h(1),
            },
            Allowed {
                handle: h(10),
                key: h(2),
            },
        ];
        let account = reconstruct(acct, &events);
        let (peaks, count) = peaks_via_append(&account.leaves);

        let proof = account.build_verified_proof(&peaks, count, 0).unwrap();
        assert!(mmr_verify(&peaks, count, account.leaves[0], &proof));

        assert_eq!(
            account.build_verified_proof(&peaks, count + 1, 0),
            Err(EncryptedValueAccountError::PeaksDiverged)
        );
        let mut tampered = peaks.clone();
        tampered[0][0] ^= 0xff;
        assert_eq!(
            account.build_verified_proof(&tampered, count, 0),
            Err(EncryptedValueAccountError::PeaksDiverged)
        );
        assert_eq!(
            account.build_verified_proof(&peaks, count, 2),
            Err(EncryptedValueAccountError::LeafIndexOutOfRange)
        );

        let missing_one = [events[0].clone()];
        assert_eq!(
            build_verified_proof_from_events(acct, &missing_one, &peaks, count, 0),
            Err(EncryptedValueAccountError::PeaksDiverged)
        );
        assert!(build_verified_proof_from_events(acct, &events, &peaks, count, 0).is_ok());
    }

    /// Sixteen allows spanning several MMR mountains, then one more so the peak set is
    /// irregular: build and verify a proof for every index on real commitment values.
    #[test]
    fn large_multi_mountain_value_account_round_trips() {
        let acct = h(0xAC);
        let events: Vec<_> = (0..17u8)
            .map(|i| Allowed {
                handle: h(10 + i / 8),
                key: h(0x20 + i),
            })
            .collect();
        let account = reconstruct(acct, &events);
        assert_eq!(account.leaf_count, 17);
        assert_eq!(account.peaks.len(), 2);
        let (peaks, count) = peaks_via_append(&account.leaves);
        assert!(account.peaks_match(&peaks, count));
        assert_every_leaf_proves(&account);
    }

    /// A proof built off the reconstruction authorizes against an `EncryptedValue` carrying
    /// only the reconstructed peaks and leaf count (what the chain stores); the account key is
    /// bound into every leaf, so the same events under another account never cross-authorize.
    #[test]
    fn matches_on_chain_append_and_authorizes() {
        let acct = h(0xAC);
        let owner = h(1);
        let events = [
            Allowed {
                handle: h(10),
                key: owner,
            },
            Allowed {
                handle: h(10),
                key: h(2),
            },
        ];
        let account = reconstruct(acct, &events);
        let value = EncryptedValue {
            current_handle: h(11),
            leaf_count: account.leaf_count,
            peaks: account.peaks.clone(),
            ..Default::default()
        };
        let proof = account.build_proof(0).unwrap();
        assert!(authorize_historical(acct, &value, h(10), owner, &proof).is_ok());
        assert!(authorize_historical(acct, &value, h(10), h(2), &proof).is_err());
        assert!(authorize_historical(acct, &value, h(11), owner, &proof).is_err());

        let other = reconstruct(h(0xBB), &events);
        assert_ne!(other.peaks, account.peaks);
        let other_proof = other.build_proof(0).unwrap();
        assert!(authorize_historical(acct, &value, h(10), owner, &other_proof).is_err());

        let pub_events = [MarkedPublic { handle: h(10) }];
        let pub_account = reconstruct(acct, &pub_events);
        let pub_value = EncryptedValue {
            leaf_count: pub_account.leaf_count,
            peaks: pub_account.peaks.clone(),
            ..Default::default()
        };
        let pub_proof = build_proof_from_events(acct, &pub_events, 0).unwrap();
        assert!(authorize_public(acct, &pub_value, h(10), &pub_proof).is_ok());
        assert!(authorize_public(acct, &pub_value, h(11), &pub_proof).is_err());
    }
}
