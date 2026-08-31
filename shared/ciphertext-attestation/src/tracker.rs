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
///
/// `Display` is redacted (no digest values); `Debug` is the full board, operator diagnostics only.
#[derive(Clone)]
pub struct Round {
    /// The handle this round is about.
    pub handle: B256,
    pub threshold: NonZeroUsize,
    /// One slot per registered Coprocessor, in roster order. Filled in place as replies arrive.
    pub replies: Vec<(CoprocessorEntry, Reply)>,
}

impl Round {
    /// The roster size, not the number of replies received.
    pub fn asked(&self) -> usize {
        self.replies.len()
    }

    pub fn attested(&self) -> Vec<Address> {
        self.addresses_where(|r| matches!(r, Reply::Attested(_)))
    }

    pub fn silent(&self) -> Vec<Address> {
        self.addresses_where(|r| matches!(r, Reply::NoReply))
    }

    pub fn rejected(&self) -> Vec<Address> {
        self.addresses_where(|r| matches!(r, Reply::Rejected))
    }

    pub fn outstanding(&self) -> Vec<Address> {
        self.addresses_where(|r| matches!(r, Reply::Outstanding))
    }

    /// Size of the largest group in [`Self::groups`], or `0` if nobody has attested yet.
    pub fn agreeing(&self) -> usize {
        self.groups()
            .first()
            .map_or(0, |(_, entries)| entries.len())
    }

