// TODO: handle response happening before receipt for pub-dec
// TODO: any unexpected failure on http request should return a message to the orchestrator queue
// TODO: any unexpected failure should be put in a specific table
// TODO: any unexpected on-chain failure should be exposed to the users
// Maybe through some special interface in the console requiring logging?
// Or through some public failure board with the error cause
// TODO: refactoring to clean things
// TODO: properly define failure modes
//  - blockchain error
//  - connectivity error
//      - chain
//      - db
//      - sqs
//  - ???
// And for each define mitigation/recovery mechanism
// TODO: add tracing based on request-id
// TODO: if an error occurs then return an error to the console-queue
// with context of the event that failed
// TODO: We make the assumption that all events come from an EVM chain
// we should split host and gateway events, and probably have the events be parsed in
// the listeners instead of the handler such that the handler is agnostic of the
// chains and the listeners are specialized.
// Such that each listener implements chain-specific logic to parse the input

use crate::events::*;
use alloy::{
    primitives::{Address, FixedBytes, LogData},
    providers::ProviderBuilder,
    sol_types::SolEvent,
};
use std::fmt::Write;
// use alloy_sol_types::SolEvent;
use async_trait::async_trait;
use diesel::{Connection, PgConnection};
use fhevm_relayer::blockchain::ethereum::bindings::Decryption::{self, CtHandleContractPair};
use fhevm_relayer::blockchain::PublicDecryptFhevmRequestData;
use fhevm_relayer::http::userdecrypt_http_listener::UserDecryptResponsePayloadJson;
use fhevm_relayer::orchestrator::traits::EventDispatcher;
use fhevm_relayer::orchestrator::{Orchestrator, TokioEventDispatcher};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fmt::Display;
use std::sync::Arc;

use fhevm_relayer::{
    blockchain::ethereum::{
        bindings::{
            Decryption::{
                PublicDecryptionRequest, PublicDecryptionResponse, UserDecryptionRequest,
                UserDecryptionResponse,
            },
            DecryptionOracle::DecryptionRequest,
            InputVerification::{VerifyProofRequest, VerifyProofResponse},
        },
        ComputeCalldata,
    },
    orchestrator::traits::{Event, EventHandler},
    transaction::{sender::RetryConfig, TransactionService, TxConfig},
};
use tracing::{debug, error, info, instrument, warn};

/// Mock handler for Transaction Manager
#[derive(Debug)]
pub struct ZWSTransactionManagerMockHandler {
    sqs_client: aws_sdk_sqs::Client,
    queue_url: String,
    // TODO: mapping chain-id to transaction-service
    transaction_services: HashMap<u64, Arc<TransactionService>>,
}

impl ZWSTransactionManagerMockHandler {
    pub async fn new(
        queue_url: String,
        transaction_services: HashMap<u64, Arc<TransactionService>>,
    ) -> Self {
        let config = aws_config::from_env().load().await;

        let sqs_client = aws_sdk_sqs::Client::new(&config);
        ZWSTransactionManagerMockHandler {
            sqs_client,
            queue_url,
            transaction_services,
        }
    }
}

// TODO: should probably include user-address too
pub async fn wait_for_ct_priv_dec_availability(
    handles_pairs: Vec<CtHandleContractPair>,
    decryption_address: Address,
    gateway_http_url: String,
    user_address: Address,
) -> Result<(), String> {
    let url = match Url::parse(&gateway_http_url) {
        Ok(url) => url,
        Err(e) => {
            return Err(format!("Invalid URL {} : {}", gateway_http_url, e).to_string());
        }
    };
    let provider = ProviderBuilder::new().connect_http(url);
    let decryption = Decryption::new(decryption_address, provider.clone());
    let max_retries = 120;
    let retry_interval = core::time::Duration::from_millis(1000);

    let mut retries = 0;
    let mut should_retry = true;

    let handles: Vec<FixedBytes<32>> = handles_pairs.iter().map(|elt| elt.ctHandle).collect();
    while should_retry && retries < max_retries {
        should_retry = false;

        match decryption
            .clone()
            .checkUserDecryptionReady(user_address, handles_pairs.clone())
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
                // NOTE: we should probably define at which level logging is done to avoid
                // duplicate logs without context
                error!(
                    "Max retries {} reached for public decryption readiness check",
                    max_retries
                );
                return Err(format!("Max retries reached: {}", max_retries).to_string());
            }
        }
    }
    Ok(())
}

