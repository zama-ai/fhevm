use crate::{
    blockchain::ethereum::{bindings::DecyptionManager, ComputeCalldata},
    config::settings::ContractConfig,
    core::{
        errors::EventProcessingError,
        event::{
            DecryptEventData, PublicDecryptResponse, RelayerEvent, RelayerEventData,
            UserDecryptResponse,
        },
        utils::{colorize_event_type, colorize_request_id},
    },
    orchestrator::{
        traits::{EventDispatcher, EventHandler},
        TokioEventDispatcher,
    },
    transaction::{ReceiptProcessor, TransactionHelper, TransactionService, TxConfig},
};
use std::str::FromStr;

use alloy::{
    primitives::{keccak256, Address, Bytes, Uint, U256},
    rpc::types::TransactionReceipt,
};

use alloy_sol_types::SolEvent;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::task;
use tracing::{error, info};
use uuid::Uuid;

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
    public_decryption_id_to_request_id: Arc<dashmap::DashMap<U256, Uuid>>,
    user_decryption_id_to_request_id: Arc<dashmap::DashMap<U256, Uuid>>,
    tx_helper: Arc<TransactionHelper>,
    contracts: ContractConfig,
}

impl ArbitrumGatewayL2Handler {
    pub fn new(
        dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
        tx_service: Arc<TransactionService>,
        tx_config: TxConfig,
        contracts: ContractConfig,
    ) -> Self {
        Self {
            dispatcher,
            tx_helper: Arc::new(TransactionHelper::new(tx_service, tx_config)),
            public_decryption_id_to_request_id: Arc::new(dashmap::DashMap::new()),
            user_decryption_id_to_request_id: Arc::new(dashmap::DashMap::new()),
            contracts,
        }
    }

