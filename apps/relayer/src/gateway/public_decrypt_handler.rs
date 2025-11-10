use crate::{
    core::{
        errors::EventProcessingError,
        event::{
            GatewayChainEventData, PublicDecryptEventData, PublicDecryptRequest,
            PublicDecryptResponse, RelayerEvent, RelayerEventData,
        },
    },
    gateway::{
        arbitrum::{
            bindings::Decryption,
            transaction::{helper::TransactionType, ReceiptProcessor, TransactionHelper},
            ComputeCalldata,
        },
        readiness_checker::ReadinessChecker,
    },
    orchestrator::{
        traits::{EventDispatcher, EventHandler},
        Orchestrator, TokioEventDispatcher,
    },
    store::{key_value_db::KVStore, CacheResult, PublicDecryptCache},
};
use alloy::{
    network::{AnyReceiptEnvelope, AnyTransactionReceipt, ReceiptResponse},
    primitives::{Address, Bytes, FixedBytes, U256},
    rpc::types::{Log, TransactionReceipt},
};

use alloy::sol_types::SolEvent;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

struct PublicDecryptionRequestProcessor {
    handler: Arc<GatewayHandler>,
}

impl ReceiptProcessor for PublicDecryptionRequestProcessor {
    type Output = U256;

    fn process(
        &self,
        receipt: &AnyTransactionReceipt,
    ) -> Result<Self::Output, EventProcessingError> {
        self.handler
            .extract_public_decryption_id_from_receipt(receipt)
    }
}

#[derive(Clone)]
pub struct GatewayHandler {
    dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    cache: Arc<PublicDecryptCache>,
    tx_helper: Arc<TransactionHelper>,
    readiness_checker: Arc<ReadinessChecker>,
    decryption_address: Address,
}

