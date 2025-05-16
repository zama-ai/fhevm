use crate::{
    blockchain::ethereum::{
        bindings::Decryption::{self, UserDecryptionRequest},
        ComputeCalldata,
    },
    config::settings::{ContractConfig, RetrySettings},
    core::{
        errors::EventProcessingError,
        event::{
            GenericEventData, HandleContractPair, RelayerEvent, RelayerEventData,
            UserDecryptEventData, UserDecryptRequest, UserDecryptResponse,
        },
    },
    orchestrator::{
        traits::{EventDispatcher, EventHandler},
        Orchestrator, TokioEventDispatcher,
    },
    transaction::{ReceiptProcessor, TransactionHelper, TransactionService, TxConfig},
};

impl From<&HandleContractPair> for Decryption::CtHandleContractPair {
    fn from(pair: &HandleContractPair) -> Self {
        Self {
            ctHandle: pair.ct_handle.into(),
            contractAddress: pair.contract_address,
        }
    }
}
use reqwest::Url;
use std::str::FromStr;
use std::time::Duration;

use alloy::{
    network::{AnyReceiptEnvelope, AnyTransactionReceipt, ReceiptResponse},
    primitives::{Address, FixedBytes, U256},
    providers::ProviderBuilder,
    rpc::types::{Log, TransactionReceipt},
};

use alloy::sol_types::SolEvent;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::task;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

struct UserDecryptionRequestProcessor {
    handler: Arc<GatewayHandler>,
}

impl ReceiptProcessor for UserDecryptionRequestProcessor {
    type Output = U256;

    fn process(
        &self,
        receipt: &AnyTransactionReceipt,
    ) -> Result<Self::Output, EventProcessingError> {
        self.handler
            .extract_user_decryption_id_from_receipt(receipt)
    }
}

#[derive(Clone)]
pub struct GatewayHandler {
    dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    user_decryption_id_to_request_id: Arc<dashmap::DashMap<U256, Uuid>>,
    tx_helper: Arc<TransactionHelper>,
    contracts: ContractConfig,
    gateway_http_url: String,
    retry_config: RetrySettings,
}

impl GatewayHandler {
    pub fn new(
        dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
        tx_service: Arc<TransactionService>,
        tx_config: TxConfig,
        contracts: ContractConfig,
        gateway_http_url: String,
        retry_config: RetrySettings,
    ) -> Self {
        Self {
            dispatcher,
            tx_helper: Arc::new(TransactionHelper::new(tx_service, tx_config)),
            user_decryption_id_to_request_id: Arc::new(dashmap::DashMap::new()),
            contracts,
            gateway_http_url,
            retry_config,
        }
    }

    /// Prepares and sends a decryption request transaction to the gateway.
    ///
    /// This function performs the following:
    /// 1. Converts the input handles to [`Uint<256, 4>`]
    /// 2. Sends transaction to the [`Decryption`] contract
    /// 3. Extracts the `decryption_public_id` from the receipt
    ///
    /// # Arguments
    /// * `event` - The [`RelayerEvent`] containing the request context and original request ID
    /// * `handles` - Vector of 32-byte arrays representing the encrypted handles to be decrypted
    ///
    /// # State Changes
    /// On success, stores mapping between `decryption_public_id` and the original request ID
    ///
    /// # Events
    /// * Success: [`RelayerEventData::DecryptionRequestSentToGw`]
    /// * Failure: [`RelayerEventData::DecryptionFailed`]
    async fn send_user_decryption_request_to_gateway(
        &self,
        event: RelayerEvent,
        user_decrypt_request: UserDecryptRequest,
    ) {
        info!(
            "User Decryption request received. Making a tx to gateway: request_id: {:?} with user request {:?}",
            event.request_id,
            user_decrypt_request
        );

        let self_clone = self.clone();
        let event_clone = event.clone();

        // Spawn a blocking task to make a transaction to gateway
        task::spawn(async move {
            match self_clone
                .process_user_decryption_request(user_decrypt_request)
                .await
            {
                Ok(user_decryption_id) => {
                    self_clone
                        .handle_successful_user_decryption_request(event_clone, user_decryption_id)
                        .await;
                }
                Err(e) => {
                    self_clone.handle_failed_request(event_clone, e).await;
                }
            }
        });
    }

