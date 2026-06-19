use super::{
    registry::{CoprocessorRegistry, CoprocessorRegistrySnapshot},
    s3,
};
use crate::core::{
    config::{Config, CtAttestationConfig},
    event_processor::ciphertext::COPROCESSOR_CONTEXT_ID,
};
use alloy::{hex, providers::Provider, transports::http::Client};
use anyhow::{Context, anyhow};
use ciphertext_attestation::consensus::{self, ConsensusMaterial};
use fhevm_gateway_bindings::decryption::Decryption::SnsCiphertextMaterial;
use futures::future::try_join_all;
use kms_grpc::kms::v1::TypedCiphertext;
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

    /// Verifies the off-chain attestations of a single material against the on-chain snapshot.
    async fn verify_material_attestations(
        &self,
        registry: &CoprocessorRegistrySnapshot,
        material: &SnsCiphertextMaterial,
    ) -> anyhow::Result<()> {
        let handle = material.ctHandle;
        let attestations =
            s3::fetch_attestations(handle, registry, &self.client, self.config.head_timeout).await;

        let consensus = consensus::evaluate(
            handle,
            COPROCESSOR_CONTEXT_ID,
            &attestations,
            &registry.signers,
            registry.threshold,
        )?;
        compare_onchain(material, &consensus.material)?;

        debug!(
            handle = hex::encode(handle),
            valid_signers = consensus.valid_signers,
            threshold = registry.threshold.get(),
            "Successful attestation verification"
        );
        Ok(())
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
