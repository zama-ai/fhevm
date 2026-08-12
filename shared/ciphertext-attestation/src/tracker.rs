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
/// The caller is expected not to produce duplicates — one bucket maps to one registered signer,
/// so one event per bucket is one event per signer, and `GatewayConfig` enforces signer uniqueness
/// on-chain. That expectation is nonetheless enforced here rather than assumed: participation is
/// tracked as a set of distinct signers, so a repeated vote cannot inflate the round towards the
/// full participation that [`ConsensusStatus::Split`] requires.
pub struct ConsensusTracker {
    coprocessor_count: usize,
    threshold: NonZeroUsize,
    /// Distinct signers that have voted. A set rather than a counter so that participation is
    /// exact even if the caller breaks the one-event-per-Coprocessor contract.
    voters: HashSet<Address>,
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
            voters: HashSet::new(),
            failures: 0,
            groups: HashMap::new(),
        }
    }

    /// Record a validated attestation and return the updated verdict.
    pub fn add_vote(&mut self, vote: ValidAttestation) -> ConsensusStatus {
        self.voters.insert(vote.signer);
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
        let votes = self.voters.len();
        let pending = self
            .coprocessor_count
            .saturating_sub(votes)
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

        if let Some((material, signers)) =
            leading.filter(|(_, signers)| signers.len() >= self.threshold.get())
        {
            return ConsensusStatus::Reached(Consensus {
                material: material.clone(),
                signers: signers.clone(),
            });
        }
        if largest + pending >= self.threshold.get() {
            return ConsensusStatus::Pending;
        }
        // The round can no longer reach threshold. Terminal only when disagreement is *proven*,
        // which takes all three of: no failures, every registered Coprocessor voted, and at
        // least two distinct groups to disagree with each other.
        //
        // Each conjunct rules out a round that merely looks decided. Without full participation
        // the verdict would depend on reply arrival order — the same round is retriable if a
        // failure lands early and terminal if it lands last. Without more than one group, a
        // threshold that exceeds the number of reachable Coprocessors (registered Coprocessors
        // with no bucket URL are dropped from the snapshot, while the threshold comes from chain)
        // would report unanimously agreeing Coprocessors as disagreeing, and reject every
        // request until an operator noticed.
        if self.failures == 0 && votes == self.coprocessor_count && self.groups.len() > 1 {
            return ConsensusStatus::Split {
                valid: votes,
                largest,
            };
        }
        // Anything else is an unresolved gap, not a disagreement. A round lost to failures says
        // nothing about agreement — attestations are published asynchronously, so the
        // overwhelmingly common cause is "not uploaded yet", which the caller's retry budget
        // handles. Never collapse this into `Split`: that would turn a retriable gap into a false
        // terminal verdict, which is exactly the misclassification this tracker exists to fix.
        ConsensusStatus::Starved {
            valid: votes,
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
    /// Every registered Coprocessor answered validly, they did not all agree, and still no group
    /// reached the threshold. Terminal: re-reading the same objects returns the same disagreement.
    ///
    /// A round that merely failed to reach threshold is [`Self::Starved`], not this — see
    /// [`ConsensusTracker::status`] for why each condition is required.
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

    /// A dissenting vote from `signer`: same handle, different SNS digest.
    async fn dissenting_vote_from(signer: &PrivateKeySigner, sns: B256) -> ValidAttestation {
        let att = CiphertextAttestationPayload::new(
            Version::V1,
            HANDLE,
            KEY_ID,
            COPROCESSOR_CONTEXT_ID,
            CT_DIGEST,
            sns,
            FORMAT,
        )
        .sign(signer)
        .await
        .unwrap();
        ValidAttestation::validate(&att, HANDLE, COPROCESSOR_CONTEXT_ID, signer.address()).unwrap()
    }

    #[tokio::test]
    async fn unanimous_agreement_below_threshold_is_starved_not_split() {
        // 2 reachable Coprocessors but a threshold of 3 — the shape a partially-onboarded
        // deployment produces, since Coprocessors registered without an S3 bucket URL are
        // dropped from the snapshot while the threshold still comes from chain. Both answer and
        // agree perfectly: there is no disagreement to be terminal about, so the round must stay
        // retriable rather than telling the operator the Coprocessors disagree.
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let mut tracker = ConsensusTracker::new(2, nz(3));

        tracker.add_vote(vote_from(&s1).await);
        let status = tracker.add_vote(vote_from(&s2).await);

        match status {
            ConsensusStatus::Starved { valid, required } => {
                assert_eq!(valid, 2);
                assert_eq!(required, 3);
            }
            other => panic!("expected Starved, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn disagreement_without_full_participation_is_starved_not_split() {
        // 5 Coprocessors, threshold 3: four disagree and the fifth has not answered. The round is
        // already unwinnable, but participation is incomplete — calling this Split would make the
        // verdict depend on arrival order (terminal if the fifth reply lands last, retriable if
        // it lands first).
        let signers: Vec<PrivateKeySigner> = (0..4).map(|_| PrivateKeySigner::random()).collect();
        let mut tracker = ConsensusTracker::new(5, nz(3));

        let mut status = ConsensusStatus::Pending;
        for (i, signer) in signers.iter().enumerate() {
            status =
                tracker.add_vote(dissenting_vote_from(signer, B256::repeat_byte(i as u8)).await);
        }

        match status {
            ConsensusStatus::Starved { valid, required } => {
                assert_eq!(valid, 4);
                assert_eq!(required, 3);
            }
            other => panic!("expected Starved, got {other:?}"),
        }
    }

    #[test]
    fn empty_round_is_starved_not_split() {
        // Zero Coprocessors: vacuously "full participation" with zero failures. Nothing has
        // disagreed, so this must not read as proven disagreement.
        let tracker = ConsensusTracker::new(0, nz(1));

        match tracker.status() {
            ConsensusStatus::Starved { valid, required } => {
                assert_eq!(valid, 0);
                assert_eq!(required, 1);
            }
            other => panic!("expected Starved, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn lone_coprocessor_below_threshold_is_starved_not_split() {
        // 1 Coprocessor, threshold 2: it answered, so participation is full and there are no
        // failures — but a single voter cannot constitute a disagreement.
        let s1 = PrivateKeySigner::random();
        let mut tracker = ConsensusTracker::new(1, nz(2));

        let status = tracker.add_vote(vote_from(&s1).await);

        match status {
            ConsensusStatus::Starved { valid, required } => {
                assert_eq!(valid, 1);
                assert_eq!(required, 2);
            }
            other => panic!("expected Starved, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn repeated_vote_from_one_signer_cannot_fake_full_participation() {
        // 3 Coprocessors, threshold 3. One signer's vote is replayed and a second signer
        // disagrees, so a naive vote counter would see 3 "votes", zero failures and two groups —
        // full participation and a terminal Split — while the third Coprocessor is still silent.
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let mut tracker = ConsensusTracker::new(3, nz(3));

        let replayed = vote_from(&s1).await;
        tracker.add_vote(replayed.clone());
        tracker.add_vote(replayed);
        let status = tracker.add_vote(dissenting_vote_from(&s2, B256::repeat_byte(0xDD)).await);

        match status {
            ConsensusStatus::Starved { valid, required } => {
                assert_eq!(valid, 2, "the replayed vote must not count twice");
                assert_eq!(required, 3);
            }
            other => panic!("expected Starved, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn split_at_full_participation_with_more_than_two_coprocessors() {
        // 4 Coprocessors, threshold 3, split 2–2: every Coprocessor answered validly, no group
        // reaches 3, and the disagreement is real. Terminal.
        let signers: Vec<PrivateKeySigner> = (0..4).map(|_| PrivateKeySigner::random()).collect();
        let mut tracker = ConsensusTracker::new(4, nz(3));

        tracker.add_vote(vote_from(&signers[0]).await);
        tracker.add_vote(vote_from(&signers[1]).await);
        let dissent = B256::repeat_byte(0xDD);
        tracker.add_vote(dissenting_vote_from(&signers[2], dissent).await);
        let status = tracker.add_vote(dissenting_vote_from(&signers[3], dissent).await);

        match status {
            ConsensusStatus::Split { valid, largest } => {
                assert_eq!(valid, 4);
                assert_eq!(largest, 2);
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