    /// Processes a successful decryption request.
    ///
    /// # Arguments
    /// * `event` - The original [`RelayerEvent`] containing request information
    /// * `decryption_public_id` - The [`U256`] ID received from the decryption request
    ///
    /// # State Changes
    /// Stores mapping in `decryption_id_to_request_id`
    ///
    /// # Events
    /// Dispatches [`RelayerEventData::DecryptionRequestSentToGw`]
    async fn handle_successful_user_decryption_request(
        &self,
        event: RelayerEvent,
        user_decryption_id: U256,
    ) {
        // Store the mapping
        self.user_decryption_id_to_request_id
            .insert(user_decryption_id, event.request_id);

        info!(
            ?event.request_id,
            ?user_decryption_id,
            "Stored mapping between decryption ID and request ID"
        );

        info!("User decryption request sent to gateway");

        // Create and dispatch the new event
        let next_event = event.derive_next_event(RelayerEventData::UserDecrypt(
            UserDecryptEventData::ReqSentToGw {
                gw_req_reference_id: user_decryption_id,
            },
        ));

        if let Err(e) = self.dispatcher.dispatch_event(next_event).await {
            error!(
                ?e,
                "Failed to dispatch UserDecryptEventData::ReqSentToGw event"
            );
        }
    }

    /// Handles a failed decryption request.
    ///
    /// # Arguments
    /// * `event` - The [`RelayerEvent`] that failed
    /// * `error` - The [`EventProcessingError`] that caused the failure
    ///
    /// # Events
    /// Dispatches [`RelayerEventData::DecryptionFailed`]
    async fn handle_failed_request(&self, event: RelayerEvent, error: EventProcessingError) {
        error!(
            error = ?error,
            "Failed to send callback transaction"
        );

        let error_event = event.derive_next_event(RelayerEventData::UserDecrypt(
            UserDecryptEventData::Failed {
                error: format!("Callback transaction failed: {}", error),
            },
        ));

        if let Err(e) = self.dispatcher.dispatch_event(error_event).await {
            error!(?e, "Failed to dispatch error event");
        }
    }

    /// Extracts decryption ID from event logs.
    ///
    /// Processes decryption response events.
    ///
    /// This function:
    /// 1. Extracts `decryption_public_id` from the event
    /// 2. Retrieves original request ID using the `decryption_public_id`
    /// 3. Creates and dispatches response event with mock data
    ///
    /// # Arguments
    /// * `event` - The [`RelayerEvent`] containing the response data
    ///
    /// # State Access
    /// Reads from `decryption_id_to_request_id` mapping
    ///
    /// # Events
    /// Dispatches [`RelayerEventData::DecryptionResponseRcvdFromGw`]
    async fn handle_user_decrypt_reponse_event_log(&self, event: RelayerEvent) {
        info!("User Decryption response received: {:?}", event.request_id,);

        if let RelayerEventData::Generic(GenericEventData::EventLogFromGw { log }) = &event.data {
            if let Some(topic) = log.topic0() {
                if *topic == Decryption::UserDecryptionResponse::SIGNATURE_HASH {
                    match Decryption::UserDecryptionResponse::decode_log_data(log.data()) {
                        Ok(req) => {
                            let user_decryption_id = req.userDecryptionId;
                            info!(?user_decryption_id, "User decryption id from event");

                            if let Some(entry) = self
                                .user_decryption_id_to_request_id
                                .get(&user_decryption_id)
                            {
                                let original_request_id = *entry.value(); // Dereference the Ref<Uuid>

                                info!(
                                    ?original_request_id,
                                    ?user_decryption_id,
                                    "Found original request ID for user decryption response"
                                );

                                let next_event_data = RelayerEventData::UserDecrypt(
                                    UserDecryptEventData::RespRcvdFromGw {
                                        decrypt_response: UserDecryptResponse {
                                            gateway_request_id: user_decryption_id,
                                            reencrypted_shares: req.reencryptedShares,
                                            signatures: req.signatures,
                                        },
                                    },
                                );

                                info!("Dispatching UserDecryptEventData::RespRcvdFromGw event");

                                // Now we can use original_request_id directly
                                let next_event = RelayerEvent::new(
                                    original_request_id,
                                    event.api_version,
                                    next_event_data,
                                );

                                let _ = self.dispatcher.dispatch_event(next_event).await;
                            } else {
                                error!(
                                    ?user_decryption_id,
                                    "No matching request ID found for decryption ID"
                                );
                            }
                        }
                        Err(e) => {
                            error!(?e, "Failed to decode event data");
                        }
                    }
                }
            }
        }
    }

