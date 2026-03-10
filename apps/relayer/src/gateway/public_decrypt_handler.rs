use crate::{
    config::settings::GwEventNotFoundRetryConfig,
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
            transaction::{
                helper::{TransactionHelper, TransactionType, TxResult},
                tx_throttler::{DynTxHook, GatewayTxTask, TxThrottlingSender},
                TxLifecycleHooks,
            },
            ComputeCalldata,
        },
        utils::{classify_revert_selector, extract_revert_selector},
    },
    logging::PublicDecryptStep,
    orchestrator::{
        traits::{Event, EventHandler},
        ContentHasher, Orchestrator,
    },
    readiness::throttler::{PublicDecryptReadinessTask, ReadinessSender},
    store::sql::{
        models::req_status_enum_model::ReqStatus,
        repositories::public_decrypt_repo::{
            PublicDecryptCompletionOutcome, PublicDecryptRepository,
        },
    },
};
use alloy::primitives::{Address, Bytes, FixedBytes, TxHash};
use alloy::sol_types::SolEvent;
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info, instrument, warn};

#[derive(Clone)]
pub struct GatewayHandler {
    dispatcher: Arc<Orchestrator>,
    tx_throttler: TxThrottlingSender<GatewayTxTask>,
    public_decrypt_readiness_throttler: ReadinessSender<PublicDecryptReadinessTask>,
    decryption_address: Address,
    public_decrypt_repo: Arc<PublicDecryptRepository>,
    gw_event_retry_config: GwEventNotFoundRetryConfig,
}

impl GatewayHandler {
    pub fn new(
        dispatcher: Arc<Orchestrator>,
        tx_throttler: TxThrottlingSender<GatewayTxTask>,
        public_decrypt_readiness_throttler: ReadinessSender<PublicDecryptReadinessTask>,
        decryption_address: Address,
        public_decrypt_repo: Arc<PublicDecryptRepository>,
        gw_event_retry_config: GwEventNotFoundRetryConfig,
    ) -> Arc<Self> {
        let handler = Arc::new(Self {
            dispatcher: Arc::clone(&dispatcher),
            tx_throttler,
            public_decrypt_readiness_throttler,
            decryption_address,
            public_decrypt_repo,
            gw_event_retry_config,
        });

        // Self-register for events
        dispatcher.register_handler(
            &[
                PublicDecryptEventId::ReqRcvdFromUser.into(),
                PublicDecryptEventId::ReqSentToGw.into(),
                // NOTE: We don't use Failed Event Id here, to allow notifying users
                PublicDecryptEventId::InternalFailure.into(),
                PublicDecryptEventId::ReadinessCheckPassed.into(),
                PublicDecryptEventId::ReadinessCheckTimedOut.into(),
                PublicDecryptEventId::ReadinessCheckFailed.into(),
                GatewayChainEventId::EventLogRcvd.into(),
            ],
            handler.clone() as Arc<dyn EventHandler<RelayerEvent>>,
        );

        handler
    }
}

