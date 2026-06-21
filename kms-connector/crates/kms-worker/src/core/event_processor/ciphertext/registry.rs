//! TTL'd snapshot of the on-chain Coprocessor registry.
//!
//! The [`CiphertextManager`] needs three things from `GatewayConfig`: the set of authorized
//! attestation signer addresses, the per-Coprocessor S3 bucket URLs (to fan attestation HEAD
//! requests at, and to retrieve ciphertexts from), and the majority threshold. Querying these on
//! every decryption request would trigger N+1 RPC calls, so the [`CoprocessorRegistry`] holds a
//! whole snapshot behind a short TTL and tolerates registration changes within one refresh window.

use crate::core::config::Config;
use alloy::{primitives::Address, providers::Provider};
use fhevm_gateway_bindings::gateway_config::GatewayConfig::{self, GatewayConfigInstance};
use futures::future::try_join_all;
use std::{
    collections::{HashMap, HashSet},
    num::NonZeroUsize,
    sync::{Arc, RwLock},
    time::Duration,
};
use tokio_util::sync::CancellationToken;
use tracing::{error, warn};

/// The Coprocessor registry as seen from the connector: a periodically-synced mirror of the
/// on-chain registry.
#[derive(Clone)]
pub struct CoprocessorRegistry<P: Provider> {
    /// Used to (re)load the registry snapshot.
    gateway_config_contract: GatewayConfigInstance<P>,

    /// The current registry snapshot.
    // Cheap to clone: the outer `Arc` shares the lock across clones; the `RwLock` gives interior
    // mutability for the swap; the inner `Arc` makes reads snapshot-and-release.
    snapshot: Arc<RwLock<Arc<CoprocessorRegistrySnapshot>>>,
}

/// An immutable snapshot of the Coprocessor registry at one point in time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoprocessorRegistrySnapshot {
    /// Addresses an attestation signature must recover to. (`getCoprocessorSigners`)
    pub signers: HashSet<Address>,
    /// `txSender -> s3BucketUrl` for every registered Coprocessor.
    /// (`getCoprocessorTxSenders` + `getCoprocessor(addr).s3BucketUrl`)
    pub tx_sender_to_bucket: HashMap<Address, String>,
    /// Number of agreeing signers required for consensus. (`getCoprocessorMajorityThreshold`)
    pub threshold: NonZeroUsize,
}

/// Why loading or refreshing the Coprocessor registry failed.
#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    /// A condition that can never arise from a healthy protocol: an invalid on-chain threshold
    /// or a poisoned snapshot lock. The worker must stop in such case.
    #[error("critical Coprocessor registry error: {0}")]
    Critical(String),

    /// A recoverable failure, e.g. a transient RPC error. The previous snapshot is kept.
    #[error(transparent)]
    Transient(#[from] anyhow::Error),
}

impl CoprocessorRegistrySnapshot {
    pub fn new(
        signers: HashSet<Address>,
        tx_sender_to_bucket: HashMap<Address, String>,
        threshold: NonZeroUsize,
    ) -> Self {
        Self {
            signers,
            tx_sender_to_bucket,
            threshold,
        }
    }

