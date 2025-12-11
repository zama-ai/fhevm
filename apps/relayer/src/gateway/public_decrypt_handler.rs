use crate::{
    core::{
        errors::{EventProcessingError, READINESS_CHECK_TIMEOUT_MSG},
        event::{
            GatewayChainEventData, GatewayChainEventId, PublicDecryptEventData,
            PublicDecryptEventId, PublicDecryptRequest, PublicDecryptResponse, RelayerEvent,
            RelayerEventData,
        },
        job_id::JobId,
    },
    gateway::{
        arbitrum::{
            bindings::Decryption,
            transaction::helper::{TransactionHelper, TransactionType},
            ComputeCalldata,
        },
        readiness_checker::{ReadinessCheckError, ReadinessChecker},
    },
    orchestrator::{
        traits::{EventDispatcher, EventHandler, HandlerRegistry},
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
    ) -> Arc<Self> {
        let handler = Arc::new(Self {
            dispatcher: Arc::clone(&dispatcher),
            tx_helper,
            readiness_checker,
            decryption_address,
            public_decrypt_repo,
        });

        // Self-register for events
        dispatcher.register_handler(
            &[
                PublicDecryptEventId::ReqRcvdFromUser.into(),
                PublicDecryptEventId::ReqSentToGw.into(),
                GatewayChainEventId::EventLogRcvd.into(),
            ],
            handler.clone() as Arc<dyn EventHandler<RelayerEvent>>,
        );

        handler
    }
}