/// Wait for public decryption ciphertexts to be available
///
/// This function returns an error if the number of max-retries is reached before the ciphertexts
/// are available for public decryption.
/// TODO: parametrize wait-time and retry
/// btw retry mechanisms with timeouts is something that could probably be factorized
pub async fn wait_for_ct_pub_dec_availability(
    handles: Vec<FixedBytes<32>>,
    decryption_address: Address,
    gateway_http_url: String,
) -> Result<(), String> {
    let url = match Url::parse(&gateway_http_url) {
        Ok(url) => url,
        Err(e) => {
            return Err(format!("Invalid URL {} : {}", gateway_http_url, e).to_string());
        }
    };
    let provider = ProviderBuilder::new().connect_http(url);
    let decryption = Decryption::new(decryption_address, provider.clone());
    let max_retries = 120;
    let retry_interval = core::time::Duration::from_millis(1000);

    let mut retries = 0;
    let mut should_retry = true;

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
                // NOTE: we should probably define at which level logging is done to avoid
                // duplicate logs without context
                error!(
                    "Max retries {} reached for public decryption readiness check",
                    max_retries
                );
                return Err(format!("Max retries reached: {}", max_retries).to_string());
            }
        }
    }
    Ok(())
}

#[async_trait]
impl EventHandler<ZwsRelayerEvent> for ZWSTransactionManagerMockHandler {
    // TODO: Make sure that we do log something about the event in the trace
    #[instrument(skip(self), fields(event=%event))]
    async fn handle_event(&self, event: ZwsRelayerEvent) {
        // Match log
        match event {
            ZwsRelayerEvent::TransactionRequest(transaction_request) => {
                debug!("TX Manager received: {}", transaction_request);
                let transaction_service =
                    match self.transaction_services.get(&transaction_request.chain_id) {
                        Some(value) => value,
                        None => {
                            error!(
                                "Couldn't fetch transaction service for chain-id {:?}",
                                transaction_request.chain_id
                            );
                            return;
                        }
                    };
                debug!(
                    "Making transaction to: {:?}, {:?}",
                    transaction_request.chain_id, transaction_request.address,
                );

                // TODO: parametrize Transaction Manager
                let tx_config = TxConfig {
                    gas_limit: Some(150000000),
                    max_priority_fee: Some(2000000000),
                    value: None,
                    nonce: None,
                    confirmations: Some(1),
                    timeout_secs: Some(60),
                    // retry_config: Some(RetryConfig::default()),
                    // TODO: this appears to not be used anywhere
                    retry_config: Some(RetryConfig {
                        max_attempts: 10,
                        base_delay: core::time::Duration::from_secs(1),
                        max_delay: core::time::Duration::from_secs(60),
                        mock_mode: false,
                    }),
                };

                let tx_request_response_result = transaction_service
                    .submit_and_wait(
                        transaction_request.address,
                        transaction_request.calldata.clone(),
                        tx_config,
                    )
                    .await;

                let tx_request_response = match tx_request_response_result {
                    Ok(value) => value,
                    Err(error) => {
                        // TODO: error handling and return transaction failure or
                        // something
                        error!("Something went wrong: {:?}", error);
                        return;
                    }
                };

                let tx_receipt = match transaction_service
                    .get_transaction_receipt(tx_request_response.transaction_hash)
                    .await
                {
                    Ok(value) => value,
                    Err(error) => {
                        error!("Error trying to get transaction receipt: {:?}", error);
                        return;
                    }
                };

                // TODO: check tx-request-response logic
                let message = ZwsRelayerEvent::TransactionResponse(Box::new(TransactionResponse {
                    request_id: transaction_request.request_id(),
                    receipt: tx_receipt.inner,
                    timestamp: current_timestamp(),
                }));

                let _ = send_message_to_sqs_queue_empty(
                    true,
                    &self.sqs_client,
                    &self.queue_url,
                    &message,
                )
                .await;
            }
            _ => {
                warn!(
                    "Not handled event {:?} {:?}",
                    event.event_name(),
                    event.request_id(),
                );
            }
        }
    }
}

/// Mock handler for Console
#[derive(Debug)]
pub struct ZWSConsoleMockHandler {
    sqs_client: aws_sdk_sqs::Client,
    queue_url: String,
}

impl ZWSConsoleMockHandler {
    pub async fn new(queue_url: String) -> Self {
        let config = aws_config::from_env().load().await;
        let sqs_client = aws_sdk_sqs::Client::new(&config);
        ZWSConsoleMockHandler {
            sqs_client,
            queue_url,
        }
    }
    pub async fn default() -> Self {
        Self::new(String::from(
            "http://sqs.eu-central-1.localhost.localstack.cloud:4566/000000000000/relayer-queue",
        ))
        .await
    }
}

// NOTE: add debug handler that allows any PaymentOracleAuthorizationRequest to mock the Console behavior
// this could be activated with an env var flag

