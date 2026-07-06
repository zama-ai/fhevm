//! Off-chain reconstruction of an encrypted-value lineage's full leaf list.
//!
//! The on-chain account stores only the MMR peaks and leaf count, never the
//! ordered leaves a historical/public-decrypt proof needs. This module rebuilds
//! that leaf list from a lineage's chronological update/make-public record and
//! builds inclusion proofs from it, reusing the shared leaf commitments and MMR
//! exactly as the host program appends them — so a reconstructed lineage's peaks
//! match the chain byte-for-byte.
//!
//! Pure data transform: no I/O, no async, no chain access. Instruction ingestion
//! and storage are a separate, later phase; here the caller supplies the events.

use crate::{
    historical_access_leaf_commitment, mmr_build_proof, mmr_peaks_from_leaves,
    public_decrypt_leaf_commitment, MmrProof,
};

/// Why a reconstruction or proof-build could not be trusted against chain state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineageError {
    /// No longer emitted by reconstruction; kept so older callers that matched
    /// this public error variant continue to compile.
    EmptySupersededSubjects,
    /// The reconstructed `(peaks, leaf_count)` diverge from the on-chain account's.
    /// The record is incomplete, reordered, or carries wrong subject snapshots;
    /// any proof built from it would be rejected by the KMS at verify time.
    PeaksDiverged,
    /// `leaf_index` is outside the reconstructed leaf list.
    LeafIndexOutOfRange,
}

/// One leaf-appending operation in a lineage's history, in chronological order.
///
/// Only the two instructions that append MMR leaves appear here;
/// `allow_subjects` appends none, so it has no leaf to log — its effect on
/// membership is captured by the next [`LineageEvent::HandleSuperseded`]'s
/// `previous_subjects` snapshot. Because `update_encrypted_value` carries
/// `previous_handle`/`previous_subjects` as verified instruction args, both
/// variants are decodable from a single transaction with no prior state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LineageEvent {
    /// An `update_encrypted_value`: appends one historical-access leaf per
    /// subject of the outgoing handle, in `previous_subjects` order.
    HandleSuperseded {
        /// The handle being superseded (the lineage's handle before this update).
        previous_handle: [u8; 32],
        /// The exact `subjects` snapshot (order preserved, index 0 first) as it
        /// stood immediately before this update executed, after any prior
        /// `allow_subjects`. Callers must NOT sort or dedup it.
        previous_subjects: Vec<[u8; 32]>,
    },
    /// A `make_handle_public`: appends one public-decrypt leaf for `handle`.
    MarkedPublic {
        /// The lineage's current handle at the time it was made public.
        handle: [u8; 32],
    },
}

impl LineageEvent {
    /// Builds a [`LineageEvent::HandleSuperseded`] from the `subjects` snapshot
    /// taken immediately before the update executed on-chain.
    ///
    /// `previous_subjects` is load-bearing and silent to get wrong: it must be
    /// the live `subjects` in insertion order, including every subject added by
    /// prior `allow_subjects` calls. A stale snapshot (e.g. the pre-`allow` set,
    /// or the post-update set) yields leaves that hash differently from the
    /// chain, so the reconstructed peaks silently diverge. When decoding
    /// `update_encrypted_value` instructions, use its verified args directly.
    pub fn handle_superseded(previous_handle: [u8; 32], previous_subjects: &[[u8; 32]]) -> Self {
        LineageEvent::HandleSuperseded {
            previous_handle,
            previous_subjects: previous_subjects.to_vec(),
        }
    }
}

/// The full ordered leaf list of a lineage plus the MMR state it implies.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReconstructedLineage {
    pub leaves: Vec<[u8; 32]>,
    pub leaf_count: u64,
    pub peaks: Vec<[u8; 32]>,
}

