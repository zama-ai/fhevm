//! Attestation consensus evaluation module.

use crate::{CiphertextAttestation, ConsensusMaterial};
use alloy_primitives::{Address, B256, U256};
use std::{collections::HashMap, num::NonZeroUsize};
use tracing::{trace, warn};

/// The on-chain identity of a registered Coprocessor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoprocessorEntry {
    pub tx_sender: Address,
    pub signer: Address,
    pub bucket: String,
}

/// One round of asking every registered Coprocessor for an attestation.
#[derive(Clone)]
pub struct ConsensusRound {
    pub handle: B256,
    pub threshold: NonZeroUsize,
    coprocessor_context_id: U256,
    /// One slot per registered Coprocessor, in the order given. Filled in place as replies arrive.
    replies: Vec<(CoprocessorEntry, CoprocessorReply)>,
}

/// What one registered Coprocessor did this round.
#[derive(Clone, Debug)]
enum CoprocessorReply {
    Attested(ConsensusMaterial),
    /// No attestation came back: timeout, HTTP error, missing or malformed header.
    NoReply,
    /// An attestation came back, but it failed validation.
    Rejected,
    /// It has not answered yet.
    Outstanding,
}

/// A reached consensus together with the Coprocessors that agreed on it.
#[derive(Debug)]
pub struct ResolvedConsensus {
    pub material: ConsensusMaterial,
    /// The Coprocessors whose attestation is in the winning group.
    pub winners: Vec<CoprocessorEntry>,
}

/// Why a handle has no attestation consensus this round.
#[derive(Debug, thiserror::Error)]
pub enum ConsensusCheckError {
    /// No group reached the threshold. Retriable: attestations are published asynchronously, so
    /// this is the normal early state.
    #[error("no attestation consensus yet: {0}")]
    NotReachedThisRound(ConsensusRound),

    /// The Coprocessors that answered disagree, and even the best possible outcome would still
    /// fall short of threshold. Terminal for this round.
    #[error("attestation consensus unreachable: {0}")]
    Unreachable(ConsensusRound),
}

/// The decision a round ends on.
pub type ConsensusOutcome = Result<ResolvedConsensus, ConsensusCheckError>;

impl ConsensusRound {
    /// Opens the round: one slot per registered Coprocessor, all [`CoprocessorReply::Outstanding`].
    pub fn new(
        handle: B256,
        coprocessor_context_id: U256,
        entries: impl IntoIterator<Item = CoprocessorEntry>,
        threshold: NonZeroUsize,
    ) -> Self {
        Self {
            handle,
            threshold,
            coprocessor_context_id,
            replies: entries
                .into_iter()
                .map(|entry| (entry, CoprocessorReply::Outstanding))
                .collect(),
        }
    }

    /// Size of the largest group of Coprocessors that attested the same material.
    pub fn largest_group_size(&self) -> usize {
        self.largest_group().map_or(0, |(_, entries)| entries.len())
    }

