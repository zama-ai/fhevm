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
        key_value_db::KVStore, sql::repositories::user_decrypt_repo::UserDecryptRepository,
        CacheResult, UserDecryptCache, UserDecryptResponseStore, UserDecryptionResponseShare,
    },
};
use alloy::sol_types::SolEvent;
use alloy::{
    network::{AnyReceiptEnvelope, AnyTransactionReceipt, ReceiptResponse},
    primitives::{Address, FixedBytes, U256},
    rpc::types::{Log, TransactionReceipt},
};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

impl From<&HandleContractPair> for Decryption::CtHandleContractPair {
    fn from(pair: &HandleContractPair) -> Self {
        Self {
            ctHandle: pair.ct_handle.into(),
            contractAddress: pair.contract_address,
        }
    }
}

struct UserDecryptionRequestProcessor {}

impl ReceiptProcessor for UserDecryptionRequestProcessor {
    type Output = U256;

    fn process(
        &self,
        receipt: &AnyTransactionReceipt,
    ) -> Result<Self::Output, EventProcessingError> {
        let target_topic = UserDecryptionRequest::SIGNATURE_HASH;
        let receipt: TransactionReceipt<AnyReceiptEnvelope<Log>> = receipt.inner.clone();

        debug!(
            "Receipt details for user decryption:\n\
                 Hash: {:?}\n\
                 Status: {}\n\
                 Number of logs: {}\n\
                 Block number: {:?}\n\
                 Looking for topic: 0x{}",
            receipt.transaction_hash,
            receipt.status(),
            receipt.inner.logs().len(),
            receipt.block_number,
            hex::encode(target_topic)
        );

        for log in receipt.inner.logs().iter() {
            if let Some(first_topic) = log.topics().first() {
                if first_topic == &target_topic {
                    return match Decryption::UserDecryptionRequest::decode_log_data(log.data()) {
                        Ok(event) => Ok(event.decryptionId),
                        Err(e) => Err(EventProcessingError::DecodingError(e.to_string())),
                    };
                }
            }
        }

        Err(EventProcessingError::HandlerError(
            "UserDecryptionRequest event not found in logs".into(),
        ))
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
    user_decrypt_repo: Arc<UserDecryptRepository>,
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
        user_decrypt_repo: Arc<UserDecryptRepository>,
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
            user_decrypt_repo,
        }
    }

    // Cache operations

    async fn act_on_cache_result(
        &self,
        cache_result: CacheResult<UserDecryptResponse>,
        event: RelayerEvent,
        decrypt_request: &UserDecryptRequest,
    ) {
        match cache_result {
            CacheResult::Hit(decrypt_response) => {
                info!("Cache hit for request {}", event.request_id);
                let next_event_data =
                    RelayerEventData::UserDecrypt(UserDecryptEventData::RespRcvdFromGw {
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
                    "Duplicate request {} found for decryption {}",
                    event.request_id, decryption_id
                );
                self.cache
                    .register_duplicate(decryption_id, event.request_id);
            }
            CacheResult::NotFound => {
                info!("Sending request {} to gateway", event.request_id);
                match self.send_user_decrypt_to_gateway(decrypt_request).await {
                    Ok(user_decryption_id) => {
                        if let Err(e) = self
                            .cache
                            .store_request_mapping(decrypt_request, user_decryption_id)
                            .await
                        {
                            if (self.cache.unlock_request(decrypt_request).await).is_err() {
                                warn!("Cache unlock failed, continuing with error dispatch");
                            }
                            self.dispatch_error_event(event, e.into()).await;
                            return;
                        }
                        self.store_and_dispatch_success(event, user_decryption_id)
                            .await;
                    }
                    Err(e) => {
                        let _ = self.cache.unlock_request(decrypt_request).await;
                        self.dispatch_error_event(event, e).await;
                    }
                }
            }
        }
    }

    // Transaction operations

    async fn send_user_decrypt_to_gateway(
        &self,
        user_decrypt_request: &UserDecryptRequest,
    ) -> Result<U256, EventProcessingError> {
        self.send_transaction_to_gateway(user_decrypt_request.clone())
            .await
    }

    async fn send_transaction_to_gateway(
        &self,
        user_decrypt_request: UserDecryptRequest,
    ) -> Result<U256, EventProcessingError> {
        let processor = UserDecryptionRequestProcessor {};

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

    async fn handle_gateway_response_log(&self, event: RelayerEvent) {
        if let RelayerEventData::GatewayChain(GatewayChainEventData::EventLogRcvd { log }) =
            &event.data
        {
            if let Some(topic0) = log.topic0() {
                if *topic0 == Decryption::UserDecryptionResponse::SIGNATURE_HASH {
                    self.handle_individual_user_decrypt_response(log, &event)
                        .await;
                } else if *topic0 == self.get_consensus_event_topic() {
                    self.handle_user_decrypt_consensus_event(log, &event).await;
                }
            }
        }
    }

    async fn handle_individual_user_decrypt_response(
        &self,
        log: &alloy::rpc::types::Log,
        event: &RelayerEvent,
    ) {
        match Decryption::UserDecryptionResponse::decode_log_data(log.data()) {
            Ok(user_decrypt_response) => {
                let user_decryption_id = user_decrypt_response.decryptionId;
                info!(
                    "Gateway response received for decryption ID {}",
                    user_decryption_id
                );

                let share = UserDecryptionResponseShare {
                    decryption_id: user_decryption_id,
                    index_share: user_decrypt_response.indexShare,
                    user_decrypted_share: user_decrypt_response.userDecryptedShare,
                    signature: user_decrypt_response.signature,
                    extra_data: user_decrypt_response.extraData,
                };

                match self.user_decrypt_response_store.add_response(share) {
                    Ok(Some(final_response)) => {
                        self.process_final_user_decrypt_response(
                            user_decryption_id,
                            final_response,
                            event,
                        )
                        .await;
                    }
                    Ok(None) => {
                        debug!("Share added for decryption ID {}, waiting for more shares or consensus", user_decryption_id);
                    }
                    Err(e) => {
                        error!(
                            "Failed to add response share for decryption ID {}: {}",
                            user_decryption_id, e
                        );
                    }
                }
            }
            Err(e) => {
                error!("Failed to decode UserDecryptionResponse event data: {}", e);
            }
        }
    }

    async fn handle_user_decrypt_consensus_event(
        &self,
        log: &alloy::rpc::types::Log,
        event: &RelayerEvent,
    ) {
        if let Some(decryption_id_topic) = log.topics().get(1) {
            let user_decryption_id = U256::from_be_bytes::<32>(
                decryption_id_topic
                    .as_slice()
                    .try_into()
                    .unwrap_or([0u8; 32]),
            );

            debug!(
                "Consensus event received for decryption ID {}",
                user_decryption_id
            );

            match self
                .user_decrypt_response_store
                .mark_consensus(user_decryption_id)
            {
                Ok(Some(final_response)) => {
                    self.process_final_user_decrypt_response(
                        user_decryption_id,
                        final_response,
                        event,
                    )
                    .await;
                }
                Ok(None) => {
                    debug!(
                        "Consensus marked for decryption ID {}, waiting for more shares",
                        user_decryption_id
                    );
                }
                Err(e) => {
                    error!(
                        "Failed to mark consensus for decryption ID {}: {}",
                        user_decryption_id, e
                    );
                }
            }
        } else {
            error!("UserDecryptionResponseThresholdReached event missing decryption_id topic");
        }
    }

    async fn process_final_user_decrypt_response(
        &self,
        user_decryption_id: U256,
        final_response: UserDecryptResponse,
        event: &RelayerEvent,
    ) {
        info!(
            "Final response assembled for decryption ID {} with {} shares",
            user_decryption_id,
            final_response.reencrypted_shares.len()
        );

        if let Err(err) = self
            .cache
            .store_response(user_decryption_id, final_response.clone())
            .await
        {
            warn!(
                "Failed to store assembled response in cache for decryption ID {}: {}",
                user_decryption_id, err
            );
        }

        if let Some(waiting_requests) = self.cache.get_waiting_requests(user_decryption_id) {
            for original_request_id in waiting_requests {
                debug!("Notifying original request {}", original_request_id);

                let next_event_data =
                    RelayerEventData::UserDecrypt(UserDecryptEventData::RespRcvdFromGw {
                        decrypt_response: final_response.clone(),
                    });

                let next_event =
                    RelayerEvent::new(original_request_id, event.api_version, next_event_data);

                let _ = self.dispatcher.dispatch_event(next_event).await;
            }
        } else {
            warn!(
                "No matching request IDs found for decryption ID {}",
                user_decryption_id
            );
        }

        self.cache.cleanup_mapping(&user_decryption_id);
        self.user_decrypt_response_store.cleanup(user_decryption_id);
    }

    fn get_consensus_event_topic(&self) -> FixedBytes<32> {
        Decryption::UserDecryptionResponseThresholdReached::SIGNATURE_HASH
    }

    // Event dispatching

    async fn store_and_dispatch_success(&self, event: RelayerEvent, user_decryption_id: U256) {
        self.cache
            .register_duplicate(user_decryption_id, event.request_id);

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

    async fn dispatch_error_event(&self, event: RelayerEvent, error: EventProcessingError) {
        let error_event = event.derive_next_event(RelayerEventData::UserDecrypt(
            UserDecryptEventData::Failed { error },
        ));

        if let Err(e) = self.dispatcher.dispatch_event(error_event).await {
            error!(?e, "Failed to dispatch error event");
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
                info!("Processing user decrypt request {}", event.request_id);
                match self.cache.check(decrypt_request).await {
                    Ok(cache_result) => {
                        self.act_on_cache_result(cache_result, event.clone(), decrypt_request)
                            .await;
                    }
                    Err(e) => {
                        error!(
                            "Failed to check cache for request {}: {}",
                            event.request_id, e
                        );
                        self.dispatch_error_event(event.clone(), e.into()).await;
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
                        info!(
                            "Processing gateway response for request {}",
                            event.request_id
                        );
                        self.handle_gateway_response_log(event).await;
                    } else {
                        debug!(
                            "Ignoring event: received topic {:?}, expected individual {:?} or consensus {:?}",
                            topic0_fixed, individual_response_topic, consensus_topic
                        );
                    }
                };
            }
            RelayerEventData::UserDecrypt(UserDecryptEventData::ReqSentToGw {
                gw_req_reference_id,
            }) => {
                info!(
                    "Request {} sent to gateway with decryption ID {}",
                    event.request_id, gw_req_reference_id
                );
            }
            _ => {}
        }
    }
}