    /// Prepares and sends a decryption request transaction to the gateway.
    ///
    /// This function performs the following:
    /// 1. Converts the input handles to [`Uint<256, 4>`]
    /// 2. Sends transaction to the [`DecyptionManager`] contract
    /// 3. Extracts the `decryption_public_id` from the receipt
    ///
    /// # Arguments
    /// * `event` - The [`RelayerEvent`] containing the request context and original request ID
    /// * `handles` - Vector of 32-byte arrays representing the encrypted handles to be decrypted
    ///
    /// # State Changes
    /// On success, stores mapping between `decryption_public_id` and the original request ID
    ///
    /// # Events
    /// * Success: [`RelayerEventData::DecryptionRequestSentToGwL2`]
    /// * Failure: [`RelayerEventData::DecryptionFailed`]
    async fn send_public_decryption_request_to_rollup(
        &self,
        event: RelayerEvent,
        handles: Vec<[u8; 32]>,
    ) {
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
                        .handle_successful_public_request(event_clone, decryption_public_id)
                        .await;
                }
                Err(e) => {
                    self_clone.handle_failed_request(event_clone, e).await;
                }
            }
        });
    }

    /// Prepares and sends a decryption request transaction to the gateway.
    ///
    /// This function performs the following:
    /// 1. Converts the input handles to [`Uint<256, 4>`]
    /// 2. Sends transaction to the [`DecyptionManager`] contract
    /// 3. Extracts the `decryption_public_id` from the receipt
    ///
    /// # Arguments
    /// * `event` - The [`RelayerEvent`] containing the request context and original request ID
    /// * `handles` - Vector of 32-byte arrays representing the encrypted handles to be decrypted
    ///
    /// # State Changes
    /// On success, stores mapping between `decryption_public_id` and the original request ID
    ///
    /// # Events
    /// * Success: [`RelayerEventData::DecryptionRequestSentToGwL2`]
    /// * Failure: [`RelayerEventData::DecryptionFailed`]
    async fn send_user_decryption_request_to_rollup(
        &self,
        event: RelayerEvent,
        handles: Vec<Bytes>,
    ) {
        let handles: Vec<Uint<256, 4>> = handles
            .iter()
            .map(|bytes| {
                let mut array = [0u8; 32];
                let bytes_slice = &bytes[..];
                array.copy_from_slice(bytes_slice);
                Uint::from_be_bytes(array)
            })
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
                Ok(decryption_user_id) => {
                    self_clone
                        .handle_successful_user_request(event_clone, decryption_user_id)
                        .await;
                }
                Err(e) => {
                    self_clone.handle_failed_request(event_clone, e).await;
                }
            }
        });
    }

    /// Processes a successful decryption request.
    ///
    /// # Arguments
    /// * `event` - The original [`RelayerEvent`] containing request information
    /// * `decryption_user_id` - The [`U256`] ID received from the decryption request
    ///
    /// # State Changes
    /// Stores mapping in `decryption_id_to_request_id`
    ///
    /// # Events
    /// Dispatches [`RelayerEventData::DecryptionRequestSentToGwL2`]
    async fn handle_successful_user_request(&self, event: RelayerEvent, decryption_user_id: U256) {
        // Store the mapping
        self.user_decryption_id_to_request_id
            .insert(decryption_user_id, event.request_id);

        info!(
            ?event.request_id,
            ?decryption_user_id,
            "Stored mapping between decryption ID and request ID"
        );

        // Create and dispatch the new event
        let next_event =
            event.derive_next_event(RelayerEventData::Decrypt(DecryptEventData::ReqSentToGwL2 {
                gateway_l2_request_id: decryption_user_id,
            }));

        if let Err(e) = self.dispatcher.dispatch_event(next_event).await {
            error!(?e, "Failed to dispatch DecryptRequestProcessed event");
        }
    }

    /// Processes a successful decryption request.
    ///
    /// # Arguments
    /// * `event` - The original [`RelayerEvent`] containing request information
    /// * `decryption_public_id` - The [`U256`] ID received from the decryption request
    ///
    /// # State Changes
    /// Stores mapping in `decryption_id_to_request_id`
    ///
    /// # Events
    /// Dispatches [`RelayerEventData::DecryptionRequestSentToGwL2`]
    async fn handle_successful_public_request(
        &self,
        event: RelayerEvent,
        decryption_public_id: U256,
    ) {
        // Store the mapping
        self.public_decryption_id_to_request_id
            .insert(decryption_public_id, event.request_id);

        info!(
            ?event.request_id,
            ?decryption_public_id,
            "Stored mapping between decryption ID and request ID"
        );

        // Create and dispatch the new event
        let next_event =
            event.derive_next_event(RelayerEventData::Decrypt(DecryptEventData::ReqSentToGwL2 {
                gateway_l2_request_id: decryption_public_id,
            }));

        if let Err(e) = self.dispatcher.dispatch_event(next_event).await {
            error!(?e, "Failed to dispatch DecryptRequestProcessed event");
        }
    }

    /// Handles a failed decryption request.
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

        let error_event =
            event.derive_next_event(RelayerEventData::Decrypt(DecryptEventData::Failed {
                error: format!("Callback transaction failed: {}", error),
            }));

        if let Err(e) = self.dispatcher.dispatch_event(error_event).await {
            error!(?e, "Failed to dispatch error event");
        }
    }

    /// Extracts decryption ID from event logs.
    ///
    /// Processes decryption response events.
    ///
    /// This function:
    /// 1. Extracts `decryption_public_id` from the event
    /// 2. Retrieves original request ID using the `decryption_public_id`
    /// 3. Creates and dispatches response event with mock data
    ///
    /// # Arguments
    /// * `event` - The [`RelayerEvent`] containing the response data
    ///
    /// # State Access
    /// Reads from `decryption_id_to_request_id` mapping
    ///
    /// # Events
    /// Dispatches [`RelayerEventData::DecryptionResponseRcvdFromGwL2`]
    async fn handle_decrypt_reponse_event_log(&self, event: RelayerEvent) {
        info!(
            "Decryption response received. Trigger a tx to L1  {:?}",
            event.request_id,
        );

        if let RelayerEventData::EventLogResponseFromGwL2 { log } = &event.data {
            match log.topic0() {
                Some(topic) => match topic {
                    &DecyptionManager::PublicDecryptionResponse::SIGNATURE_HASH => {
                        match DecyptionManager::PublicDecryptionResponse::decode_log_data(
                            log.data(),
                            true,
                        ) {
                            Ok(req) => {
                                let public_decryption_id = req.publicDecryptionId;
                                info!(?public_decryption_id, "Public decryption id from event");

                                if let Some(entry) = self
                                    .public_decryption_id_to_request_id
                                    .get(&public_decryption_id)
                                {
                                    let original_request_id = *entry.value(); // Dereference the Ref<Uuid>

                                    info!(
                                        ?original_request_id,
                                        ?public_decryption_id,
                                        "Found original request ID for decryption response"
                                    );

                                    let next_event_data = RelayerEventData::Decrypt(
                                        DecryptEventData::PublicDecryptRespFromGwL2 {
                                            decrypt_response: PublicDecryptResponse {
                                                gateway_request_id: public_decryption_id,
                                                decrypted_value: req.decryptedResult,
                                                signatures: req.signatures,
                                            },
                                        },
                                    );

                                    // Now we can use original_request_id directly
                                    let next_event = RelayerEvent::new(
                                        original_request_id,
                                        event.api_version,
                                        next_event_data,
                                    );

                                    let _ = self.dispatcher.dispatch_event(next_event).await;
                                } else {
                                    error!(
                                        ?public_decryption_id,
                                        "No matching request ID found for decryption ID"
                                    );
                                }
                            }
                            Err(e) => {
                                error!(?e, "Failed to decode event data");
                            }
                        }
                    }

                    &DecyptionManager::UserDecryptionResponse::SIGNATURE_HASH => {
                        match DecyptionManager::UserDecryptionResponse::decode_log_data(
                            log.data(),
                            true,
                        ) {
                            Ok(req) => {
                                let user_decryption_id = req.userDecryptionId;
                                info!(?user_decryption_id, "User decryption id from event");

                                if let Some(entry) = self
                                    .user_decryption_id_to_request_id
                                    .get(&user_decryption_id)
                                {
                                    let original_request_id = *entry.value(); // Dereference the Ref<Uuid>

                                    info!(
                                        ?original_request_id,
                                        ?user_decryption_id,
                                        "Found original request ID for decryption response"
                                    );

                                    let next_event_data = RelayerEventData::Decrypt(
                                        DecryptEventData::UserDecryptRespFromGwL2 {
                                            decrypt_response: UserDecryptResponse {
                                                gateway_request_id: user_decryption_id,
                                                reencrypted_shares: req.reencryptedShares,
                                                signatures: req.signatures,
                                            },
                                        },
                                    );

                                    // Now we can use original_request_id directly
                                    let next_event = RelayerEvent::new(
                                        original_request_id,
                                        event.api_version,
                                        next_event_data,
                                    );

                                    let _ = self.dispatcher.dispatch_event(next_event).await;
                                } else {
                                    error!(
                                        ?user_decryption_id,
                                        "No matching request ID found for decryption ID"
                                    );
                                }
                            }
                            Err(e) => {
                                error!(?e, "Failed to decode event data");
                            }
                        }
                    }
                    _ => {}
                },
                None => {}
            }
        }
    }

    /// Extracts the decryption ID from a transaction receipt.
    ///
    /// Searches for the [`PublicDecryptionRequest`] event in the logs and decodes it.
    ///
    /// # Arguments
    /// * `receipt` - The [`TransactionReceipt`] to process
    ///
    /// # Returns
    /// * `Ok(`[`U256`]`)` - The extracted decryption ID
    /// * `Err(`[`EventProcessingError`]`)` - If event is not found or decoding fails
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
                        log.data(),
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

    /// Processes a decryption request by sending it to the L2 contract.
    ///
    /// Uses [`TransactionHelper`] with [`DecryptionRequestProcessor`] to send
    /// and process the transaction.
    ///
    /// # Arguments
    /// * `handles` - Vector of [`Uint<256, 4>`] representing the decryption handles
    ///
    /// # Returns
    /// * `Ok(`[`U256`]`)` - The decryption ID from the transaction
    /// * `Err(`[`EventProcessingError`]`)` - If the transaction fails
    async fn process_decryption_request(
        &self,
        handles: Vec<Uint<256, 4>>,
    ) -> Result<U256, EventProcessingError> {
        let processor = DecryptionRequestProcessor {
            handler: Arc::new(self.clone()),
        };

        let decryption_manager_address =
            Address::from_str(&self.contracts.decryption_manager_address).map_err(|_| {
                EventProcessingError::ConfigError(
                    crate::config::settings::AppConfigError::InvalidAddress(
                        "contracts.decryption_manager_address".to_owned(),
                    ),
                )
            })?;
        self.tx_helper
            .send_transaction(
                "decryption_request",
                decryption_manager_address,
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
            RelayerEventData::Decrypt(DecryptEventData::PublicDecryptReq {
                ref decrypt_request,
                ..
            }) => {
                let handles = decrypt_request.ct_handles.clone();
                self.send_public_decryption_request_to_rollup(event, handles)
                    .await;
            }
            RelayerEventData::Decrypt(DecryptEventData::UserDecryptReq {
                ref decrypt_request,
                ..
            }) => {
                let handles = decrypt_request.ct_handles.clone();
                self.send_user_decryption_request_to_rollup(event, handles)
                    .await;
            }
            RelayerEventData::EventLogResponseFromGwL2 { .. } => {
                self.handle_decrypt_reponse_event_log(event).await;
            }
            RelayerEventData::Decrypt(DecryptEventData::ReqSentToGwL2 {
                gateway_l2_request_id,
            }) => {
                self.handle_decrypt_request_sent(gateway_l2_request_id);
            }
            _ => {
                self.noop_handle_decrypt_reponse_event_log(event).await;
            }
        }
    }
}
