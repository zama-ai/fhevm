//! TTL'd snapshot of the on-chain Coprocessor registry.
//!
//! To check consensus, we need the current Coprocessor set, read from the `CoprocessorConfig`
//! contract on Ethereum. For each Coprocessor, we need its signer and its storage URL, where we
//! send the attestation HEAD requests. We also need the majority threshold.
//!
//! Reading the contract on every request would take several RPC calls. Instead, the
//! [`CoprocessorRegistry`] caches a snapshot for a short TTL. A registration change is picked up
//! at the next refresh.

pub use crate::consensus::CoprocessorEntry;

use alloy::{
    network::{Ethereum, Network},
    primitives::Address,
    providers::Provider,
};
use fhevm_host_bindings::i_coprocessor_config_base::ICoprocessorConfigBase::{
    self, Coprocessor, ICoprocessorConfigBaseInstance,
};
use std::{
    num::NonZeroUsize,
    sync::{Arc, RwLock},
    time::Duration,
};
use tokio_util::sync::CancellationToken;
use tracing::{error, warn};
use url::Url;

/// A periodically-synced mirror of the on-chain Coprocessor registry.
///
/// `N` defaults to [`Ethereum`] but any [`Network`] the caller's provider is built for works.
#[derive(Clone)]
pub struct CoprocessorRegistry<P: Provider<N>, N: Network = Ethereum> {
    coprocessor_config_contract: ICoprocessorConfigBaseInstance<P, N>,
    snapshot: Arc<RwLock<Arc<CoprocessorRegistrySnapshot>>>,
    critical_failure: Arc<RwLock<Option<String>>>,
}

/// An immutable snapshot of the Coprocessor registry at one point in time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoprocessorRegistrySnapshot {
    /// Every Coprocessor of the current set. (`getCoprocessorsForContext`)
    pub coprocessors: Vec<CoprocessorEntry>,
    /// Number of agreeing signers required for consensus.
    /// (`getCoprocessorMajorityThresholdForContext`)
    pub threshold: NonZeroUsize,
}

/// What a [`RegistryError::Critical`] means for the embedding service.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CriticalFailurePolicy {
    /// Keep serving the last good snapshot.
    ServeStale,
    /// Cancel the registry's `cancel_token`; the embedding service is expected to shut down.
    Shutdown,
}

/// Why loading or refreshing the Coprocessor registry failed.
///
/// Reporting only — the registry acts on neither variant. What a failure means is the embedding
/// service's call: whether to refuse requests, serve on from the previous snapshot, or stop.
#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    /// The registry was read and what came back cannot be used: a majority threshold that is not a
    /// `NonZeroUsize`, too few Coprocessors to reach it, or a poisoned snapshot lock. No retry
    /// helps.
    #[error("critical Coprocessor registry error: {0}")]
    Critical(String),

    /// The registry could not be read, e.g. the RPC node was unreachable. A retry may fix it,
    /// and the previous snapshot stays in place until one does.
    #[error(transparent)]
    Transient(#[from] anyhow::Error),
}

impl CoprocessorRegistrySnapshot {
    pub fn new(coprocessors: Vec<CoprocessorEntry>, threshold: NonZeroUsize) -> Self {
        Self {
            coprocessors,
            threshold,
        }
    }

