use crate::errors::Result;
use crate::event::processor::EventProcessor;
use crate::event::registry::EventRegistry;
use crate::event::types::EventType;
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use alloy::pubsub::PubSubFrontend;
use alloy::rpc::types::{BlockNumberOrTag, Filter, Log as RpcLog};
use futures_util::StreamExt;
use std::sync::Arc;
use tracing::{debug, info, instrument, warn};

pub struct RealEventHandler {
    provider: Arc<dyn Provider<PubSubFrontend> + Send + Sync>,
    registry: Arc<EventRegistry>, // Uses event registry for contract-specific event handling
}

//  Implement Send + Sync for RealEventHandler
unsafe impl Send for RealEventHandler {}
unsafe impl Sync for RealEventHandler {}

impl RealEventHandler {
    #[instrument(skip_all)]
    pub async fn new(ws_url: &str, registry: Arc<EventRegistry>) -> Result<Self> {
        let ws = WsConnect::new(ws_url);
        let provider = ProviderBuilder::new().on_ws(ws).await?;

        Ok(RealEventHandler {
            provider: Arc::new(provider),
            registry,
        })
    }

    pub async fn listen_for_contract_events(&self) -> Result<()> {
        let contracts = self.registry.get_contracts();

        info!("Subscribing to logs for contracts: {:?}", contracts);

        info!("Connecting to Ethereum provider...");
        // Use a single filter with multiple addresses
        let filter = Filter::new()
            .from_block(BlockNumberOrTag::Latest)
            .address(contracts);

        info!("Subscribing to logs with filters: {:?}", filter);

        let sub = self.provider.subscribe_logs(&filter).await?;

        info!("Subscription successful. Listening for logs...");
        let mut stream = sub.into_stream();

        while let Some(log) = stream.next().await {
            debug!("🔹 Received Log: {:#?}", log);
            let contract_address = log.inner.address;

            if let Some(event_name) = self.identify_event(&log) {
                self.registry
                    .process_event(contract_address, &event_name, &log)
                    .ok();
            }
        }

        Ok(())
    }

    fn identify_event(&self, log: &RpcLog) -> Option<String> {
        // Example: Identify events by first topic (assuming it's a signature)
        let event_name = log
            .inner
            .data
            .topics()
            .first()
            .map(|signature| format!("{:?}", signature));
        debug!("In indentify_event {:?}", event_name);
        event_name
    }
}

impl EventProcessor for RealEventHandler {
    #[instrument(skip_all)]
    fn process_event(&self, log: &RpcLog) -> Result<()> {
        debug!("Processing Event in RealEventHandler");

        let event = EventType::from_log(log);

        match event {
            EventType::EventDecryption(decoded) => {
                info!(?decoded, "Handling EventDecryption from old version");
            }
            EventType::DecryptionRequest(decoded) => {
                info!(?decoded, "Handling DecryptionRequest from new version");
            }

            EventType::FheAdd(decoded) => {
                info!(?decoded, "Handling FheAdd operation from TFHEEXECUTOR");
            }
            EventType::Unknown => {
                warn!("Unknown event type. Skipping log");
            }
        }

        Ok(())
    }
}

impl EventProcessor for Arc<RealEventHandler> {
    fn process_event(&self, log: &RpcLog) -> Result<()> {
        (**self).process_event(log)
    }
}
