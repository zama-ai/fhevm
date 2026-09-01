//! Incremental consensus tracking over one fan-out round.
//!
//! [`ConsensusTracker::record`] returns a freshly computed [`ThresholdStatus`] after every
//! single reply, not once at the end. Acting on a mid-round verdict is safe because every
//! verdict is monotone: a slot, once filled, is never overwritten, so a later reply can only
//! reinforce a verdict already given, never contradict it.

use crate::consensus::ConsensusMaterial;
use crate::{AttestationError, CiphertextAttestation};
use alloy_primitives::{Address, B256, U256};
use std::{collections::HashMap, num::NonZeroUsize};

/// Validates that the signature recovers to the embedded signer and that this signer equals
/// `registered_signer`. Without the second check, an attacker controlling one Coprocessor's
/// bucket could serve a *different* Coprocessor's genuine attestation as its own — cross-serving.
pub fn validate(
    attestation: &CiphertextAttestation,
    handle: B256,
    coprocessor_context_id: U256,
    registered_signer: Address,
) -> Result<ConsensusMaterial, AttestationError> {
    attestation.verify(handle, coprocessor_context_id)?;
    if attestation.signer != registered_signer {
        return Err(AttestationError::SignerNotRegisteredForBucket {
            embedded: attestation.signer,
            registered: registered_signer,
        });
    }
    Ok(ConsensusMaterial::from(attestation))
}

/// One registered Coprocessor's on-chain identity triple: which tx sender it is, which signer it
/// is bound to, and which bucket serves its attestations. (`getCoprocessor(txSender)`)
///
/// Lives here rather than in `client::registry` because [`Round`] holds one per slot, and
/// `tracker` is not behind the `client` feature. `registry` re-exports it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoprocessorEntry {
    pub tx_sender: Address,
    pub signer: Address,
    pub bucket: String,
}

/// What one registered Coprocessor did this round.
#[derive(Clone, Debug)]
pub enum Reply {
    Attested(ConsensusMaterial),
    /// It never answered: timeout, HTTP error, missing or malformed header.
    NoReply,
    /// It answered, but the attestation failed validation.
    Rejected,
    /// It has not answered yet.
    Outstanding,
}

/// One round of asking every registered Coprocessor for an attestation.
#[derive(Clone)]
pub struct Round {
    /// The handle this round is about.
    pub handle: B256,
    pub threshold: NonZeroUsize,
    /// One slot per registered Coprocessor, in roster order. Filled in place as replies arrive.
    pub replies: Vec<(CoprocessorEntry, Reply)>,
}

impl Round {
    pub fn attested(&self) -> Vec<Address> {
        self.addresses_where(|r| matches!(r, Reply::Attested(_)))
    }

    fn silent(&self) -> Vec<Address> {
        self.addresses_where(|r| matches!(r, Reply::NoReply))
    }

    fn rejected(&self) -> Vec<Address> {
        self.addresses_where(|r| matches!(r, Reply::Rejected))
    }

    pub(crate) fn outstanding(&self) -> Vec<Address> {
        self.addresses_where(|r| matches!(r, Reply::Outstanding))
    }

    /// Size of the largest agreeing group, or `0` if nobody has attested yet.
    pub fn agreeing(&self) -> usize {
        self.winner().map_or(0, |(_, entries)| entries.len())
    }

    /// The largest group of Coprocessors that attested the same material, with that material.
    /// Ties are broken by material — the same tie-break `consensus::evaluate` uses.
    fn winner(&self) -> Option<(ConsensusMaterial, Vec<CoprocessorEntry>)> {
        let mut grouped: HashMap<&ConsensusMaterial, Vec<CoprocessorEntry>> = HashMap::new();
        for (entry, reply) in &self.replies {
            if let Reply::Attested(material) = reply {
                grouped.entry(material).or_default().push(entry.clone());
            }
        }
        grouped
            .into_iter()
            .max_by(
                |(left_material, left_entries), (right_material, right_entries)| {
                    left_entries
                        .len()
                        .cmp(&right_entries.len())
                        .then_with(|| right_material.cmp(left_material))
                },
            )
            .map(|(material, entries)| (material.clone(), entries))
    }

    fn addresses_where(&self, pred: impl Fn(&Reply) -> bool) -> Vec<Address> {
        self.replies
            .iter()
            .filter(|(_, reply)| pred(reply))
            .map(|(entry, _)| entry.signer)
            .collect()
    }
}

