use crate::{
    blockchain::gateway::arbitrum::transaction::{
        helper::TransactionType, ReceiptProcessor, TransactionHelper,
    },
    blockchain::gateway::arbitrum::{bindings::Decryption, ComputeCalldata},
    config::settings::{ContractConfig, RetrySettings},
    core::{
        errors::EventProcessingError,
        event::{
            ApiVersion, GatewayChainEventData, PublicDecryptEventData, PublicDecryptRequest,
            PublicDecryptResponse, RelayerEvent, RelayerEventData,
        },
    },
    orchestrator::{
        traits::{EventDispatcher, EventHandler},
        Orchestrator, TokioEventDispatcher,
    },
    store::{PublicDecryptRequestCacheStore, PublicDecryptResponseCacheStore},
};
use std::{str::FromStr, time::Duration};

use alloy::{
    network::{AnyReceiptEnvelope, AnyTransactionReceipt, ReceiptResponse},
    primitives::{Address, Bytes, FixedBytes, U256},
    providers::ProviderBuilder,
    rpc::types::{Log, TransactionReceipt},
};

use alloy::sol_types::SolEvent;
use async_trait::async_trait;
use reqwest::Url;
use std::sync::Arc;
use tokio::task;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

#[derive(Clone)]
pub struct PublicDecryptCaches {
    pub responses: Arc<PublicDecryptResponseCacheStore>,
    pub requests: Arc<PublicDecryptRequestCacheStore>,
}

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
    caches: PublicDecryptCaches,
    public_decryption_id_to_request_id: Arc<dashmap::DashMap<U256, Vec<Uuid>>>,
    tx_helper: Arc<TransactionHelper>,
    contracts: ContractConfig,
    gateway_http_url: String,
    retry_config: RetrySettings,
}

