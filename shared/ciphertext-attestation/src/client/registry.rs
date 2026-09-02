//! TTL'd snapshot of the on-chain Coprocessor registry.
//!
//! A consensus consumer needs the on-chain signer↔bucket binding for every registered
//! Coprocessor (which signer a bucket is allowed to speak for, and the S3 bucket URL to fan
//! attestation HEAD requests at) plus the majority threshold. Querying these on every request
//! would trigger N+1 RPC calls, so the [`CoprocessorRegistry`] holds a whole snapshot behind a
//! short TTL and tolerates registration changes within one refresh window.

pub use crate::tracker::CoprocessorEntry;

use alloy::{
    network::{Ethereum, Network},
    primitives::Address,
    providers::Provider,
};
use fhevm_gateway_bindings::gateway_config::GatewayConfig::{self, GatewayConfigInstance};
use futures::future::try_join_all;
use std::{
    num::NonZeroUsize,
    sync::{Arc, RwLock},
    time::Duration,
};
use tokio_util::sync::CancellationToken;
use tracing::{error, warn};

/// A periodically-synced mirror of the on-chain Coprocessor registry.
///
/// `N` defaults to [`Ethereum`] but any [`Network`] the caller's provider is built for works.
#[derive(Clone)]
pub struct CoprocessorRegistry<P: Provider<N>, N: Network = Ethereum> {
    gateway_config_contract: GatewayConfigInstance<P, N>,
    snapshot: Arc<RwLock<Arc<CoprocessorRegistrySnapshot>>>,
    critical_failure: Arc<RwLock<Option<String>>>,
}

/// An immutable snapshot of the Coprocessor registry at one point in time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoprocessorRegistrySnapshot {
    /// Every registered Coprocessor with a non-empty bucket. (`getCoprocessorTxSenders` +
    /// `getCoprocessor(addr)`)
    pub coprocessors: Vec<CoprocessorEntry>,
    /// Number of agreeing signers required for consensus. (`getCoprocessorMajorityThreshold`)
    pub threshold: NonZeroUsize,
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

    /// The registry could not be read, e.g. the gateway RPC was unreachable. A retry may fix it,
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

    /// Loads a fresh snapshot from the `GatewayConfig` contract.
    pub async fn load<P: Provider<N>, N: Network>(
        contract: &GatewayConfigInstance<P, N>,
    ) -> Result<Self, RegistryError> {
        let get_copro_tx_senders = contract.getCoprocessorTxSenders();
        let get_copro_threshold = contract.getCoprocessorMajorityThreshold();

        let (tx_senders, threshold_u256) = tokio::try_join!(
            biased;
            get_copro_tx_senders.call(),
            get_copro_threshold.call()
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

        let coprocessors: Vec<CoprocessorEntry> =
            try_join_all(tx_senders.into_iter().map(|tx_sender| async move {
                fetch_coprocessor_entry(contract, tx_sender).await
            }))
            .await
            .map_err(RegistryError::Transient)?
            .into_iter()
            .flatten()
            .collect();

        // Entries without a bucket URL are dropped above, while the threshold counts every
        // registered Coprocessor, so a clean read can still fall short. Retrying reads the same
        // list, so this is `Critical`.
        if coprocessors.len() < threshold.get() {
            return Err(RegistryError::Critical(format!(
                "{} of the {threshold} Coprocessors required for a majority have a registered \
                 bucket URL",
                coprocessors.len()
            )));
        }

        Ok(Self::new(coprocessors, threshold))
    }
}

/// Stringifies an RPC error, stripping the ` for url (…)` suffix that `reqwest::Error::Display`
/// appends: Gateway RPC endpoints can carry embedded API keys.
fn redact_rpc_url(err: impl std::fmt::Display) -> String {
    let msg = err.to_string();
    match msg.find(" for url (") {
        Some(at) => msg[..at].to_owned(),
        None => msg,
    }
}