#[async_trait]
impl EventHandler<ZwsRelayerEvent> for ZWSConsoleMockHandler {
    // TODO: Make sure that we do log something about the event in the trace
    #[instrument(skip(self), fields(event=%event))]
    async fn handle_event(&self, event: ZwsRelayerEvent) {
        // Match log
        match event {
            ZwsRelayerEvent::OracleAuthorizationRequest(authorization_request) => {
                info!("Received authorization request from SQS pushing auth response to SQS relayer queue.");
                // NOTE: this is just a mock of the Console so we authorized all requests
                let message =
                    ZwsRelayerEvent::OracleAuthorizationResponse(OracleAuthorizationResponse {
                        request_id: authorization_request.request_id(),
                        authorized: true,
                        timestamp: current_timestamp(),
                    });

                // SQS
                let _ = send_message_to_sqs_queue_empty(
                    true,
                    &self.sqs_client,
                    &self.queue_url,
                    &message,
                )
                .await;
            }
            _ => {
                warn!(
                    "Not handled event {:?} {:?}",
                    event.event_name(),
                    event.request_id(),
                );
            }
        }
    }
}

pub struct ZWSRelayerHandler {
    sqs_client: aws_sdk_sqs::Client,
    console_queue_url: String,
    tx_manager_queue_url: String,
    orchestrator: Arc<Orchestrator<TokioEventDispatcher<ZwsRelayerEvent>, ZwsRelayerEvent>>,
    zkpok_manager_address: Address,
    decryption_manager_address: Address,
    gateway_chain_id: u64,
    gateway_http_url: String,
}

// TODO: pass full configuration to handler
impl ZWSRelayerHandler {
    pub async fn new(
        console_queue_url: String,
        tx_manager_queue_url: String,
        orchestrator: Arc<Orchestrator<TokioEventDispatcher<ZwsRelayerEvent>, ZwsRelayerEvent>>,
        zkpok_manager_address: Address,
        decryption_manager_address: Address,
        gateway_chain_id: u64,
        gateway_http_url: String,
    ) -> Self {
        let config = aws_config::from_env().load().await;
        debug!("{:?}", config);
        let sqs_client = aws_sdk_sqs::Client::new(&config);

        ZWSRelayerHandler {
            sqs_client,
            console_queue_url,
            tx_manager_queue_url,
            orchestrator,
            zkpok_manager_address,
            decryption_manager_address,
            gateway_chain_id,
            gateway_http_url,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SupportedBlockchainEvent {
    DecryptionRequest(DecryptionRequest),           // Oracle event
    UserDecryptionResponse(UserDecryptionResponse), // Decryption Manager event
    PublicDecryptionResponse(PublicDecryptionResponse), // Decryption Manager event
    VerifyProofResponse(VerifyProofResponse),       // Zkpok Manager event
    // Unused - Processed by FHEVM stack services
    VerifyProofRequest(VerifyProofRequest),
    UserDecryptionRequest(UserDecryptionRequest),
    PublicDecryptionRequest(PublicDecryptionRequest),
}

// TODO: add decoded ciphertext handles, addresses, ...
impl Display for SupportedBlockchainEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedBlockchainEvent::DecryptionRequest(_) => {
                write!(f, "DecryptionRequest")
            }
            SupportedBlockchainEvent::PublicDecryptionRequest(_) => {
                write!(f, "PublicDecryptionRequest")
            }
            SupportedBlockchainEvent::UserDecryptionRequest(_) => {
                write!(f, "DecryptionRequest")
            }
            SupportedBlockchainEvent::VerifyProofRequest(_) => {
                write!(f, "VerifyProofRequest")
            }
            SupportedBlockchainEvent::PublicDecryptionResponse(_) => {
                write!(f, "PublicDecryptionResponse")
            }
            SupportedBlockchainEvent::UserDecryptionResponse(_) => {
                write!(f, "DecryptionResponse")
            }
            SupportedBlockchainEvent::VerifyProofResponse(_) => {
                write!(f, "VerifyProofResponse")
            }
        }
    }
}

impl SupportedBlockchainEvent {
    pub fn decode_log_data(log: &LogData) -> Result<SupportedBlockchainEvent, std::string::String> {
        if let Ok(event) = DecryptionRequest::decode_log_data(log) {
            return Ok(SupportedBlockchainEvent::DecryptionRequest(event));
        };
        if let Ok(event) = UserDecryptionResponse::decode_log_data(log) {
            return Ok(SupportedBlockchainEvent::UserDecryptionResponse(event));
        };
        if let Ok(event) = PublicDecryptionResponse::decode_log_data(log) {
            return Ok(SupportedBlockchainEvent::PublicDecryptionResponse(event));
        };
        if let Ok(event) = VerifyProofResponse::decode_log_data(log) {
            return Ok(SupportedBlockchainEvent::VerifyProofResponse(event));
        };
        if let Ok(event) = VerifyProofRequest::decode_log_data(log) {
            return Ok(SupportedBlockchainEvent::VerifyProofRequest(event));
        };
        if let Ok(event) = UserDecryptionRequest::decode_log_data(log) {
            return Ok(SupportedBlockchainEvent::UserDecryptionRequest(event));
        };
        if let Ok(event) = PublicDecryptionRequest::decode_log_data(log) {
            return Ok(SupportedBlockchainEvent::PublicDecryptionRequest(event));
        };
        Err("Failed to decode log data into any supported event type".to_string())
    }
}

