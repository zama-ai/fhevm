use crate::{
    ethereum::bindings::DecryptionOracle,
    orchestrator::{traits::EventDispatcher, traits::EventHandler, TokioEventDispatcher},
    relayer_event::{DecryptionType, RelayerEvent, RelayerEventData},
};
use alloy::rpc::types::Log;
use async_trait::async_trait;
use std::sync::Arc;

use alloy_sol_types::SolEvent;

pub struct EthereumHostL1EventLogHandler {
    dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
}

impl EthereumHostL1EventLogHandler {
    pub fn new(dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>) -> Self {
        Self { dispatcher }
    }
}

#[async_trait]
impl EventHandler<RelayerEvent> for EthereumHostL1EventLogHandler {
    async fn handle_event(&self, event: RelayerEvent) {
        let eth_event_log: Log;
        match event.clone().data {
            RelayerEventData::HostL1EventLogReceived { log: l } => eth_event_log = l,
            _ => {
                return;
            }
        }

        let next_event: RelayerEvent;
        match DecryptionOracle::DecryptionRequest::decode_log_data(eth_event_log.data(), true) {
            Ok(_eth_decryption_request) => {
                next_event = event.derive_next_event(RelayerEventData::DecryptionRequestReceived {
                    ct_handle: "sample ct handler".to_string(),
                    operation: DecryptionType::PublicDecrypt,
                });
            }
            Err(e) => {
                next_event = event.derive_next_event(RelayerEventData::DecryptionFailed {
                    error: format!("error decoding ethereum event log data: {:?}", e),
                });
            }
        }
        self.dispatcher.dispatch_event(next_event).await;
    }
}
