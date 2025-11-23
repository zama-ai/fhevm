use crate::{
    config::settings::ContractConfig,
    core::{
        errors::EventProcessingError,
        event::{
            GatewayChainEventData, InputProofEventData, InputProofResponse, RelayerEvent,
            RelayerEventData,
        },
        job_id::JobId,
    },
    gateway::arbitrum::transaction::{
        helper::TransactionType, ReceiptProcessor, TransactionHelper,
    },
    gateway::arbitrum::{bindings::InputVerification, ComputeCalldata},
    orchestrator::{
        traits::{Event, EventDispatcher, EventHandler},
        Orchestrator, TokioEventDispatcher,
    },
    store::sql::repositories::input_proof_repo::InputProofRepository,
};
use std::str::FromStr;

use alloy::{
    network::{AnyReceiptEnvelope, AnyTransactionReceipt, ReceiptResponse},
    primitives::{Address, Bytes, FixedBytes, U256},
    rpc::types::{Log, TransactionReceipt},
};

use alloy::sol_types::SolEvent;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{debug, error, info, instrument};
use uuid::Uuid;

struct InputVerificationReceiptProcessor {}

impl ReceiptProcessor for InputVerificationReceiptProcessor {
    type Output = U256;

    fn process(
        &self,
        receipt: &AnyTransactionReceipt,
    ) -> Result<Self::Output, EventProcessingError> {
        let receipt: TransactionReceipt<AnyReceiptEnvelope<Log>> = receipt.inner.clone();

        debug!(
            "Receipt details:\n\
                 Hash: {:?}\n\
                 Status: {}\n\
                 Number of logs: {}\n\
                 Block number: {:?}",
            receipt.transaction_hash,
            receipt.status(),
            receipt.inner.logs().len(),
            receipt.block_number
        );

        let target_topic = InputVerification::VerifyProofRequest::SIGNATURE_HASH;
        info!(
            "Looking for topic: {}",
            InputVerification::VerifyProofRequest::SIGNATURE
        );

        for log in receipt.inner.logs().iter() {
            if let Some(first_topic) = log.topics().first() {
                if first_topic == &target_topic {
                    return match InputVerification::VerifyProofRequest::decode_log_data(log.data())
                    {
                        Ok(event) => {
                            info!(
                                ?receipt.transaction_hash,
                                proof_id = ?event.zkProofId,
                                chain_id = ?event.contractChainId,
                                contract = ?event.contractAddress,
                                user = ?event.userAddress,
                                proof_size = event.ciphertextWithZKProof.len(),
                                "Decoded VerifyProofRequest event"
                            );
                            Ok(event.zkProofId)
                        }
                        Err(e) => {
                            error!(
                                ?receipt.transaction_hash,
                                error = ?e,
                                "Failed to decode VerifyProofRequest event"
                            );
                            Err(EventProcessingError::DecodingError(e.to_string()))
                        }
                    };
                }
            }
        }

        Err(EventProcessingError::HandlerError(
            "VerifyProofRequest event not found in receipt logs".into(),
        ))
    }
}

#[derive(Clone)]
pub struct GatewayHandler {
    dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    input_verification_id_to_request_id: Arc<dashmap::DashMap<U256, Vec<Uuid>>>,
    tx_helper: Arc<TransactionHelper>,
    contracts: ContractConfig,
    input_proof_repo: Arc<InputProofRepository>,
}

impl GatewayHandler {
    pub fn new(
        dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
        tx_helper: Arc<TransactionHelper>,
        contracts: ContractConfig,
        input_proof_repo: Arc<InputProofRepository>,
    ) -> Self {
        Self {
            dispatcher,
            tx_helper,
            input_verification_id_to_request_id: Arc::new(dashmap::DashMap::new()),
            contracts,
            input_proof_repo,
        }
    }

    async fn submit_input_proof_to_gateway(
        &self,
        event: RelayerEvent,
        req_data: InputProofEventData,
    ) {
        if let InputProofEventData::ReqRcvdFromUser {
            input_proof_request,
        } = req_data
        {
            info!(
                "Input request received. Making tx to gateway: chain_id : {:?},request_id: {:?}, contract: {:?}, user: {:?}",
                input_proof_request.contract_chain_id, event.job_id, input_proof_request.contract_address, input_proof_request.user_address
            );

            match self
                .send_input_verification_transaction(
                    input_proof_request.contract_chain_id,
                    input_proof_request.contract_address,
                    input_proof_request.user_address,
                    input_proof_request.ciphetext_with_zk_proof,
                    input_proof_request.extra_data,
                )
                .await
            {
                Ok(input_verification_id) => {
                    self.store_mapping_and_dispatch_success(event, input_verification_id)
                        .await;
                }
                Err(e) => {
                    self.dispatch_input_proof_error(event, e).await;
                }
            }
        }
    }