// NOTE: add debug handler that allows any PaymentOracleAuthorizationRequest to mock the Console behavior
// this could be activated with an env var flag
//
fn fetch_or_store_request_from_response(
    on_chain_req_id: Vec<u8>,
    op_type: GatewayOperation,
    db_connection: &mut PgConnection,
    event: BlockchainEvent,
) -> Result<std::option::Option<uuid::Uuid>, String> {
    match fetch_gateway_request_chain_id(db_connection, on_chain_req_id.clone(), op_type) {
        Ok(rows) => match rows.first() {
            Some(value) => Ok(Some(value.request_id)),
            None => {
                debug!("Couldn't find request in db, storing in db.");
                debug!("Storing response in db (in case of race-condition with request receipt)");
                let insertion_result = create_gateway_response(
                    db_connection,
                    NewGatewayResponseRow {
                        on_chain_request_id: on_chain_req_id,
                        event_log: diesel_json::Json(event.event_log),
                        op: op_type,
                    },
                );
                match insertion_result {
                    Ok(_) => {
                        debug!("Insertion success");
                        Ok(None)
                    }
                    Err(error) => {
                        let msg = format!("Insertion failure: {:?}", error);
                        Err(msg.to_string())
                    }
                }
            }
        },
        Err(error) => {
            debug!(
                "Failed to find request in db because of an error: {:?}",
                error
            );
            debug!("Storing response in db (in case of race-condition with request receipt)");
            let insertion_result = create_gateway_response(
                db_connection,
                NewGatewayResponseRow {
                    on_chain_request_id: on_chain_req_id,
                    event_log: diesel_json::Json(event.event_log),
                    op: op_type,
                },
            );
            match insertion_result {
                Ok(_) => {
                    debug!("Insertion success");
                    Ok(None)
                }
                Err(error) => {
                    let msg = format!("Insertion failure: {:?}", error);
                    Err(msg.to_string())
                }
            }
        }
    }
}

fn fixed_bytes_to_hex_string(bytes: &FixedBytes<32>) -> String {
    // Preallocate a String with appropriate capacity (2 hex chars per byte + "0x" prefix)
    let mut result = String::with_capacity(2 + 32 * 2);

    // Add "0x" prefix
    result.push_str("0x");

    // Convert each byte to hex and append to the result
    for byte in bytes.as_slice() {
        write!(result, "{:02x}", byte).unwrap();
    }

    result
}

impl ZWSRelayerHandler {
    #[instrument(skip(self, db_connection), fields(event=%response))]
    async fn handle_transaction_response(
        &self,
        response: TransactionResponse,
        mut db_connection: PgConnection,
    ) {
        // TODO: Support other op-types
        // TODO: Emit transaction-receipt event to SQS

        // TODO: check why this is here and add comment
        let _gateway_request = match fetch_gateway_request(&mut db_connection, response.request_id)
        {
            Ok(value) => value,
            Err(error) => {
                debug!("Value not in DB: {:?}", error);
                return;
            }
        };

        let parsed = response
            .receipt
            .inner
            .logs()
            .iter()
            .map(|log| SupportedBlockchainEvent::decode_log_data(log.data()))
            .next();

        let (on_chain_id, op_type) = match parsed {
            Some(Ok(underlying_value)) => match underlying_value {
                SupportedBlockchainEvent::VerifyProofRequest(sub_value) => {
                    // Store on-chain-request-id + update operation status in db
                    let on_chain_id = sub_value.zkProofId.to_be_bytes_vec();
                    (on_chain_id, GatewayOperation::InputRegistration)
                }
                SupportedBlockchainEvent::UserDecryptionRequest(sub_value) => {
                    let on_chain_id = sub_value.userDecryptionId.to_be_bytes_vec();
                    (on_chain_id, GatewayOperation::PrivateDecryption)
                }
                SupportedBlockchainEvent::PublicDecryptionRequest(sub_value) => {
                    let on_chain_id = sub_value.publicDecryptionId.to_be_bytes_vec();
                    (on_chain_id, GatewayOperation::PublicDecryption)
                }
                _ => {
                    error!("Unsupported transaction receipt: {:?}", underlying_value);
                    return;
                }
            },
            _ => {
                error!("Unsupported transaction receipt: {:?}", parsed);
                return;
            }
        };

        match update_gateway_request_onchain_id(
            &mut db_connection,
            response.request_id,
            GatewayOperationStatus::TXFulfilled,
            Some(on_chain_id.clone()),
        ) {
            Ok(value) => {
                info!("Insertion successful: {:?}", value);
            }
            Err(error) => {
                error!("Insertion error: {:?}", error);
            }
        };

        // Check if response is already present
        match fetch_gateway_response(&mut db_connection, on_chain_id, op_type) {
            Ok(db_response) => match db_response.first() {
                Some(elt) => {
                    let new_event = ZwsRelayerEvent::BlockchainEvent(BlockchainEvent {
                        request_id: response.request_id,
                        event_log: elt.event_log.0.clone(),
                        chain_id: self.gateway_chain_id,
                        timestamp: current_timestamp(),
                    });

                    // Re-emit internally the event
                    debug!("response found in db (receipt)");
                    let dispatch = self.orchestrator.dispatch_event(new_event).await;
                    debug!("response dispatched: {:?}", dispatch);
                }
                None => {
                    debug!("response not already there");
                }
            },
            Err(_) => {
                debug!("response not already there");
            }
        }
    }

