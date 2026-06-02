//! Off-chain ciphertext-attestation verifier.
//!
//! On every decryption request this verifier fans HEAD requests out to every registered
//! Coprocessor bucket, computes an off-chain consensus verdict, and logs a structured comparison
//! against the authoritative on-chain `SnsCiphertextMaterial` snapshot. It never changes
//! decryption behavior: the on-chain result stays authoritative regardless of the verdict.

use super::{
    consensus::{self, Consensus, ConsensusMaterial},
    error::AttestationError,
    fetch::{fetch_and_check_ciphertext, fetch_attestations},
    registry::CoprocessorRegistry,
};
use crate::core::{Config, config::CtAttestationConfig};
use alloy::{
    hex,
    primitives::{Address, B256},
    providers::Provider,
    transports::http::Client,
};
use ciphertext_attestation::CiphertextAttestation;
use connector_utils::tasks::spawn_with_limit;
use fhevm_gateway_bindings::{
    decryption::Decryption::SnsCiphertextMaterial,
    gateway_config::GatewayConfig::{self, GatewayConfigInstance},
};
use std::sync::{Arc, RwLock};
use tracing::{info, warn};

/// Verifies off-chain ciphertext attestations against the on-chain snapshot.
///
/// Cheap to clone: every field is a handle to a shared resource or a small value.
#[derive(Clone)]
pub struct AttestationVerifier<P: Provider> {
    /// Used to (re)load the registry snapshot.
    gateway_config_contract: GatewayConfigInstance<P>,

    /// HTTP client for the S3 HEAD/GET fan-out.
    client: Client,

    /// TTL'd Coprocessor registry. The outer `Arc` shares the lock across clones; the `RwLock`
    /// gives interior mutability for the swap; the inner `Arc` makes reads snapshot-and-release.
    registry: Arc<RwLock<Arc<CoprocessorRegistry>>>,

    config: CtAttestationConfig,
}

