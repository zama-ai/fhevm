use crate::{
    blockchain::ethereum::{bindings::Decryption, ComputeCalldata},
    config::settings::{ContractConfig, RetrySettings},
    core::{
        errors::EventProcessingError,
        event::{
            GenericEventData, PublicDecryptEventData, PublicDecryptResponse, RelayerEvent,
            RelayerEventData,
        },
    },
    orchestrator::{
        traits::{EventDispatcher, EventHandler},
        Orchestrator, TokioEventDispatcher,
    },
    transaction::{ReceiptProcessor, TransactionHelper, TransactionService, TxConfig},
};
use std::{str::FromStr, time::Duration};

use alloy::{
    network::{AnyReceiptEnvelope, AnyTransactionReceipt, ReceiptResponse},
    primitives::{Address, FixedBytes, U256},
    providers::ProviderBuilder,
    rpc::types::{Log, TransactionReceipt},
};

use alloy::sol_types::SolEvent;
use async_trait::async_trait;
use reqwest::Url;
use std::sync::Arc;
use tokio::task;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

struct PublicDecryptionRequestProcessor {
    handler: Arc<GatewayHandler>,
}

impl ReceiptProcessor for PublicDecryptionRequestProcessor {
    type Output = U256;

    fn process(
        &self,
        receipt: &AnyTransactionReceipt,
    ) -> Result<Self::Output, EventProcessingError> {
        self.handler
            .extract_public_decryption_id_from_receipt(receipt)
    }
}

