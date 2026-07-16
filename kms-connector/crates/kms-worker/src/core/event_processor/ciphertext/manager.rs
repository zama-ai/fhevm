use super::{
    registry::{CoprocessorRegistry, CoprocessorRegistrySnapshot},
    s3,
};
use crate::core::{
    config::{Config, CtAttestationConfig},
    event_processor::{
        ProcessingError, RequestCheckError, RequestCheckKind,
        ciphertext::{COPROCESSOR_CONTEXT_ID, VerifiedCiphertexts},
    },
};
use alloy::{
    primitives::{Address, B256, U256},
    providers::Provider,
    transports::http::Client,
};
use anyhow::anyhow;
use ciphertext_attestation::{
    CiphertextAttestation,
    consensus::{self, Consensus, ConsensusMaterial},
};
use futures::future::try_join_all;
use kms_grpc::kms::v1::TypedCiphertext;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, warn};

/// Manages the ciphertext materials of incoming decryption requests: off-chain attestation
/// consensus verification and S3 retrieval.
///
/// Cheap to clone: every field is a handle to a shared resource or a small value.
#[derive(Clone)]
pub struct CiphertextManager<P: Provider> {
    /// Periodically-synced mirror of the on-chain Coprocessor registry.
    registry: CoprocessorRegistry<P>,

    /// HTTP client for the attestation HEAD fan-out and the ciphertext retrieval.
    client: Client,

    /// Off-chain ciphertext-attestation verification config.
    config: CtAttestationConfig,

    /// Number of retries for S3 ciphertext retrieval.
    s3_ciphertext_retrieval_retries: u8,
}

impl<P> CiphertextManager<P>
where
    P: Provider + Clone + 'static,
{
    pub async fn connect(
        provider: P,
        client: Client,
        config: &Config,
        cancel_token: CancellationToken,
    ) -> anyhow::Result<Self> {
        let registry = CoprocessorRegistry::connect(provider, config, cancel_token).await?;

        Ok(Self {
            registry,
            client,
            config: config.ct_attestation.clone(),
            s3_ciphertext_retrieval_retries: config.s3_ciphertext_retrieval_retries,
        })
    }

    /// Resolves and retrieves the verified ciphertexts of a decryption request from `handles`.
    ///
    /// Each handle is resolved independently against the off-chain attestation consensus and its
    /// ciphertext fetched from a winning-group bucket. The request fails as soon as any single
    /// handle fails.
    pub async fn verify_and_retrieve(
        &self,
        handles: &[B256],
    ) -> Result<VerifiedCiphertexts, ProcessingError> {
        let registry = self.registry.snapshot();
        info!(
            "Resolving {} handle(s) via off-chain attestation consensus...",
            handles.len()
        );

        let resolved_handles = try_join_all(
            handles
                .iter()
                .map(|&handle| self.verify_and_retrieve_handle(&registry, handle)),
        )
        .await?;

        let verified = aggregate_resolved_handles(resolved_handles)?;

        info!(
            "All {} handle(s) resolved and verified! (key_id: {:#066x})",
            verified.ciphertexts.len(),
            verified.key_id,
        );
        Ok(verified)
    }

    /// Resolves a single handle's material via consensus and fetches its verified ciphertext.
    async fn verify_and_retrieve_handle(
        &self,
        registry: &CoprocessorRegistrySnapshot,
        handle: B256,
    ) -> Result<ResolvedHandle, ProcessingError> {
        let ResolvedConsensus {
            consensus,
            winning_buckets,
        } = self
            .fetch_attestations_and_check_consensus(handle, registry)
            .await
            .map_err(|e| {
                RequestCheckError::recoverable(
                    RequestCheckKind::CoproConsensus,
                    anyhow!("consensus unreachable for handle {handle}: {e}"),
                )
                .record()
            })?;

        debug!(
            %handle,
            valid_signers = consensus.signers.len(),
            threshold = registry.threshold.get(),
            winning_buckets = winning_buckets.len(),
            "Consensus reached for handle"
        );

        let ciphertext = s3::retrieve_verified_ciphertext(
            &self.client,
            handle,
            &consensus.material,
            &winning_buckets,
            self.s3_ciphertext_retrieval_retries,
        )
        .await?;

        Ok(ResolvedHandle {
            key_id: consensus.material.key_id,
            ciphertext,
        })
    }

    /// Fetches the attestation for a `handle` from the registered Coprocessor buckets concurrently
    /// and evaluates the consensus.
    ///
    /// Tries to evaluate the consensus as soon as enough attestations are received, without waiting
    /// for slow or unreachable buckets.
    ///
    /// On success returns the winning [`Consensus`] together with the URLs of the winning-group
    /// buckets (those that served a valid attestation for the winning material) to fetch the
    /// ciphertext from.
    async fn fetch_attestations_and_check_consensus(
        &self,
        handle: B256,
        registry: &CoprocessorRegistrySnapshot,
    ) -> anyhow::Result<ResolvedConsensus> {
        let mut fetch_attestation_tasks = JoinSet::new();
        for (tx_sender, bucket) in registry.tx_sender_to_bucket.iter() {
            let (client, head_timeout) = (self.client.clone(), self.config.head_timeout);
            let (bucket, tx_sender) = (bucket.clone(), *tx_sender);
            fetch_attestation_tasks.spawn(async move {
                let result =
                    s3::fetch_single_attestation(&client, &bucket, handle, head_timeout).await;
                (tx_sender, result)
            });
        }

        let mut attestations: Vec<(Address, CiphertextAttestation)> = vec![];
        let mut last_error = anyhow!("no attestation fetched yet");
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

            if attestations.len() < registry.threshold.get() {
                last_error = anyhow!(
                    "not enough attestations successfully fetched ({})",
                    attestations.len()
                );
                continue;
            }

            match consensus::evaluate(
                handle,
                COPROCESSOR_CONTEXT_ID,
                &attestations,
                &registry.signers,
                registry.threshold,
            ) {
                Ok(consensus) => {
                    let winning_buckets =
                        winning_group_buckets(&attestations, &consensus, registry);
                    // Early-exit: dropping remaining tasks aborts the other in-flight HEAD requests.
                    return Ok(ResolvedConsensus {
                        consensus,
                        winning_buckets,
                    });
                }
                Err(e) => last_error = e.into(),
            }
        }
        Err(last_error)
    }
}