#[async_trait]
impl EventHandler<RelayerEvent> for GatewayHandler {
    #[instrument(skip_all, fields(event_type=%event.event_name(), job_id=%event.job_id()))]
    async fn handle_event(&self, event: RelayerEvent) {
        let result: Result<_, EventProcessingError> = match &event.data {
            RelayerEventData::PublicDecrypt(public_decrypt_event) => match public_decrypt_event {
                PublicDecryptEventData::ReqRcvdFromUser {
                    ref decrypt_request,
                    ..
                } => {
                    info!("Processing public decrypt request {}", event.job_id);
                    async {
                        let job_id_hash = decrypt_request.content_hash();
                        self.readiness_check_enqueue(job_id_hash, decrypt_request)
                            .await
                    }
                    .await
                }
                PublicDecryptEventData::ReadinessCheckPassed {
                    ref decrypt_request,
                    ..
                } => {
                    async {
                        info!(
                            step = %PublicDecryptStep::ReadinessCheckPassed,
                            int_job_id = %event.job_id,
                            "Readiness check passed"
                        );

                        let job_id_hash = decrypt_request.content_hash();
                        self.mark_processing(job_id_hash).await?;

                        self.send_public_decrypt_request(event.clone(), decrypt_request.clone())
                            .await
                    }
                    .await
                }
                PublicDecryptEventData::ReadinessCheckTimedOut { ref error, .. } => {
                    Err(error.clone())
                }
                PublicDecryptEventData::ReadinessCheckFailed { ref error, .. } => {
                    Err(error.clone())
                }
                PublicDecryptEventData::InternalFailure { error } => Err(error.clone()),
                _ => {
                    warn!("unexpected event received in public decrypt handler");
                    return;
                }
            },

            RelayerEventData::GatewayChain(GatewayChainEventData::EventLogRcvd {
                ref log,
                tx_hash,
            }) => {
                if let Some(topic0) = log.topic0() {
                    if FixedBytes::<32>::from_slice(topic0.as_slice())
                        == Decryption::PublicDecryptionResponse::SIGNATURE_HASH
                    {
                        info!(
                            "Processing gateway response for public decrypt request {:?}",
                            event.job_id
                        );
                        self.process_decrypt_response(&event, log, tx_hash).await
                    } else {
                        return;
                    }
                } else {
                    return;
                }
            }
            _ => return,
        };

        if let Err(e) = result {
            self.handle_error(event, e).await;
        }
    }
}

impl GatewayHandler {
    /// Validates that all ciphertext handles are ready for decryption on fhevm.
    ///
    /// Checks if handles exist on fhevm blockchain and are accessible for decryption.
    /// Enqueue the readiness check event with a semaphore for throttling requests.
    async fn readiness_check_enqueue(
        &self,
        job_id_hash: [u8; 32],
        decrypt_request: &PublicDecryptRequest,
    ) -> Result<(), EventProcessingError> {
        let task: PublicDecryptReadinessTask = PublicDecryptReadinessTask {
            id: hex::encode(job_id_hash),
            job_id: JobId::from(job_id_hash),
            request: decrypt_request.clone(),
        };

        match self
            .public_decrypt_readiness_throttler
            .push(task.clone())
            .await
        {
            Ok(()) => {
                info!(
                    step = %PublicDecryptStep::ReadinessQueued,
                    int_job_id = %task.job_id,
                    "Request queued for readiness check"
                );
            }
            // Thoses errors are putting request in failure mode.
            // This introduce a new termination error, which is failure for readiness,
            // should NEVER happen with the bouncer.
            // NOTE: time_out instead ?
            Err(e) => match e {
                EventProcessingError::QueueFull => {
                    return Err(EventProcessingError::ProtocolOverload(
                        "Relayer is full for public readiness check, retry later.".to_string(),
                    ));
                }
                EventProcessingError::ChannelClosed => {
                    error!("FATAL: Cannot accept request for public readiness check, internal worker is dead.");
                    return Err(e);
                }
                _ => {
                    return Err(e);
                }
            },
        };

        Ok(())
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
            "Sending public decrypt request to gateway for {:?}",
            event.job_id
        );

        let job_id_hash = decrypt_request.content_hash();

        let handles_fixed_bytes: Vec<FixedBytes<32>> = decrypt_request
            .ct_handles
            .iter()
            .map(|bytes| FixedBytes::from(*bytes))
            .collect();

        self.send_to_gateway(
            handles_fixed_bytes,
            decrypt_request.extra_data.clone(),
            job_id_hash,
        )
        .await?;