    /// The largest group of Coprocessors that attested the same material, with that material.
    ///
    /// Ties are broken deterministically by material, smallest wins, so the outcome never depends
    /// on reply order.
    fn largest_group(&self) -> Option<(ConsensusMaterial, Vec<CoprocessorEntry>)> {
        let mut grouped: HashMap<&ConsensusMaterial, Vec<CoprocessorEntry>> = HashMap::new();
        for (entry, reply) in &self.replies {
            if let CoprocessorReply::Attested(material) = reply {
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

    /// Verifies the attestation served by `signer`'s bucket and fills its slot.
    pub fn record_attestation(
        &mut self,
        signer: Address,
        attestation: &CiphertextAttestation,
    ) -> Option<ConsensusOutcome> {
        let handle = self.handle;
        let reply = match attestation.verify(handle, self.coprocessor_context_id, signer) {
            Ok(()) => CoprocessorReply::Attested(ConsensusMaterial::from(attestation)),
            Err(e) => {
                warn!(%signer, %handle, "Discarding invalid attestation: {e}");
                CoprocessorReply::Rejected
            }
        };
        self.record(signer, reply)
    }

    /// Fills `signer`'s slot for a Coprocessor that served no attestation.
    pub fn record_no_reply(&mut self, signer: Address) -> Option<ConsensusOutcome> {
        self.record(signer, CoprocessorReply::NoReply)
    }

    /// Ends the round, turning every slot still outstanding into [`CoprocessorReply::NoReply`].
    pub fn close(mut self) -> ConsensusOutcome {
        for (_, reply) in &mut self.replies {
            if matches!(reply, CoprocessorReply::Outstanding) {
                *reply = CoprocessorReply::NoReply;
            }
        }
        self.outcome()
            .expect("a round with no outstanding slot is always decided")
    }

    /// Records a coprocessor reply for this round.
    fn record(&mut self, signer: Address, reply: CoprocessorReply) -> Option<ConsensusOutcome> {
        trace!(%signer, handle = %self.handle, ?reply, "Coprocessor reply recorded");
        match self
            .replies
            .iter_mut()
            .find_map(|(entry, slot)| (entry.signer == signer).then_some(slot))
        {
            Some(slot @ CoprocessorReply::Outstanding) => *slot = reply,
            // Within a round, a repeat reply for the same Coprocessor is ignored: first write wins.
            Some(_) => {}
            None => {
                warn!("dropping a reply for {signer}, not a Coprocessor registered this round")
            }
        }
        self.outcome()
    }

    /// Evaluates the consensus with the currently registered coprocessor replies.
    ///
    /// `None` while not everyone has answered and the threshold is still reachable this round.
    fn outcome(&self) -> Option<ConsensusOutcome> {
        let threshold = self.threshold.get();
        let largest_group = self.largest_group();
        let largest = largest_group
            .as_ref()
            .map_or(0, |(_, entries)| entries.len());
        if let Some((material, winners)) = largest_group.filter(|_| largest >= threshold) {
            return Some(Ok(ResolvedConsensus { material, winners }));
        }

        let attested = self.attested().len();
        let outstanding = self.outstanding().len();
        if largest + outstanding >= threshold {
            return None;
        }

        let missing = self.replies.len() - attested;
        let disagreed = attested > largest;
        if disagreed && largest + missing < threshold {
            return Some(Err(ConsensusCheckError::Unreachable(self.clone())));
        }
        Some(Err(ConsensusCheckError::NotReachedThisRound(self.clone())))
    }

    pub fn attested(&self) -> Vec<Address> {
        self.addresses_where(|r| matches!(r, CoprocessorReply::Attested(_)))
    }

    pub fn never_replied(&self) -> Vec<Address> {
        self.addresses_where(|r| matches!(r, CoprocessorReply::NoReply))
    }

    pub fn rejected(&self) -> Vec<Address> {
        self.addresses_where(|r| matches!(r, CoprocessorReply::Rejected))
    }

    pub fn outstanding(&self) -> Vec<Address> {
        self.addresses_where(|r| matches!(r, CoprocessorReply::Outstanding))
    }

    fn addresses_where(&self, pred: impl Fn(&CoprocessorReply) -> bool) -> Vec<Address> {
        self.replies
            .iter()
            .filter(|(_, reply)| pred(reply))
            .map(|(entry, _)| entry.signer)
            .collect()
    }
}

impl std::fmt::Display for ConsensusRound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "handle {}: {} of {} required attested",
            self.handle,
            self.largest_group_size(),
            self.threshold.get()
        )?;
        let never_replied = self.never_replied();
        if !never_replied.is_empty() {
            write!(f, ", {} never replied", format_addrs(&never_replied))?;
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

impl std::fmt::Debug for ConsensusRound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "handle {}: need {}: ", self.handle, self.threshold.get())?;
        for (i, (entry, reply)) in self.replies.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}→", entry.signer)?;
            match reply {
                CoprocessorReply::Attested(material) => write!(f, "attested{{{material:?}}}")?,
                CoprocessorReply::NoReply => write!(f, "no reply")?,
                CoprocessorReply::Rejected => write!(f, "rejected")?,
                CoprocessorReply::Outstanding => write!(f, "outstanding")?,
            }
        }
        Ok(())
    }
}

