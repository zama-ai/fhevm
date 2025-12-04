use crate::{
    core::{
        errors::EventProcessingError,
        event::{
            GatewayChainEventData, HandleContractPair, RelayerEvent, RelayerEventData,
            UserDecryptEventData, UserDecryptRequest, UserDecryptResponse,
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
    ) -> Self {
        Self {
            dispatcher,
            tx_helper,
            readiness_checker,
            decryption_address,
            user_decrypt_repo,
            user_decrypt_shares_threshold: user_decrypt_shares_threshold as i64,
        }
    }

    // Request handlers

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

    async fn mark_processing(&self, event: RelayerEvent, job_id_hash: [u8; 32]) {
        if let Err(e) = self
            .user_decrypt_repo
            .update_status_to_processing(&job_id_hash[..])
            .await
        {
            sql_errors::user_decrypt_sql_error(
                &self.dispatcher,
                event,
                "user_decrypt.update_status_to_processing",
                &e,
                Some(&job_id_hash),
            )
            .await;
        }
    }

    async fn send_user_decrypt_request(
        &self,
        event: RelayerEvent,
        decrypt_request: UserDecryptRequest,
    ) {
        info!(
            "Sending user decrypt request to gateway for {}",
            event.job_id
        );

        match self.send_to_gateway(decrypt_request.clone()).await {
            Ok((user_decryption_id, tx_hash)) => {
                info!("User decrypt request sent to gateway for {}", event.job_id);
                self.store_request_receipt(event, decrypt_request, user_decryption_id, tx_hash)
                    .await;
            }
            Err(e) => {
                self.handle_transaction_failure(event, decrypt_request, e)
                    .await;
            }
        }
    }

    async fn store_request_receipt(
        &self,
        event: RelayerEvent,
        decrypt_request: UserDecryptRequest,
        user_decryption_id: U256,
        tx_hash: TxHash,
    ) {
        let job_id_hash = decrypt_request.content_hash();
        let tx_hash_str = format!("{:?}", tx_hash);
        if let Err(e) = self
            .user_decrypt_repo
            .update_status_to_receipt_received_on_tx_success(
                &job_id_hash[..],
                &tx_hash_str,
                user_decryption_id,
            )
            .await
        {
            sql_errors::user_decrypt_sql_error(
                &self.dispatcher,
                event,
                "user_decrypt.update_status_to_receipt_received_on_tx_success",
                &e,
                Some(&job_id_hash),
            )
            .await;
        }
    }

    async fn handle_transaction_failure(
        &self,
        event: RelayerEvent,
        decrypt_request: UserDecryptRequest,
        error: EventProcessingError,
    ) {
        let job_id_hash = decrypt_request.content_hash();
        let err_reason = format!("Transaction Failed: {}", error);
        if let Err(sql_error) = self
            .user_decrypt_repo
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

        self.notify_failed(event, error).await;
    }

    // Transaction operations

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

    async fn decode_share_from_log(
        &self,
        log: &alloy::rpc::types::Log,
        event: RelayerEvent,
        tx_hash: TxHash,
    ) {
        match Decryption::UserDecryptionResponse::decode_log_data(log.data()) {
            Ok(user_decrypt_response) => {
                let user_decryption_id = user_decrypt_response.decryptionId;
                info!(
                    "Gateway response received for decryption ID {}, share index {}",
                    user_decryption_id, user_decrypt_response.indexShare
                );

                self.store_share_and_check_threshold(event, user_decrypt_response, tx_hash)
                    .await;
            }
            Err(e) => {
                error!("Failed to decode UserDecryptionResponse event data: {}", e);
                self.notify_failed(
                    event,
                    EventProcessingError::EventDecodingFailed {
                        event_type: "UserDecryptionResponse".to_string(),
                        reason: e.to_string(),
                    },
                )
                .await;
            }
        }
    }

    async fn store_share_and_check_threshold(
        &self,
        event: RelayerEvent,
        user_decrypt_response: Decryption::UserDecryptionResponse,
        tx_hash: TxHash,
    ) {
        let user_decryption_id = user_decrypt_response.decryptionId;

        let tx_hash_str = format!("{:?}", tx_hash);
        match self
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
        {
            Ok(count) => {
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
                        .await;
                } else {
                    info!(
                        "Count not equal to threshold, going forward {}, {}",
                        count, self.user_decrypt_shares_threshold
                    );
                }
            }
            Err(sql_error) => {
                sql_errors::user_decrypt_sql_error(
                    &self.dispatcher,
                    event,
                    "user_decrypt.insert_share_and_return_count",
                    &sql_error,
                    None,
                )
                .await;
            }
        }
    }

    async fn handle_threshold_reached(&self, event: RelayerEvent, user_decryption_id: U256) {
        match self
            .user_decrypt_repo
            .complete_req_and_get_shares_metadata(user_decryption_id)
            .await
        {
            Ok((consensus_state, shares)) => {
                info!(
                    "fetched all shares. Status = {:?}",
                    consensus_state.req_status
                );

                match consensus_state.req_status {
                    ReqStatus::Completed => {
                        self.assemble_final_response(event, consensus_state, shares)
                            .await;
                    }
                    ReqStatus::TimedOut => {
                        error!(
                            job_id = %event.job_id,
                            "User decrypt request timed out (response timed out)"
                        );
                        self.notify_failed(
                            event,
                            EventProcessingError::ValidationFailed {
                                field: "request_status".to_string(),
                                reason: "request timed out waiting for response".to_string(),
                            },
                        )
                        .await;
                    }
                    _ => {
                        error!(
                            job_id = %event.job_id,
                            status = ?consensus_state.req_status,
                            "Unexpected state of requests"
                        );
                        self.notify_failed(
                            event,
                            EventProcessingError::ValidationFailed {
                                field: "request_status".to_string(),
                                reason: "unexpected request state".to_string(),
                            },
                        )
                        .await;
                    }
                }
            }
            Err(sql_error) => {
                sql_errors::user_decrypt_sql_error(
                    &self.dispatcher,
                    event,
                    "user_decrypt.complete_req_and_get_shares_metadata",
                    &sql_error,
                    None,
                )
                .await;
            }
        }
    }

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
                println!("Share: {:?} {:?}", share.gw_reference_id, share.id);
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

    fn get_consensus_event_topic(&self) -> FixedBytes<32> {
        Decryption::UserDecryptionResponseThresholdReached::SIGNATURE_HASH
    }

    // Event dispatching

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
impl EventHandler<RelayerEvent> for GatewayHandler {
    async fn handle_event(&self, event: RelayerEvent) {
        match event.data {
            RelayerEventData::UserDecrypt(UserDecryptEventData::ReqRcvdFromUser {
                ref decrypt_request,
                ..
            }) => {
                info!("Processing user decrypt request {}", event.job_id);
                let job_id_hash = decrypt_request.content_hash();
                let decrypt_request_clone = decrypt_request.clone();

                // Stage 1: Check readiness (ReadinessChecker component)
                match self.check_readiness(&decrypt_request_clone).await {
                    Ok(()) => {
                        info!("Readiness validation passed for {}", event.job_id);

                        // Stage 2: Update SQL status to processing
                        self.mark_processing(event.clone(), job_id_hash).await;

                        // Stage 3: Send to gateway (pure transaction execution)
                        self.send_user_decrypt_request(event, decrypt_request_clone)
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
                            self.decode_share_from_log(log, event.clone(), tx_hash)
                                .await;
                        }
                        topic if topic == consensus_topic => {
                            info!("Processing consensus response for request {}", event.job_id);
                            self.update_consensus_hash(log, event.clone(), tx_hash)
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
