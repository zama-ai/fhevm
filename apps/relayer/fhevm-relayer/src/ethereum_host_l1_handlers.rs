use crate::{
    errors::{EventProcessingError, TransactionServiceError},
    ethereum::{bindings::DecryptionOracle, callback_handler::CallbackHandler},
    orchestrator::{
        traits::{EventDispatcher, EventHandler},
        TokioEventDispatcher,
    },
    relayer_event::{DecryptedValue, DecryptionType, RelayerEvent, RelayerEventData},
    transaction::{TransactionService, TxConfig},
};
use alloy::primitives::{Address, FixedBytes, Uint};
use alloy::rpc::types::Log;
use async_trait::async_trait;
use std::{sync::Arc, time::Duration};
use tokio::task;
use tracing::{error, info};
use uuid::Uuid;

use alloy::primitives::{Bytes, U256};
use alloy_sol_types::SolEvent;

#[derive(Debug, Clone)]
pub struct DecryptionRequestData {
    pub host_l1_request_id: Uint<256, 4>,
    pub callback_selector: FixedBytes<4>,
    pub contract_caller: Address,
}

#[derive(Clone)]
pub struct EthereumHostL1Handler {
    dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
    context_data: dashmap::DashMap<Uuid, DecryptionRequestData>,
    tx_service: Arc<TransactionService>,
    tx_config: TxConfig,
}

impl EthereumHostL1Handler {
    pub fn new(
        dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
        tx_service: Arc<TransactionService>,
        tx_config: TxConfig,
    ) -> Self {
        Self {
            dispatcher,
            context_data: dashmap::DashMap::new(),
            tx_service,
            tx_config,
        }
    }

    async fn handle_public_decrypt_event_log(&self, event: RelayerEvent, eth_event_log: Log) {
        let next_event: RelayerEvent = match DecryptionOracle::DecryptionRequest::decode_log_data(
            eth_event_log.data(),
            true,
        ) {
            Ok(eth_decryption_request) => {
                self.context_data.insert(
                    event.request_id,
                    DecryptionRequestData {
                        host_l1_request_id: eth_decryption_request.requestID,
                        callback_selector: eth_decryption_request.callbackSelector,
                        contract_caller: eth_decryption_request.contractCaller,
                    },
                );
                info!(
                    "Decryption event log received from listener: request_id: {:?} block number: {:?}, ethereum_request_id: {:?}, selector {:?}",
                    event.request_id, eth_event_log.block_number, eth_decryption_request.requestID, eth_decryption_request.callbackSelector
                );

                let mut ct_handles: Vec<[u8; 32]> = Vec::new();
                for ct_handle in eth_decryption_request.cts {
                    // TODO: Check if to_le_bytes will work.
                    ct_handles.push(ct_handle.to_le_bytes());
                }
                event.derive_next_event(RelayerEventData::DecryptRequestRcvd {
                    ct_handles,
                    operation: DecryptionType::PublicDecrypt,
                })
            }
            Err(e) => event.derive_next_event(RelayerEventData::DecryptionFailed {
                error: format!("error decoding ethereum event log data: {:?}", e),
            }),
        };
        _ = self.dispatcher.dispatch_event(next_event).await;
    }

    async fn handle_decrypt_response(&self, event: RelayerEvent, decrypted_value: DecryptedValue) {
        match self.context_data.get(&event.request_id) {
            Some(decrypted_request_data) => {
                info!(
                    "Decryption response received: request_id: {:?}, value: {:?}",
                    event.request_id, decrypted_value,
                );
                // send the transaction using the request_id and callback selection from request data
                let req_clone = decrypted_request_data.clone();
                let self_clone = self.clone(); // Clone self since we need to move it to the task

                // Spawn a blocking task for the async operation
                task::spawn(async move {
                    if let Err(e) = self_clone.send_callback_transaction(&req_clone).await {
                        error!(?e, "Failed to send callback transaction");
                    }
                });
            }
            None => {
                let request_id = event.clone().request_id;
                info!("unknown request id: {:?}", request_id);
                let _next_event = event.derive_next_event(RelayerEventData::DecryptionFailed {
                    error: format!(
                        "httpz response received for unknown request id: {:?}",
                        &request_id
                    ),
                });
            }
        }
    }

    async fn try_send_callback(
        &self,
        req: &DecryptionRequestData,
    ) -> Result<(), EventProcessingError> {
        let decrypted_value = U256::from(18446744073709551600u64);
        let calldata = CallbackHandler::prepare_callback_data(req, decrypted_value, 4)?;

        info!(
            calldata = ?hex::encode(&calldata),
            "Submitting callback transaction"
        );

        let tx_hash = self
            .tx_service
            .submit_transaction(req.contract_caller, calldata, self.tx_config.clone())
            .await
            .map_err(EventProcessingError::from)?;

        info!(?tx_hash, "Waiting for transaction confirmation");

        match self.tx_service.get_transaction_status(tx_hash).await {
            Ok(Some(true)) => {
                info!(?tx_hash, "Transaction confirmed");
                Ok(())
            }
            Ok(Some(false)) => {
                Err(TransactionServiceError::Failed("Transaction reverted".into()).into())
            }
            Ok(None) => Err(TransactionServiceError::Failed("Transaction not found".into()).into()),
            Err(e) => Err(e.into()),
        }
    }

    async fn send_callback_transaction(
        &self,
        req: &DecryptionRequestData,
    ) -> Result<(), EventProcessingError> {
        const MAX_RETRIES: u32 = 3;
        let mut attempt = 0;

        while attempt < MAX_RETRIES {
            match self.try_send_callback(req).await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    if attempt < MAX_RETRIES - 1 {
                        error!(?e, attempt, "Transaction failed, retrying...");
                        tokio::time::sleep(Duration::from_secs(1 << attempt)).await;
                        attempt += 1;
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        Err(EventProcessingError::HandlerError(
            "Max retries exceeded".to_string(),
        ))
    }
}

#[async_trait]
impl EventHandler<RelayerEvent> for EthereumHostL1Handler {
    async fn handle_event(&self, event: RelayerEvent) {
        match event.clone().data {
            RelayerEventData::PubDecryptEventLogRcvdFromHostL1 {
                event_log: eth_event_log,
            } => {
                self.handle_public_decrypt_event_log(event, eth_event_log)
                    .await;
            }
            RelayerEventData::DecryptionResponseRcvdFromGwL2 { decrypted_value } => {
                info!("In ethereum_host_L1_handler, received  DecryptionResponseRcvdFromGwL2");
                self.handle_decrypt_response(event, decrypted_value).await;
            }
            _ => {
                return;
            }
        }
    }
}