/// Full addresses, comma-separated.
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
    const OTHER_SNS_DIGEST: B256 = B256::repeat_byte(0xDD);
    const FORMAT: CiphertextFormat = CiphertextFormat::UncompressedOnCpu;

    /// Signs a default-material attestation for `HANDLE`.
    async fn signed(signer: &PrivateKeySigner) -> CiphertextAttestation {
        signed_with_sns(signer, SNS_DIGEST).await
    }

    async fn signed_with_sns(signer: &PrivateKeySigner, sns: B256) -> CiphertextAttestation {
        CiphertextAttestationPayload::new(
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
        .unwrap()
    }

    /// A round over one registered Coprocessor per signer, each bound to the bucket a real
    /// registry would give it.
    fn open_round(signers: impl IntoIterator<Item = Address>, threshold: usize) -> ConsensusRound {
        let entries = signers.into_iter().map(|signer| CoprocessorEntry {
            tx_sender: signer,
            signer,
            bucket: format!("http://bucket-{signer}"),
        });
        ConsensusRound::new(
            HANDLE,
            COPROCESSOR_CONTEXT_ID,
            entries,
            NonZeroUsize::new(threshold).unwrap(),
        )
    }

    fn random_address() -> Address {
        PrivateKeySigner::random().address()
    }

    #[tokio::test]
    async fn reaches_consensus_at_threshold() {
        // Pins the threshold boundary: the reply that meets it decides the round, not an earlier
        // one.
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let mut round = open_round([s1.address(), s2.address()], 2);

        assert!(
            round
                .record_attestation(s1.address(), &signed(&s1).await)
                .is_none()
        );

        match round.record_attestation(s2.address(), &signed(&s2).await) {
            Some(Ok(ResolvedConsensus {
                material, winners, ..
            })) => {
                assert_eq!(winners.len(), 2);
                assert_eq!(material.ciphertext_digest, CT_DIGEST);
            }
            other => panic!("expected a reached consensus, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn undecided_while_replies_outstanding() {
        let s1 = PrivateKeySigner::random();
        let mut round = open_round([s1.address(), random_address(), random_address()], 2);

        let outcome = round.record_attestation(s1.address(), &signed(&s1).await);
        assert!(outcome.is_none(), "expected undecided, got {outcome:?}");
    }

    #[tokio::test]
    async fn missed_this_round_when_failures_make_round_unwinnable() {
        // Nobody disagreed, so an unwinnable round is NotReachedThisRound, never Unreachable.
        let s1 = PrivateKeySigner::random();
        let (s2, s3) = (random_address(), random_address());
        let mut round = open_round([s1.address(), s2, s3], 2);

        round.record_attestation(s1.address(), &signed(&s1).await);
        round.record_no_reply(s2);

        match round.record_no_reply(s3) {
            Some(Err(ConsensusCheckError::NotReachedThisRound(round))) => {
                assert_eq!(round.attested().len(), 1);
                assert_eq!(round.threshold.get(), 2);
            }
            other => panic!("expected NotReachedThisRound, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn unreachable_when_every_coprocessor_answered_and_disagreed() {
        // Terminal rather than retriable because nobody is left to vote.
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let mut round = open_round([s1.address(), s2.address()], 2);

        round.record_attestation(s1.address(), &signed(&s1).await);
        let other = signed_with_sns(&s2, OTHER_SNS_DIGEST).await;

        match round.record_attestation(s2.address(), &other) {
            Some(Err(ConsensusCheckError::Unreachable(round))) => {
                assert_eq!(round.attested().len(), 2);
                assert_eq!(round.largest_group_size(), 1);
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
        let mut round = open_round([s1.address(), s2.address()], 3);

        round.record_attestation(s1.address(), &signed(&s1).await);

        match round.record_attestation(s2.address(), &signed(&s2).await) {
            Some(Err(ConsensusCheckError::NotReachedThisRound(round))) => {
                assert_eq!(round.attested().len(), 2);
                assert_eq!(round.threshold.get(), 3);
            }
            other => panic!("expected NotReachedThisRound, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn unreachable_before_the_last_reply_arrives() {
        // Terminal before every reply is in: whatever the outstanding Coprocessor says joins a
        // group too small to ever reach threshold.
        let signers: Vec<PrivateKeySigner> = (0..4).map(|_| PrivateKeySigner::random()).collect();
        let mut addresses: Vec<Address> = signers.iter().map(|s| s.address()).collect();
        addresses.push(random_address());
        let mut round = open_round(addresses, 3);

        let mut outcome = None;
        for (i, signer) in signers.iter().enumerate() {
            let att = signed_with_sns(signer, B256::repeat_byte(i as u8)).await;
            outcome = round.record_attestation(signer.address(), &att);
        }

        match outcome {
            Some(Err(ConsensusCheckError::Unreachable(round))) => {
                assert_eq!(round.attested().len(), 4);
                assert_eq!(round.largest_group_size(), 1);
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
        let s3 = random_address();
        let mut round = open_round([s1.address(), s2.address(), s3], 3);

        round.record_attestation(s1.address(), &signed(&s1).await);
        let other = signed_with_sns(&s2, OTHER_SNS_DIGEST).await;
        round.record_attestation(s2.address(), &other);

        match round.record_no_reply(s3) {
            Some(Err(ConsensusCheckError::Unreachable(round))) => {
                assert_eq!(round.attested().len(), 2);
                assert_eq!(round.largest_group_size(), 1);
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
        let s3 = random_address();
        let mut round = open_round([s1.address(), s2.address(), s3], 2);

        round.record_attestation(s1.address(), &signed(&s1).await);
        let other = signed_with_sns(&s2, OTHER_SNS_DIGEST).await;
        round.record_attestation(s2.address(), &other);

        match round.record_no_reply(s3) {
            Some(Err(ConsensusCheckError::NotReachedThisRound(round))) => {
                assert_eq!(round.attested().len(), 2);
                assert_eq!(round.threshold.get(), 2);
            }
            other => panic!("expected NotReachedThisRound, got {other:?}"),
        }
    }

    #[test]
    fn empty_round_is_missed_not_unreachable() {
        // With no registered Coprocessors there is nothing to answer and nothing to fail, and that
        // must not read as proven disagreement.
        match open_round(std::iter::empty(), 1).close() {
            Err(ConsensusCheckError::NotReachedThisRound(round)) => {
                assert_eq!(round.attested().len(), 0);
                assert_eq!(round.threshold.get(), 1);
            }
            other => panic!("expected NotReachedThisRound, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn lone_coprocessor_below_threshold_is_missed_not_unreachable() {
        // Participation is full and nothing failed, but a single voter cannot constitute a
        // disagreement.
        let s1 = PrivateKeySigner::random();
        let mut round = open_round([s1.address()], 2);

        match round.record_attestation(s1.address(), &signed(&s1).await) {
            Some(Err(ConsensusCheckError::NotReachedThisRound(round))) => {
                assert_eq!(round.attested().len(), 1);
                assert_eq!(round.threshold.get(), 2);
            }
            other => panic!("expected NotReachedThisRound, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn replayed_vote_from_one_signer_counts_once() {
        // The outcome is terminal either way; the count is the point. Only first-write-wins keeps
        // `largest_group_size()` honest — a round that let a replay refill the slot would fail
        // open.
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let mut round = open_round([s1.address(), s2.address(), random_address()], 3);

        let att = signed(&s1).await;
        round.record_attestation(s1.address(), &att);
        round.record_attestation(s1.address(), &att);
        let other = signed_with_sns(&s2, OTHER_SNS_DIGEST).await;

        match round.record_attestation(s2.address(), &other) {
            Some(Err(ConsensusCheckError::Unreachable(round))) => {
                assert_eq!(
                    round.attested().len(),
                    2,
                    "the replayed reply must not count twice"
                );
                assert_eq!(round.largest_group_size(), 1);
            }
            other => panic!("expected Unreachable, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn unreachable_with_more_than_two_coprocessors() {
        let signers: Vec<PrivateKeySigner> = (0..4).map(|_| PrivateKeySigner::random()).collect();
        let mut round = open_round(signers.iter().map(|s| s.address()), 3);

        round.record_attestation(signers[0].address(), &signed(&signers[0]).await);
        round.record_attestation(signers[1].address(), &signed(&signers[1]).await);
        let other = signed_with_sns(&signers[2], OTHER_SNS_DIGEST).await;
        round.record_attestation(signers[2].address(), &other);
        let other = signed_with_sns(&signers[3], OTHER_SNS_DIGEST).await;

        match round.record_attestation(signers[3].address(), &other) {
            Some(Err(ConsensusCheckError::Unreachable(round))) => {
                assert_eq!(round.attested().len(), 4);
                assert_eq!(round.largest_group_size(), 2);
            }
            other => panic!("expected Unreachable, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn reached_while_replies_outstanding() {
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let mut round = open_round(
            [
                s1.address(),
                s2.address(),
                random_address(),
                random_address(),
                random_address(),
            ],
            2,
        );

        round.record_attestation(s1.address(), &signed(&s1).await);
        let outcome = round.record_attestation(s2.address(), &signed(&s2).await);

        assert!(matches!(outcome, Some(Ok(_))));
    }

    #[tokio::test]
    async fn invalid_attestation_is_rejected_not_counted() {
        let s1 = PrivateKeySigner::random();
        let mut round = open_round([s1.address(), random_address()], 2);

        let mut att = signed(&s1).await;
        att.sns_ciphertext_digest = OTHER_SNS_DIGEST;
        round.record_attestation(s1.address(), &att);

        assert!(round.attested().is_empty());
        assert_eq!(round.rejected(), vec![s1.address()]);
    }

    #[tokio::test]
    async fn cross_served_attestation_is_rejected() {
        // Validly signed by s1, but served by the bucket registered to s2.
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let mut round = open_round([s1.address(), s2.address()], 2);

        round.record_attestation(s2.address(), &signed(&s1).await);

        assert!(round.attested().is_empty());
        assert_eq!(round.rejected(), vec![s2.address()]);
    }

    #[tokio::test]
    async fn second_different_vote_from_same_signer_does_not_create_second_group() {
        // A different attestation arriving for a signer that already answered (e.g. a stray
        // retry) must not open a second group: at threshold 2 with one other Coprocessor, that
        // would fabricate a consensus from a single voter. One signer, one slot, first write wins.
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let mut round = open_round([s1.address(), s2.address()], 2);

        let first = signed(&s1).await;
        round.record_attestation(s1.address(), &first);
        let second = signed_with_sns(&s1, OTHER_SNS_DIGEST).await;
        let outcome = round.record_attestation(s1.address(), &second);

        assert!(outcome.is_none(), "expected undecided, got {outcome:?}");
        // The replies themselves, not just the outcome: one slot is filled, so no second group can
        // exist, and the largest group still holds the *first* material.
        assert_eq!(round.attested(), vec![s1.address()]);
        let (material, _) = round
            .largest_group()
            .expect("the first reply opened a group");
        assert_eq!(
            material,
            ConsensusMaterial::from(&first),
            "the replay must not displace the first material"
        );

        // A second, real signer now completes the group the first reply opened.
        let outcome = round.record_attestation(s2.address(), &signed(&s2).await);
        assert!(matches!(outcome, Some(Ok(_))));
    }

    #[tokio::test]
    async fn majority_group_wins_over_minority() {
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let s3 = PrivateKeySigner::random();
        let mut round = open_round([s1.address(), s2.address(), s3.address()], 2);

        assert!(
            SNS_DIGEST < OTHER_SNS_DIGEST,
            "the majority must hold the larger material, or a material-only comparator would pass"
        );

        // The minority group opens first, and the majority forms on the losing material.
        round.record_attestation(s3.address(), &signed(&s3).await);
        let other = signed_with_sns(&s1, OTHER_SNS_DIGEST).await;
        round.record_attestation(s1.address(), &other);
        let other = signed_with_sns(&s2, OTHER_SNS_DIGEST).await;

        match round.record_attestation(s2.address(), &other) {
            Some(Ok(ResolvedConsensus {
                material, winners, ..
            })) => {
                assert_eq!(winners.len(), 2);
                assert_eq!(material.sns_ciphertext_digest, OTHER_SNS_DIGEST);
            }
            other => panic!("expected a reached consensus, got {other:?}"),
        }
        assert_eq!(round.largest_group_size(), 2);
    }

    #[tokio::test]
    async fn equal_size_groups_use_deterministic_material_tie_break() {
        // Equal-size groups must still resolve deterministically, or the outcome would depend on
        // reply arrival order.
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        // Threshold 3 with only 2 Coprocessors keeps this off the reached path: the test is about
        // the tie-break, not about winning.
        let mut round = open_round([s1.address(), s2.address()], 3);

        let att1 = signed(&s1).await;
        let att2 = signed_with_sns(&s2, OTHER_SNS_DIGEST).await;
        let (material1, material2) = (
            ConsensusMaterial::from(&att1),
            ConsensusMaterial::from(&att2),
        );
        assert_ne!(
            material1, material2,
            "the two groups must disagree for this test to mean anything"
        );

        round.record_attestation(s1.address(), &att1);
        let round = match round.record_attestation(s2.address(), &att2) {
            Some(Err(ConsensusCheckError::Unreachable(round))) => round,
            other => panic!("expected Unreachable (kept off the reached path), got {other:?}"),
        };
        let (winning_material, winners) = round.largest_group().expect("both replies attested");
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
            "the winning group must hold the smaller material on a tie"
        );
    }

    #[tokio::test]
    async fn close_turns_outstanding_slots_into_no_reply() {
        // What a panicked fetch task leaves behind: s1 attests alone and s2's slot is never
        // filled. Threshold 2 is still reachable while s2 is outstanding, so only `close` can end
        // the round, on `NotReachedThisRound` (a lone attestation is a shortfall, not a
        // disagreement).
        let s1 = PrivateKeySigner::random();
        let s2 = random_address();
        let mut round = open_round([s1.address(), s2], 2);

        assert!(
            round
                .record_attestation(s1.address(), &signed(&s1).await)
                .is_none()
        );

        match round.close() {
            Err(ConsensusCheckError::NotReachedThisRound(round)) => {
                assert_eq!(round.attested(), vec![s1.address()]);
                assert_eq!(round.never_replied(), vec![s2]);
                assert!(round.outstanding().is_empty());
            }
            other => panic!("expected NotReachedThisRound, got {other:?}"),
        }
    }

    #[test]
    fn close_decides_a_round_with_a_duplicate_signer() {
        // A hand-built registry can list the same signer twice. Replies only ever fill the first
        // slot, so `close` must fill the second one itself rather than look it up by signer.
        let s1 = random_address();
        let mut round = open_round([s1, s1], 1);

        assert!(round.record_no_reply(s1).is_none());
        assert!(round.record_no_reply(s1).is_none());

        assert!(matches!(
            round.close(),
            Err(ConsensusCheckError::NotReachedThisRound(_))
        ));
    }
}
