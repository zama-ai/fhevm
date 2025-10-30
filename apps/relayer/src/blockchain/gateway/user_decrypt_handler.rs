use crate::{
    blockchain::gateway::arbitrum::transaction::{
        helper::TransactionType, ReceiptProcessor, TransactionHelper,
    },
    blockchain::gateway::arbitrum::{
        bindings::Decryption::{self, UserDecryptionRequest},
        ComputeCalldata,
    },
    config::settings::{ContractConfig, RetrySettings},
    core::{
        errors::EventProcessingError,
        event::{
            ApiVersion, GatewayChainEventData, HandleContractPair, RelayerEvent, RelayerEventData,
            UserDecryptEventData, UserDecryptRequest, UserDecryptResponse,
        },
    },
    orchestrator::{
        traits::{EventDispatcher, EventHandler},
        Orchestrator, TokioEventDispatcher,
    },
    store::{
        UserDecryptRequestCacheStore, UserDecryptResponseCacheStore, UserDecryptResponseStore,
        UserDecryptionResponseShare,
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
use reqwest::Url;
use std::str::FromStr;
use std::time::Duration;

use alloy::{
    network::{AnyReceiptEnvelope, AnyTransactionReceipt, ReceiptResponse},
    primitives::{Address, FixedBytes, U256},
    providers::ProviderBuilder,
    rpc::types::{Log, TransactionReceipt},
};
use hex;

use alloy::sol_types::SolEvent;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::task;
use tracing::{debug, error, info, instrument, warn};
use uuid::Uuid;

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
    user_decryption_responses_cache: Arc<UserDecryptResponseCacheStore>,
    user_decryption_requests_cache: Arc<UserDecryptRequestCacheStore>,
    user_decrypt_response_store: Arc<UserDecryptResponseStore>,
    user_decryption_id_to_request_id: Arc<dashmap::DashMap<U256, Vec<Uuid>>>,
    tx_helper: Arc<TransactionHelper>,
    contracts: ContractConfig,
    gateway_http_url: String,
    retry_config: RetrySettings,
}

impl GatewayHandler {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
        user_decryption_responses_cache: Arc<UserDecryptResponseCacheStore>,
        user_decryption_requests_cache: Arc<UserDecryptRequestCacheStore>,
        user_decrypt_response_store: Arc<UserDecryptResponseStore>,
        tx_helper: Arc<TransactionHelper>,
        contracts: ContractConfig,
        gateway_http_url: String,
        retry_config: RetrySettings,
    ) -> Self {
        Self {
            dispatcher,
            user_decryption_responses_cache,
            user_decryption_requests_cache,
            user_decrypt_response_store,
            user_decryption_id_to_request_id: Arc::new(dashmap::DashMap::new()),
            tx_helper,
            contracts,
            gateway_http_url,
            retry_config,
        }
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
    async fn send_user_decryption_request_to_gateway(
        &self,
        event: RelayerEvent,
        user_decrypt_request: UserDecryptRequest,
    ) {
        info!(
            "User Decryption request received. Making a tx to gateway: request_id: {:?} with user request {:?}",
            event.request_id,
            user_decrypt_request
        );

        let self_clone = self.clone();
        let event_clone = event.clone();

        // Store hash(user-decrypt-request) => request-id for the last request-id that
        // requested a decryption matching this hash
        // Spawn a blocking task to make a transaction to gateway
        task::spawn(async move {
            match self_clone
                .process_user_decryption_request(user_decrypt_request.clone())
                .await
            {
                Ok(user_decryption_id) => {
                    if let Err(err) = self_clone
                        .user_decryption_requests_cache
                        .persist_value(&user_decrypt_request, user_decryption_id)
                        .await
                    {
                        error!(
                            "Error: {err} trying to store user-decrypt request for request: {}",
                            event.request_id
                        );
                        self_clone
                            .user_decryption_requests_cache
                            .unlock(&user_decrypt_request)
                            .await;
                    }
                    // TODO: handle for all matching requests
                    self_clone
                        .handle_successful_user_decryption_request(event_clone, user_decryption_id)
                        .await;
                }
                Err(e) => {
                    self_clone
                        .user_decryption_requests_cache
                        .unlock(&user_decrypt_request)
                        .await;
                    self_clone.handle_failed_request(event_clone, e).await;
                }
            }
        });
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
        self.user_decryption_id_to_request_id
            .entry(user_decryption_id)
            .or_default()
            .push(event.request_id);

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
            .user_decryption_responses_cache
            .persist_value(user_decryption_id, final_response.clone())
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
        if let Some(entry) = self
            .user_decryption_id_to_request_id
            .get(&user_decryption_id)
        {
            for original_request_id in entry.value() {
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
                    RelayerEvent::new(*original_request_id, event.api_version, next_event_data);

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

        // Clean up the assembly store after successful processing
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

        let url = Url::parse(&self.gateway_http_url).unwrap();

        let provider = ProviderBuilder::new()
            .network::<alloy::network::AnyNetwork>()
            .connect_http(url);

        let decryption_address =
            Address::from_str(&self.contracts.decryption_address).map_err(|_| {
                EventProcessingError::ConfigError(
                    crate::config::settings::AppConfigError::InvalidAddress(
                        "contracts.decryption_address".to_owned(),
                    ),
                )
            })?;
        let decryption = Decryption::new(decryption_address, provider.clone());

        // Convert to contract related pairs
        let contract_pairs: Vec<_> = user_decrypt_request
            .ct_handle_contract_pairs
            .iter()
            .map(Decryption::CtHandleContractPair::from)
            .collect();

        // Perform readiness check with retry logic
        let max_retries = self.retry_config.max_attempts;
        let retry_interval = Duration::from_secs(self.retry_config.base_delay_secs);

        let mut retries = 0;
        let mut should_retry = true;

        info!(
            "Checking if the decryption manager is ready for user address: {:?} and contract pairs: {:?}",
            user_decrypt_request.user_address, contract_pairs
        );

        if let Some(retry_config) = &self.tx_helper.tx_config.retry_config {
            if retry_config.mock_mode {
                info!("Mock mode is enabled, skipping readiness check");
                should_retry = false;
            }
        }

        while should_retry && retries < max_retries {
            should_retry = false;

            match decryption
                .clone()
                .isUserDecryptionReady(
                    user_decrypt_request.user_address,
                    contract_pairs.clone(),
                    user_decrypt_request.extra_data.clone(),
                )
                .call()
                .await
            {
                Ok(is_ready) => {
                    if is_ready {
                        info!(
                            "Function call succeeded for user address: {:?}",
                            user_decrypt_request.user_address
                        );
                    } else {
                        info!(
                            "Gateway not ready for handles: {:?}, retrying... ",
                            user_decrypt_request.user_address
                        );
                        should_retry = true;
                    }
                }
                Err(err) => {
                    error!(
                        "Check should not revert and render boolean value: {:?}, still retrying... error: {} ",
                        user_decrypt_request.user_address, err
                    );
                    should_retry = true;
                }
            }

            if should_retry {
                retries += 1;
                if retries < max_retries {
                    info!(
                        "Retrying user decryption readiness check (attempt {}/{})",
                        retries, max_retries
                    );
                    tokio::time::sleep(retry_interval).await;
                } else {
                    warn!("Max retries reached for user decryption readiness check");
                    return Err(EventProcessingError::HandlerError(format!(
                        "Gateway not ready after {max_retries} retries"
                    )));
                }
            }
        }

        self.tx_helper
            .send_transaction(
                TransactionType::UserDecryptRequest,
                decryption_address,
                || ComputeCalldata::user_decryption_req(user_decrypt_request.clone()),
                &processor,
            )
            .await
    }

    /// Checks cache system to know if we can skip transaction making
    #[instrument(skip_all, fields(%request_id))]
    async fn user_decrypt_cache_check(
        &self,
        decrypt_request: &UserDecryptRequest,
        request_id: &Uuid,
        api_version: &ApiVersion,
    ) -> bool {
        // Check user-decrypt request cache
        // hash(request) -> decryption-id
        match self
            .user_decryption_requests_cache
            .get_value(decrypt_request)
            .await
        {
            Ok(optional_decryption_id) => {
                if let Some(decryption_id) = optional_decryption_id {
                    // Check response cache
                    // decryption-id -> response
                    match self
                        .user_decryption_responses_cache
                        .get_value(decryption_id)
                        .await
                    {
                        Ok(optional_decryption_response) => {
                            if let Some(decryption_response) = optional_decryption_response {
                                let next_event_data = RelayerEventData::UserDecrypt(
                                    UserDecryptEventData::RespRcvdFromGw {
                                        decrypt_response: UserDecryptResponse {
                                            gateway_request_id: decryption_response
                                                .gateway_request_id,
                                            reencrypted_shares: decryption_response
                                                .reencrypted_shares,
                                            signatures: decryption_response.signatures,
                                            extra_data: decrypt_request.extra_data.clone(),
                                        },
                                    },
                                );
                                info!("Dispatching UserDecryptEventData::RespRcvdFromGw event");
                                // Now we can use original_request_id directly
                                let next_event =
                                    RelayerEvent::new(*request_id, *api_version, next_event_data);
                                // We dispatch the return value
                                let _ = self.dispatcher.dispatch_event(next_event).await;
                                debug!("Returning prematurely user-decryption response was found for the request");
                                return true;
                            }
                        }
                        Err(err) => {
                            error!(
                                "Failed to access cache for user_decryption_responses_cache for request: {} with error: {}",
                                request_id, err
                            );
                        }
                    };

                    // If request was already made but there is no hit on the response cache
                    // then we add the current request-id to matching with decryption ids and
                    // just wait for response
                    // decryption-id => relayer-request-id[]
                    self.user_decryption_id_to_request_id
                        .entry(decryption_id)
                        .or_default()
                        .push(*request_id);
                    info!(
                        ?request_id,
                        ?decryption_id,
                        "Stored mapping between decryption ID and request ID"
                    );
                    debug!(
                        "Returning prematurely because user-decryption request was already made."
                    );
                    return true;
                }
            }
            Err(err) => {
                error!(
                    "Failed to access cache for user_decryption_requests_cache for request: {} with error: {}",
                    request_id, err
                );
            }
        };
        false
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
                // If the cache system tells use not to procede we just return
                // Cases where that should happen are:
                // 1. Response already present and we dispatched the response
                // 2. Request already made and we wait for the response
                if self
                    .user_decrypt_cache_check(
                        decrypt_request,
                        &event.request_id,
                        &event.api_version,
                    )
                    .await
                {
                    return;
                }

                self.send_user_decryption_request_to_gateway(
                    event.clone(),
                    decrypt_request.clone(),
                )
                .await;
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
