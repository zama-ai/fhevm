use crate::core::errors::Error;
use alloy::{
    network::AnyNetwork,
    primitives::Address,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Filter, Log as RpcLog},
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

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
    pub async fn new(
        ws_url: &str,
        max_retries: u32,
        retry_interval_ms: u64,
    ) -> Result<Self, Error> {
        // Configure WebSocket with reconnection parameters
        let ws = WsConnect::new(ws_url)
            .with_max_retries(max_retries)
            .with_retry_interval(Duration::from_millis(retry_interval_ms));

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
        contract_addresses: Vec<Address>,
        from_block_number: Option<u64>,
    ) -> Result<alloy::pubsub::SubscriptionStream<RpcLog>, Error> {
        let block_number_or_tag = from_block_number
            .map(BlockNumberOrTag::Number)
            .unwrap_or(BlockNumberOrTag::Latest);

        let filter = Filter::new()
            .from_block(block_number_or_tag)
            .address(contract_addresses);

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
