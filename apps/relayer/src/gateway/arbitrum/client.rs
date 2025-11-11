use crate::core::errors::Error;
use crate::gateway::arbitrum::ContractAndTopicsFilter;
use alloy::{
    network::AnyNetwork,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Log as RpcLog},
};
use serde::{Deserialize, Serialize};

use std::sync::Arc;
use tracing::{info, instrument};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainName {
    Gateway,
}

pub struct ArbitrumJsonRPCWsClient {
    provider: Arc<dyn Provider<AnyNetwork> + Send + Sync>,
}

unsafe impl Send for ArbitrumJsonRPCWsClient {}
unsafe impl Sync for ArbitrumJsonRPCWsClient {}

impl ArbitrumJsonRPCWsClient {
    #[instrument(skip_all)]
    pub async fn new(ws_url: &str) -> Result<Self, Error> {
        let ws = WsConnect::new(ws_url);
        let provider = ProviderBuilder::new()
            .network::<alloy::network::AnyNetwork>()
            .connect_ws(ws)
            .await
            .map_err(Error::Transport)?;

        let provider = Arc::new(provider);

        Ok(ArbitrumJsonRPCWsClient { provider })
    }

    pub async fn new_subscription(
        &self,
        filter: ContractAndTopicsFilter,
        from_block_number: Option<u64>,
    ) -> Result<alloy::pubsub::SubscriptionStream<RpcLog>, Error> {
        let block_number_or_tag = from_block_number
            .map(BlockNumberOrTag::Number)
            .unwrap_or(BlockNumberOrTag::Latest);
        let filter = filter.to_eth_subscription_filter(block_number_or_tag);

        let sub = self
            .provider
            .subscribe_logs(&filter)
            .await
            .map_err(Error::Transport)?;

        info!("Subscription to gateway chain is successful. Listening for logs...",);
        let stream = sub.into_stream();
        Ok(stream)
    }
}