impl<P> AttestationVerifier<P>
where
    P: Provider + Clone + 'static,
{
    pub async fn connect(provider: P, client: Client, config: &Config) -> Self {
        let gateway_config_contract =
            GatewayConfig::new(config.gateway_config_contract.address, provider);

        let registry = if config.ct_attestation.enabled {
            CoprocessorRegistry::load(&gateway_config_contract)
                .await
                .unwrap_or_else(|e| {
                    warn!(
                        "Initial Coprocessor registry load failed, starting empty \
                         (refresh task will retry): {e}"
                    );
                    CoprocessorRegistry::default()
                })
        } else {
            info!("Ciphertext-attestation verifier disabled by config");
            CoprocessorRegistry::default()
        };

        let verifier = Self {
            gateway_config_contract,
            client,
            registry: Arc::new(RwLock::new(Arc::new(registry))),
            config: config.ct_attestation.clone(),
        };

        if verifier.config.enabled {
            verifier.spawn_refresh_task();
        }
        verifier
    }

    /// Builds a permanently-disabled verifier (no registry load, no refresh task,
    /// every verification a no-op). For tests that don't exercise shadow mode.
    pub fn disabled(provider: P, client: Client) -> Self {
        Self {
            gateway_config_contract: GatewayConfig::new(Address::ZERO, provider),
            client,
            registry: Arc::new(RwLock::new(Arc::new(CoprocessorRegistry::default()))),
            config: CtAttestationConfig {
                enabled: false,
                ..CtAttestationConfig::default()
            },
        }
    }

    /// Clones the inner `Arc` of the copro registry and drops the guard, so no lock is held.
    pub fn registry(&self) -> Arc<CoprocessorRegistry> {
        self.registry
            .read()
            .expect("copro registry lock poisoned")
            .clone()
    }

    /// Production entry point: fire off best-effort shadow-mode verification of
    /// the on-chain snapshot in a bounded background task, so it never blocks the
    /// gRPC dispatch. No-op when disabled — no clone, no task.
    pub async fn spawn_verification(&self, sns_materials: &[SnsCiphertextMaterial]) {
        let this = self.clone();
        let materials = sns_materials.to_vec();
        spawn_with_limit(async move {
            this.verify_materials(materials).await;
        })
        .await;
    }

    /// Shadow-mode core: for every handle in the request's on-chain materials, fan
    /// HEAD requests out to every bucket, compute an off-chain verdict, and log how
    /// it compares to the on-chain tuple. Never affects decryption — `info!` on full
    /// corroboration, `warn!` on any failure.
    ///
    /// The defensive disabled-guard backs up [`Self::spawn_verification`]'s gate.
    pub async fn verify_materials(&self, materials: Vec<SnsCiphertextMaterial>) {
        if !self.config.enabled {
            return;
        }
        let registry = self.registry();

        for material in &materials {
            let attestations = fetch_attestations(
                material.ctHandle,
                &registry,
                &self.client,
                self.config.http_timeout,
            )
            .await;
            let attestations_seen = attestations.len();
            let threshold = registry.threshold;
            let handle = hex::encode(material.ctHandle);

            match self.verify_handle(material, &attestations, &registry).await {
                Ok(consensus) => info!(
                    handle,
                    attestations_seen,
                    valid_signers = consensus.valid_signers,
                    threshold,
                    "Off-chain attestations corroborated the on-chain ciphertext tuple"
                ),
                Err(e) => warn!(
                    handle,
                    attestations_seen,
                    threshold,
                    "Off-chain ciphertext-attestation verification failed: {e}"
                ),
            }
        }
    }

    /// Evaluates a single handle: consensus, then (on a winning group) the
    /// download-and-digest check and the on-chain comparison. Returns the winning
    /// [`Consensus`] on full corroboration, or the first failure encountered.
    async fn verify_handle(
        &self,
        material: &SnsCiphertextMaterial,
        attestations: &[(Address, CiphertextAttestation)],
        registry: &CoprocessorRegistry,
    ) -> Result<Consensus, AttestationError> {
        let handle = material.ctHandle;
        let consensus =
            consensus::evaluate(handle, attestations, &registry.signers, registry.threshold)?;

        let buckets = registry.tx_sender_to_bucket.values().map(String::as_str);
        let ciphertext = fetch_and_check_ciphertext(
            &self.client,
            buckets,
            handle,
            consensus.material.sns_ciphertext_digest,
            self.config.http_timeout,
        )
        .await?;
        compare_onchain(material, &consensus.material)?;

        Ok(consensus)
    }

    /// Spawns the background task that reloads the registry on the configured TTL.
    /// A failed reload keeps the previous snapshot.
    fn spawn_refresh_task(&self) {
        let this = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(this.config.registry_refresh);
            // First tick fires immediately; the snapshot is already fresh from
            // `connect`, so consume it before the reload loop.
            interval.tick().await;

            loop {
                interval.tick().await;
                match CoprocessorRegistry::load(&this.gateway_config_contract).await {
                    Ok(new_registry) => {
                        *this.registry.write().expect("registry lock poisoned") =
                            Arc::new(new_registry);
                    }
                    Err(e) => warn!(
                        "Failed to refresh Coprocessor registry, keeping previous snapshot: {e}"
                    ),
                }
            }
        });
    }
}

/// Compares the consensus material against the on-chain `SnsCiphertextMaterial`.
/// Only `keyId` and `snsCiphertextDigest` exist on-chain; `ciphertext_digest` and
/// `format` are off-chain-only (bound by the signature). A divergence here is the
/// key signal shadow mode exists to surface.
fn compare_onchain(
    onchain: &SnsCiphertextMaterial,
    material: &ConsensusMaterial,
) -> Result<(), AttestationError> {
    if onchain.keyId != material.key_id {
        return Err(AttestationError::OnchainTupleMismatch {
            field: "key_id",
            onchain: onchain.keyId.to_string(),
            attested: material.key_id.to_string(),
        });
    }
    if onchain.snsCiphertextDigest != material.sns_ciphertext_digest {
        return Err(AttestationError::OnchainTupleMismatch {
            field: "sns_ciphertext_digest",
            onchain: onchain.snsCiphertextDigest.to_string(),
            attested: material.sns_ciphertext_digest.to_string(),
        });
    }
    Ok(())
}
