use crate::{
    errors::EventProcessingError,
    ethereum::{bindings::DecryptionOracle, ComputeCalldata},
    orchestrator::{
        traits::{EventDispatcher, EventHandler},
        TokioEventDispatcher,
    },
    relayer_event::{DecryptedValue, DecryptionType, RelayerEvent, RelayerEventData},
    transaction::{TransactionHelper, TransactionService, TxConfig},
    utils::{colorize_event_type, colorize_request_id},
};
use alloy::primitives::{Address, FixedBytes, Uint};
use alloy::rpc::types::Log;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::task;
use tracing::{error, info};
use uuid::Uuid;

use alloy::primitives::U256;
use alloy_sol_types::SolEvent;

#[derive(Debug, Clone)]
pub struct DecryptionRequestData {
    pub host_l1_request_id: Uint<256, 4>,
    pub callback_selector: FixedBytes<4>,
    pub contract_caller: Address,
}

#[derive(Clone)]
pub struct EthereumHostL1Handler {
    dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
    context_data: dashmap::DashMap<Uuid, DecryptionRequestData>,
    tx_helper: Arc<TransactionHelper>,
}

impl EthereumHostL1Handler {
    pub fn new(
        dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
        tx_service: Arc<TransactionService>,
        tx_config: TxConfig,
    ) -> Self {
        Self {
            dispatcher,
            context_data: dashmap::DashMap::new(),
            tx_helper: Arc::new(TransactionHelper::new(tx_service, tx_config)),
        }
    }

    /// Entrypoint for decryption flow.
    ///
    /// From the decryption request event, we extract:
    /// -
    async fn handle_public_decrypt_event_log(&self, event: RelayerEvent, eth_event_log: Log) {
        let next_event: RelayerEvent = match DecryptionOracle::DecryptionRequest::decode_log_data(
            eth_event_log.data(),
            true,
        ) {
            Ok(eth_decryption_request) => {
                self.context_data.insert(
                    event.request_id,
                    DecryptionRequestData {
                        host_l1_request_id: eth_decryption_request.requestID,
                        callback_selector: eth_decryption_request.callbackSelector,
                        contract_caller: eth_decryption_request.contractCaller,
                    },
                );
                info!(
                    "Decryption event log received from listener: request_id: {:?} block number: {:?}, ethereum_request_id: {:?}, selector {:?}",
                    event.request_id, eth_event_log.block_number, eth_decryption_request.requestID, eth_decryption_request.callbackSelector
                );

                let mut ct_handles: Vec<[u8; 32]> = Vec::new();
                for ct_handle in eth_decryption_request.cts {
                    // TODO: Check if to_le_bytes will work.
                    ct_handles.push(ct_handle.to_le_bytes());
                }
                event.derive_next_event(RelayerEventData::DecryptRequestRcvd {
                    ct_handles,
                    operation: DecryptionType::PublicDecrypt,
                })
            }
            Err(e) => event.derive_next_event(RelayerEventData::DecryptionFailed {
                error: format!("error decoding ethereum event log data: {:?}", e),
            }),
        };
        _ = self.dispatcher.dispatch_event(next_event).await;
    }

    async fn send_decrypt_response_to_fhevm(
        &self,
        event: RelayerEvent,
        decrypted_value: DecryptedValue,
    ) {
        match self.context_data.get(&event.request_id) {
            Some(decrypted_request_data) => {
                info!(
                    "Decryption response received: request_id: {:?}, value: {:?}",
                    event.request_id, decrypted_value,
                );
                // send the transaction using the request_id and callback selection from request data
                let req_clone = decrypted_request_data.clone();
                let self_clone = self.clone(); // Clone self since we need to move it to the task
                let event_clone = event.clone();

                task::spawn(async move {
                    match self_clone.process_decryption_response(&req_clone).await {
                        Ok(()) => {
                            self_clone.handle_successful_request(event_clone).await;
                        }
                        Err(e) => {
                            self_clone.handle_failed_request(event_clone, e).await;
                        }
                    }
                });
            }
            None => {
                let request_id = event.clone().request_id;
                info!("unknown request id: {:?}", request_id);
                let _next_event = event.derive_next_event(RelayerEventData::DecryptionFailed {
                    error: format!(
                        "httpz response received for unknown request id: {:?}",
                        &request_id
                    ),
                });
            }
        }
    }

    /// Handles a successful decryption response
    async fn handle_successful_request(&self, event: RelayerEvent) {
        // Store the mapping

        // Create and dispatch the new event
        let next_event = event.derive_next_event(RelayerEventData::DecryptResponseSentToHostL1);

        if let Err(e) = self.dispatcher.dispatch_event(next_event).await {
            error!(?e, "Failed to dispatch DecryptRequestProcessed event");
        }
    }

    async fn handle_failed_request(&self, event: RelayerEvent, error: EventProcessingError) {
        error!(
            error = ?error,
            "Failed to send callback transaction"
        );

        let error_event = event.derive_next_event(RelayerEventData::DecryptionFailed {
            error: format!("Callback transaction failed: {}", error),
        });

        if let Err(e) = self.dispatcher.dispatch_event(error_event).await {
            error!(?e, "Failed to dispatch error event");
        }
    }

    async fn process_decryption_response(
        &self,
        req: &DecryptionRequestData,
    ) -> Result<(), EventProcessingError> {
        let decrypted_value = U256::from(18446744073709551600u64);
        self.tx_helper
            .send_transaction_simple("decryption_response", req.contract_caller, || {
                ComputeCalldata::callback_req(req, decrypted_value, 4)
            })
            .await
    }

    fn handle_decrypt_response_sent(&self) {
        info!("Transaction to fhevm has been done");
    }
}

#[async_trait]
impl EventHandler<RelayerEvent> for EthereumHostL1Handler {
    async fn handle_event(&self, event: RelayerEvent) {
        info!(
            event_type = %colorize_event_type(event.data.as_ref()),
            request_id = %colorize_request_id(&event.request_id),
            "Processing relayer event"
        );
        match event.clone().data {
            RelayerEventData::PubDecryptEventLogRcvdFromHostL1 {
                event_log: eth_event_log,
            } => {
                self.handle_public_decrypt_event_log(event, eth_event_log)
                    .await;
            }
            RelayerEventData::DecryptionResponseRcvdFromGwL2 { decrypted_value } => {
                self.send_decrypt_response_to_fhevm(event, decrypted_value)
                    .await;
            }
            RelayerEventData::DecryptResponseSentToHostL1 => {
                self.handle_decrypt_response_sent();
            }
            _ => {
                return;
            }
        }
    }
}
