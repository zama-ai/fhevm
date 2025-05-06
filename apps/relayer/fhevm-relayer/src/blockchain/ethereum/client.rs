use crate::blockchain::ethereum::ContractAndTopicsFilter;
use crate::core::errors::Error;
use alloy::{
    providers::{Provider, ProviderBuilder, WsConnect},
    pubsub::PubSubFrontend,
    rpc::types::{BlockNumberOrTag, Log as RpcLog},
};
use serde::{Deserialize, Serialize};

use std::sync::Arc;
use tracing::{info, instrument};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainName {
    Fhevm,
    Gateway,
}

pub struct EthereumJsonRPCWsClient {
    chain_name: ChainName,
    provider: Arc<dyn Provider<PubSubFrontend> + Send + Sync>,
}

unsafe impl Send for EthereumJsonRPCWsClient {}
unsafe impl Sync for EthereumJsonRPCWsClient {}

impl EthereumJsonRPCWsClient {
    #[instrument(skip_all)]
    pub async fn new(chain_name: ChainName, ws_url: &str) -> Result<Self, Error> {
        let ws = WsConnect::new(ws_url);
        let provider = ProviderBuilder::new()
            .on_ws(ws)
            .await
            .map_err(Error::Transport)?;

        Ok(EthereumJsonRPCWsClient {
            chain_name,
            provider: Arc::new(provider),
        })
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

        info!(
            "Subscription to {:?} is successful. Listening for logs...",
            self.chain_name
        );
        let stream = sub.into_stream();
        Ok(stream)
    }
}
