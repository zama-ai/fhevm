//! Incremental consensus tracking over one fan-out round.
//!
//! [`ConsensusTracker`] turns "N validated replies out of M registered Coprocessors" into a
//! definite verdict after every reply, so a caller's retry-vs-give-up decision is driven by
//! tracker state instead of a raw reply count. See [`ConsensusStatus`] for the verdict shape
//! and why `Starved` and `Split` must stay distinct.

use crate::consensus::{Consensus, ConsensusMaterial};
use crate::{AttestationError, CiphertextAttestation};
use alloy_primitives::{Address, B256, U256};
use std::{
    collections::{HashMap, HashSet},
    num::NonZeroUsize,
};

/// An attestation that passed both gates for the specific bucket that served it.
#[derive(Clone, Debug)]
pub struct ValidAttestation {
    material: ConsensusMaterial,
    signer: Address,
}

impl ValidAttestation {
    /// Gate 1: signature recovers to the embedded signer ([`CiphertextAttestation::verify`]).
    /// Gate 2: that embedded signer equals the signer this bucket is registered to on-chain —
    /// without this gate a bucket could serve another bucket's (validly signed) attestation and
    /// have it counted as its own.
    pub fn validate(
        attestation: &CiphertextAttestation,
        handle: B256,
        coprocessor_context_id: U256,
        registered_signer: Address,
    ) -> Result<Self, ValidationError> {
        attestation.verify(handle, coprocessor_context_id)?;
        if attestation.signer != registered_signer {
            return Err(ValidationError::SignerNotRegisteredForBucket {
                embedded: attestation.signer,
                registered: registered_signer,
            });
        }
        Ok(Self {
            material: ConsensusMaterial::from(attestation),
            signer: attestation.signer,
        })
    }

    pub fn signer(&self) -> Address {
        self.signer
    }

    pub fn material(&self) -> &ConsensusMaterial {
        &self.material
    }
}

/// Why [`ValidAttestation::validate`] rejected an attestation.
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error(transparent)]
    Signature(#[from] AttestationError),
    /// The signature is genuine, but for a different bucket's key — cross-serving.
    #[error("signer {embedded} is not the registered signer {registered} for this bucket")]
    SignerNotRegisteredForBucket {
        embedded: Address,
        registered: Address,
    },
}

/// Incremental consensus over one fan-out round. No network, no time, no I/O: the caller feeds
/// it exactly one event per registered Coprocessor — a vote or a failure — and reads the
/// verdict back after each one.
///
/// Duplicates are unrepresentable by construction: one bucket maps to one registered signer, so
/// one vote per bucket is one vote per signer, and `GatewayConfig` enforces signer uniqueness
/// on-chain. Group membership is a `HashSet<Address>` only because that is the natural shape
/// for [`Consensus::signers`], not because dedup logic is needed here.
pub struct ConsensusTracker {
    coprocessor_count: usize,
    threshold: NonZeroUsize,
    votes: usize,
    failures: usize,
    groups: HashMap<ConsensusMaterial, HashSet<Address>>,
}

impl ConsensusTracker {
    /// `coprocessor_count` is the number of registered Coprocessors polled this round; the
    /// tracker expects exactly one [`Self::add_vote`] or [`Self::record_failure`] call per
    /// Coprocessor.
    pub fn new(coprocessor_count: usize, threshold: NonZeroUsize) -> Self {
        Self {
            coprocessor_count,
            threshold,
            votes: 0,
            failures: 0,
            groups: HashMap::new(),
        }
    }

    /// Record a validated attestation and return the updated verdict.
    pub fn add_vote(&mut self, vote: ValidAttestation) -> ConsensusStatus {
        self.votes += 1;
        self.groups
            .entry(vote.material)
            .or_default()
            .insert(vote.signer);
        self.status()
    }

    /// Record any non-vote outcome for one Coprocessor: fetch error, timeout, HTTP error, a
    /// reply that failed [`ValidAttestation::validate`], or a panicked task. The tracker is
    /// kind-blind — all of these mean "this Coprocessor produced no usable signal this round",
    /// and that is all the arithmetic below needs.
    pub fn record_failure(&mut self) -> ConsensusStatus {
        self.failures += 1;
        self.status()
    }

