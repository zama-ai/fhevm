use crate::{
    config::settings::ContractConfig,
    errors::EventProcessingError,
    ethereum::{
        bindings::{DecyptionManager::PublicDecryptionRequest, ZKPoKManager},
        ComputeCalldata,
    },
    kms_connector_relayer_event::{KmsInputEventData, KmsRelayerEvent, KmsRelayerEventData},
    orchestrator::{traits::EventHandler, TokioEventDispatcher},
    transaction::{TransactionHelper, TransactionService, TxConfig},
    utils::{colorize_event_type, colorize_request_id},
};
use std::str::FromStr;

use alloy::primitives::{Address, U256};
use alloy_sol_types::SolEvent;
use async_trait::async_trait;
use std::{sync::Arc, time::Duration};
use tracing::{debug, error, info};

#[derive(Clone)]
pub struct KmsConnectorHandler {
    _dispatcher: Arc<TokioEventDispatcher<KmsRelayerEvent>>,
    tx_helper: Arc<TransactionHelper>,
    contracts: ContractConfig,
}

impl KmsConnectorHandler {
    pub fn new(
        _dispatcher: Arc<TokioEventDispatcher<KmsRelayerEvent>>,
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
        event: &KmsRelayerEvent,
    ) -> Result<(), EventProcessingError> {
        if let KmsRelayerEventData::KmsInput(KmsInputEventData::EventLogRequestFromGwL2 { log }) =
            &event.data
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

                    // Generate mock handles and signatures
                    // In real implementation, this would involve actual cryptographic operations
                    let signatures = vec![1u8; 65];

                    let handles = vec![[1u8; 32], [2u8; 32]];

                    self.send_input_response(request_event.zkProofId, handles, signatures)
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
        _signatures: Vec<u8>,
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
                ComputeCalldata::verify_proof_response(zkpok_id, handles.clone(), 4)
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
    async fn process_decryption_request(&self, event: KmsRelayerEvent) {
        info!(
            "Decryption request received. Prepare decryption response transaction to Gateway received. {:?}",
            event.request_id,
        );

        // Simulate some computation time

        tokio::time::sleep(Duration::from_secs(2)).await;

        if let KmsRelayerEventData::EventLogFromGwL2 { log } = &event.data {
            match PublicDecryptionRequest::decode_log_data(log.data(), true) {
                Ok(req) => {
                    let public_decryption_id = req.publicDecryptionId;
                    info!(?public_decryption_id,);
                    info!(
                        public_decryption_id = ?req.publicDecryptionId,
                        handles = ?req.ciphertextHandles,
                        "Processing DecryptRequest event"
                    );

                    match self.send_decryption_response(req).await {
                        Ok(()) => {
                            info!("Decryption response sent succesfull!");
                        }
                        Err(e) => {
                            error!(?e, "Decryption response processing failed!")
                        }
                    }
                }
                Err(e) => {
                    error!(?e, "Failed to decode event data");
                }
            }
        }
    }
}

#[async_trait]
impl EventHandler<KmsRelayerEvent> for KmsConnectorHandler {
    async fn handle_event(&self, event: KmsRelayerEvent) {
        info!(
            event_type = %colorize_event_type(event.data.as_ref()),
            request_id = %colorize_request_id(&event.request_id),
            "Processing kms input event"
        );

        match &event.data {
            KmsRelayerEventData::KmsInput(input_event) => match input_event {
                KmsInputEventData::EventLogRequestFromGwL2 { .. } => {
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
            KmsRelayerEventData::EventLogFromGwL2 { .. } => {
                info!("Received decryption event log from Gateway L2");
                self.process_decryption_request(event).await;
            }
        }
    }
}
