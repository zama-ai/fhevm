//! Fans out a handle's attestation fetch across every registered Coprocessor bucket and
//! evaluates consensus over the results.

use crate::{
    CiphertextAttestation,
    client::{
        registry::CoprocessorRegistrySnapshot,
        s3::{BoundedClient, BucketError},
    },
    consensus::ConsensusMaterial,
    tracker::{ConsensusTracker, Reply, Round, ThresholdStatus, ValidAttestation},
};
use alloy::primitives::{Address, B256, U256};
use std::{collections::HashSet, time::Duration};
use tokio::task::JoinSet;
use tracing::{debug, error, warn};

/// Why a handle has no attestation consensus this round.
///
/// The two variants call for opposite responses, which is why they are distinguished here rather
/// than collapsed into one opaque error: [`Self::MissedThisRound`] is the expected early state,
/// since Coprocessors publish attestations asynchronously, so it is worth retrying.
/// [`Self::Unreachable`] means the signers that answered disagree and no vote still to come could
/// break the tie — retrying re-reads the same disagreement.
#[derive(Debug, thiserror::Error)]
pub enum ConsensusCheckError {
    /// Every registered Coprocessor answered or failed this round and no group reached the
    /// threshold. Retriable: attestations are published asynchronously, so this is the normal
    /// early state.
    #[error("no attestation consensus yet for handle {handle}: {round}")]
    MissedThisRound { handle: B256, round: Round },

    /// The Coprocessors that answered disagree, and even every vote still outstanding joining the
    /// largest group would fall short of the threshold. Terminal: cast votes do not change.
    #[error("attestation consensus unreachable for handle {handle}: {round}")]
    Unreachable { handle: B256, round: Round },
}

/// A reached consensus together with the buckets to fetch the ciphertext from.
///
/// `winning_buckets` (the buckets whose registered signer is in the winning group) exists only so
/// a ciphertext-retrieving consumer (e.g. the KMS Connector) can fetch the ciphertext bytes from a
/// winning-group bucket; a consumer that only needs the consensus verdict can ignore it.
#[derive(Debug)]
pub struct ResolvedConsensus {
    pub material: ConsensusMaterial,
    pub signers: Vec<Address>,
    pub winning_buckets: Vec<String>,
}

/// Fetches the attestation for a `handle` from the registered Coprocessor buckets concurrently
/// and evaluates the consensus.
///
/// Tries to evaluate the consensus as soon as enough attestations are received, without waiting
/// for slow or unreachable buckets.
///
/// On success returns the winning material together with the URLs of the winning-group buckets
/// (those whose registered signer vouches for the winning material).
pub async fn fetch_attestations_and_check_consensus(
    client: &BoundedClient,
    handle: B256,
    registry: &CoprocessorRegistrySnapshot,
    head_timeout: Duration,
    context_id: U256,
) -> Result<ResolvedConsensus, ConsensusCheckError> {
    let mut fetch_attestation_tasks = JoinSet::new();
    for entry in &registry.coprocessors {
        let (client, head_timeout) = (client.clone(), head_timeout);
        let (bucket, signer) = (entry.bucket.clone(), entry.signer);
        fetch_attestation_tasks.spawn(async move {
            let result = client
                .fetch_single_attestation(&bucket, handle, head_timeout, context_id)
                .await;
            (signer, result)
        });
    }

    let signers = registry.coprocessors.iter().map(|entry| entry.signer);
    let tracker = ConsensusTracker::new(signers, registry.threshold);

    drain(
        fetch_attestation_tasks,
        handle,
        context_id,
        registry,
        tracker,
    )
    .await
}