/// Where a round stands, recomputed after every reply.
#[derive(Clone, Debug)]
pub enum ThresholdStatus {
    /// Not everyone has answered and the threshold is still reachable this round.
    AwaitingReplies,
    /// A group of at least `threshold` distinct signers agreed on the same material.
    Reached {
        material: ConsensusMaterial,
        winners: Vec<CoprocessorEntry>,
    },
    /// Every Coprocessor answered or failed; no group reached the threshold. Retriable.
    MissedThisRound(Round),
    /// The Coprocessors that answered disagree, and even the best possible outcome would still
    /// fall short of threshold. Terminal for this round's registered signer set: cast
    /// attestations are immutable.
    Unreachable(Round),
}

/// Incremental consensus over one fan-out round. No network, no time, no I/O: the caller feeds
/// it exactly one reply per registered Coprocessor and reads a freshly recomputed verdict back
/// after each one.
pub struct ConsensusTracker {
    round: Round,
}

impl ConsensusTracker {
    /// Builds the board from the roster: one slot per registered signer, all
    /// [`Reply::Outstanding`].
    pub fn new(
        handle: B256,
        entries: impl IntoIterator<Item = CoprocessorEntry>,
        threshold: NonZeroUsize,
    ) -> Self {
        Self {
            round: Round {
                handle,
                threshold,
                replies: entries
                    .into_iter()
                    .map(|entry| (entry, Reply::Outstanding))
                    .collect(),
            },
        }
    }

    /// Fills `signer`'s slot and returns the freshly recomputed verdict. First write wins: a
    /// later reply for the same signer is dropped, which makes a single Coprocessor structurally
    /// unable to occupy two groups at once. An unknown signer is ignored (`debug_assert!`).
    pub fn record(&mut self, signer: Address, reply: Reply) -> ThresholdStatus {
        match self
            .round
            .replies
            .iter_mut()
            .find_map(|(entry, slot)| (entry.signer == signer).then_some(slot))
        {
            Some(slot @ Reply::Outstanding) => *slot = reply,
            Some(_) => {}
            None => debug_assert!(
                false,
                "record() called for signer {signer}, not in the roster"
            ),
        }
        self.verdict()
    }

    /// Reads the board without changing it.
    pub fn verdict(&self) -> ThresholdStatus {
        let round = &self.round;
        let threshold = round.threshold.get();
        let winner = round.winner();
        let largest = winner.as_ref().map_or(0, |(_, entries)| entries.len());
        let attested = round.attested().len();
        let disagreed = attested > largest;
        let outstanding = round.outstanding().len();
        // Counts failures too, unlike `outstanding`: a Coprocessor that failed can attest again
        // next round.
        let missing = round.replies.len() - attested;

        if largest >= threshold {
            let (material, winners) =
                winner.expect("largest >= threshold > 0 implies a winning group exists");
            return ThresholdStatus::Reached { material, winners };
        }
        if largest + outstanding >= threshold {
            return ThresholdStatus::AwaitingReplies;
        }
        if disagreed && largest + missing < threshold {
            return ThresholdStatus::Unreachable(round.clone());
        }
        ThresholdStatus::MissedThisRound(round.clone())
    }
}

impl std::fmt::Display for Round {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "handle {}: {} of {} required attested",
            self.handle,
            self.agreeing(),
            self.threshold.get()
        )?;
        let silent = self.silent();
        if !silent.is_empty() {
            write!(f, ", {} never replied", format_addrs(&silent))?;
        }
        let rejected = self.rejected();
        if !rejected.is_empty() {
            write!(f, ", {} rejected", format_addrs(&rejected))?;
        }
        let outstanding = self.outstanding();
        if !outstanding.is_empty() {
            write!(f, ", {} still outstanding", format_addrs(&outstanding))?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for Round {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "handle {}: need {}: ", self.handle, self.threshold.get())?;
        for (i, (entry, reply)) in self.replies.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}→", entry.signer)?;
            match reply {
                Reply::Attested(material) => write!(f, "attested{{{material:?}}}")?,
                Reply::NoReply => write!(f, "no reply")?,
                Reply::Rejected => write!(f, "rejected")?,
                Reply::Outstanding => write!(f, "outstanding")?,
            }
        }
        Ok(())
    }
}