impl GatewayHandler {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
        caches: PublicDecryptCaches,
        tx_helper: Arc<TransactionHelper>,
        contracts: ContractConfig,
        gateway_http_url: String,
        retry_config: RetrySettings,
    ) -> Self {
        Self {
            dispatcher,
            caches,
            tx_helper,
            public_decryption_id_to_request_id: Arc::new(dashmap::DashMap::new()),
            contracts,
            gateway_http_url,
            retry_config,
        }
    }

    /// Checks cache system to know if we can skip transaction making
    async fn public_decrypt_cache_check(
        &self,
        decrypt_request: &PublicDecryptRequest,
        request_id: &Uuid,
        api_version: &ApiVersion,
    ) -> bool {
        // Uses deduplication logic: get_value will block if another request is in-flight and return once the original request is sent and updated in cache.
        match self.caches.requests.get_value(decrypt_request).await {
            Ok(None) => {
                // First request for this payload. Nothing to do with regards to cache.
                return false;
            }
            Ok(optional_decryption_id) => {
                // Duplicate requests.
                // Look for response and use if found.
                // If not, queue decryption_id. Response will be routed once available.
                if let Some(decryption_id) = optional_decryption_id {
                    match self.caches.responses.get_value(decryption_id).await {
                        Ok(optional_decryption_response) => {
                            if let Some(decryption_response) = optional_decryption_response {
                                let next_event_data = RelayerEventData::PublicDecrypt(
                                    PublicDecryptEventData::RespRcvdFromGw {
                                        decrypt_response: decryption_response,
                                    },
                                );
                                let next_event =
                                    RelayerEvent::new(*request_id, *api_version, next_event_data);
                                let _ = self.dispatcher.dispatch_event(next_event).await;
                                info!(
                                    "using cached response for public decrypt with request-id = {}",
                                    request_id
                                );
                                return true;
                            }
                        }
                        Err(err) => {
                            error!(
                                "Failed to access cache for public_decryption_responses_cache for request: {} with error: {}",
                                request_id, err
                            );
                        }
                    };

                    self.public_decryption_id_to_request_id
                        .entry(decryption_id)
                        .or_default()
                        .push(*request_id);
                    info!(
                        ?request_id,
                        ?decryption_id,
                        "Stored mapping between decryption ID and request ID"
                    );
                    debug!(
                        "duplicate public decrypt request id = {}, result from original request will be used once available", request_id
                    );
                    return true;
                }
            }
            Err(err) => {
                error!(
                    "Failed to access cache for public_decryption_requests_cache for request: {} with error: {}",
                    request_id, err
                );
            }
        };
        false
    }

    /// Prepares and sends a decryption request transaction to the gateway.
    ///
    /// This function performs the following:
    /// 1. Converts the input handles to [`Uint<256, 4>`]
    /// 2. Sends transaction to the [`Decryption`] contract
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
    /// * Success: [`RelayerEventData::DecryptionRequestSentToGw`]
    /// * Failure: [`RelayerEventData::DecryptionFailed`]
    async fn send_public_decryption_request_to_gateway(
        &self,
        event: RelayerEvent,
        public_decryption_request: &PublicDecryptRequest,
    ) {
        let handles = public_decryption_request.ct_handles.clone();
        let handles_fixed_bytes: Vec<FixedBytes<32>> = handles
            .iter()
            .map(|bytes| FixedBytes::from(*bytes))
            .collect();

        info!(
            "Decryption request received. Making a tx to gateway: request_id: {:?} with handles {:?}",
            event.request_id,
            handles_fixed_bytes
        );

        let url = match Url::parse(&self.gateway_http_url) {
            Ok(url) => url,
            Err(e) => {
                let error = EventProcessingError::HandlerError(format!("Invalid URL: {e}"));
                self.handle_failed_request(event.clone(), error).await;
                return;
            }
        };

        let provider = ProviderBuilder::new()
            .network::<alloy::network::AnyNetwork>()
            .connect_http(url);

        let decryption_address = match Address::from_str(&self.contracts.decryption_address) {
            Ok(addr) => addr,
            Err(_) => {
                let error = EventProcessingError::ConfigError(
                    crate::config::settings::AppConfigError::InvalidAddress(
                        "contracts.decryption_address".to_owned(),
                    ),
                );
                self.handle_failed_request(event.clone(), error).await;
                return;
            }
        };

        let decryption = Decryption::new(decryption_address, provider.clone());

        let max_retries = self.retry_config.max_attempts;
        let retry_interval = Duration::from_secs(self.retry_config.base_delay_secs);

        let mut retries = 0;
        let mut should_retry = true;

        while should_retry && retries < max_retries {
            should_retry = false;

            match decryption
                .clone()
                .isPublicDecryptionReady(
                    handles_fixed_bytes.clone(),
                    public_decryption_request.extra_data.clone(),
                )
                .call()
                .await
            {
                Ok(is_ready) => {
                    if is_ready {
                        info!(
                            "Function call succeeded for handles: {:?}",
                            handles_fixed_bytes
                        );
                    } else {
                        info!(
                            "Gateway not ready for handles: {:?}, retrying... ",
                            handles_fixed_bytes
                        );
                        should_retry = true;
                    }
                }
                Err(err) => {
                    error!(
                        "Check should not revert and render boolean value: {:?}, still retrying... error: {} ",
                        handles_fixed_bytes, err
                    );
                    should_retry = true;
                }
            }

            if should_retry {
                retries += 1;
                if retries < max_retries {
                    info!(
                        "Retrying public decryption readiness check (attempt {}/{})",
                        retries, max_retries
                    );
                    tokio::time::sleep(retry_interval).await;
                } else {
                    warn!("Max retries reached for public decryption readiness check");

                    // Return an error instead of proceeding with the transaction
                    let error = EventProcessingError::HandlerError(format!(
                        "Gateway not ready after {max_retries} retries"
                    ));
                    self.handle_failed_request(event.clone(), error).await;
                    return;
                }
            }
        }

        let public_decryption_request_clone = public_decryption_request.clone();
        // Spawn a blocking task to make a transaction to gateway
        let self_clone = self.clone();
        task::spawn(async move {
            match self_clone
                .process_decryption_request(
                    handles_fixed_bytes,
                    public_decryption_request_clone.extra_data.clone(),
                )
                .await
            {
                Ok(decryption_public_id) => {
                    self_clone
                        .handle_successful_public_request(
                            event.clone(),
                            public_decryption_request_clone,
                            decryption_public_id,
                        )
                        .await;
                }
                Err(e) => {
                    self_clone
                        .caches
                        .requests
                        .unlock(&public_decryption_request_clone)
                        .await;
                    self_clone.handle_failed_request(event.clone(), e).await;
                }
            }
        });
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
        self.public_decryption_id_to_request_id
            .entry(decryption_public_id)
            .or_default()
            .push(event.request_id);

        info!(
            ?event.request_id,
            ?decryption_public_id,
            "Stored mapping between decryption ID and request ID"
        );

        // Cache the request -> decryption_id mapping for future requests and notify waiters
        if let Err(e) = self
            .caches
            .requests
            .persist_value(&decrypt_request, decryption_public_id)
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
                                .caches
                                .responses
                                .persist_value(public_decryption_id, decrypt_response.clone())
                                .await
                            {
                                warn!(
                                    ?public_decryption_id,
                                    "error persisting public decrypt response to cache: {}", e
                                );
                            }

                            if let Some(entry) = self
                                .public_decryption_id_to_request_id
                                .get(&public_decryption_id)
                            {
                                // For all requests that match this request-on-chain-id
                                for original_request_id in entry.value() {
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
                                        *original_request_id,
                                        event.api_version,
                                        next_event_data,
                                    );

                                    let _ = self.dispatcher.dispatch_event(next_event).await;
                                }
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

        let decryption_address =
            Address::from_str(&self.contracts.decryption_address).map_err(|_| {
                EventProcessingError::ConfigError(
                    crate::config::settings::AppConfigError::InvalidAddress(
                        "contracts.decryption_address".to_owned(),
                    ),
                )
            })?;
        self.tx_helper
            .send_raw_transaction_sync(
                TransactionType::PublicDecryptRequest,
                decryption_address,
                || ComputeCalldata::public_decryption_req(handles.clone(), extra_data.clone()),
                &processor,
            )
            .await
    }
}

#[async_trait]
impl EventHandler<RelayerEvent> for GatewayHandler {
    async fn handle_event(&self, event: RelayerEvent) {
        match event.data {
            RelayerEventData::PublicDecrypt(PublicDecryptEventData::ReqRcvdFromUser {
                ref decrypt_request,
                ..
            }) => {
                // Check cache first to see if we can skip transaction making
                if self
                    .public_decrypt_cache_check(
                        decrypt_request,
                        &event.request_id,
                        &event.api_version,
                    )
                    .await
                {
                    return;
                }

                // Clone the decrypt_request to avoid borrowing issues
                self.send_public_decryption_request_to_gateway(event.clone(), decrypt_request)
                    .await;
            }
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
