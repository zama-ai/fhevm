use crate::{
    blockchain::ethereum::{
        bindings::{
            Decryption::{PublicDecryptionRequest, UserDecryptionRequest},
            InputVerification,
        },
        ComputeCalldata,
    },
    config::settings::ContractConfig,
    core::errors::EventProcessingError,
    gateway_processors_mock::event::{
        GatewayProcessorsEvent, GatewayProcessorsEventData, GatewayProcessorsInputEventData,
        PublicDecryptionEventData,
    },
    orchestrator::traits::EventHandler,
    transaction::{TransactionHelper, TransactionService, TxConfig},
};
use std::str::FromStr;

use alloy::primitives::{Address, FixedBytes, U256};
use alloy::signers::{local::PrivateKeySigner, Signer};
use alloy::sol_types::SolEvent;
use alloy::{
    sol,
    sol_types::{eip712_domain, SolStruct},
};
use async_trait::async_trait;
use std::{sync::Arc, time::Duration};
use tracing::{debug, error, info};

use super::event::UserDecryptionEventData;

sol! {
    struct CiphertextVerification {
        bytes32[] ctHandles;
        address userAddress;
        address contractAddress;
        uint256 contractChainId;
    }
}

#[derive(Clone)]
pub struct GatewayProcessorsHandler {
    tx_helper: Arc<TransactionHelper>,
    contracts: ContractConfig,
}

impl GatewayProcessorsHandler {
    pub fn new(
        tx_service: Arc<TransactionService>,
        tx_config: TxConfig,
        contracts: ContractConfig,
    ) -> Self {
        Self {
            tx_helper: Arc::new(TransactionHelper::new(tx_service, tx_config)),
            contracts,
        }
    }

