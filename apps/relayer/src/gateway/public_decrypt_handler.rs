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
    store::{key_value_db::KVStore, CacheResult, PublicDecryptCache, sql::repositories::public_decrypt_repo::PublicDecryptRepository},
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

    // Cache operations

    async fn act_on_cache_result(
        &self,
        cache_result: CacheResult<PublicDecryptResponse>,
        event: RelayerEvent,
        decrypt_request: &PublicDecryptRequest,
    ) {
        match cache_result {
            CacheResult::Hit(decrypt_response) => {
                info!("Cache hit for request {}", event.request_id);
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
                    "Duplicate request {} found for decryption {}",
                    event.request_id, decryption_id
                );
                self.cache
                    .register_duplicate(decryption_id, event.request_id);
            }
            CacheResult::NotFound => {
                info!("Sending request {} to gateway", event.request_id);
                match self.send_public_decrypt_to_gateway(decrypt_request).await {
                    Ok(decryption_id) => {
                        if let Err(e) = self
                            .cache
                            .store_request_mapping(decrypt_request, decryption_id)
                            .await
                        {
                            if (self.cache.unlock_request(decrypt_request).await).is_err() {
                                warn!("Cache unlock failed, continuing with error dispatch");
                            }
                            self.dispatch_error_event(event, e.into()).await;
                            return;
                        }
                        self.store_and_dispatch_success(
                            event,
                            decrypt_request.clone(),
                            decryption_id,
                        )
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

    async fn send_public_decrypt_to_gateway(
        &self,
        decrypt_request: &PublicDecryptRequest,
    ) -> Result<U256, EventProcessingError> {
        let handles_fixed_bytes: Vec<FixedBytes<32>> = decrypt_request
            .ct_handles
            .iter()
            .map(|bytes| FixedBytes::from(*bytes))
            .collect();

        self.readiness_checker
            .check_public_decryption_readiness(
                handles_fixed_bytes.clone(),
                decrypt_request.extra_data.clone(),
            )
            .await?;

        self.send_transaction_to_gateway(handles_fixed_bytes, decrypt_request.extra_data.clone())
            .await
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
                                for original_request_id in waiting_requests {
                                    let req_response = decrypt_response.clone();
                                    debug!("Notifying original request {}", original_request_id);

                                    let next_event_data = RelayerEventData::PublicDecrypt(
                                        PublicDecryptEventData::RespRcvdFromGw {
                                            decrypt_response: req_response,
                                        },
                                    );

                                    let next_event = RelayerEvent::new(
                                        original_request_id,
                                        event.api_version,
                                        next_event_data,
                                    );

                                    let _ = self.dispatcher.dispatch_event(next_event).await;
                                }

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

    // Event dispatching

    async fn store_and_dispatch_success(
        &self,
        event: RelayerEvent,
        decrypt_request: PublicDecryptRequest,
        decryption_public_id: U256,
    ) {
        self.cache
            .register_duplicate(decryption_public_id, event.request_id);

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
            }) => match self.cache.check(decrypt_request).await {
                Ok(cache_result) => {
                    self.act_on_cache_result(cache_result, event.clone(), decrypt_request)
                        .await;
                }
                Err(e) => {
                    error!(?e, "Failed to check cache for request {}", event.request_id);
                    self.dispatch_error_event(event.clone(), e.into()).await;
                }
            },
            RelayerEventData::GatewayChain(GatewayChainEventData::EventLogRcvd { ref log }) => {
                if let Some(topic0) = log.topic0() {
                    if FixedBytes::<32>::from_slice(topic0.as_slice())
                        == Decryption::PublicDecryptionResponse::SIGNATURE_HASH
                    {
                        info!(
                            "Processing gateway response for request {}",
                            event.request_id
                        );
                        self.handle_gateway_response_log(event).await;
                    }
                };
            }
            RelayerEventData::PublicDecrypt(PublicDecryptEventData::ReqSentToGw {
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