/// A reached consensus together with the buckets to fetch the ciphertext from.
struct ResolvedConsensus {
    consensus: Consensus,
    winning_buckets: Vec<String>,
}

/// A handle resolved through consensus: the agreed key plus its verified ciphertext.
struct ResolvedHandle {
    key_id: U256,
    ciphertext: TypedCiphertext,
}

/// Aggregates the independently-resolved handles of a request into a single [`VerifiedCiphertexts`].
///
/// The KMS request carries a single `key_id`, so every handle must resolve to the same one; a
/// request whose handles resolve to different key ids is rejected as `Irrecoverable`. Ciphertext
/// order is preserved.
fn aggregate_resolved_handles(
    resolved_handles: Vec<ResolvedHandle>,
) -> Result<VerifiedCiphertexts, ProcessingError> {
    let Some(key_id) = resolved_handles.first().map(|r| r.key_id) else {
        return Err(ProcessingError::Recoverable(anyhow!("no handles resolved")));
    };

    let mut ciphertexts = Vec::with_capacity(resolved_handles.len());
    for resolved_handle in resolved_handles.into_iter() {
        if resolved_handle.key_id != key_id {
            return Err(ProcessingError::Irrecoverable(anyhow!(
                "handles of the request resolve to different key ids: {:#066x} and {:#066x}",
                key_id,
                resolved_handle.key_id
            )));
        }

        ciphertexts.push(resolved_handle.ciphertext);
    }

    Ok(VerifiedCiphertexts {
        ciphertexts,
        key_id,
    })
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

#[cfg(test)]
impl<P> CiphertextManager<P>
where
    P: Provider + Clone + 'static,
{
    /// Test constructor: an empty registry and default config. The resolution path is never
    /// exercised by the tests that use it (they fail earlier, at the ACL or signature stage).
    pub fn for_test(provider: P, client: Client) -> Self {
        Self {
            registry: CoprocessorRegistry::empty(provider),
            client,
            config: CtAttestationConfig::default(),
            s3_ciphertext_retrieval_retries: Config::default().s3_ciphertext_retrieval_retries,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resolved(key_id: U256, handle_byte: u8) -> ResolvedHandle {
        ResolvedHandle {
            key_id,
            ciphertext: TypedCiphertext {
                ciphertext: vec![handle_byte],
                external_handle: vec![handle_byte; 32],
                fhe_type: handle_byte as i32,
                ciphertext_format: 0,
            },
        }
    }

    /// Handles all resolving to the same `key_id` are aggregated in order.
    #[test]
    fn aggregate_accepts_matching_key_ids() {
        let key_id = U256::from(7u64);
        let verified =
            aggregate_resolved_handles(vec![resolved(key_id, 1), resolved(key_id, 2)]).unwrap();

        assert_eq!(verified.key_id, key_id);
        assert_eq!(verified.ciphertexts.len(), 2);
        // Order is preserved.
        assert_eq!(verified.ciphertexts[0].fhe_type, 1);
        assert_eq!(verified.ciphertexts[1].fhe_type, 2);
    }

    /// A request whose handles resolve to different `key_id`s is rejected as irrecoverable.
    #[test]
    fn aggregate_rejects_divergent_key_ids() {
        let result = aggregate_resolved_handles(vec![
            resolved(U256::from(1u64), 1),
            resolved(U256::from(2u64), 2),
        ]);

        assert!(matches!(result, Err(ProcessingError::Irrecoverable(_))));
    }

    /// An empty resolution set (no handles) is rejected as recoverable.
    #[test]
    fn aggregate_rejects_empty() {
        let result = aggregate_resolved_handles(vec![]);
        assert!(matches!(result, Err(ProcessingError::Recoverable(_))));
    }
}