/// Resolves the signer↔bucket binding of a single Coprocessor. An empty `s3BucketUrl` is skipped
/// with a warning: it is persistent on-chain state, so failing on it would crash-loop the caller.
async fn fetch_coprocessor_entry<P: Provider<N>, N: Network>(
    contract: &GatewayConfigInstance<P, N>,
    copro_tx_sender_addr: Address,
) -> anyhow::Result<Option<CoprocessorEntry>> {
    let copro = contract
        .getCoprocessor(copro_tx_sender_addr)
        .call()
        .await
        .map_err(|e| anyhow::anyhow!("{}", redact_rpc_url(e)))?;
    if copro.s3BucketUrl.is_empty() {
        warn!("No S3 bucket URL registered for Coprocessor {copro_tx_sender_addr}, skipping it");
        return Ok(None);
    }
    Ok(Some(CoprocessorEntry {
        tx_sender: copro_tx_sender_addr,
        signer: copro.signerAddress,
        bucket: copro.s3BucketUrl,
    }))
}

impl<P, N> CoprocessorRegistry<P, N>
where
    P: Provider<N> + Clone + 'static,
    N: Network,
{
    /// Loads the initial snapshot and spawns the background refresh task, which runs until
    /// `cancel_token` is cancelled.
    pub async fn connect(
        provider: P,
        gateway_config_address: Address,
        refresh_interval: Duration,
        cancel_token: CancellationToken,
    ) -> anyhow::Result<Self> {
        // Check for code first — `load` would see an address with no code as a decode failure
        // and classify it `Transient`.
        let deployed_code = provider
            .get_code_at(gateway_config_address)
            .await
            .map_err(|e| anyhow::anyhow!("{}", redact_rpc_url(e)))?;

        let gateway_config_contract = GatewayConfig::new(gateway_config_address, provider);
        let loaded = if deployed_code.is_empty() {
            Err(RegistryError::Critical(format!(
                "no contract deployed at the configured GatewayConfig address \
                 {gateway_config_address}"
            )))
        } else {
            CoprocessorRegistrySnapshot::load(&gateway_config_contract).await
        };

        // `Critical` here is chain-side config, so the consumer starts and refuses per request
        // instead of failing to boot. The placeholder is never read: only a successful refresh
        // clears the reason.
        let (snapshot, critical_failure) = match loaded {
            Ok(snapshot) => (snapshot, None),
            Err(RegistryError::Critical(reason)) => {
                // TODO: alert on this, once a metric exists to carry it.
                error!("Coprocessor registry unusable at startup: {reason}");
                (
                    CoprocessorRegistrySnapshot::new(Vec::new(), NonZeroUsize::MIN),
                    Some(reason),
                )
            }
            Err(transient) => return Err(transient.into()),
        };

        let registry = Self {
            gateway_config_contract,
            snapshot: Arc::new(RwLock::new(Arc::new(snapshot))),
            critical_failure: Arc::new(RwLock::new(critical_failure)),
        };
        registry.spawn_refresh_task(refresh_interval, cancel_token);

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
                    Ok(()) => this.set_critical_failure(None),
                    Err(RegistryError::Transient(e)) => warn!(
                        "Failed to refresh Coprocessor registry, keeping previous snapshot: {e}"
                    ),
                    Err(RegistryError::Critical(critical)) => {
                        // TODO: alert on this, once a metric exists to carry it.
                        error!(
                            "Coprocessor registry unusable, keeping previous snapshot: {critical}"
                        );
                        this.set_critical_failure(Some(critical));
                    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        primitives::{Bytes, U256},
        providers::{ProviderBuilder, mock::Asserter},
        sol_types::SolValue,
    };
    use fhevm_gateway_bindings::gateway_config::GatewayConfig::Coprocessor;

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

    fn mocked_contract(asserter: &Asserter) -> GatewayConfigInstance<impl Provider + Clone> {
        let provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter.clone());
        GatewayConfig::new(Address::ZERO, provider)
    }

    /// Pushes the snapshot responses: tx-senders, threshold, then one `getCoprocessor` response
    /// per bucket URL, each with a distinct `signerAddress` (`tx_senders[i]` is bound to
    /// `signer(i)`).
    fn mock_registry_load(asserter: &Asserter, tx_senders: &[Address], buckets: &[&str]) {
        mock_registry_load_at_threshold(asserter, tx_senders, buckets, U256::ONE);
    }

    /// As [`mock_registry_load`], with the on-chain majority threshold chosen by the caller.
    fn mock_registry_load_at_threshold(
        asserter: &Asserter,
        tx_senders: &[Address],
        buckets: &[&str],
        threshold: U256,
    ) {
        asserter.push_success(&tx_senders.to_vec().abi_encode());
        asserter.push_success(&threshold.abi_encode());
        for (index, bucket) in buckets.iter().enumerate() {
            let coprocessor = Coprocessor {
                signerAddress: signer(index as u8),
                s3BucketUrl: bucket.to_string(),
                ..Default::default()
            };
            asserter.push_success(&coprocessor.abi_encode());
        }
    }

    /// Distinct signer addresses, disjoint from [`addr`]'s tx-sender range.
    fn signer(index: u8) -> Address {
        Address::repeat_byte(0x50 + index)
    }

    #[tokio::test]
    async fn load_skips_coprocessor_with_empty_bucket_url() {
        let asserter = Asserter::new();
        let (bad_copro, good_copro) = (addr(0x02), addr(0x03));
        mock_registry_load(&asserter, &[bad_copro, good_copro], &["", "http://bucket"]);

        let snapshot = CoprocessorRegistrySnapshot::load(&mocked_contract(&asserter))
            .await
            .unwrap();
        assert_eq!(
            snapshot.coprocessors,
            vec![CoprocessorEntry {
                tx_sender: good_copro,
                signer: signer(1),
                bucket: "http://bucket".to_string(),
            }]
        );
    }

    #[tokio::test]
    async fn load_rejects_all_bucket_urls_empty_as_critical() {
        let asserter = Asserter::new();
        mock_registry_load(&asserter, &[addr(0x04)], &[""]);

        let err = CoprocessorRegistrySnapshot::load(&mocked_contract(&asserter))
            .await
            .unwrap_err();
        assert!(matches!(err, RegistryError::Critical(_)), "{err:?}");
    }

    /// A clean read that is short of the threshold is `Critical`: a retry reads the same list.
    #[tokio::test]
    async fn load_rejects_a_list_short_of_the_threshold_as_critical() {
        let asserter = Asserter::new();
        mock_registry_load_at_threshold(
            &asserter,
            &[addr(0x04), addr(0x05)],
            &["http://bucket", ""],
            U256::from(2),
        );

        let err = CoprocessorRegistrySnapshot::load(&mocked_contract(&asserter))
            .await
            .unwrap_err();
        let RegistryError::Critical(reason) = err else {
            panic!("a short list did not classify as critical: {err:?}");
        };
        assert!(reason.contains('1') && reason.contains('2'), "{reason}");
    }

    /// Pushes the `eth_getCode` response `connect` probes with before its first load.
    fn mock_deployed_code(asserter: &Asserter) {
        asserter.push_success(&Bytes::from_static(&[0x60]));
    }

    /// Pushes a `load` that fails with [`RegistryError::Critical`] (zero on-chain threshold).
    fn mock_critical_load(asserter: &Asserter) {
        asserter.push_success(&vec![addr(0x07)].abi_encode());
        asserter.push_success(&U256::ZERO.abi_encode());
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
        mock_registry_load(&asserter, &[addr(0x07)], &["http://bucket"]);
        // Twice, so the flag stays observably set for longer than one refresh interval.
        mock_critical_load(&asserter);
        mock_critical_load(&asserter);
        mock_registry_load(&asserter, &[addr(0x07)], &["http://bucket"]);

        let provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter.clone());
        let registry = CoprocessorRegistry::connect(
            provider,
            Address::ZERO,
            Duration::from_millis(100),
            CancellationToken::new(),
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
        )
        .await
        .expect("connect refused to start over chain-side config");

        let reason = registry.critical_failure().expect("no critical reason set");
        assert!(reason.contains("no contract deployed"), "{reason}");
    }

    #[tokio::test]
    async fn load_rejects_zero_threshold_as_critical() {
        let asserter = Asserter::new();
        asserter.push_success(&vec![addr(0x06)].abi_encode());
        asserter.push_success(&U256::ZERO.abi_encode());

        let err = CoprocessorRegistrySnapshot::load(&mocked_contract(&asserter))
            .await
            .unwrap_err();
        assert!(matches!(err, RegistryError::Critical(_)));
    }
}