    // TODO: factorize this
    #[instrument(skip(self, db_connection), fields(event=%event))]
    async fn handle_blockchain_event(
        &self,
        event: BlockchainEvent,
        mut db_connection: PgConnection,
    ) {
        // NOTE: There is a potential race condition between the result being posted
        // on-chain and the receipt of the transaction being obtained.
        // As such we need to store all on-chain events in a database for further
        // use even if they don't match a known request yet.
        // We should add a date and a block-number to expire responses.

        match SupportedBlockchainEvent::decode_log_data(event.event_log.data()) {
            Ok(decoded_event) => {
                debug!("Successfuly decoded event: {}", decoded_event);

                match decoded_event {
                    SupportedBlockchainEvent::DecryptionRequest(decryption_request) => {
                        match create_host_event(&mut db_connection, event.clone()) {
                            Ok(_) => {
                                debug!("Host event pushed to db");
                            }
                            Err(error) => {
                                error!("Failed to push event to db: {error}");

                                // Publish authorization request to SQS
                                let message =
                                    ZwsRelayerEvent::UnrecoverableError(UnrecoverableError {
                                        request_id: event.request_id(),
                                        event: RelayerEventNoError::BlockchainEvent(event),
                                        timestamp: current_timestamp(),
                                    });
                                let _ = send_message_to_sqs_queue_empty(
                                    true,
                                    &self.sqs_client,
                                    &self.console_queue_url,
                                    &message,
                                )
                                .await;
                                return;
                            }
                        }

                        // Match and decode host decryption request event
                        info!("Decryption request: {:?}", decryption_request);
                        info!("Decryption event log received from listener: block number: {:?}, ethereum_request_id: {:?}, selector {:?}",
                            event.event_log.block_number, decryption_request.requestID, decryption_request.callbackSelector);
                        let mut ct_handles: Vec<[u8; 32]> = Vec::new();
                        for ct_handle in &decryption_request.cts {
                            ct_handles.push((*ct_handle).into());
                        }
                        let contract_caller = decryption_request.contractCaller;

                        // Publish authorization request to SQS
                        let message = ZwsRelayerEvent::OracleAuthorizationRequest(
                            OracleAuthorizationRequest {
                                request_id: event.request_id(),
                                caller_address: contract_caller,
                                timestamp: current_timestamp(),
                            },
                        );
                        let _ = send_message_to_sqs_queue_empty(
                            true,
                            &self.sqs_client,
                            &self.console_queue_url,
                            &message,
                        )
                        .await;
                    }
                    SupportedBlockchainEvent::VerifyProofResponse(verification_response) => {
                        // Check if on-chain request-id in db, if-not store response on-chain-request-id
                        // in db with how to retrieve it.
                        let zkpokid = verification_response.zkProofId.to_be_bytes_vec();
                        match fetch_or_store_request_from_response(
                            zkpokid.clone(),
                            GatewayOperation::InputRegistration,
                            &mut db_connection,
                            event.clone(),
                        ) {
                            Ok(value) => match value {
                                Some(source_request_id) => {
                                    debug!(
                                        "request-id: {}, ct-handles: [{:?}]",
                                        source_request_id,
                                        verification_response
                                            .ctHandles
                                            .iter()
                                            .map(fixed_bytes_to_hex_string)
                                            .collect::<Vec<String>>()
                                            .join(", ")
                                    );
                                    let message = ZwsRelayerEvent::HTTPInputRegistrationResponse(
                                        HTTPInputRegistrationResponse {
                                            request_id: source_request_id,
                                            handles: verification_response.ctHandles,
                                            signatures: verification_response.signatures,
                                            timestamp: current_timestamp(),
                                        },
                                    );

                                    let _ = send_message_to_sqs_queue_empty(
                                        true,
                                        &self.sqs_client,
                                        &self.console_queue_url,
                                        &message,
                                    )
                                    .await;
                                }
                                None => {
                                    debug!("Couldnt' find request-id from response-id");
                                }
                            },
                            Err(error) => {
                                error!(error);
                                return;
                            }
                        }
                    }
                    SupportedBlockchainEvent::UserDecryptionResponse(
                        private_decryption_response,
                    ) => {
                        // Check if on-chain request-id in db, if-not store response on-chain-request-id
                        // in db with how to retrieve it.
                        let private_decryption_id = private_decryption_response
                            .userDecryptionId
                            .to_be_bytes_vec();
                        match fetch_or_store_request_from_response(
                            private_decryption_id.clone(),
                            GatewayOperation::PrivateDecryption,
                            &mut db_connection,
                            event.clone(),
                        ) {
                            Ok(value) => match value {
                                Some(source_request_id) => {
                                    let response: Vec<UserDecryptResponsePayloadJson> =
                                        private_decryption_response
                                            .reencryptedShares
                                            .iter()
                                            .zip(private_decryption_response.signatures.iter())
                                            .map(|(payload, signature)| {
                                                UserDecryptResponsePayloadJson {
                                                    payload: payload.clone(),
                                                    signature: signature.clone(),
                                                }
                                            })
                                            .collect();
                                    let message = ZwsRelayerEvent::HTTPPrivateDecryptionResponse(
                                        PrivateDecryptionResponse {
                                            request_id: source_request_id,
                                            response,
                                            timestamp: current_timestamp(),
                                        },
                                    );
                                    let _ = send_message_to_sqs_queue_empty(
                                        true,
                                        &self.sqs_client,
                                        &self.console_queue_url,
                                        &message,
                                    )
                                    .await;
                                }
                                None => {
                                    debug!("Couldnt' find request-id from response-id");
                                }
                            },
                            Err(error) => {
                                error!(error);
                                return;
                            }
                        }
                    }
                    SupportedBlockchainEvent::PublicDecryptionResponse(
                        public_decryption_response,
                    ) => {
                        // NOTE: we should add an expiration date to events to avoid storing
                        // un-used/old events

                        // 1. Check if on-chain request-id in db:
                        // 1.1. if-not store response response in db in case the receipt comes
                        //   later.
                        // 1.2. if-so fetch request from db and process callback

                        let public_decryption_id = public_decryption_response
                            .publicDecryptionId
                            .to_be_bytes_vec();

                        match fetch_or_store_request_from_response(
                            public_decryption_id.clone(),
                            GatewayOperation::PublicDecryption,
                            &mut db_connection,
                            event.clone(),
                        ) {
                            Ok(value) => match value {
                                Some(source_request_id) => {
                                    // Fetch request using source-request-id
                                    let response = match fetch_host_event(
                                        &mut db_connection,
                                        source_request_id,
                                    ) {
                                        Ok(value) => value,
                                        Err(error) => {
                                            error!(
                                    "Couldn't find host event in DB for request-id: {:?} {:?}",
                                    source_request_id, error
                                );
                                            return;
                                        }
                                    };

                                    debug!("DB Fetch Response: {:?}", response);
                                    // Craft callback
                                    match SupportedBlockchainEvent::decode_log_data(
                                        response.event_log.data(),
                                    ) {
                                        Ok(event) => match event {
                                            SupportedBlockchainEvent::DecryptionRequest(value) => {
                                                let public_decrypt_response: PublicDecryptionResponse =
                                        PublicDecryptionResponse {
                                            publicDecryptionId: public_decryption_response
                                                .publicDecryptionId,
                                            decryptedResult: public_decryption_response
                                                .decryptedResult,
                                            signatures: public_decryption_response.signatures,
                                        };
                                                let public_decrypt_resquest =
                                                    PublicDecryptFhevmRequestData {
                                                        fhevm_request_id: value.requestID,
                                                        callback_selector: value.callbackSelector,
                                                        contract_caller: value.contractCaller,
                                                    };

                                                let calldata = match ComputeCalldata::callback_req(
                                                    &public_decrypt_resquest,
                                                    public_decrypt_response.clone(),
                                                ) {
                                                    Ok(result) => result,
                                                    Err(error) => {
                                                        error!("{:?}", error);
                                                        return;
                                                    }
                                                };
                                                let message = ZwsRelayerEvent::TransactionRequest(
                                                    TransactionRequest {
                                                        request_id: source_request_id,
                                                        address: value.contractCaller,
                                                        chain_id: response.chain_id,
                                                        calldata,
                                                        timestamp: current_timestamp(),
                                                    },
                                                );

                                                // NOTE: adding a name to the tx-request might ease debugging
                                                debug!("Sending callback tx request");
                                                let _ = send_message_to_sqs_queue_empty(
                                                    true,
                                                    &self.sqs_client,
                                                    &self.tx_manager_queue_url,
                                                    &message,
                                                )
                                                .await;
                                            }
                                            _ => {
                                                debug!("Not supported")
                                            }
                                        },
                                        _ => {
                                            debug!("Not supported")
                                        }
                                    }
                                }
                                None => {
                                    debug!("Couldnt' find request-id from response-id");
                                }
                            },
                            Err(error) => {
                                error!(error);
                                return;
                            }
                        }
                    }
                    _ => {
                        debug!("Nothing to do event")
                    }
                }
            }
            Err(_e) => {
                // TODO: add all possible events to SupportedBlockchainEvent to avoid reaching this block
                warn!("Caught event that couldn't be decoded as a supported event using log data");
            }
        }
    }

