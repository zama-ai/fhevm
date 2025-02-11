use crate::{
    errors::{EventProcessingError, TransactionServiceError},
    ethereum::{bindings::DecyptionManager, ComputeCalldata},
    orchestrator::{
        traits::{EventDispatcher, EventHandler},
        TokioEventDispatcher,
    },
    relayer_event::{DecryptedValue, RelayerEvent, RelayerEventData},
    transaction::{TransactionService, TxConfig},
    utils::{colorize_event_type, colorize_request_id},
};
use alloy::primitives::{hex, keccak256, Uint, U256};

use alloy_sol_types::SolEvent;
use async_trait::async_trait;
use std::{sync::Arc, time::Duration};
use tokio::task;
use tracing::{debug, error, info};
use uuid::Uuid;

#[derive(Clone)]
pub struct ArbitrumGatewayL2Handler {
    dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
    tx_service: Arc<TransactionService>,
    tx_config: TxConfig,
    decryption_id_to_request_id: Arc<dashmap::DashMap<U256, Uuid>>,
}

impl ArbitrumGatewayL2Handler {
    pub fn new(
        dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
        tx_service: Arc<TransactionService>,
        tx_config: TxConfig,
    ) -> Self {
        Self {
            dispatcher,
            tx_service,
            tx_config,
            decryption_id_to_request_id: Arc::new(dashmap::DashMap::new()),
        }
    }

    async fn mock_handle_decrypt_request_received(
        &self,
        event: RelayerEvent,
        handles: Vec<[u8; 32]>,
    ) {
        let handles: Vec<Uint<256, 4>> = handles
            .iter()
            .map(|bytes| {
                // big-endian is used
                Uint::from_be_bytes(*bytes)
            })
            .collect();

        info!(
            "Decryption request received. Making a tx to rollup: request_id: {:?} with handles {:?}",
            event.request_id,
            handles
        );

        let self_clone = self.clone(); // Clone self since we need to move it to the task
        let dispatcher = Arc::clone(&self.dispatcher);
        let event_clone = event.clone();

        // Spawn a blocking task to make a transaction to rollup
        // From the receipt, the decryption_public_id is extracted
        // this information is emitted through an event in DecryptionManager.sol contract
        task::spawn(async move {
            match self_clone.try_send_callback(handles).await {
                Ok(decryption_public_id) => {
                    // Store the mapping
                    self_clone
                        .decryption_id_to_request_id
                        .insert(decryption_public_id, event.request_id);

                    info!(
                        ?event.request_id,
                        ?decryption_public_id,
                        "Stored mapping between decryption ID and request ID"
                    );

                    // Create and dispatch the new event
                    let next_event = event_clone.derive_next_event(
                        RelayerEventData::DecryptionRequestSentToGwL2 {
                            decryption_public_id,
                        },
                    );

                    if let Err(e) = dispatcher.dispatch_event(next_event).await {
                        error!(?e, "Failed to dispatch DecryptRequestProcessed event");
                    }
                }
                Err(e) => {
                    error!(
                        error = ?e,
                        "Failed to send callback transaction"
                    );

                    // Emit an error event
                    let error_event =
                        event_clone.derive_next_event(RelayerEventData::DecryptionFailed {
                            error: format!("Callback transaction failed: {}", e),
                        });

                    if let Err(e) = dispatcher.dispatch_event(error_event).await {
                        error!(?e, "Failed to dispatch error event");
                    }
                }
            }
        });
    }

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

    async fn noop_handle_decrypt_reponse_event_log(&self, _event: RelayerEvent) {}

    async fn try_send_callback_inner(
        &self,
        handles: Vec<Uint<256, 4>>,
    ) -> Result<Uint<256, 4>, EventProcessingError> {
        let calldata = ComputeCalldata::decryption_req(handles)?;

        let contract_address = hex!("2Fb4341027eb1d2aD8B5D9708187df8633cAFA92").into();

        info!(
            calldata = %format!("0x{}...", hex::encode(&calldata[..20])),  // First 20 bytes with prefix
            "Submitting callback transaction"
        );

        debug!(
            full_calldata = %format!("0x{}", hex::encode(&calldata)),
            "Full callback transaction data"
        );

        let tx_hash = self
            .tx_service
            .submit_transaction(contract_address, calldata, self.tx_config.clone())
            .await
            .map_err(EventProcessingError::from)?;

        info!(?tx_hash, "Waiting for transaction confirmation");

        let receipt = self
            .tx_service
            .get_transaction_receipt(tx_hash)
            .await
            .map_err(EventProcessingError::from)?
            .ok_or_else(|| EventProcessingError::HandlerError("Receipt not found".into()))?;

        // Log receipt details
        info!(
            ?tx_hash,
            block_number = ?receipt.block_number,
            block_hash = ?receipt.block_hash,
            logs_count = receipt.inner.logs().len(),
            "Got receipt with logs"
        );

        // Find and parse the event
        let target_topic = keccak256("PublicDecryptionRequest(uint256,uint256[])");

        for log in receipt.inner.logs() {
            if let Some(first_topic) = log.topics().first() {
                if *first_topic == target_topic {
                    match DecyptionManager::PublicDecryptionRequest::decode_log_data(
                        log.data(),
                        true,
                    ) {
                        Ok(event) => {
                            let public_decryption_id = event.publicDecryptionId;
                            info!(
                                ?tx_hash,
                                ?public_decryption_id,
                                "Found decryption ID from event"
                            );
                            return Ok(public_decryption_id);
                        }
                        Err(e) => {
                            error!(?tx_hash, ?e, "Failed to decode event data");
                        }
                    }
                }
            }
        }

        Err(EventProcessingError::HandlerError(
            "Event not found in logs".into(),
        ))
    }

    async fn try_send_callback(
        &self,
        handles: Vec<Uint<256, 4>>,
    ) -> Result<Uint<256, 4>, EventProcessingError> {
        const MAX_RETRIES: u32 = 3;
        let mut attempt = 0;

        while attempt < MAX_RETRIES {
            match self.try_send_callback_inner(handles.clone()).await {
                Ok(id) => return Ok(id),
                Err(e) => {
                    if attempt < MAX_RETRIES - 1 {
                        error!(?e, attempt, "Transaction failed, retrying...");
                        tokio::time::sleep(Duration::from_secs(1 << attempt)).await;
                        attempt += 1;
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        Err(EventProcessingError::HandlerError(
            "Max retries exceeded".to_string(),
        ))
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
                self.mock_handle_decrypt_request_received(event, handles)
                    .await;
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
