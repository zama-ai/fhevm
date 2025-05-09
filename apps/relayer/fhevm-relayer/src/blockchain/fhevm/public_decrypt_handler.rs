use crate::{
    blockchain::ethereum::{
        bindings::{Decryption::PublicDecryptionResponse, DecryptionOracle},
        ComputeCalldata,
    },
    core::{
        errors::EventProcessingError,
        event::{
            GenericEventData, PublicDecryptEventData, PublicDecryptRequest, PublicDecryptResponse,
            RelayerEvent, RelayerEventData,
        },
    },
    orchestrator::{
        traits::{EventDispatcher, EventHandler},
        TokioEventDispatcher,
    },
    transaction::{TransactionHelper, TransactionService, TxConfig},
};
use alloy::primitives::{Address, FixedBytes, Uint};
use alloy::rpc::types::Log;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::task;
use tracing::{debug, error, info};
use uuid::Uuid;

use alloy::sol_types::SolEvent;

/// Contains the context data for a decryption request from fhevm.
///
/// This data is stored when processing the initial request and is used
/// when sending the decryption response back to fhEVM.
#[derive(Debug, Clone)]
pub struct DecryptionRequestData {
    pub fhevm_request_id: Uint<256, 4>,
    pub callback_selector: FixedBytes<4>,
    pub contract_caller: Address,
}

#[derive(Clone)]
pub struct FhevmHandler {
    dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
    context_data: dashmap::DashMap<Uuid, DecryptionRequestData>,
    tx_helper: Arc<TransactionHelper>,
}

impl FhevmHandler {
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

    /// Entry point for the decryption flow. Processes Ethereum decryption request events.
    ///
    /// # Arguments
    /// * `event` - The [`RelayerEvent`] containing the request context and ID
    /// * `eth_event_log` - The raw [`Log`] containing the decryption request data from Ethereum
    ///
    /// # State Changes
    /// Stores context in [`DecryptionRequestData`] mapped to the event's request ID:
    /// - `fhevm_request_id`: Original fhevm request ID
    /// - `callback_selector`: Function selector for the callback
    /// - `contract_caller`: [`Address`] of the contract that initiated the request
    async fn handle_public_decrypt_event_log(&self, event: RelayerEvent, eth_event_log: Log) {
        info!("Handling public decrypt event log");
        let next_event: RelayerEvent = match DecryptionOracle::DecryptionRequest::decode_log_data(
            eth_event_log.data(),
            true,
        ) {
            Ok(eth_decryption_request) => {
                self.context_data.insert(
                    event.request_id,
                    DecryptionRequestData {
                        fhevm_request_id: eth_decryption_request.requestID,
                        callback_selector: eth_decryption_request.callbackSelector,
                        contract_caller: eth_decryption_request.contractCaller,
                    },
                );
                info!(
                    "Decryption event log received from listener: request_id: {:?} block number: {:?}, decryption_request_id: {:?}, selector {:?}",
                    event.request_id, eth_event_log.block_number, eth_decryption_request.requestID, eth_decryption_request.callbackSelector
                );

                let mut ct_handles: Vec<[u8; 32]> = Vec::new();
                for ct_handle in eth_decryption_request.cts {
                    ct_handles.push(ct_handle.into());
                }
                event.derive_next_event(RelayerEventData::PublicDecrypt(
                    PublicDecryptEventData::ReqRcvdFromFhevm {
                        decrypt_request: PublicDecryptRequest { ct_handles },
                    },
                ))
            }
            Err(e) => event.derive_next_event(RelayerEventData::PublicDecrypt(
                PublicDecryptEventData::Failed {
                    error: format!("error decoding ethereum event log data: {:?}", e),
                },
            )),
        };
        _ = self.dispatcher.dispatch_event(next_event).await;
    }

