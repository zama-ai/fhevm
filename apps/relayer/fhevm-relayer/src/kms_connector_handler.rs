use crate::{
    errors::EventProcessingError,
    ethereum::{bindings::ZKPoKManager, ComputeCalldata},
    orchestrator::{traits::EventHandler, TokioEventDispatcher},
    relayer_event::{KmsInputEventData, RelayerEvent, RelayerEventData},
    transaction::{TransactionHelper, TransactionService, TxConfig},
    utils::{colorize_event_type, colorize_request_id},
};

use alloy::primitives::{Address, U256};
use alloy_sol_types::SolEvent;
use async_trait::async_trait;
use std::{sync::Arc, time::Duration};
use tracing::{debug, error, info};

const ZKPOK_MANAGER_ADDRESS: Address = Address::new([
    0x12, 0xB0, 0x64, 0xFB, 0x84, 0x5C, 0x1c, 0xc0, 0x5e, 0x94, 0x93, 0x85, 0x6a, 0x1D, 0x63, 0x7a,
    0x73, 0xe9, 0x44, 0xbE,
]);

#[derive(Clone)]
pub struct KmsConnectorHandler {
    dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
    tx_helper: Arc<TransactionHelper>,
}

impl KmsConnectorHandler {
    pub fn new(
        dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
        tx_service: Arc<TransactionService>,
        tx_config: TxConfig,
    ) -> Self {
        Self {
            dispatcher,
            tx_helper: Arc::new(TransactionHelper::new(tx_service, tx_config)),
        }
    }

    /// Process the InputRequest event and prepare response
    async fn process_input_request(
        &self,
        event: &RelayerEvent,
    ) -> Result<(), EventProcessingError> {
        if let RelayerEventData::KmsInput(KmsInputEventData::EventLogRequestFromGwL2 { log }) =
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
            error!("Invalid event type received");
            Err(EventProcessingError::HandlerError(
                "Invalid event type received".into(),
            ))
        }
    }

    /// Send InputResponse transaction
    async fn send_input_response(
        &self,
        zkpok_id: U256,
        handles: Vec<[u8; 32]>,
        signatures: Vec<u8>,
    ) -> Result<(), EventProcessingError> {
        info!(?zkpok_id, "Sending InputResponse transaction");

        self.tx_helper
            .send_transaction_simple("input_response", ZKPOK_MANAGER_ADDRESS, || {
                ComputeCalldata::verify_proof_response(zkpok_id, handles.clone(), 4)
            })
            .await?;

        Ok(())
    }
}

#[async_trait]
impl EventHandler<RelayerEvent> for KmsConnectorHandler {
    async fn handle_event(&self, event: RelayerEvent) {
        info!(
            event_type = %colorize_event_type(event.data.as_ref()),
            request_id = %colorize_request_id(&event.request_id),
            "Processing kms input event"
        );

        match &event.data {
            RelayerEventData::KmsInput(input_event) => match input_event {
                KmsInputEventData::EventLogRequestFromGwL2 { .. } => {
                    info!("Received input event log from Gateway L2");
                    self.process_input_request(&event).await;
                }
            },

            _ => {
                // Ignore other event types
            }
        }
    }
}
