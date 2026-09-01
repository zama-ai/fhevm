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
    sync::{
        Arc, RwLock,
        atomic::{AtomicBool, Ordering},
    },
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
    refresh_failed_critically: Arc<AtomicBool>,
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
    /// No retry will fix this: an on-chain majority threshold that is not a `NonZeroUsize`, or a
    /// poisoned snapshot lock. An operator has to intervene.
    #[error("critical Coprocessor registry error: {0}")]
    Critical(String),

    /// A retry may well fix this, e.g. the gateway RPC was unreachable. The previous snapshot
    /// stays in place until one does.
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

    /// Coprocessors without a bucket URL are dropped at load, but the on-chain threshold counts
    /// every registered one — so an accurate snapshot can still put consensus out of reach.
    pub fn consensus_reachable(&self) -> bool {
        self.coprocessors.len() >= self.threshold.get()
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

        if tx_senders.is_empty() {
            return Err(RegistryError::Transient(anyhow::anyhow!(
                "Not a single Coprocessor tx sender in the registry"
            )));
        }

        let coprocessors: Vec<CoprocessorEntry> =
            try_join_all(tx_senders.into_iter().map(|tx_sender| async move {
                fetch_coprocessor_entry(contract, tx_sender).await
            }))
            .await
            .map_err(RegistryError::Transient)?
            .into_iter()
            .flatten()
            .collect();

        if coprocessors.is_empty() {
            return Err(RegistryError::Transient(anyhow::anyhow!(
                "Not a single Coprocessor with a non-empty S3 bucket URL in the registry"
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
        // Without this, a codeless address reaches `load` as a decode failure and classifies as
        // `Transient` — a permanent fault dressed as a retryable one.
        if provider
            .get_code_at(gateway_config_address)
            .await
            .map_err(|e| anyhow::anyhow!("{}", redact_rpc_url(e)))?
            .is_empty()
        {
            return Err(RegistryError::Critical(format!(
                "no contract deployed at the configured GatewayConfig address \
                 {gateway_config_address}"
            ))
            .into());
        }

        let gateway_config_contract = GatewayConfig::new(gateway_config_address, provider);

        let snapshot = CoprocessorRegistrySnapshot::load(&gateway_config_contract).await?;
        let registry = Self {
            gateway_config_contract,
            snapshot: Arc::new(RwLock::new(Arc::new(snapshot))),
            refresh_failed_critically: Arc::new(AtomicBool::new(false)),
        };
        registry.spawn_refresh_task(refresh_interval, cancel_token);

        Ok(registry)
    }

    /// True while the served snapshot is the last good one and the registry could not replace it.
    pub fn last_refresh_failed_critically(&self) -> bool {
        self.refresh_failed_critically.load(Ordering::Relaxed)
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
                    Ok(()) => this
                        .refresh_failed_critically
                        .store(false, Ordering::Relaxed),
                    Err(RegistryError::Transient(e)) => warn!(
                        "Failed to refresh Coprocessor registry, keeping previous snapshot: {e}"
                    ),
                    Err(RegistryError::Critical(critical)) => {
                        this.refresh_failed_critically
                            .store(true, Ordering::Relaxed);
                        error!(
                            "Coprocessor registry unusable, keeping previous snapshot: {critical}"
                        );
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
        asserter.push_success(&tx_senders.to_vec().abi_encode());
        asserter.push_success(&U256::ONE.abi_encode());
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
    async fn load_fails_when_all_bucket_urls_are_empty() {
        let asserter = Asserter::new();
        mock_registry_load(&asserter, &[addr(0x04)], &[""]);

        CoprocessorRegistrySnapshot::load(&mocked_contract(&asserter))
            .await
            .unwrap_err();
    }

    #[test]
    fn consensus_is_reachable_exactly_at_the_threshold() {
        let entry = |byte| CoprocessorEntry {
            tx_sender: addr(byte),
            signer: signer(byte),
            bucket: "http://bucket".to_string(),
        };
        let snapshot = |count: u8| {
            CoprocessorRegistrySnapshot::new(
                (0..count).map(entry).collect(),
                NonZeroUsize::new(2).unwrap(),
            )
        };

        assert!(!snapshot(1).consensus_reachable());
        assert!(snapshot(2).consensus_reachable());
        assert!(snapshot(3).consensus_reachable());
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

        assert!(!registry.last_refresh_failed_critically());
        eventually("a critical refresh", || {
            registry.last_refresh_failed_critically()
        })
        .await;
        eventually("a recovered refresh", || {
            !registry.last_refresh_failed_critically()
        })
        .await;
    }

    #[tokio::test]
    async fn connect_rejects_an_address_with_no_contract_code() {
        let asserter = Asserter::new();
        asserter.push_success(&Bytes::new());

        let Err(err) = CoprocessorRegistry::connect(
            ProviderBuilder::new()
                .disable_recommended_fillers()
                .connect_mocked_client(asserter.clone()),
            Address::ZERO,
            Duration::from_secs(60),
            CancellationToken::new(),
        )
        .await
        else {
            panic!("connect accepted an address with no contract code");
        };
        assert!(err.to_string().contains("no contract deployed"));
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
