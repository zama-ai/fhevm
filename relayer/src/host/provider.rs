//! Shared plumbing for read-only host-chain (Ethereum) contract access.
//!
//! Both [`crate::host::threshold_resolver`] and [`crate::host::keyurl_poller`] build the
//! same alloy HTTP provider from `protocol_config.ethereum_http_rpc_url`, so the provider
//! type and its constructor live here.
//!
//! Retry/error-redaction is intentionally NOT shared as a generic helper: alloy's
//! `CallBuilder::call(&self)` borrows the builder, so a closure that builds-and-calls
//! would drop the temporary builder while the returned future still borrows it. Each
//! caller keeps its retry loop local (see `keyurl_poller::retry_view!`).

use std::sync::Arc;

use alloy::providers::{fillers::FillProvider, ProviderBuilder, RootProvider};
use reqwest::Url;

/// The concrete alloy provider type used for read-only host-chain calls.
pub type Provider = FillProvider<
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

/// Build an HTTP provider for the Ethereum host chain from its RPC URL.
pub fn build_host_provider(rpc_url: &str) -> anyhow::Result<Arc<Provider>> {
    let url =
        Url::parse(rpc_url).map_err(|e| anyhow::anyhow!("Invalid host-chain RPC URL: {e}"))?;
    Ok(Arc::new(
        ProviderBuilder::new()
            .network::<alloy::network::AnyNetwork>()
            .connect_http(url),
    ))
}
