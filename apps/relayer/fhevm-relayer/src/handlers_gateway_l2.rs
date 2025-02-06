use crate::{
    ethereum::bindings::DecryptionOracle,
    orchestrator::{traits::EventHandler, TokioEventDispatcher},
    relayer_event::{DecryptedValue, DecryptionType, RelayerEvent, RelayerEventData},
};
use alloy::primitives::{FixedBytes, Uint};
use alloy::rpc::types::Log;
use std::sync::Arc;
use uuid::Uuid;

use alloy_sol_types::SolEvent;

struct DecryptionResultData {
    request_id: String,
}

pub struct ArbitrumGatewayL2Handler {
    _dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
    context_data: dashmap::DashMap<Uuid, DecryptionResultData>,
}

impl ArbitrumGatewayL2Handler {
    pub fn new(dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>) -> Self {
        Self {
            _dispatcher: dispatcher,
            context_data: dashmap::DashMap::new(),
        }
    }

    // fn handle_ethereum_host_l1_event_log_received(&self, event: RelayerEvent, eth_event_log: Log) {
    // }

    // fn handle_httpz_response_received(
    // }
}

impl EventHandler<RelayerEvent> for ArbitrumGatewayL2Handler {
    fn handle_event(&self, event: RelayerEvent) {
        // match event.clone().data {
        //     RelayerEventData::HostL1EventLogReceived { log: eth_event_log } => {
        //         self.handle_ethereum_host_l1_event_log_received(event, eth_event_log);
        //     }
        //     RelayerEventData::HttpzResponseReceived { decrypted_value } => {
        //         self.handle_httpz_response_received(event, decrypted_value);
        //     }
        //     _ => {
        //         return;
        //     }
        // }
    }
}