    fn handle_user_decrypt_request_sent(&self, id: U256) {
        info!(
            "Transaction to gateway has been done, the associated user decryption id is {}",
            id
        );
    }

    fn extract_user_decryption_id_from_receipt(
        &self,
        receipt: &AnyTransactionReceipt,
    ) -> Result<U256, EventProcessingError> {
        // Get the event signature for UserDecryptionRequest with the correct parameters
        let target_topic = UserDecryptionRequest::SIGNATURE_HASH;

        info!("Looking for topic: {}", UserDecryptionRequest::SIGNATURE);

        let receipt: TransactionReceipt<AnyReceiptEnvelope<Log>> = receipt.inner.clone();
        debug!(
            "Receipt details for user decryption:\n\
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

        info!("Looking for topic: 0x{}", hex::encode(target_topic));

        for log in receipt.inner.logs().iter() {
            if let Some(first_topic) = log.topics().first() {
                if first_topic == &target_topic {
                    return match Decryption::UserDecryptionRequest::decode_log_data(log.data()) {
                        Ok(event) => {
                            info!(
                                ?receipt.transaction_hash,
                                ?event.userDecryptionId,
                                "Found user decryption ID from event"
                            );
                            Ok(event.userDecryptionId)
                        }
                        Err(e) => {
                            error!(?receipt.transaction_hash, ?e, "Failed to decode user decryption event data");
                            Err(EventProcessingError::DecodingError(e))
                        }
                    };
                }
            }
        }

        error!(
            ?receipt.transaction_hash,
            "UserDecryptionRequest event not found in transaction logs"
        );

        Err(EventProcessingError::HandlerError(
            "UserDecryptionRequest event not found in logs".into(),
        ))
    }

    async fn noop_handle_decrypt_reponse_event_log(&self, _event: &RelayerEvent) {}

    async fn process_user_decryption_request(
        &self,
        user_decrypt_request: UserDecryptRequest,
    ) -> Result<U256, EventProcessingError> {
        let processor = UserDecryptionRequestProcessor {
            handler: Arc::new(self.clone()),
        };

        let url = Url::parse(&self.gateway_http_url).unwrap();

        let provider = ProviderBuilder::new()
            .network::<alloy::network::AnyNetwork>()
            .connect_http(url);

        let decryption_address =
            Address::from_str(&self.contracts.decryption_address).map_err(|_| {
                EventProcessingError::ConfigError(
                    crate::config::settings::AppConfigError::InvalidAddress(
                        "contracts.decryption_address".to_owned(),
                    ),
                )
            })?;
        let decryption = Decryption::new(decryption_address, provider.clone());

        // Convert to contract related pairs
        let contract_pairs: Vec<_> = user_decrypt_request
            .ct_handle_contract_pairs
            .iter()
            .map(Decryption::CtHandleContractPair::from)
            .collect();

        // Perform readiness check with retry logic
        let max_retries = self.retry_config.max_attempts;
        let retry_interval = Duration::from_secs(self.retry_config.base_delay_secs);

        let mut retries = 0;
        let mut should_retry = true;

        info!(
            "Checking if the decryption manager is ready for user address: {:?} and contract pairs: {:?}",
            user_decrypt_request.user_address, contract_pairs
        );

        if let Some(retry_config) = &self.tx_helper.tx_config.retry_config {
            if retry_config.mock_mode {
                info!("Mock mode is enabled, skipping readiness check");
                should_retry = false;
            }
        }

        while should_retry && retries < max_retries {
            should_retry = false;

            match decryption
                .clone()
                .checkUserDecryptionReady(user_decrypt_request.user_address, contract_pairs.clone())
                .call()
                .await
            {
                Ok(_) => {
                    info!(
                        "Function call succeeded for user address: {:?}",
                        user_decrypt_request.user_address
                    );
                }
                Err(err) => {
                    info!(
                        "Gateway not ready yet: {:?} error info: {}",
                        user_decrypt_request.user_address, err
                    );

                    should_retry = true;
                }
            }

            if should_retry {
                retries += 1;
                if retries < max_retries {
                    info!(
                        "Retrying user decryption readiness check (attempt {}/{})",
                        retries, max_retries
                    );
                    tokio::time::sleep(retry_interval).await;
                } else {
                    warn!("Max retries reached for user decryption readiness check");
                    return Err(EventProcessingError::HandlerError(format!(
                        "Gateway not ready after {} retries",
                        max_retries
                    )));
                }
            }
        }

        self.tx_helper
            .send_transaction(
                "user_decryption_request",
                decryption_address,
                || ComputeCalldata::user_decryption_req(user_decrypt_request.clone()),
                &processor,
            )
            .await
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
                let cloned_request = decrypt_request.clone();
                self.send_user_decryption_request_to_gateway(event.clone(), cloned_request)
                    .await;
            }
            RelayerEventData::Generic(GenericEventData::EventLogFromGw { ref log }) => {
                if let Some(topic0) = log.topic0() {
                    if FixedBytes::<32>::from_slice(topic0.as_slice())
                        != Decryption::UserDecryptionResponse::SIGNATURE_HASH
                    {
                        debug!(
                            "Ignore this event: expected event: {:?}, received {} ",
                            log.topic0(),
                            Decryption::UserDecryptionResponse::SIGNATURE_HASH
                        );
                        self.noop_handle_decrypt_reponse_event_log(&event).await;
                    } else {
                        self.handle_user_decrypt_reponse_event_log(event).await;
                    }
                };
            }
            RelayerEventData::UserDecrypt(UserDecryptEventData::ReqSentToGw {
                gw_req_reference_id,
            }) => {
                self.handle_user_decrypt_request_sent(gw_req_reference_id);
            }
            _ => {
                self.noop_handle_decrypt_reponse_event_log(&event).await;
            }
        }
    }
}

