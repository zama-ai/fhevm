//! Fans out a handle's attestation fetch across every registered Coprocessor bucket and
//! evaluates consensus over the results.

use crate::{
    CiphertextAttestation,
    client::{
        registry::CoprocessorRegistrySnapshot,
        s3::{BoundedClient, FetchAttestationError},
    },
    consensus::ConsensusMaterial,
    tracker::{ConsensusTracker, Reply, Round, ThresholdStatus, validate},
};
use alloy::primitives::{Address, B256, U256};
use std::collections::HashSet;
use tokio::task::JoinSet;
use tracing::{debug, warn};

/// Why a handle has no attestation consensus this round.
#[derive(Debug, thiserror::Error)]
pub enum ConsensusCheckError {
    /// No group reached the threshold. Retriable: attestations are published asynchronously, so
    /// this is the normal early state.
    #[error("no attestation consensus yet: {0}")]
    MissedThisRound(Round),

    /// The Coprocessors that answered disagree. Terminal: cast votes do not change.
    #[error("attestation consensus unreachable: {0}")]
    Unreachable(Round),
}

/// A reached consensus together with the buckets to fetch the ciphertext from.
///
/// `winning_buckets` are the buckets whose registered signer is in the winning group; a consumer
/// that only needs the consensus verdict can ignore it.
#[derive(Debug)]
pub struct ResolvedConsensus {
    pub material: ConsensusMaterial,
    pub signers: Vec<Address>,
    pub winning_buckets: Vec<String>,
}

/// Fetches the attestation for a `handle` from the registered Coprocessor buckets concurrently
/// and evaluates the consensus as soon as enough attestations are received, without waiting
/// for slow or unreachable buckets.
pub async fn fetch_attestations_and_check_consensus(
    client: &BoundedClient,
    handle: B256,
    registry: &CoprocessorRegistrySnapshot,
) -> Result<ResolvedConsensus, ConsensusCheckError> {
    let context_id = client.context_id();

    let mut fetch_attestation_tasks = JoinSet::new();
    for entry in &registry.coprocessors {
        let client = client.clone();
        let (bucket, signer) = (entry.bucket.clone(), entry.signer);
        fetch_attestation_tasks.spawn(async move {
            let result = client.fetch_single_attestation(&bucket, handle).await;
            (signer, result)
        });
    }

    let entries = registry.coprocessors.iter().cloned();
    let tracker = ConsensusTracker::new(handle, entries, registry.threshold);

    resolve_round(
        fetch_attestation_tasks,
        handle,
        context_id,
        registry,
        tracker,
    )
    .await
}

/// Drains `tasks`, feeding each reply to `tracker`, and returns the round's verdict.
async fn resolve_round(
    mut tasks: JoinSet<(
        Address,
        Result<CiphertextAttestation, FetchAttestationError>,
    )>,
    handle: B256,
    context_id: U256,
    registry: &CoprocessorRegistrySnapshot,
    mut tracker: ConsensusTracker,
) -> Result<ResolvedConsensus, ConsensusCheckError> {
    // Signers whose probe has not yet joined: used only to log what an early exit on `Reached`
    // abandons.
    let mut outstanding: HashSet<Address> =
        registry.coprocessors.iter().map(|e| e.signer).collect();

    while let Some(joined) = tasks.join_next().await {
        let (signer, reply) = match joined {
            Err(e) => {
                // A `JoinError` carries no result, so this slot stays `Outstanding` until the
                // post-loop sweep turns it into `NoReply`.
                warn!(%handle, "Attestation fetch task panicked: {e}");
                continue;
            }
            Ok((signer, fetch_result)) => {
                outstanding.remove(&signer);
                let reply = match fetch_result {
                    Err(e) => {
                        warn!(%signer, %handle, "Failed to fetch attestation: {e}");
                        Reply::NoReply
                    }
                    Ok(attestation) => match validate(&attestation, handle, context_id, signer) {
                        Ok(material) => Reply::Attested(material),
                        Err(e) => {
                            warn!(%signer, %handle, "Discarding invalid attestation: {e}");
                            Reply::Rejected
                        }
                    },
                };
                (signer, reply)
            }
        };
        debug!(%signer, %handle, ?reply, "Coprocessor reply recorded");

        let verdict = tracker.record(signer, reply);
        if let Some(result) = resolve(verdict) {
            if result.is_ok() {
                debug!(
                    %handle,
                    abandoned = ?outstanding,
                    "Consensus reached; abandoning probes still in flight"
                );
            }
            // Early-exit: dropping remaining tasks aborts the other in-flight HEAD requests.
            return result;
        }
    }

    // Only reachable when a probe panicked and left its slot `Outstanding`. `record` is
    // idempotent for an already-filled slot, so sweeping every registered signer to `NoReply`
    // only fills the ones a panicked probe left open.
    let mut verdict = tracker.verdict();
    for entry in &registry.coprocessors {
        verdict = tracker.record(entry.signer, Reply::NoReply);
    }
    resolve(verdict)
        .expect("every registered signer's slot is filled after the sweep, so the round is closed")
}

