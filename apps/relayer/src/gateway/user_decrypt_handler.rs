use crate::{
    core::{
        errors::{EventProcessingError, READINESS_CHECK_TIMEOUT_MSG},
        event::{
            DelegatedUserDecryptEventData, DelegatedUserDecryptRequest, GatewayChainEventData,
            GatewayChainEventId, HandleContractPair, RelayerEvent, RelayerEventData,
            UserDecryptEventData, UserDecryptEventId, UserDecryptRequest, UserDecryptResponse,
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
        readiness_check::readiness_throttler::{
            DelegatedUserDecryptReadinessTask, ReadinessSender, UserDecryptReadinessTask,
        },
        utils::{classify_revert_selector, extract_revert_selector},
    },
    logging::UserDecryptStep,
    orchestrator::{
        traits::{Event, EventDispatcher, EventHandler, HandlerRegistry},
        ContentHasher, Orchestrator, TokioEventDispatcher,
    },
    store::sql::{
        models::{
            req_status_enum_model::ReqStatus,
            user_decrypt_req_model::ConsensusReqState,
            user_decrypt_share_model::{ShareInsertParams, UserDecryptShare},
        },
        repositories::user_decrypt_repo::{ShareCompletionOutcome, UserDecryptRepository},
    },
};
use alloy::primitives::{Address, Bytes, FixedBytes, TxHash, U256};
use alloy::sol_types::SolEvent;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{debug, error, info, instrument, warn};

impl From<&HandleContractPair> for Decryption::CtHandleContractPair {
    fn from(pair: &HandleContractPair) -> Self {
        Self {
            ctHandle: pair.ct_handle.into(),
            contractAddress: pair.contract_address,
        }
    }
}

#[derive(Clone)]
pub struct GatewayHandler {
    dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    tx_throttler: TxThrottlingSender<GatewayTxTask>,
    user_decrypt_readiness_throttler: ReadinessSender<UserDecryptReadinessTask>,
    delegated_user_decrypt_readiness_throttler: ReadinessSender<DelegatedUserDecryptReadinessTask>,
    decryption_address: Address,
    user_decrypt_repo: Arc<UserDecryptRepository>,
    user_decrypt_shares_threshold: i64,
}

impl GatewayHandler {
    pub fn new(
        dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
        tx_throttler: TxThrottlingSender<GatewayTxTask>,
        user_decrypt_readiness_throttler: ReadinessSender<UserDecryptReadinessTask>,
        delegated_user_decrypt_readiness_throttler: ReadinessSender<
            DelegatedUserDecryptReadinessTask,
        >,
        decryption_address: Address,
        user_decrypt_shares_threshold: usize,
        user_decrypt_repo: Arc<UserDecryptRepository>,
    ) -> Arc<Self> {
        let handler = Arc::new(Self {
            dispatcher: Arc::clone(&dispatcher),
            tx_throttler,
            user_decrypt_readiness_throttler,
            delegated_user_decrypt_readiness_throttler,
            decryption_address,
            user_decrypt_repo,
            user_decrypt_shares_threshold: user_decrypt_shares_threshold as i64,
        });

        // Self-register for events
        dispatcher.register_handler(
            &[
                UserDecryptEventId::ReqRcvdFromUser.into(),
                UserDecryptEventId::ReqSentToGw.into(),
                // NOTE: We don't use Failed Event Id here, to allow notifying users
                UserDecryptEventId::InternalFailure.into(),
                UserDecryptEventId::ReadinessCheckPassed.into(),
                UserDecryptEventId::ReadinessCheckTimedOut.into(),
                UserDecryptEventId::ReadinessCheckFailed.into(),
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
            RelayerEventData::UserDecrypt(user_decrypt_event) => match user_decrypt_event {
                UserDecryptEventData::ReqRcvdFromUser {
                    ref decrypt_request,
                    ..
                } => {
                    info!("Processing user decrypt request {}", event.job_id);
                    async {
                        let job_id_hash = decrypt_request.content_hash();
                        self.readiness_check_enqueue(job_id_hash, decrypt_request)
                            .await
                    }
                    .await
                }
                UserDecryptEventData::ReadinessCheckPassed {
                    ref decrypt_request,
                    ..
                } => {
                    async {
                        info!(
                            step = %UserDecryptStep::ReadinessCheckPassed,
                            int_job_id = %event.job_id,
                            "Readiness check passed, sending user decrypt request to gateway"
                        );

                        let job_id_hash = decrypt_request.content_hash();
                        self.mark_processing(job_id_hash).await?;

                        self.send_user_decrypt_request(event.clone(), decrypt_request.clone())
                            .await
                    }
                    .await
                }
                UserDecryptEventData::ReadinessCheckTimedOut { ref error, .. } => {
                    Err(error.clone())
                }
                UserDecryptEventData::ReadinessCheckFailed { ref error, .. } => Err(error.clone()),
                UserDecryptEventData::InternalFailure { error } => Err(error.clone()),
                _ => {
                    warn!("unexpected event received in user decrypt handler");
                    return;
                }
            },

            RelayerEventData::DelegatedUserDecrypt(delegated_user_decrypt_event) => {
                match delegated_user_decrypt_event {
                    DelegatedUserDecryptEventData::ReqRcvdFromUser {
                        ref decrypt_request,
                        ..
                    } => {
                        info!("Processing delegated user decrypt request {}", event.job_id);
                        async {
                            let job_id_hash = decrypt_request.content_hash();
                            self.delegated_user_decrypt_readiness_check_enqueue(job_id_hash, decrypt_request)
                                .await
                        }
                        .await
                    }
                    DelegatedUserDecryptEventData::ReadinessCheckPassed {
                        ref decrypt_request,
                        ..
                    } => {
                        async {
                            info!(
                        "Readiness check passed. Throttling delegated user decrypt request to gateway {}",
                        event.job_id
                    );

                            let job_id_hash = decrypt_request.content_hash();
                            self.mark_processing(job_id_hash).await?;

                            self.send_delegated_user_decrypt_request(event.clone(), decrypt_request.clone())
                                .await
                        }
                        .await
                    }
                    DelegatedUserDecryptEventData::ReadinessCheckTimedOut { ref error, .. } => {
                        Err(error.clone())
                    }
                    DelegatedUserDecryptEventData::ReadinessCheckFailed { ref error, .. } => {
                        Err(error.clone())
                    }
                    DelegatedUserDecryptEventData::InternalFailure { error } => Err(error.clone()),
                    _ => {
                        warn!("Unexpected event received in delegated user decrypt handler");
                        return;
                    }
                }
            }

            RelayerEventData::GatewayChain(GatewayChainEventData::EventLogRcvd {
                ref log,
                tx_hash,
            }) => {
                if let Some(topic0) = log.topic0() {
                    let topic0_fixed = FixedBytes::<32>::from_slice(topic0.as_slice());
                    let individual_response_topic =
                        Decryption::UserDecryptionResponse::SIGNATURE_HASH;
                    let consensus_topic = self.get_consensus_event_topic();

                    match topic0_fixed {
                        topic if topic == individual_response_topic => {
                            info!("Processing share response for request {}", event.job_id);
                            self.decode_share_from_log(log, event.clone(), *tx_hash)
                                .await
                        }
                        topic if topic == consensus_topic => {
                            info!(
                                "Processing consensus response for request {:?}",
                                event.job_id
                            );
                            self.update_consensus_hash(log, event.clone(), *tx_hash)
                                .await;
                            return;
                        }
                        _ => {
                            debug!(
                                "Ignoring event: received topic {:?}, expected individual {:?} or consensus {:?}",
                                topic0_fixed, individual_response_topic, consensus_topic
                            );
                            return;
                        }
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
    /// Validates that all ciphertext handles are ready and user is authorized for decryption.
    ///
    /// Checks if handles exist on fhevm and user has permission to decrypt them.
    /// Enqueue the readiness check event with a semaphore for throttling requests.
    async fn readiness_check_enqueue(
        &self,
        job_id_hash: [u8; 32],
        decrypt_request: &UserDecryptRequest,
    ) -> Result<(), EventProcessingError> {
        let task: UserDecryptReadinessTask = UserDecryptReadinessTask {
            id: hex::encode(job_id_hash),
            job_id: JobId::from(job_id_hash),
            request: decrypt_request.clone(),
        };

        match self
            .user_decrypt_readiness_throttler
            .push(task.clone())
            .await
        {
            Ok(()) => {
                info!(
                    step = %UserDecryptStep::ReadinessQueued,
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

    /// Validates that all ciphertext handles are ready and user is authorized for delegated decryption.
    ///
    /// Checks if handles exist on fhevm and user has permission to decrypt them.
    /// Enqueue the readiness check event with a semaphore for throttling requests.
    async fn delegated_user_decrypt_readiness_check_enqueue(
        &self,
        job_id_hash: [u8; 32],
        decrypt_request: &DelegatedUserDecryptRequest,
    ) -> Result<(), EventProcessingError> {
        let task: DelegatedUserDecryptReadinessTask = DelegatedUserDecryptReadinessTask {
            id: hex::encode(job_id_hash),
            job_id: JobId::from(job_id_hash),
            request: decrypt_request.clone(),
        };

        match self
            .delegated_user_decrypt_readiness_throttler
            .push(task)
            .await
        {
            Ok(()) => {}
            // The following are termination errors, which means a failure on readiness.
            // These should NEVER happen with the bouncer.
            Err(e) => match e {
                EventProcessingError::QueueFull => {
                    return Err(EventProcessingError::ProtocolOverload(
                        "Relayer is full for delegated user readiness check, retry later."
                            .to_string(),
                    ));
                }
                EventProcessingError::ChannelClosed => {
                    error!("FATAL: Cannot accept request for delegated user readiness check, internal worker is dead.");
                    return Err(e);
                }
                _ => {
                    return Err(e);
                }
            },
        };

        Ok(())
    }

    /// Processes user decrypt request by sending it to the Gateway blockchain.
    ///
    /// Steps:
    /// 1. Send transaction to Gateway Decryption contract
    /// 2. Extract user_decryption_id from receipt
    /// 3. Store receipt in database
    async fn send_user_decrypt_request(
        &self,
        event: RelayerEvent,
        decrypt_request: UserDecryptRequest,
    ) -> Result<(), EventProcessingError> {
        info!(
            "Sending user decrypt request to gateway for {:?}",
            event.job_id
        );

        let job_id_hash = decrypt_request.content_hash();

        let calldata_bytes = ComputeCalldata::user_decryption_req(decrypt_request.clone())?;

        self.send_to_gateway(calldata_bytes, job_id_hash).await?;
        info!(
            "User decrypt request sent to gateway for {:?}",
            event.job_id
        );
        Ok(())
    }

    /// Processes delegated user decrypt request by sending it to the Gateway blockchain.
    ///
    /// Steps:
    /// 1. Send transaction to Gateway Decryption contract
    /// 2. Extract user_decryption_id from receipt
    /// 3. Store receipt in database
    async fn send_delegated_user_decrypt_request(
        &self,
        event: RelayerEvent,
        decrypt_request: DelegatedUserDecryptRequest,
    ) -> Result<(), EventProcessingError> {
        info!(
            "Sending delegated user decrypt request to gateway for {}",
            event.job_id
        );

        let job_id_hash = decrypt_request.content_hash();

        let calldata_bytes =
            ComputeCalldata::delegated_user_decryption_req(decrypt_request.clone())?;

        self.send_to_gateway(calldata_bytes, job_id_hash).await?;
        info!(
            "Delegated user decryption request sent to the gateway for {}",
            event.job_id
        );
        Ok(())
    }

    /// Sends transactions to the Gateway Decryption contract.
    ///
    /// Returns the gateway reference ID (decryptionId) and transaction hash.
    async fn send_to_gateway(
        &self,
        calldata_bytes: Bytes,
        job_id_hash: [u8; 32],
    ) -> Result<(), EventProcessingError> {
        let decryption_address = self.decryption_address;

        let job_id = JobId::from(job_id_hash);

        let task = GatewayTxTask {
            id: hex::encode(job_id_hash), // Used for Queue tracking/dedup
            job_id,
            transaction_type: TransactionType::UserDecryptRequest, // Important to dispatch errors.
            target: decryption_address,
            calldata: calldata_bytes,
            hook: DynTxHook(Arc::new(self.clone())),
        };

        info!(
            step = %UserDecryptStep::TxQueued,
            int_job_id = %job_id,
            "Enqueuing user decrypt request to tx throttler"
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

    /// Decodes individual share response from Gateway event log.
    ///
    /// Extracts user decryption share and delegates to share storage logic.
    async fn decode_share_from_log(
        &self,
        log: &alloy::rpc::types::Log,
        event: RelayerEvent,
        tx_hash: TxHash,
    ) -> Result<(), EventProcessingError> {
        let user_decrypt_response = Decryption::UserDecryptionResponse::decode_log_data(log.data())
            .map_err(|e| {
                error!("Failed to decode UserDecryptionResponse event data: {}", e);
                EventProcessingError::EventDecodingFailed {
                    event_type: "UserDecryptionResponse".to_string(),
                    reason: e.to_string(),
                }
            })?;

        let user_decryption_id = user_decrypt_response.decryptionId;
        info!(
            step = %UserDecryptStep::ShareReceived,
            int_job_id = %event.job_id,
            tx_hash = %tx_hash,
            gw_reference_id = %user_decryption_id,
            share_index = %user_decrypt_response.indexShare,
            "Gateway share response received"
        );

        self.store_share_and_check_threshold(event, user_decrypt_response, tx_hash)
            .await
    }

    /// Stores individual share in database and atomically completes request if threshold is reached.
    ///
    /// Uses atomic transaction to prevent race conditions with timeout jobs.
    async fn store_share_and_check_threshold(
        &self,
        event: RelayerEvent,
        user_decrypt_response: Decryption::UserDecryptionResponse,
        tx_hash: TxHash,
    ) -> Result<(), EventProcessingError> {
        let user_decryption_id = user_decrypt_response.decryptionId;
        let threshold = self.user_decrypt_shares_threshold;

        let tx_hash_str = format!("{:?}", tx_hash);
        let params = ShareInsertParams {
            gw_reference_id: user_decryption_id,
            share_index: user_decrypt_response.indexShare,
            share: &hex::encode(&user_decrypt_response.userDecryptedShare),
            kms_signature: &hex::encode(&user_decrypt_response.signature),
            extra_data: &hex::encode(&user_decrypt_response.extraData),
            tx_hash: &tx_hash_str,
        };

        let outcome = self
            .user_decrypt_repo
            .insert_share_and_complete_if_threshold_reached(params, threshold)
            .await
            .map_err(|e| EventProcessingError::SqlOperationFailed {
                operation: "user_decrypt.insert_share_and_complete_if_threshold_reached"
                    .to_string(),
                reason: e.to_string(),
            })?;

        match outcome {
            ShareCompletionOutcome::Completed {
                count,
                metadata,
                shares,
            } => {
                info!(
                    step = %UserDecryptStep::ThresholdReached,
                    int_job_id = %event.job_id,
                    gw_reference_id = %user_decryption_id,
                    share_count = count,
                    threshold = threshold,
                    "Threshold reached and completion successful"
                );
                self.assemble_final_response(event, metadata, shares).await;
            }
            ShareCompletionOutcome::ThresholdNotReached { count } => {
                debug!(
                    int_job_id = %event.job_id,
                    gw_reference_id = %user_decryption_id,
                    share_count = count,
                    threshold = threshold,
                    "Threshold not yet reached, waiting for more shares"
                );
            }
            ShareCompletionOutcome::AlreadyCompleted {
                count,
                metadata,
                shares,
            } => {
                info!(
                    job_id = %event.job_id,
                    gw_reference_id = %user_decryption_id,
                    share_count = count,
                    threshold = threshold,
                    "Threshold reached but request already completed (duplicate share)"
                );
                // Request already completed - re-dispatch the response for any waiting HTTP handlers
                self.assemble_final_response(event, metadata, shares).await;
            }
            ShareCompletionOutcome::AlreadyInFinalState {
                count,
                current_status,
            } => {
                match current_status {
                    ReqStatus::Failure => {
                        debug!(
                            job_id = %event.job_id,
                            gw_reference_id = %user_decryption_id,
                            share_count = count,
                            threshold = threshold,
                            "Request already in failure state, skipping share"
                        );
                        // Already failed, no need to error again
                    }
                    ReqStatus::TimedOut => {
                        info!(
                            step = %UserDecryptStep::LateShareReceived,
                            int_job_id = %event.job_id,
                            gw_reference_id = %user_decryption_id,
                            share_count = count,
                            threshold = threshold,
                            "Late share arrival after timeout, skipping"
                        );
                        // Timeout handling already completed, user was notified.
                        // Late share arrival is expected in distributed systems - just skip.
                    }
                    other_status => {
                        error!(
                            job_id = %event.job_id,
                            gw_reference_id = %user_decryption_id,
                            current_status = ?other_status,
                            share_count = count,
                            threshold = threshold,
                            "Request in unexpected status - possible state corruption"
                        );
                        return Err(EventProcessingError::ShareAggregationFailed(format!(
                            "Cannot aggregate shares - request in unexpected status {:?}. \
                                 Shares should only arrive after ReceiptReceived status.",
                            other_status
                        )));
                    }
                }
            }
        }

        Ok(())
    }

    /// Assembles and dispatches final user decrypt response from collected shares.
    ///
    /// Steps:
    /// 1. Validate share count matches threshold
    /// 2. Assemble final response from all shares (hex decode, sort by index)
    /// 3. Dispatch response event to notify HTTP handler
    async fn assemble_final_response(
        &self,
        event: RelayerEvent,
        consensus_state: ConsensusReqState,
        shares: Vec<UserDecryptShare>,
    ) {
        let count = shares.len();
        let threshold = self.user_decrypt_shares_threshold as usize;

        // Validate share count matches threshold exactly (database LIMIT should ensure this)
        if shares.len() != threshold {
            error!(
                job_id = %event.job_id,
                got_count = %count,
                expected_count = %threshold,
                threshold = %self.user_decrypt_shares_threshold,
                "Number of shares not matching count"
            );
            for share in shares {
                error!(
                    share_gw_ref_id = ?share.gw_reference_id,
                    share_id = ?share.id,
                    "Share details for debugging count mismatch"
                );
            }
            self.notify_failed(
                event,
                EventProcessingError::ShareAggregationFailed(
                    "number of shares does not match expected count".to_string(),
                ),
            )
            .await;
            return;
        }

        // Assemble and dispatch final response
        match assemble_final_response(shares) {
            Ok(final_response) => {
                info!(
                    "Response assembled and sending to user for {:?}",
                    event.job_id
                );

                // Create JobId from the stored content hash (int_job_id database field)
                let int_job_id_hex = hex::encode(&consensus_state.int_job_id);
                let int_job_id_len = consensus_state.int_job_id.len();
                let job_id: JobId = match consensus_state.int_job_id.try_into() {
                    Ok(id) => id,
                    Err(_) => {
                        error!(
                            alert = true,
                            int_job_id_len,
                            int_job_id_hex,
                            "int_job_id has invalid length, expected 32 bytes, HTTP handler will timeout"
                        );
                        return;
                    }
                };

                let response_event_data =
                    RelayerEventData::UserDecrypt(UserDecryptEventData::RespRcvdFromGw {
                        decrypt_response: final_response,
                    });

                let response_event =
                    RelayerEvent::new(job_id, event.api_version, response_event_data);

                if let Err(e) = self.dispatcher.dispatch_event(response_event).await {
                    error!(?e, "Failed to dispatch response event to HTTP handlers");
                } else {
                    info!(
                        step = %UserDecryptStep::RespSent,
                        int_job_id = %job_id,
                        "Response dispatched to HTTP handlers"
                    );
                }
            }
            Err(hex_error) => {
                error!(
                    job_id = %event.job_id,
                    hex_error = %hex_error,
                    "Failed to decode hex data in shares"
                );
                self.notify_failed(
                    event,
                    EventProcessingError::ShareAggregationFailed(format!(
                        "failed to decode hex data in shares: {}",
                        hex_error
                    )),
                )
                .await;
            }
        }
    }

    /// Updates consensus hash in database when threshold event is received.
    ///
    /// Called when Gateway emits UserDecryptionResponseThresholdReached event.
    async fn update_consensus_hash(
        &self,
        log: &alloy::rpc::types::Log,
        _event: RelayerEvent,
        tx_hash: TxHash,
    ) {
        if let Some(decryption_id_topic) = log.topics().get(1) {
            let topic_bytes: [u8; 32] = match decryption_id_topic.as_slice().try_into() {
                Ok(bytes) => bytes,
                Err(_) => {
                    error!(
                        "Invalid decryption ID topic: expected 32 bytes, got {}",
                        decryption_id_topic.as_slice().len()
                    );
                    return;
                }
            };
            let user_decryption_id = U256::from_be_bytes(topic_bytes);

            info!(
                "Consensus event received for decryption ID {}",
                user_decryption_id
            );

            let tx_hash_str = format!("{:?}", tx_hash);

            match self
                .user_decrypt_repo
                .update_consensus_hash_and_return_state(user_decryption_id, &tx_hash_str)
                .await
            {
                Ok(Some(state)) => {
                    info!(
                        "Consensus hash updated for decryption ID {}, status: {:?}",
                        user_decryption_id, state.req_status
                    );
                }
                Ok(None) => {
                    error!(
                        "Failed to update consensus hash for decryption ID {}",
                        user_decryption_id
                    );
                }
                Err(e) => {
                    error!("Database error updating consensus hash: {}", e);
                }
            }
        } else {
            error!("UserDecryptionResponseThresholdReached event missing decryption_id topic");
        }
    }

    /// Returns event signature hash for UserDecryptionResponseThresholdReached event.
    fn get_consensus_event_topic(&self) -> FixedBytes<32> {
        Decryption::UserDecryptionResponseThresholdReached::SIGNATURE_HASH
    }

    /// Updates database status to "processing" after readiness check passes.
    async fn mark_processing(&self, job_id_hash: [u8; 32]) -> Result<(), EventProcessingError> {
        self.user_decrypt_repo
            .update_status_to_processing(&job_id_hash[..])
            .await
            .map(|_| ())
            .map_err(|e| EventProcessingError::SqlOperationFailed {
                operation: "user_decrypt.update_status_to_processing".to_string(),
                reason: e.to_string(),
            })
    }

    /// Handles errors during user decrypt processing.
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
                    handler_type = "user_decrypt",
                    "SQL operation failed"
                );
            }

            EventProcessingError::TransactionError(tx_error) => {
                error!(
                    job_id = %event.job_id,
                    error = ?tx_error,
                    "Transaction failed - status updated in helper, notifying user"
                );
            }

            EventProcessingError::ReadinessCheckTimedOut => {
                error!(
                    job_id = %event.job_id,
                    "Readiness check failed - updating database with timeout status"
                );

                if let RelayerEventData::UserDecrypt(
                    UserDecryptEventData::ReadinessCheckTimedOut {
                        ref decrypt_request,
                        ..
                    },
                ) = event.data
                {
                    let job_id_hash = decrypt_request.content_hash();

                    if let Err(db_err) = self
                        .user_decrypt_repo
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

                if let RelayerEventData::UserDecrypt(UserDecryptEventData::ReqRcvdFromUser {
                    ref decrypt_request,
                    ..
                }) = event.data
                {
                    let job_id_hash = decrypt_request.content_hash();
                    let err_reason = format!("Processing Failed: {}", error);

                    // TODO(mano): Review if nested error logging is necessary or can be simplified
                    if let Err(db_err) = self
                        .user_decrypt_repo
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

                if let RelayerEventData::UserDecrypt(UserDecryptEventData::ReadinessCheckFailed {
                    ref decrypt_request,
                    ..
                }) = event.data
                {
                    let job_id_hash = decrypt_request.content_hash();
                    let err_reason = format!("Processing Failed: {}", error);

                    // TODO(mano): Review if nested error logging is necessary or can be simplified
                    if let Err(db_err) = self
                        .user_decrypt_repo
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

                if let RelayerEventData::UserDecrypt(UserDecryptEventData::ReadinessCheckPassed {
                    ref decrypt_request,
                    ..
                }) = event.data
                {
                    let job_id_hash = decrypt_request.content_hash();
                    let err_reason = format!("Processing Failed: {}", error);

                    // TODO(mano): Review if nested error logging is necessary or can be simplified
                    if let Err(db_err) = self
                        .user_decrypt_repo
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
        let error_event = event.derive_next_event(RelayerEventData::UserDecrypt(
            UserDecryptEventData::Failed { error },
        ));

        if let Err(e) = self.dispatcher.dispatch_event(error_event).await {
            error!(?e, "Failed to dispatch error event");
        }
    }
}

#[async_trait]
impl TxLifecycleHooks for GatewayHandler {
    async fn on_tx_in_flight(&self, job_id: &JobId) -> Result<(), EventProcessingError> {
        self.user_decrypt_repo
            .update_status_to_tx_in_flight(&job_id[..])
            .await
            .map(|_| ())
            .map_err(|e| EventProcessingError::SqlOperationFailed {
                operation: "user_decrypt.update_status_to_tx_in_flight".to_string(),
                reason: e.to_string(),
            })
    }

    async fn on_receipt_received(
        &self,
        job_id: &JobId,
        receipt: &TxResult,
    ) -> Result<(), EventProcessingError> {
        let gw_reference_id = TransactionHelper::extract_gateway_id_from_receipt::<
            Decryption::UserDecryptionRequest,
        >(
            receipt,
            Decryption::UserDecryptionRequest::SIGNATURE_HASH,
            |event| event.decryptionId,
        )?;

        let tx_hash = format!("{:?}", receipt.transaction_hash);

        info!(
            step = %UserDecryptStep::TxConfirmed,
            int_job_id = %job_id,
            tx_hash = %tx_hash,
            gw_reference_id = %gw_reference_id,
            "Transaction confirmed, receipt received"
        );

        self.user_decrypt_repo
            .update_status_to_receipt_received_on_tx_success(&job_id[..], &tx_hash, gw_reference_id)
            .await
            .map(|_| ())
            .map_err(|e| EventProcessingError::SqlOperationFailed {
                operation: "user_decrypt.update_status_to_receipt_received_on_tx_success"
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
            crate::metrics::transaction::track_revert_with_request_type(reason, "user_decrypt");
        }

        self.user_decrypt_repo
            .update_status_to_failure_on_tx_failed(&job_id[..], err_reason)
            .await
            .map(|_| ())
            .map_err(|e| EventProcessingError::SqlOperationFailed {
                operation: "user_decrypt.update_status_to_failure_on_tx_failed".to_string(),
                reason: e.to_string(),
            })
    }
}

/// Assembles final UserDecryptResponse from individual shares.
///
/// Steps:
/// 1. Sort shares by index_share
/// 2. Hex decode all shares, signatures, and extra_data
/// 3. Construct final response with decryption_id from first share
fn assemble_final_response(shares: Vec<UserDecryptShare>) -> Result<UserDecryptResponse, String> {
    // Defensive check: should never occur since threshold >= 1 is validated at startup
    // and caller validates shares.len() == threshold before calling.
    if shares.is_empty() {
        return Err("assemble_final_response called with empty shares".to_string());
    }

    // Sort shares by index_share to maintain order
    let mut shares_vec: Vec<_> = shares.to_vec();
    shares_vec.sort_by_key(|share| share.share_index);

    let first_share = &shares_vec[0];
    let decryption_id = U256::from_be_slice(&first_share.gw_reference_id);

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

    // Use extra_data from first share as string
    let extra_data = first_share.extra_data.clone();

    Ok(UserDecryptResponse {
        gateway_request_id: decryption_id,
        reencrypted_shares,
        signatures,
        extra_data,
    })
}
