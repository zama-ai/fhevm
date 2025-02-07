use crate::{
    ethereum::bindings::DecryptionOracle,
    orchestrator::{traits::EventDispatcher, traits::EventHandler, TokioEventDispatcher},
    relayer_event::{DecryptedValue, DecryptionType, RelayerEvent, RelayerEventData},
};
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

    async fn handle_public_decrypt_event_log(&self, event: RelayerEvent, eth_event_log: Log) {
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
                    "Handling decryption event: orch request_id: {:?} block number: {:?}, ethereum_request_id: {:?}",
                    event.request_id, eth_event_log.block_number, eth_decryption_request.requestID
                );
                let mut ct_handles: Vec<[u8; 32]> = Vec::new();
                for ct_handle in eth_decryption_request.cts {
                    // TODO: Check if to_le_bytes will work.
                    ct_handles.push(ct_handle.to_le_bytes());
                }
                next_event = event.derive_next_event(RelayerEventData::DecryptRequestRcvd {
                    ct_handles,
                    operation: DecryptionType::PublicDecrypt,
                });
            }
            Err(e) => {
                next_event = event.derive_next_event(RelayerEventData::DecryptionFailed {
                    error: format!("error decoding ethereum event log data: {:?}", e),
                });
            }
        }
        _ = self.dispatcher.dispatch_event(next_event).await;
    }

    async fn handle_decrypt_response(&self, event: RelayerEvent, _decrypted_value: DecryptedValue) {
        // TODO: Send the decryped value to ethereum L1.
        match self.context_data.get(&event.request_id) {
            Some(_decrypted_request_data) => {
                info!(
                    "Handling decryption event: orch request_id: {:?}",
                    event.request_id,
                );
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
            RelayerEventData::PubDecryptEventLogRcvdFromHostL1 {
                event_log: eth_event_log,
            } => {
                self.handle_public_decrypt_event_log(event, eth_event_log)
                    .await;
            }
            RelayerEventData::DecryptionResponseRcvdFromGwL2 { decrypted_value } => {
                self.handle_decrypt_response(event, decrypted_value).await;
            }
            _ => {
                return;
            }
        }
    }
}