    /// Loads a fresh snapshot from the `GatewayConfig` contract.
    pub async fn load<P: Provider>(
        contract: &GatewayConfigInstance<P>,
    ) -> Result<Self, RegistryError> {
        let get_copro_signers = contract.getCoprocessorSigners();
        let get_copro_tx_senders = contract.getCoprocessorTxSenders();
        let get_copro_threshold = contract.getCoprocessorMajorityThreshold();

        let (signers, tx_senders, threshold_u256) = tokio::try_join!(
            biased;
            get_copro_signers.call(),
            get_copro_tx_senders.call(),
            get_copro_threshold.call()
        )
        .map_err(|e| RegistryError::Transient(e.into()))?;

        // A zero or oversized threshold can never come from a healthy `GatewayConfig`: treat it
        // as critical so the worker refuses to run.
        let threshold = threshold_u256
            .try_into()
            .ok()
            .and_then(NonZeroUsize::new)
            .ok_or_else(|| {
                RegistryError::Critical(format!(
                    "invalid on-chain Coprocessor majority threshold: {threshold_u256}"
                ))
            })?;

        let signers: HashSet<Address> = signers.into_iter().collect();
        if signers.is_empty() {
            return Err(RegistryError::Transient(anyhow::anyhow!(
                "Not a single Coprocessor signer in the registry"
            )));
        }

        let tx_sender_to_bucket: HashMap<Address, String> = try_join_all(
            tx_senders
                .into_iter()
                .map(|tx_sender| async move { get_copro_bucket(contract, tx_sender).await }),
        )
        .await
        .map_err(RegistryError::Transient)?
        .into_iter()
        .flatten()
        .collect();

        if tx_sender_to_bucket.is_empty() {
            return Err(RegistryError::Transient(anyhow::anyhow!(
                "Not a single Coprocessor with a non-empty S3 bucket URL in the registry"
            )));
        }

        Ok(Self::new(signers, tx_sender_to_bucket, threshold))
    }
}

/// Resolves the S3 bucket URL of a single Coprocessor.
///
/// An empty `s3BucketUrl` is skipped with a warning: it is persistent on-chain state, so
/// failing on it would crash-loop the worker. Transient RPC failures still propagate.
async fn get_copro_bucket<P: Provider>(
    contract: &GatewayConfigInstance<P>,
    copro_tx_sender_addr: Address,
) -> anyhow::Result<Option<(Address, String)>> {
    let copro = contract.getCoprocessor(copro_tx_sender_addr).call().await?;
    if copro.s3BucketUrl.is_empty() {
        warn!("No S3 bucket URL registered for Coprocessor {copro_tx_sender_addr}, skipping it");
        return Ok(None);
    }
    Ok(Some((copro_tx_sender_addr, copro.s3BucketUrl)))
}

impl<P> CoprocessorRegistry<P>
where
    P: Provider + Clone + 'static,
{
    /// Loads the initial snapshot and spawns the background refresh task.
    ///
    /// `cancel_token` is the worker-wide shutdown token: the refresh task cancels it on a
    /// critical failure (see [`Self::spawn_refresh_task`]).
    pub async fn connect(
        provider: P,
        config: &Config,
        cancel_token: CancellationToken,
    ) -> anyhow::Result<Self> {
        let gateway_config_contract =
            GatewayConfig::new(config.gateway_config_contract.address, provider);

        let snapshot = CoprocessorRegistrySnapshot::load(&gateway_config_contract).await?;
        let registry = Self {
            gateway_config_contract,
            snapshot: Arc::new(RwLock::new(Arc::new(snapshot))),
        };
        registry.spawn_refresh_task(config.ct_attestation.registry_refresh, cancel_token);

        Ok(registry)
    }

    /// Clones the inner `Arc` of the current snapshot and drops the guard, so no lock is held.
    pub fn snapshot(&self) -> Arc<CoprocessorRegistrySnapshot> {
        self.snapshot
            .read()
            .expect("copro registry lock poisoned")
            .clone()
    }

    /// Spawns the background task that reloads the registry on the configured TTL.
    ///
    /// A transient reload failure keeps the previous snapshot. A critical failure (invalid
    /// on-chain threshold or a poisoned snapshot lock) cancels `cancel_token` to bring the whole
    /// worker down.
    fn spawn_refresh_task(&self, refresh_interval: Duration, cancel_token: CancellationToken) {
        let this = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(refresh_interval);
            // First tick fires immediately; the snapshot is already fresh from
            // `connect`, so consume it before the reload loop.
            interval.tick().await;

            loop {
                tokio::select! {
                    _ = cancel_token.cancelled() => break,
                    _ = interval.tick() => {}
                }

                match CoprocessorRegistrySnapshot::load(&this.gateway_config_contract)
                    .await
                    .and_then(|s| this.store_snapshot(s))
                {
                    Ok(()) => (),
                    Err(RegistryError::Transient(e)) => warn!(
                        "Failed to refresh Coprocessor registry, keeping previous snapshot: {e}"
                    ),
                    Err(RegistryError::Critical(critical)) => {
                        error!("Shutting down worker on critical registry failure: {critical}");
                        cancel_token.cancel();
                        break;
                    }
                };
            }
        });
    }

    /// Swaps in a fresh snapshot.
    fn store_snapshot(&self, snapshot: CoprocessorRegistrySnapshot) -> Result<(), RegistryError> {
        let mut guard = self.snapshot.write().map_err(|_| {
            RegistryError::Critical("Coprocessor registry lock poisoned".to_string())
        })?;
        *guard = Arc::new(snapshot);
        Ok(())
    }
}