/// Full addresses, comma-separated. Signer addresses are fine in user-facing strings — public
/// on-chain state.
fn format_addrs(addrs: &[Address]) -> String {
    addrs
        .iter()
        .map(Address::to_string)
        .collect::<Vec<_>>()
        .join(", ")
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

    /// A validated `(signer, reply)` pair, as if `signer`'s own bucket served its own
    /// attestation.
    async fn reply_from(signer: &PrivateKeySigner) -> (Address, Reply) {
        let att = signed(signer).await;
        let material = validate(&att, HANDLE, COPROCESSOR_CONTEXT_ID, signer.address()).unwrap();
        (signer.address(), Reply::Attested(material))
    }

    /// A dissenting `(signer, reply)` pair: same handle, different SNS digest.
    async fn dissenting_reply_from(signer: &PrivateKeySigner, sns: B256) -> (Address, Reply) {
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
        let material = validate(&att, HANDLE, COPROCESSOR_CONTEXT_ID, signer.address()).unwrap();
        (signer.address(), Reply::Attested(material))
    }

    /// Roster entries for `signers`, each bound to the bucket a real registry would give it.
    fn entries(signers: impl IntoIterator<Item = Address>) -> Vec<CoprocessorEntry> {
        signers
            .into_iter()
            .map(|signer| CoprocessorEntry {
                tx_sender: signer,
                signer,
                bucket: format!("http://bucket-{signer}"),
            })
            .collect()
    }

    #[tokio::test]
    async fn reaches_consensus_at_threshold() {
        // Pins the threshold boundary: the reply that meets it flips the verdict, not an earlier
        // one.
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let mut tracker =
            ConsensusTracker::new(HANDLE, entries([s1.address(), s2.address()]), nz(2));

        let (signer, reply) = reply_from(&s1).await;
        assert!(matches!(
            tracker.record(signer, reply),
            ThresholdStatus::AwaitingReplies
        ));

        let (signer, reply) = reply_from(&s2).await;
        match tracker.record(signer, reply) {
            ThresholdStatus::Reached { material, winners } => {
                assert_eq!(winners.len(), 2);
                assert_eq!(material.ciphertext_digest, CT_DIGEST);
            }
            other => panic!("expected Reached, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn awaiting_replies_while_replies_outstanding() {
        let s1 = PrivateKeySigner::random();
        let mut tracker = ConsensusTracker::new(
            HANDLE,
            entries([
                s1.address(),
                PrivateKeySigner::random().address(),
                PrivateKeySigner::random().address(),
            ]),
            nz(2),
        );

        let (signer, reply) = reply_from(&s1).await;
        let status = tracker.record(signer, reply);
        assert!(matches!(status, ThresholdStatus::AwaitingReplies));
    }

    #[tokio::test]
    async fn missed_this_round_when_failures_make_round_unwinnable() {
        // Nobody disagreed, so an unwinnable round is MissedThisRound, never Unreachable.
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random().address();
        let s3 = PrivateKeySigner::random().address();
        let mut tracker = ConsensusTracker::new(HANDLE, entries([s1.address(), s2, s3]), nz(2));

        let (signer, reply) = reply_from(&s1).await;
        tracker.record(signer, reply);
        tracker.record(s2, Reply::NoReply);
        let status = tracker.record(s3, Reply::NoReply);

        match &status {
            ThresholdStatus::MissedThisRound(round) => {
                assert_eq!(round.attested().len(), 1);
                assert_eq!(round.threshold.get(), 2);
            }
            other => panic!("expected MissedThisRound, got {other:?}"),
        }
        assert!(!matches!(status, ThresholdStatus::Unreachable(_)));
    }

    #[tokio::test]
    async fn unreachable_when_every_coprocessor_answered_and_disagreed() {
        // Terminal rather than retriable because nobody is left to vote.
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let mut tracker =
            ConsensusTracker::new(HANDLE, entries([s1.address(), s2.address()]), nz(2));

        let (signer, reply) = reply_from(&s1).await;
        tracker.record(signer, reply);
        let (signer, reply) = dissenting_reply_from(&s2, B256::repeat_byte(0xDD)).await;
        let status = tracker.record(signer, reply);

        match status {
            ThresholdStatus::Unreachable(round) => {
                assert_eq!(round.attested().len(), 2);
                assert_eq!(round.agreeing(), 1);
            }
            other => panic!("expected Unreachable, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn unanimous_agreement_below_threshold_is_missed_not_unreachable() {
        // The shape a partially-onboarded deployment produces: Coprocessors registered without an
        // S3 bucket URL are dropped from the snapshot while the threshold still comes from chain.
        // Unanimity is not disagreement, so there is nothing to be terminal about.
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let mut tracker =
            ConsensusTracker::new(HANDLE, entries([s1.address(), s2.address()]), nz(3));

        let (signer, reply) = reply_from(&s1).await;
        tracker.record(signer, reply);
        let (signer, reply) = reply_from(&s2).await;
        let status = tracker.record(signer, reply);

        match status {
            ThresholdStatus::MissedThisRound(round) => {
                assert_eq!(round.attested().len(), 2);
                assert_eq!(round.threshold.get(), 3);
            }
            other => panic!("expected MissedThisRound, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn unreachable_before_the_last_reply_arrives() {
        // Terminal before every reply is in: whatever the outstanding Coprocessor says joins a
        // group too small to ever reach threshold.
        let signers: Vec<PrivateKeySigner> = (0..4).map(|_| PrivateKeySigner::random()).collect();
        let mut roster: Vec<Address> = signers.iter().map(|s| s.address()).collect();
        roster.push(PrivateKeySigner::random().address());
        let mut tracker = ConsensusTracker::new(HANDLE, entries(roster), nz(3));

        let mut status = ThresholdStatus::AwaitingReplies;
        for (i, signer) in signers.iter().enumerate() {
            let (addr, reply) = dissenting_reply_from(signer, B256::repeat_byte(i as u8)).await;
            status = tracker.record(addr, reply);
        }

        match status {
            ThresholdStatus::Unreachable(round) => {
                assert_eq!(round.attested().len(), 4);
                assert_eq!(round.agreeing(), 1);
            }
            other => panic!("expected Unreachable, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn unreachable_with_a_failure_when_unwinnable() {
        // A retry turning the failure into an attestation would still fall short of threshold, so
        // the failure does not make the round retriable.
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let s3 = PrivateKeySigner::random().address();
        let mut tracker =
            ConsensusTracker::new(HANDLE, entries([s1.address(), s2.address(), s3]), nz(3));

        let (signer, reply) = reply_from(&s1).await;
        tracker.record(signer, reply);
        let (signer, reply) = dissenting_reply_from(&s2, B256::repeat_byte(0xDD)).await;
        tracker.record(signer, reply);
        let status = tracker.record(s3, Reply::NoReply);

        match status {
            ThresholdStatus::Unreachable(round) => {
                assert_eq!(round.attested().len(), 2);
                assert_eq!(round.agreeing(), 1);
            }
            other => panic!("expected Unreachable, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn missed_this_round_with_a_failure_when_a_returning_vote_can_win() {
        // The retriable side of that frontier: a retry where the failing Coprocessor answers with
        // an already-attested material would meet the threshold.
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let s3 = PrivateKeySigner::random().address();
        let mut tracker =
            ConsensusTracker::new(HANDLE, entries([s1.address(), s2.address(), s3]), nz(2));

        let (signer, reply) = reply_from(&s1).await;
        tracker.record(signer, reply);
        let (signer, reply) = dissenting_reply_from(&s2, B256::repeat_byte(0xDD)).await;
        tracker.record(signer, reply);
        let status = tracker.record(s3, Reply::NoReply);

        match status {
            ThresholdStatus::MissedThisRound(round) => {
                assert_eq!(round.attested().len(), 2);
                assert_eq!(round.threshold.get(), 2);
            }
            other => panic!("expected MissedThisRound, got {other:?}"),
        }
    }

    #[test]
    fn empty_round_is_missed_not_unreachable() {
        // An empty roster is vacuously full participation with zero failures, which must not read
        // as proven disagreement.
        let tracker = ConsensusTracker::new(HANDLE, entries(std::iter::empty()), nz(1));

        match tracker.verdict() {
            ThresholdStatus::MissedThisRound(round) => {
                assert_eq!(round.attested().len(), 0);
                assert_eq!(round.threshold.get(), 1);
            }
            other => panic!("expected MissedThisRound, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn lone_coprocessor_below_threshold_is_missed_not_unreachable() {
        // Participation is full and nothing failed, but a single voter cannot constitute a
        // disagreement.
        let s1 = PrivateKeySigner::random();
        let mut tracker = ConsensusTracker::new(HANDLE, entries([s1.address()]), nz(2));

        let (signer, reply) = reply_from(&s1).await;
        let status = tracker.record(signer, reply);

        match status {
            ThresholdStatus::MissedThisRound(round) => {
                assert_eq!(round.attested().len(), 1);
                assert_eq!(round.threshold.get(), 2);
            }
            other => panic!("expected MissedThisRound, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn replayed_vote_from_one_signer_counts_once() {
        // The verdict is terminal either way; the count is the point. Only first-write-wins keeps
        // `agreeing()` honest — a tracker that let a replay refill the slot would fail open.
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let s3 = PrivateKeySigner::random().address();
        let mut tracker =
            ConsensusTracker::new(HANDLE, entries([s1.address(), s2.address(), s3]), nz(3));

        let (signer, reply) = reply_from(&s1).await;
        tracker.record(signer, reply.clone());
        tracker.record(signer, reply);
        let (signer, reply) = dissenting_reply_from(&s2, B256::repeat_byte(0xDD)).await;
        let status = tracker.record(signer, reply);

        match status {
            ThresholdStatus::Unreachable(round) => {
                assert_eq!(
                    round.attested().len(),
                    2,
                    "the replayed reply must not count twice"
                );
                assert_eq!(round.agreeing(), 1);
            }
            other => panic!("expected Unreachable, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn unreachable_with_more_than_two_coprocessors() {
        let signers: Vec<PrivateKeySigner> = (0..4).map(|_| PrivateKeySigner::random()).collect();
        let roster: Vec<Address> = signers.iter().map(|s| s.address()).collect();
        let mut tracker = ConsensusTracker::new(HANDLE, entries(roster), nz(3));

        let (signer, reply) = reply_from(&signers[0]).await;
        tracker.record(signer, reply);
        let (signer, reply) = reply_from(&signers[1]).await;
        tracker.record(signer, reply);
        let dissent = B256::repeat_byte(0xDD);
        let (signer, reply) = dissenting_reply_from(&signers[2], dissent).await;
        tracker.record(signer, reply);
        let (signer, reply) = dissenting_reply_from(&signers[3], dissent).await;
        let status = tracker.record(signer, reply);

        match status {
            ThresholdStatus::Unreachable(round) => {
                assert_eq!(round.attested().len(), 4);
                assert_eq!(round.agreeing(), 2);
            }
            other => panic!("expected Unreachable, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn reached_while_replies_outstanding() {
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let roster = vec![
            s1.address(),
            s2.address(),
            PrivateKeySigner::random().address(),
            PrivateKeySigner::random().address(),
            PrivateKeySigner::random().address(),
        ];
        let mut tracker = ConsensusTracker::new(HANDLE, entries(roster), nz(2));

        let (signer, reply) = reply_from(&s1).await;
        tracker.record(signer, reply);
        let (signer, reply) = reply_from(&s2).await;
        let status = tracker.record(signer, reply);

        assert!(matches!(status, ThresholdStatus::Reached { .. }));
    }

    #[tokio::test]
    async fn validate_rejects_bad_signature() {
        let s1 = PrivateKeySigner::random();
        let mut att = signed(&s1).await;
        att.sns_ciphertext_digest = B256::repeat_byte(0xEE);

        let err = validate(&att, HANDLE, COPROCESSOR_CONTEXT_ID, s1.address()).unwrap_err();
        assert!(matches!(err, AttestationError::SignerMismatch { .. }));
    }

    #[tokio::test]
    async fn validate_rejects_cross_served_attestation() {
        // Validly signed by s1, but served by a bucket registered to s2.
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let att = signed(&s1).await;

        let err = validate(&att, HANDLE, COPROCESSOR_CONTEXT_ID, s2.address()).unwrap_err();
        assert!(matches!(
            err,
            AttestationError::SignerNotRegisteredForBucket { embedded, registered }
                if embedded == s1.address() && registered == s2.address()
        ));
    }

    #[tokio::test]
    async fn second_different_vote_from_same_signer_does_not_create_second_group() {
        // New: spec 1.6 bug 3. One signer answers, then a *different* attestation arrives for
        // the same signer (e.g. a stray retry). Before the slot board this could occupy a
        // second group and, at threshold 2 with one other Coprocessor, fabricate a Reached
        // verdict from a single voter. The slot model makes it structurally impossible: one
        // signer, one slot, first write wins.
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let mut tracker =
            ConsensusTracker::new(HANDLE, entries([s1.address(), s2.address()]), nz(2));

        let (signer, first_reply) = reply_from(&s1).await;
        let first_material = match &first_reply {
            Reply::Attested(material) => material.clone(),
            other => panic!("expected Attested, got {other:?}"),
        };
        tracker.record(signer, first_reply);
        let (_, second_reply) = dissenting_reply_from(&s1, B256::repeat_byte(0xDD)).await;
        let status = tracker.record(signer, second_reply);

        assert!(
            matches!(status, ThresholdStatus::AwaitingReplies),
            "expected AwaitingReplies, got {status:?}"
        );
        // The board itself, not just the verdict: one slot is filled, so no second group can
        // exist, and the winner still holds the *first* material.
        assert_eq!(tracker.round.attested(), vec![s1.address()]);
        let (material, _) = tracker
            .round
            .winner()
            .expect("the first reply opened a group");
        assert_eq!(
            material, first_material,
            "the replay must not displace the first material"
        );

        // A second, real signer now completes the group the first reply opened.
        let (signer, reply) = reply_from(&s2).await;
        let status = tracker.record(signer, reply);
        assert!(matches!(status, ThresholdStatus::Reached { .. }));
    }

    #[tokio::test]
    async fn majority_group_wins_over_minority() {
        // Length dominance on this crate's path. `consensus.rs`'s test of the same name pins it
        // for the frozen `evaluate`, so inverting `winner`'s length comparison leaves every other
        // test here passing: the multi-group tests all hold groups of equal size.
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let s3 = PrivateKeySigner::random();
        let mut tracker = ConsensusTracker::new(
            HANDLE,
            entries([s1.address(), s2.address(), s3.address()]),
            nz(2),
        );

        let dissent = B256::repeat_byte(0xDD);
        assert!(
            SNS_DIGEST < dissent,
            "the majority must hold the larger material, or a material-only comparator would pass"
        );

        // The minority group opens first, and the majority forms on the losing material.
        let (signer, reply) = reply_from(&s3).await;
        tracker.record(signer, reply);
        let (signer, reply) = dissenting_reply_from(&s1, dissent).await;
        tracker.record(signer, reply);
        let (signer, reply) = dissenting_reply_from(&s2, dissent).await;
        let status = tracker.record(signer, reply);

        match status {
            ThresholdStatus::Reached { material, winners } => {
                assert_eq!(winners.len(), 2);
                assert_eq!(material.sns_ciphertext_digest, dissent);
            }
            other => panic!("expected Reached, got {other:?}"),
        }
        assert_eq!(tracker.round.agreeing(), 2);
    }

    #[tokio::test]
    async fn equal_size_groups_use_deterministic_material_tie_break() {
        // `consensus.rs`'s test of the same name pins this for the frozen `evaluate`; nothing
        // pinned it for `winner()`, so a flipped tie-break would leave every other test here
        // passing.
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        // Threshold 3 with only 2 Coprocessors keeps this off the `Reached` path: the test is
        // about the tie-break, not about winning.
        let mut tracker =
            ConsensusTracker::new(HANDLE, entries([s1.address(), s2.address()]), nz(3));

        let (signer1, reply1) = reply_from(&s1).await;
        let (signer2, reply2) = dissenting_reply_from(&s2, B256::repeat_byte(0xDD)).await;
        let material1 = match &reply1 {
            Reply::Attested(material) => material.clone(),
            other => panic!("expected Attested, got {other:?}"),
        };
        let material2 = match &reply2 {
            Reply::Attested(material) => material.clone(),
            other => panic!("expected Attested, got {other:?}"),
        };
        assert_ne!(
            material1, material2,
            "the two groups must disagree for this test to mean anything"
        );

        tracker.record(signer1, reply1);
        let status = tracker.record(signer2, reply2);

        let round = match status {
            ThresholdStatus::Unreachable(round) => round,
            other => panic!("expected Unreachable (kept off the Reached path), got {other:?}"),
        };
        let (winning_material, winners) = round.winner().expect("both replies attested");
        assert_eq!(round.attested().len(), 2);
        assert_eq!(
            winners.len(),
            1,
            "the two groups must be equal-sized for a tie-break to decide"
        );

        // Derive the expected winner from the fixtures themselves, not a hardcoded pick, so this
        // test does not go vacuous if the fixtures' digests ever change.
        assert_eq!(
            winning_material,
            material1.min(material2),
            "the winning group must hold the smaller material on a tie, matching consensus::evaluate"
        );
    }
}