    #[instrument(skip_all, fields(event_type=%event.event_name(), job_id=%event.job_id()))]
    async fn store_mapping_and_dispatch_success(&self, event: RelayerEvent, zkproof_id: U256) {
        let mut zk_id_to_req_id = self
            .input_verification_id_to_request_id
            .entry(zkproof_id)
            .or_default();
        zk_id_to_req_id
            .value_mut()
            .push(match event.job_id().as_uuid_v7() {
                Some(uuid) => uuid,
                None => {
                    error!("JobId is not a UUID variant, cannot register duplicate");
                    return;
                }
            });

        info!(
            ?event.job_id,
            ?zkproof_id,
            "Stored mapping between input_verification ID and request ID"
        );

        let next_event = event.derive_next_event(RelayerEventData::InputProof(
            InputProofEventData::ReqSentToGw {
                gw_req_reference_id: zkproof_id,
            },
        ));

        if let Err(e) = self.dispatcher.dispatch_event(next_event).await {
            error!(?e, "Failed to dispatch RequestSentToGw event");
        }
    }

    #[instrument(skip_all, fields(event_type=%event.event_name(), job_id=%event.job_id()))]
    async fn dispatch_input_proof_error(&self, event: RelayerEvent, error: EventProcessingError) {
        error!(
            error = ?error,
            "Failed to process input request"
        );

        let error_event =
            event.derive_next_event(RelayerEventData::InputProof(InputProofEventData::Failed {
                error: EventProcessingError::TransactionError(format!(
                    "Input request failed: {error}"
                )),
            }));

        if let Err(e) = self.dispatcher.dispatch_event(error_event).await {
            error!(?e, "Failed to dispatch error event");
        }
    }

    async fn send_input_verification_transaction(
        &self,
        contract_chain_id: u64,
        contract_address: Address,
        user_address: Address,
        input_verification: Bytes,
        extra_data: Bytes,
    ) -> Result<U256, EventProcessingError> {
        let processor = InputVerificationReceiptProcessor {};

        let input_verification_address =
            Address::from_str(&self.contracts.input_verification_address).map_err(|_| {
                EventProcessingError::ConfigError(
                    crate::config::settings::AppConfigError::InvalidAddress(
                        "contracts.input_verification_address".to_owned(),
                    ),
                )
            })?;

        info!(
            "input_verification_address used for input request {:?}",
            input_verification_address
        );

        self.tx_helper
            .send_raw_transaction_sync(
                TransactionType::InputRequest,
                input_verification_address,
                || {
                    ComputeCalldata::verify_proof_req(
                        contract_chain_id,
                        contract_address,
                        user_address,
                        input_verification.clone(),
                        extra_data.clone(),
                    )
                },
                &processor,
            )
            .await
    }
    #[instrument(skip_all, fields(event_type=%event.event_name(), job_id=%event.job_id()))]
    async fn handle_gateway_response_log(&self, event: RelayerEvent) {
        info!(
            "Input response received. Return result to user {:?}",
            event.job_id,
        );

        if let RelayerEventData::GatewayChain(GatewayChainEventData::EventLogRcvd { log }) =
            &event.data
        {
            debug!(
                topics = ?log.topics().iter().map(hex::encode).collect::<Vec<_>>(),
                "Processing log data for input response"
            );

            match log.topic0() {
                Some(topic) => {
                    match *topic {
                        InputVerification::VerifyProofResponse::SIGNATURE_HASH => {
                            match InputVerification::VerifyProofResponse::decode_log_data(
                                log.data(),
                            ) {
                                Ok(request_event) => {
                                    info!(
                                        input_verification_id = ?request_event.zkProofId,
                                        handles = ?request_event.ctHandles,
                                        signatures = ?request_event.signatures,
                                        "Processing InputResponse event"
                                    );

                                    // FIXME: https://github.com/zama-ai/fhevm-relayer/issues/234
                                    info!("Wait half a second to make sure we receive and process the request ");
                                    info!("event before the current response one");
                                    info!(
                                        "This race conditions should not happen in a real scenario"
                                    );
                                    info!("Please REMOVE this SLEEP when using websocket instead");
                                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

                                    if let Some(entry) = self
                                        .input_verification_id_to_request_id
                                        .get(&request_event.zkProofId)
                                    {
                                        let original_request_ids = entry.value();

                                        info!(
                                            ?original_request_ids,
                                            ?request_event.zkProofId,
                                            "Found original request ID for input response"
                                        );

                                        let next_event_data: RelayerEventData =
                                            RelayerEventData::InputProof(
                                                InputProofEventData::RespRcvdFromGw {
                                                    input_proof_response: InputProofResponse {
                                                        handles: request_event.ctHandles,
                                                        signatures: request_event.signatures,
                                                    },
                                                },
                                            );

                                        let mut dispatch_set = tokio::task::JoinSet::new();
                                        for original_request_id in original_request_ids {
                                            let next_event = next_event_data.clone();
                                            let handler = self.clone();
                                            let id = *original_request_id;

                                            dispatch_set.spawn(async move {
                                                let next_event = RelayerEvent::new(
                                                    JobId::from_uuid_v7(id),
                                                    event.api_version,
                                                    next_event,
                                                );

                                                let _ = handler
                                                    .dispatcher
                                                    .dispatch_event(next_event)
                                                    .await;
                                            });
                                        }
                                        dispatch_set.join_all().await;
                                    }
                                }
                                Err(err) => {
                                    error!(?err, "Failed to decode InputRequest event");
                                }
                            }
                        }
                        InputVerification::RejectProofResponse::SIGNATURE_HASH => {
                            match InputVerification::RejectProofResponse::decode_log_data(
                                log.data(),
                            ) {
                                Ok(reject_proof_response) => {
                                    if let Some(entry) = self
                                        .input_verification_id_to_request_id
                                        .get(&reject_proof_response.zkProofId)
                                    {
                                        let original_request_ids = entry.value();

                                        info!(
                                            ?original_request_ids,
                                            ?reject_proof_response.zkProofId,
                                            "Found original request ID for input response"
                                        );

                                        let next_event_data: RelayerEventData =
                                            RelayerEventData::InputProof(
                                                InputProofEventData::Failed {
                                                    error: EventProcessingError::TransactionError(
                                                        "Rejected".to_string(),
                                                    ),
                                                },
                                            );

                                        let mut dispatch_set = tokio::task::JoinSet::new();
                                        for original_request_id in original_request_ids {
                                            let next_event = next_event_data.clone();
                                            let handler = self.clone();
                                            let id = *original_request_id;
                                            dispatch_set.spawn(async move {
                                                let next_event = RelayerEvent::new(
                                                    JobId::from_uuid_v7(id),
                                                    event.api_version,
                                                    next_event,
                                                );

                                                let _ = handler
                                                    .dispatcher
                                                    .dispatch_event(next_event)
                                                    .await;
                                            });
                                        }
                                        dispatch_set.join_all().await;
                                    }
                                }
                                Err(err) => {
                                    error!(?err, "Failed to decode InputRequest event");
                                }
                            }
                        }
                        _ => {}
                    }
                }
                None => {
                    info!("Not a input response event");
                }
            }
        } else {
            error!("Invalid event type received");
        }
    }
}

