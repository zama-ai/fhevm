use crate::{
    config::settings::{ProtocolConfigSettings, RetrySettings},
    host::error_redact::redact_alloy_error,
};
use alloy::{
    primitives::{Address, U256},
    providers::{fillers::FillProvider, ProviderBuilder, RootProvider},
};
use fhevm_host_bindings::i_protocol_config::IProtocolConfig;
use fhevm_host_bindings::i_protocol_config::IProtocolConfig::IProtocolConfigInstance;
use moka::future::Cache;
use reqwest::Url;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, warn};

type Provider = FillProvider<
    alloy::providers::fillers::JoinFill<
        alloy::providers::Identity,
        alloy::providers::fillers::JoinFill<
            alloy::providers::fillers::GasFiller,
            alloy::providers::fillers::JoinFill<
                alloy::providers::fillers::BlobGasFiller,
                alloy::providers::fillers::JoinFill<
                    alloy::providers::fillers::NonceFiller,
                    alloy::providers::fillers::ChainIdFiller,
                >,
            >,
        >,
    >,
    RootProvider<alloy::network::AnyNetwork>,
    alloy::network::AnyNetwork,
>;

type HostProtocolConfig = IProtocolConfigInstance<Arc<Provider>, alloy::network::AnyNetwork>;

#[derive(Debug, thiserror::Error)]
pub enum ThresholdResolverError {
    #[error(
        "Failed to fetch threshold for context {context_id} after {attempts} attempts: {message}"
    )]
    FetchFailed {
        context_id: U256,
        attempts: u32,
        message: String,
    },
}

/// Resolves the user decrypt threshold per KMS context ID by querying the
/// ProtocolConfig contract on the Ethereum host chain.
///
/// Uses moka over DashMap/HashMap+Mutex because:
/// 1. `entry().or_try_insert_with()` coalesces concurrent fetches for the same
///    key — only one RPC call is made, others wait for its result.
/// 2. Built-in max capacity bound with eviction.
///
/// Thresholds are cached permanently (no TTL) since they don't change after
/// context creation. Context ID 0 is pre-seeded with the static config default.
pub struct ThresholdResolver {
    contract: HostProtocolConfig,
    /// Cached thresholds keyed by context ID. The on-chain uint256 is narrowed
    /// to u32 at the contract fetch boundary with an explicit range check.
    /// The repository layer handles u32 ↔ i64 conversion for DB storage (BIGINT).
    context_thresholds: Cache<U256, u32>,
    retry_config: RetrySettings,
}

impl ThresholdResolver {
    pub async fn new(
        config: &ProtocolConfigSettings,
        default_threshold: u32,
        max_capacity: u64,
    ) -> anyhow::Result<Self> {
        let url = Url::parse(&config.ethereum_http_rpc_url)
            .map_err(|e| anyhow::anyhow!("Invalid ProtocolConfig URL: {}", e))?;

        let address = Address::from_str(&config.address)
            .map_err(|e| anyhow::anyhow!("Invalid ProtocolConfig address: {}", e))?;

        let provider = Arc::new(
            ProviderBuilder::new()
                .network::<alloy::network::AnyNetwork>()
                .connect_http(url),
        );

        let contract = IProtocolConfig::new(address, provider);

        let context_thresholds = Cache::builder().max_capacity(max_capacity).build();

        // Pre-seed default: context_id 0 is invalid and maps to the static config value
        context_thresholds
            .insert(U256::ZERO, default_threshold)
            .await;

        Ok(Self {
            contract,
            context_thresholds,
            retry_config: config.retry.clone(),
        })
    }

    /// Resolve the user decrypt threshold for a given context ID.
    /// Returns error if all retries are exhausted (errors are not cached).
    pub async fn resolve(&self, context_id: U256) -> Result<u32, ThresholdResolverError> {
        let contract = self.contract.clone();
        let retry_config = self.retry_config.clone();

        self.context_thresholds
            .entry(context_id)
            .or_try_insert_with(async move {
                fetch_with_retry(&contract, &retry_config, context_id).await
            })
            .await
            .map(|entry| entry.into_value())
            .map_err(|arc_err| {
                // Unwrap the Arc — we own the only reference after moka returns
                match Arc::try_unwrap(arc_err) {
                    Ok(err) => err,
                    Err(arc) => ThresholdResolverError::FetchFailed {
                        context_id,
                        attempts: arc.attempts(),
                        message: arc.to_string(),
                    },
                }
            })
    }
}

impl ThresholdResolverError {
    fn attempts(&self) -> u32 {
        match self {
            Self::FetchFailed { attempts, .. } => *attempts,
        }
    }
}

async fn fetch_with_retry(
    contract: &HostProtocolConfig,
    retry_config: &RetrySettings,
    context_id: U256,
) -> Result<u32, ThresholdResolverError> {
    let max_attempts = retry_config.max_attempts;
    let retry_interval = Duration::from_millis(retry_config.retry_interval_ms);
    let mut last_error = String::new();

    for attempt in 0..max_attempts {
        match contract
            .getUserDecryptionThresholdForContext(context_id)
            .call()
            .await
        {
            Ok(ret) => {
                let threshold: u32 =
                    u32::try_from(ret).map_err(|_| ThresholdResolverError::FetchFailed {
                        context_id,
                        attempts: attempt + 1,
                        message: format!("threshold {} exceeds u32 range", ret),
                    })?;
                debug!(
                    context_id = %context_id,
                    threshold,
                    "Fetched threshold from ProtocolConfig"
                );
                return Ok(threshold);
            }
            Err(e) => {
                last_error = redact_alloy_error(&e);
                if attempt + 1 < max_attempts {
                    warn!(
                        context_id = %context_id,
                        attempt = attempt + 1,
                        max_attempts,
                        error = %last_error,
                        "Threshold fetch failed, retrying"
                    );
                    tokio::time::sleep(retry_interval).await;
                }
            }
        }
    }

    error!(
        context_id = %context_id,
        error = %last_error,
        "Threshold fetch failed after all retries"
    );
    Err(ThresholdResolverError::FetchFailed {
        context_id,
        attempts: max_attempts,
        message: last_error,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn context_zero_returns_preseeded_default() {
        let config = ProtocolConfigSettings {
            ethereum_http_rpc_url: "http://localhost:9999".to_string(),
            address: "0x1234567890123456789012345678901234567890".to_string(),
            retry: RetrySettings {
                max_attempts: 1,
                retry_interval_ms: 10,
            },
        };
        let resolver = ThresholdResolver::new(&config, 9u32, 100).await.unwrap();

        // context_id 0 always hits the pre-seeded entry
        assert_eq!(resolver.resolve(U256::ZERO).await.unwrap(), 9u32);
    }

    #[tokio::test]
    async fn unconfigured_chain_fails_for_nonzero_context() {
        let config = ProtocolConfigSettings {
            // Points at nothing — RPC will fail
            ethereum_http_rpc_url: "http://localhost:1".to_string(),
            address: "0x1234567890123456789012345678901234567890".to_string(),
            retry: RetrySettings {
                max_attempts: 1,
                retry_interval_ms: 10,
            },
        };
        let resolver = ThresholdResolver::new(&config, 9u32, 100).await.unwrap();

        // Non-zero context_id with unreachable RPC should error
        let result = resolver.resolve(U256::from(42)).await;
        assert!(result.is_err());
    }
}
