use crate::{
    errors::{EventProcessingError, TransactionServiceError},
    ethereum::{bindings::DecyptionManager, ComputeCalldata},
    orchestrator::{
        traits::{EventDispatcher, EventHandler},
        TokioEventDispatcher,
    },
    relayer_event::{DecryptedValue, RelayerEvent, RelayerEventData},
    transaction::{ReceiptProcessor, TransactionHelper, TransactionService, TxConfig},
    utils::{colorize_event_type, colorize_request_id},
};

use alloy::{
    primitives::{hex, keccak256, Address, Uint, U256},
    rpc::types::TransactionReceipt,
};

use alloy_sol_types::SolEvent;
use async_trait::async_trait;
use std::{sync::Arc, time::Duration};
use tokio::task;
use tracing::{debug, error, info};
use uuid::Uuid;

const DECRYPTION_MANAGER_ADDRESS: Address = Address::new([
    0x2F, 0xb4, 0x34, 0x10, 0x27, 0xeb, 0x1d, 0x2a, 0xD8, 0xB5, 0xD9, 0x70, 0x81, 0x87, 0xdf, 0x86,
    0x33, 0xcA, 0xFA, 0x92,
]);

struct DecryptionRequestProcessor {
    handler: Arc<ArbitrumGatewayL2Handler>,
}

impl ReceiptProcessor for DecryptionRequestProcessor {
    type Output = U256;

    fn process(&self, receipt: &TransactionReceipt) -> Result<Self::Output, EventProcessingError> {
        self.handler.extract_decryption_id_from_receipt(receipt)
    }
}

#[derive(Clone)]
pub struct ArbitrumGatewayL2Handler {
    dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
    decryption_id_to_request_id: Arc<dashmap::DashMap<U256, Uuid>>,
    tx_helper: Arc<TransactionHelper>,
}

impl ArbitrumGatewayL2Handler {
    pub fn new(
        dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
        tx_service: Arc<TransactionService>,
        tx_config: TxConfig,
    ) -> Self {
        Self {
            dispatcher,
            tx_helper: Arc::new(TransactionHelper::new(tx_service, tx_config)),
            decryption_id_to_request_id: Arc::new(dashmap::DashMap::new()),
        }
    }

    /// Prepare the decryption request transaction to gateway.
    ///
    /// From the receipt, the decryption_public_id is extracted.
    ///
    /// This information is emitted through an event in DecryptionManager.sol contract
    ///
    /// This information well be used to make the link between  the decryption request transaction
    /// and the future decryption response.
    async fn send_decryption_request_to_rollup(&self, event: RelayerEvent, handles: Vec<[u8; 32]>) {
        let handles: Vec<Uint<256, 4>> = handles
            .iter()
            .map(|bytes| Uint::from_be_bytes(*bytes))
            .collect();

        info!(
            "Decryption request received. Making a tx to rollup: request_id: {:?} with handles {:?}",
            event.request_id,
            handles
        );

        let self_clone = self.clone();
        let event_clone = event.clone();

        // Spawn a blocking task to make a transaction to rollup
        task::spawn(async move {
            match self_clone.process_decryption_request(handles).await {
                Ok(decryption_public_id) => {
                    self_clone
                        .handle_successful_request(event_clone, decryption_public_id)
                        .await;
                }
                Err(e) => {
                    self_clone.handle_failed_request(event_clone, e).await;
                }
            }
        });
    }

    /// Handles a successful decryption request
    async fn handle_successful_request(&self, event: RelayerEvent, decryption_public_id: U256) {
        // Store the mapping
        self.decryption_id_to_request_id
            .insert(decryption_public_id, event.request_id);

        info!(
            ?event.request_id,
            ?decryption_public_id,
            "Stored mapping between decryption ID and request ID"
        );

        // Create and dispatch the new event
        let next_event = event.derive_next_event(RelayerEventData::DecryptionRequestSentToGwL2 {
            decryption_public_id,
        });

        if let Err(e) = self.dispatcher.dispatch_event(next_event).await {
            error!(?e, "Failed to dispatch DecryptRequestProcessed event");
        }
    }

    /// Handles a failed decryption request
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

    /// This methods extract
    fn extract_decryption_id_from_event(
        &self,
        event: &RelayerEvent,
    ) -> Result<U256, EventProcessingError> {
        if let RelayerEventData::DecryptResponseEventLogRcvdFromGwL2 { log } = &event.data {
            match DecyptionManager::PublicDecryptionRequest::decode_log_data(log.data(), true) {
                Ok(event) => {
                    let public_decryption_id = event.publicDecryptionId;
                    info!(?public_decryption_id, "Public decryption id from event");
                    return Ok(public_decryption_id);
                }
                Err(e) => {
                    error!(?e, "Failed to decode event data");
                }
            }
        }
        Err(EventProcessingError::HandlerError(
            "Failed to extract decryption ID from event".into(),
        ))
    }

