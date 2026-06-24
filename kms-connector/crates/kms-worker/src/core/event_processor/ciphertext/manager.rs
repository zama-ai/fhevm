use super::{
    registry::{CoprocessorRegistry, CoprocessorRegistrySnapshot},
    s3,
};
use crate::core::{
    config::{Config, CtAttestationConfig},
    event_processor::ciphertext::COPROCESSOR_CONTEXT_ID,
};
use alloy::{hex, primitives::B256, providers::Provider, transports::http::Client};
use anyhow::{Context, anyhow};
use ciphertext_attestation::consensus::{self, Consensus, ConsensusMaterial};
use fhevm_gateway_bindings::decryption::Decryption::SnsCiphertextMaterial;
use futures::future::try_join_all;
use kms_grpc::kms::v1::TypedCiphertext;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, warn};

/// Manages the ciphertext materials of incoming decryption requests: off-chain attestation
/// verification and S3 retrieval.
///
/// Cheap to clone: every field is a handle to a shared resource or a small value.
#[derive(Clone)]
pub struct CiphertextManager<P: Provider> {
    /// Periodically-synced mirror of the on-chain Coprocessor registry.
    registry: CoprocessorRegistry<P>,

    /// HTTP client for the attestation HEAD fan-out and the ciphertext retrieval.
    client: Client,

    /// Off-chain ciphertext-attestation config.
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
        if !config.ct_attestation.enabled {
            info!("Ciphertext-attestation verification disabled by config");
        }

        let registry = CoprocessorRegistry::connect(provider, config, cancel_token).await?;

        Ok(Self {
            registry,
            client,
            config: config.ct_attestation.clone(),
            s3_ciphertext_retrieval_retries: config.s3_ciphertext_retrieval_retries,
        })
    }

    /// Retrieves the ciphertexts of `sns_materials` from the Coprocessors' S3 buckets, after
    /// running the off-chain attestation check.
    ///
    /// Shadow mode (RFC 023): an attestation failure is only logged for now — the on-chain
    /// snapshot stays authoritative and retrieval proceeds regardless of the verdict.
    pub async fn retrieve_verified_ciphertexts(
        &self,
        sns_materials: &[SnsCiphertextMaterial],
    ) -> anyhow::Result<Vec<TypedCiphertext>> {
        if let Err(e) = self.verify_attestations(sns_materials).await {
            warn!("{e:#}");
        }

        s3::retrieve_sns_ciphertext_materials(
            &self.client,
            &self.registry.snapshot(),
            sns_materials,
            self.s3_ciphertext_retrieval_retries,
        )
        .await
    }

    /// Verifies the off-chain attestations of `materials` against the on-chain snapshot.
    ///
    /// Fans HEAD requests out to every registered Coprocessor bucket and computes an off-chain
    /// consensus verdict for each handle. Stops at the first failing material: a single failed
    /// verification invalidates the whole decryption request.
    pub async fn verify_attestations(
        &self,
        materials: &[SnsCiphertextMaterial],
    ) -> anyhow::Result<()> {
        if !self.config.enabled {
            return Ok(());
        }
        let registry = self.registry.snapshot();

        info!(
            "Starting attestation verification for {} materials...",
            materials.len()
        );
        let verification_futures = materials.iter().map(|material| async {
            self.verify_material_attestations(&registry, material)
                .await
                .with_context(|| {
                    format!(
                        "Attestation verification failed for handle {}",
                        hex::encode(material.ctHandle)
                    )
                })
        });
        try_join_all(verification_futures).await?;
        info!("All materials passed the attestations verification!");

        Ok(())
    }

    /// Verifies the off-chain attestations of a single material.
    async fn verify_material_attestations(
        &self,
        registry: &CoprocessorRegistrySnapshot,
        material: &SnsCiphertextMaterial,
    ) -> anyhow::Result<()> {
        let handle = material.ctHandle;
        let consensus = self
            .fetch_attestations_and_check_consensus(handle, registry)
            .await?;
        compare_onchain(material, &consensus.material)?;

        debug!(
            %handle,
            valid_signers = consensus.valid_signers,
            threshold = registry.threshold.get(),
            "Successful attestation verification"
        );
        Ok(())
    }

    /// Fetches the attestation for a `handle` from the registered Coprocessor buckets concurrently.
    ///
    /// Tries to evaluate the consensus as soon as enough attestations are received, without waiting
    /// for slow or unreachable buckets.
    async fn fetch_attestations_and_check_consensus(
        &self,
        handle: B256,
        registry: &CoprocessorRegistrySnapshot,
    ) -> anyhow::Result<Consensus> {
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

        let mut attestations = vec![];
        let mut verdict = Err(anyhow!("Not enough attestations fetched yet (0)..."));
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
                verdict = Err(anyhow!(
                    "Not enough attestations fetched yet ({})...",
                    attestations.len()
                ));
                continue;
            }

            verdict = consensus::evaluate(
                handle,
                COPROCESSOR_CONTEXT_ID,
                &attestations,
                &registry.signers,
                registry.threshold,
            )
            .map_err(anyhow::Error::from);
            if verdict.is_ok() {
                // Early-exit: dropping remaining tasks aborts the other in-flight HEAD requests.
                return verdict;
            }
        }
        verdict
    }
}

