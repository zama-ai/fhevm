use crate::{
    core::{
        errors::EventProcessingError,
        event::{
            GatewayChainEventData, GatewayChainEventId, HandleContractPair, RelayerEvent,
            RelayerEventData, UserDecryptEventData, UserDecryptEventId, UserDecryptRequest,
            UserDecryptResponse,
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
    store::sql::{
        models::{
            req_status_enum_model::ReqStatus, user_decrypt_req_model::ConsensusReqState,
            user_decrypt_share_model::UserDecryptShare,
        },
        repositories::user_decrypt_repo::UserDecryptRepository,
    },
};
use alloy::primitives::{Address, Bytes, FixedBytes, TxHash, U256};
use alloy::sol_types::SolEvent;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{debug, error, info};

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
    tx_helper: Arc<TransactionHelper>,
    readiness_checker: Arc<ReadinessChecker>,
    decryption_address: Address,
    user_decrypt_repo: Arc<UserDecryptRepository>,
    user_decrypt_shares_threshold: i64,
}

impl GatewayHandler {
    pub fn new(
        dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
        tx_helper: Arc<TransactionHelper>,
        readiness_checker: Arc<ReadinessChecker>,
        decryption_address: Address,
        user_decrypt_shares_threshold: usize,
        user_decrypt_repo: Arc<UserDecryptRepository>,
    ) -> Arc<Self> {
        let handler = Arc::new(Self {
            dispatcher: Arc::clone(&dispatcher),
            tx_helper,
            readiness_checker,
            decryption_address,
            user_decrypt_repo,
            user_decrypt_shares_threshold: user_decrypt_shares_threshold as i64,
        });

        // Self-register for events
        dispatcher.register_handler(
            &[
                UserDecryptEventId::ReqRcvdFromUser.into(),
                UserDecryptEventId::ReqSentToGw.into(),
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
            RelayerEventData::UserDecrypt(UserDecryptEventData::ReqRcvdFromUser {
                ref decrypt_request,
                ..
            }) => {
                info!("Processing user decrypt request {}", event.job_id);

                let result = async {
                    self.check_readiness(decrypt_request).await?;
                    info!("Readiness validation passed for {}", event.job_id);

                    let job_id_hash = decrypt_request.content_hash();
                    self.mark_processing(job_id_hash).await?;

                    self.send_user_decrypt_request(event.clone(), decrypt_request.clone())
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
                    let topic0_fixed = FixedBytes::<32>::from_slice(topic0.as_slice());
                    let individual_response_topic =
                        Decryption::UserDecryptionResponse::SIGNATURE_HASH;
                    let consensus_topic = self.get_consensus_event_topic();

                    match topic0_fixed {
                        topic if topic == individual_response_topic => {
                            info!("Processing share response for request {}", event.job_id);
                            let result = self
                                .decode_share_from_log(log, event.clone(), *tx_hash)
                                .await;

                            if let Err(e) = result {
                                self.handle_error(event, e).await;
                            }
                        }
                        topic if topic == consensus_topic => {
                            info!("Processing consensus response for request {}", event.job_id);
                            self.update_consensus_hash(log, event.clone(), *tx_hash)
                                .await;
                        }
                        _ => {
                            debug!(
                                "Ignoring event: received topic {:?}, expected individual {:?} or consensus {:?}",
                                topic0_fixed, individual_response_topic, consensus_topic
                            );
                        }
                    }
                };
            }
            _ => {}
        }
    }
}

impl GatewayHandler {
    /// Validates that all ciphertext handles are ready and user is authorized for decryption.
    ///
    /// Checks if handles exist on fhevm and user has permission to decrypt them.
    async fn check_readiness(
        &self,
        user_decrypt_request: &UserDecryptRequest,
    ) -> Result<(), EventProcessingError> {
        let contract_pairs: Vec<_> = user_decrypt_request
            .ct_handle_contract_pairs
            .iter()
            .map(Decryption::CtHandleContractPair::from)
            .collect();

        match self
            .readiness_checker
            .check_user_decryption_readiness(
                user_decrypt_request.user_address,
                contract_pairs,
                user_decrypt_request.extra_data.clone(),
            )
            .await
        {
            Ok(()) => {
                info!("User readiness check passed");
                Ok(())
            }
            Err(ReadinessCheckError::Timeout) => {
                error!("User readiness check timed out");
                Err(EventProcessingError::ReadinessCheckFailed)
            }
            Err(ReadinessCheckError::ContractError(err)) => {
                error!("User readiness check contract error: {}", err);
                Err(EventProcessingError::ContractCallFailed(err.to_string()))
            }
        }
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
            "Sending user decrypt request to gateway for {}",
            event.job_id
        );

        let (user_decryption_id, tx_hash) = self.send_to_gateway(decrypt_request.clone()).await?;

        info!("User decrypt request sent to gateway for {}", event.job_id);
        self.store_request_receipt(decrypt_request, user_decryption_id, tx_hash)
            .await?;
        Ok(())
    }

    /// Sends user decryption transaction to Gateway Decryption contract.
    ///
    /// Returns the gateway reference ID (decryptionId) and transaction hash.
    async fn send_to_gateway(
        &self,
        user_decrypt_request: UserDecryptRequest,
    ) -> Result<(U256, TxHash), EventProcessingError> {
        let decryption_address = self.decryption_address;

        let receipt = self
            .tx_helper
            .send_raw_transaction_sync(
                TransactionType::UserDecryptRequest,
                decryption_address,
                || ComputeCalldata::user_decryption_req(user_decrypt_request.clone()),
            )
            .await?;

        // Extract gateway reference ID from the UserDecryptionRequest event
        let gw_reference_id = TransactionHelper::extract_gateway_id_from_receipt::<
            Decryption::UserDecryptionRequest,
        >(
            &receipt,
            Decryption::UserDecryptionRequest::SIGNATURE_HASH,
            |event| event.decryptionId,
        )?;

        Ok((gw_reference_id, receipt.transaction_hash))
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
            "Gateway response received for decryption ID {}, share index {}",
            user_decryption_id, user_decrypt_response.indexShare
        );

        self.store_share_and_check_threshold(event, user_decrypt_response, tx_hash)
            .await
    }

    /// Stores individual share in database and checks if threshold is reached.
    ///
    /// When share count equals threshold, triggers final response assembly.
    async fn store_share_and_check_threshold(
        &self,
        event: RelayerEvent,
        user_decrypt_response: Decryption::UserDecryptionResponse,
        tx_hash: TxHash,
    ) -> Result<(), EventProcessingError> {
        let user_decryption_id = user_decrypt_response.decryptionId;

        let tx_hash_str = format!("{:?}", tx_hash);
        let count = self
            .user_decrypt_repo
            .insert_share_and_return_count(
                user_decryption_id,
                user_decrypt_response.indexShare,
                &hex::encode(&user_decrypt_response.userDecryptedShare),
                &hex::encode(&user_decrypt_response.signature),
                &hex::encode(&user_decrypt_response.extraData),
                &tx_hash_str,
            )
            .await
            .map_err(|e| EventProcessingError::SqlOperationFailed {
                operation: "user_decrypt.insert_share_and_return_count".to_string(),
                reason: e.to_string(),
            })?;

        info!(
            "COUNT AFTER INSERT of index: {:?} {} for Gateway reference ID {}",
            user_decrypt_response.indexShare, count, user_decryption_id
        );

        if count == self.user_decrypt_shares_threshold {
            info!(
                "Count equals threshold {}, {}",
                count, self.user_decrypt_shares_threshold
            );
            self.handle_threshold_reached(event, user_decryption_id)
                .await?;
        } else {
            info!(
                "Count not equal to threshold, going forward {}, {}",
                count, self.user_decrypt_shares_threshold
            );
        }

        Ok(())
    }

    /// Handles threshold reached event by fetching shares and assembling final response.
    ///
    /// Validates request status and assembles final decryption response from all shares.
    async fn handle_threshold_reached(
        &self,
        event: RelayerEvent,
        user_decryption_id: U256,
    ) -> Result<(), EventProcessingError> {
        let (consensus_state, shares) = self
            .user_decrypt_repo
            .complete_req_and_get_shares_metadata(user_decryption_id)
            .await
            .map_err(|e| EventProcessingError::SqlOperationFailed {
                operation: "user_decrypt.complete_req_and_get_shares_metadata".to_string(),
                reason: e.to_string(),
            })?;

        info!(
            "fetched all shares. Status = {:?}",
            consensus_state.req_status
        );

        match consensus_state.req_status {
            ReqStatus::Completed => {
                self.assemble_final_response(event, consensus_state, shares)
                    .await;
                Ok(())
            }
            ReqStatus::TimedOut => {
                error!(
                    job_id = %event.job_id,
                    "User decrypt request timed out (response timed out)"
                );
                Err(EventProcessingError::ValidationFailed {
                    field: "request_status".to_string(),
                    reason: "request timed out waiting for response".to_string(),
                })
            }
            _ => {
                error!(
                    job_id = %event.job_id,
                    status = ?consensus_state.req_status,
                    "Unexpected state of requests"
                );
                Err(EventProcessingError::ValidationFailed {
                    field: "request_status".to_string(),
                    reason: "unexpected request state".to_string(),
                })
            }
        }
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

        // Validate share count matches threshold
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
                    "Response assembled and sending to user for {}",
                    event.job_id
                );

                // Create JobId from the stored content hash (int_indexer_id database field)
                let job_id = JobId::from_sha256_hash(
                    consensus_state
                        .int_indexer_id
                        .try_into()
                        .unwrap_or([0u8; 32]), // TODO(xyz): return an error
                );

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
                        "User decrypt response successfully sent for {}",
                        event.job_id
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
            let user_decryption_id = U256::from_be_bytes::<32>(
                decryption_id_topic
                    .as_slice()
                    .try_into()
                    .unwrap_or([0u8; 32]),
            );

            info!(
                "Consensus event received for decryption ID {}",
                user_decryption_id
            );

            let tx_hash_str = format!("{:?}", tx_hash);
            let _result = self
                .user_decrypt_repo
                .update_consensus_hash_and_return_state(user_decryption_id, &tx_hash_str);
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

    /// Stores transaction receipt in database after successful Gateway submission.
    ///
    /// Updates request status to "receipt_received" with gateway reference ID.
    async fn store_request_receipt(
        &self,
        decrypt_request: UserDecryptRequest,
        user_decryption_id: U256,
        tx_hash: TxHash,
    ) -> Result<(), EventProcessingError> {
        let job_id_hash = decrypt_request.content_hash();
        let tx_hash_str = format!("{:?}", tx_hash);
        self.user_decrypt_repo
            .update_status_to_receipt_received_on_tx_success(
                &job_id_hash[..],
                &tx_hash_str,
                user_decryption_id,
            )
            .await
            .map(|_| ())
            .map_err(|e| EventProcessingError::SqlOperationFailed {
                operation: "user_decrypt.update_status_to_receipt_received_on_tx_success"
                    .to_string(),
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
                    "Transaction failed - updating database and notifying user"
                );

                if let RelayerEventData::UserDecrypt(UserDecryptEventData::ReqRcvdFromUser {
                    ref decrypt_request,
                    ..
                }) = event.data
                {
                    let job_id_hash = decrypt_request.content_hash();
                    let err_reason = format!("Transaction Failed: {}", error);

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

/// Assembles final UserDecryptResponse from individual shares.
///
/// Steps:
/// 1. Sort shares by index_share
/// 2. Hex decode all shares, signatures, and extra_data
/// 3. Construct final response with decryption_id from first share
fn assemble_final_response(shares: Vec<UserDecryptShare>) -> Result<UserDecryptResponse, String> {
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

    // Use extra_data from first share with hex decoding
    let extra_data = match hex::decode(&first_share.extra_data) {
        Ok(decoded) => Bytes::from(decoded),
        Err(e) => return Err(format!("Failed to decode extra_data hex: {}", e)),
    };

    Ok(UserDecryptResponse {
        gateway_request_id: decryption_id,
        reencrypted_shares,
        signatures,
        extra_data,
    })
}
