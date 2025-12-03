use crate::{
    core::{
        errors::EventProcessingError,
        event::{
            GatewayChainEventData, PublicDecryptEventData, PublicDecryptRequest,
            PublicDecryptResponse, RelayerEvent, RelayerEventData,
        },
        job_id::JobId,
    },
    gateway::utils::sql_errors,
    gateway::{
        arbitrum::{
            bindings::Decryption,
            transaction::helper::{TransactionHelper, TransactionType},
            ComputeCalldata,
        },
        readiness_checker::{ReadinessCheckError, ReadinessChecker},
    },
    orchestrator::{
        traits::{EventDispatcher, EventHandler},
        ContentHasher, Orchestrator, TokioEventDispatcher,
    },
    store::sql::repositories::public_decrypt_repo::PublicDecryptRepository,
};
use alloy::primitives::{Address, Bytes, FixedBytes, TxHash, U256};
use alloy::sol_types::SolEvent;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{error, info, warn};

#[derive(Clone)]
pub struct GatewayHandler {
    dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    tx_helper: Arc<TransactionHelper>,
    readiness_checker: Arc<ReadinessChecker>,
    decryption_address: Address,
    public_decrypt_repo: Arc<PublicDecryptRepository>,
}

impl GatewayHandler {
    pub fn new(
        dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
        tx_helper: Arc<TransactionHelper>,
        readiness_checker: Arc<ReadinessChecker>,
        decryption_address: Address,
        public_decrypt_repo: Arc<PublicDecryptRepository>,
    ) -> Self {
        Self {
            dispatcher,
            tx_helper,
            readiness_checker,
            decryption_address,
            public_decrypt_repo,
        }
    }

