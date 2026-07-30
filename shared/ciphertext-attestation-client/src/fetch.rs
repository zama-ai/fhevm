//! Fans out a handle's attestation fetch across every registered Coprocessor bucket and
//! evaluates consensus over the results.

use crate::{registry::CoprocessorRegistrySnapshot, s3};
use alloy::{
    primitives::{Address, B256, U256},
    transports::http::Client,
};
use ciphertext_attestation::{
    CiphertextAttestation,
    consensus::{self, Consensus, ConsensusError, ConsensusMaterial},
};
use std::time::Duration;
use tokio::task::JoinSet;
use tracing::warn;

/// Why a handle has no attestation consensus.
///
/// The two variants call for opposite responses, which is why they are distinguished here rather
/// than collapsed into one opaque error: [`Self::Unavailable`] is the expected early state, since
/// Coprocessors publish attestations asynchronously, so it is worth retrying. [`Self::NoConsensus`]
/// means enough attestations were read and the registered signers did not agree on the same
/// material — retrying re-reads the same disagreement.
#[derive(Debug, thiserror::Error)]
pub enum ConsensusCheckError {
    /// Fewer than `required` buckets served a usable attestation.
    #[error("only {fetched} of {required} required attestations could be fetched")]
    Unavailable { fetched: usize, required: usize },

    /// Enough attestations were read, but no group of registered signers reached the threshold.
    #[error(transparent)]
    NoConsensus(#[from] ConsensusError),
}

/// A reached consensus together with the buckets to fetch the ciphertext from.
///
/// `winning_buckets` (the buckets whose attestation vouches for the winning material) exists only
/// so a ciphertext-retrieving consumer (e.g. the KMS Connector) can fetch the ciphertext bytes
/// from a winning-group bucket; a consumer that only needs the consensus verdict can ignore it.
pub struct ResolvedConsensus {
    pub consensus: Consensus,
    pub winning_buckets: Vec<String>,
}

/// Fetches the attestation for a `handle` from the registered Coprocessor buckets concurrently
/// and evaluates the consensus.
///
/// Tries to evaluate the consensus as soon as enough attestations are received, without waiting
/// for slow or unreachable buckets.
///
/// On success returns the winning [`Consensus`] together with the URLs of the winning-group
/// buckets (those that served a valid attestation for the winning material).
pub async fn fetch_attestations_and_check_consensus(
    client: &Client,
    handle: B256,
    registry: &CoprocessorRegistrySnapshot,
    head_timeout: Duration,
    context_id: U256,
) -> Result<ResolvedConsensus, ConsensusCheckError> {
    let mut fetch_attestation_tasks = JoinSet::new();
    for (tx_sender, bucket) in registry.tx_sender_to_bucket.iter() {
        let (client, head_timeout) = (client.clone(), head_timeout);
        let (bucket, tx_sender) = (bucket.clone(), *tx_sender);
        fetch_attestation_tasks.spawn(async move {
            let result =
                s3::fetch_single_attestation(&client, &bucket, handle, head_timeout, context_id)
                    .await;
            (tx_sender, result)
        });
    }

    let required = registry.threshold.get();
    let mut attestations: Vec<(Address, CiphertextAttestation)> = vec![];
    let mut last_error = ConsensusCheckError::Unavailable {
        fetched: 0,
        required,
    };
    while let Some(joined) = fetch_attestation_tasks.join_next().await {
        match joined {
            Err(e) => {
                warn!("Attestation fetch task panicked: {e}");
                continue;
            }
            Ok((tx_sender, Err(e))) => {
                warn!(%tx_sender, %handle, "Failed to fetch attestation: {e}");
                continue;
            }
            Ok((tx_sender, Ok(attestation))) => attestations.push((tx_sender, attestation)),
        };

        if attestations.len() < required {
            last_error = ConsensusCheckError::Unavailable {
                fetched: attestations.len(),
                required,
            };
            continue;
        }

        match consensus::evaluate(
            handle,
            context_id,
            &attestations,
            &registry.signers,
            registry.threshold,
        ) {
            Ok(consensus) => {
                let winning_buckets = winning_group_buckets(&attestations, &consensus, registry);
                // Early-exit: dropping remaining tasks aborts the other in-flight HEAD requests.
                return Ok(ResolvedConsensus {
                    consensus,
                    winning_buckets,
                });
            }
            Err(e) => last_error = ConsensusCheckError::NoConsensus(e),
        }
    }
    Err(last_error)
}

/// Collects the URLs of the buckets whose attestation vouches for the winning material.
fn winning_group_buckets(
    attestations: &[(Address, CiphertextAttestation)],
    consensus: &Consensus,
    registry: &CoprocessorRegistrySnapshot,
) -> Vec<String> {
    let mut buckets = Vec::new();
    for (tx_sender, attestation) in attestations {
        if ConsensusMaterial::from(attestation) != consensus.material
            || !consensus.signers.contains(&attestation.signer)
        {
            continue;
        }
        if let Some(bucket) = registry.tx_sender_to_bucket.get(tx_sender)
            && !buckets.contains(bucket)
        {
            buckets.push(bucket.clone());
        }
    }
    buckets
}