#[tokio::test]
async fn test_user_decryption_request() -> Result<(), Box<dyn std::error::Error>> {
    use crate::blockchain::ethereum::ComputeCalldata;
    use crate::config::settings::Settings;
    use crate::core::event::{HandleContractPair, RequestValidity};
    use crate::transaction::sender::TransactionManager;
    use crate::transaction::TxConfig;
    use alloy::primitives::{Address, Bytes, U256};
    use alloy::signers::{local::PrivateKeySigner, Signer};
    use std::str::FromStr;

    // Load configuration
    let settings = Settings::new().expect("Failed to load configuration");

    // Get network settings
    let gateway_settings = settings
        .get_network("gateway")
        .cloned()
        .expect("Failed to get gateway settings");

    // Test private key from environment variable or use default
    let private_key =
        std::env::var(&settings.transaction.private_key_gateway_env).unwrap_or_else(|_| {
            "7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f".to_string()
        });
    let mut gateway_signer: PrivateKeySigner = private_key.parse()?;
    gateway_signer.set_chain_id(Some(gateway_settings.chain_id));

    // Create transaction manager
    println!("Setting up manager with configured private key...");
    let manager = TransactionManager::new(&gateway_settings.ws_url, Arc::new(gateway_signer))
        .await
            .unwrap_or_else(|error| panic!(
                "Failed to create transaction manager. Make sure chain node is running at {}.\n{error}", gateway_settings.ws_url
            ));

    println!("Using address: {:?}", manager.sender_address());

    // Target contract address from config
    let decryption_address = Address::from_str(&settings.contracts.decryption_address)
        .expect("Invaliddecryption contract address");

    println!("Using decryption manager: {:?}", decryption_address);

    println!("Checking contract state...");
    let code = manager
        .verify_contract_code(decryption_address)
        .await
        .expect("Failed to verify contract code");
    println!("Contract code size: {} bytes", code.len());

    // Create minimal test data
    println!("Creating minimal test data...");

    let simple_handle = U256::from(123); // Random handle
    let contract_addresses = vec![decryption_address];
    let ct_handle_contract_pairs = vec![HandleContractPair {
        ct_handle: simple_handle,
        contract_address: decryption_address,
    }];
    let request_validity = RequestValidity {
        start_timestamp: U256::from(1672531200), // random unix timestamp
        duration_days: U256::from(10),
    };

    let contracts_chain_id = gateway_settings.chain_id;
    let user_address = manager.sender_address();

    let public_key = Bytes::from(vec![1, 2, 3, 4, 5]);
    let signature = Bytes::from(vec![9, 8, 7, 6, 5]);

    let user_decrypt_request: UserDecryptRequest = UserDecryptRequest {
        ct_handle_contract_pairs,
        request_validity,
        contracts_chain_id,
        contract_addresses,
        user_address,
        public_key,
        signature,
    };

    // Create and prepare calldata using your existing function
    let calldata = ComputeCalldata::user_decryption_req(user_decrypt_request)
        .expect("Failed to prepare calldata");

    println!("Calldata prepared: 0x{}", hex::encode(&calldata));

    // Set up transaction config from app config
    let config = TxConfig::from(settings.transaction);

    // Try sending the actual transaction
    println!("Sending transaction...");
    match manager
        .send_transaction_and_wait(decryption_address, calldata, Some(config))
        .await
    {
        Ok(receipt) => {
            let receipt: TransactionReceipt<AnyReceiptEnvelope<Log>> = receipt.inner;
            println!("Receipt status: {}", receipt.status());
            println!("Gas used: {}", receipt.gas_used);

            // Check for events
            for log in receipt.inner.logs() {
                println!(
                    "Log topics: {:?}",
                    log.topics().iter().map(hex::encode).collect::<Vec<_>>()
                );
            }
        }
        Err(e) => {
            println!("Error getting receipt: {}", e);
        }
    }
    Ok(())
}