/// Compares the consensus material against the on-chain `SnsCiphertextMaterial`.
///
/// Only `keyId` and `snsCiphertextDigest` exist on-chain; `ciphertext_digest` and `format` are
/// off-chain-only (bound by the signature).
fn compare_onchain(
    onchain: &SnsCiphertextMaterial,
    material: &ConsensusMaterial,
) -> anyhow::Result<()> {
    if onchain.keyId != material.key_id {
        return Err(anyhow!(
            "on-chain tuple mismatch on `key_id`: onchain {}, attested {}",
            onchain.keyId,
            material.key_id
        ));
    }
    if onchain.snsCiphertextDigest != material.sns_ciphertext_digest {
        return Err(anyhow!(
            "on-chain tuple mismatch on `sns_ciphertext_digest`: onchain {}, attested {}",
            onchain.snsCiphertextDigest,
            material.sns_ciphertext_digest
        ));
    }
    Ok(())
}

#[cfg(test)]
impl<P> CiphertextManager<P>
where
    P: Provider + Clone + 'static,
{
    pub fn disabled(provider: P, client: Client) -> Self {
        Self {
            registry: CoprocessorRegistry::empty(provider),
            client,
            config: CtAttestationConfig {
                enabled: false,
                ..CtAttestationConfig::default()
            },
            s3_ciphertext_retrieval_retries: Config::default().s3_ciphertext_retrieval_retries,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{B256, U256};
    use ciphertext_attestation::CiphertextFormat;

    fn onchain_material(key_id: U256, sns_digest: B256) -> SnsCiphertextMaterial {
        SnsCiphertextMaterial {
            keyId: key_id,
            snsCiphertextDigest: sns_digest,
            ..Default::default()
        }
    }

    fn consensus_material(key_id: U256, sns_digest: B256) -> ConsensusMaterial {
        ConsensusMaterial {
            key_id,
            // Off-chain-only fields: not part of the on-chain comparison.
            ciphertext_digest: B256::repeat_byte(0xFF),
            sns_ciphertext_digest: sns_digest,
            format: CiphertextFormat::CompressedOnCpu,
        }
    }

    #[test]
    fn accepts_matching_tuple() {
        let key_id = U256::from(69);
        let sns_digest = B256::repeat_byte(0xAB);

        let onchain = onchain_material(key_id, sns_digest);
        let material = consensus_material(key_id, sns_digest);

        assert!(compare_onchain(&onchain, &material).is_ok());
    }

    #[test]
    fn ignores_offchain_only_fields() {
        // `ciphertext_digest` and `format` live only off-chain (bound by the signature), so they
        // must not influence the comparison: the matching `key_id`/`sns_digest` tuple is enough.
        let key_id = U256::from(7);
        let sns_digest = B256::repeat_byte(0xCD);

        let onchain = onchain_material(key_id, sns_digest);
        let mut material = consensus_material(key_id, sns_digest);
        material.ciphertext_digest = B256::ZERO;
        material.format = CiphertextFormat::UncompressedOnGpu;

        assert!(compare_onchain(&onchain, &material).is_ok());
    }

    #[test]
    fn rejects_key_id_mismatch() {
        let sns_digest = B256::repeat_byte(0xAB);

        let onchain = onchain_material(U256::from(1), sns_digest);
        let material = consensus_material(U256::from(2), sns_digest);

        let err = compare_onchain(&onchain, &material).unwrap_err();
        assert!(
            err.to_string().contains("key_id"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn rejects_sns_ciphertext_digest_mismatch() {
        let key_id = U256::from(7);

        let onchain = onchain_material(key_id, B256::repeat_byte(0x01));
        let material = consensus_material(key_id, B256::repeat_byte(0x02));

        let err = compare_onchain(&onchain, &material).unwrap_err();
        assert!(
            err.to_string().contains("sns_ciphertext_digest"),
            "unexpected error: {err}"
        );
    }
}
