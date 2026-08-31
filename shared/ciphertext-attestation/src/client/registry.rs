//! TTL'd snapshot of the on-chain Coprocessor registry.
//!
//! A consensus consumer needs the on-chain signer↔bucket binding for every registered
//! Coprocessor (which signer a bucket is allowed to speak for, and the S3 bucket URL to fan
//! attestation HEAD requests at) plus the majority threshold. Querying these on every request
//! would trigger N+1 RPC calls, so the [`CoprocessorRegistry`] holds a whole snapshot behind a
//! short TTL and tolerates registration changes within one refresh window.

use alloy::{
    network::{Ethereum, Network},
    primitives::Address,
    providers::Provider,
};
use fhevm_gateway_bindings::gateway_config::GatewayConfig::{self, GatewayConfigInstance};
use futures::future::try_join_all;
use std::{
    collections::HashSet,
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
}

/// One registered Coprocessor's on-chain identity triple: which tx sender it is, which signer it
/// is bound to, and which bucket serves its attestations. (`getCoprocessor(txSender)`)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoprocessorEntry {
    pub tx_sender: Address,
    pub signer: Address,
    pub bucket: String,
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
#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    /// A condition that can never arise from a healthy protocol: an invalid on-chain threshold
    /// or a poisoned snapshot lock. The caller should treat this as fatal.
    #[error("critical Coprocessor registry error: {0}")]
    Critical(String),

    /// A recoverable failure, e.g. a transient RPC error. The previous snapshot is kept.
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

        // Defense-in-depth: `GatewayConfig.sol` reverts duplicate registrations, so a duplicate
        // here can only mean a broken invariant somewhere upstream. Fail closed rather than
        // silently letting one bucket vote twice.
        let mut seen_tx_senders = HashSet::with_capacity(coprocessors.len());
        let mut seen_signers = HashSet::with_capacity(coprocessors.len());
        for entry in &coprocessors {
            if !seen_tx_senders.insert(entry.tx_sender) {
                return Err(RegistryError::Critical(format!(
                    "duplicate Coprocessor tx sender in registry: {}",
                    entry.tx_sender
                )));
            }
            if !seen_signers.insert(entry.signer) {
                return Err(RegistryError::Critical(format!(
                    "duplicate Coprocessor signer in registry: {}",
                    entry.signer
                )));
            }
        }

        // Not `Critical`: crash-looping the caller over persistent on-chain state would be worse.
        if coprocessors.len() < threshold.get() {
            error!(
                reachable = coprocessors.len(),
                threshold = threshold.get(),
                "Fewer Coprocessors have a registered S3 bucket URL than the on-chain majority \
                 threshold requires: attestation consensus is unreachable and every decryption \
                 request will be refused until the missing bucket URLs are registered"
            );
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
    /// Loads the initial snapshot and spawns the background refresh task. `cancel_token` is the
    /// caller-wide shutdown token: the refresh task cancels it on a critical failure.
    pub async fn connect(
        provider: P,
        gateway_config_address: Address,
        refresh_interval: Duration,
        cancel_token: CancellationToken,
    ) -> anyhow::Result<Self> {
        let gateway_config_contract = GatewayConfig::new(gateway_config_address, provider);

        let snapshot = CoprocessorRegistrySnapshot::load(&gateway_config_contract).await?;
        let registry = Self {
            gateway_config_contract,
            snapshot: Arc::new(RwLock::new(Arc::new(snapshot))),
        };
        registry.spawn_refresh_task(refresh_interval, cancel_token);

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
                        error!("Shutting down on critical registry failure: {critical}");
                        cancel_token.cancel();
                        break;
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
        primitives::U256,
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

    #[tokio::test]
    async fn load_rejects_duplicate_tx_sender_as_critical() {
        let asserter = Asserter::new();
        let dup = addr(0x07);
        mock_registry_load(&asserter, &[dup, dup], &["http://a", "http://b"]);

        let err = CoprocessorRegistrySnapshot::load(&mocked_contract(&asserter))
            .await
            .unwrap_err();
        assert!(matches!(err, RegistryError::Critical(_)));
    }

    #[tokio::test]
    async fn load_rejects_duplicate_signer_as_critical() {
        let asserter = Asserter::new();
        // Two distinct tx senders, but `mock_registry_load` binds both to `signer(0)` here since
        // both `getCoprocessor` responses are built with the same explicit `signerAddress`.
        asserter.push_success(&vec![addr(0x08), addr(0x09)].abi_encode());
        asserter.push_success(&U256::ONE.abi_encode());
        for tx_sender in [addr(0x08), addr(0x09)] {
            let coprocessor = Coprocessor {
                signerAddress: signer(0),
                s3BucketUrl: format!("http://{tx_sender}"),
                ..Default::default()
            };
            asserter.push_success(&coprocessor.abi_encode());
        }

        let err = CoprocessorRegistrySnapshot::load(&mocked_contract(&asserter))
            .await
            .unwrap_err();
        assert!(matches!(err, RegistryError::Critical(_)));
    }
}
