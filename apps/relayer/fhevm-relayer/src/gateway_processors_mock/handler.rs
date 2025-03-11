use crate::{
    blockchain::ethereum::{
        bindings::{
            DecyptionManager::{PublicDecryptionRequest, UserDecryptionRequest},
            ZKPoKManager,
        },
        ComputeCalldata,
    },
    config::settings::ContractConfig,
    core::{
        errors::EventProcessingError,
        event::UserDecryptRequest,
        utils::{colorize_event_type, colorize_request_id},
    },
    gateway_processors_mock::event::{
        GatewayProcessorsEvent, GatewayProcessorsEventData, GatewayProcessorsInputEventData,
        PublicDecryptionEventData,
    },
    orchestrator::{traits::EventHandler, TokioEventDispatcher},
    transaction::{TransactionHelper, TransactionService, TxConfig},
};
use std::str::FromStr;

use alloy::primitives::{Address, FixedBytes, Log, U256};
use alloy::signers::{local::PrivateKeySigner, Signer};
use alloy::{
    sol,
    sol_types::{eip712_domain, SolStruct},
};
use alloy_sol_types::SolEvent;
use async_trait::async_trait;
use std::{sync::Arc, time::Duration};
use tracing::{debug, error, info};

use super::event::{DecryptionType, UserDecryptionEventData};

sol! {
    struct EIP712ZKPoK {
        bytes32[] handles;
        address userAddress;
        address contractAddress;
        uint256 contractChainId;
    }
}

#[derive(Clone)]
pub struct GatewayProcessorsHandler {
    _dispatcher: Arc<TokioEventDispatcher<GatewayProcessorsEvent>>,
    tx_helper: Arc<TransactionHelper>,
    contracts: ContractConfig,
}

impl GatewayProcessorsHandler {
    pub fn new(
        _dispatcher: Arc<TokioEventDispatcher<GatewayProcessorsEvent>>,
        tx_service: Arc<TransactionService>,
        tx_config: TxConfig,
        contracts: ContractConfig,
    ) -> Self {
        Self {
            _dispatcher,
            tx_helper: Arc::new(TransactionHelper::new(tx_service, tx_config)),
            contracts,
        }
    }

    /// Process the InputRequest event and prepare response
    async fn process_input_request(
        &self,
        event: &GatewayProcessorsEvent,
    ) -> Result<(), EventProcessingError> {
        if let GatewayProcessorsEventData::KmsInput(
            GatewayProcessorsInputEventData::EventLogRequestFromGwL2 { log },
        ) = &event.data
        {
            // Log the raw data for debugging
            debug!(
                topics = ?log.topics().iter().map(hex::encode).collect::<Vec<_>>(),
                "Processing log data"
            );

            match ZKPoKManager::VerifyProofRequest::decode_log_data(log.data(), true) {
                Ok(request_event) => {
                    info!(
                        zkpok_id = ?request_event.zkProofId,
                        chain_id = ?request_event.contractChainId,
                        contract = ?request_event.contractAddress,
                        user = ?request_event.userAddress,
                        "Processing InputRequest event"
                    );

                    // Simulate some computation time
                    tokio::time::sleep(Duration::from_secs(2)).await;

                    // Generate mock handles
                    // In real implementation, this would involve actual cryptographic operations
                    let mut handles = vec![[5u8; 32], [2u8; 32]];
                    handles[0][29] = 0;
                    handles[0][31] = 0;
                    handles[1][29] = 1;
                    handles[1][31] = 0;

                    //let signatures = vec![1u8; 65];
                    let signer = PrivateKeySigner::from_str(
                        "c2454775cca95e6d17d70b68105f48009fc4bf661f025e6a7911a6b4acf2a2f3",
                    )
                    .unwrap();

                    let domain = eip712_domain! {
                        name: "ZKPoKManager",
                        version: "1",
                        chain_id: 654321,
                        verifying_contract: Address::from_str(&self.contracts.zkpok_manager_address).unwrap(),
                    };

                    let handles_formatted: Vec<FixedBytes<32>> =
                        handles.clone().into_iter().map(FixedBytes::from).collect();

                    let signing_hash = EIP712ZKPoK {
                        handles: handles_formatted.clone(),
                        userAddress: request_event.userAddress,
                        contractAddress: request_event.contractAddress,
                        contractChainId: U256::from(request_event.contractChainId),
                    }
                    .eip712_signing_hash(&domain);

                    let signature = signer.sign_hash(&signing_hash).await.unwrap();
                    println!("Signature: 0x{}", hex::encode(signature.as_bytes()));

                    self.send_input_response(
                        request_event.zkProofId,
                        handles,
                        signature.as_bytes().to_vec(),
                    )
                    .await?;

                    Ok(())
                }
                Err(e) => {
                    error!(?e, "Failed to decode InputRequest event");
                    Err(EventProcessingError::DecodingError(e))
                }
            }
        } else {
            Err(EventProcessingError::HandlerError(
                "Input request log not found".to_owned(),
            ))
        }
    }

