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
        traits::{Event, EventDispatcher, EventHandler},
        ContentHasher, Orchestrator, TokioEventDispatcher,
    },
    store::{
        key_value_db::KVStore,
        sql::{
            models::{
                req_status_enum_model::ReqStatus, user_decrypt_share_model::UserDecryptShare,
            },
            repositories::user_decrypt_repo::UserDecryptRepository,
        },
        CacheResult, UserDecryptCache, UserDecryptResponseStore, UserDecryptionResponseShare,
    },
};
use alloy::sol_types::SolEvent;
use alloy::{
    network::{AnyReceiptEnvelope, AnyTransactionReceipt, ReceiptResponse},
    primitives::{Address, Bytes, FixedBytes, U256},
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
    user_decrypt_shares_threshold: i64,
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
            user_decrypt_shares_threshold: user_decrypt_shares_threshold as i64,
        }
    }

    // Cache state handlers - focused single-responsibility functions

    async fn dispatch_cached_response(
        &self,
        event: RelayerEvent,
        decrypt_response: UserDecryptResponse,
    ) {
        info!("Cache hit for request {}", event.job_id);
        let next_event_data = RelayerEventData::UserDecrypt(UserDecryptEventData::RespRcvdFromGw {
            decrypt_response,
        });
        let next_event = RelayerEvent::new(event.job_id(), event.api_version, next_event_data);
        if let Err(e) = self.dispatcher.dispatch_event(next_event).await {
            error!(?e, "Failed to dispatch cached response event");
        }
    }

    async fn handle_new_request(
        &self,
        event: RelayerEvent,
        decrypt_request: &UserDecryptRequest,
        indexer_id: [u8; 32],
    ) {
        info!("Sending request {} to gateway", event.job_id);

        // SQL operations using indexer_id
        if let Err(e) = self
            .user_decrypt_repo
            .update_status_to_processing(&indexer_id[..])
            .await
        {
            // ALWAYS log immediately with full context (guaranteed)
            error!(
                job_id = %event.job_id,
                indexer_id = %hex::encode(indexer_id),
                sql_operation = "user_decrypt.update_status_to_processing",
                sql_error = %e,
                "SQL operation failed"
            );

            // Forward simple message to HTTP handler for 500
            self.dispatch_error_event(
                event,
                EventProcessingError::HandlerError("Failed SQL operation".to_string()),
            )
            .await;
            return;
        }

        match self.send_user_decrypt_to_gateway(decrypt_request).await {
            Ok(user_decryption_id) => {
                if let Err(e) = self
                    .cache
                    .store_request_mapping(decrypt_request, user_decryption_id, event.job_id())
                    .await
                {
                    if (self.cache.unlock_request(decrypt_request).await).is_err() {
                        warn!("Cache unlock failed, continuing with error dispatch");
                    }
                    self.dispatch_error_event(event, e.into()).await;
                    return;
                }
                self.store_and_dispatch_success(event, decrypt_request.clone(), user_decryption_id)
                    .await;
            }
            Err(e) => {
                let _ = self.cache.unlock_request(decrypt_request).await;

                // Update database status to failure for transaction errors
                let indexer_id = decrypt_request.content_hash();
                let err_reason = format!("Transaction Failed: {}", e);
                if let Err(sql_error) = self
                    .user_decrypt_repo
                    .update_status_to_failure_on_tx_failed(&indexer_id[..], &err_reason)
                    .await
                {
                    error!(
                        job_id = %event.job_id,
                        indexer_id = %hex::encode(indexer_id),
                        sql_error = %sql_error,
                        "Failed to update transaction failure status in database"
                    );
                }

                self.dispatch_error_event(event, e).await;
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
                    self.handle_individual_user_decrypt_response(log, event.clone())
                        .await;
                } else if *topic0 == self.get_consensus_event_topic() {
                    self.handle_user_decrypt_consensus_event(log, event.clone())
                        .await;
                }
            }
        }
    }

    async fn handle_individual_user_decrypt_response(
        &self,
        log: &alloy::rpc::types::Log,
        event: RelayerEvent,
    ) {
        match Decryption::UserDecryptionResponse::decode_log_data(log.data()) {
            Ok(user_decrypt_response) => {
                let user_decryption_id = user_decrypt_response.decryptionId;
                info!(
                    "Gateway response received for decryption ID {}",
                    user_decryption_id
                );

                let gw_reference_id = match super::utils::u256_to_i64(user_decryption_id) {
                    Ok(id) => id,
                    Err(e) => {
                        error!(
                            job_id = %event.job_id,
                            decryption_id = %user_decryption_id,
                            conversion_error = %e,
                            "Failed to convert U256 decryption ID to i64"
                        );
                        //TODO(Mano): Better stratgfy to handle theg errro and confirmn with garteway team.
                        self.dispatch_error_event(
                            event,
                            EventProcessingError::HandlerError(
                                "Decryption ID too large".to_string(),
                            ),
                        )
                        .await;
                        return;
                    }
                };

                let index_share = match super::utils::u256_to_i32(user_decrypt_response.indexShare)
                {
                    Ok(id) => id,
                    Err(e) => {
                        error!(
                            job_id = %event.job_id,
                            decryption_id = %user_decryption_id,
                            conversion_error = %e,
                            "Failed to convert U256 index of share to i32"
                        );
                        //TODO(Mano): Better stratgfy to handle theg errro and confirmn with garteway team.
                        self.dispatch_error_event(
                            event,
                            EventProcessingError::HandlerError(
                                "Index of share too large".to_string(),
                            ),
                        )
                        .await;
                        return;
                    }
                };

                match self
                    .user_decrypt_repo
                    .insert_share_and_return_count(
                        gw_reference_id,
                        index_share,
                        &hex::encode(&user_decrypt_response.userDecryptedShare),
                        &hex::encode(&user_decrypt_response.signature),
                        Some(&hex::encode(&user_decrypt_response.extraData)),
                    )
                    .await
                {
                    Ok(count) => {
                        // even before fetching count, detect timeoout and send to user.
                        if count == self.user_decrypt_shares_threshold {
                            match self
                                .user_decrypt_repo
                                .complete_req_and_get_shares_metadata(gw_reference_id)
                                .await
                            {
                                Ok((consensus_state, shares)) => {
                                    match consensus_state.req_status {
                                        ReqStatus::Completed => {
                                            // build response and send
                                            if shares.len() != (count as usize)
                                                || shares.len()
                                                    != self.user_decrypt_shares_threshold as usize
                                            {
                                                error!(
                                                    job_id = %event.job_id,
                                                    got_count = %count,
                                                    expected_count = %shares.len(),
                                                    threshold = %self.user_decrypt_shares_threshold,
                                                    "Number of shares not matching count"
                                                );
                                                for share in shares {
                                                    println!(
                                                        "Share: {:?} {:?}",
                                                        share.gw_reference_id, share.id
                                                    );
                                                }
                                                self.dispatch_error_event(
                                                    event.clone(),
                                                    EventProcessingError::HandlerError(
                                                        "Number of shares not matching count"
                                                            .to_string(),
                                                    ),
                                                )
                                                .await;
                                            } else {
                                                // Success case: assemble final response and send to user
                                                match assemble_final_response(shares) {
                                                    Ok(final_response) => {
                                                        // Dispatch response event to notify waiting HTTP handlers
                                                        // We need to find the original JobId for this decryption_id
                                                        if let Some(original_job_id) =
                                                            self.cache.get_job_id_for_decryption_id(
                                                                user_decryption_id,
                                                            )
                                                        {
                                                            let response_event_data =
                                                                RelayerEventData::UserDecrypt(
                                                                    UserDecryptEventData::RespRcvdFromGw {
                                                                        decrypt_response: final_response
                                                                            .clone(),
                                                                    },
                                                                );

                                                            let response_event = RelayerEvent::new(
                                                                original_job_id,
                                                                event.api_version,
                                                                response_event_data,
                                                            );

                                                            if let Err(e) = self
                                                                .dispatcher
                                                                .dispatch_event(response_event)
                                                                .await
                                                            {
                                                                error!(?e, "Failed to dispatch response event to HTTP handlers");
                                                            }
                                                        } else {
                                                            error!(
                                                                "No JobId found for decryption_id={}",
                                                                user_decryption_id
                                                            );
                                                        }
                                                    }
                                                    Err(hex_error) => {
                                                        error!(
                                                            job_id = %event.job_id,
                                                            hex_error = %hex_error,
                                                            "Failed to decode hex data in shares"
                                                        );
                                                        self.dispatch_error_event(
                                                            event,
                                                            EventProcessingError::HandlerError(
                                                                format!("Failed to decode share data: {}", hex_error),
                                                            ),
                                                        )
                                                        .await;
                                                        return;
                                                    }
                                                }
                                            }
                                        }
                                        ReqStatus::TimedOut => {
                                            // build error object and send
                                            error!(
                                                job_id = %event.job_id,
                                                "User decrypt request timed out (response timed out)"
                                            );
                                            self.dispatch_error_event(
                                                event.clone(),
                                                EventProcessingError::HandlerError(
                                                    "User decrypt request timed out (response timed out)".to_string(),
                                                ),
                                            )
                                            .await;
                                            // unexpected state, trigger internal server error
                                        }
                                        _ => {
                                            error!(
                                                job_id = %event.job_id,
                                                status = ?consensus_state.req_status,
                                                "Unexpected state of requests"
                                            );
                                            self.dispatch_error_event(
                                                event.clone(),
                                                EventProcessingError::HandlerError(
                                                    "Unexpected state of requests".to_string(),
                                                ),
                                            )
                                            .await;
                                            // unexpected state, trigger internal server error
                                        }
                                    }
                                }
                                Err(sql_error) => {
                                    error!(
                                        job_id = %event.job_id,
                                        sql_operation = "user_decrypt.complete_req_and_get_shares_metadata",
                                        sql_error = %sql_error,
                                        "SQL operation failed"
                                    );
                                    self.dispatch_error_event(
                                        event,
                                        EventProcessingError::HandlerError(
                                            "Cannot fetch user decrypt shares after completion"
                                                .to_string(),
                                        ),
                                    )
                                    .await;
                                    return;
                                }
                            }
                        }
                        // Else pass
                    }
                    Err(sql_error) => {
                        error!(
                            job_id = %event.job_id,
                            sql_operation = "user_decrypt.insert_share_and_return_count",
                            sql_error = %sql_error,
                            "SQL operation failed"
                        );
                        self.dispatch_error_event(
                            event,
                            EventProcessingError::HandlerError(
                                "Cannot insert user decrypt share".to_string(),
                            ),
                        )
                        .await;
                        return;
                    }
                }

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
        event: RelayerEvent,
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
        event: RelayerEvent,
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

        // Dispatch response event to notify waiting HTTP handlers
        // We need to find the original JobId for this decryption_id
        if let Some(original_job_id) = self.cache.get_job_id_for_decryption_id(user_decryption_id) {
            let response_event_data =
                RelayerEventData::UserDecrypt(UserDecryptEventData::RespRcvdFromGw {
                    decrypt_response: final_response.clone(),
                });

            let response_event =
                RelayerEvent::new(original_job_id, event.api_version, response_event_data);

            if let Err(e) = self.dispatcher.dispatch_event(response_event).await {
                error!(?e, "Failed to dispatch response event to HTTP handlers");
            }
        } else {
            error!("No JobId found for decryption_id={}", user_decryption_id);
        }

        self.cache.cleanup_mapping(&user_decryption_id);
        self.user_decrypt_response_store.cleanup(user_decryption_id);
    }

    fn get_consensus_event_topic(&self) -> FixedBytes<32> {
        Decryption::UserDecryptionResponseThresholdReached::SIGNATURE_HASH
    }

    // Event dispatching

    async fn store_and_dispatch_success(
        &self,
        event: RelayerEvent,
        decrypt_request: UserDecryptRequest,
        user_decryption_id: U256,
    ) {
        // No need to register duplicate - orchestrator handles distribution to all
        // HTTP handlers subscribed to the same content-based JobId

        // Convert U256 to i64 for SQL operation (BIGINT)
        let gw_reference_id = match super::utils::u256_to_i64(user_decryption_id) {
            Ok(id) => id,
            Err(e) => {
                error!(
                    job_id = %event.job_id,
                    decryption_id = %user_decryption_id,
                    conversion_error = %e,
                    "Failed to convert U256 decryption ID to i64"
                );
                //TODO(Mano): Better strategy to handle the error and confirm with gateway team.
                self.dispatch_error_event(
                    event,
                    EventProcessingError::HandlerError("Decryption ID too large".to_string()),
                )
                .await;
                return;
            }
        };

        //TODO(SQL): For now TxHash is empty, since its only for informational purpose. Later fill it properly.
        if let Err(e) = self
            .user_decrypt_repo
            .update_status_to_receipt_received_on_tx_success(
                &decrypt_request.content_hash()[..],
                "",
                gw_reference_id,
            )
            .await
        {
            // ALWAYS log immediately with full context (guaranteed)
            error!(
                job_id = %event.job_id,
                indexer_id = %hex::encode(decrypt_request.content_hash()),
                sql_operation = "user_decrypt.update_status_to_receipt_received_on_tx_success",
                sql_error = %e,
                "SQL operation failed"
            );

            // Forward simple message to HTTP handler for 500
            self.dispatch_error_event(
                event,
                EventProcessingError::HandlerError("Failed SQL operation".to_string()),
            )
            .await;
            return;
        }

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
                info!("Processing user decrypt request {}", event.job_id);
                let indexer_id = decrypt_request.content_hash();

                match self.cache.check(decrypt_request).await {
                    Ok(cache_result) => match cache_result {
                        CacheResult::Hit(decrypt_response) => {
                            self.dispatch_cached_response(event.clone(), decrypt_response)
                                .await;
                        }
                        CacheResult::InProgress(_) => {
                            // Request already in progress - the orchestrator will handle
                            // distributing the result to all HTTP handlers with the same JobId
                            info!(
                                "Request already in progress for job_id={}, waiting for result",
                                event.job_id
                            );
                        }
                        CacheResult::NotFound => {
                            self.handle_new_request(event.clone(), decrypt_request, indexer_id)
                                .await;
                        }
                    },
                    Err(e) => {
                        error!("Failed to check cache for request {}: {}", event.job_id, e);
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
                        info!("Processing gateway response for request {}", event.job_id);
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
                    event.job_id, gw_req_reference_id
                );
            }
            _ => {}
        }
    }
}