/// Drains `tasks`, feeding each reply to `tracker`, and returns the round's verdict.
///
/// Split out from [`fetch_attestations_and_check_consensus`] so the post-drain sweep (see below)
/// can be unit-tested against a `JoinSet` built directly, including one task that genuinely
/// panics — something no HTTP mock can trigger, since `reqwest`/`hyper` guarantee a `Result` for
/// every malformed or hostile response, never a panic.
async fn drain(
    mut tasks: JoinSet<(Address, Result<CiphertextAttestation, BucketError>)>,
    handle: B256,
    context_id: U256,
    registry: &CoprocessorRegistrySnapshot,
    mut tracker: ConsensusTracker,
) -> Result<ResolvedConsensus, ConsensusCheckError> {
    // Signers whose probe has not yet joined: still in flight, or its task panicked and its
    // signer was never learned (see the `Err(e)` arm below). Client-owned probe-fate knowledge,
    // used only to log what an early exit on `Reached` abandons.
    let mut outstanding: HashSet<Address> =
        registry.coprocessors.iter().map(|e| e.signer).collect();

    // Every one of the `registry.coprocessors.len()` spawned tasks either feeds the tracker
    // exactly one reply below, or panics and is left for the post-loop sweep — that is what makes
    // the verdict, once the `JoinSet` drains, always terminal.
    while let Some(joined) = tasks.join_next().await {
        let (signer, reply) = match joined {
            Err(e) => {
                // No signer: a `JoinError` carries no result, so this slot cannot be addressed
                // here. It stays `Outstanding` until the post-loop sweep turns it into `NoReply`.
                warn!(%handle, "Attestation fetch task panicked: {e}");
                continue;
            }
            Ok((signer, fetch_result)) => {
                outstanding.remove(&signer);
                let outcome: Result<ConsensusMaterial, BucketError> =
                    fetch_result.and_then(|attestation| {
                        ValidAttestation::validate(&attestation, handle, context_id, signer)
                            .map(|valid| valid.material().clone())
                            .map_err(BucketError::from)
                    });
                match outcome {
                    Ok(material) => (signer, Reply::Attested(material)),
                    Err(BucketError::Invalid(e)) => {
                        warn!(%signer, %handle, "Discarding invalid attestation: {e}");
                        (signer, Reply::Rejected)
                    }
                    Err(e) => {
                        warn!(%signer, %handle, "Failed to fetch attestation: {e}");
                        (signer, Reply::NoReply)
                    }
                }
            }
        };
        debug!(%signer, %handle, ?reply, "Coprocessor reply recorded");

        let status = tracker.record(signer, reply);
        if let Some(result) = resolve(handle, registry, status) {
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

    // The `JoinSet` is drained with no verdict returned above: only possible when a probe
    // panicked and left its slot `Outstanding` (see the `Err(e)` arm above) — a healthy last
    // reply always resolves the round from inside the loop, since at that point no slot is left
    // `Outstanding`. `record` is idempotent for an already-filled slot, so sweeping every
    // registered signer to `NoReply` only fills the ones a panicked probe left open, then the
    // verdict is recomputed one last time. With an empty registry this loop never runs and the
    // pre-loop `status` — already terminal for zero signers — is used as-is.
    let mut status = tracker.status();
    for entry in &registry.coprocessors {
        status = tracker.record(entry.signer, Reply::NoReply);
    }
    resolve(handle, registry, status).unwrap_or_else(|| {
        // Every registered signer's slot is filled after the sweep above, which should make the
        // verdict terminal; reaching this arm means it somehow did not. Failing open (letting an
        // unattested handle through) would be worse than failing shut on a request-serving path,
        // so this logs loudly and hands back the same retriable verdict an all-silent round would
        // produce, rather than panicking here as a prior revision of this function did.
        //
        // Provably unreachable *today*, but only because the registry cannot contain a duplicate
        // signer: `ConsensusTracker::new` would give a duplicated address two slots, `record`'s
        // `find_map` only ever fills the first one it finds, and the second would stay
        // `Outstanding` forever — surviving the sweep above (which calls `record` once per
        // registry entry, including the duplicate, and still only reaches that same first slot)
        // and landing exactly here, where the fabricated all-`NoReply` board below would then
        // silently misreport that Coprocessor's slot. What closes this off is
        // `CoprocessorRegistrySnapshot::load` rejecting a duplicate signer as
        // `RegistryError::Critical` before a registry ever reaches this code, pinned by
        // `registry::tests::load_rejects_duplicate_signer_as_critical`. If that invariant is ever
        // relaxed, this fallback stops being a defensive no-op and starts being reachable.
        error!(
            %handle,
            "Post-sweep consensus verdict was still open after every registered signer's slot \
             was filled; this should be unreachable. Falling back to a retriable verdict."
        );
        Err(ConsensusCheckError::MissedThisRound {
            handle,
            round: Round {
                threshold: registry.threshold,
                replies: registry
                    .coprocessors
                    .iter()
                    .map(|entry| (entry.signer, Reply::NoReply))
                    .collect(),
            },
        })
    })
}

/// Converts a freshly recomputed verdict into this function's return type, or `None` if the round
/// is still open.
fn resolve(
    handle: B256,
    registry: &CoprocessorRegistrySnapshot,
    status: ThresholdStatus,
) -> Option<Result<ResolvedConsensus, ConsensusCheckError>> {
    match status {
        ThresholdStatus::AwaitingReplies => None,
        ThresholdStatus::Reached { material, signers } => {
            let winning_buckets = winning_buckets(registry, &signers);
            Some(Ok(ResolvedConsensus {
                material,
                signers,
                winning_buckets,
            }))
        }
        ThresholdStatus::MissedThisRound(round) => {
            Some(Err(ConsensusCheckError::MissedThisRound { handle, round }))
        }
        ThresholdStatus::Unreachable(round) => {
            Some(Err(ConsensusCheckError::Unreachable { handle, round }))
        }
    }
}

/// Collects the URLs of the buckets whose registered signer is in the winning group.
fn winning_buckets(registry: &CoprocessorRegistrySnapshot, signers: &[Address]) -> Vec<String> {
    registry
        .coprocessors
        .iter()
        .filter(|entry| signers.contains(&entry.signer))
        .map(|entry| entry.bucket.clone())
        .collect()
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
    /// panic or abort) and which no HTTP mock can trigger (see [`drain`]'s doc comment).
    fn panicking_task(tasks: &mut JoinSet<(Address, Result<CiphertextAttestation, BucketError>)>) {
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

        let signers = registry.coprocessors.iter().map(|e| e.signer);
        let tracker = ConsensusTracker::new(signers, registry.threshold);

        let result = drain(tasks, HANDLE, CONTEXT_ID, &registry, tracker).await;

        match result {
            Err(ConsensusCheckError::MissedThisRound { round, .. }) => {
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
