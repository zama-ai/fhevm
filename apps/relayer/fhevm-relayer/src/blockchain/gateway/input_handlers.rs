use crate::{
    blockchain::ethereum::{bindings::InputVerification, ComputeCalldata},
    config::settings::ContractConfig,
    core::{
        errors::EventProcessingError,
        event::{
            GenericEventData, InputProofEventData, InputProofResponse, RelayerEvent,
            RelayerEventData,
        },
    },
    orchestrator::{
        traits::{EventDispatcher, EventHandler},
        TokioEventDispatcher,
    },
    transaction::{ReceiptProcessor, TransactionHelper, TransactionService, TxConfig},
};
use std::str::FromStr;

use alloy::{
    primitives::{keccak256, Address, Bytes, FixedBytes, U256},
    rpc::types::TransactionReceipt,
};

use alloy_sol_types::SolEvent;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{debug, error, info};
use uuid::Uuid;

struct InputRequestProcessor {
    handler: Arc<GatewayHandler>,
}

impl ReceiptProcessor for InputRequestProcessor {
    type Output = U256;

    fn process(&self, receipt: &TransactionReceipt) -> Result<Self::Output, EventProcessingError> {
        self.handler
            .extract_input_verification_id_from_receipt(receipt)
    }
}

#[derive(Clone)]
pub struct GatewayHandler {
    dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
    input_verification_id_to_request_id: Arc<dashmap::DashMap<U256, Uuid>>,
    tx_helper: Arc<TransactionHelper>,
    contracts: ContractConfig,
}

impl GatewayHandler {
    pub fn new(
        dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
        tx_service: Arc<TransactionService>,
        tx_config: TxConfig,
        contracts: ContractConfig,
    ) -> Self {
        Self {
            dispatcher,
            tx_helper: Arc::new(TransactionHelper::new(tx_service, tx_config)),
            input_verification_id_to_request_id: Arc::new(dashmap::DashMap::new()),
            contracts,
        }
    }

    /// Sends an input request transaction to the gateway with ZK proof of knowledge.
    ///
    /// # Arguments
    /// * `event` - The [`RelayerEvent`] containing the request context
    /// * `req_data` - The [`InputEventData`] containing the request parameters:
    ///   - contract_chain_id: Chain ID of the target contract
    ///   - contract_address: Address of the target contract
    ///   - user_address: Address of the user making the request
    ///   - input_verification: Vector of bytes containing the ZK proof
    ///
    /// # State Changes
    /// On success, stores mapping between input_verification_id and request_id in input_verification_id_to_request_id
    async fn send_input_request_to_gateway(
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
                input_proof_request.contract_chain_id, event.request_id, input_proof_request. contract_address, input_proof_request.user_address
            );

            let self_clone = self.clone();
            let event_clone = event.clone();

