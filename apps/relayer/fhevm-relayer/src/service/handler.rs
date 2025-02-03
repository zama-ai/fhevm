use crate::{errors::Error, event::registry::EventRegistry};
use alloy::{
    providers::{Provider, ProviderBuilder, WsConnect},
    pubsub::PubSubFrontend,
    rpc::types::{BlockNumberOrTag, Filter, Log as RpcLog},
};
use futures_util::StreamExt;
use std::sync::Arc;
use tracing::{debug, info, instrument, warn};

pub struct RealEventHandler {
    provider: Arc<dyn Provider<PubSubFrontend> + Send + Sync>,
    registry: Arc<EventRegistry>,
}

unsafe impl Send for RealEventHandler {}
unsafe impl Sync for RealEventHandler {}

impl RealEventHandler {
    #[instrument(skip_all)]
    pub async fn new(ws_url: &str, registry: Arc<EventRegistry>) -> Result<Self, Error> {
        let ws = WsConnect::new(ws_url);
        let provider = ProviderBuilder::new()
            .on_ws(ws)
            .await
            .map_err(Error::Transport)?;

        Ok(RealEventHandler {
            provider: Arc::new(provider),
            registry,
        })
    }

    pub async fn new_subscription(
        &self,
    ) -> Result<alloy::pubsub::SubscriptionStream<RpcLog>, Error> {
        let contracts = self.registry.get_contracts();

        let filter = new_filter(contracts);
        info!("Subscribing to logs with filters: {:?}", filter);

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

    fn extract_event_topic(&self, log: &RpcLog) -> Option<String> {
        log.inner.data.topics().first().map(|sig| sig.to_string())
    }
}

fn new_filter(contracts: Vec<alloy::primitives::Address>) -> Filter {
    info!("Subscribing to logs for contracts: {:?}", contracts);
    info!("Connecting to Ethereum provider...");

    let filter = Filter::new()
        .from_block(BlockNumberOrTag::Latest)
        .address(contracts);
    filter
}