    /// Loads a fresh snapshot of the current Coprocessor set from the `CoprocessorConfig` contract.
    pub async fn load<P: Provider<N>, N: Network>(
        contract: &ICoprocessorConfigBaseInstance<P, N>,
    ) -> Result<Self, RegistryError> {
        let context_id = contract
            .getCurrentCoprocessorContextId()
            .call()
            .await
            .map_err(|e| RegistryError::Transient(anyhow::anyhow!("{}", redact_rpc_url(e))))?;

        let get_coprocessors = contract.getCoprocessorsForContext(context_id);
        let get_threshold = contract.getCoprocessorMajorityThresholdForContext(context_id);

        let (coprocessors, threshold_u256) = tokio::try_join!(
            biased;
            get_coprocessors.call(),
            get_threshold.call()
        )
        .map_err(|e| RegistryError::Transient(anyhow::anyhow!("{}", redact_rpc_url(e))))?;

        let threshold = threshold_u256
            .try_into()
            .ok()
            .and_then(NonZeroUsize::new)
            .ok_or_else(|| {
                RegistryError::Critical(format!(
                    "invalid on-chain Coprocessor majority threshold: {threshold_u256}"
                ))
            })?;

        let coprocessors: Vec<_> = coprocessors
            .into_iter()
            .filter_map(parse_coprocessor_entry)
            .collect();
        if coprocessors.len() < threshold.get() {
            return Err(RegistryError::Critical(format!(
                "{} of the {threshold} Coprocessors required for a majority have a valid storage \
                 URL",
                coprocessors.len()
            )));
        }
        Ok(Self::new(coprocessors, threshold))
    }
}

/// Stringifies an RPC error, stripping the ` for url (…)` suffix that `reqwest::Error::Display`
/// appends: RPC endpoints can carry embedded API keys.
fn redact_rpc_url(err: impl std::fmt::Display) -> String {
    let msg = err.to_string();
    match msg.find(" for url (") {
        Some(at) => msg[..at].to_owned(),
        None => msg,
    }
}

/// Parses an on-chain Coprocessor to a validated entry.
///
/// An invalid URL drops the Coprocessor with a warning.
fn parse_coprocessor_entry(copro: Coprocessor) -> Option<CoprocessorEntry> {
    let tx_sender = copro.txSenderAddress;
    let storage_url = Url::parse(&copro.storageUrl)
        .inspect_err(|e| {
            warn!("Invalid storage URL registered for Copro {tx_sender}, ignoring it: {e}")
        })
        .ok()?;
    let prf_endpoint_url = match copro.prfEndpointUrl.as_str() {
        "" => None,
        url => Url::parse(url)
            .inspect_err(|e| {
                warn!("Invalid PRF endpoint URL registered for Copro {tx_sender}, ignoring it: {e}")
            })
            .ok(),
    };
    Some(CoprocessorEntry {
        tx_sender,
        signer: copro.signerAddress,
        storage_url,
        prf_endpoint_url,
    })
}