fn assemble_final_response(shares: Vec<UserDecryptShare>) -> Result<UserDecryptResponse, String> {
    // Sort shares by index_share to maintain order
    let mut shares_vec: Vec<_> = shares.iter().map(|entry| entry.clone()).collect();
    shares_vec.sort_by_key(|share| share.share_index);

    let first_share = &shares_vec[0];
    let decryption_id = U256::from(first_share.gw_reference_id);

    // Extract reencrypted_shares with hex decoding
    let mut reencrypted_shares = Vec::new();
    for share in &shares_vec {
        match hex::decode(&share.share) {
            Ok(decoded) => reencrypted_shares.push(Bytes::from(decoded)),
            Err(e) => return Err(format!("Failed to decode share hex: {}", e)),
        }
    }

    // Extract signatures with hex decoding
    let mut signatures = Vec::new();
    for share in &shares_vec {
        match hex::decode(&share.kms_signature) {
            Ok(decoded) => signatures.push(Bytes::from(decoded)),
            Err(e) => return Err(format!("Failed to decode signature hex: {}", e)),
        }
    }

    // Use extra_data from first share with hex decoding
    let extra_data = match &first_share.extra_data {
        Some(hex_str) => match hex::decode(hex_str) {
            Ok(decoded) => Bytes::from(decoded),
            Err(e) => return Err(format!("Failed to decode extra_data hex: {}", e)),
        },
        None => Bytes::new(),
    };

    Ok(UserDecryptResponse {
        gateway_request_id: decryption_id,
        reencrypted_shares,
        signatures,
        extra_data,
    })
}
