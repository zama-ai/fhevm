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

    pub async fn listen_for_contract_events(&self) -> Result<(), Error> {
        let contracts = self.registry.get_contracts();

        info!("Subscribing to logs for contracts: {:?}", contracts);
        info!("Connecting to Ethereum provider...");

        let filter = Filter::new()
            .from_block(BlockNumberOrTag::Latest)
            .address(contracts);

        info!("Subscribing to logs with filters: {:?}", filter);

        let sub = self
            .provider
            .subscribe_logs(&filter)
            .await
            .map_err(Error::Transport)?;

        info!("Subscription successful. Listening for logs...");

        let mut stream = sub.into_stream();

        while let Some(log) = stream.next().await {
            debug!("Received Log: {:#?}", log);
            let contract_address = log.inner.address;

            if let Some(event_name) = self.identify_event(&log) {
                if let Err(e) = self
                    .registry
                    .process_event(contract_address, &event_name, &log)
                {
                    warn!(error = ?e, "Failed to process event");
                }
            }
        }

        Ok(())
    }

    fn identify_event(&self, log: &RpcLog) -> Option<String> {
        log.inner.data.topics().first().map(|sig| sig.to_string())
    }
}