    #[instrument(skip(self, db_connection), fields(event=%authorization_response))]
    async fn handle_authorization_response(
        &self,
        authorization_response: OracleAuthorizationResponse,
        mut db_connection: PgConnection,
    ) {
        debug!(
            "Received authorization response {:?}",
            authorization_response
        );
        // Should emit PublicDecryptionRequest instead of doing the transaction directly

        let response = match fetch_host_event(&mut db_connection, authorization_response.request_id)
        {
            Ok(value) => value,
            Err(error) => {
                error!(
                    "Error fetching host event from DB for request-id: {:?} {:?}",
                    authorization_response.request_id, error
                );
                return;
            }
        };
        debug!("DB Fetch Response: {:?}", response);

        // NOTE: Only public decryption requests require an authorization request
        match SupportedBlockchainEvent::decode_log_data(response.event_log.data()) {
            Ok(event) => match event {
                SupportedBlockchainEvent::DecryptionRequest(value) => {
                    // TODO: DEBUG
                    let mut ct_handles: Vec<[u8; 32]> = Vec::new();
                    for ct_handle in value.cts {
                        ct_handles.push(ct_handle.into());
                    }
                    let handles: Vec<FixedBytes<32>> = ct_handles
                        .iter()
                        .map(|bytes| FixedBytes::from(*bytes))
                        .collect();

                    let calldata = match ComputeCalldata::public_decryption_req(handles.clone()) {
                        Ok(value) => value,
                        Err(error) => {
                            let err_msg = format!("Error computing calldata: {:?}", error);
                            error!(err_msg);
                            return;
                        }
                    };

                    let message = ZwsRelayerEvent::TransactionRequest(TransactionRequest {
                        request_id: authorization_response.request_id(),
                        address: self.decryption_manager_address,
                        chain_id: self.gateway_chain_id,
                        calldata,
                        timestamp: current_timestamp(),
                    });

                    let gateway_request_insertion_result = create_gateway_request(
                        &mut db_connection,
                        GatewayRequestRow {
                            request_id: authorization_response.request_id,
                            on_chain_request_id: None,
                            op: GatewayOperation::PublicDecryption,
                            status: GatewayOperationStatus::TXRequested,
                        },
                    );

                    debug!(
                        "GATEWAY REQUEST INSERTION RESULT: {:?}",
                        gateway_request_insertion_result
                    );

                    match wait_for_ct_pub_dec_availability(
                        handles,
                        self.decryption_manager_address,
                        self.gateway_http_url.clone(),
                    )
                    .await
                    {
                        Ok(()) => {}
                        Err(error) => {
                            error!(error);
                            return;
                        }
                    };

                    let _ = send_message_to_sqs_queue_empty(
                        true,
                        &self.sqs_client,
                        &self.tx_manager_queue_url,
                        &message,
                    )
                    .await;
                }
                _ => {
                    debug!("Not supported")
                }
            },
            _ => {
                debug!("Not supported")
            }
        }
    }