            tokio::spawn(async move {
                match self_clone
                    .process_input_request(
                        input_proof_request.contract_chain_id,
                        input_proof_request.contract_address,
                        input_proof_request.user_address,
                        input_proof_request.ciphetext_with_zk_proof,
                    )
                    .await
                {
                    Ok(input_verification_id) => {
                        self_clone
                            .handle_successful_request(event_clone, input_verification_id)
                            .await;
                    }
                    Err(e) => {
                        self_clone.handle_failed_request(event_clone, e).await;
                    }
                }
            });
        }
    }

    /// Processes a successful input request by storing state and dispatching event.
    async fn handle_successful_request(&self, event: RelayerEvent, zkproof_id: U256) {
        self.input_verification_id_to_request_id
            .insert(zkproof_id, event.request_id);

        info!(
            ?event.request_id,
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

    /// Handles a failed input request by dispatching error event.
    async fn handle_failed_request(&self, event: RelayerEvent, error: EventProcessingError) {
        error!(
            error = ?error,
            "Failed to process input request"
        );

        let error_event =
            event.derive_next_event(RelayerEventData::InputProof(InputProofEventData::Failed {
                error: format!("Input request failed: {}", error),
            }));

        if let Err(e) = self.dispatcher.dispatch_event(error_event).await {
            error!(?e, "Failed to dispatch error event");
        }
    }

    /// Extracts input_verification ID from transaction receipt logs.
    ///
    /// Searches for the [`VerifyProofRequest`] event in the logs and decodes it to extract
    /// the zkProofId.
    ///
    /// # Arguments
    /// * `receipt` - The [`TransactionReceipt`] to process
    ///
    /// # Returns
    /// * `Ok(`[`U256`]`)` - The extracted input_verification ID
    /// * `Err(`[`EventProcessingError`]`)` - If event is not found or decoding fails
    fn extract_input_verification_id_from_receipt(
        &self,
        receipt: &TransactionReceipt,
    ) -> Result<U256, EventProcessingError> {
        // Event signature without indexed parameters
        let target_topic = keccak256("VerifyProofRequest(uint256,uint256,address,address,bytes)");

        debug!(
            "Receipt details:\n\
             Hash: {:?}\n\
             Status: {}\n\
             Gas used: {:?}\n\
             Number of logs: {}\n\
             Block number: {:?}",
            receipt.transaction_hash,
            receipt.status(),
            receipt.gas_used,
            receipt.inner.logs().len(),
            receipt.block_number
        );

        // Calculate expected topic
        let expected_topic = keccak256("VerifyProofRequest(uint256,uint256,address,address,bytes)");
        info!("Looking for topic: 0x{}", hex::encode(expected_topic));

        for log in receipt.inner.logs().iter() {
            if let Some(first_topic) = log.topics().first() {
                if first_topic == &target_topic {
                    return match InputVerification::VerifyProofRequest::decode_log_data(
                        log.data(),
                        false, // No indexed parameters in this event
                    ) {
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
                            Err(EventProcessingError::DecodingError(e))
                        }
                    };
                }
            }
        }

        Err(EventProcessingError::HandlerError(
            "VerifyProofRequest event not found in receipt logs".into(),
        ))
    }

    /// Processes input request by sending transaction to gateway.
    ///
    /// # Arguments
    /// * `contract_chain_id` - [`U256`] Chain ID of the target contract
    /// * `contract_address` - [`Address`] Address of the target contract
    /// * `user_address` - [`Address`] Address of the user making the request
    /// * `input_verification` - [`Vec<u8>`] ZK proof data
    ///
    /// # Returns
    /// * `Ok(`[`U256`]`)` - The input_verification ID from the transaction
    /// * `Err(`[`EventProcessingError`]`)` - If the transaction fails
    async fn process_input_request(
        &self,
        contract_chain_id: u64,
        contract_address: Address,
        user_address: Address,
        input_verification: Bytes,
    ) -> Result<U256, EventProcessingError> {
        let processor = InputRequestProcessor {
            handler: Arc::new(self.clone()),
        };

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
            .send_transaction(
                "input_request",
                input_verification_address,
                || {
                    ComputeCalldata::verify_proof_req(
                        contract_chain_id,
                        contract_address,
                        user_address,
                        input_verification.clone(),
                    )
                },
                &processor,
            )
            .await
    }

    /// Processes input response events from gateway.
    ///
    /// This function:
    /// 1. Extracts `input_verification_id` from the event
    /// 2. Retrieves original request ID using the `input_verification_id`
    /// 3. Creates and dispatches response event with mock handles and signatures
    ///
    /// # Arguments
    /// * `event` - The [`RelayerEvent`] containing the response data
    ///
    /// # State Access
    /// Reads from `input_verification_id_to_request_id` mapping
    ///
    /// # Events
    /// Dispatches [`RelayerEventData::Input`] with [`InputEventData::RespFromGw`]
    /// containing handles and signatures
    async fn handle_input_reponse_event_log(&self, event: RelayerEvent) {
        info!(
            "Input response received. Return result to user {:?}",
            event.request_id,
        );

        if let RelayerEventData::Generic(GenericEventData::EventLogFromGw { log }) = &event.data {
            // Log the raw data for debugging
            debug!(
                topics = ?log.topics().iter().map(hex::encode).collect::<Vec<_>>(),
                "Processing log data for input response"
            );

            match log.topic0() {
                Some(topic) => {
                    if topic == &InputVerification::VerifyProofResponse::SIGNATURE_HASH {
                        match InputVerification::VerifyProofResponse::decode_log_data(
                            log.data(),
                            true,
                        ) {
                            Ok(request_event) => {
                                info!(
                                    input_verification_id = ?request_event.zkProofId,
                                    handles = ?request_event.ctHandles,
                                    signatures = ?request_event.signatures,
                                    "Processing InputResponse event"
                                );

                                // FIXME: https://github.com/zama-ai/httpz-relayer/issues/234
                                info!("Wait half a second to make sure we receive and process the request ");
                                info!("event before the current response one");
                                info!("This race conditions should not happen in a real scenario");
                                info!("Please REMOVE this SLEEP when using websocket instead");
                                tokio::time::sleep(std::time::Duration::from_millis(500)).await;

                                // Use get_key_value to get both key and value, or use remove if you want to clean up
                                if let Some(entry) = self
                                    .input_verification_id_to_request_id
                                    .get(&request_event.zkProofId)
                                {
                                    let original_request_id = *entry.value(); // Dereference the Ref<Uuid>

                                    info!(
                                        ?original_request_id,
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

                                    // Now we can use original_request_id directly
                                    let next_event = RelayerEvent::new(
                                        original_request_id,
                                        event.api_version,
                                        next_event_data,
                                    );

                                    let _ = self.dispatcher.dispatch_event(next_event).await;
                                } else {
                                    error!(?request_event.zkProofId, "No matching request ID found for zkproof ID");
                                }
                            }
                            Err(e) => {
                                error!(?e, "Failed to decode InputRequest event");
                                // Err(EventProcessingError::DecodingError(e))
                            }
                        }
                    }
                }
                None => {
                    info!("Not a input response event");
                }
            }
        } else {
            error!("Invalid event type received");
            // Err(EventProcessingError::HandlerError(
            //     "Invalid event type received".into(),
            // ))
        }
    }

    async fn noop_handle_input_reponse_event_log(&self, _event: &RelayerEvent) {}
}

#[async_trait]
impl EventHandler<RelayerEvent> for GatewayHandler {
    async fn handle_event(&self, event: RelayerEvent) {
        match &event.data {
            // Borrow event.data instead of moving it
            RelayerEventData::InputProof(input_event) => {
                match input_event {
                    InputProofEventData::ReqRcvdFromUser {
                        input_proof_request,
                    } => {
                        // Create a new InputEventData with cloned values
                        let req_data = InputProofEventData::ReqRcvdFromUser {
                            input_proof_request: input_proof_request.clone(),
                        };
                        self.send_input_request_to_gateway(event, req_data).await;
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
                }
            }

            RelayerEventData::Generic(GenericEventData::EventLogFromGw { ref log }) => {
                if let Some(topic0) = log.topic0() {
                    if FixedBytes::<32>::from_slice(topic0.as_slice())
                        != InputVerification::VerifyProofResponse::SIGNATURE_HASH
                    {
                        debug!(
                            "Ignore this event: expected event: {:?}, received {} ",
                            log.topic0(),
                            InputVerification::VerifyProofResponse::SIGNATURE_HASH
                        );
                        self.noop_handle_input_reponse_event_log(&event).await;
                    } else {
                        self.handle_input_reponse_event_log(event).await;
                    }
                };
            }
            _ => {
                // Ignore other event types
                self.noop_handle_input_reponse_event_log(&event).await;
            }
        }
    }
}
