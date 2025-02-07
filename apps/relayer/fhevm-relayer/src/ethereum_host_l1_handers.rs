use crate::{
    ethereum::bindings::DecryptionOracle,
    orchestrator::{traits::EventDispatcher, traits::EventHandler, TokioEventDispatcher},
    relayer_event::{DecryptedValue, DecryptionType, RelayerEvent, RelayerEventData},
};
use alloy::primitives::{FixedBytes, Uint};
use alloy::primitives::{Address, FixedBytes, Uint};
use alloy::rpc::types::Log;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::info;
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

    async fn handle_host_l1_event_log_received(&self, event: RelayerEvent, eth_event_log: Log) {
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
                info!(
                    "Decryption event received: block number: {:?}, ethereum_request_id: {:?}",
                    eth_event_log.block_number, eth_decryption_request.requestID
                );
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

    async fn handle_httpz_response_received(
        &self,
        event: RelayerEvent,
        _decrypted_value: DecryptedValue,
    ) {
        // TODO: Send the decryped value to ethereum L1.
        match self.context_data.get(&event.request_id) {
            Some(_decrypted_request_data) => {
                // send the transaction using the request_id and callback selection from request data
            }
            None => {
                let request_id = event.clone().request_id;
                let _next_event = event.derive_next_event(RelayerEventData::DecryptionFailed {
                    error: format!(
                        "httpz response received for unknown request id: {:?}",
                        &request_id
                    ),
                });
            }
        }
    }
}

#[async_trait]
impl EventHandler<RelayerEvent> for EthereumHostL1Handler {
    async fn handle_event(&self, event: RelayerEvent) {
        match event.clone().data {
            RelayerEventData::HostL1EventLogReceived { log: eth_event_log } => {
                self.handle_host_l1_event_log_received(event, eth_event_log)
                    .await;
            }
            RelayerEventData::HttpzResponseReceived { decrypted_value } => {
                self.handle_httpz_response_received(event, decrypted_value)
                    .await;
            }
            _ => {
                return;
            }
        }
    }
}