    /// Attested slots bucketed by agreed-on material, largest group first. Ties are broken by
    /// material — the same tie-break `consensus::evaluate` uses.
    pub fn groups(&self) -> Vec<(ConsensusMaterial, Vec<CoprocessorEntry>)> {
        let mut grouped: HashMap<&ConsensusMaterial, Vec<CoprocessorEntry>> = HashMap::new();
        for (entry, reply) in &self.replies {
            if let Reply::Attested(material) = reply {
                grouped.entry(material).or_default().push(entry.clone());
            }
        }
        let mut groups: Vec<(ConsensusMaterial, Vec<CoprocessorEntry>)> = grouped
            .into_iter()
            .map(|(material, entries)| (material.clone(), entries))
            .collect();
        groups.sort_by(
            |(left_material, left_entries), (right_material, right_entries)| {
                right_entries
                    .len()
                    .cmp(&left_entries.len())
                    .then_with(|| left_material.cmp(right_material))
            },
        );
        groups
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
    /// A group of at least `threshold` distinct signers agreed on the same material. `winners`
    /// are their roster entries, so the caller reads both the signers and their buckets off it.
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
        let groups = round.groups();
        let largest = groups.first().map_or(0, |(_, entries)| entries.len());
        let outstanding = round.outstanding().len();
        // Deliberately not `outstanding`: a Coprocessor that merely failed still counts as
        // reachable next round.
        let missing = round.asked() - round.attested().len();

        if largest >= threshold {
            let (material, winners) = groups
                .into_iter()
                .next()
                .expect("largest >= threshold > 0 implies a leading group exists");
            return ThresholdStatus::Reached { material, winners };
        }
        if largest + outstanding >= threshold {
            return ThresholdStatus::AwaitingReplies;
        }
        if groups.len() > 1 && largest + missing < threshold {
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
    const DISSENTING_SNS_DIGEST: B256 = B256::repeat_byte(0xDD);
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

    /// A filler roster slot that never replies this round.
    fn filler() -> Address {
        PrivateKeySigner::random().address()
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
        let mut tracker =
            ConsensusTracker::new(HANDLE, entries([s1.address(), filler(), filler()]), nz(2));

        let (signer, reply) = reply_from(&s1).await;
        let status = tracker.record(signer, reply);
        assert!(matches!(status, ThresholdStatus::AwaitingReplies));
    }

    #[tokio::test]
    async fn missed_this_round_when_failures_make_round_unwinnable() {
        // Nobody disagreed, so an unwinnable round is MissedThisRound, never Unreachable.
        let s1 = PrivateKeySigner::random();
        let s2 = filler();
        let s3 = filler();
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
        roster.push(filler());
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
        let s3 = filler();
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
        let s3 = filler();
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
        let s3 = filler();
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
        let roster = vec![s1.address(), s2.address(), filler(), filler(), filler()];
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
        tracker.record(signer, first_reply);
        let (_, second_reply) = dissenting_reply_from(&s1, B256::repeat_byte(0xDD)).await;
        let status = tracker.record(signer, second_reply);

        assert!(
            matches!(status, ThresholdStatus::AwaitingReplies),
            "expected AwaitingReplies, got {status:?}"
        );
        // The board itself, not just the verdict: only one group exists, and it still holds the
        // *first* material — the replayed reply was dropped, not merged into a second group.
        assert_eq!(
            tracker.round.groups().len(),
            1,
            "the replay must not open a second group"
        );
        assert_eq!(tracker.round.attested(), vec![s1.address()]);

        // A second, real signer now completes the group the first reply opened.
        let (signer, reply) = reply_from(&s2).await;
        let status = tracker.record(signer, reply);
        assert!(matches!(status, ThresholdStatus::Reached { .. }));
    }

    /// A disagreeing, terminal round holding three distinct digest values, so a rendering that
    /// leaked any one of them would show it.
    async fn disagreeing_round() -> Round {
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let mut tracker =
            ConsensusTracker::new(HANDLE, entries([s1.address(), s2.address()]), nz(2));

        let (signer, reply) = reply_from(&s1).await;
        tracker.record(signer, reply);
        let (signer, reply) = dissenting_reply_from(&s2, DISSENTING_SNS_DIGEST).await;
        match tracker.record(signer, reply) {
            ThresholdStatus::Unreachable(round) => round,
            other => panic!("expected Unreachable, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn display_is_redacted_no_digest_values() {
        // `Display` fires implicitly wherever a `Round` is interpolated, including from an error
        // type's own message, so it must never carry digest values.
        let round = disagreeing_round().await;
        let rendered = round.to_string();

        for leaked in [CT_DIGEST, SNS_DIGEST, DISSENTING_SNS_DIGEST] {
            assert!(
                !rendered.contains(&format!("{leaked:x}")),
                "leaked digest {leaked}: {rendered}"
            );
        }
    }

    #[tokio::test]
    async fn debug_is_full_board_with_digest_values() {
        // The diagnostics rendering must show every attested material in a disagreement, not just
        // the winner — a differing field must never be hidden from an operator.
        let round = disagreeing_round().await;
        let rendered = format!("{round:?}");

        for expected in [CT_DIGEST, SNS_DIGEST, DISSENTING_SNS_DIGEST] {
            assert!(
                rendered.contains(&format!("{expected:x}")),
                "missing digest {expected}: {rendered}"
            );
        }
    }

    #[tokio::test]
    async fn groups_orders_by_material_matching_evaluates_tie_break() {
        // Coverage gap flagged by an independent differential review: the only executable pin on
        // tie-break direction lived in `consensus.rs`'s
        // `equal_size_groups_use_deterministic_material_tie_break`, which exercises the frozen
        // `evaluate`, not this crate's `Round::groups()`. If `groups()`'s comparator were ever
        // flipped, every test in this file would still pass. This pins `groups()` directly: two
        // equal-sized groups from two different materials, and `groups()[0]` must be the group
        // whose material is the *smaller* of the two — the same choice `evaluate` makes on a
        // tie — so the two orderings cannot silently drift apart.
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        // Threshold 3 with only 2 Coprocessors keeps this off the `Reached` path: the test is
        // about ordering, not about winning.
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
        let groups = round.groups();
        assert_eq!(
            groups.len(),
            2,
            "two distinct materials must yield two groups"
        );
        assert_eq!(groups[0].1.len(), 1);
        assert_eq!(groups[1].1.len(), 1);

        // Derive the expected winner from the fixtures themselves, not a hardcoded pick, so this
        // test does not go vacuous if the fixtures' digests ever change.
        let (smaller, larger) = if material1 < material2 {
            (&material1, &material2)
        } else {
            (&material2, &material1)
        };
        assert_eq!(
            &groups[0].0, smaller,
            "groups()[0] must be the smaller material on a tie, matching consensus::evaluate"
        );
        assert_eq!(&groups[1].0, larger);
    }
}
