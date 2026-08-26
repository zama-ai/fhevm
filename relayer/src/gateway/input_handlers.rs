use crate::{
    config::settings::{AppConfigError, ContractConfig, GwEventNotFoundRetryConfig},
    core::{
        errors::EventProcessingError,
        event::{
            GatewayChainEventData, GatewayChainEventId, InputProofEventData, InputProofEventId,
            InputProofRequest, InputProofResponse, RelayerEvent, RelayerEventData,
        },
        job_id::JobId,
    },
    gateway::{
        arbitrum::{
            bindings::InputVerification,
            transaction::{
                helper::{
                    ReceiptRecordOutcome, TransactionHelper, TransactionType, TxClaimOutcome,
                    TxResult,
                },
                tx_throttler::{DynTxHook, GatewayTxTask, TxThrottlingSender},
                TxLifecycleHooks,
            },
            ComputeCalldata,
        },
        utils::{classify_revert_selector, extract_revert_selector},
    },
    logging::InputProofStep,
    orchestrator::{
        traits::{Event, EventHandler},
        Orchestrator,
    },
    store::sql::{
        models::req_status_enum_model::ReqStatus,
        repositories::input_proof_repo::{InputProofCompletionOutcome, InputProofRepository},
    },
};
use std::str::FromStr;
use std::time::Duration;

use alloy::primitives::{Address, TxHash};

use alloy::sol_types::SolEvent;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{debug, error, info, instrument, warn};

#[derive(Clone)]
pub struct InputProofGatewayHandler {
    dispatcher: Arc<Orchestrator>,
    tx_throttler: TxThrottlingSender<GatewayTxTask>,
    contracts: ContractConfig,
    input_proof_repo: Arc<InputProofRepository>,
    gw_event_retry_config: GwEventNotFoundRetryConfig,
}

impl InputProofGatewayHandler {
    pub fn new(
        dispatcher: Arc<Orchestrator>,
        tx_throttler: TxThrottlingSender<GatewayTxTask>,
        contracts: ContractConfig,
        input_proof_repo: Arc<InputProofRepository>,
        gw_event_retry_config: GwEventNotFoundRetryConfig,
    ) -> Arc<Self> {
        let handler = Arc::new(Self {
            dispatcher: Arc::clone(&dispatcher),
            tx_throttler,
            contracts,
            input_proof_repo,
            gw_event_retry_config,
        });

        // Self-register for events
        dispatcher.register_handler(
            &[
                InputProofEventId::ReqRcvdFromUser.into(),
                InputProofEventId::ReqSentToGw.into(),
                InputProofEventId::RespRcvdFromGw.into(),
                // NOTE: We don't use Failed Event Id here, to allow notifying users
                InputProofEventId::InternalFailure.into(),
                GatewayChainEventId::VerifyProofResponse.into(),
                GatewayChainEventId::RejectProofResponse.into(),
            ],
            handler.clone() as Arc<dyn EventHandler<RelayerEvent>>,
        );

        handler
    }
}

#[async_trait]
impl EventHandler<RelayerEvent> for InputProofGatewayHandler {
    #[instrument(skip_all, fields(event_type=%event.event_name(), job_id=%event.job_id()))]
    async fn handle_event(&self, event: RelayerEvent) {
        let result = match &event.data {
            RelayerEventData::InputProof(input_event) => match input_event {
                InputProofEventData::ReqRcvdFromUser {
                    input_proof_request,
                } => {
                    info!("Processing input proof request {}", event.job_id);
                    self.send_input_proof_request(event.clone(), input_proof_request.clone())
                        .await
                }
                InputProofEventData::InternalFailure { error, .. } => Err(error.clone()),
                _ => {
                    warn!("unexpected event received in input handler");
                    return;
                }
            },

            RelayerEventData::GatewayChain(GatewayChainEventData::VerifyProofResponse {
                ref log,
                tx_hash,
            }) => {
                debug!(
                    step = %InputProofStep::GwEventReceived,
                    "Observed gateway input-proof accept response"
                );
                self.complete_proof_verification(event.clone(), log, *tx_hash)
                    .await
            }
            RelayerEventData::GatewayChain(GatewayChainEventData::RejectProofResponse {
                ref log,
                tx_hash,
            }) => {
                debug!(
                    step = %InputProofStep::GwEventReceived,
                    "Observed gateway input-proof reject response"
                );
                self.reject_proof_verification(event.clone(), log, *tx_hash)
                    .await
            }
            _ => return,
        };

        if let Err(e) = result {
            self.handle_error(event, e).await;
        }
    }
}