impl<P, N> CoprocessorRegistry<P, N>
where
    P: Provider<N> + Clone + 'static,
    N: Network,
{
    /// Loads the initial snapshot and spawns the background refresh task.
    pub async fn connect(
        provider: P,
        coprocessor_config_address: Address,
        refresh_interval: Duration,
        cancel_token: CancellationToken,
        policy: CriticalFailurePolicy,
    ) -> anyhow::Result<Self> {
        // Check for code first — `load` would see an address with no code as a decode failure
        // and classify it `Transient`.
        let deployed_code = provider
            .get_code_at(coprocessor_config_address)
            .await
            .map_err(|e| anyhow::anyhow!("{}", redact_rpc_url(e)))?;

        let coprocessor_config_contract =
            ICoprocessorConfigBase::new(coprocessor_config_address, provider);
        let loaded = if deployed_code.is_empty() {
            Err(RegistryError::Critical(format!(
                "no contract deployed at the configured CoprocessorConfig address \
                 {coprocessor_config_address}"
            )))
        } else {
            CoprocessorRegistrySnapshot::load(&coprocessor_config_contract).await
        };

        let (snapshot, critical_failure) = match loaded {
            Ok(snapshot) => (snapshot, None),
            Err(RegistryError::Critical(reason)) => match policy {
                CriticalFailurePolicy::ServeStale => {
                    error!("Coprocessor registry unusable at startup: {reason}");
                    (
                        CoprocessorRegistrySnapshot::new(Vec::new(), NonZeroUsize::MIN),
                        Some(reason),
                    )
                }
                CriticalFailurePolicy::Shutdown => {
                    return Err(anyhow::anyhow!(
                        "Coprocessor registry unusable at startup: {reason}"
                    ));
                }
            },
            Err(transient) => return Err(transient.into()),
        };

        let registry = Self {
            coprocessor_config_contract,
            snapshot: Arc::new(RwLock::new(Arc::new(snapshot))),
            critical_failure: Arc::new(RwLock::new(critical_failure)),
        };
        registry.spawn_refresh_task(refresh_interval, cancel_token, policy);

        Ok(registry)
    }

    /// The registry's `Critical` reason, or `None` while the served snapshot is good.
    pub fn critical_failure(&self) -> Option<String> {
        self.critical_failure
            .read()
            .expect("copro registry critical-failure lock poisoned")
            .clone()
    }

    fn set_critical_failure(&self, reason: Option<String>) {
        *self
            .critical_failure
            .write()
            .expect("copro registry critical-failure lock poisoned") = reason;
    }

    /// Clones the inner `Arc` of the current snapshot and drops the guard, so no lock is held.
    pub fn snapshot(&self) -> Arc<CoprocessorRegistrySnapshot> {
        self.snapshot
            .read()
            .expect("copro registry lock poisoned")
            .clone()
    }

    /// Spawns the background task that reloads the registry on the configured TTL.
    fn spawn_refresh_task(
        &self,
        refresh_interval: Duration,
        cancel_token: CancellationToken,
        policy: CriticalFailurePolicy,
    ) {
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

                match CoprocessorRegistrySnapshot::load(&this.coprocessor_config_contract)
                    .await
                    .and_then(|s| this.store_snapshot(s))
                {
                    Ok(()) => this.set_critical_failure(None),
                    Err(RegistryError::Transient(e)) => warn!(
                        "Failed to refresh Coprocessor registry, keeping previous snapshot: {e}"
                    ),
                    Err(RegistryError::Critical(critical)) => match policy {
                        CriticalFailurePolicy::ServeStale => {
                            // TODO: alert on this, once a metric exists to carry it.
                            error!(
                                "Coprocessor registry unusable, keeping previous snapshot: \
                                 {critical}"
                            );
                            this.set_critical_failure(Some(critical));
                        }
                        CriticalFailurePolicy::Shutdown => {
                            error!("Shutting down on critical registry failure: {critical}");
                            cancel_token.cancel();
                            break;
                        }
                    },
                };
            }
        });
    }

    fn store_snapshot(&self, snapshot: CoprocessorRegistrySnapshot) -> Result<(), RegistryError> {
        let mut guard = self.snapshot.write().map_err(|_| {
            RegistryError::Critical("Coprocessor registry lock poisoned".to_string())
        })?;
        *guard = Arc::new(snapshot);
        Ok(())
    }
}