    async fn process_decryption_response(&self, decryption_public_id: U256, event: RelayerEvent) {
        if let Some(entry) = self.decryption_id_to_request_id.get(&decryption_public_id) {
            let original_request_id = *entry.value();

            info!(
                ?original_request_id,
                ?decryption_public_id,
                "Found original request ID for decryption response"
            );

            let next_event_data = RelayerEventData::DecryptionResponseRcvdFromGwL2 {
                decrypted_value: DecryptedValue::PublicDecrypt {
                    plaintext: vec![1, 2, 3],        // Mock data
                    signatures: vec![vec![1, 2, 3]], // Mock signatures
                },
            };

            let next_event =
                RelayerEvent::new(original_request_id, event.api_version, next_event_data);

            if let Err(e) = self.dispatcher.dispatch_event(next_event).await {
                error!(?e, "Failed to dispatch decryption response event");
            }
        } else {
            error!(
                ?decryption_public_id,
                "No matching request ID found for decryption ID"
            );
        }
    }

    /// The decryption response event contains the plaintext and the associated signatures.
    ///
    /// In order to link this response to the original decryption request catched by L1 listener, we
    /// need to replace the future dispatched relayer event `DecryptionResponseRcvdFromGwL2` with the original
    /// request id.
    ///
    /// To achieve it, we use the received decryption_public_id to retieve the original relayer event request id
    /// using method `extract_decryption_id_from_event`
    ///
    /// Eventually, this relayer request id will be used in Ethereum handler to retrieve the contextual data which was stored
    /// initially when the L1 decryption event was catched (with the contract caller, request id and selector).
    async fn handle_decrypt_reponse_event_log(&self, event: RelayerEvent) {
        info!(
            "Decryption response received. Trigger a tx to L1  {:?}",
            event.request_id,
        );

        // Artificial sleep for this mock, normally the decryption Response is taking more time
        // Here we took the decryptionRequest Event as trigger, will change when real behavior will be implemented
        // on rollup side
        // TODO think about this getter

        tokio::time::sleep(Duration::from_secs(2)).await;

        if let Ok(decryption_public_id) = self.extract_decryption_id_from_event(&event) {
            // Use get_key_value to get both key and value, or use remove if you want to clean up
            if let Some(entry) = self.decryption_id_to_request_id.get(&decryption_public_id) {
                let original_request_id = *entry.value(); // Dereference the Ref<Uuid>

                info!(
                    ?original_request_id,
                    ?decryption_public_id,
                    "Found original request ID for decryption response"
                );

                let next_event_data = RelayerEventData::DecryptionResponseRcvdFromGwL2 {
                    decrypted_value: DecryptedValue::PublicDecrypt {
                        plaintext: vec![1, 2, 3],
                        signatures: vec![vec![1, 2, 3]],
                    },
                };

                // Now we can use original_request_id directly
                let next_event =
                    RelayerEvent::new(original_request_id, event.api_version, next_event_data);

                let _ = self.dispatcher.dispatch_event(next_event).await;
            } else {
                error!(
                    ?decryption_public_id,
                    "No matching request ID found for decryption ID"
                );
            }
        }
    }

    fn handle_decrypt_request_sent(&self, id: U256) {
        info!(
            "Transaction to rollup has been done, the associated public decryption id is {}",
            id
        );
    }

    fn extract_decryption_id_from_receipt(
        &self,
        receipt: &TransactionReceipt,
    ) -> Result<U256, EventProcessingError> {
        let target_topic = keccak256("PublicDecryptionRequest(uint256,uint256[])");

        for log in receipt.inner.logs().iter() {
            if let Some(first_topic) = log.topics().first() {
                if first_topic == &target_topic {
                    return match DecyptionManager::PublicDecryptionRequest::decode_log_data(
                        &log.data(),
                        true,
                    ) {
                        Ok(event) => {
                            info!(
                                ?receipt.transaction_hash,
                                ?event.publicDecryptionId,
                                "Found decryption ID from event"
                            );
                            Ok(event.publicDecryptionId)
                        }
                        Err(e) => {
                            error!(?receipt.transaction_hash, ?e, "Failed to decode event data");
                            Err(EventProcessingError::DecodingError(e))
                        }
                    };
                }
            }
        }

        Err(EventProcessingError::HandlerError(
            "Event not found in logs".into(),
        ))
    }

    async fn noop_handle_decrypt_reponse_event_log(&self, _event: RelayerEvent) {}

    async fn process_decryption_request(
        &self,
        handles: Vec<Uint<256, 4>>,
    ) -> Result<U256, EventProcessingError> {
        let processor = DecryptionRequestProcessor {
            handler: Arc::new(self.clone()),
        };

        self.tx_helper
            .send_transaction(
                "decryption_request",
                DECRYPTION_MANAGER_ADDRESS,
                || ComputeCalldata::decryption_req(handles.clone()),
                &processor,
            )
            .await
    }
}

#[async_trait]
impl EventHandler<RelayerEvent> for ArbitrumGatewayL2Handler {
    async fn handle_event(&self, event: RelayerEvent) {
        info!(
            event_type = %colorize_event_type(event.data.as_ref()),
            request_id = %colorize_request_id(&event.request_id),
            "Processing relayer event"
        );
        match event.data {
            RelayerEventData::DecryptRequestRcvd { ref ct_handles, .. } => {
                let handles = ct_handles.clone();
                self.send_decryption_request_to_rollup(event, handles).await;
            }
            RelayerEventData::DecryptResponseEventLogRcvdFromGwL2 { .. } => {
                self.handle_decrypt_reponse_event_log(event).await;
            }
            RelayerEventData::DecryptionRequestSentToGwL2 {
                decryption_public_id,
            } => {
                self.handle_decrypt_request_sent(decryption_public_id);
            }
            _ => {
                self.noop_handle_decrypt_reponse_event_log(event).await;
            }
        }
    }
}