#[cfg(test)]
impl<P> CoprocessorRegistry<P>
where
    P: Provider + Clone + 'static,
{
    pub fn empty(provider: P) -> Self {
        Self {
            gateway_config_contract: GatewayConfig::new(Address::ZERO, provider),
            snapshot: Arc::new(RwLock::new(
                Arc::new(CoprocessorRegistrySnapshot::default()),
            )),
        }
    }
}

#[cfg(test)]
impl Default for CoprocessorRegistrySnapshot {
    fn default() -> Self {
        Self {
            signers: HashSet::default(),
            tx_sender_to_bucket: HashMap::default(),
            threshold: NonZeroUsize::MIN,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        primitives::U256,
        providers::{ProviderBuilder, mock::Asserter},
        sol_types::SolValue,
    };
    use connector_utils::tests::rand::rand_address;
    use fhevm_gateway_bindings::gateway_config::GatewayConfig::Coprocessor;

    fn mocked_contract(asserter: &Asserter) -> GatewayConfigInstance<impl Provider + Clone> {
        let provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter.clone());
        GatewayConfig::new(Address::ZERO, provider)
    }

    /// Pushes the snapshot responses: signers, tx-senders, threshold, then one
    /// `getCoprocessor` response per bucket URL.
    fn mock_registry_load(asserter: &Asserter, tx_senders: &[Address], buckets: &[&str]) {
        asserter.push_success(&vec![rand_address()].abi_encode());
        asserter.push_success(&tx_senders.to_vec().abi_encode());
        asserter.push_success(&U256::ONE.abi_encode());
        for bucket in buckets {
            let coprocessor = Coprocessor {
                s3BucketUrl: bucket.to_string(),
                ..Default::default()
            };
            asserter.push_success(&coprocessor.abi_encode());
        }
    }

    #[tokio::test]
    async fn load_skips_coprocessor_with_empty_bucket_url() {
        let asserter = Asserter::new();
        let (bad_copro, good_copro) = (rand_address(), rand_address());
        mock_registry_load(&asserter, &[bad_copro, good_copro], &["", "http://bucket"]);

        let snapshot = CoprocessorRegistrySnapshot::load(&mocked_contract(&asserter))
            .await
            .unwrap();
        assert_eq!(
            snapshot.tx_sender_to_bucket,
            HashMap::from([(good_copro, "http://bucket".to_string())])
        );
    }

    #[tokio::test]
    async fn load_fails_when_all_bucket_urls_are_empty() {
        let asserter = Asserter::new();
        mock_registry_load(&asserter, &[rand_address()], &[""]);

        CoprocessorRegistrySnapshot::load(&mocked_contract(&asserter))
            .await
            .unwrap_err();
    }

    #[tokio::test]
    async fn load_rejects_zero_threshold_as_critical() {
        let asserter = Asserter::new();
        asserter.push_success(&vec![rand_address()].abi_encode());
        asserter.push_success(&vec![rand_address()].abi_encode());
        asserter.push_success(&U256::ZERO.abi_encode());

        let err = CoprocessorRegistrySnapshot::load(&mocked_contract(&asserter))
            .await
            .unwrap_err();
        assert!(matches!(err, RegistryError::Critical(_)));
    }
}
