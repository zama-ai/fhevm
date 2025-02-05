use crate::errors::Error;
use crate::errors::EventProcessingError;
use alloy::primitives::Address;
use alloy::{
    providers::{Provider, ProviderBuilder, WsConnect},
    pubsub::PubSubFrontend,
    rpc::types::{BlockNumberOrTag, Filter, Log as RpcLog},
};

use std::sync::Arc;
use tracing::{info, instrument};

pub struct ContractAndTopicsFilter {
    contract_addresses: Vec<Address>,
    topics: Vec<String>,
}

impl ContractAndTopicsFilter {
    pub fn new(contract_addresses: Vec<Address>, topics: Vec<String>) -> Self {
        Self {
            contract_addresses,
            topics,
        }
    }
    fn ethereum_filter(&self, block_number_or_tag: BlockNumberOrTag) -> Filter {
        let filter = Filter::new()
            .from_block(block_number_or_tag)
            .address(self.contract_addresses.clone());
        filter
    }
}

pub struct EthereumHostL1 {
    provider: Arc<dyn Provider<PubSubFrontend> + Send + Sync>,
}

unsafe impl Send for EthereumHostL1 {}
unsafe impl Sync for EthereumHostL1 {}

impl EthereumHostL1 {
    #[instrument(skip_all)]
    pub async fn new(ws_url: &str) -> Result<Self, Error> {
        let ws = WsConnect::new(ws_url);
        let provider = ProviderBuilder::new()
            .on_ws(ws)
            .await
            .map_err(Error::Transport)?;

        Ok(EthereumHostL1 {
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
        let filter = filter.ethereum_filter(block_number_or_tag);

        let sub = self
            .provider
            .subscribe_logs(&filter)
            .await
            .map_err(Error::Transport)?;

        info!("Subscription successful. Listening for logs...");
        let stream = sub.into_stream();
        Ok(stream)

        // let subscription = self.new_subscription(filter).await?;

        // self.listen_and_process(subscription).await;

        // Ok(())
    }

    // async fn listen_and_process(
    //     &self,
    //     mut subscription: alloy::pubsub::SubscriptionStream<RpcLog>,
    // ) {
    //     while let Some(log) = subscription.next().await {
    //         debug!("Received Log: {:#?}", log);
    //         let contract_address = log.inner.address;

    //         if let Some(event_topic) = self.extract_event_topic(&log) {
    //             if let Err(e) = self
    //                 .registry
    //                 .process_event(contract_address, &event_topic, &log)
    //             {
    //                 warn!(error = ?e, "Failed to process event");
    //             }
    //         }
    //     }
    // }

    // async fn new_subscription(
    //     &self,
    //     filter: Filter,
    // ) -> Result<alloy::pubsub::SubscriptionStream<RpcLog>, Error> {
    // }
}

pub fn extract_event_signature(
    log: &RpcLog,
) -> Result<&alloy::primitives::FixedBytes<32>, EventProcessingError> {
    log.inner
        .data
        .topics()
        .first()
        .ok_or(EventProcessingError::MissingTopic)
}
