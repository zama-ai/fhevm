use crate::{
    blockchain::ethereum::{
        bindings::DecyptionManager::{self, UserDecryptionRequest},
        ComputeCalldata,
    },
    config::settings::ContractConfig,
    core::{
        errors::EventProcessingError,
        event::{
            GenericEventData, PublicDecryptEventData, PublicDecryptResponse, RelayerEvent,
            RelayerEventData, UserDecryptEventData, UserDecryptRequest, UserDecryptResponse,
        },
        utils::{colorize_event_type, colorize_request_id},
    },
    orchestrator::{
        traits::{EventDispatcher, EventHandler},
        TokioEventDispatcher,
    },
    transaction::{ReceiptProcessor, TransactionHelper, TransactionService, TxConfig},
};
use std::str::FromStr;

use alloy::{
    primitives::{keccak256, Address, Bytes, Uint, U256},
    rpc::types::TransactionReceipt,
};

use alloy_sol_types::SolEvent;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::task;
use tracing::{debug, error, info};
use uuid::Uuid;

struct DecryptionRequestProcessor {
    handler: Arc<ArbitrumGatewayL2Handler>,
}

impl ReceiptProcessor for DecryptionRequestProcessor {
    type Output = U256;

    fn process(&self, receipt: &TransactionReceipt) -> Result<Self::Output, EventProcessingError> {
        self.handler.extract_decryption_id_from_receipt(receipt)
    }
}

struct UserDecryptionRequestProcessor {
    handler: Arc<ArbitrumGatewayL2Handler>,
}

impl ReceiptProcessor for UserDecryptionRequestProcessor {
    type Output = U256;

    fn process(&self, receipt: &TransactionReceipt) -> Result<Self::Output, EventProcessingError> {
        self.handler
            .extract_user_decryption_id_from_receipt(receipt)
    }
}

#[derive(Clone)]
pub struct ArbitrumGatewayL2Handler {
    dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
    public_decryption_id_to_request_id: Arc<dashmap::DashMap<U256, Uuid>>,
    user_decryption_id_to_request_id: Arc<dashmap::DashMap<U256, Uuid>>,
    tx_helper: Arc<TransactionHelper>,
    contracts: ContractConfig,
}

impl ArbitrumGatewayL2Handler {
    pub fn new(
        dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
        tx_service: Arc<TransactionService>,
        tx_config: TxConfig,
        contracts: ContractConfig,
    ) -> Self {
        Self {
            dispatcher,
            tx_helper: Arc::new(TransactionHelper::new(tx_service, tx_config)),
            public_decryption_id_to_request_id: Arc::new(dashmap::DashMap::new()),
            user_decryption_id_to_request_id: Arc::new(dashmap::DashMap::new()),
            contracts,
        }
    }

    /// Prepares and sends a decryption request transaction to the gateway.
    ///
    /// This function performs the following:
    /// 1. Converts the input handles to [`Uint<256, 4>`]
    /// 2. Sends transaction to the [`DecyptionManager`] contract
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
    /// * Success: [`RelayerEventData::DecryptionRequestSentToGwL2`]
    /// * Failure: [`RelayerEventData::DecryptionFailed`]
    async fn send_public_decryption_request_to_rollup(
        &self,
        event: RelayerEvent,
        handles: Vec<[u8; 32]>,
    ) {
        let handles: Vec<Uint<256, 4>> = handles
            .iter()
            .map(|bytes| Uint::from_be_bytes(*bytes))
            .collect();

        info!(
            "Decryption request received. Making a tx to rollup: request_id: {:?} with handles {:?}",
            event.request_id,
            handles
        );

        let self_clone = self.clone();
        let event_clone = event.clone();

        // Spawn a blocking task to make a transaction to rollup
        task::spawn(async move {
            match self_clone.process_decryption_request(handles).await {
                Ok(decryption_public_id) => {
                    self_clone
                        .handle_successful_public_request(event_clone, decryption_public_id)
                        .await;
                }
                Err(e) => {
                    self_clone.handle_failed_request(event_clone, e).await;
                }
            }
        });
    }