    async fn check_readiness(
        &self,
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
                Err(EventProcessingError::ContractCallFailed(err.to_string()))
            }
        }
    }

    async fn mark_processing(&self, event: RelayerEvent, job_id_hash: [u8; 32]) {
        if let Err(e) = self
            .public_decrypt_repo
            .update_status_to_processing(&job_id_hash[..])
            .await
        {
            sql_errors::public_decrypt_sql_error(
                &self.dispatcher,
                event,
                "public_decrypt.update_status_to_processing",
                &e,
                Some(("job_id_hash", &hex::encode(job_id_hash))),
            )
            .await;
        }
    }

    async fn send_public_decrypt_request(
        &self,
        event: RelayerEvent,
        decrypt_request: PublicDecryptRequest,
    ) {
        info!(
            "Sending public decrypt request to gateway for {}",
            event.job_id
        );

        let handles_fixed_bytes: Vec<FixedBytes<32>> = decrypt_request
            .ct_handles
            .iter()
            .map(|bytes| FixedBytes::from(*bytes))
            .collect();

        match self
            .send_to_gateway(handles_fixed_bytes, decrypt_request.extra_data.clone())
            .await
        {
            Ok((decryption_id, tx_hash)) => {
                info!(
                    "Public decrypt request sent to gateway for {}",
                    event.job_id
                );

                // Update status to receipt received (no event dispatching)
                let job_id_hash = decrypt_request.content_hash();
                let tx_hash_str = format!("{:?}", tx_hash);
                if let Err(e) = self
                    .public_decrypt_repo
                    .update_status_to_receipt_received_on_tx_success(
                        &job_id_hash[..],
                        &tx_hash_str,
                        decryption_id,
                    )
                    .await
                {
                    sql_errors::public_decrypt_sql_error(
                        &self.dispatcher,
                        event,
                        "public_decrypt.update_status_to_receipt_received_on_tx_success",
                        &e,
                        Some(("job_id_hash", &hex::encode(job_id_hash))),
                    )
                    .await;
                }
            }
            Err(e) => {
                // Update database status to failure for transaction errors
                let job_id_hash = decrypt_request.content_hash();
                let err_reason = format!("Transaction Failed: {}", e);
                if let Err(sql_error) = self
                    .public_decrypt_repo
                    .update_status_to_failure_on_tx_failed(&job_id_hash[..], &err_reason)
                    .await
                {
                    error!(
                        job_id = %event.job_id,
                        job_id_hash = %hex::encode(job_id_hash),
                        sql_error = %sql_error,
                        "Failed to update transaction failure status in database"
                    );
                }

                self.notify_failed(event, e).await;
            }
        }
    }

    async fn send_to_gateway(
        &self,
        handles: Vec<FixedBytes<32>>,
        extra_data: Bytes,
    ) -> Result<(U256, TxHash), EventProcessingError> {
        let decryption_address = self.decryption_address;

        let receipt = self
            .tx_helper
            .send_raw_transaction_sync(
                TransactionType::PublicDecryptRequest,
                decryption_address,
                || ComputeCalldata::public_decryption_req(handles.clone(), extra_data.clone()),
            )
            .await?;

        // Extract gateway reference ID from the PublicDecryptionRequest event
        let gw_reference_id = TransactionHelper::extract_gateway_id_from_receipt::<
            Decryption::PublicDecryptionRequest,
        >(
            &receipt,
            Decryption::PublicDecryptionRequest::SIGNATURE_HASH,
            |event| event.decryptionId,
        )?;

        Ok((gw_reference_id, receipt.transaction_hash))
    }

    async fn decode_and_complete_response(&self, event: RelayerEvent) {
        info!(
            "Processing gateway response for public decrypt request {}",
            event.job_id
        );
        if let RelayerEventData::GatewayChain(GatewayChainEventData::EventLogRcvd {
            log,
            tx_hash,
        }) = &event.data
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

                            let tx_hash_str = format!("{:?}", tx_hash);
                            let req_state = match self
                                .public_decrypt_repo
                                .complete_req_with_res(
                                    public_decryption_id,
                                    decrypt_response.clone(),
                                    &tx_hash_str,
                                )
                                .await
                            {
                                Ok(Some(state)) => state,
                                Ok(None) => {
                                    warn!("Request not found or already completed/failed for gw_reference_id: {}", public_decryption_id);
                                    return;
                                }
                                Err(e) => {
                                    // ALWAYS log immediately with full context (guaranteed)
                                    error!(
                                        job_id = %event.job_id,
                                        decryption_id = %public_decryption_id,
                                        gw_reference_id = %public_decryption_id,
                                        sql_operation = "public_decrypt.complete_req_with_res",
                                        sql_error = %e,
                                        "SQL operation failed"
                                    );

                                    // Forward simple message to HTTP handler for 500
                                    self.notify_failed(
                                        event,
                                        EventProcessingError::SqlOperationFailed {
                                            operation: "public_decrypt.complete_req_with_res"
                                                .to_string(),
                                            reason: e.to_string(),
                                        },
                                    )
                                    .await;
                                    return;
                                }
                            };

                            // Create JobId from content hash stored in database
                            let job_id = JobId::from_sha256_hash(
                                req_state.int_indexer_id.try_into().unwrap_or([0u8; 32]),
                            );

                            // Dispatch response event to notify waiting HTTP handlers
                            let response_event_data = RelayerEventData::PublicDecrypt(
                                PublicDecryptEventData::RespRcvdFromGw {
                                    decrypt_response: decrypt_response.clone(),
                                },
                            );

                            let response_event =
                                RelayerEvent::new(job_id, event.api_version, response_event_data);

                            if let Err(e) = self.dispatcher.dispatch_event(response_event).await {
                                error!(?e, "Failed to dispatch response event to HTTP handlers");
                            } else {
                                info!(
                                    "Public decrypt response successfully sent for {}",
                                    event.job_id
                                );
                            }
                        }
                        Err(e) => {
                            error!(?e, "Failed to decode PublicDecryptionResponse event");
                            self.notify_failed(
                                event,
                                EventProcessingError::EventDecodingFailed {
                                    event_type: "PublicDecryptionResponse".to_string(),
                                    reason: e.to_string(),
                                },
                            )
                            .await;
                        }
                    }
                }
            }
        }
    }

    // Event dispatching

    async fn notify_failed(&self, event: RelayerEvent, error: EventProcessingError) {
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
                let job_id_hash = decrypt_request.content_hash();
                let decrypt_request_clone = decrypt_request.clone();

                info!("Processing public decrypt request {}", event.job_id);

                // Stage 1: Check readiness (ReadinessChecker component)
                match self.check_readiness(&decrypt_request_clone).await {
                    Ok(()) => {
                        info!("Readiness validation passed for {}", event.job_id);

                        // Stage 2: Update SQL status to processing
                        self.mark_processing(event.clone(), job_id_hash).await;

                        // Stage 3: Send to gateway (pure transaction execution)
                        self.send_public_decrypt_request(event, decrypt_request_clone)
                            .await;
                    }
                    Err(readiness_error) => {
                        error!(
                            "Readiness validation failed for {}: {:?}",
                            event.job_id, readiness_error
                        );
                        self.notify_failed(event, readiness_error).await;
                    }
                }
            }
            RelayerEventData::GatewayChain(GatewayChainEventData::EventLogRcvd {
                ref log,
                tx_hash: _,
            }) => {
                if let Some(topic0) = log.topic0() {
                    if FixedBytes::<32>::from_slice(topic0.as_slice())
                        == Decryption::PublicDecryptionResponse::SIGNATURE_HASH
                    {
                        info!(
                            "Decoding and completing public decrypt response for request {}",
                            event.job_id
                        );
                        self.decode_and_complete_response(event).await;
                    }
                };
            }
            _ => {}
        }
    }
}