#[derive(Clone)]
pub struct GatewayHandler {
    dispatcher: Arc<Orchestrator<TokioEventDispatcher<RelayerEvent>, RelayerEvent>>,
    public_decryption_id_to_request_id: Arc<dashmap::DashMap<U256, Uuid>>,
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
            public_decryption_id_to_request_id: Arc::new(dashmap::DashMap::new()),
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
    async fn send_public_decryption_request_to_gateway(
        &self,
        event: RelayerEvent,
        handles: Vec<[u8; 32]>,
    ) {
        let handles: Vec<FixedBytes<32>> = handles
            .iter()
            .map(|bytes| FixedBytes::from(*bytes))
            .collect();

        info!(
            "Decryption request received. Making a tx to gateway: request_id: {:?} with handles {:?}",
            event.request_id,
            handles
        );

        let url = match Url::parse(&self.gateway_http_url) {
            Ok(url) => url,
            Err(e) => {
                let error = EventProcessingError::HandlerError(format!("Invalid URL: {}", e));
                self.handle_failed_request(event, error).await;
                return;
            }
        };

        let provider = ProviderBuilder::new()
            .network::<alloy::network::AnyNetwork>()
            .on_http(url);

        let decryption_address = match Address::from_str(&self.contracts.decryption_address) {
            Ok(addr) => addr,
            Err(_) => {
                let error = EventProcessingError::ConfigError(
                    crate::config::settings::AppConfigError::InvalidAddress(
                        "contracts.decryption_address".to_owned(),
                    ),
                );
                self.handle_failed_request(event, error).await;
                return;
            }
        };

        let decryption = Decryption::new(decryption_address, provider.clone());

        let max_retries = self.retry_config.max_attempts;
        let retry_interval = Duration::from_secs(self.retry_config.base_delay_secs);

        let mut retries = 0;
        let mut should_retry = true;

        if let Some(retry_config) = &self.tx_helper.tx_config.retry_config {
            if retry_config.mock_mode {
                should_retry = false;
            }
        }

        while should_retry && retries < max_retries {
            should_retry = false;

            match decryption
                .clone()
                .checkPublicDecryptionReady(handles.clone())
                .call()
                .await
            {
                Ok(_) => {
                    info!("Function call succeeded for handles: {:?}", handles);
                }
                Err(err) => {
                    info!("Gateway not ready for handles: {:?}, retrying... ", handles);
                    debug!("Gateway not ready yet: {:?} error info: {}", handles, err);
                    should_retry = true;
                }
            }

            if should_retry {
                retries += 1;
                if retries < max_retries {
                    info!(
                        "Retrying public decryption readiness check (attempt {}/{})",
                        retries, max_retries
                    );
                    tokio::time::sleep(retry_interval).await;
                } else {
                    warn!("Max retries reached for public decryption readiness check");

                    // Return an error instead of proceeding with the transaction
                    let error = EventProcessingError::HandlerError(format!(
                        "Gateway not ready after {} retries",
                        max_retries
                    ));
                    self.handle_failed_request(event, error).await;
                    return;
                }
            }
        }

        let self_clone = self.clone();
        let event_clone = event.clone();

        // Spawn a blocking task to make a transaction to gateway
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
                gw_req_reference_id: decryption_public_id,
            },
        ));

        if let Err(e) = self.dispatcher.dispatch_event(next_event).await {
            error!(?e, "Failed to dispatch DecryptRequestProcessed event");
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
    /// Dispatches [`RelayerEventData::DecryptionResponseRcvdFromGw`]
    async fn handle_decrypt_reponse_event_log(&self, event: RelayerEvent) {
        info!(
            "Public Decryption response received. Trigger a tx to L1  {:?}",
            event.request_id,
        );

        if let RelayerEventData::Generic(GenericEventData::EventLogFromGw { log }) = &event.data {
            if let Some(topic) = log.topic0() {
                if *topic == Decryption::PublicDecryptionResponse::SIGNATURE_HASH {
                    match Decryption::PublicDecryptionResponse::decode_log_data(log.data(), true) {
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
            "Transaction to gateway has been done, the associated public decryption id is {}",
            id
        );
    }

    fn extract_public_decryption_id_from_receipt(
        &self,
        receipt: &AnyTransactionReceipt,
    ) -> Result<U256, EventProcessingError> {
        let target_topic = Decryption::PublicDecryptionRequest::SIGNATURE_HASH;
        info!(
            "Looking for topic: {}",
            Decryption::PublicDecryptionRequest::SIGNATURE
        );

        let receipt: TransactionReceipt<AnyReceiptEnvelope<Log>> = receipt.inner.clone();

        debug!(
            "Receipt details for public decryption:\n\
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
                    return match Decryption::PublicDecryptionRequest::decode_log_data(
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

    async fn noop_handle_decrypt_reponse_event_log(&self, _event: &RelayerEvent) {}

    /// Processes a decryption request by sending it to the gateway contract.
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
        handles: Vec<FixedBytes<32>>,
    ) -> Result<U256, EventProcessingError> {
        let processor = PublicDecryptionRequestProcessor {
            handler: Arc::new(self.clone()),
        };

        let decryption_address =
            Address::from_str(&self.contracts.decryption_address).map_err(|_| {
                EventProcessingError::ConfigError(
                    crate::config::settings::AppConfigError::InvalidAddress(
                        "contracts.decryption_address".to_owned(),
                    ),
                )
            })?;
        self.tx_helper
            .send_transaction(
                "decryption_request",
                decryption_address,
                || ComputeCalldata::public_decryption_req(handles.clone()),
                &processor,
            )
            .await
    }
}

#[async_trait]
impl EventHandler<RelayerEvent> for GatewayHandler {
    async fn handle_event(&self, event: RelayerEvent) {
        match event.data {
            RelayerEventData::PublicDecrypt(PublicDecryptEventData::ReqRcvdFromFhevm {
                ref decrypt_request,
                ..
            }) => {
                let handles = decrypt_request.ct_handles.clone();
                self.send_public_decryption_request_to_gateway(event, handles)
                    .await;
            }
            RelayerEventData::Generic(GenericEventData::EventLogFromGw { ref log }) => {
                if let Some(topic0) = log.topic0() {
                    if FixedBytes::<32>::from_slice(topic0.as_slice())
                        != Decryption::PublicDecryptionResponse::SIGNATURE_HASH
                    {
                        debug!(
                            "Ignore this event: expected event: {:?}, received {} ",
                            log.topic0(),
                            Decryption::PublicDecryptionResponse::SIGNATURE_HASH
                        );
                        self.noop_handle_decrypt_reponse_event_log(&event).await;
                    } else {
                        self.handle_decrypt_reponse_event_log(event).await;
                    }
                };
            }
            RelayerEventData::PublicDecrypt(PublicDecryptEventData::ReqSentToGw {
                gw_req_reference_id,
            }) => {
                self.handle_decrypt_request_sent(gw_req_reference_id);
            }
            _ => {
                self.noop_handle_decrypt_reponse_event_log(&event).await;
            }
        }
    }
}
