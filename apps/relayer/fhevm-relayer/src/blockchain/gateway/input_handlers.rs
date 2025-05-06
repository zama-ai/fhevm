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
    primitives::{Address, Bytes, FixedBytes, U256},
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

        let target_topic = InputVerification::VerifyProofRequest::SIGNATURE_HASH;
        info!(
            "Looking for topic: {}",
            InputVerification::VerifyProofRequest::SIGNATURE
        );

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

                                // FIXME: https://github.com/zama-ai/fhevm-relayer/issues/234
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
#[tokio::test]
async fn test_input_verification_request() -> Result<(), Box<dyn std::error::Error>> {
    use crate::blockchain::ethereum::ComputeCalldata;
    use crate::config::settings::Settings;
    use crate::transaction::sender::TransactionManager;
    use crate::transaction::TxConfig;
    use alloy::primitives::{Address, Bytes};
    use alloy::signers::{local::PrivateKeySigner, Signer};
    use std::str::FromStr;
    use std::sync::Arc;

    println!("\n========== INPUT VERIFICATION (ZK PROOF) TEST ==========\n");

    // Load configuration
    let settings = Settings::new().expect("Failed to load configuration");

    // Get network settings
    let gateway_settings = settings
        .get_network("gateway")
        .cloned()
        .expect("Failed to get gateway settings");

    println!("Network URL: {}", gateway_settings.http_url);
    println!("Chain ID: {}", gateway_settings.chain_id);

    // Test private key from environment variable or use default
    let private_key =
        std::env::var(&settings.transaction.private_key_gateway_env).unwrap_or_else(|_| {
            "9f5e213176c6d97cba246563083794ebeb8098c51dbcaf91e9f71a29db2ffd88".to_string()
        });
    let mut gateway_signer: PrivateKeySigner = private_key.parse()?;
    gateway_signer.set_chain_id(Some(gateway_settings.chain_id));

    // Create transaction manager
    println!("Setting up manager with configured private key...");
    let manager = TransactionManager::new(&gateway_settings.http_url, Arc::new(gateway_signer))
        .await
        .expect("Failed to create transaction manager");

    let sender_address = manager.sender_address();
    println!("Sender address: {:#x}", sender_address);

    // Get the input verification contract address from config
    let input_verification_address =
        Address::from_str(&settings.contracts.input_verification_address)
            .expect("Invalid input verification contract address");

    println!(
        "Target input verification contract: {:#x}",
        input_verification_address
    );

    // Check contract code
    println!("Checking contract state...");
    let code = manager
        .verify_contract_code(input_verification_address)
        .await
        .expect("Failed to verify contract code");
    println!("Contract code size: {} bytes", code.len());

    if code.is_empty() {
        println!(
            "⚠️ WARNING: No code at target address! The contract might be a proxy or not deployed."
        );
    }

    // Create test data for the input verification request
    println!("\nCreating test data for input verification...");

    // Target contract for the proof
    let target_contract_address = input_verification_address; // Using same address for simplicity
    let target_contract_chain_id = gateway_settings.chain_id;
    let user_address = sender_address;

    // Create dummy proof data - in a real scenario this would be actual ZK proof data
    // Note: This is just for testing - a real ZK proof would be generated properly
    let proof_data = vec![
        // Mock header (8 bytes)
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        // Mock proof body (variable length)
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
        0x00, // Mock ciphertext data (variable length)
        0xde, 0xad, 0xbe, 0xef, 0xca, 0xfe, 0xba, 0xbe, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd,
        0xef,
    ];

    let ciphertext_with_zk_proof = Bytes::from(proof_data);

    println!("Proof data size: {} bytes", ciphertext_with_zk_proof.len());
    println!("Target contract: {:#x}", target_contract_address);
    println!("Target chain ID: {}", target_contract_chain_id);
    println!("User address: {:#x}", user_address);

    // Generate calldata for the verification request
    let calldata = ComputeCalldata::verify_proof_req(
        target_contract_chain_id,
        target_contract_address,
        user_address,
        ciphertext_with_zk_proof,
    )
    .expect("Failed to prepare calldata");

    println!(
        "Calldata prepared: 0x{}...",
        hex::encode(&calldata[..std::cmp::min(64, calldata.len())])
    );
    println!("Total calldata length: {} bytes", calldata.len());

    // Simulate transaction first (dry run)
    println!("\nSimulating transaction call...");
    let simulation_result = manager
        .call_view(input_verification_address, calldata.clone())
        .await;
    match simulation_result {
        Ok(result) => {
            println!("Simulation successful!");
            if !result.is_empty() {
                println!("Result: 0x{}", hex::encode(&result));
            } else {
                println!("No return data (this is normal for many transactions)");
            }
        }
        Err(e) => {
            println!("Simulation failed: {}", e);
            println!("This indicates the transaction would likely revert.");
        }
    }

    // Estimate gas for the transaction
    println!("\nEstimating gas...");
    let gas_result = manager
        .estimate_gas(input_verification_address, calldata.clone(), None)
        .await;

    match gas_result {
        Ok(gas) => {
            println!("Gas estimation successful: {} gas units", gas);

            // Set up transaction config
            let mut config = TxConfig::from(settings.transaction.clone());

            // Override with our estimated gas (plus buffer)
            let gas_with_buffer = (gas as f64 * 1.2) as u64; // 20% buffer
            config.gas_limit = Some(gas_with_buffer);

            println!("Using gas limit: {} (added 20% buffer)", gas_with_buffer);

            // Send the transaction without asking for confirmation
            println!("Sending transaction...");
            match manager
                .send_transaction_and_wait(input_verification_address, calldata, Some(config))
                .await
            {
                Ok(receipt) => {
                    println!("\n✅ TRANSACTION SUCCESSFUL!");
                    println!("Transaction hash: {:#x}", receipt.transaction_hash);
                    println!("Block number: {}", receipt.block_number.unwrap_or_default());
                    println!("Gas used: {}", receipt.gas_used);
                    println!(
                        "Status: {}",
                        if receipt.status() {
                            "SUCCESS"
                        } else {
                            "FAILED"
                        }
                    );

                    // Look for the VerifyProofRequest event
                    println!("\nEvent logs ({}):", receipt.inner.logs().len());

                    for (i, log) in receipt.inner.logs().iter().enumerate() {
                        println!("Log #{}:", i + 1);

                        let topics: Vec<String> = log
                            .topics()
                            .iter()
                            .map(|t| format!("0x{}", hex::encode(t)))
                            .collect();

                        println!("  Topics: {:?}", topics);

                        // Safe way to handle log data
                        let data = log.data();
                        if !data.data.is_empty() {
                            // Only show first 64 bytes if data is longer
                            println!("  Data length: {} bytes", data.data.len());
                        } else {
                            println!("  Data: Empty");
                        }
                    }
                }
                Err(e) => {
                    println!("\n❌ TRANSACTION FAILED: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Gas estimation failed: {}", e);
            println!("This indicates the transaction would likely revert if sent.");
        }
    }

    Ok(())
}