    // TODO: store request-id, operation
    #[instrument(skip(self, db_connection), fields(event=%input_registration_request))]
    async fn handle_input_registration_request(
        &self,
        input_registration_request: HTTPInputRegistrationRequest,
        mut db_connection: PgConnection,
    ) {
        debug!("Received request {}", input_registration_request);

        // TODO: add decryption manager address to handler as configuration

        let calldata = match ComputeCalldata::verify_proof_req(
            input_registration_request.contract_chain_id,
            input_registration_request.contract_address,
            input_registration_request.user_address,
            input_registration_request.ciphetext_with_zk_proof.clone(),
        ) {
            Ok(value) => value,
            Err(error) => {
                error!(
                    "Couldn't compute calldata for request: {:?} with error: {:?}",
                    input_registration_request, error
                );
                return;
            }
        };

        // TODO: host chain ids should be a list
        // gateway chain id should be a single value
        // make sure that the `Log` contains the chain-id
        let message = ZwsRelayerEvent::TransactionRequest(TransactionRequest {
            request_id: input_registration_request.request_id(),
            address: self.zkpok_manager_address,
            chain_id: self.gateway_chain_id,
            calldata,
            timestamp: current_timestamp(),
        });

        let gateway_request_insertion_result = create_gateway_request(
            &mut db_connection,
            GatewayRequestRow {
                request_id: input_registration_request.request_id,
                on_chain_request_id: None,
                op: GatewayOperation::InputRegistration,
                status: GatewayOperationStatus::TXRequested,
            },
        );
        debug!(
            "GATEWAY REQUEST INSERTION RESULT: {:?}",
            gateway_request_insertion_result
        );

        let _ = send_message_to_sqs_queue_empty(
            true,
            &self.sqs_client,
            &self.tx_manager_queue_url,
            &message,
        )
        .await;
    }

