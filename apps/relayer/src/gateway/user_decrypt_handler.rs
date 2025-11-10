use crate::{
    core::{
        errors::EventProcessingError,
        event::{
            GatewayChainEventData, HandleContractPair, RelayerEvent, RelayerEventData,
            UserDecryptEventData, UserDecryptRequest, UserDecryptResponse,
        },
    },
    gateway::{
        arbitrum::{
            bindings::Decryption::{self, UserDecryptionRequest},
            transaction::{helper::TransactionType, ReceiptProcessor, TransactionHelper},
            ComputeCalldata,
        },
        readiness_checker::ReadinessChecker,
    },
    orchestrator::{
        traits::{EventDispatcher, EventHandler},
        Orchestrator, TokioEventDispatcher,
    },
    store::{
        key_value_db::KVStore,
        CacheResult, UserDecryptCache,
        UserDecryptResponseStore, UserDecryptionResponseShare,
    },
};

impl From<&HandleContractPair> for Decryption::CtHandleContractPair {
    fn from(pair: &HandleContractPair) -> Self {
        Self {
            ctHandle: pair.ct_handle.into(),
            contractAddress: pair.contract_address,
        }
    }
}

use alloy::{
    network::{AnyReceiptEnvelope, AnyTransactionReceipt, ReceiptResponse},
    primitives::{Address, FixedBytes, U256},
    rpc::types::{Log, TransactionReceipt},
};
use hex;

use alloy::sol_types::SolEvent;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{debug, error, info, warn};


// NOTE: I wonder if we should store the full Event instead of just the request and the response
// We could choose not to cache hit if the version of the payload doesn't match for example.

struct UserDecryptionRequestProcessor {
    handler: Arc<GatewayHandler>,
}

impl ReceiptProcessor for UserDecryptionRequestProcessor {
    type Output = U256;

    fn process(
        &self,
        receipt: &AnyTransactionReceipt,
    ) -> Result<Self::Output, EventProcessingError> {
        self.handler
            .extract_user_decryption_id_from_receipt(receipt)
    }
}

#[derive(Clone)]
pub struct GatewayHandler {
    dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    cache: Arc<UserDecryptCache>,
    user_decrypt_response_store: Arc<UserDecryptResponseStore>,
    tx_helper: Arc<TransactionHelper>,
    readiness_checker: Arc<ReadinessChecker>,
    decryption_address: Address,
    user_decrypt_shares_threshold: usize,
}