        info!(
            "Public decrypt request sent to gateway for {:?}",
            event.job_id
        );
        Ok(())
    }

    /// Sends public decryption transaction to Gateway Decryption contract.
    ///
    /// Returns the gateway reference ID (decryptionId) and transaction hash.
    async fn send_to_gateway(
        &self,
        handles: Vec<FixedBytes<32>>,
        extra_data: Bytes,
        job_id_hash: [u8; 32],
    ) -> Result<(), EventProcessingError> {
        let decryption_address = self.decryption_address;

        let calldata_bytes =
            ComputeCalldata::public_decryption_req(handles.clone(), extra_data.clone())?;

        let job_id = JobId::from(job_id_hash);

        let task = GatewayTxTask {
            id: hex::encode(job_id_hash), // Used for Queue tracking/dedup
            job_id,
            transaction_type: TransactionType::PublicDecryptRequest,
            target: decryption_address,
            calldata: calldata_bytes,
            hook: DynTxHook(Arc::new(self.clone())),
        };

        info!(
            step = %PublicDecryptStep::TxQueued,
            int_job_id = %job_id,
            "Request enqueued to tx throttler"
        );

        // PUSH TO QUEUE
        // Catch error from here and pass the request to failure.
        // This case MUST never happen on this flow.
        // The request should never be injected in the system, and bounced after the cache check if the queue is full.
        match self.tx_throttler.push(task).await {
            Ok(()) => {}
            Err(e) => match e {
                EventProcessingError::QueueFull => {
                    return Err(EventProcessingError::ProtocolOverload(
                        "Relayer is full, retry later.".to_string(),
                    ));
                }
                EventProcessingError::ChannelClosed => {
                    error!("FATAL: Cannot accept request, internal worker is dead.");
                    return Err(e);
                }
                _ => {
                    return Err(e);
                }
            },
        };

        Ok(())
    }

    /// Processes public decrypt response from Gateway.
    ///
    /// Steps:
    /// 1. Decode PublicDecryptionResponse event from log
    /// 2. Update database with decrypted value and signatures
    /// 3. Dispatch response event to notify HTTP handler (only if newly completed)
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
            step = %PublicDecryptStep::GwEventReceived,
            int_job_id = %event.job_id,
            tx_hash = %tx_hash,
            gw_reference_id = %public_decryption_id,
            "Gateway response received"
        );

        let decrypt_response = PublicDecryptResponse {
            gateway_request_id: public_decryption_id,
            decrypted_value: req.decryptedResult,
            signatures: req.signatures,
            extra_data: format!("0x{}", hex::encode(&req.extraData)),
        };

        let tx_hash_str = format!("{:?}", tx_hash);
        let outcome = self
            .public_decrypt_repo
            .complete_req_with_res(public_decryption_id, decrypt_response.clone(), &tx_hash_str)
            .await
            .map_err(|e| EventProcessingError::SqlOperationFailed {
                operation: "public_decrypt.complete_req_with_res".to_string(),
                reason: e.to_string(),
            })?;

        match outcome {
            PublicDecryptCompletionOutcome::Completed { int_job_id } => {
                let job_id = int_job_id;

                // Dispatch response event to notify waiting HTTP handlers
                let response_event_data =
                    RelayerEventData::PublicDecrypt(PublicDecryptEventData::RespRcvdFromGw {
                        decrypt_response: decrypt_response.clone(),
                    });

                let response_event =
                    RelayerEvent::new(job_id, event.api_version, response_event_data);

                if let Err(e) = self.dispatcher.dispatch_event(response_event).await {
                    error!(?e, "Failed to dispatch response event to HTTP handlers");
                } else {
                    info!(
                        step = %PublicDecryptStep::RespSent,
                        int_job_id = %job_id,
                        "Response dispatched to HTTP handlers"
                    );
                }
            }
            PublicDecryptCompletionOutcome::AlreadyCompleted { int_job_id } => {
                debug!(
                    int_job_id = %int_job_id,
                    "Public decrypt already completed (duplicate event), skipping"
                );
            }
            PublicDecryptCompletionOutcome::AlreadyInFinalState {
                int_job_id,
                current_status,
            } => match current_status {
                ReqStatus::Failure => {
                    debug!(
                        int_job_id = %int_job_id,
                        "Public decrypt already in failure state, skipping response event"
                    );
                }
                ReqStatus::TimedOut => {
                    debug!(
                        int_job_id = %int_job_id,
                        "Public decrypt already timed out (late response event), skipping"
                    );
                }
                other_status => {
                    warn!(
                        int_job_id = %int_job_id,
                        current_status = ?other_status,
                        "Public decrypt in unexpected state, skipping response event - possible race condition or late event"
                    );
                }
            },
            PublicDecryptCompletionOutcome::NotFound => {
                // TEMPORARY FIX: Retry with delays - gateway event may have arrived before
                // gw_reference_id was stored (race condition during high RPC latency).
                // TODO: Replace with proper event buffering solution.
                let retry_config = &self.gw_event_retry_config;
                for attempt in 1..=retry_config.max_retries {
                    warn!(
                        step = %PublicDecryptStep::GwEventRetrying,
                        gw_reference_id = %public_decryption_id,
                        attempt = attempt,
                        max_retries = retry_config.max_retries,
                        "Gateway event arrived before gw_reference_id stored, retrying"
                    );

                    tokio::time::sleep(Duration::from_millis(retry_config.retry_delay_ms)).await;

                    let retry_outcome = self
                        .public_decrypt_repo
                        .complete_req_with_res(
                            public_decryption_id,
                            decrypt_response.clone(),
                            &tx_hash_str,
                        )
                        .await
                        .map_err(|e| EventProcessingError::SqlOperationFailed {
                            operation: "public_decrypt.complete_req_with_res".to_string(),
                            reason: e.to_string(),
                        })?;

                    match retry_outcome {
                        PublicDecryptCompletionOutcome::Completed { int_job_id } => {
                            info!(
                                step = %PublicDecryptStep::GwEventReceived,
                                int_job_id = %int_job_id,
                                gw_reference_id = %public_decryption_id,
                                attempt = attempt,
                                "Request found on retry, processing complete"
                            );

                            // Dispatch response event to notify waiting HTTP handlers
                            let response_event_data = RelayerEventData::PublicDecrypt(
                                PublicDecryptEventData::RespRcvdFromGw {
                                    decrypt_response: decrypt_response.clone(),
                                },
                            );

                            let response_event = RelayerEvent::new(
                                int_job_id,
                                event.api_version,
                                response_event_data,
                            );

                            if let Err(e) = self.dispatcher.dispatch_event(response_event).await {
                                error!(?e, "Failed to dispatch response event to HTTP handlers");
                            } else {
                                info!(
                                    step = %PublicDecryptStep::RespSent,
                                    int_job_id = %int_job_id,
                                    "Response dispatched to HTTP handlers"
                                );
                            }
                            return Ok(());
                        }
                        PublicDecryptCompletionOutcome::AlreadyCompleted { int_job_id } => {
                            debug!(
                                int_job_id = %int_job_id,
                                attempt = attempt,
                                "Public decrypt already completed on retry"
                            );
                            return Ok(());
                        }
                        PublicDecryptCompletionOutcome::AlreadyInFinalState {
                            int_job_id,
                            current_status,
                        } => {
                            debug!(
                                int_job_id = %int_job_id,
                                current_status = ?current_status,
                                attempt = attempt,
                                "Public decrypt in final state on retry"
                            );
                            return Ok(());
                        }
                        PublicDecryptCompletionOutcome::NotFound => {
                            if attempt == retry_config.max_retries {
                                warn!(
                                    step = %PublicDecryptStep::GwEventRetrying,
                                    gw_reference_id = %public_decryption_id,
                                    max_retries = retry_config.max_retries,
                                    "Request not found after all retries, dropping event"
                                );
                            }
                            // Continue to next attempt
                        }
                    }
                }
            }
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
                    "Transaction failed - Status updated in the helper, notifying user"
                );
            }

            EventProcessingError::ReadinessCheckTimedOut => {
                error!(
                    job_id = %event.job_id,
                    "Readiness check failed - updating database with timeout status"
                );

                if let RelayerEventData::PublicDecrypt(
                    PublicDecryptEventData::ReadinessCheckTimedOut {
                        ref decrypt_request,
                        ..
                    },
                ) = event.data
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

            EventProcessingError::NotAllowedOnHostAcl(_)
            | EventProcessingError::HostAclFailed(_) => {
                let err_reason = error.to_string();
                error!(
                    job_id = %event.job_id,
                    error = %err_reason,
                    "Host ACL check failed"
                );

                if let RelayerEventData::PublicDecrypt(
                    PublicDecryptEventData::ReadinessCheckFailed {
                        ref decrypt_request,
                        ..
                    },
                ) = event.data
                {
                    let job_id_hash = decrypt_request.content_hash();

                    if let Err(db_err) = self
                        .public_decrypt_repo
                        .update_status_to_failure_from_queued(&job_id_hash[..], &err_reason)
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

                    if let Err(db_err) = self
                        .public_decrypt_repo
                        .update_status_to_failure_from_queued(&job_id_hash[..], &err_reason)
                        .await
                    {
                        error!(
                            job_id = %event.job_id,
                            db_error = %db_err,
                            "Failed to update failure status in database"
                        );
                    }
                }

                if let RelayerEventData::PublicDecrypt(
                    PublicDecryptEventData::ReadinessCheckFailed {
                        ref decrypt_request,
                        ..
                    },
                ) = event.data
                {
                    let job_id_hash = decrypt_request.content_hash();
                    let err_reason = format!("Processing Failed: {}", error);

                    if let Err(db_err) = self
                        .public_decrypt_repo
                        .update_status_to_failure_from_queued(&job_id_hash[..], &err_reason)
                        .await
                    {
                        error!(
                            job_id = %event.job_id,
                            db_error = %db_err,
                            "Failed to update failure status in database"
                        );
                    }
                }

                if let RelayerEventData::PublicDecrypt(
                    PublicDecryptEventData::ReadinessCheckPassed {
                        ref decrypt_request,
                        ..
                    },
                ) = event.data
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

#[async_trait]
impl TxLifecycleHooks for GatewayHandler {
    async fn on_tx_in_flight(&self, job_id: &JobId) -> Result<(), EventProcessingError> {
        self.public_decrypt_repo
            .update_status_to_tx_in_flight(&job_id[..])
            .await
            .map(|_| ())
            .map_err(|e| EventProcessingError::SqlOperationFailed {
                operation: "public_decrypt.update_status_to_tx_in_flight".to_string(),
                reason: e.to_string(),
            })
    }

    async fn on_receipt_received(
        &self,
        job_id: &JobId,
        receipt: &TxResult,
    ) -> Result<(), EventProcessingError> {
        let gw_reference_id = TransactionHelper::extract_gateway_id_from_receipt::<
            Decryption::PublicDecryptionRequest,
        >(
            receipt,
            Decryption::PublicDecryptionRequest::SIGNATURE_HASH,
            |event| event.decryptionId,
        )?;

        let tx_hash = format!("{:?}", receipt.transaction_hash);

        info!(
            step = %PublicDecryptStep::TxConfirmed,
            int_job_id = %job_id,
            tx_hash = %tx_hash,
            gw_reference_id = %gw_reference_id,
            "Transaction confirmed, receipt received"
        );

        self.public_decrypt_repo
            .update_status_to_receipt_received_on_tx_success(&job_id[..], &tx_hash, gw_reference_id)
            .await
            .map(|_| ())
            .map_err(|e| EventProcessingError::SqlOperationFailed {
                operation: "public_decrypt.update_status_to_receipt_received_on_tx_success"
                    .to_string(),
                reason: e.to_string(),
            })
    }

    async fn on_failure(
        &self,
        job_id: &JobId,
        err_reason: &str,
    ) -> Result<(), EventProcessingError> {
        // Only track revert metrics if we can extract a selector (means it's actually a revert)
        if let Some(selector) = extract_revert_selector(err_reason) {
            let reason = classify_revert_selector(&selector);
            crate::metrics::transaction::track_revert_with_request_type(reason, "public_decrypt");
        }

        self.public_decrypt_repo
            .update_status_to_failure_on_tx_failed(&job_id[..], err_reason)
            .await
            .map(|_| ())
            .map_err(|e| EventProcessingError::SqlOperationFailed {
                operation: "public_decrypt.update_status_to_failure_on_tx_failed".to_string(),
                reason: e.to_string(),
            })
    }
}
