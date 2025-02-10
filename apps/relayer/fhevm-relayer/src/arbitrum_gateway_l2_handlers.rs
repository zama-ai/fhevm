use crate::{
    errors::{EventProcessingError, TransactionServiceError},
    ethereum::bindings::DecyptionManager,
    orchestrator::{
        traits::{EventDispatcher, EventHandler},
        TokioEventDispatcher,
    },
    relayer_event::{DecryptedValue, RelayerEvent, RelayerEventData},
    transaction::{TransactionService, TxConfig},
};
use alloy::primitives::{hex, keccak256, Bytes, Uint, U256};
use alloy_sol_types::SolEvent;
use async_trait::async_trait;
use std::{sync::Arc, time::Duration};
use tokio::task;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Clone)]
struct DecryptionResultData {
    gateway_l2_request_id: String,
}

#[derive(Clone)]
pub struct ArbitrumGatewayL2Handler {
    dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
    context_data: dashmap::DashMap<Uuid, DecryptionResultData>,
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
            context_data: dashmap::DashMap::new(),
            tx_service,
            tx_config,
            decryption_id_to_request_id: Arc::new(dashmap::DashMap::new()),
        }
    }

    async fn mock_handle_decrypt_request_received(&self, event: RelayerEvent) {
        // TODO: make a tx to Rollup

        let handles: Vec<Uint<256, 4>> = vec![U256::from(1), U256::from(2)];

        // let next_event_data = RelayerEventData::DecryptionResponseRcvdFromGwL2 {
        //     decrypted_value: DecryptedValue::PublicDecrypt {
        //         plaintext: vec![1, 2, 3],
        //         signatures: vec![vec![1, 2, 3]],
        //     },
        // };
        info!(
            "Decryption request received. Making a tx to rollup: request_id: {:?}",
            event.request_id,
        );

        let self_clone = self.clone(); // Clone self since we need to move it to the task

        // Spawn a blocking task for the async operation
        task::spawn(async move {
            match self_clone.try_send_callback(handles).await {
                Ok(decryption_public_id) => {
                    // Store the mapping between decryption_public_id and original request_id
                    self_clone
                        .decryption_id_to_request_id
                        .insert(decryption_public_id, event.request_id);
                    info!(
                        ?event.request_id,
                        ?decryption_public_id,
                        "Stored mapping between decryption ID and request ID"
                    );
                }
                Err(e) => {
                    error!(?e, "Failed to send callback transaction");
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

    async fn noop_handle_decrypt_reponse_event_log(&self, event: RelayerEvent) {}

    async fn try_send_callback(
        &self,
        handles: Vec<Uint<256, 4>>,
    ) -> Result<Uint<256, 4>, EventProcessingError> {
        let calldata = Self::prepare_callback_data(handles)?;

        let contract_address = hex!("2Fb4341027eb1d2aD8B5D9708187df8633cAFA92").into();

        info!(
            calldata = ?hex::encode(&calldata),
            "Submitting callback transaction"
        );

        let tx_hash = self
            .tx_service
            .submit_transaction(contract_address, calldata, self.tx_config.clone())
            .await
            .map_err(EventProcessingError::from)?;

        info!(?tx_hash, "Waiting for transaction confirmation");

        // Wait for confirmation with retries
        let mut retries = 5;
        let mut receipt = None;
        while retries > 0 {
            tokio::time::sleep(Duration::from_secs(1)).await;

            match self.tx_service.get_transaction_receipt(tx_hash).await {
                Ok(Some(r)) => {
                    receipt = Some(r);
                    break;
                }
                Ok(None) => {
                    info!(?tx_hash, retries, "Receipt not yet available, retrying...");
                    retries -= 1;
                }
                Err(e) => {
                    error!(?tx_hash, ?e, "Error getting receipt");
                    return Err(EventProcessingError::from(e));
                }
            }
        }

        let receipt = receipt.ok_or_else(|| {
            error!(?tx_hash, "Failed to get receipt after retries");
            EventProcessingError::HandlerError("Transaction receipt not found after retries".into())
        })?;

        // Log receipt details
        info!(
            ?tx_hash,
            block_number = ?receipt.block_number,
            block_hash = ?receipt.block_hash,
            logs_count = receipt.inner.logs().len(),
            "Receipt details"
        );

        if receipt.inner.logs().is_empty() {
            error!(?tx_hash, "No logs found in receipt");
            return Err(EventProcessingError::HandlerError(
                "No logs in receipt".into(),
            ));
        }

        // Continue with event parsing...
        let target_topic = keccak256("PublicDecryptionRequest(uint256,uint256[])");

        // Find matching log
        for log in receipt.inner.logs() {
            if let Some(first_topic) = log.topics().first() {
                if *first_topic == target_topic {
                    match DecyptionManager::PublicDecryptionRequest::decode_log_data(
                        log.data(),
                        true,
                    ) {
                        Ok(event) => {
                            let public_decryption_id = event.publicDecryptionId;
                            info!(?tx_hash, ?public_decryption_id, "Found and decoded event");
                            return Ok(public_decryption_id);
                        }
                        Err(e) => {
                            error!(?tx_hash, ?e, "Failed to decode event data");
                        }
                    }
                }
            }
        }

        error!(?tx_hash, "Event not found in logs");
        Err(EventProcessingError::HandlerError(
            "Event not found in logs".into(),
        ))
    }

    pub(crate) fn prepare_callback_data(
        handles: Vec<Uint<256, 4>>,
    ) -> Result<Bytes, EventProcessingError> {
        let selector = &keccak256("publicDecryptionRequest(uint256[])")[..4];
        // Encode the parameters properly following ABI encoding rules
        let mut calldata = Vec::new();

        // 1. Add function selector
        calldata.extend_from_slice(selector);

        // 2. Add offset to start of array (32 bytes from start of parameters)
        calldata.extend_from_slice(&U256::from(32).to_be_bytes::<32>());

        // 3. Add array length
        calldata.extend_from_slice(&U256::from(handles.len()).to_be_bytes::<32>());

        // 4. Add array elements
        for handle in handles {
            calldata.extend_from_slice(&handle.to_be_bytes::<32>());
        }

        println!("Full calldata: 0x{}", hex::encode(&calldata));

        Ok(Bytes::from(calldata))
    }

    async fn send_callback_transaction(
        &self,
        handles: Vec<Uint<256, 4>>,
    ) -> Result<(), EventProcessingError> {
        const MAX_RETRIES: u32 = 3;
        let mut attempt = 0;

        while attempt < MAX_RETRIES {
            match self.try_send_callback(handles.clone()).await {
                Ok(_) => return Ok(()),
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
        match event.clone().data {
            RelayerEventData::DecryptRequestRcvd {
                ct_handles,
                operation,
            } => {
                self.mock_handle_decrypt_request_received(event).await;
            }
            RelayerEventData::DecryptResponseEventLogRcvdFromGwL2 { log: _ } => {
                self.handle_decrypt_reponse_event_log(event).await;
            }
            _ => {
                self.noop_handle_decrypt_reponse_event_log(event).await;
            }
        }
    }
}