    /// Processes a decryption response and sends it back to the FHEVM.
    ///
    /// This function performs the following:
    /// 1. Retrieves the original request context using the request_id
    /// 2. Spawns an async task to process the response
    /// 3. Handles success/failure through respective handlers
    ///
    /// # Arguments
    /// * `event` - The [`RelayerEvent`] containing the response context
    /// * `decrypted_value` - The [`DecryptedValue`] containing the result and signatures
    ///
    /// # State Access
    /// Retrieves [`DecryptionRequestData`] using the event's request ID to get callback information
    ///
    /// # Task Behavior
    /// Spawns an async task that:
    /// - Processes the decryption response
    /// - Sends transaction to fhEVM with callback
    /// - Handles success/failure cases
    async fn send_decrypt_response_to_fhevm(
        &self,
        event: RelayerEvent,
        public_decryption_response: PublicDecryptResponse,
    ) {
        match self.context_data.get(&event.request_id) {
            Some(decrypted_request_data) => {
                info!(
                    "Decryption response received: request_id: {:?}, value: {:?}",
                    event.request_id, public_decryption_response,
                );
                // send the transaction using the request_id and callback selection from request data
                let req_clone = decrypted_request_data.clone();
                let self_clone = self.clone(); // Clone self since we need to move it to the task
                let event_clone = event.clone();

                task::spawn(async move {
                    match self_clone
                        .process_decryption_response(&req_clone, public_decryption_response)
                        .await
                    {
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
                let _next_event = event.derive_next_event(RelayerEventData::PublicDecrypt(
                    PublicDecryptEventData::Failed {
                        error: format!(
                            "fhevm response received for unknown request id: {:?}",
                            &request_id
                        ),
                    },
                ));
            }
        }
    }

    /// Handles a successful decryption request by dispatching a confirmation event.
    ///
    /// # Arguments
    /// * `event` - The [`RelayerEvent`] to derive the confirmation event from
    ///
    /// # Events
    /// Dispatches [`RelayerEventData::DecryptResponseSentToFhevm`]
    async fn handle_successful_request(&self, event: RelayerEvent) {
        // Store the mapping

        // Create and dispatch the new event
        let next_event = event.derive_next_event(RelayerEventData::PublicDecrypt(
            PublicDecryptEventData::RespSentToFhevm,
        ));

        if let Err(e) = self.dispatcher.dispatch_event(next_event).await {
            error!(?e, "Failed to dispatch DecryptRequestProcessed event");
        }
    }

    /// Handles a failed decryption request by creating and dispatching an error event.
    ///
    /// # Arguments
    /// * `event` - The [`RelayerEvent`] that failed
    /// * `error` - The [`EventProcessingError`] that caused the failure
    ///
    /// # Events
    /// Dispatches [`RelayerEventData::DecryptionFailed`]
    async fn handle_failed_request(&self, event: RelayerEvent, error: EventProcessingError) {
        error!(
            error = ?error,
            "Failed to send callback transaction"
        );

        let error_event = event.derive_next_event(RelayerEventData::PublicDecrypt(
            PublicDecryptEventData::Failed {
                error: format!("Callback transaction failed: {}", error),
            },
        ));

        if let Err(e) = self.dispatcher.dispatch_event(error_event).await {
            error!(?e, "Failed to dispatch error event");
        }
    }

    /// Processes a decryption response by sending a callback transaction to FHEVM.
    ///
    /// # Arguments
    /// * `req` - The [`DecryptionRequestData`] containing callback information and contract details
    ///
    /// # Returns
    /// * `Ok(())` - If the callback transaction was successful
    /// * `Err(`[`EventProcessingError`]`)` - If the transaction failed
    async fn process_decryption_response(
        &self,
        req: &DecryptionRequestData,
        public_decryption_response: PublicDecryptResponse,
    ) -> Result<(), EventProcessingError> {
        // let decrypted_value = U256::from(18446744073709551600u64);
        let public_decrypt_response: PublicDecryptionResponse = PublicDecryptionResponse {
            publicDecryptionId: public_decryption_response.gateway_request_id,
            decryptedResult: public_decryption_response.decrypted_value,
            signatures: public_decryption_response.signatures,
        };
        self.tx_helper
            .send_transaction_simple("decryption_response", req.contract_caller, || {
                ComputeCalldata::callback_req(req, public_decrypt_response.clone())
            })
            .await
    }

    fn handle_decrypt_response_sent(&self) {
        info!("Transaction to fhevm chain has been done");
    }
}

#[async_trait]
impl EventHandler<RelayerEvent> for FhevmHandler {
    async fn handle_event(&self, event: RelayerEvent) {
        match event.clone().data {
            RelayerEventData::Generic(GenericEventData::EventLogFromFhevm {
                log: eth_event_log,
            }) => {
                if let Some(topic0) = eth_event_log.topic0() {
                    if FixedBytes::<32>::from_slice(topic0.as_slice())
                        != DecryptionOracle::DecryptionRequest::SIGNATURE_HASH
                    {
                        debug!(
                            "Ignore this event: expected event: {:?}, received {} ",
                            eth_event_log.topic0(),
                            DecryptionOracle::DecryptionRequest::SIGNATURE_HASH
                        );
                    } else {
                        self.handle_public_decrypt_event_log(event, eth_event_log)
                            .await;
                    }
                };
            }
            RelayerEventData::PublicDecrypt(PublicDecryptEventData::RespRcvdFromGw {
                decrypt_response,
            }) => {
                self.send_decrypt_response_to_fhevm(event, decrypt_response)
                    .await;
            }
            RelayerEventData::PublicDecrypt(PublicDecryptEventData::RespSentToFhevm) => {
                self.handle_decrypt_response_sent();
            }
            _ => {
                return;
            }
        }
    }
}
