use crate::{
    ethereum::bindings::DecryptionOracle,
    orchestrator::{traits::EventDispatcher, traits::EventHandler, TokioEventDispatcher},
    relayer_event::{DecryptionType, RelayerEvent, RelayerEventData},
};
use alloy::primitives::{FixedBytes, Uint};
use alloy::primitives::{Address, FixedBytes, Uint};
use alloy::rpc::types::Log;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use alloy_sol_types::SolEvent;

struct DecryptionRequestData {
    request_id: Uint<256, 4>,
    callback_selector: FixedBytes<4>,
    contract_Address: Address,
}

pub struct EthereumHostL1Handler {
    dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
    context_data: dashmap::DashMap<Uuid, DecryptionRequestData>,
}

impl EthereumHostL1Handler {
    pub fn new(dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>) -> Self {
        Self {
            dispatcher,
            context_data: dashmap::DashMap::new(),
        }
    }
}

#[async_trait]
impl EventHandler<RelayerEvent> for EthereumHostL1Handler {
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
            Ok(eth_decryption_request) => {
                self.context_data.insert(
                    event.request_id,
                    DecryptionRequestData {
                        request_id: eth_decryption_request.requestID,
                        callback_selector: eth_decryption_request.callbackSelector,
                        contract_Address: eth_decryption_request.contractCaller,
                    },
                );
                next_event =
                    event.derive_next_event(RelayerEventData::DecryptionRequestReceived {
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