impl GatewayHandler {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
        kv_store: Arc<dyn KVStore>,
        tx_helper: Arc<TransactionHelper>,
        readiness_checker: Arc<ReadinessChecker>,
        decryption_address: Address,
        user_decrypt_shares_threshold: usize,
    ) -> Self {
        let cache = Arc::new(UserDecryptCache::new(kv_store));

        let user_decrypt_response_store = Arc::new(UserDecryptResponseStore::new(
            user_decrypt_shares_threshold as u16,
        ));

        Self {
            dispatcher,
            cache,
            user_decrypt_response_store,
            tx_helper,
            readiness_checker,
            decryption_address,
            user_decrypt_shares_threshold,
        }
    }


    /// Pure gateway send function - only handles sending transaction to gateway
    async fn send_user_decrypt_to_gateway(
        &self,
        user_decrypt_request: &UserDecryptRequest,
    ) -> Result<U256, EventProcessingError> {
        info!(
            "Sending user decryption request to gateway: {:?}",
            user_decrypt_request
        );

        // Send transaction to gateway
        self.process_user_decryption_request(user_decrypt_request.clone()).await
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
    /// Dispatches [`RelayerEventData::DecryptionRequestSentToGw`]
    async fn handle_successful_user_decryption_request(
        &self,
        event: RelayerEvent,
        user_decryption_id: U256,
    ) {
        // Store the mapping
        self.cache.register_duplicate(user_decryption_id, event.request_id);

        info!(
            ?event.request_id,
            ?user_decryption_id,
            "Stored mapping between decryption ID and request ID"
        );

        info!("User decryption request sent to gateway");

        // Create and dispatch the new event
        let next_event = event.derive_next_event(RelayerEventData::UserDecrypt(
            UserDecryptEventData::ReqSentToGw {
                gw_req_reference_id: user_decryption_id,
            },
        ));

        if let Err(e) = self.dispatcher.dispatch_event(next_event).await {
            error!(
                ?e,
                "Failed to dispatch UserDecryptEventData::ReqSentToGw event"
            );
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

        let error_event = event.derive_next_event(RelayerEventData::UserDecrypt(
            UserDecryptEventData::Failed {
                error: EventProcessingError::HandlerError(format!(
                    "Callback transaction failed: {error}"
                )),
            },
        ));

        if let Err(e) = self.dispatcher.dispatch_event(error_event).await {
            error!(?e, "Failed to dispatch error event");
        }
    }

    /// Handles user decryption response events from the gateway.
    ///
    /// Processes two types of events:
    /// 1. Individual UserDecryptionResponse events (one per share)
    /// 2. UserDecryptionResponseThresholdReached consensus events
    ///
    /// This function assembles individual shares into the final response format
    /// and maintains backward compatibility with existing cache and event flow.
    ///
    /// # Arguments
    /// * `event` - The [`RelayerEvent`] containing the response data
    ///
    /// # State Access
    /// - Reads/writes to `user_decrypt_response_store` for assembly
    /// - Reads from `user_decryption_id_to_request_id` mapping
    /// - Writes to `user_decryption_responses_cache` for final responses
    ///
    /// # Events
    /// Dispatches [`RelayerEventData::UserDecrypt::RespRcvdFromGw`] when ready
    async fn handle_user_decrypt_response_event_log(&self, event: RelayerEvent) {
        info!("User Decryption response received: {:?}", event.request_id,);

        if let RelayerEventData::GatewayChain(GatewayChainEventData::EventLogRcvd { log }) =
            &event.data
        {
            if let Some(topic0) = log.topic0() {
                // Handle individual UserDecryptionResponse events
                if *topic0 == Decryption::UserDecryptionResponse::SIGNATURE_HASH {
                    self.handle_individual_user_decrypt_response(log, &event)
                        .await;
                }
                // Handle consensus UserDecryptionResponseThresholdReached events
                else if *topic0 == self.get_consensus_event_topic() {
                    self.handle_user_decrypt_consensus_event(log, &event).await;
                }
            }
        }
    }

    /// Handles individual UserDecryptionResponse events
    async fn handle_individual_user_decrypt_response(
        &self,
        log: &alloy::rpc::types::Log,
        event: &RelayerEvent,
    ) {
        match Decryption::UserDecryptionResponse::decode_log_data(log.data()) {
            Ok(user_decrypt_response) => {
                let user_decryption_id = user_decrypt_response.decryptionId;
                info!(
                    decryption_id = %user_decryption_id,
                    index_share = %user_decrypt_response.indexShare,
                    "Received individual user decrypt response share"
                );

                // Convert to our internal share structure
                let share = UserDecryptionResponseShare {
                    decryption_id: user_decryption_id,
                    index_share: user_decrypt_response.indexShare,
                    user_decrypted_share: user_decrypt_response.userDecryptedShare,
                    signature: user_decrypt_response.signature,
                    extra_data: user_decrypt_response.extraData,
                };

                // Add to assembly store
                match self.user_decrypt_response_store.add_response(share) {
                    Ok(Some(final_response)) => {
                        // Assembly complete - process final response
                        self.process_final_user_decrypt_response(
                            user_decryption_id,
                            final_response,
                            event,
                        )
                        .await;
                    }
                    Ok(None) => {
                        debug!(
                            decryption_id = %user_decryption_id,
                            "Share added, waiting for more shares or consensus"
                        );
                    }
                    Err(e) => {
                        error!(
                            decryption_id = %user_decryption_id,
                            error = ?e,
                            "Failed to add individual response share"
                        );
                    }
                }
            }
            Err(e) => {
                error!(
                    request_id = %event.request_id,
                    error = ?e,
                    "Failed to decode individual UserDecryptionResponse event data"
                );
            }
        }
    }

    /// Handles UserDecryptionResponseThresholdReached consensus events
    async fn handle_user_decrypt_consensus_event(
        &self,
        log: &alloy::rpc::types::Log,
        event: &RelayerEvent,
    ) {
        // Extract decryption_id from the first indexed topic (after the event signature)
        if let Some(decryption_id_topic) = log.topics().get(1) {
            let user_decryption_id = U256::from_be_bytes::<32>(
                decryption_id_topic
                    .as_slice()
                    .try_into()
                    .unwrap_or([0u8; 32]),
            );

            info!(
                decryption_id = %user_decryption_id,
                "Received user decrypt consensus event"
            );

            // Mark consensus reached
            match self
                .user_decrypt_response_store
                .mark_consensus(user_decryption_id)
            {
                Ok(Some(final_response)) => {
                    // Assembly complete - process final response
                    self.process_final_user_decrypt_response(
                        user_decryption_id,
                        final_response,
                        event,
                    )
                    .await;
                }
                Ok(None) => {
                    debug!(
                        decryption_id = %user_decryption_id,
                        "Consensus marked, waiting for more shares"
                    );
                }
                Err(e) => {
                    error!(
                        decryption_id = %user_decryption_id,
                        error = ?e,
                        "Failed to mark consensus for user decrypt response"
                    );
                }
            }
        } else {
            error!(
                request_id = %event.request_id,
                "UserDecryptionResponseThresholdReached event missing decryption_id topic"
            );
        }
    }

    /// Processes the final assembled user decrypt response
    async fn process_final_user_decrypt_response(
        &self,
        user_decryption_id: U256,
        final_response: UserDecryptResponse,
        event: &RelayerEvent,
    ) {
        info!(
            decryption_id = %user_decryption_id,
            shares_count = final_response.reencrypted_shares.len(),
            "Processing final assembled user decrypt response"
        );

        // Store in cache (maintain existing cache behavior)
        if let Err(err) = self
            .cache
            .store_response(user_decryption_id, final_response.clone())
            .await
        {
            error!(
                decryption_id = %user_decryption_id,
                error = ?err,
                "Failed to store assembled user decrypt response in cache"
            );
        } else {
            debug!(
                decryption_id = %user_decryption_id,
                "Successfully stored assembled user decrypt response in cache"
            );
        }

        // Dispatch events for all matching request IDs
        if let Some(waiting_requests) = self
            .cache
            .get_waiting_requests(user_decryption_id)
        {
            for original_request_id in waiting_requests {
                info!(
                    original_request_id = %original_request_id,
                    decryption_id = %user_decryption_id,
                    "Dispatching assembled response to original request"
                );

                let next_event_data =
                    RelayerEventData::UserDecrypt(UserDecryptEventData::RespRcvdFromGw {
                        decrypt_response: final_response.clone(),
                    });

                let next_event =
                    RelayerEvent::new(original_request_id, event.api_version, next_event_data);

                if let Err(e) = self.dispatcher.dispatch_event(next_event).await {
                    error!(
                        original_request_id = %original_request_id,
                        error = ?e,
                        "Failed to dispatch assembled user decrypt response event"
                    );
                }
            }
        } else {
            warn!(
                decryption_id = %user_decryption_id,
                "No matching request IDs found for assembled user decrypt response"
            );
        }

        // Clean up the cache mapping and assembly store after successful processing
        self.cache.cleanup_mapping(&user_decryption_id);
        self.user_decrypt_response_store.cleanup(user_decryption_id);
    }

    fn get_consensus_event_topic(&self) -> FixedBytes<32> {
        Decryption::UserDecryptionResponseThresholdReached::SIGNATURE_HASH
    }

    fn handle_user_decrypt_request_sent(&self, id: U256) {
        info!(
            "Transaction to gateway has been done, the associated user decryption id is {}",
            id
        );
    }

    fn extract_user_decryption_id_from_receipt(
        &self,
        receipt: &AnyTransactionReceipt,
    ) -> Result<U256, EventProcessingError> {
        // Get the event signature for UserDecryptionRequest with the correct parameters
        let target_topic = UserDecryptionRequest::SIGNATURE_HASH;

        info!("Looking for topic: {}", UserDecryptionRequest::SIGNATURE);

        let receipt: TransactionReceipt<AnyReceiptEnvelope<Log>> = receipt.inner.clone();
        debug!(
            "Receipt details for user decryption:\n\
             Hash: {:?}\n\
             Status: {}\n\
             Gas used: {:?}\n\
             Number of logs: {}\n\
             Block number: {:?}",
            receipt.transaction_hash,
            receipt.status(),
            receipt.gas_used,
            receipt.inner.logs().len(),
            receipt.block_number
        );

        info!("Looking for topic: 0x{}", hex::encode(target_topic));

        for log in receipt.inner.logs().iter() {
            if let Some(first_topic) = log.topics().first() {
                if first_topic == &target_topic {
                    return match Decryption::UserDecryptionRequest::decode_log_data(log.data()) {
                        Ok(event) => {
                            info!(
                                ?receipt.transaction_hash,
                                ?event.decryptionId,
                                "Found user decryption ID from event"
                            );
                            Ok(event.decryptionId)
                        }
                        Err(e) => {
                            error!(?receipt.transaction_hash, ?e, "Failed to decode user decryption event data");
                            Err(EventProcessingError::DecodingError(e.to_string()))
                        }
                    };
                }
            }
        }

        error!(
            ?receipt.transaction_hash,
            "UserDecryptionRequest event not found in transaction logs"
        );

        Err(EventProcessingError::HandlerError(
            "UserDecryptionRequest event not found in logs".into(),
        ))
    }

    async fn noop_handle_decrypt_response_event_log(&self, _event: &RelayerEvent) {}

    async fn process_user_decryption_request(
        &self,
        user_decrypt_request: UserDecryptRequest,
    ) -> Result<U256, EventProcessingError> {
        let processor = UserDecryptionRequestProcessor {
            handler: Arc::new(self.clone()),
        };

        // Convert to contract related pairs
        let contract_pairs: Vec<_> = user_decrypt_request
            .ct_handle_contract_pairs
            .iter()
            .map(Decryption::CtHandleContractPair::from)
            .collect();

        self.readiness_checker
            .check_user_decryption_readiness(
                user_decrypt_request.user_address,
                contract_pairs,
                user_decrypt_request.extra_data.clone(),
            )
            .await?;

        // Use the stored decryption address
        let decryption_address = self.decryption_address;

        self.tx_helper
            .send_raw_transaction_sync(
                TransactionType::UserDecryptRequest,
                decryption_address,
                || ComputeCalldata::user_decryption_req(user_decrypt_request.clone()),
                &processor,
            )
            .await
    }


    /// Handles the result of cache checking and takes appropriate action
    async fn handle_cache_result(
        &self,
        cache_result: CacheResult<UserDecryptResponse>,
        event: RelayerEvent,
        decrypt_request: &UserDecryptRequest,
    ) {
        match cache_result {
            CacheResult::Hit(decrypt_response) => {
                info!("Cache hit - dispatching cached response for request {}", event.request_id);
                let next_event_data = RelayerEventData::UserDecrypt(
                    UserDecryptEventData::RespRcvdFromGw {
                        decrypt_response,
                    },
                );
                let next_event = RelayerEvent::new(event.request_id, event.api_version, next_event_data);
                if let Err(e) = self.dispatcher.dispatch_event(next_event).await {
                    error!(?e, "Failed to dispatch cached response event");
                }
            }
            CacheResult::InProgress(decryption_id) => {
                info!(
                    "Duplicate request {} - registering for existing decryption {}",
                    event.request_id, decryption_id
                );
                self.cache.register_duplicate(decryption_id, event.request_id);
            }
            CacheResult::NotFound => {
                info!("New request {} - sending to gateway", event.request_id);
                match self.send_user_decrypt_to_gateway(decrypt_request).await {
                    Ok(user_decryption_id) => {
                        if let Err(e) = self.cache.store_request_mapping(decrypt_request, user_decryption_id).await {
                            error!(?e, "Failed to store request mapping for request {}", event.request_id);
                            if let Err(unlock_e) = self.cache.unlock_request(decrypt_request).await {
                                error!(?unlock_e, "Failed to unlock request after mapping failure");
                            }
                            self.handle_failed_request(event, e.into()).await;
                            return;
                        }
                        self.handle_successful_user_decryption_request(event, user_decryption_id).await;
                    }
                    Err(e) => {
                        if let Err(unlock_e) = self.cache.unlock_request(decrypt_request).await {
                            error!(?unlock_e, "Failed to unlock request after gateway send failure");
                        }
                        self.handle_failed_request(event, e).await;
                    }
                }
            }
        }
    }
}

#[async_trait]
impl EventHandler<RelayerEvent> for GatewayHandler {
    async fn handle_event(&self, event: RelayerEvent) {
        match event.data {
            RelayerEventData::UserDecrypt(UserDecryptEventData::ReqRcvdFromUser {
                ref decrypt_request,
                ..
            }) => {
                match self.cache.check(decrypt_request).await {
                    Ok(cache_result) => {
                        self.handle_cache_result(cache_result, event.clone(), decrypt_request).await;
                    }
                    Err(e) => {
                        error!(?e, "Failed to check cache for request {}", event.request_id);
                        self.handle_failed_request(event.clone(), e.into()).await;
                    }
                }
            }
            RelayerEventData::GatewayChain(GatewayChainEventData::EventLogRcvd { ref log }) => {
                if let Some(topic0) = log.topic0() {
                    let topic0_fixed = FixedBytes::<32>::from_slice(topic0.as_slice());
                    let individual_response_topic =
                        Decryption::UserDecryptionResponse::SIGNATURE_HASH;
                    let consensus_topic = self.get_consensus_event_topic();

                    if topic0_fixed == individual_response_topic || topic0_fixed == consensus_topic
                    {
                        self.handle_user_decrypt_response_event_log(event).await;
                    } else {
                        debug!(
                            "Ignoring event: received topic {:?}, expected individual {:?} or consensus {:?}",
                            topic0_fixed,
                            individual_response_topic,
                            consensus_topic
                        );
                        self.noop_handle_decrypt_response_event_log(&event).await;
                    }
                };
            }
            RelayerEventData::UserDecrypt(UserDecryptEventData::ReqSentToGw {
                gw_req_reference_id,
            }) => {
                self.handle_user_decrypt_request_sent(gw_req_reference_id);
            }
            _ => {
                self.noop_handle_decrypt_response_event_log(&event).await;
            }
        }
    }
}