    /// Send InputResponse transaction
    async fn send_input_response(
        &self,
        zkpok_id: U256,
        handles: Vec<[u8; 32]>,
        signature: Vec<u8>,
    ) -> Result<(), EventProcessingError> {
        info!(?zkpok_id, "Sending InputResponse transaction");
        let zkpok_manager_address = Address::from_str(&self.contracts.zkpok_manager_address)
            .map_err(|_| {
                EventProcessingError::ConfigError(
                    crate::config::settings::AppConfigError::InvalidAddress(
                        "self.contracts.zkpok_manager_address".to_owned(),
                    ),
                )
            })?;
        self.tx_helper
            .send_transaction_simple("input_response", zkpok_manager_address, || {
                ComputeCalldata::verify_proof_response(zkpok_id, handles.clone(), signature.clone())
            })
            .await?;

        Ok(())
    }

    /// Send InputResponse transaction
    async fn send_decryption_response(
        &self,
        req: PublicDecryptionRequest,
    ) -> Result<(), EventProcessingError> {
        let decryption_manager_address =
            Address::from_str(&self.contracts.decryption_manager_address).map_err(|_| {
                EventProcessingError::ConfigError(
                    crate::config::settings::AppConfigError::InvalidAddress(
                        "contracts.decryption_manager_address".to_owned(),
                    ),
                )
            })?;
        self.tx_helper
            .send_transaction_simple("decryption_response", decryption_manager_address, || {
                ComputeCalldata::decryption_response(req.clone(), decryption_manager_address)
            })
            .await?;

        Ok(())
    }

    async fn send_user_decryption_response(
        &self,
        req: UserDecryptionRequest,
    ) -> Result<(), EventProcessingError> {
        let decryption_manager_address =
            Address::from_str(&self.contracts.decryption_manager_address).map_err(|_| {
                EventProcessingError::ConfigError(
                    crate::config::settings::AppConfigError::InvalidAddress(
                        "contracts.decryption_manager_address".to_owned(),
                    ),
                )
            })?;
        self.tx_helper
            .send_transaction_simple("decryption_response", decryption_manager_address, || {
                ComputeCalldata::user_decryption_response(req.clone())
            })
            .await?;

        Ok(())
    }

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
    async fn process_public_decryption_request(
        &self,
        event: GatewayProcessorsEvent,
    ) -> Result<(), EventProcessingError> {
        info!(
            "Public Decryption request received. Prepare user decryption response transaction to Gateway received. {:?}",
            event.request_id,
        );

        // Simulate some computation time
        tokio::time::sleep(Duration::from_secs(2)).await;

        if let GatewayProcessorsEventData::PublicDecrypt(
            PublicDecryptionEventData::EventLogRequestFromGwL2 { log },
        ) = &event.data
        {
            match PublicDecryptionRequest::decode_log_data(log.data(), true) {
                Ok(req) => {
                    let public_decryption_id = req.publicDecryptionId;
                    info!(?public_decryption_id);

                    let mut ciphertext_handles: Vec<U256> = Vec::new();
                    for sns_ct_material in req.snsCtMaterials.clone() {
                        ciphertext_handles.push(sns_ct_material.ctHandle);
                    }

                    info!(
                        public_decryption_id = ?req.publicDecryptionId,
                        handles = ?ciphertext_handles,
                        "Processing PublicDecryptRequest event"
                    );

                    match self.send_decryption_response(req).await {
                        Ok(()) => {
                            info!("Public decryption response sent successfully!");
                            return Ok(());
                        }
                        Err(e) => {
                            error!(?e, "Public decryption response processing failed!");
                            return Err(EventProcessingError::HandlerError(
                                "Failed to decode public decrypt event data".to_owned(),
                            ));
                        }
                    }
                }

                Err(e) => {
                    error!(?e, "Failed to decode public decrypt event data");
                    return Err(EventProcessingError::HandlerError(
                        "Failed to decode public decrypt event data".to_owned(),
                    ));
                }
            }
        }
        Err(EventProcessingError::HandlerError(
            "Failed to decode public decrypt event data".to_owned(),
        ))
    }