impl GatewayHandler {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
        kv_store: Arc<dyn KVStore>,
        tx_helper: Arc<TransactionHelper>,
        readiness_checker: Arc<ReadinessChecker>,
        decryption_address: Address,
    ) -> Self {
        let cache = Arc::new(PublicDecryptCache::new(kv_store));

        Self {
            dispatcher,
            cache,
            tx_helper,
            readiness_checker,
            decryption_address,
        }
    }


    /// Pure gateway send function - only handles sending transaction to gateway
    async fn send_public_decrypt_to_gateway(
        &self,
        decrypt_request: &PublicDecryptRequest,
    ) -> Result<U256, EventProcessingError> {
        let handles_fixed_bytes: Vec<FixedBytes<32>> = decrypt_request
            .ct_handles
            .iter()
            .map(|bytes| FixedBytes::from(*bytes))
            .collect();

        info!(
            "Sending public decryption request to gateway with handles {:?}",
            handles_fixed_bytes
        );

        // Check readiness first
        self.readiness_checker
            .check_public_decryption_readiness(
                handles_fixed_bytes.clone(),
                decrypt_request.extra_data.clone(),
            )
            .await?;

        // Send transaction to gateway
        self.process_decryption_request(handles_fixed_bytes, decrypt_request.extra_data.clone())
            .await
    }

    /// Processes a successful decryption request.
    ///
    /// # Arguments
    /// * `event` - The original [`RelayerEvent`] containing request information
    /// * `decrypt_request` - The [`PublicDecryptRequest`] that was processed
    /// * `decryption_public_id` - The [`U256`] ID received from the decryption request
    ///
    /// # State Changes
    /// Stores mapping in `decryption_id_to_request_id` and caches the request
    ///
    /// # Events
    /// Dispatches [`RelayerEventData::DecryptionRequestSentToGw`]
    async fn handle_successful_public_request(
        &self,
        event: RelayerEvent,
        decrypt_request: PublicDecryptRequest,
        decryption_public_id: U256,
    ) {
        // Store the mapping between decryption ID and request ID (for multiple waiting requests)
        self.cache
            .register_duplicate(decryption_public_id, event.request_id);

        info!(
            ?event.request_id,
            ?decryption_public_id,
            "Stored mapping between decryption ID and request ID"
        );

        // Cache the request -> decryption_id mapping for future requests and notify waiters
        if let Err(e) = self
            .cache
            .store_request_mapping(&decrypt_request, decryption_public_id)
            .await
        {
            warn!(
                ?event.request_id,
                "error persisting public decrypt request to cache: {}", e
            );
        }

        // Create and dispatch the new event
        let next_event = event.derive_next_event(RelayerEventData::PublicDecrypt(
            PublicDecryptEventData::ReqSentToGw {
                gw_req_reference_id: decryption_public_id,
            },
        ));

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

        let error_event = event.derive_next_event(RelayerEventData::PublicDecrypt(
            PublicDecryptEventData::Failed {
                error: EventProcessingError::TransactionError(format!(
                    "Callback transaction failed: {error}"
                )),
            },
        ));

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
    /// Dispatches [`RelayerEventData::DecryptionResponseRcvdFromGw`]
    async fn handle_decrypt_reponse_event_log(&self, event: RelayerEvent) {
        info!(
            "Public Decryption response received. Trigger a tx to L1  {:?}",
            event.request_id,
        );

        if let RelayerEventData::GatewayChain(GatewayChainEventData::EventLogRcvd { log }) =
            &event.data
        {
            if let Some(topic) = log.topic0() {
                if *topic == Decryption::PublicDecryptionResponse::SIGNATURE_HASH {
                    match Decryption::PublicDecryptionResponse::decode_log_data(log.data()) {
                        Ok(req) => {
                            let public_decryption_id = req.decryptionId;
                            info!(?public_decryption_id, "Public decryption id from event");

                            let decrypt_response = PublicDecryptResponse {
                                gateway_request_id: public_decryption_id,
                                decrypted_value: req.decryptedResult,
                                signatures: req.signatures,
                                extra_data: req.extraData,
                            };

                            // Store response in cache for future requests
                            if let Err(e) = self
                                .cache
                                .store_response(public_decryption_id, decrypt_response.clone())
                                .await
                            {
                                warn!(
                                    ?public_decryption_id,
                                    "error persisting public decrypt response to cache: {}", e
                                );
                            }

                            if let Some(waiting_requests) =
                                self.cache.get_waiting_requests(public_decryption_id)
                            {
                                // For all requests that match this request-on-chain-id
                                for original_request_id in waiting_requests {
                                    let req_response = decrypt_response.clone();
                                    info!(
                                        ?original_request_id,
                                        ?public_decryption_id,
                                        "Found original request ID for public decryption response"
                                    );

                                    let next_event_data = RelayerEventData::PublicDecrypt(
                                        PublicDecryptEventData::RespRcvdFromGw {
                                            decrypt_response: req_response,
                                        },
                                    );

                                    // Now we can use original_request_id directly
                                    let next_event = RelayerEvent::new(
                                        original_request_id,
                                        event.api_version,
                                        next_event_data,
                                    );

                                    let _ = self.dispatcher.dispatch_event(next_event).await;
                                }

                                // Cleanup the mapping after processing all requests
                                self.cache.cleanup_mapping(&public_decryption_id);
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
            "Transaction to gateway has been done, the associated public decryption id is {}",
            id
        );
    }

    fn extract_public_decryption_id_from_receipt(
        &self,
        receipt: &AnyTransactionReceipt,
    ) -> Result<U256, EventProcessingError> {
        let target_topic = Decryption::PublicDecryptionRequest::SIGNATURE_HASH;
        info!(
            "Looking for topic: {}",
            Decryption::PublicDecryptionRequest::SIGNATURE
        );

        let receipt: TransactionReceipt<AnyReceiptEnvelope<Log>> = receipt.inner.clone();

        debug!(
            "Receipt details for public decryption:\n\
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
                    return match Decryption::PublicDecryptionRequest::decode_log_data(log.data()) {
                        Ok(event) => {
                            info!(
                                ?receipt.transaction_hash,
                                ?event.decryptionId,
                                "Found decryption ID from event"
                            );
                            Ok(event.decryptionId)
                        }
                        Err(e) => {
                            error!(?receipt.transaction_hash, ?e, "Failed to decode event data");
                            Err(EventProcessingError::DecodingError(e.to_string()))
                        }
                    };
                }
            }
        }

        Err(EventProcessingError::HandlerError(
            "Event not found in logs".into(),
        ))
    }

    async fn noop_handle_decrypt_reponse_event_log(&self, event: &RelayerEvent) {
        debug!("Gateway hanlding no-op on {event:?}");
    }

    /// Processes a decryption request by sending it to the gateway contract.
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
        handles: Vec<FixedBytes<32>>,
        extra_data: Bytes,
    ) -> Result<U256, EventProcessingError> {
        let processor = PublicDecryptionRequestProcessor {
            handler: Arc::new(self.clone()),
        };

        // Use the stored decryption address
        let decryption_address = self.decryption_address;

        self.tx_helper
            .send_raw_transaction_sync(
                TransactionType::PublicDecryptRequest,
                decryption_address,
                || ComputeCalldata::public_decryption_req(handles.clone(), extra_data.clone()),
                &processor,
            )
            .await
    }

    /// Handles the result of cache checking and takes appropriate action
    async fn handle_cache_result(
        &self,
        cache_result: CacheResult<PublicDecryptResponse>,
        event: RelayerEvent,
        decrypt_request: &PublicDecryptRequest,
    ) {
        match cache_result {
            CacheResult::Hit(decrypt_response) => {
                info!(
                    "Cache hit - dispatching cached response for request {}",
                    event.request_id
                );
                let next_event_data =
                    RelayerEventData::PublicDecrypt(PublicDecryptEventData::RespRcvdFromGw {
                        decrypt_response,
                    });
                let next_event =
                    RelayerEvent::new(event.request_id, event.api_version, next_event_data);
                if let Err(e) = self.dispatcher.dispatch_event(next_event).await {
                    error!(?e, "Failed to dispatch cached response event");
                }
            }
            CacheResult::InProgress(decryption_id) => {
                info!(
                    "Duplicate request {} - registering for existing decryption {}",
                    event.request_id, decryption_id
                );
                self.cache
                    .register_duplicate(decryption_id, event.request_id);
            }
            CacheResult::NotFound => {
                info!("New request {} - sending to gateway", event.request_id);
                match self.send_public_decrypt_to_gateway(decrypt_request).await {
                    Ok(decryption_id) => {
                        if let Err(e) = self
                            .cache
                            .store_request_mapping(decrypt_request, decryption_id)
                            .await
                        {
                            error!(
                                ?e,
                                "Failed to store request mapping for request {}", event.request_id
                            );
                            if let Err(unlock_e) = self.cache.unlock_request(decrypt_request).await
                            {
                                error!(?unlock_e, "Failed to unlock request after mapping failure");
                            }
                            self.handle_failed_request(event, e.into()).await;
                            return;
                        }
                        self.handle_successful_public_request(
                            event,
                            decrypt_request.clone(),
                            decryption_id,
                        )
                        .await;
                    }
                    Err(e) => {
                        if let Err(unlock_e) = self.cache.unlock_request(decrypt_request).await {
                            error!(
                                ?unlock_e,
                                "Failed to unlock request after gateway send failure"
                            );
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
            RelayerEventData::PublicDecrypt(PublicDecryptEventData::ReqRcvdFromUser {
                ref decrypt_request,
                ..
            }) => match self.cache.check(decrypt_request).await {
                Ok(cache_result) => {
                    self.handle_cache_result(cache_result, event.clone(), decrypt_request)
                        .await;
                }
                Err(e) => {
                    error!(?e, "Failed to check cache for request {}", event.request_id);
                    self.handle_failed_request(event.clone(), e.into()).await;
                }
            },
            RelayerEventData::GatewayChain(GatewayChainEventData::EventLogRcvd { ref log }) => {
                if let Some(topic0) = log.topic0() {
                    if FixedBytes::<32>::from_slice(topic0.as_slice())
                        != Decryption::PublicDecryptionResponse::SIGNATURE_HASH
                    {
                        debug!(
                            "Ignore this event: expected event: {:?}, received {} ",
                            log.topic0(),
                            Decryption::PublicDecryptionResponse::SIGNATURE_HASH
                        );
                        self.noop_handle_decrypt_reponse_event_log(&event).await;
                    } else {
                        self.handle_decrypt_reponse_event_log(event).await;
                    }
                };
            }
            RelayerEventData::PublicDecrypt(PublicDecryptEventData::ReqSentToGw {
                gw_req_reference_id,
            }) => {
                self.handle_decrypt_request_sent(gw_req_reference_id);
            }
            _ => {
                self.noop_handle_decrypt_reponse_event_log(&event).await;
            }
        }
    }
}