impl InputProofGatewayHandler {
    /// Processes user input proof request by sending it to the Gateway blockchain.
    ///
    /// Steps:
    /// 1. Send transaction to Gateway InputVerification contract
    /// 2. Extract input_verification_id from receipt
    /// 3. Store receipt in database
    async fn send_input_proof_request(
        &self,
        event: RelayerEvent,
        input_proof_request: InputProofRequest,
    ) -> Result<(), EventProcessingError> {
        info!(
            "Sending input proof request to gateway for {:?}",
            event.job_id
        );

        self.send_to_gateway(&input_proof_request, &event.job_id)
            .await?;
        info!("Input proof request sent to gateway for {}", event.job_id);
        Ok(())
    }

    /// Sends input proof verification transaction to Gateway InputVerification contract.
    ///
    /// Returns the gateway reference ID (zkProofId) and transaction hash.
    async fn send_to_gateway(
        &self,
        input_proof_request: &InputProofRequest,
        int_job_id_bytes: &[u8; 32],
    ) -> Result<(), EventProcessingError> {
        let input_verification_address =
            Address::from_str(&self.contracts.input_verification_address).map_err(|_| {
                EventProcessingError::ConfigError(AppConfigError::InvalidAddress(
                    "contracts.input_verification_address".to_owned(),
                ))
            })?;

        // PRE-CALCULATE CALLDATA
        let calldata_bytes = ComputeCalldata::verify_proof_req(
            input_proof_request.contract_chain_id,
            input_proof_request.contract_address,
            input_proof_request.user_address,
            input_proof_request.ciphetext_with_zk_proof.clone(),
            input_proof_request.extra_data.clone(),
        )?;

        // CONSTRUCT TASK
        let task = GatewayTxTask {
            id: hex::encode(int_job_id_bytes), // Used for Queue tracking/dedup
            job_id: JobId::from(*int_job_id_bytes),
            transaction_type: TransactionType::InputRequest,
            target: input_verification_address,
            calldata: calldata_bytes,
            // We clone self (cheap Arc clone) and wrap it
            hook: DynTxHook(Arc::new(self.clone())),
        };

        info!(
            step = %InputProofStep::TxQueued,
            int_job_id = %task.job_id,
            "Enqueuing input proof request to tx throttler"
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

    /// Processes accepted input proof response from Gateway.
    ///
    /// Steps:
    /// 1. Decode VerifyProofResponse event from log
    /// 2. Update database with handles and signatures
    /// 3. Dispatch response event to notify HTTP handler (only if newly completed)
    async fn complete_proof_verification(
        &self,
        event: RelayerEvent,
        log: &alloy::rpc::types::Log,
        tx_hash: TxHash,
    ) -> Result<(), EventProcessingError> {
        let request_event = InputVerification::VerifyProofResponse::decode_log_data(log.data())
            .map_err(|err| {
                error!(?err, "Failed to decode VerifyProofResponse event");
                EventProcessingError::EventDecodingFailed {
                    event_type: "VerifyProofResponse".to_string(),
                    reason: err.to_string(),
                }
            })?;

        debug!(
            step = %InputProofStep::GwEventReceived,
            tx_hash = %tx_hash,
            gw_reference_id = ?request_event.zkProofId,
            "Observed gateway input-proof accept response"
        );

        let input_proof_response = InputProofResponse {
            handles: request_event.ctHandles,
            signatures: request_event.signatures,
        };

        let tx_hash_str = format!("{:?}", tx_hash);
        let outcome = self
            .input_proof_repo
            .accept_and_complete_input_proof_req(
                request_event.zkProofId,
                input_proof_response.clone(),
                &tx_hash_str,
            )
            .await
            .map_err(|e| EventProcessingError::SqlOperationFailed {
                operation: "input_proof.accept_and_complete_input_proof_req".to_string(),
                reason: e.to_string(),
            })?;

        match outcome {
            InputProofCompletionOutcome::Completed { int_job_id } => {
                info!(
                    step = %InputProofStep::ProofAccepted,
                    int_job_id = %int_job_id,
                    gw_reference_id = ?request_event.zkProofId,
                    "Matched gateway response to request"
                );

                let next_event_data: RelayerEventData =
                    RelayerEventData::InputProof(InputProofEventData::RespRcvdFromGw {
                        accepted: true,
                        input_proof_response: Some(input_proof_response),
                    });

                let next_event = RelayerEvent::new(int_job_id, event.api_version, next_event_data);

                if let Err(e) = self.dispatcher.dispatch_event(next_event).await {
                    error!(?e, "Failed to dispatch input proof response event");
                } else {
                    info!(
                        step = %InputProofStep::RespSent,
                        int_job_id = %int_job_id,
                        "Response dispatched to HTTP handlers"
                    );
                }
            }
            InputProofCompletionOutcome::AlreadyCompleted { int_job_id } => {
                debug!(
                    int_job_id = %int_job_id,
                    "Input proof already completed (duplicate event), skipping"
                );
            }
            InputProofCompletionOutcome::AlreadyInFinalState {
                int_job_id,
                current_status,
            } => match current_status {
                ReqStatus::Failure => {
                    debug!(
                        int_job_id = %int_job_id,
                        "Input proof already in failure state, skipping accept event"
                    );
                }
                ReqStatus::TimedOut => {
                    debug!(
                        int_job_id = %int_job_id,
                        "Input proof already timed out (late accept event), skipping"
                    );
                }
                other_status => {
                    warn!(
                        int_job_id = %int_job_id,
                        current_status = ?other_status,
                        "Input proof in unexpected state, skipping accept event - possible race condition or late event"
                    );
                }
            },
            InputProofCompletionOutcome::NotFound => {
                // TEMPORARY FIX: Retry with delays - gateway event may have arrived before
                // gw_reference_id was stored (race condition during high RPC latency).
                // TODO: Replace with proper event buffering solution.
                let retry_config = &self.gw_event_retry_config;
                for attempt in 1..=retry_config.max_retries {
                    debug!(
                        step = %InputProofStep::GwEventRetrying,
                        gw_reference_id = ?request_event.zkProofId,
                        attempt = attempt,
                        max_retries = retry_config.max_retries,
                        "No request matched gateway reference id, retrying"
                    );

                    tokio::time::sleep(Duration::from_millis(retry_config.retry_delay_ms)).await;

                    let retry_outcome = self
                        .input_proof_repo
                        .accept_and_complete_input_proof_req(
                            request_event.zkProofId,
                            input_proof_response.clone(),
                            &tx_hash_str,
                        )
                        .await
                        .map_err(|e| EventProcessingError::SqlOperationFailed {
                            operation: "input_proof.accept_and_complete_input_proof_req"
                                .to_string(),
                            reason: e.to_string(),
                        })?;

                    match retry_outcome {
                        InputProofCompletionOutcome::Completed { int_job_id } => {
                            info!(
                                step = %InputProofStep::ProofAccepted,
                                int_job_id = %int_job_id,
                                gw_reference_id = ?request_event.zkProofId,
                                attempt = attempt,
                                "Matched gateway response to request on retry"
                            );

                            let next_event_data: RelayerEventData =
                                RelayerEventData::InputProof(InputProofEventData::RespRcvdFromGw {
                                    accepted: true,
                                    input_proof_response: Some(input_proof_response.clone()),
                                });

                            let next_event =
                                RelayerEvent::new(int_job_id, event.api_version, next_event_data);

                            if let Err(e) = self.dispatcher.dispatch_event(next_event).await {
                                error!(?e, "Failed to dispatch input proof response event");
                            } else {
                                info!(
                                    step = %InputProofStep::RespSent,
                                    int_job_id = %int_job_id,
                                    "Response dispatched to HTTP handlers"
                                );
                            }
                            return Ok(());
                        }
                        InputProofCompletionOutcome::AlreadyCompleted { int_job_id } => {
                            debug!(
                                int_job_id = %int_job_id,
                                attempt = attempt,
                                "Input proof already completed on retry"
                            );
                            return Ok(());
                        }
                        InputProofCompletionOutcome::AlreadyInFinalState {
                            int_job_id,
                            current_status,
                        } => {
                            debug!(
                                int_job_id = %int_job_id,
                                current_status = ?current_status,
                                attempt = attempt,
                                "Input proof in final state on retry"
                            );
                            return Ok(());
                        }
                        InputProofCompletionOutcome::NotFound => {
                            if attempt == retry_config.max_retries {
                                debug!(
                                    step = %InputProofStep::GwEventRetrying,
                                    gw_reference_id = ?request_event.zkProofId,
                                    max_retries = retry_config.max_retries,
                                    "No request matched gateway reference id; event ignored"
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

    /// Processes rejected input proof response from Gateway.
    ///
    /// Steps:
    /// 1. Decode RejectProofResponse event from log
    /// 2. Update database with rejection status
    /// 3. Dispatch rejection event to notify HTTP handler (only if newly completed)
    async fn reject_proof_verification(
        &self,
        event: RelayerEvent,
        log: &alloy::rpc::types::Log,
        tx_hash: TxHash,
    ) -> Result<(), EventProcessingError> {
        let reject_proof_response =
            InputVerification::RejectProofResponse::decode_log_data(log.data()).map_err(|err| {
                error!(?err, "Failed to decode RejectProofResponse event");
                EventProcessingError::EventDecodingFailed {
                    event_type: "RejectProofResponse".to_string(),
                    reason: err.to_string(),
                }
            })?;

        let outcome = self
            .input_proof_repo
            .reject_and_complete_input_proof_req(
                reject_proof_response.zkProofId,
                "Proof Rejected".to_string(),
                &format!("{:?}", tx_hash),
            )
            .await
            .map_err(|e| EventProcessingError::SqlOperationFailed {
                operation: "input_proof.reject_and_complete_input_proof_req".to_string(),
                reason: e.to_string(),
            })?;

        match outcome {
            InputProofCompletionOutcome::Completed { int_job_id } => {
                info!(
                    step = %InputProofStep::ProofRejected,
                    int_job_id = %int_job_id,
                    gw_reference_id = ?reject_proof_response.zkProofId,
                    "Matched gateway response to request"
                );

                let next_event_data: RelayerEventData =
                    RelayerEventData::InputProof(InputProofEventData::RespRcvdFromGw {
                        accepted: false,
                        input_proof_response: None,
                    });

                let next_event = RelayerEvent::new(int_job_id, event.api_version, next_event_data);

                if let Err(e) = self.dispatcher.dispatch_event(next_event).await {
                    error!(?e, "Failed to dispatch input proof rejection event");
                } else {
                    info!(
                        step = %InputProofStep::RespSent,
                        int_job_id = %int_job_id,
                        "Rejection response dispatched to HTTP handlers"
                    );
                }
            }
            InputProofCompletionOutcome::AlreadyCompleted { int_job_id } => {
                debug!(
                    int_job_id = %int_job_id,
                    "Input proof already completed (duplicate rejection event), skipping"
                );
            }
            InputProofCompletionOutcome::AlreadyInFinalState {
                int_job_id,
                current_status,
            } => match current_status {
                ReqStatus::Failure => {
                    debug!(
                        int_job_id = %int_job_id,
                        "Input proof already in failure state, skipping reject event"
                    );
                }
                ReqStatus::TimedOut => {
                    debug!(
                        int_job_id = %int_job_id,
                        "Input proof already timed out (late reject event), skipping"
                    );
                }
                other_status => {
                    warn!(
                        int_job_id = %int_job_id,
                        current_status = ?other_status,
                        "Input proof in unexpected state, skipping reject event - possible race condition or late event"
                    );
                }
            },
            InputProofCompletionOutcome::NotFound => {
                // TEMPORARY FIX: Retry with delays - gateway event may have arrived before
                // gw_reference_id was stored (race condition during high RPC latency).
                // TODO: Replace with proper event buffering solution.
                let retry_config = &self.gw_event_retry_config;
                for attempt in 1..=retry_config.max_retries {
                    debug!(
                        step = %InputProofStep::GwEventRetrying,
                        gw_reference_id = ?reject_proof_response.zkProofId,
                        attempt = attempt,
                        max_retries = retry_config.max_retries,
                        "No request matched gateway reference id, retrying"
                    );

                    tokio::time::sleep(Duration::from_millis(retry_config.retry_delay_ms)).await;

                    let retry_outcome = self
                        .input_proof_repo
                        .reject_and_complete_input_proof_req(
                            reject_proof_response.zkProofId,
                            "Proof Rejected".to_string(),
                            &format!("{:?}", tx_hash),
                        )
                        .await
                        .map_err(|e| EventProcessingError::SqlOperationFailed {
                            operation: "input_proof.reject_and_complete_input_proof_req"
                                .to_string(),
                            reason: e.to_string(),
                        })?;

                    match retry_outcome {
                        InputProofCompletionOutcome::Completed { int_job_id } => {
                            info!(
                                step = %InputProofStep::ProofRejected,
                                int_job_id = %int_job_id,
                                gw_reference_id = ?reject_proof_response.zkProofId,
                                attempt = attempt,
                                "Matched gateway response to request on retry"
                            );

                            let next_event_data: RelayerEventData =
                                RelayerEventData::InputProof(InputProofEventData::RespRcvdFromGw {
                                    accepted: false,
                                    input_proof_response: None,
                                });

                            let next_event =
                                RelayerEvent::new(int_job_id, event.api_version, next_event_data);

                            if let Err(e) = self.dispatcher.dispatch_event(next_event).await {
                                error!(?e, "Failed to dispatch input proof rejection event");
                            } else {
                                info!(
                                    step = %InputProofStep::RespSent,
                                    int_job_id = %int_job_id,
                                    "Rejection response dispatched to HTTP handlers"
                                );
                            }
                            return Ok(());
                        }
                        InputProofCompletionOutcome::AlreadyCompleted { int_job_id } => {
                            debug!(
                                int_job_id = %int_job_id,
                                attempt = attempt,
                                "Input proof already completed on retry (rejection)"
                            );
                            return Ok(());
                        }
                        InputProofCompletionOutcome::AlreadyInFinalState {
                            int_job_id,
                            current_status,
                        } => {
                            debug!(
                                int_job_id = %int_job_id,
                                current_status = ?current_status,
                                attempt = attempt,
                                "Input proof in final state on retry (rejection)"
                            );
                            return Ok(());
                        }
                        InputProofCompletionOutcome::NotFound => {
                            if attempt == retry_config.max_retries {
                                debug!(
                                    step = %InputProofStep::GwEventRetrying,
                                    gw_reference_id = ?reject_proof_response.zkProofId,
                                    max_retries = retry_config.max_retries,
                                    "No request matched gateway reference id; event ignored"
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

    /// Handles errors during input proof processing.
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
                    handler_type = "input_proof",
                    "SQL operation failed"
                );
            }

            EventProcessingError::TransactionError(tx_error) => {
                error!(
                    job_id = %event.job_id,
                    error = ?tx_error,
                    "Transaction failed - helper updated tx to error, notifying user"
                );
            }

            _ => {
                error!(
                    job_id = %event.job_id,
                    error = ?error,
                    "Request processing failed - notifying user"
                );

                match self
                    .input_proof_repo
                    .update_status_to_failure(event.job_id.as_ref(), &error.to_string())
                    .await
                {
                    Ok(0) => info!(
                        job_id = %event.job_id,
                        "failure write refused (stale epoch or status already advanced)"
                    ),
                    Ok(_) => {}
                    Err(db_err) => error!(
                        alert = true,
                        job_id = %event.job_id,
                        db_error = %db_err,
                        "Failed to update failure status in database"
                    ),
                }
            }
        }

        self.notify_failed(event, error).await;
    }

    /// Dispatches failure event to notify waiting HTTP handlers.
    #[instrument(skip_all, fields(event_type=%event.event_name(), job_id=%event.job_id()))]
    async fn notify_failed(&self, event: RelayerEvent, error: EventProcessingError) {
        let error_event =
            event.derive_next_event(RelayerEventData::InputProof(InputProofEventData::Failed {
                error,
            }));

        if let Err(e) = self.dispatcher.dispatch_event(error_event).await {
            error!(?e, "Failed to dispatch error event");
        }
    }
}

#[async_trait]
impl TxLifecycleHooks for InputProofGatewayHandler {
    async fn on_tx_in_flight(
        &self,
        job_id: &JobId,
    ) -> Result<TxClaimOutcome, EventProcessingError> {
        let rows_affected = self
            .input_proof_repo
            .update_status_to_tx_in_flight(job_id.as_ref())
            .await
            .map_err(|e| EventProcessingError::SqlOperationFailed {
                operation: "input_proof.update_status_to_tx_in_flight".to_string(),
                reason: e.to_string(),
            })?;

        if rows_affected == 0 {
            info!(
                int_job_id = %job_id,
                "Tx-in-flight claim did not apply for input proof request, skipping send"
            );
            return Ok(TxClaimOutcome::ClaimLost);
        }

        Ok(TxClaimOutcome::Claimed)
    }

    async fn on_receipt_received(
        &self,
        job_id: &JobId,
        receipt: &TxResult,
    ) -> Result<ReceiptRecordOutcome, EventProcessingError> {
        let gw_reference_id = TransactionHelper::extract_gateway_id_from_receipt::<
            InputVerification::VerifyProofRequest,
        >(
            receipt,
            InputVerification::VerifyProofRequest::SIGNATURE_HASH,
            |event| event.zkProofId,
        )?;

        let tx_hash = format!("{:?}", receipt.transaction_hash);

        info!(
            step = %InputProofStep::TxConfirmed,
            int_job_id = %job_id,
            tx_hash = %tx_hash,
            gw_reference_id = %gw_reference_id,
            "Transaction confirmed, receipt received"
        );

        let rows_affected = self
            .input_proof_repo
            .update_input_proof_status_to_receipt_received(
                job_id.as_ref(),
                &tx_hash,
                gw_reference_id,
            )
            .await
            .map_err(|e| EventProcessingError::SqlOperationFailed {
                operation: "input_proof.update_input_proof_status_to_receipt_received".to_string(),
                reason: e.to_string(),
            })?;

        if rows_affected == 0 {
            // This pod sent the transaction (it won `on_tx_in_flight`'s claim), but the write
            // requires both `req_status = 'tx_in_flight'` and the epoch fence, and either can
            // be why it lost: a successor's claim (a *different*, newer epoch) resets a claimed
            // `tx_in_flight` row to `processing` in the same statement that takes it, so the
            // row is no longer `tx_in_flight` by the time we get here. Or, within this pod's
            // *own* epoch, its own later sweep tick reclaimed and reset this same row
            // (build-order step 8's `claim_after` staleness case) while this send was still
            // genuinely in flight - no epoch changed at all, just the status raced out from
            // under it. Either way `gw_reference_id` is NOT stored: this must not read as
            // success, since the gateway-event listener keys off it to complete the request.
            warn!(
                int_job_id = %job_id,
                gw_reference_id = %gw_reference_id,
                "receipt_received write refused (row no longer tx_in_flight under this epoch - \
                 a successor's claim or this pod's own reclaim got there first); \
                 gw_reference_id not stored, a re-drive is expected to record its own receipt"
            );
            return Ok(ReceiptRecordOutcome::Refused);
        }

        Ok(ReceiptRecordOutcome::Recorded)
    }

    async fn on_failure(
        &self,
        job_id: &JobId,
        err_reason: &str,
    ) -> Result<(), EventProcessingError> {
        // Only track revert metrics if we can extract a selector (means it's actually a revert)
        if let Some(selector) = extract_revert_selector(err_reason) {
            let reason = classify_revert_selector(&selector);
            crate::metrics::transaction::track_revert_with_request_type(reason, "input_proof");
        }

        let rows_affected = self
            .input_proof_repo
            .update_status_to_failure(job_id.as_ref(), err_reason)
            .await
            .map_err(|e| EventProcessingError::SqlOperationFailed {
                operation: "input_proof.update_status_to_failure".to_string(),
                reason: e.to_string(),
            })?;

        if rows_affected == 0 {
            info!(
                int_job_id = %job_id,
                "failure write refused (stale epoch or status already advanced)"
            );
        }

        Ok(())
    }
}