    async fn process_user_decryption_request(
        &self,
        event: GatewayProcessorsEvent,
    ) -> Result<(), EventProcessingError> {
        info!(
        "User Decryption request received. Prepare decryption response transaction to Gateway received. {:?}",
        event.request_id,
    );

        // Simulate some computation time
        tokio::time::sleep(Duration::from_secs(2)).await;

        if let GatewayProcessorsEventData::UserDecrypt(
            UserDecryptionEventData::EventLogRequestFromGwL2 { log },
        ) = &event.data
        {
            match UserDecryptionRequest::decode_log_data(log.data(), true) {
                Ok(req) => {
                    let user_decryption_id = req.userDecryptionId;
                    info!(?user_decryption_id);

                    info!(
                        user_decryption_id = ?req.userDecryptionId,
                        "Processing UserDecryptRequest event"
                    );

                    match self.send_user_decryption_response(req).await {
                        Ok(()) => {
                            info!("Public decryption response sent successfully!");
                            return Ok(());
                        }
                        Err(e) => {
                            error!(?e, "Public decryption response processing failed!");
                            return Err(EventProcessingError::HandlerError(
                                "Failed to decode public decrypt event data".to_owned(),
                            ));
                        }
                    }
                }
                Err(e) => {
                    error!(?e, "Failed to decode public decrypt event data");
                    return Err(EventProcessingError::HandlerError(
                        "Failed to decode public decrypt event data".to_owned(),
                    ));
                }
            }
        }
        Err(EventProcessingError::HandlerError(
            "Failed to decode public decrypt event data".to_owned(),
        ))
    }
}

#[async_trait]
impl EventHandler<GatewayProcessorsEvent> for GatewayProcessorsHandler {
    async fn handle_event(&self, event: GatewayProcessorsEvent) {
        info!(
            event_type = %colorize_event_type(event.data.as_ref()),
            request_id = %colorize_request_id(&event.request_id),
            "Processing event in processors mock"
        );

        match &event.clone().data {
            GatewayProcessorsEventData::KmsInput(input_event) => match input_event {
                GatewayProcessorsInputEventData::EventLogRequestFromGwL2 { .. } => {
                    info!("Received input event log from Gateway L2");
                    match self.process_input_request(&event).await {
                        Ok(()) => {
                            info!("Input request processing succesfull!");
                        }
                        Err(e) => {
                            error!(?e, "Input request processing failed!")
                        }
                    }
                }
            },
            GatewayProcessorsEventData::UserDecrypt(user_decrypt_event) => match user_decrypt_event
            {
                UserDecryptionEventData::EventLogRequestFromGwL2 { .. } => {
                    info!("Received user decryption event log from Gateway L2");
                    match self.process_user_decryption_request(event).await {
                        Ok(()) => {
                            info!("Input request processing succesfull!");
                        }
                        Err(e) => {
                            error!(?e, "Input request processing failed!")
                        }
                    }
                }
            },
            GatewayProcessorsEventData::PublicDecrypt(public_decrypt_event) => {
                match public_decrypt_event {
                    PublicDecryptionEventData::EventLogRequestFromGwL2 { .. } => {
                        info!("Received decryption event log from Gateway L2");
                        match self.process_public_decryption_request(event).await {
                            Ok(()) => {
                                info!("Input request processing succesfull!");
                            }
                            Err(e) => {
                                error!(?e, "Input request processing failed!")
                            }
                        }
                    }
                }
            }
        }
    }
}