    // TODO: store request-id, operation
    #[instrument(skip(self, db_connection), fields(event=%private_decryption_request))]
    async fn handle_private_decryption_request(
        &self,
        private_decryption_request: PrivateDecryptionRequest,
        mut db_connection: PgConnection,
    ) {
        debug!("Received request {}", private_decryption_request);

        // TODO: add decryption manager address to handler as configuration
        let ct_pairs: Vec<CtHandleContractPair> = private_decryption_request
            .ct_handle_contract_pairs
            .iter()
            .map(|elt| CtHandleContractPair {
                ctHandle: elt.ct_handle.into(),
                contractAddress: elt.contract_address,
            })
            .collect();

        match wait_for_ct_priv_dec_availability(
            ct_pairs,
            self.decryption_manager_address,
            self.gateway_http_url.clone(),
            private_decryption_request.user_address,
        )
        .await
        {
            Ok(()) => {}
            Err(error) => {
                error!(error);
                return;
            }
        };

        let calldata =
            match ComputeCalldata::user_decryption_req(private_decryption_request.clone().into()) {
                Ok(value) => value,
                Err(error) => {
                    error!(
                        "Couldn't compute calldata for request: {:?} with error: {:?}",
                        private_decryption_request, error
                    );
                    return;
                }
            };

        // TODO: host chain ids should be a list
        // gateway chain id should be a single value
        // make sure that the `Log` contains the chain-id
        let message = ZwsRelayerEvent::TransactionRequest(TransactionRequest {
            request_id: private_decryption_request.request_id(),
            address: self.decryption_manager_address,
            chain_id: self.gateway_chain_id,
            calldata,

            timestamp: current_timestamp(),
        });

        let gateway_request_insertion_result = create_gateway_request(
            &mut db_connection,
            GatewayRequestRow {
                request_id: private_decryption_request.request_id,
                on_chain_request_id: None,
                op: GatewayOperation::PrivateDecryption,
                status: GatewayOperationStatus::TXRequested,
            },
        );

        debug!(
            "GATEWAY REQUEST INSERTION RESULT: {:?}",
            gateway_request_insertion_result
        );

        let _ = send_message_to_sqs_queue_empty(
            true,
            &self.sqs_client,
            &self.tx_manager_queue_url,
            &message,
        )
        .await;
    }
}

#[async_trait]
impl EventHandler<ZwsRelayerEvent> for ZWSRelayerHandler {
    // TODO: Make sure that we do log something about the event in the trace
    #[instrument(skip(self), fields(event=%event))]
    async fn handle_event(&self, event: ZwsRelayerEvent) {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let postgres_connection = match PgConnection::establish(&database_url) {
            Ok(value) => value,
            Err(error) => {
                error!("Postgres connexion failed: {error}");
                return;
            }
        };

        match event {
            // Any blockchain event
            ZwsRelayerEvent::BlockchainEvent(host_event) => {
                self.handle_blockchain_event(host_event, postgres_connection)
                    .await;
            }
            // Authorization response for on-host-chain request
            ZwsRelayerEvent::OracleAuthorizationResponse(authorization_response) => {
                self.handle_authorization_response(authorization_response, postgres_connection)
                    .await;
            }
            // Input registration request out of HTTP endpoint
            ZwsRelayerEvent::HTTPInputRegistrationRequest(input_registration_request) => {
                info!("{}", input_registration_request);
                self.handle_input_registration_request(
                    input_registration_request,
                    postgres_connection,
                )
                .await;
            }
            // Transaction response
            ZwsRelayerEvent::TransactionResponse(transaction_response) => {
                self.handle_transaction_response(*transaction_response, postgres_connection)
                    .await;
            }
            ZwsRelayerEvent::HTTPPrivateDecryptionRequest(decryption_request) => {
                self.handle_private_decryption_request(decryption_request, postgres_connection)
                    .await;
            }
            _ => {
                debug!(
                    "Not handled event {:?} {:?}",
                    event.event_name(),
                    event.request_id(),
                );
            }
        }
    }
}