/// Converts a freshly recomputed verdict into this function's return type, or `None` if the round
/// is still open.
fn resolve(verdict: ThresholdStatus) -> Option<Result<ResolvedConsensus, ConsensusCheckError>> {
    match verdict {
        ThresholdStatus::AwaitingReplies => None,
        ThresholdStatus::Reached { material, winners } => Some(Ok(ResolvedConsensus {
            material,
            signers: winners.iter().map(|entry| entry.signer).collect(),
            winning_buckets: winners.into_iter().map(|entry| entry.bucket).collect(),
        })),
        ThresholdStatus::MissedThisRound(round) => {
            Some(Err(ConsensusCheckError::MissedThisRound(round)))
        }
        ThresholdStatus::Unreachable(round) => Some(Err(ConsensusCheckError::Unreachable(round))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::registry::CoprocessorEntry;
    use crate::{CiphertextAttestationPayload, CiphertextFormat, Version};
    use alloy_signer_local::PrivateKeySigner;
    use std::num::NonZeroUsize;

    const HANDLE: B256 = B256::repeat_byte(0xAA);
    const CONTEXT_ID: U256 = U256::ONE;
    const KEY_ID: U256 = U256::ONE;
    const CT_DIGEST: B256 = B256::repeat_byte(0xBB);
    const SNS_DIGEST: B256 = B256::repeat_byte(0xCC);
    const FORMAT: CiphertextFormat = CiphertextFormat::UncompressedOnCpu;

    fn registry(signers: &[Address]) -> CoprocessorRegistrySnapshot {
        let coprocessors = signers
            .iter()
            .map(|&signer| CoprocessorEntry {
                tx_sender: signer,
                signer,
                bucket: format!("http://bucket-{signer}"),
            })
            .collect();
        CoprocessorRegistrySnapshot::new(coprocessors, NonZeroUsize::new(2).unwrap())
    }

    async fn signed_attestation(signer: &PrivateKeySigner) -> CiphertextAttestation {
        CiphertextAttestationPayload::new(
            Version::V1,
            HANDLE,
            KEY_ID,
            CONTEXT_ID,
            CT_DIGEST,
            SNS_DIGEST,
            FORMAT,
        )
        .sign(signer)
        .await
        .unwrap()
    }

    /// A probe that never joins with a signer at all — the case a `JoinError` produces (task
    /// panic or abort) and which no HTTP mock can trigger.
    fn panicking_task(
        tasks: &mut JoinSet<(
            Address,
            Result<CiphertextAttestation, FetchAttestationError>,
        )>,
    ) {
        tasks.spawn(async { panic!("simulates a JoinSet task panicking mid-probe") });
    }

    #[tokio::test]
    async fn sweep_turns_a_panicked_probes_slot_into_a_terminal_verdict() {
        // s1 attests alone. With s2's slot still Outstanding, threshold 2 is still reachable this
        // round (1 attested + 1 outstanding >= 2), so the loop must not resolve on s1's reply
        // alone — it has to wait for s2. s2's task then panics instead of ever producing a reply,
        // so without the post-drain sweep the function would have nothing left to await and no
        // verdict to return. The sweep must turn that permanently-open slot into `NoReply` and
        // recompute, landing on `MissedThisRound` (s1's lone attestation is not a proven
        // disagreement, just a shortfall).
        let s1 = PrivateKeySigner::random();
        let s2 = PrivateKeySigner::random();
        let registry = registry(&[s1.address(), s2.address()]);

        let mut tasks = JoinSet::new();
        let att = signed_attestation(&s1).await;
        let s1_addr = s1.address();
        tasks.spawn(async move { (s1_addr, Ok(att)) });
        panicking_task(&mut tasks);

        let entries = registry.coprocessors.iter().cloned();
        let tracker = ConsensusTracker::new(HANDLE, entries, registry.threshold);

        let result = resolve_round(tasks, HANDLE, CONTEXT_ID, &registry, tracker).await;

        match result {
            Err(ConsensusCheckError::MissedThisRound(round)) => {
                assert_eq!(round.attested(), vec![s1.address()]);
                assert!(
                    round.outstanding().is_empty(),
                    "the swept slot must not still read as Outstanding"
                );
            }
            other => panic!("expected MissedThisRound, got {other:?}"),
        }
    }
}