/// Test for diagnosing the user decryption request
/// This test checks if the contract has the expected function and if the function selector is present in the bytecode.
/// It works only with mock contracts because original contracts are deployed behind a proxy
#[tokio::test]
async fn test_diagnose_user_decryption_request() -> Result<(), Box<dyn std::error::Error>> {
    use crate::config::settings::Settings;
    use crate::transaction::sender::TransactionManager;
    use alloy::primitives::{keccak256, Address};
    use std::str::FromStr;

    use alloy::signers::{local::PrivateKeySigner, Signer};

    println!("========== RUNNING DIAGNOSTIC TEST FOR USER DECRYPTION REQUEST ==========");

    // Load configuration
    let settings = Settings::new().expect("Failed to load configuration");
    let gateway_settings = settings
        .get_network("gateway")
        .cloned()
        .expect("Failed to get gateway settings");

    // Test private key from environment variable or use default
    let private_key =
        std::env::var(&settings.transaction.private_key_gateway_env).unwrap_or_else(|_| {
            "7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f".to_string()
        });
    let mut gateway_signer: PrivateKeySigner = private_key.parse()?;
    gateway_signer.set_chain_id(Some(gateway_settings.chain_id));

    println!("Setting up manager with configured private key...");
    let manager = TransactionManager::new(&gateway_settings.ws_url, Arc::new(gateway_signer))
        .await
            .unwrap_or_else(|error| panic!(
                "Failed to create transaction manager. Make sure chain node is running at {}.\n{error}", gateway_settings.ws_url
            ));

    let decryption_address = Address::from_str(&settings.contracts.decryption_address)
        .expect("Invaliddecryption contract address");

    println!("Using decryption manager: {:?}", decryption_address);
    println!("Sender address: {:?}", manager.sender_address());
    println!("Looking for topic: {}", UserDecryptionRequest::SIGNATURE);

    // STEP 1: Check if the contract has the expected function
    println!("\nSTEP 1: Checking if contract implements userDecryptionRequest...");

    // Get the function selector for userDecryptionRequest
    let func_selector = &keccak256(
        "UserDecryptionRequest(uint256,(bytes32,uint256,bytes32,address[])[],address,bytes)",
    )[..4];
    println!("Function selector : 0x{}", hex::encode(func_selector));

    // STEP 2: Check contract code size
    let code = manager.provider.get_code_at(decryption_address).await?;
    println!("Contract code size: {} bytes", code.len());

    // Search for our function selector in the bytecode
    let selector_hex = hex::encode(func_selector);
    let code_hex = hex::encode(&code);
    if code_hex.contains(&selector_hex) {
        println!("✅ Function selector found in contract bytecode");
    } else {
        println!("❓ Function selector not found in bytecode (might be a proxy contract)");
    }

    Ok(())
}