    /// Prepares and sends a decryption request transaction to the gateway.
    ///
    /// This function performs the following:
    /// 1. Converts the input handles to [`Uint<256, 4>`]
    /// 2. Sends transaction to the [`DecyptionManager`] contract
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
    /// * Success: [`RelayerEventData::DecryptionRequestSentToGwL2`]
    /// * Failure: [`RelayerEventData::DecryptionFailed`]
    async fn send_user_decryption_request_to_rollup(
        &self,
        event: RelayerEvent,
        user_decrypt_request: UserDecryptRequest,
    ) {
        info!(
            "User Decryption request received. Making a tx to rollup: request_id: {:?} with user request {:?}",
            event.request_id,
            user_decrypt_request
        );

        let self_clone = self.clone();
        let event_clone = event.clone();

        // Spawn a blocking task to make a transaction to rollup
        task::spawn(async move {
            match self_clone
                .process_user_decryption_request(
                    user_decrypt_request.ct_handles,
                    user_decrypt_request.contracts_chain_id,
                    user_decrypt_request.contract_address,
                    user_decrypt_request.user_address,
                    user_decrypt_request.encryption_key.clone(),
                    user_decrypt_request.signature.clone(),
                )
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
    /// Dispatches [`RelayerEventData::DecryptionRequestSentToGwL2`]
    async fn handle_successful_public_request(
        &self,
        event: RelayerEvent,
        decryption_public_id: U256,
    ) {
        // Store the mapping
        self.public_decryption_id_to_request_id
            .insert(decryption_public_id, event.request_id);

        info!(
            ?event.request_id,
            ?decryption_public_id,
            "Stored mapping between decryption ID and request ID"
        );

        // Create and dispatch the new event
        let next_event = event.derive_next_event(RelayerEventData::PublicDecrypt(
            PublicDecryptEventData::ReqSentToGw {
                public_decryption_id: decryption_public_id,
            },
        ));

        if let Err(e) = self.dispatcher.dispatch_event(next_event).await {
            error!(?e, "Failed to dispatch DecryptRequestProcessed event");
        }
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
    /// Dispatches [`RelayerEventData::DecryptionRequestSentToGwL2`]
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

        // Create and dispatch the new event
        let next_event = event.derive_next_event(RelayerEventData::UserDecrypt(
            UserDecryptEventData::ReqSentToGw { user_decryption_id },
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

        let error_event = event.derive_next_event(RelayerEventData::PublicDecrypt(
            PublicDecryptEventData::Failed {
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
    /// Dispatches [`RelayerEventData::DecryptionResponseRcvdFromGwL2`]
    async fn handle_decrypt_reponse_event_log(&self, event: RelayerEvent) {
        info!(
            "Decryption response received. Trigger a tx to L1  {:?}",
            event.request_id,
        );

        if let RelayerEventData::Generic(GenericEventData::EventLogFromGw { log }) = &event.data {
            if let Some(topic) = log.topic0() {
                match *topic {
                    DecyptionManager::PublicDecryptionResponse::SIGNATURE_HASH => {
                        match DecyptionManager::PublicDecryptionResponse::decode_log_data(
                            log.data(),
                            true,
                        ) {
                            Ok(req) => {
                                let public_decryption_id = req.publicDecryptionId;
                                info!(?public_decryption_id, "Public decryption id from event");

                                if let Some(entry) = self
                                    .public_decryption_id_to_request_id
                                    .get(&public_decryption_id)
                                {
                                    let original_request_id = *entry.value(); // Dereference the Ref<Uuid>

                                    info!(
                                        ?original_request_id,
                                        ?public_decryption_id,
                                        "Found original request ID for decryption response"
                                    );

                                    let next_event_data = RelayerEventData::PublicDecrypt(
                                        PublicDecryptEventData::RespRcvdFromGw {
                                            decrypt_response: PublicDecryptResponse {
                                                gateway_request_id: public_decryption_id,
                                                decrypted_value: req.decryptedResult,
                                                signatures: req.signatures,
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
                                    error!(
                                        ?public_decryption_id,
                                        "No matching request ID found for decryption ID"
                                    );
                                }
                            }
                            Err(e) => {
                                error!(?e, "Failed to decode event data");
                            }
                        }
                    }

                    DecyptionManager::UserDecryptionResponse::SIGNATURE_HASH => {
                        match DecyptionManager::UserDecryptionResponse::decode_log_data(
                            log.data(),
                            true,
                        ) {
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
                    _ => {}
                }
            }
        }
    }

    /// Extracts the decryption ID from a transaction receipt.
    ///
    /// Searches for the [`PublicDecryptionRequest`] event in the logs and decodes it.
    ///
    /// # Arguments
    /// * `receipt` - The [`TransactionReceipt`] to process
    ///
    /// # Returns
    /// * `Ok(`[`U256`]`)` - The extracted decryption ID
    /// * `Err(`[`EventProcessingError`]`)` - If event is not found or decoding fails
    fn handle_decrypt_request_sent(&self, id: U256) {
        info!(
            "Transaction to rollup has been done, the associated public decryption id is {}",
            id
        );
    }

    fn handle_user_decrypt_request_sent(&self, id: U256) {
        info!(
            "Transaction to rollup has been done, the associated user decryption id is {}",
            id
        );
    }

    fn extract_decryption_id_from_receipt(
        &self,
        receipt: &TransactionReceipt,
    ) -> Result<U256, EventProcessingError> {
        let target_topic = keccak256("PublicDecryptionRequest(uint256,uint256[])");

        for log in receipt.inner.logs().iter() {
            if let Some(first_topic) = log.topics().first() {
                if first_topic == &target_topic {
                    return match DecyptionManager::PublicDecryptionRequest::decode_log_data(
                        log.data(),
                        true,
                    ) {
                        Ok(event) => {
                            info!(
                                ?receipt.transaction_hash,
                                ?event.publicDecryptionId,
                                "Found decryption ID from event"
                            );
                            Ok(event.publicDecryptionId)
                        }
                        Err(e) => {
                            error!(?receipt.transaction_hash, ?e, "Failed to decode event data");
                            Err(EventProcessingError::DecodingError(e))
                        }
                    };
                }
            }
        }

        Err(EventProcessingError::HandlerError(
            "Event not found in logs".into(),
        ))
    }

    fn extract_user_decryption_id_from_receipt(
        &self,
        receipt: &TransactionReceipt,
    ) -> Result<U256, EventProcessingError> {
        // Get the event signature for UserDecryptionRequest with the correct parameters
        let target_topic = UserDecryptionRequest::SIGNATURE_HASH;

        info!("Looking for topic: {}", UserDecryptionRequest::SIGNATURE);

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
                    return match DecyptionManager::UserDecryptionRequest::decode_log_data(
                        log.data(),
                        true,
                    ) {
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

    async fn noop_handle_decrypt_reponse_event_log(&self, _event: RelayerEvent) {}

    /// Processes a decryption request by sending it to the L2 contract.
    ///
    /// Uses [`TransactionHelper`] with [`DecryptionRequestProcessor`] to send
    /// and process the transaction.
    ///
    /// # Arguments
    /// * `handles` - Vector of [`Uint<256, 4>`] representing the decryption handles
    ///
    /// # Returns
    /// * `Ok(`[`U256`]`)` - The decryption ID from the transaction
    /// * `Err(`[`EventProcessingError`]`)` - If the transaction fails
    async fn process_decryption_request(
        &self,
        handles: Vec<Uint<256, 4>>,
    ) -> Result<U256, EventProcessingError> {
        let processor = DecryptionRequestProcessor {
            handler: Arc::new(self.clone()),
        };

        let decryption_manager_address =
            Address::from_str(&self.contracts.decryption_manager_address).map_err(|_| {
                EventProcessingError::ConfigError(
                    crate::config::settings::AppConfigError::InvalidAddress(
                        "contracts.decryption_manager_address".to_owned(),
                    ),
                )
            })?;
        self.tx_helper
            .send_transaction(
                "decryption_request",
                decryption_manager_address,
                || ComputeCalldata::decryption_req(handles.clone()),
                &processor,
            )
            .await
    }

    async fn process_user_decryption_request(
        &self,
        ct_handles: Vec<Bytes>,
        contract_chain_id: U256,
        contract_address: Address,
        user_address: Address,
        public_key: Bytes,
        signature: Bytes,
    ) -> Result<U256, EventProcessingError> {
        let processor = UserDecryptionRequestProcessor {
            handler: Arc::new(self.clone()),
        };

        let decryption_manager_address =
            Address::from_str(&self.contracts.decryption_manager_address).map_err(|_| {
                EventProcessingError::ConfigError(
                    crate::config::settings::AppConfigError::InvalidAddress(
                        "contracts.decryption_manager_address".to_owned(),
                    ),
                )
            })?;
        self.tx_helper
            .send_transaction(
                "user_decryption_request",
                decryption_manager_address,
                || {
                    ComputeCalldata::user_decryption_req(
                        ct_handles.clone(),
                        contract_chain_id,
                        contract_address,
                        user_address,
                        public_key.clone(),
                        signature.clone(),
                    )
                },
                &processor,
            )
            .await
    }
}

#[async_trait]
impl EventHandler<RelayerEvent> for ArbitrumGatewayL2Handler {
    async fn handle_event(&self, event: RelayerEvent) {
        info!(
            event_type = %colorize_event_type(event.data.as_ref()),
            request_id = %colorize_request_id(&event.request_id),
            "Processing relayer event"
        );
        match event.data {
            RelayerEventData::PublicDecrypt(PublicDecryptEventData::ReqRcvdFromHostBc {
                ref decrypt_request,
                ..
            }) => {
                let handles = decrypt_request.ct_handles.clone();
                self.send_public_decryption_request_to_rollup(event, handles)
                    .await;
            }
            RelayerEventData::UserDecrypt(UserDecryptEventData::ReqRcvdFromUser {
                ref decrypt_request,
                ..
            }) => {
                let cloned_request = decrypt_request.clone();
                self.send_user_decryption_request_to_rollup(event.clone(), cloned_request)
                    .await;
            }
            RelayerEventData::Generic(GenericEventData::EventLogFromGw { .. }) => {
                self.handle_decrypt_reponse_event_log(event).await;
            }
            RelayerEventData::PublicDecrypt(PublicDecryptEventData::ReqSentToGw {
                public_decryption_id,
            }) => {
                self.handle_decrypt_request_sent(public_decryption_id);
            }
            RelayerEventData::UserDecrypt(UserDecryptEventData::ReqSentToGw {
                user_decryption_id,
            }) => {
                self.handle_user_decrypt_request_sent(user_decryption_id);
            }
            _ => {
                self.noop_handle_decrypt_reponse_event_log(event).await;
            }
        }
    }
}

#[tokio::test]
async fn test_user_decryption_request() -> Result<(), Box<dyn std::error::Error>> {
    use crate::blockchain::ethereum::ComputeCalldata;
    use crate::config::settings::Settings;
    use crate::transaction::sender::TransactionManager;
    use crate::transaction::TxConfig;
    use alloy::primitives::{Address, Bytes, U256};
    use std::str::FromStr;

    // Load configuration
    let settings = Settings::new().expect("Failed to load configuration");

    // Get network settings
    let rollup_settings = settings
        .get_network("rollup")
        .cloned()
        .expect("Failed to get rollup settings");

    // Test private key from environment variable or use default
    let private_key =
        std::env::var(&settings.transaction.private_key_gateway_env).unwrap_or_else(|_| {
            "7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f".to_string()
        });

    // Create transaction manager
    println!("Setting up manager with configured private key...");
    let manager = TransactionManager::new(
        &rollup_settings.http_url,
        &private_key,
        rollup_settings.chain_id,
    )
    .await
    .expect("Failed to create transaction manager");

    println!("Using address: {:?}", manager.sender_address());

    // Target contract address from config
    let decryption_manager_address =
        Address::from_str(&settings.contracts.decryption_manager_address)
            .expect("Invalid decryption manager address");

    println!("Using decryption manager: {:?}", decryption_manager_address);

    println!("Checking contract state...");
    let code = manager
        .verify_contract_code(decryption_manager_address)
        .await
        .expect("Failed to verify contract code");
    println!("Contract code size: {} bytes", code.len());

    // Create minimal test data
    println!("Creating minimal test data...");

    // 1. Simple handle - just a small number
    let simple_handle = U256::from(123);
    let mut handle_bytes = [0u8; 32];
    simple_handle
        .to_be_bytes::<32>()
        .iter()
        .enumerate()
        .for_each(|(i, b)| handle_bytes[i] = *b);
    let ct_handles = vec![Bytes::from(handle_bytes.to_vec())];

    // 2. Chain ID from config
    let contract_chain_id = U256::from(rollup_settings.chain_id);

    // 3. Contract address from config & user address from the transaction manager
    let contract_address = decryption_manager_address;
    let user_address = manager.sender_address();

    // 4. Simple public key and signature
    let public_key = Bytes::from(vec![1, 2, 3, 4, 5]);
    let signature = Bytes::from(vec![9, 8, 7, 6, 5]);

    // Create and prepare calldata using your existing function
    let calldata = ComputeCalldata::user_decryption_req(
        ct_handles,
        contract_chain_id,
        contract_address,
        user_address,
        public_key,
        signature,
    )
    .expect("Failed to prepare calldata");

    println!("Calldata prepared: 0x{}", hex::encode(&calldata));

    // Set up transaction config from app config
    let config = TxConfig::from(settings.transaction);

    // Try sending the actual transaction
    println!("Sending transaction...");
    match manager
        .send_transaction_and_wait(decryption_manager_address, calldata, Some(config))
        .await
    {
        Ok(receipt) => {
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

#[tokio::test]
async fn test_diagnose_user_decryption_request() -> Result<(), Box<dyn std::error::Error>> {
    use crate::config::settings::Settings;
    use crate::transaction::sender::TransactionManager;
    use alloy::primitives::{keccak256, Address};
    use std::str::FromStr;

    println!("========== RUNNING DIAGNOSTIC TEST FOR USER DECRYPTION REQUEST ==========");

    // Load configuration
    let settings = Settings::new().expect("Failed to load configuration");
    let rollup_settings = settings
        .get_network("rollup")
        .cloned()
        .expect("Failed to get rollup settings");

    // Create transaction manager
    let private_key =
        std::env::var(&settings.transaction.private_key_gateway_env).unwrap_or_else(|_| {
            "7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f".to_string()
        });

    println!("Setting up manager with configured private key...");
    let manager = TransactionManager::new(
        &rollup_settings.http_url,
        &private_key,
        rollup_settings.chain_id,
    )
    .await
    .expect("Failed to create transaction manager");

    let decryption_manager_address =
        Address::from_str(&settings.contracts.decryption_manager_address)
            .expect("Invalid decryption manager address");

    println!("Using decryption manager: {:?}", decryption_manager_address);
    println!("Sender address: {:?}", manager.sender_address());

    // STEP 1: Check if the contract has the expected function
    println!("\nSTEP 1: Checking if contract implements userDecryptionRequest...");

    // Get the function selector for userDecryptionRequest
    let func_selector =
        &keccak256("userDecryptionRequest((uint256,address)[],(uint256,uint256),uint256,address[],address,bytes,bytes)")
            [..4];
    println!("Function selector: 0x{}", hex::encode(func_selector));

    // STEP 2: Check contract code size
    let code = manager
        .provider
        .get_code_at(decryption_manager_address)
        .await?;
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
