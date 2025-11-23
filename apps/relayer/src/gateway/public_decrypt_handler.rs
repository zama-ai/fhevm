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
        readiness_checker::{ReadinessCheckError, ReadinessChecker},
    },
    orchestrator::{
        traits::{Event, EventDispatcher, EventHandler},
        IndexerIdGenerator, Orchestrator, TokioEventDispatcher,
    },
    store::{
        key_value_db::KVStore, sql::repositories::public_decrypt_repo::PublicDecryptRepository,
        CacheResult, PublicDecryptCache,
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

struct PublicDecryptionRequestProcessor {}

impl ReceiptProcessor for PublicDecryptionRequestProcessor {
    type Output = U256;

    fn process(
        &self,
        receipt: &AnyTransactionReceipt,
    ) -> Result<Self::Output, EventProcessingError> {
        let target_topic = Decryption::PublicDecryptionRequest::SIGNATURE_HASH;
        let receipt: TransactionReceipt<AnyReceiptEnvelope<Log>> = receipt.inner.clone();

        debug!(
            "Receipt details for public decryption:\n\
             Hash: {:?}\n\
             Status: {}\n\
             Gas used: {:?}\n\
             Number of logs: {}\n\
             Block number: {:?}\n\
             Looking for topic: 0x{}",
            receipt.transaction_hash,
            receipt.status(),
            receipt.gas_used,
            receipt.inner.logs().len(),
            receipt.block_number,
            hex::encode(target_topic)
        );

        for log in receipt.inner.logs().iter() {
            if let Some(first_topic) = log.topics().first() {
                if first_topic == &target_topic {
                    return match Decryption::PublicDecryptionRequest::decode_log_data(log.data()) {
                        Ok(event) => Ok(event.decryptionId),
                        Err(e) => Err(EventProcessingError::DecodingError(e.to_string())),
                    };
                }
            }
        }

        Err(EventProcessingError::HandlerError(
            "Event not found in logs".into(),
        ))
    }
}

#[derive(Clone)]
pub struct GatewayHandler {
    dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    cache: Arc<PublicDecryptCache>,
    tx_helper: Arc<TransactionHelper>,
    readiness_checker: Arc<ReadinessChecker>,
    decryption_address: Address,
    public_decrypt_repo: Arc<PublicDecryptRepository>,
}

impl GatewayHandler {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
        kv_store: Arc<dyn KVStore>,
        tx_helper: Arc<TransactionHelper>,
        readiness_checker: Arc<ReadinessChecker>,
        decryption_address: Address,
        public_decrypt_repo: Arc<PublicDecryptRepository>,
    ) -> Self {
        let cache = Arc::new(PublicDecryptCache::new(kv_store));

        Self {
            dispatcher,
            cache,
            tx_helper,
            readiness_checker,
            decryption_address,
            public_decrypt_repo,
        }
    }

    // Cache state handlers - focused single-responsibility functions

    async fn dispatch_cached_response(
        &self,
        event: RelayerEvent,
        decrypt_response: PublicDecryptResponse,
    ) {
        info!("Cache hit for request {}", event.job_id);
        let next_event_data =
            RelayerEventData::PublicDecrypt(PublicDecryptEventData::RespRcvdFromGw {
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
        decrypt_request: &PublicDecryptRequest,
        indexer_id: [u8; 32],
    ) {
        info!("Starting readiness check for request {}", event.job_id);

        match self
            .start_readiness_check(event.clone(), decrypt_request)
            .await
        {
            Ok(()) => {
                // SQL operations using indexer_id
                if let Err(e) = self
                    .public_decrypt_repo
                    .update_status_to_processing(&indexer_id[..])
                    .await
                {
                    // ALWAYS log immediately with full context (guaranteed)
                    error!(
                        job_id = %event.job_id,
                        indexer_id = %hex::encode(indexer_id),
                        sql_operation = "public_decrypt.update_status_to_processing",
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

                // Emit ReadinessCheckPassed event
                let readiness_event_data =
                    RelayerEventData::PublicDecrypt(PublicDecryptEventData::ReadinessCheckPassed {
                        decrypt_request: decrypt_request.clone(),
                    });
                let readiness_event =
                    RelayerEvent::new(event.job_id(), event.api_version, readiness_event_data);
                if let Err(e) = self.dispatcher.dispatch_event(readiness_event).await {
                    error!(?e, "Failed to dispatch ReadinessCheckPassed event");
                }

                // Proceed with transaction sending
                self.forward_transaction_to_gateway(event, decrypt_request)
                    .await;
            }
            Err(readiness_error) => {
                let _ = self.cache.unlock_request(decrypt_request).await;
                self.dispatch_error_event(event, readiness_error).await;
            }
        }
    }

    async fn start_readiness_check(
        &self,
        _event: RelayerEvent,
        decrypt_request: &PublicDecryptRequest,
    ) -> Result<(), EventProcessingError> {
        let handles_fixed_bytes: Vec<FixedBytes<32>> = decrypt_request
            .ct_handles
            .iter()
            .map(|bytes| FixedBytes::from(*bytes))
            .collect();

        match self
            .readiness_checker
            .check_public_decryption_readiness(
                handles_fixed_bytes,
                decrypt_request.extra_data.clone(),
            )
            .await
        {
            Ok(()) => {
                info!("Readiness check passed");
                Ok(())
            }
            Err(ReadinessCheckError::Timeout) => {
                error!("Readiness check timed out");
                Err(EventProcessingError::ReadinessCheckFailed)
            }
            Err(ReadinessCheckError::ContractError(err)) => {
                error!("Readiness check contract error: {}", err);
                Err(EventProcessingError::HandlerError(err.to_string()))
            }
        }
    }

    async fn forward_transaction_to_gateway(
        &self,
        event: RelayerEvent,
        decrypt_request: &PublicDecryptRequest,
    ) {
        let handles_fixed_bytes: Vec<FixedBytes<32>> = decrypt_request
            .ct_handles
            .iter()
            .map(|bytes| FixedBytes::from(*bytes))
            .collect();

        match self
            .send_transaction_to_gateway(handles_fixed_bytes, decrypt_request.extra_data.clone())
            .await
        {
            Ok(decryption_id) => {
                if let Err(e) = self
                    .cache
                    .store_request_mapping(decrypt_request, decryption_id, event.job_id())
                    .await
                {
                    if (self.cache.unlock_request(decrypt_request).await).is_err() {
                        warn!("Cache unlock failed, continuing with error dispatch");
                    }
                    self.dispatch_error_event(event, e.into()).await;
                    return;
                }
                self.store_and_dispatch_success(event, decrypt_request.clone(), decryption_id)
                    .await;
            }
            Err(e) => {
                let _ = self.cache.unlock_request(decrypt_request).await;

                // Update database status to failure for transaction errors
                let indexer_id = decrypt_request.compute_indexer_id();
                let err_reason = format!("Transaction Failed: {}", e);
                if let Err(sql_error) = self
                    .public_decrypt_repo
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

    async fn send_transaction_to_gateway(
        &self,
        handles: Vec<FixedBytes<32>>,
        extra_data: Bytes,
    ) -> Result<U256, EventProcessingError> {
        let processor = PublicDecryptionRequestProcessor {};

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

    async fn handle_gateway_response_log(&self, event: RelayerEvent) {
        if let RelayerEventData::GatewayChain(GatewayChainEventData::EventLogRcvd { log }) =
            &event.data
        {
            if let Some(topic) = log.topic0() {
                if *topic == Decryption::PublicDecryptionResponse::SIGNATURE_HASH {
                    match Decryption::PublicDecryptionResponse::decode_log_data(log.data()) {
                        Ok(req) => {
                            let public_decryption_id = req.decryptionId;
                            info!(
                                "Gateway response received for decryption ID {}",
                                public_decryption_id
                            );

                            let decrypt_response = PublicDecryptResponse {
                                gateway_request_id: public_decryption_id,
                                decrypted_value: req.decryptedResult,
                                signatures: req.signatures,
                                extra_data: req.extraData,
                            };

                            let gw_reference_id =
                                match super::utils::u256_to_i64(public_decryption_id) {
                                    Ok(id) => id,
                                    Err(e) => {
                                        error!(
                                            job_id = %event.job_id,
                                            decryption_id = %public_decryption_id,
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

                            let respond_json = match serde_json::to_value(decrypt_response.clone())
                            {
                                Ok(json) => json,
                                Err(e) => {
                                    error!("Failed to serialize request data to JSON: {}", e);
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

                            //TODO(SQL): For now TxHash is empty, since its only for infomrational purpose. Later fill it prooperly.
                            if let Err(e) = self
                                .public_decrypt_repo
                                .complete_req_with_res(gw_reference_id, respond_json, "")
                                .await
                            {
                                // ALWAYS log immediately with full context (guaranteed)
                                error!(
                                    job_id = %event.job_id,
                                    decryption_id = %public_decryption_id,
                                    gw_reference_id = %gw_reference_id,
                                    sql_operation = "public_decrypt.complete_req_with_res",
                                    sql_error = %e,
                                    "SQL operation failed"
                                );

                                // Forward simple message to HTTP handler for 500
                                self.dispatch_error_event(
                                    event,
                                    EventProcessingError::HandlerError(
                                        "Failed SQL operation".to_string(),
                                    ),
                                )
                                .await;
                                return;
                            }

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

                            // Dispatch response event to notify waiting HTTP handlers
                            // We need to find the original JobId for this decryption_id
                            if let Some(original_job_id) = self.cache.get_job_id_for_decryption_id(public_decryption_id) {
                                let response_event_data = RelayerEventData::PublicDecrypt(
                                    PublicDecryptEventData::RespRcvdFromGw {
                                        decrypt_response: decrypt_response.clone(),
                                    },
                                );
                                
                                let response_event = RelayerEvent::new(
                                    original_job_id,
                                    event.api_version,
                                    response_event_data,
                                );
                                
                                if let Err(e) = self.dispatcher.dispatch_event(response_event).await {
                                    error!(?e, "Failed to dispatch response event to HTTP handlers");
                                }
                            } else {
                                error!("No JobId found for decryption_id={}", public_decryption_id);
                            }
                            
                            self.cache.cleanup_mapping(&public_decryption_id);
                        }
                        Err(e) => {
                            error!(?e, "Failed to decode event data");
                        }
                    }
                }
            }
        }
    }

    // Event dispatching

    async fn store_and_dispatch_success(
        &self,
        event: RelayerEvent,
        decrypt_request: PublicDecryptRequest,
        decryption_public_id: U256,
    ) {
        // No need to register duplicate - orchestrator handles distribution to all
        // HTTP handlers subscribed to the same content-based JobId

        if let Err(e) = self
            .cache
            .store_request_mapping(&decrypt_request, decryption_public_id, event.job_id())
            .await
        {
            warn!(
                ?event.job_id,
                "error persisting public decrypt request to cache: {}", e
            );
        }

        // Convert U256 to i64 for SQL operation (BIGINT)
        let gw_reference_id = match super::utils::u256_to_i64(decryption_public_id) {
            Ok(id) => id,
            Err(e) => {
                error!(
                    job_id = %event.job_id,
                    decryption_id = %decryption_public_id,
                    conversion_error = %e,
                    "Failed to convert U256 decryption ID to i64"
                );
                //TODO(Mano): Better stratgfy to handle theg errro and confirmn with garteway team.
                self.dispatch_error_event(
                    event,
                    EventProcessingError::HandlerError("Decryption ID too large".to_string()),
                )
                .await;
                return;
            }
        };

        //TODO(SQL): For now TxHash is empty, since its only for infomrational purpose. Later fill it prooperly.
        if let Err(e) = self
            .public_decrypt_repo
            .update_status_to_receipt_received_on_tx_success(
                &decrypt_request.compute_indexer_id()[..],
                "",
                gw_reference_id,
            )
            .await
        {
            // ALWAYS log immediately with full context (guaranteed)
            error!(
                job_id = %event.job_id,
                indexer_id = %hex::encode(decrypt_request.compute_indexer_id()),
                sql_operation = "public_decrypt.update_status_to_receipt_received_on_tx_success",
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
        let next_event = event.derive_next_event(RelayerEventData::PublicDecrypt(
            PublicDecryptEventData::ReqSentToGw {
                gw_req_reference_id: decryption_public_id,
            },
        ));

        if let Err(e) = self.dispatcher.dispatch_event(next_event).await {
            error!(?e, "Failed to dispatch DecryptRequestProcessed event");
        }
    }

    async fn dispatch_error_event(&self, event: RelayerEvent, error: EventProcessingError) {
        let error_event = event.derive_next_event(RelayerEventData::PublicDecrypt(
            PublicDecryptEventData::Failed { error },
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
            RelayerEventData::PublicDecrypt(PublicDecryptEventData::ReqRcvdFromUser {
                ref decrypt_request,
                ..
            }) => {
                let indexer_id = decrypt_request.compute_indexer_id();

                match self.cache.check(decrypt_request).await {
                    Ok(cache_result) => match cache_result {
                        CacheResult::Hit(decrypt_response) => {
                            self.dispatch_cached_response(event.clone(), decrypt_response)
                                .await;
                        }
                        CacheResult::InProgress(_) => {
                            // Request already in progress - the orchestrator will handle
                            // distributing the result to all HTTP handlers with the same JobId
                            info!("Request already in progress for job_id={}, waiting for result", event.job_id);
                        }
                        CacheResult::NotFound => {
                            self.handle_new_request(event.clone(), decrypt_request, indexer_id)
                                .await;
                        }
                    },
                    Err(e) => {
                        error!(?e, "Failed to check cache for request {}", event.job_id);
                        self.dispatch_error_event(event.clone(), e.into()).await;
                    }
                }
            }
            RelayerEventData::GatewayChain(GatewayChainEventData::EventLogRcvd { ref log }) => {
                if let Some(topic0) = log.topic0() {
                    if FixedBytes::<32>::from_slice(topic0.as_slice())
                        == Decryption::PublicDecryptionResponse::SIGNATURE_HASH
                    {
                        info!("Processing gateway response for request {}", event.job_id);
                        self.handle_gateway_response_log(event).await;
                    }
                };
            }
            RelayerEventData::PublicDecrypt(PublicDecryptEventData::ReqSentToGw {
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