    /// Recompute the verdict from the votes and failures recorded so far.
    pub fn status(&self) -> ConsensusStatus {
        let pending = self
            .coprocessor_count
            .saturating_sub(self.votes)
            .saturating_sub(self.failures);
        let leading = self.groups.iter().max_by(
            |(left_material, left_signers), (right_material, right_signers)| {
                left_signers
                    .len()
                    .cmp(&right_signers.len())
                    // Deterministic tie-break, mirroring `consensus::evaluate`; unreachable in
                    // practice since a single vote can grow at most one group past threshold.
                    .then_with(|| right_material.cmp(left_material))
            },
        );
        let largest = leading.map_or(0, |(_, signers)| signers.len());

        if largest >= self.threshold.get() {
            let (material, signers) = leading.expect("largest > 0 implies a leading group");
            return ConsensusStatus::Reached(Consensus {
                material: material.clone(),
                signers: signers.clone(),
            });
        }
        if largest + pending >= self.threshold.get() {
            return ConsensusStatus::Pending;
        }
        // No pending slots remain, so the round cannot improve — the only question is whether
        // that is a proven disagreement or an unresolved gap left by failures.
        if self.failures == 0 {
            return ConsensusStatus::Split {
                valid: self.votes,
                largest,
            };
        }
        // Terminal only when disagreement is proven. A round lost to failures says nothing
        // about agreement — attestations are published asynchronously, so the overwhelmingly
        // common cause is "not uploaded yet", which the caller's retry budget handles. Never
        // collapse this into `Split`: that would turn a retriable gap into a false terminal
        // verdict, which is exactly the misclassification this tracker exists to fix.
        ConsensusStatus::Starved {
            valid: self.votes,
            required: self.threshold.get(),
        }
    }
}