    /// Process the InputRequest event and prepare response
    async fn process_input_proof_request(
        &self,
        event: &GatewayProcessorsEvent,
    ) -> Result<(), EventProcessingError> {
        if let GatewayProcessorsEventData::KmsInput(
            GatewayProcessorsInputEventData::EventLogRequestFromGw { log },
        ) = &event.data
        {
            // Log the raw data for debugging
            debug!(
                topics = ?log.topics().iter().map(hex::encode).collect::<Vec<_>>(),
                "Processing log data"
            );

            match InputVerification::VerifyProofRequest::decode_log_data(log.data()) {
                Ok(request_event) => {
                    info!(
                        input_verification_id = ?request_event.zkProofId,
                        chain_id = ?request_event.contractChainId,
                        contract = ?request_event.contractAddress,
                        user = ?request_event.userAddress,
                        "Processing InputRequest event"
                    );

                    if request_event
                        .ciphertextWithZKProof
                        .to_string()
                        .contains("aaaaaaaaaaa")
                    {
                        self.send_input_proof_rejection_response(request_event.zkProofId)
                            .await?;
                        return Ok(());
                    }

                    // Simulate some computation time
                    // TODO: make it configurable
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
                        name: "InputVerification",
                        version: "1",
                        chain_id: 654321,
                        verifying_contract: Address::from_str(&self.contracts.input_verification_address).unwrap(),
                    };

                    let handles_formatted: Vec<FixedBytes<32>> =
                        handles.clone().into_iter().map(FixedBytes::from).collect();

                    let signing_hash = CiphertextVerification {
                        ctHandles: handles_formatted.clone(),
                        userAddress: request_event.userAddress,
                        contractAddress: request_event.contractAddress,
                        contractChainId: U256::from(request_event.contractChainId),
                    }
                    .eip712_signing_hash(&domain);

                    let signature = signer.sign_hash(&signing_hash).await.unwrap();
                    println!("Signature: 0x{}", hex::encode(signature.as_bytes()));

                    self.send_input_proof_response(
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

    /// Send RejectInputResponse transaction
    async fn send_input_proof_rejection_response(
        &self,
        input_verification_id: U256,
    ) -> Result<(), EventProcessingError> {
        info!(?input_verification_id, "Sending InputResponse transaction");
        let input_verification_address =
            Address::from_str(&self.contracts.input_verification_address).map_err(|_| {
                EventProcessingError::ConfigError(
                    crate::config::settings::AppConfigError::InvalidAddress(
                        "self.contracts.input_verification_address".to_owned(),
                    ),
                )
            })?;
        self.tx_helper
            .send_transaction_simple("input_response", input_verification_address, || {
                ComputeCalldata::reject_proof_response(input_verification_id)
            })
            .await?;

        Ok(())
    }

    /// Send InputResponse transaction
    async fn send_input_proof_response(
        &self,
        input_verification_id: U256,
        handles: Vec<[u8; 32]>,
        signature: Vec<u8>,
    ) -> Result<(), EventProcessingError> {
        info!(?input_verification_id, "Sending InputResponse transaction");
        let input_verification_address =
            Address::from_str(&self.contracts.input_verification_address).map_err(|_| {
                EventProcessingError::ConfigError(
                    crate::config::settings::AppConfigError::InvalidAddress(
                        "self.contracts.input_verification_address".to_owned(),
                    ),
                )
            })?;
        self.tx_helper
            .send_transaction_simple("input_response", input_verification_address, || {
                ComputeCalldata::verify_proof_response(
                    input_verification_id,
                    handles.clone(),
                    signature.clone(),
                )
            })
            .await?;

        Ok(())
    }

    /// Send InputResponse transaction
    async fn send_public_decryption_response(
        &self,
        req: PublicDecryptionRequest,
    ) -> Result<(), EventProcessingError> {
        let decryption_address =
            Address::from_str(&self.contracts.decryption_address).map_err(|_| {
                EventProcessingError::ConfigError(
                    crate::config::settings::AppConfigError::InvalidAddress(
                        "contracts.decryption_address".to_owned(),
                    ),
                )
            })?;
        self.tx_helper
            .send_transaction_simple("decryption_response", decryption_address, || {
                ComputeCalldata::decryption_response(req.clone(), decryption_address)
            })
            .await?;

        Ok(())
    }

    async fn send_user_decryption_response(
        &self,
        req: UserDecryptionRequest,
    ) -> Result<(), EventProcessingError> {
        let decryption_address =
            Address::from_str(&self.contracts.decryption_address).map_err(|_| {
                EventProcessingError::ConfigError(
                    crate::config::settings::AppConfigError::InvalidAddress(
                        "contracts.decryption_address".to_owned(),
                    ),
                )
            })?;
        self.tx_helper
            .send_transaction_simple("decryption_response", decryption_address, || {
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
    /// Dispatches [`RelayerEventData::DecryptionResponseRcvdFromGw`]
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
            PublicDecryptionEventData::EventLogRequestFromGw { log },
        ) = &event.data
        {
            match PublicDecryptionRequest::decode_log_data(log.data()) {
                Ok(req) => {
                    let public_decryption_id = req.publicDecryptionId;
                    info!(?public_decryption_id);

                    let mut ciphertext_handles: Vec<U256> = Vec::new();
                    for sns_ct_material in req.snsCtMaterials.clone() {
                        ciphertext_handles.push(sns_ct_material.ctHandle.into());
                    }

                    info!(
                        public_decryption_id = ?req.publicDecryptionId,
                        handles = ?ciphertext_handles,
                        "Processing PublicDecryptRequest event"
                    );

                    match self.send_public_decryption_response(req).await {
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
            UserDecryptionEventData::EventLogRequestFromGw { log },
        ) = &event.data
        {
            match UserDecryptionRequest::decode_log_data(log.data()) {
                Ok(req) => {
                    let user_decryption_id = req.userDecryptionId;
                    info!(?user_decryption_id);

                    info!(
                        user_decryption_id = ?req.userDecryptionId,
                        "Processing UserDecryptRequest event"
                    );

                    match self.send_user_decryption_response(req).await {
                        Ok(()) => {
                            info!("User decryption response sent successfully!");
                            return Ok(());
                        }
                        Err(e) => {
                            error!(?e, "User decryption response processing failed!");
                            return Err(EventProcessingError::HandlerError(
                                "Failed to decode user decrypt event data".to_owned(),
                            ));
                        }
                    }
                }
                Err(e) => {
                    error!(?e, "Failed to decode user decrypt event data");
                    return Err(EventProcessingError::HandlerError(
                        "Failed to decode user decrypt event data".to_owned(),
                    ));
                }
            }
        }
        Err(EventProcessingError::HandlerError(
            "Failed to decode user decrypt event data".to_owned(),
        ))
    }
}

#[async_trait]
impl EventHandler<GatewayProcessorsEvent> for GatewayProcessorsHandler {
    async fn handle_event(&self, event: GatewayProcessorsEvent) {
        match &event.clone().data {
            GatewayProcessorsEventData::KmsInput(input_event) => match input_event {
                GatewayProcessorsInputEventData::EventLogRequestFromGw { .. } => {
                    info!("Received input event log from Gateway");
                    match self.process_input_proof_request(&event).await {
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
                UserDecryptionEventData::EventLogRequestFromGw { .. } => {
                    info!("Received user decryption event log from Gateway");
                    match self.process_user_decryption_request(event).await {
                        Ok(()) => {
                            info!("User decrypt request processing succesfull!");
                        }
                        Err(e) => {
                            error!(?e, "User decrypt request processing failed!")
                        }
                    }
                }
            },
            GatewayProcessorsEventData::PublicDecrypt(public_decrypt_event) => {
                match public_decrypt_event {
                    PublicDecryptionEventData::EventLogRequestFromGw { .. } => {
                        info!("Received public decryption event log from Gateway");
                        match self.process_public_decryption_request(event).await {
                            Ok(()) => {
                                info!("Public request processing succesfull!");
                            }
                            Err(e) => {
                                error!(?e, "Public request processing failed!")
                            }
                        }
                    }
                }
            }
        }
    }
}