#[async_trait]
impl EventHandler<RelayerEvent> for GatewayHandler {
    #[instrument(skip_all, fields(event_type=%event.event_name(), job_id=%event.job_id()))]
    async fn handle_event(&self, event: RelayerEvent) {
        match &event.data {
            RelayerEventData::InputProof(input_event) => match input_event {
                InputProofEventData::ReqRcvdFromUser {
                    input_proof_request,
                } => {
                    info!("Processing input proof request {}", event.job_id);
                    let req_data = InputProofEventData::ReqRcvdFromUser {
                        input_proof_request: input_proof_request.clone(),
                    };
                    self.submit_input_proof_to_gateway(event, req_data).await;
                }
                InputProofEventData::ReqSentToGw {
                    gw_req_reference_id,
                } => {
                    info!(
                        ?gw_req_reference_id,
                        "Input request sent to gateway successfully"
                    );
                }
                InputProofEventData::RespRcvdFromGw {
                    input_proof_response,
                } => {
                    info!(
                        handles_count = input_proof_response.handles.len(),
                        signatures_count = input_proof_response.signatures.len(),
                        "Received gateway response, ready for HTTP handler"
                    );
                }
                InputProofEventData::Failed { error } => {
                    error!(?error, "Input request failed");
                }
            },

            RelayerEventData::GatewayChain(GatewayChainEventData::EventLogRcvd { ref log }) => {
                if let Some(topic0) = log.topic0() {
                    let topic0_fixed = FixedBytes::<32>::from_slice(topic0.as_slice());
                    let verify_proof_response_topic =
                        InputVerification::VerifyProofResponse::SIGNATURE_HASH;
                    let reject_proof_response_topic =
                        InputVerification::RejectProofResponse::SIGNATURE_HASH;

                    if topic0_fixed == verify_proof_response_topic
                        || topic0_fixed == reject_proof_response_topic
                    {
                        info!("Processing gateway response for request {}", event.job_id);
                        self.handle_gateway_response_log(event).await;
                    }
                };
            }
            _ => {}
        }
    }
}