/// Rebuilds the full ordered leaf list from a lineage's chronological events.
///
/// Mirrors the host program's append order exactly: each
/// [`LineageEvent::HandleSuperseded`] appends one
/// `historical_access_leaf_commitment` per subject in slice order
/// (`update_encrypted_value`), each [`LineageEvent::MarkedPublic`] appends one
/// `public_decrypt_leaf_commitment` (`make_handle_public`). The leaf index bound
/// into every commitment comes from a single running counter — the
/// authoritative source, exactly as the on-chain handler uses `leaf_count`
/// before each append — so indices can never desynchronize from event order.
///
pub fn reconstruct(
    encrypted_value_account: [u8; 32],
    events: &[LineageEvent],
) -> Result<ReconstructedLineage, LineageError> {
    let mut leaves = Vec::new();
    let mut leaf_count: u64 = 0;
    for event in events {
        match event {
            LineageEvent::HandleSuperseded {
                previous_handle,
                previous_subjects,
            } => {
                for subject in previous_subjects {
                    leaves.push(historical_access_leaf_commitment(
                        encrypted_value_account,
                        leaf_count,
                        *previous_handle,
                        *subject,
                    ));
                    leaf_count += 1;
                }
            }
            LineageEvent::MarkedPublic { handle } => {
                leaves.push(public_decrypt_leaf_commitment(
                    encrypted_value_account,
                    leaf_count,
                    *handle,
                ));
                leaf_count += 1;
            }
        }
    }
    let peaks = mmr_peaks_from_leaves(&leaves);
    Ok(ReconstructedLineage {
        leaves,
        leaf_count,
        peaks,
    })
}

impl ReconstructedLineage {
    /// Builds the inclusion proof for the leaf at `leaf_index`, or `None` if out of range.
    pub fn build_proof(&self, leaf_index: u64) -> Option<MmrProof> {
        mmr_build_proof(&self.leaves, leaf_index)
    }

    /// Cross-checks the reconstruction against the on-chain `(peaks, leaf_count)`:
    /// a missed or reordered event yields a different leaf list whose peaks diverge.
    pub fn peaks_match(&self, on_chain_peaks: &[[u8; 32]], on_chain_leaf_count: u64) -> bool {
        self.leaf_count == on_chain_leaf_count && self.peaks == on_chain_peaks
    }

    /// Builds a proof only after confirming the reconstruction matches chain state.
    ///
    /// Guards the silent-divergence footgun: a wrong/incomplete record, or a
    /// stale `previous_subjects` snapshot, produces a proof that is internally
    /// self-consistent but is rejected by the KMS against the real on-chain
    /// peaks. Pass the account's stored `(peaks, leaf_count)` and this returns
    /// [`LineageError::PeaksDiverged`] before handing back a doomed proof.
    pub fn build_verified_proof(
        &self,
        on_chain_peaks: &[[u8; 32]],
        on_chain_leaf_count: u64,
        leaf_index: u64,
    ) -> Result<MmrProof, LineageError> {
        if !self.peaks_match(on_chain_peaks, on_chain_leaf_count) {
            return Err(LineageError::PeaksDiverged);
        }
        self.build_proof(leaf_index)
            .ok_or(LineageError::LeafIndexOutOfRange)
    }
}

/// One-shot reconstruction + proof build for the leaf at `leaf_index`.
///
/// Reconstructs the full leaf list on every call. For several proofs on the same
/// lineage (e.g. a batch after one update) call [`reconstruct`] once and reuse
/// the returned [`ReconstructedLineage`] instead. This convenience does NOT
/// cross-check against chain state; for that use [`build_verified_proof_from_events`].
pub fn build_proof_from_events(
    encrypted_value_account: [u8; 32],
    events: &[LineageEvent],
    leaf_index: u64,
) -> Result<Option<MmrProof>, LineageError> {
    Ok(reconstruct(encrypted_value_account, events)?.build_proof(leaf_index))
}

