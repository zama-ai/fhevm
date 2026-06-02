//! Off-chain ciphertext-attestation verifier.
//!
//! On every decryption request this verifier fans HEAD requests out to every registered
//! Coprocessor bucket, computes an off-chain consensus verdict, and logs a structured comparison
//! against the authoritative on-chain `SnsCiphertextMaterial` snapshot. It never changes
//! decryption behavior: the on-chain result stays authoritative regardless of the verdict.

use super::registry::CoprocessorRegistry;
use alloy::{primitives::Address, providers::Provider, transports::http::Client};
use fhevm_gateway_bindings::gateway_config::GatewayConfig::{self, GatewayConfigInstance};
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};
use tracing::warn;

/// Verifies off-chain ciphertext attestations against the on-chain snapshot.
///
/// Cheap to clone: every field is either a handle to a shared resource or a small `Copy` value.
/// The registry lives behind `Arc<RwLock<Arc<_>>>` so the single background refresh task can swap
/// the snapshot through `&self` while readers take a lock-free snapshot-and-release clone.
#[derive(Clone)]
pub struct AttestationVerifier<P: Provider> {
    /// The `GatewayConfig` contract, used to (re)load the registry snapshot.
    gateway_config_contract: GatewayConfigInstance<P>,

    /// HTTP client reused for the S3 HEAD/GET fan-out.
    client: Client,

    /// TTL'd snapshot of the Coprocessor registry. The outer `Arc` shares the
    /// lock across clones; the `RwLock` gives interior mutability for the swap;
    /// the inner `Arc` makes reads snapshot-and-release.
    registry: Arc<RwLock<Arc<CoprocessorRegistry>>>,

    /// Per-bucket HEAD/GET timeout.
    head_timeout: Duration,

    /// How often the background task reloads the registry snapshot.
    registry_refresh: Duration,
}

impl<P> AttestationVerifier<P>
where
    P: Provider + Clone + 'static,
{
    pub fn new(
        gateway_config_contract: GatewayConfigInstance<P>,
        client: Client,
        registry: CoprocessorRegistry,
        head_timeout: Duration,
        registry_refresh: Duration,
    ) -> Self {
        Self {
            gateway_config_contract,
            client,
            registry: Arc::new(RwLock::new(Arc::new(registry))),
            head_timeout,
            registry_refresh,
        }
    }

    /// Builds a verifier from the `GatewayConfig` contract: loads the initial
    /// registry snapshot, then starts the background refresh task.
    pub async fn connect(
        gateway_config_address: Address,
        provider: P,
        client: Client,
        head_timeout: Duration,
        registry_refresh: Duration,
    ) -> anyhow::Result<Self> {
        let gateway_config_contract = GatewayConfig::new(gateway_config_address, provider);
        let registry = CoprocessorRegistry::load(&gateway_config_contract).await?;
        let verifier = Self::new(
            gateway_config_contract,
            client,
            registry,
            head_timeout,
            registry_refresh,
        );
        verifier.spawn_refresh_task();
        Ok(verifier)
    }

    /// Returns a snapshot-and-release clone of the current registry: clones the
    /// inner `Arc` (one atomic increment) and drops the guard, so callers hold
    /// no lock across the subsequent `.await`-heavy fan-out.
    pub fn registry(&self) -> Arc<CoprocessorRegistry> {
        self.registry
            .read()
            .expect("attestation registry lock poisoned")
            .clone()
    }

    /// HTTP client handle. Consumed by the fetcher in a later step.
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Spawns the single background task that reloads the registry on the configured TTL.
    ///
    /// A failed reload keeps the previous snapshot.
    fn spawn_refresh_task(&self) {
        let this = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(this.registry_refresh);
            // The first tick fires immediately; the snapshot is already fresh
            // from `connect`, so consume it before entering the reload loop.
            interval.tick().await;

            loop {
                interval.tick().await;
                match CoprocessorRegistry::load(&this.gateway_config_contract).await {
                    Ok(new_registry) => {
                        *this.registry.write().expect("registry lock poisoned") =
                            Arc::new(new_registry);
                    }
                    Err(e) => warn!(
                        target: "ct_attestation",
                        "Failed to refresh Coprocessor registry, keeping previous snapshot: {e}"
                    ),
                }
            }
        });
    }
}
