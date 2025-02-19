use crate::errors::Error;
use crate::ethereum::ContractAndTopicsFilter;
use alloy::{
    providers::{Provider, ProviderBuilder, WsConnect},
    pubsub::PubSubFrontend,
    rpc::types::{BlockNumberOrTag, Log as RpcLog},
};

use std::sync::Arc;
use tracing::{info, instrument};

pub struct RollupL2 {
    provider: Arc<dyn Provider<PubSubFrontend> + Send + Sync>,
}

unsafe impl Send for RollupL2 {}
unsafe impl Sync for RollupL2 {}

impl RollupL2 {
    #[instrument(skip_all)]
    pub async fn new(ws_url: &str) -> Result<Self, Error> {
        let ws = WsConnect::new(ws_url);
        let provider = ProviderBuilder::new()
            .on_ws(ws)
            .await
            .map_err(Error::Transport)?;

        Ok(RollupL2 {
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

        info!("Subscription successful to Rollup L2 aka Gateway. Listening for logs...");
        let stream = sub.into_stream();
        Ok(stream)
    }
}