/// Verdict for one fan-out round, recomputed after every event.
#[derive(Debug)]
pub enum ConsensusStatus {
    /// A group of at least `threshold` distinct signers agreed on the same material.
    Reached(Consensus),
    /// Outstanding replies could still tip an undersized group over threshold.
    Pending,
    /// The round cannot reach threshold, but at least one Coprocessor produced no usable signal
    /// rather than actively disagreeing. Retriable: a re-poll may turn a failure into a vote.
    Starved { valid: usize, required: usize },
    /// Every registered Coprocessor answered validly and still no group reached the threshold.
    /// Terminal: re-reading the same objects returns the same disagreement.
    Split { valid: usize, largest: usize },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CiphertextAttestationPayload, CiphertextFormat, Version};
    use alloy_signer_local::PrivateKeySigner;

    const HANDLE: B256 = B256::repeat_byte(0xAA);
    const COPROCESSOR_CONTEXT_ID: U256 = U256::ONE;
    const KEY_ID: U256 = U256::from_limbs([0xdead_beef, 0, 0, 0]);
    const CT_DIGEST: B256 = B256::repeat_byte(0xBB);
    const SNS_DIGEST: B256 = B256::repeat_byte(0xCC);
    const FORMAT: CiphertextFormat = CiphertextFormat::UncompressedOnCpu;

    fn nz(threshold: usize) -> NonZeroUsize {
        NonZeroUsize::new(threshold).unwrap()
    }

    /// Signs a default-material attestation for `HANDLE`.
    async fn signed(signer: &PrivateKeySigner) -> CiphertextAttestation {
        CiphertextAttestationPayload::new(
            Version::V1,
            HANDLE,
            KEY_ID,
            COPROCESSOR_CONTEXT_ID,
            CT_DIGEST,
            SNS_DIGEST,
            FORMAT,
        )
        .sign(signer)
        .await
        .unwrap()
    }

    /// A validated vote from `signer`, as if its own bucket served its own attestation.
    async fn vote_from(signer: &PrivateKeySigner) -> ValidAttestation {
        let att = signed(signer).await;
        ValidAttestation::validate(&att, HANDLE, COPROCESSOR_CONTEXT_ID, signer.address()).unwrap()
    }

    #[tokio::test]
    async fn reaches_consensus_at_threshold() {
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let mut tracker = ConsensusTracker::new(2, nz(2));

        assert!(matches!(
            tracker.add_vote(vote_from(&s1).await),
            ConsensusStatus::Pending
        ));
        match tracker.add_vote(vote_from(&s2).await) {
            ConsensusStatus::Reached(consensus) => {
                assert_eq!(consensus.signers.len(), 2);
                assert_eq!(consensus.material.ciphertext_digest, CT_DIGEST);
            }
            other => panic!("expected Reached, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn pending_while_replies_outstanding() {
        let s1 = PrivateKeySigner::random();
        let mut tracker = ConsensusTracker::new(3, nz(2));

        let status = tracker.add_vote(vote_from(&s1).await);
        assert!(matches!(status, ConsensusStatus::Pending));
    }

    #[tokio::test]
    async fn starved_when_failures_make_round_unwinnable() {
        // 3 Coprocessors, threshold 2: one valid vote, two failures. The round cannot reach
        // threshold, but a failure occurred — this must be Starved, never Split.
        let s1 = PrivateKeySigner::random();
        let mut tracker = ConsensusTracker::new(3, nz(2));

        tracker.add_vote(vote_from(&s1).await);
        tracker.record_failure();
        let status = tracker.record_failure();

        match status {
            ConsensusStatus::Starved { valid, required } => {
                assert_eq!(valid, 1);
                assert_eq!(required, 2);
            }
            other => panic!("expected Starved, got {other:?}"),
        }
        assert!(!matches!(status, ConsensusStatus::Split { .. }));
    }

    #[tokio::test]
    async fn split_only_at_full_valid_participation() {
        // 2 Coprocessors, threshold 2: both answer validly but disagree. Full participation,
        // zero failures, no group reaches threshold — disagreement is proven.
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let mut tracker = ConsensusTracker::new(2, nz(2));

        tracker.add_vote(vote_from(&s1).await);
        let dissenting = CiphertextAttestationPayload::new(
            Version::V1,
            HANDLE,
            KEY_ID,
            COPROCESSOR_CONTEXT_ID,
            CT_DIGEST,
            B256::repeat_byte(0xDD),
            FORMAT,
        )
        .sign(&s2)
        .await
        .unwrap();
        let vote2 =
            ValidAttestation::validate(&dissenting, HANDLE, COPROCESSOR_CONTEXT_ID, s2.address())
                .unwrap();

        match tracker.add_vote(vote2) {
            ConsensusStatus::Split { valid, largest } => {
                assert_eq!(valid, 2);
                assert_eq!(largest, 1);
            }
            other => panic!("expected Split, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn reached_early_even_with_replies_outstanding() {
        // 5 Coprocessors, threshold 2: two agreeing votes reach consensus before the other
        // three have answered.
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let mut tracker = ConsensusTracker::new(5, nz(2));

        tracker.add_vote(vote_from(&s1).await);
        let status = tracker.add_vote(vote_from(&s2).await);

        assert!(matches!(status, ConsensusStatus::Reached(_)));
    }

    #[tokio::test]
    async fn validate_rejects_bad_signature() {
        let s1 = PrivateKeySigner::random();
        let mut att = signed(&s1).await;
        att.sns_ciphertext_digest = B256::repeat_byte(0xEE);

        let err = ValidAttestation::validate(&att, HANDLE, COPROCESSOR_CONTEXT_ID, s1.address())
            .unwrap_err();
        assert!(matches!(err, ValidationError::Signature(_)));
    }

    #[tokio::test]
    async fn validate_rejects_cross_served_attestation() {
        // Validly signed by s1, but served by a bucket registered to s2.
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let att = signed(&s1).await;

        let err = ValidAttestation::validate(&att, HANDLE, COPROCESSOR_CONTEXT_ID, s2.address())
            .unwrap_err();
        assert!(matches!(
            err,
            ValidationError::SignerNotRegisteredForBucket { embedded, registered }
                if embedded == s1.address() && registered == s2.address()
        ));
    }
}
