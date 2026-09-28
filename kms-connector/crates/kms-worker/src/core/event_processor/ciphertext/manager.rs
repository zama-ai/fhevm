use super::{VerifiedCiphertexts, s3};
use crate::core::{
    config::Config,
    event_processor::{ProcessingError, RequestCheckError, RequestCheckKind},
};
use alloy::{
    primitives::{B256, U256},
    providers::Provider,
};
use anyhow::anyhow;
use ciphertext_attestation::{
    BoundedClient, CoprocessorRegistry, CoprocessorRegistrySnapshot, CriticalFailurePolicy,
    fetch_attestations_and_check_consensus,
};
use futures::future::try_join_all;
use kms_connector_api::ErrorCode;
use kms_grpc::kms::v1::TypedCiphertext;
use std::num::NonZeroUsize;
use tokio_util::sync::CancellationToken;
use tracing::{debug, info};

/// Manages the ciphertext materials of incoming decryption requests: off-chain attestation
/// consensus verification and S3 retrieval.
///
/// Cheap to clone: every field is a handle to a shared resource or a small value.
#[derive(Clone)]
pub struct CiphertextManager<P: Provider> {
    /// Periodically-synced mirror of the on-chain Coprocessor registry.
    registry: CoprocessorRegistry<P>,

    /// HTTP client for the attestation HEAD fan-out and the ciphertext retrieval.
    s3_client: BoundedClient,

    /// Number of attempts of a ciphertext retrieval, per winning-group bucket.
    retrieval_attempts: NonZeroUsize,
}

impl<P> CiphertextManager<P>
where
    P: Provider + Clone + 'static,
{
    pub async fn connect(
        provider: P,
        config: &Config,
        cancel_token: CancellationToken,
    ) -> anyhow::Result<Self> {
        let registry = CoprocessorRegistry::connect(
            provider,
            config.gateway_config_contract.address,
            config.copro_registry_refresh,
            cancel_token,
            CriticalFailurePolicy::Shutdown,
        )
        .await?;

        Ok(Self {
            registry,
            s3_client: s3::s3_client_from_config(config)?,
            retrieval_attempts: config.s3_ciphertext_retrieval_attempts,
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
        let consensus = fetch_attestations_and_check_consensus(&self.s3_client, handle, registry)
            .await
            .map_err(|e| {
                RequestCheckError::recoverable(
                    RequestCheckKind::CoproConsensus,
                    ErrorCode::CoproConsensusFailed,
                    anyhow!("consensus unreachable for handle {handle}: {e}"),
                )
                .record()
            })?;

        debug!(
            %handle,
            valid_signers = consensus.signers.len(),
            threshold = registry.threshold.get(),
            winning_buckets = consensus.winning_buckets.len(),
            "Consensus reached for handle"
        );

        let ciphertext = s3::retrieve_verified_ciphertext(
            &self.s3_client,
            handle,
            &consensus.material,
            &consensus.winning_buckets,
            self.retrieval_attempts,
        )
        .await?;

        Ok(ResolvedHandle {
            key_id: consensus.material.key_id,
            ciphertext,
        })
    }
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
        return Err(ProcessingError::recoverable(
            ErrorCode::CoproConsensusFailed,
            anyhow!("no handles resolved"),
        ));
    };

    let mut ciphertexts = Vec::with_capacity(resolved_handles.len());
    for resolved_handle in resolved_handles.into_iter() {
        if resolved_handle.key_id != key_id {
            return Err(ProcessingError::irrecoverable(
                ErrorCode::Unprocessable,
                anyhow!(
                    "handles of the request resolve to different key ids: {:#066x} and {:#066x}",
                    key_id,
                    resolved_handle.key_id
                ),
            ));
        }

        ciphertexts.push(resolved_handle.ciphertext);
    }

    Ok(VerifiedCiphertexts {
        ciphertexts,
        key_id,
    })
}

#[cfg(test)]
impl<P> CiphertextManager<P>
where
    P: Provider + Clone + 'static,
{
    /// Test constructor: an empty registry and default config.
    pub fn for_test(provider: P) -> Self {
        Self {
            registry: CoprocessorRegistry::for_test(provider),
            s3_client: s3::s3_client_from_config(&Config::default()).expect("S3 client"),
            retrieval_attempts: Config::default().s3_ciphertext_retrieval_attempts,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::event_processor::ProcessingErrorKind;

    fn resolved(key_id: U256, handle_byte: u8) -> ResolvedHandle {
        ResolvedHandle {
            key_id,
            ciphertext: TypedCiphertext {
                ciphertext: vec![handle_byte].into(),
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

        assert!(matches!(result, Err(ref e) if e.kind == ProcessingErrorKind::Irrecoverable));
    }

    /// An empty resolution set (no handles) is rejected as recoverable.
    #[test]
    fn aggregate_rejects_empty() {
        let result = aggregate_resolved_handles(vec![]);
        assert!(matches!(result, Err(ref e) if e.kind == ProcessingErrorKind::Recoverable));
    }
}
