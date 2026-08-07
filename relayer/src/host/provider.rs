//! Shared plumbing for read-only contract access over HTTP.
//!
//! [`crate::host::threshold_resolver`] and [`crate::host::keyurl_poller`] build the same alloy
//! HTTP provider from `protocol_config.ethereum_http_rpc_url`, and
//! [`crate::gateway::ciphertext_checker`] builds the identical thing against the gateway chain's
//! read node, so the provider type and its constructors live here.

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
    build_provider(rpc_url, "host-chain")
}

/// Build an HTTP provider for the gateway chain's read node from its RPC URL.
pub fn build_gateway_provider(rpc_url: &str) -> anyhow::Result<Arc<Provider>> {
    build_provider(rpc_url, "gateway-chain")
}

fn build_provider(rpc_url: &str, chain: &str) -> anyhow::Result<Arc<Provider>> {
    let url = Url::parse(rpc_url).map_err(|e| anyhow::anyhow!("Invalid {chain} RPC URL: {e}"))?;
    Ok(Arc::new(
        ProviderBuilder::new()
            .network::<alloy::network::AnyNetwork>()
            .connect_http(url),
    ))
}