/// One-shot reconstruction + chain-verified proof build for the leaf at `leaf_index`.
///
/// Like [`build_proof_from_events`] but cross-checks the reconstructed
/// `(peaks, leaf_count)` against the on-chain account's before returning a proof,
/// so a wrong record surfaces as [`LineageError::PeaksDiverged`] here rather
/// than as a silent KMS rejection later. See [`ReconstructedLineage::build_verified_proof`].
pub fn build_verified_proof_from_events(
    encrypted_value_account: [u8; 32],
    events: &[LineageEvent],
    on_chain_peaks: &[[u8; 32]],
    on_chain_leaf_count: u64,
    leaf_index: u64,
) -> Result<MmrProof, LineageError> {
    reconstruct(encrypted_value_account, events)?.build_verified_proof(
        on_chain_peaks,
        on_chain_leaf_count,
        leaf_index,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        authorize_historical, authorize_public, mmr_append, mmr_verify, EncryptedValue,
        MAX_ENCRYPTED_VALUE_SUBJECTS,
    };

    fn h(tag: u8) -> [u8; 32] {
        [tag; 32]
    }

    fn superseded(previous_handle: [u8; 32], subjects: &[[u8; 32]]) -> LineageEvent {
        LineageEvent::handle_superseded(previous_handle, subjects)
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

    #[test]
    fn empty_events_produce_no_leaves() {
        let acct = h(0xAC);
        let lineage = reconstruct(acct, &[]).unwrap();
        assert!(lineage.leaves.is_empty());
        assert_eq!(lineage.leaf_count, 0);
        assert!(lineage.peaks.is_empty());
        assert!(build_proof_from_events(acct, &[], 0).unwrap().is_none());
        assert!(lineage.peaks_match(&[], 0));
    }

    #[test]
    fn single_update_one_subject() {
        let acct = h(0xAC);
        let events = [superseded(h(10), &[h(1)])];
        let lineage = reconstruct(acct, &events).unwrap();
        assert_eq!(
            lineage.leaves,
            vec![historical_access_leaf_commitment(acct, 0, h(10), h(1))]
        );
        let proof = lineage.build_proof(0).unwrap();
        assert!(mmr_verify(
            &lineage.peaks,
            lineage.leaf_count,
            lineage.leaves[0],
            &proof
        ));
    }

    #[test]
    fn update_two_subjects_keeps_order() {
        let acct = h(0xAC);
        let events = [superseded(h(10), &[h(1), h(2)])];
        let lineage = reconstruct(acct, &events).unwrap();
        assert_eq!(
            lineage.leaves,
            vec![
                historical_access_leaf_commitment(acct, 0, h(10), h(1)),
                historical_access_leaf_commitment(acct, 1, h(10), h(2)),
            ]
        );
        for i in 0..2 {
            let proof = lineage.build_proof(i).unwrap();
            assert!(mmr_verify(
                &lineage.peaks,
                lineage.leaf_count,
                lineage.leaves[i as usize],
                &proof
            ));
        }
        // Subject order is load-bearing: swapping it yields different leaves.
        let swapped = reconstruct(acct, &[superseded(h(10), &[h(2), h(1)])]).unwrap();
        assert_ne!(lineage.leaves, swapped.leaves);
    }

    #[test]
    fn single_make_public() {
        let acct = h(0xAC);
        let events = [LineageEvent::MarkedPublic { handle: h(10) }];
        let lineage = reconstruct(acct, &events).unwrap();
        assert_eq!(
            lineage.leaves,
            vec![public_decrypt_leaf_commitment(acct, 0, h(10))]
        );
        let proof = lineage.build_proof(0).unwrap();
        assert!(mmr_verify(
            &lineage.peaks,
            lineage.leaf_count,
            lineage.leaves[0],
            &proof
        ));
    }

    #[test]
    fn realistic_sequence_indices_and_proofs() {
        let acct = h(0xAC);
        let events = [
            LineageEvent::MarkedPublic { handle: h(10) },
            superseded(h(10), &[h(1), h(2)]),
            LineageEvent::MarkedPublic { handle: h(11) },
        ];
        let lineage = reconstruct(acct, &events).unwrap();
        assert_eq!(
            lineage.leaves,
            vec![
                public_decrypt_leaf_commitment(acct, 0, h(10)),
                historical_access_leaf_commitment(acct, 1, h(10), h(1)),
                historical_access_leaf_commitment(acct, 2, h(10), h(2)),
                public_decrypt_leaf_commitment(acct, 3, h(11)),
            ]
        );
        assert_eq!(lineage.leaf_count, 4);
        for i in 0..4 {
            let proof = lineage.build_proof(i).unwrap();
            assert!(mmr_verify(
                &lineage.peaks,
                lineage.leaf_count,
                lineage.leaves[i as usize],
                &proof
            ));
        }
        assert!(lineage.build_proof(4).is_none());
    }

    /// Two updates back-to-back, with no `MarkedPublic` between them and a
    /// growing subject set (1 then 3). Pins the second update's leaf indices,
    /// which must continue from where the first left off (`leaf_count == 1`), so a
    /// regression that reset or miscounted the running counter would be caught.
    #[test]
    fn two_consecutive_updates_continue_leaf_indices() {
        let acct = h(0xAC);
        let events = [
            superseded(h(10), &[h(1)]),
            superseded(h(11), &[h(1), h(2), h(3)]),
        ];
        let lineage = reconstruct(acct, &events).unwrap();
        assert_eq!(
            lineage.leaves,
            vec![
                historical_access_leaf_commitment(acct, 0, h(10), h(1)),
                historical_access_leaf_commitment(acct, 1, h(11), h(1)),
                historical_access_leaf_commitment(acct, 2, h(11), h(2)),
                historical_access_leaf_commitment(acct, 3, h(11), h(3)),
            ]
        );
        assert_eq!(lineage.leaf_count, 4);
        let (peaks, count) = peaks_via_append(&lineage.leaves);
        assert!(lineage.peaks_match(&peaks, count));
        for i in 0..4 {
            let proof = lineage.build_proof(i).unwrap();
            assert!(mmr_verify(
                &lineage.peaks,
                lineage.leaf_count,
                lineage.leaves[i as usize],
                &proof
            ));
        }
    }

    /// On-chain lifecycle `create_encrypted_value([s1])` → `allow_subjects([s2])`
    /// → `update_encrypted_value`: the supersession snapshot must be the
    /// post-`allow` set `[s1, s2]`, so the chain appends two leaves. Supplying the
    /// stale pre-`allow` set `[s1]` is the most plausible ingestion bug; this
    /// asserts it produces different peaks and the correct snapshot's peaks match
    /// an independent append.
    #[test]
    fn allow_subjects_grows_next_update_snapshot() {
        let acct = h(0xAC);
        let s1 = h(1);
        let s2 = h(2);

        let correct = reconstruct(acct, &[superseded(h(10), &[s1, s2])]).unwrap();
        assert_eq!(
            correct.leaves,
            vec![
                historical_access_leaf_commitment(acct, 0, h(10), s1),
                historical_access_leaf_commitment(acct, 1, h(10), s2),
            ]
        );
        let (peaks, count) = peaks_via_append(&correct.leaves);
        assert!(correct.peaks_match(&peaks, count));

        // The stale pre-`allow` snapshot omits s2: one leaf instead of two, so the
        // peaks diverge from the chain's and no proof would verify.
        let stale = reconstruct(acct, &[superseded(h(10), &[s1])]).unwrap();
        assert_ne!(correct.peaks, stale.peaks);
        assert!(!stale.peaks_match(&peaks, count));
    }

    #[test]
    fn empty_superseded_subjects_append_no_leaves() {
        let acct = h(0xAC);
        let events = [
            LineageEvent::MarkedPublic { handle: h(9) },
            superseded(h(10), &[]),
            LineageEvent::MarkedPublic { handle: h(11) },
        ];
        let lineage = reconstruct(acct, &events).unwrap();
        let expected = vec![
            public_decrypt_leaf_commitment(acct, 0, h(9)),
            public_decrypt_leaf_commitment(acct, 1, h(11)),
        ];
        assert_eq!(lineage.leaves, expected);
        let (peaks, count) = peaks_via_append(&lineage.leaves);
        assert!(lineage.peaks_match(&peaks, count));
    }

    #[test]
    fn peaks_invariant_and_match() {
        let acct = h(0xAC);
        let events = [
            LineageEvent::MarkedPublic { handle: h(10) },
            superseded(h(10), &[h(1), h(2)]),
            LineageEvent::MarkedPublic { handle: h(11) },
        ];
        let lineage = reconstruct(acct, &events).unwrap();
        let (peaks, count) = peaks_via_append(&lineage.leaves);
        assert_eq!(lineage.peaks, peaks);
        assert_eq!(lineage.leaf_count, count);
        assert!(lineage.peaks_match(&peaks, count));

        let mut tampered = peaks.clone();
        tampered[0][0] ^= 0xff;
        assert!(!lineage.peaks_match(&tampered, count));
        assert!(!lineage.peaks_match(&peaks, count + 1));

        // A dropped event is the realistic divergence: reconstruct the same
        // history with the middle update omitted and confirm its shorter leaf
        // list fails to match the full reconstruction's peaks/count.
        let missing = reconstruct(
            acct,
            &[
                LineageEvent::MarkedPublic { handle: h(10) },
                LineageEvent::MarkedPublic { handle: h(11) },
            ],
        )
        .unwrap();
        assert!(!missing.peaks_match(&lineage.peaks, lineage.leaf_count));
    }

    /// `build_verified_proof` refuses to hand back a proof when the reconstruction
    /// diverges from chain state, turning an otherwise-silent KMS rejection into a
    /// local error at the library boundary.
    #[test]
    fn build_verified_proof_guards_divergence() {
        let acct = h(0xAC);
        let events = [superseded(h(10), &[h(1), h(2)])];
        let lineage = reconstruct(acct, &events).unwrap();
        let (peaks, count) = peaks_via_append(&lineage.leaves);

        // Matching chain state: proof is returned and verifies.
        let proof = lineage.build_verified_proof(&peaks, count, 0).unwrap();
        assert!(mmr_verify(&peaks, count, lineage.leaves[0], &proof));

        // Wrong leaf_count and wrong peaks each surface as PeaksDiverged before any
        // proof is built.
        assert_eq!(
            lineage.build_verified_proof(&peaks, count + 1, 0),
            Err(LineageError::PeaksDiverged)
        );
        let mut tampered = peaks.clone();
        tampered[0][0] ^= 0xff;
        assert_eq!(
            lineage.build_verified_proof(&tampered, count, 0),
            Err(LineageError::PeaksDiverged)
        );

        // Out-of-range index on a matching reconstruction is its own error.
        assert_eq!(
            lineage.build_verified_proof(&peaks, count, 2),
            Err(LineageError::LeafIndexOutOfRange)
        );

        // One-shot variant catches a divergent record (stale subject snapshot)
        // against real on-chain peaks.
        let stale = [superseded(h(10), &[h(1)])];
        assert_eq!(
            build_verified_proof_from_events(acct, &stale, &peaks, count, 0),
            Err(LineageError::PeaksDiverged)
        );
        assert!(build_verified_proof_from_events(acct, &events, &peaks, count, 0).is_ok());
    }

    /// Two updates of `MAX_ENCRYPTED_VALUE_SUBJECTS` subjects each yield 16
    /// domain-separated leaves spanning multiple MMR mountains; build and verify a
    /// proof for every index to exercise mountain selection end-to-end on real
    /// commitment values rather than synthetic bytes.
    #[test]
    fn large_multi_mountain_lineage_round_trips() {
        let acct = h(0xAC);
        let subjects: Vec<[u8; 32]> = (0..MAX_ENCRYPTED_VALUE_SUBJECTS as u8)
            .map(|i| h(0x20 + i))
            .collect();
        let events = [superseded(h(10), &subjects), superseded(h(11), &subjects)];
        let lineage = reconstruct(acct, &events).unwrap();
        assert_eq!(lineage.leaf_count, 16);
        assert_eq!(lineage.leaves.len(), 16);
        let (peaks, count) = peaks_via_append(&lineage.leaves);
        assert!(lineage.peaks_match(&peaks, count));
        for i in 0..16 {
            let proof = lineage.build_proof(i).unwrap();
            assert!(mmr_verify(
                &lineage.peaks,
                lineage.leaf_count,
                lineage.leaves[i as usize],
                &proof
            ));
        }
    }

    /// The private `Lineage` helper in `lib.rs` tests is the already-trusted
    /// mirror of the on-chain append order; this reproduces its
    /// `post_update_then_historical_proof` sequence and checks the new public
    /// API reconstructs the identical leaves/peaks, then round-trips a built
    /// proof through `authorize_historical`/`authorize_public` against an
    /// account carrying the reconstructed peaks and leaf count.
    #[test]
    fn matches_on_chain_append_and_authorizes() {
        // Mirror `Lineage::new(h(10), &[owner])` then `update(h(11))`: the account
        // tag is `h(0xAC)`, the supersession logs the previous subjects for h(10).
        let acct = h(0xAC);
        let owner = h(1);
        let events = [superseded(h(10), &[owner])];
        let lineage = reconstruct(acct, &events).unwrap();

        // Independent on-chain-style append over the same single leaf.
        let leaf = historical_access_leaf_commitment(acct, 0, h(10), owner);
        let (peaks, count) = peaks_via_append(&[leaf]);
        assert_eq!(lineage.leaves, vec![leaf]);
        assert_eq!(lineage.peaks, peaks);
        assert_eq!(lineage.leaf_count, count);

        // Build an EncryptedValue carrying only the reconstructed peaks/leaf_count
        // (what the chain stores) and authorize against a proof built off the leaf list.
        let value = EncryptedValue {
            current_handle: h(11),
            subjects: vec![owner],
            leaf_count: lineage.leaf_count,
            peaks: lineage.peaks.clone(),
            ..Default::default()
        };
        let proof = lineage.build_proof(0).unwrap();
        assert!(authorize_historical(acct, &value, h(10), owner, &proof).is_ok());
        assert!(authorize_historical(acct, &value, h(10), h(2), &proof).is_err());

        // Cross-account isolation: the account key is bound into every leaf
        // commitment, so the same events under a different account yield different
        // peaks, and a proof built under account 2 is rejected by an account
        // carrying account 1's peaks. Use a two-subject supersession so the proof
        // carries a real sibling that differs between accounts (a single-leaf MMR
        // proof is empty and would not distinguish the accounts on its own).
        let two = [superseded(h(10), &[owner, h(2)])];
        let lin1 = reconstruct(acct, &two).unwrap();
        let acct2 = h(0xBB);
        let lin2 = reconstruct(acct2, &two).unwrap();
        assert_ne!(lin1.peaks, lin2.peaks);
        let value1 = EncryptedValue {
            leaf_count: lin1.leaf_count,
            peaks: lin1.peaks.clone(),
            ..Default::default()
        };
        // The matching account/peaks authorize; the cross-account proof does not.
        let proof1 = lin1.build_proof(0).unwrap();
        assert!(authorize_historical(acct, &value1, h(10), owner, &proof1).is_ok());
        let proof2 = lin2.build_proof(0).unwrap();
        assert!(authorize_historical(acct, &value1, h(10), owner, &proof2).is_err());

        // A public-decrypt lineage round-trips through `authorize_public`.
        let pub_events = [LineageEvent::MarkedPublic { handle: h(10) }];
        let pub_lineage = reconstruct(acct, &pub_events).unwrap();
        let pub_value = EncryptedValue {
            leaf_count: pub_lineage.leaf_count,
            peaks: pub_lineage.peaks.clone(),
            ..Default::default()
        };
        let pub_proof = build_proof_from_events(acct, &pub_events, 0)
            .unwrap()
            .unwrap();
        assert!(authorize_public(acct, &pub_value, h(10), &pub_proof).is_ok());
        assert!(authorize_public(acct, &pub_value, h(11), &pub_proof).is_err());
    }
}