#[async_trait]
impl EventHandler<RelayerEvent> for GatewayHandler {
    async fn handle_event(&self, event: RelayerEvent) {
        match &event.data {
            RelayerEventData::PublicDecrypt(PublicDecryptEventData::ReqRcvdFromUser {
                ref decrypt_request,
                ..
            }) => {
                info!("Processing public decrypt request {}", event.job_id);

                let result = async {
                    self.check_readiness(decrypt_request).await?;
                    info!("Readiness validation passed for {}", event.job_id);

                    let job_id_hash = decrypt_request.content_hash();
                    self.mark_processing(job_id_hash).await?;

                    self.send_public_decrypt_request(event.clone(), decrypt_request.clone())
                        .await
                }
                .await;

                if let Err(e) = result {
                    self.handle_error(event, e).await;
                }
            }
            RelayerEventData::GatewayChain(GatewayChainEventData::EventLogRcvd {
                ref log,
                tx_hash,
            }) => {
                if let Some(topic0) = log.topic0() {
                    if FixedBytes::<32>::from_slice(topic0.as_slice())
                        == Decryption::PublicDecryptionResponse::SIGNATURE_HASH
                    {
                        info!(
                            "Processing gateway response for public decrypt request {}",
                            event.job_id
                        );
                        let result = self.process_decrypt_response(&event, log, tx_hash).await;
                        if let Err(e) = result {
                            self.handle_error(event, e).await;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

impl GatewayHandler {
    /// Validates that all ciphertext handles are ready for decryption on fhevm.
    ///
    /// Checks if handles exist on fhevm blockchain and are accessible for decryption.
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

    /// Processes user public decrypt request by sending it to the Gateway blockchain.
    ///
    /// Steps:
    /// 1. Send transaction to Gateway Decryption contract
    /// 2. Extract decryption_id from receipt
    /// 3. Store receipt in database
    async fn send_public_decrypt_request(
        &self,
        event: RelayerEvent,
        decrypt_request: PublicDecryptRequest,
    ) -> Result<(), EventProcessingError> {
        info!(
            "Sending public decrypt request to gateway for {}",
            event.job_id
        );

        let handles_fixed_bytes: Vec<FixedBytes<32>> = decrypt_request
            .ct_handles
            .iter()
            .map(|bytes| FixedBytes::from(*bytes))
            .collect();

        let (decryption_id, tx_hash) = self
            .send_to_gateway(handles_fixed_bytes, decrypt_request.extra_data.clone())
            .await?;

        info!(
            "Public decrypt request sent to gateway for {}",
            event.job_id
        );
        self.store_request_receipt(decrypt_request, decryption_id, tx_hash)
            .await?;
        Ok(())
    }

    /// Sends public decryption transaction to Gateway Decryption contract.
    ///
    /// Returns the gateway reference ID (decryptionId) and transaction hash.
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

    /// Processes public decrypt response from Gateway.
    ///
    /// Steps:
    /// 1. Decode PublicDecryptionResponse event from log
    /// 2. Update database with decrypted value and signatures
    /// 3. Dispatch response event to notify HTTP handler
    async fn process_decrypt_response(
        &self,
        event: &RelayerEvent,
        log: &alloy::rpc::types::Log,
        tx_hash: &TxHash,
    ) -> Result<(), EventProcessingError> {
        let req =
            Decryption::PublicDecryptionResponse::decode_log_data(log.data()).map_err(|err| {
                error!(?err, "Failed to decode PublicDecryptionResponse event");
                EventProcessingError::EventDecodingFailed {
                    event_type: "PublicDecryptionResponse".to_string(),
                    reason: err.to_string(),
                }
            })?;

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
        let req_state = self
            .public_decrypt_repo
            .complete_req_with_res(public_decryption_id, decrypt_response.clone(), &tx_hash_str)
            .await
            .map_err(|e| EventProcessingError::SqlOperationFailed {
                operation: "public_decrypt.complete_req_with_res".to_string(),
                reason: e.to_string(),
            })?
            .ok_or_else(|| {
                warn!(
                    "Request not found or already completed/failed for gw_reference_id: {}",
                    public_decryption_id
                );
                EventProcessingError::ValidationFailed {
                    field: "gw_reference_id".to_string(),
                    reason: "Request not found or already completed/failed".to_string(),
                }
            })?;

        // Create JobId from content hash stored in database
        let job_id = JobId::from_sha256_hash(req_state.int_job_id.try_into().unwrap_or([0u8; 32]));

        // Dispatch response event to notify waiting HTTP handlers
        let response_event_data =
            RelayerEventData::PublicDecrypt(PublicDecryptEventData::RespRcvdFromGw {
                decrypt_response: decrypt_response.clone(),
            });

        let response_event = RelayerEvent::new(job_id, event.api_version, response_event_data);

        if let Err(e) = self.dispatcher.dispatch_event(response_event).await {
            error!(?e, "Failed to dispatch response event to HTTP handlers");
        } else {
            info!(
                "Public decrypt response successfully sent for {}",
                event.job_id
            );
        }

        Ok(())
    }

    /// Updates database status to "processing" after readiness check passes.
    async fn mark_processing(&self, job_id_hash: [u8; 32]) -> Result<(), EventProcessingError> {
        self.public_decrypt_repo
            .update_status_to_processing(&job_id_hash[..])
            .await
            .map(|_| ())
            .map_err(|e| EventProcessingError::SqlOperationFailed {
                operation: "public_decrypt.update_status_to_processing".to_string(),
                reason: e.to_string(),
            })
    }

    /// Stores transaction receipt in database after successful Gateway submission.
    ///
    /// Updates request status to "receipt_received" with gateway reference ID.
    async fn store_request_receipt(
        &self,
        decrypt_request: PublicDecryptRequest,
        decryption_id: U256,
        tx_hash: TxHash,
    ) -> Result<(), EventProcessingError> {
        let job_id_hash = decrypt_request.content_hash();
        let tx_hash_str = format!("{:?}", tx_hash);
        self.public_decrypt_repo
            .update_status_to_receipt_received_on_tx_success(
                &job_id_hash[..],
                &tx_hash_str,
                decryption_id,
            )
            .await
            .map(|_| ())
            .map_err(|e| EventProcessingError::SqlOperationFailed {
                operation: "public_decrypt.update_status_to_receipt_received_on_tx_success"
                    .to_string(),
                reason: e.to_string(),
            })
    }

    /// Handles errors during public decrypt processing.
    ///
    /// - SqlOperationFailed: Log + notify user
    /// - TransactionError: Update database status + notify user
    /// - Other errors: Update database status + notify user
    async fn handle_error(&self, event: RelayerEvent, error: EventProcessingError) {
        match &error {
            EventProcessingError::SqlOperationFailed { operation, reason } => {
                error!(
                    job_id = %event.job_id,
                    operation = %operation,
                    reason = %reason,
                    handler_type = "public_decrypt",
                    "SQL operation failed"
                );
            }

            EventProcessingError::TransactionError(tx_error) => {
                error!(
                    job_id = %event.job_id,
                    error = ?tx_error,
                    "Transaction failed - updating database and notifying user"
                );

                if let RelayerEventData::PublicDecrypt(PublicDecryptEventData::ReqRcvdFromUser {
                    ref decrypt_request,
                    ..
                }) = event.data
                {
                    let job_id_hash = decrypt_request.content_hash();
                    let err_reason = format!("Transaction Failed: {}", error);

                    // TODO(mano): Review if nested error logging is necessary or can be simplified
                    if let Err(db_err) = self
                        .public_decrypt_repo
                        .update_status_to_failure_on_tx_failed(&job_id_hash[..], &err_reason)
                        .await
                    {
                        error!(
                            job_id = %event.job_id,
                            db_error = %db_err,
                            "Failed to update failure status in database"
                        );
                    }
                }
            }

            EventProcessingError::ReadinessCheckFailed => {
                error!(
                    job_id = %event.job_id,
                    "Readiness check failed - updating database with timeout status"
                );

                if let RelayerEventData::PublicDecrypt(PublicDecryptEventData::ReqRcvdFromUser {
                    ref decrypt_request,
                    ..
                }) = event.data
                {
                    let job_id_hash = decrypt_request.content_hash();

                    if let Err(db_err) = self
                        .public_decrypt_repo
                        .update_status_to_timed_out(&job_id_hash[..], READINESS_CHECK_TIMEOUT_MSG)
                        .await
                    {
                        error!(
                            job_id = %event.job_id,
                            db_error = %db_err,
                            "Failed to update timeout status in database"
                        );
                    }
                }
            }

            _ => {
                error!(
                    job_id = %event.job_id,
                    error = ?error,
                    "Request processing failed - notifying user"
                );

                if let RelayerEventData::PublicDecrypt(PublicDecryptEventData::ReqRcvdFromUser {
                    ref decrypt_request,
                    ..
                }) = event.data
                {
                    let job_id_hash = decrypt_request.content_hash();
                    let err_reason = format!("Processing Failed: {}", error);

                    // TODO(mano): Review if nested error logging is necessary or can be simplified
                    if let Err(db_err) = self
                        .public_decrypt_repo
                        .update_status_to_failure_on_tx_failed(&job_id_hash[..], &err_reason)
                        .await
                    {
                        error!(
                            job_id = %event.job_id,
                            db_error = %db_err,
                            "Failed to update failure status in database"
                        );
                    }
                }
            }
        }

        self.notify_failed(event, error).await;
    }

    /// Dispatches failure event to notify waiting HTTP handlers.
    async fn notify_failed(&self, event: RelayerEvent, error: EventProcessingError) {
        let error_event = event.derive_next_event(RelayerEventData::PublicDecrypt(
            PublicDecryptEventData::Failed { error },
        ));

        if let Err(e) = self.dispatcher.dispatch_event(error_event).await {
            error!(?e, "Failed to dispatch error event");
        }
    }
}