#[cfg(feature = "test-utils")]
impl<P, N> CoprocessorRegistry<P, N>
where
    P: Provider<N> + Clone + 'static,
    N: Network,
{
    /// A registry with an empty snapshot, never reading the chain. For consumers exercising their
    /// own code against this crate's client without hitting the chain.
    pub fn for_test(provider: P) -> Self {
        Self {
            coprocessor_config_contract: ICoprocessorConfigBase::new(Address::ZERO, provider),
            snapshot: Arc::new(RwLock::new(Arc::new(CoprocessorRegistrySnapshot::new(
                Vec::new(),
                NonZeroUsize::MIN,
            )))),
            critical_failure: Arc::new(RwLock::new(None)),
        }
    }

    /// Replaces the served snapshot without touching the chain.
    pub fn with_snapshot(self, snapshot: CoprocessorRegistrySnapshot) -> Self {
        Self {
            snapshot: Arc::new(RwLock::new(Arc::new(snapshot))),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        primitives::{Bytes, U256},
        providers::{ProviderBuilder, mock::Asserter},
        sol_types::SolValue,
    };

    /// Distinct, deterministic addresses for test fixtures (no external `rand` dependency).
    fn addr(byte: u8) -> Address {
        Address::repeat_byte(byte)
    }

    #[test]
    fn redact_rpc_url_strips_the_endpoint_and_keeps_the_reason() {
        assert_eq!(
            redact_rpc_url("error sending request for url (https://rpc.example/v1/SECRET_KEY)"),
            "error sending request"
        );
    }

    #[test]
    fn redact_rpc_url_passes_through_messages_without_a_url() {
        assert_eq!(
            redact_rpc_url("server returned an error"),
            "server returned an error"
        );
    }

    fn mocked_contract(
        asserter: &Asserter,
    ) -> ICoprocessorConfigBaseInstance<impl Provider + Clone> {
        let provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter.clone());
        ICoprocessorConfigBase::new(Address::ZERO, provider)
    }

    /// Pushes the snapshot responses: current context ID, `coprocessors`, then `threshold`.
    fn mock_registry_load(asserter: &Asserter, coprocessors: &[Coprocessor], threshold: U256) {
        asserter.push_success(&U256::ONE.abi_encode());
        asserter.push_success(&coprocessors.to_vec().abi_encode());
        asserter.push_success(&threshold.abi_encode());
    }

    /// The `index`-th Coprocessor, with a distinct tx sender and signer.
    fn coprocessor(index: u8, prf_endpoint_url: &str) -> Coprocessor {
        Coprocessor {
            txSenderAddress: addr(index),
            signerAddress: Address::repeat_byte(0x50 + index),
            storageUrl: format!("http://bucket-{index}"),
            prfEndpointUrl: prf_endpoint_url.to_string(),
        }
    }

    #[tokio::test]
    async fn load_parses_the_urls() {
        let asserter = Asserter::new();
        let invalid_storage_url = Coprocessor {
            storageUrl: "not a url".to_string(),
            ..coprocessor(4, "http://prf")
        };
        let coprocessors = [
            coprocessor(1, "http://prf"),
            coprocessor(2, ""),
            coprocessor(3, "not a url"),
            invalid_storage_url,
        ];
        mock_registry_load(&asserter, &coprocessors, U256::from(3));

        let snapshot = CoprocessorRegistrySnapshot::load(&mocked_contract(&asserter))
            .await
            .unwrap();
        let urls: Vec<_> = snapshot
            .coprocessors
            .into_iter()
            .map(|entry| (entry.tx_sender, entry.prf_endpoint_url))
            .collect();
        assert_eq!(
            urls,
            vec![
                (addr(1), Some(Url::parse("http://prf").unwrap())),
                (addr(2), None),
                (addr(3), None),
            ]
        );
    }

    /// A clean read that is short of the threshold is `Critical`: a retry reads the same list.
    #[tokio::test]
    async fn load_rejects_a_list_short_of_the_threshold_as_critical() {
        let asserter = Asserter::new();
        let invalid_storage_url = Coprocessor {
            storageUrl: "not a url".to_string(),
            ..coprocessor(2, "")
        };
        mock_registry_load(
            &asserter,
            &[coprocessor(1, ""), invalid_storage_url],
            U256::from(2),
        );

        let err = CoprocessorRegistrySnapshot::load(&mocked_contract(&asserter))
            .await
            .unwrap_err();
        std::assert_matches!(err, RegistryError::Critical(_));
    }

    /// Pushes the `eth_getCode` response `connect` probes with before its first load.
    fn mock_deployed_code(asserter: &Asserter) {
        asserter.push_success(&Bytes::from_static(&[0x60]));
    }

    /// Pushes a successful `load` of one Coprocessor.
    fn mock_good_load(asserter: &Asserter) {
        mock_registry_load(asserter, &[coprocessor(7, "")], U256::ONE);
    }

    /// Pushes a `load` that fails with [`RegistryError::Critical`] (zero on-chain threshold).
    fn mock_critical_load(asserter: &Asserter) {
        mock_registry_load(asserter, &[coprocessor(7, "")], U256::ZERO);
    }

    /// Polls until `cond` holds — the refresh task runs on its own timer.
    async fn eventually(what: &str, mut cond: impl FnMut() -> bool) {
        for _ in 0..300 {
            if cond() {
                return;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        panic!("{what} did not happen within three seconds");
    }

    #[tokio::test]
    async fn refresh_flag_follows_the_latest_outcome() {
        let asserter = Asserter::new();
        mock_deployed_code(&asserter);
        mock_good_load(&asserter);
        // Twice, so the flag stays observably set for longer than one refresh interval.
        mock_critical_load(&asserter);
        mock_critical_load(&asserter);
        mock_good_load(&asserter);

        let provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter.clone());
        let registry = CoprocessorRegistry::connect(
            provider,
            Address::ZERO,
            Duration::from_millis(100),
            CancellationToken::new(),
            CriticalFailurePolicy::ServeStale,
        )
        .await
        .unwrap();

        assert!(registry.critical_failure().is_none());
        eventually("a critical refresh", || {
            registry.critical_failure().is_some()
        })
        .await;
        eventually("a recovered refresh", || {
            registry.critical_failure().is_none()
        })
        .await;
    }

    /// An address with no code is chain-side config: `connect` succeeds with the reason set.
    #[tokio::test]
    async fn connect_starts_unusable_when_no_contract_is_deployed() {
        let asserter = Asserter::new();
        asserter.push_success(&Bytes::new());

        let registry = CoprocessorRegistry::connect(
            ProviderBuilder::new()
                .disable_recommended_fillers()
                .connect_mocked_client(asserter.clone()),
            Address::ZERO,
            Duration::from_secs(60),
            CancellationToken::new(),
            CriticalFailurePolicy::ServeStale,
        )
        .await
        .expect("connect refused to start over chain-side config");

        let reason = registry.critical_failure().expect("no critical reason set");
        assert!(reason.contains("no contract deployed"), "{reason}");
    }

    /// `Shutdown` refuses to boot on a critical startup error instead of serving a placeholder.
    #[tokio::test]
    async fn shutdown_policy_refuses_to_start_on_critical_startup_error() {
        let asserter = Asserter::new();
        asserter.push_success(&Bytes::new());

        let result = CoprocessorRegistry::connect(
            ProviderBuilder::new()
                .disable_recommended_fillers()
                .connect_mocked_client(asserter.clone()),
            Address::ZERO,
            Duration::from_secs(60),
            CancellationToken::new(),
            CriticalFailurePolicy::Shutdown,
        )
        .await;
        let err = match result {
            Ok(_) => panic!("connect should have refused to start"),
            Err(e) => e,
        };
        assert!(err.to_string().contains("no contract deployed"), "{err}");
    }

    /// `Shutdown` cancels `cancel_token` on a critical refresh, instead of serving the previous
    /// snapshot.
    #[tokio::test]
    async fn shutdown_policy_cancels_the_token_on_critical_refresh() {
        let asserter = Asserter::new();
        mock_deployed_code(&asserter);
        mock_good_load(&asserter);
        mock_critical_load(&asserter);

        let provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter.clone());
        let cancel_token = CancellationToken::new();
        let registry = CoprocessorRegistry::connect(
            provider,
            Address::ZERO,
            Duration::from_millis(100),
            cancel_token.clone(),
            CriticalFailurePolicy::Shutdown,
        )
        .await
        .unwrap();

        assert!(!cancel_token.is_cancelled());
        eventually("the token to be cancelled", || cancel_token.is_cancelled()).await;
        // Shutdown does not report through `critical_failure`: the embedding service is expected
        // to stop, not keep serving.
        assert!(registry.critical_failure().is_none());
    }

    #[tokio::test]
    async fn load_rejects_zero_threshold_as_critical() {
        let asserter = Asserter::new();
        mock_critical_load(&asserter);

        let err = CoprocessorRegistrySnapshot::load(&mocked_contract(&asserter))
            .await
            .unwrap_err();
        assert!(matches!(err, RegistryError::Critical(_)));
    }
}
