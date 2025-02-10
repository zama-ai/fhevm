use crate::{
    errors::{EventProcessingError, TransactionServiceError},
    orchestrator::{
        traits::{EventDispatcher, EventHandler},
        TokioEventDispatcher,
    },
    relayer_event::{DecryptedValue, RelayerEvent, RelayerEventData},
    transaction::{TransactionService, TxConfig},
};
use alloy::primitives::{hex, keccak256, Bytes, Uint, U256};
use async_trait::async_trait;
use std::{sync::Arc, time::Duration};
use tokio::task;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Clone)]
struct DecryptionResultData {
    gateway_l2_request_id: String,
}

#[derive(Clone)]
pub struct ArbitrumGatewayL2Handler {
    dispatcher: Arc<TokioEventDispatcher<RelayerEvent>>,
    context_data: dashmap::DashMap<Uuid, DecryptionResultData>,
    tx_service: Arc<TransactionService>,
    tx_config: TxConfig,
}

impl ArbitrumGatewayL2Handler {
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

    async fn mock_handle_decrypt_request_received(&self, event: RelayerEvent) {
        // TODO: make a tx to Rollup

        let handles: Vec<Uint<256, 4>> = vec![U256::from(1), U256::from(2)];

        // let next_event_data = RelayerEventData::DecryptionResponseRcvdFromGwL2 {
        //     decrypted_value: DecryptedValue::PublicDecrypt {
        //         plaintext: vec![1, 2, 3],
        //         signatures: vec![vec![1, 2, 3]],
        //     },
        // };
        info!(
            "Decryption request received. Making a tx to rollup: request_id: {:?}",
            event.request_id,
        );

        let self_clone = self.clone(); // Clone self since we need to move it to the task

        // Spawn a blocking task for the async operation
        task::spawn(async move {
            if let Err(e) = self_clone.send_callback_transaction(handles).await {
                error!(?e, "Failed to send callback transaction");
            }
        });
    }
    async fn handle_decrypt_reponse_event_log(&self, event: RelayerEvent) {
        info!(
            "Decryption response received. Trigger a tx to L1  {:?}",
            event.request_id,
        );
        let next_event_data = RelayerEventData::DecryptionResponseRcvdFromGwL2 {
            decrypted_value: DecryptedValue::PublicDecrypt {
                plaintext: vec![1, 2, 3],
                signatures: vec![vec![1, 2, 3]],
            },
        };

        let _ = self
            .dispatcher
            .dispatch_event(event.derive_next_event(next_event_data))
            .await;
    }

    async fn noop_handle_decrypt_reponse_event_log(&self, event: RelayerEvent) {}

    async fn try_send_callback(
        &self,
        handles: Vec<Uint<256, 4>>,
    ) -> Result<(), EventProcessingError> {
        let calldata = Self::prepare_callback_data(handles)?;

        let contract_address = hex!("2Fb4341027eb1d2aD8B5D9708187df8633cAFA92").into();

        info!(
            calldata = ?hex::encode(&calldata),
            "Submitting callback transaction"
        );

        let tx_hash = self
            .tx_service
            .submit_transaction(contract_address, calldata, self.tx_config.clone())
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

    pub(crate) fn prepare_callback_data(
        handles: Vec<Uint<256, 4>>,
    ) -> Result<Bytes, EventProcessingError> {
        let selector = &keccak256("publicDecryptionRequest(uint256[])")[..4];
        // Encode the parameters properly following ABI encoding rules
        let mut calldata = Vec::new();

        // 1. Add function selector
        calldata.extend_from_slice(selector);

        // 2. Add offset to start of array (32 bytes from start of parameters)
        calldata.extend_from_slice(&U256::from(32).to_be_bytes::<32>());

        // 3. Add array length
        calldata.extend_from_slice(&U256::from(handles.len()).to_be_bytes::<32>());

        // 4. Add array elements
        for handle in handles {
            calldata.extend_from_slice(&handle.to_be_bytes::<32>());
        }

        println!("Full calldata: 0x{}", hex::encode(&calldata));

        Ok(Bytes::from(calldata))
    }

    async fn send_callback_transaction(
        &self,
        handles: Vec<Uint<256, 4>>,
    ) -> Result<(), EventProcessingError> {
        const MAX_RETRIES: u32 = 3;
        let mut attempt = 0;

        while attempt < MAX_RETRIES {
            match self.try_send_callback(handles.clone()).await {
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
impl EventHandler<RelayerEvent> for ArbitrumGatewayL2Handler {
    async fn handle_event(&self, event: RelayerEvent) {
        match event.clone().data {
            RelayerEventData::DecryptRequestRcvd {
                ct_handles,
                operation,
            } => {
                self.mock_handle_decrypt_request_received(event).await;
            }
            RelayerEventData::DecryptResponseEventLogRcvdFromGwL2 { log: _ } => {
                self.handle_decrypt_reponse_event_log(event).await;
            }
            _ => {
                self.noop_handle_decrypt_reponse_event_log(event).await;
            }
        }
    }
}
