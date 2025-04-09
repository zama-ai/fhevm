use alloy::primitives::{keccak256, Address, LogData, Uint, U256};
use alloy::rpc::types::TransactionReceipt;
use alloy_sol_types::SolEvent;
use async_trait::async_trait;
use diesel::{Connection, PgConnection};
use fhevm_relayer::orchestrator::traits::EventDispatcher;
use fhevm_relayer::orchestrator::{Orchestrator, TokioEventDispatcher};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::str::FromStr;
use std::sync::Arc;

use fhevm_relayer::{
    blockchain::ethereum::{
        bindings::{
            DecryptionOracle::DecryptionRequest,
            DecyptionManager::{
                PublicDecryptionRequest, PublicDecryptionResponse, UserDecryptionRequest,
                UserDecryptionResponse,
            },
            ZKPoKManager::{VerifyProofRequest, VerifyProofResponse},
        },
        ComputeCalldata,
    },
    orchestrator::traits::{Event, EventHandler},
    transaction::{sender::RetryConfig, TransactionService, TxConfig},
};
use tracing::{debug, error, info, instrument, warn};

use crate::events::*;

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
    // TODO: implement a `from_env`?

    pub async fn default() -> Self {
        let gateway_chain_id: u64 = 54321;
        let private_httpz_key_env = "";
        let gateway_rpc_url = "";
        let gateway_tx_service =
            match TransactionService::new(gateway_rpc_url, private_httpz_key_env, gateway_chain_id)
                .await
            {
                Ok(value) => value,
                Err(error) => {
                    let err_msg = format!(
                        "Couldn't initialize gateway transaction service: {:?}",
                        error
                    );
                    error!(err_msg);
                    panic!("{}", err_msg);
                }
            };
        let mut tx_services = HashMap::new();
        tx_services.insert(gateway_chain_id, gateway_tx_service);
        Self::new(
            String::from("http://sqs.eu-central-1.localhost.localstack.cloud:4566/000000000000/relayer-queue"),
            tx_services,
        )
        .await
    }
}

#[async_trait]
impl EventHandler<ZwsRelayerEvent> for ZWSTransactionManagerMockHandler {
    // TODO: Make sure that we do log something about the event in the trace
    #[instrument(skip(self, event))]
    async fn handle_event(&self, event: ZwsRelayerEvent) {
        // Match log
        match event {
            ZwsRelayerEvent::SQSRelayerTransactionRequest(transaction_request) => {
                debug!("TX Manager received: {:?}", transaction_request);
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
                    "Making transaction to: {:?}, {:?}, {:?}, {:?}",
                    transaction_request.chain_id,
                    transaction_request.address,
                    transaction_service,
                    transaction_request.calldata,
                );

                let tx_config = TxConfig {
                    gas_limit: Some(150000000),
                    max_priority_fee: Some(2000000000),
                    value: None,
                    nonce: None,
                    confirmations: Some(1),
                    timeout_secs: Some(60),
                    retry_config: Some(RetryConfig::default()),
                };

                let tx_request_response_result = transaction_service
                    .submit_and_wait(
                        transaction_request.address,
                        transaction_request.calldata.clone(),
                        tx_config,
                    )
                    .await;
                // debug!("tx-request-response = {:?}", tx_request_response_result);

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
                let message = ZwsRelayerEvent::SQSRelayerTransactionResponse(Box::new(
                    SQSRelayerTransactionResponse {
                        request_id: transaction_request.request_id(),
                        receipt: tx_receipt,
                    },
                ));
                match send_message_to_sqs_queue(true, &self.sqs_client, &self.queue_url, message)
                    .await
                {
                    Ok(_) => {
                        debug!("Successfuly sent authorization response")
                    }
                    Err(error) => {
                        error!(
                            "Error sending SQSRelayerTransactionResponse to {:?}: {:?}",
                            self.queue_url, error
                        )
                    }
                }
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

// NOTE: add debug handler that allows any PaymentAuthorizationRequest to mock the Console behavior
// this could be activated with an env var flag

#[async_trait]
impl EventHandler<ZwsRelayerEvent> for ZWSConsoleMockHandler {
    // TODO: Make sure that we do log something about the event in the trace
    #[instrument(skip(self, event))]
    async fn handle_event(&self, event: ZwsRelayerEvent) {
        // Match log
        match event {
            ZwsRelayerEvent::SQSRelayerAuthorizationRequest(authorization_request) => {
                info!("Received authorization request from SQS pushing auth response to SQS relayer queue.");
                // NOTE: this is just a mock of the Console so we authorized all requests
                let message = ZwsRelayerEvent::SQSRelayerAuthorizationResponse(
                    SQSRelayerAuthorizationResponse {
                        request_id: authorization_request.request_id(),
                        authorized: true,
                    },
                );

                // SQS
                match send_message_to_sqs_queue(true, &self.sqs_client, &self.queue_url, message)
                    .await
                {
                    Ok(_) => {
                        debug!("Successfuly sent authorization response")
                    }
                    Err(error) => {
                        error!(
                            "Error sending SQSRelayerAuthorizationResponse to {:?}: {:?}",
                            self.queue_url, error
                        )
                    }
                }
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
    // TODO: Add gateway chain-id
}

impl ZWSRelayerHandler {
    pub async fn new(
        console_queue_url: String,
        tx_manager_queue_url: String,
        orchestrator: Arc<Orchestrator<TokioEventDispatcher<ZwsRelayerEvent>, ZwsRelayerEvent>>,
    ) -> Self {
        let config = aws_config::from_env().load().await;
        debug!("{:?}", config);
        let sqs_client = aws_sdk_sqs::Client::new(&config);

        ZWSRelayerHandler {
            sqs_client,
            console_queue_url,
            tx_manager_queue_url,
            orchestrator,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SupportedBlockchainEvent {
    DecryptionRequest(DecryptionRequest),           // Oracle event
    UserDecryptionResponse(UserDecryptionResponse), // Decryption Manager event
    PublicDecryptionResponse(PublicDecryptionResponse), // Decryption Manager event
    VerifyProofResponse(VerifyProofResponse),       // Zkpok Manager event
    // Unused
    VerifyProofRequest(VerifyProofRequest),
    UserDecryptionRequest(UserDecryptionRequest),
    PublicDecryptionRequest(PublicDecryptionRequest),
}

impl SupportedBlockchainEvent {
    pub fn decode_log_data(
        log: &LogData,
        validate: bool,
    ) -> Result<SupportedBlockchainEvent, std::string::String> {
        if let Ok(event) = DecryptionRequest::decode_log_data(log, validate) {
            return Ok(SupportedBlockchainEvent::DecryptionRequest(event));
        };
        if let Ok(event) = UserDecryptionResponse::decode_log_data(log, validate) {
            return Ok(SupportedBlockchainEvent::UserDecryptionResponse(event));
        };
        if let Ok(event) = PublicDecryptionResponse::decode_log_data(log, validate) {
            return Ok(SupportedBlockchainEvent::PublicDecryptionResponse(event));
        };
        if let Ok(event) = VerifyProofResponse::decode_log_data(log, validate) {
            return Ok(SupportedBlockchainEvent::VerifyProofResponse(event));
        };
        if let Ok(event) = VerifyProofRequest::decode_log_data(log, validate) {
            return Ok(SupportedBlockchainEvent::VerifyProofRequest(event));
        };
        if let Ok(event) = UserDecryptionRequest::decode_log_data(log, validate) {
            return Ok(SupportedBlockchainEvent::UserDecryptionRequest(event));
        };
        if let Ok(event) = PublicDecryptionRequest::decode_log_data(log, validate) {
            return Ok(SupportedBlockchainEvent::PublicDecryptionRequest(event));
        };
        Err("Failed to decode log data into any supported event type".to_string())
    }
}

// NOTE: add debug handler that allows any PaymentAuthorizationRequest to mock the Console behavior
// this could be activated with an env var flag
//
fn extract_zkpok_id_from_receipt(receipt: &TransactionReceipt) -> Result<U256, String> {
    // Event signature without indexed parameters
    let target_topic = keccak256("VerifyProofRequest(uint256,uint256,address,address,bytes)");
    for log in receipt.inner.logs().iter() {
        if let Some(first_topic) = log.topics().first() {
            if first_topic == &target_topic {
                return match VerifyProofRequest::decode_log_data(
                    log.data(),
                    false, // No indexed parameters in this event
                ) {
                    Ok(event) => {
                        debug!(
                            ?receipt.transaction_hash,
                            proof_id = ?event.zkProofId,
                            chain_id = ?event.contractChainId,
                            contract = ?event.contractAddress,
                            user = ?event.userAddress,
                            proof_size = event.ciphertextWithZKProof.len(),
                            "Decoded VerifyProofRequest event"
                        );
                        Ok(event.zkProofId)
                    }
                    Err(e) => {
                        error!(
                            ?receipt.transaction_hash,
                            error = ?e,
                            "Failed to decode VerifyProofRequest event"
                        );
                        Err("ERROR".to_string())
                    }
                };
            }
        }
    }

    Err("ERROR".to_string())
}

// fn convert_log(log: &alloy::primitives::Log) -> alloy::rpc::types::Log {
//     log.into()
//     // alloy::rpc::types::Log {
//     // inner: Log<T>,
//     // block_hash: Option<FixedBytes<32>>,
//     // block_number: Option<u64>,
//     // block_timestamp: Option<u64>,
//     // transaction_hash: Option<FixedBytes<32>>,
//     // }
// }

impl ZWSRelayerHandler {
    async fn handle_transaction_response(
        &self,
        response: SQSRelayerTransactionResponse,
        mut db_connection: PgConnection,
    ) {
        // TODO: Support other op-types
        // TODO: Emit transaction-receipt event to SQS

        let gateway_request = match fetch_gateway_request(&mut db_connection, response.request_id) {
            Ok(value) => value,
            Err(error) => {
                debug!("Value not in DB: {:?}", error);
                return;
            }
        };
        debug!("{:?}", gateway_request);

        // Method 1
        let parsed = response
            .receipt
            .inner
            .logs()
            .iter()
            .map(|log| SupportedBlockchainEvent::decode_log_data(log.data(), true))
            .next();
        // .collect::<Vec<Result<SupportedBlockchainEvent, _>>>();

        let _ = match parsed {
            Some(Ok(SupportedBlockchainEvent::VerifyProofRequest(sub_value))) => sub_value,
            _ => {
                error!("ERROR");
                return;
            }
        };

        // Method 2
        let zkpokid = match extract_zkpok_id_from_receipt(&response.receipt) {
            Ok(value) => value,

            Err(error) => {
                error!("{:?}", error);
                return;
            }
        };
        debug!("zkpokid: {:?}", zkpokid);

        // Store on-chain-request-id + update operation status in db
        match update_gateway_request_onchain_id(
            &mut db_connection,
            response.request_id,
            GatewayOperationStatus::TXFulfilled,
            Some(zkpokid.to_be_bytes_vec()),
        ) {
            Ok(value) => {
                info!("insertion successful: {:?}", value);
            }
            Err(error) => {
                error!("insertion error: {:?}", error);
            }
        };

        let check_response = fetch_gateway_response(
            &mut db_connection,
            zkpokid.to_be_bytes_vec(),
            GatewayOperation::InputRegistration,
        );
        debug!("{:?}", check_response);
        // TODO: handle callback if response already there
        //
        match check_response {
            Ok(db_response) => match db_response.first() {
                Some(elt) => {
                    let new_event = ZwsRelayerEvent::BlockchainEvent(BlockchainEvent {
                        request_id: response.request_id,
                        event_log: elt.event_log.0.clone(),
                    });

                    // Re-emit internally the event
                    let dispatch = self.orchestrator.dispatch_event(new_event).await;
                    debug!("Dispatch response: {:?}", dispatch);
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

    async fn handle_blockchain_event(
        &self,
        event: BlockchainEvent,
        mut db_connection: PgConnection,
    ) {
        // NOTE: store event-log for further use
        // - When asking for the operation to be handled on the L2
        // - When responding with callback

        match SupportedBlockchainEvent::decode_log_data(event.event_log.data(), true) {
            Ok(decoded_event) => {
                match decoded_event {
                    SupportedBlockchainEvent::DecryptionRequest(decryption_request) => {
                        match create_host_event(&mut db_connection, event.clone()) {
                            Ok(_) => {
                                debug!("Host event pushed to postgres");
                            }
                            Err(error) => {
                                error!("Failed to push event to postgres: {error}");
                                return;
                            }
                        }

                        // Match and decode host decryption request event
                        // pub counter: Uint<256, 4>,
                        // pub requestID: Uint<256, 4>,
                        // pub cts: Vec<Uint<256, 4>>,
                        // pub contractCaller: Address,
                        // pub callbackSelector: FixedBytes<4>,
                        info!("Decryption request: {:?}", decryption_request);
                        info!(
                                            "Decryption event log received from listener: block number: {:?}, ethereum_request_id: {:?}, selector {:?}",
                                            event.event_log.block_number, decryption_request.requestID, decryption_request.callbackSelector
                                        );
                        let mut ct_handles: Vec<[u8; 32]> = Vec::new();
                        for ct_handle in decryption_request.cts {
                            // TODO: Check if to_le_bytes will work.
                            ct_handles.push(ct_handle.to_le_bytes());
                        }
                        let contract_caller = decryption_request.contractCaller;

                        // Publish authorization request to SQS
                        let message = ZwsRelayerEvent::SQSRelayerAuthorizationRequest(
                            SQSRelayerAuthorizationRequest {
                                request_id: event.request_id(),
                                caller_address: contract_caller,
                            },
                        );
                        match send_message_to_sqs_queue(
                            true,
                            &self.sqs_client,
                            &self.console_queue_url,
                            message,
                        )
                        .await
                        {
                            Ok(_) => {
                                debug!("Successfuly sent authorization response")
                            }
                            Err(error) => {
                                error!(
                                    "Error sending SQSRelayerAuthorizationRequest: {:?} to {:?}",
                                    error, self.console_queue_url
                                )
                            }
                        }
                    }
                    SupportedBlockchainEvent::VerifyProofResponse(verification_response) => {
                        debug!("todo: implement: {:?}", verification_response);

                        // Check if on-chain request-id in db, if-not store response on-chain-request-id
                        // in db with how to retrieve it.
                        let zkpokid = verification_response.zkProofId.to_be_bytes_vec();
                        let source_request_id = match fetch_gateway_request_chain_id(
                            &mut db_connection,
                            zkpokid.clone(),
                        ) {
                            Ok(rows) => match rows.first() {
                                Some(value) => value.request_id,
                                None => {
                                    let insertion_result = create_gateway_response(
                                        &mut db_connection,
                                        NewGatewayResponseRow {
                                            on_chain_request_id: zkpokid,
                                            event_log: diesel_json::Json(event.event_log),
                                            op: GatewayOperation::InputRegistration,
                                        },
                                    );
                                    debug!("Couldn't find request in db, storing in db.");
                                    match insertion_result {
                                        Ok(_) => {
                                            debug!("insertion success");
                                        }
                                        Err(error) => {
                                            debug!("insertion failure: {:?}", error);
                                        }
                                    }
                                    return;
                                }
                            },
                            Err(error) => {
                                // TODO: store in db
                                let insertion_result = create_gateway_response(
                                    &mut db_connection,
                                    NewGatewayResponseRow {
                                        on_chain_request_id: zkpokid,
                                        event_log: diesel_json::Json(event.event_log),
                                        op: GatewayOperation::InputRegistration,
                                    },
                                );
                                debug!("failed to fetch from db {:?}", error);
                                match insertion_result {
                                    Ok(_) => {
                                        debug!("insertion success");
                                    }
                                    Err(error) => {
                                        debug!("insertion failure: {:?}", error);
                                    }
                                }
                                return;
                            }
                        };

                        let response = ZwsRelayerEvent::SQSRelayerInputRegistrationResponse(
                            SQSRelayerInputRegistrationResponse {
                                request_id: source_request_id,
                                handles: verification_response.handles,
                                signatures: verification_response.signatures,
                            },
                        );
                        match send_message_to_sqs_queue(
                            true,
                            &self.sqs_client,
                            &self.console_queue_url,
                            response.clone(),
                        )
                        .await
                        {
                            Ok(_) => {
                                debug!(
                                    "Successfuly sent input registration response to {:?}: {:?}",
                                    self.console_queue_url, response
                                )
                            }
                            Err(error) => {
                                error!(
                                    "Error sending SQSRelayerInputRegistrationResponse to {:?}: {:?}",
                                    self.console_queue_url, error
                                )
                            }
                        }
                    }
                    SupportedBlockchainEvent::UserDecryptionResponse(user_decryption_response) => {
                        debug!("todo: implement: {:?}", user_decryption_response)
                    }
                    SupportedBlockchainEvent::PublicDecryptionRequest(
                        public_decryption_response,
                    ) => {
                        debug!("todo: implement: {:?}", public_decryption_response)
                    }
                    _ => {
                        debug!("Nothing to do event")
                    }
                }
            }
            Err(_e) => {
                error!("Couldn't decode log data as a supported event");
            }
        }
    }

    async fn handle_authorization_response(
        &self,
        authorization_response: SQSRelayerAuthorizationResponse,
        mut db_connection: PgConnection,
    ) {
        debug!(
            "Received authorization response {:?}",
            authorization_response
        );

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
        let _ct_handles: Vec<Uint<256, 4>> =
            match DecryptionRequest::decode_log_data(response.event_log.data(), true) {
                Ok(eth_decryption_request) => {
                    let mut ct_handles: Vec<[u8; 32]> = Vec::new();
                    for ct_handle in eth_decryption_request.cts {
                        // TODO: Check if to_le_bytes will work.
                        ct_handles.push(ct_handle.to_le_bytes());
                    }
                    ct_handles
                }
                Err(_e) => {
                    error!("Couldn't decode log data as Decryption Request");
                    return;
                }
            }
            .iter()
            .map(|bytes| Uint::from_be_bytes(*bytes))
            .collect();

        // TODO: add decryption manager address to handler as configuration
        let _decryption_manager_address =
            match Address::from_str("0x2Fb4341027eb1d2aD8B5D9708187df8633cAFA92") {
                Ok(value) => value,
                Err(error) => {
                    let err_msg = format!("Error parsing DecryptionManager address: {:?}", error);
                    error!(err_msg);
                    return;
                }
            };

        // // TODO: implement (handler needs to hold chain-id)
        //
        // let calldata = match ComputeCalldata::user_decryption_req(ct_handles.clone()) {
        //     Ok(value) => value,
        //     Err(error) => {
        //         let err_msg = format!("Error compututing calldata: {:?}", error);
        //         error!(err_msg);
        //         return;
        //     }
        // };
        //
        // debug!("{:?} {:?}", calldata, decryption_manager_address);
        // // TODO: host chain ids should be a list
        // // gateway chain id should be a single value
        // // make sure that the `Log` contains the chain-id
        // let gateway_chain_id: u64 = 54321;
        // let message = ZwsRelayerEvent::SQSRelayerTransactionRequest(SQSRelayerTransactionRequest {
        //     request_id: authorization_response.request_id(),
        //     address: decryption_manager_address,
        //     chain_id: gateway_chain_id,
        //     calldata,
        // });
        //
        // match send_message_to_sns_topic(true, &self.sns_client, &self.topic_arn, message).await {
        //     Ok(_) => {
        //         debug!("Successfuly sent transaction request")
        //     }
        //     Err(error) => {
        //         error!("Error sending transaction request: {:?}", error)
        //     }
        // }
    }

    // TODO: store request-id, operation
    async fn handle_input_registration_request(
        &self,
        input_registration_request: SQSRelayerInputRegistrationRequest,
        mut db_connection: PgConnection,
    ) {
        debug!(
            "Received input registration request {:?}",
            input_registration_request
        );

        // TODO: add decryption manager address to handler as configuration
        let zkpok_manager_address =
            match Address::from_str("0x812b06e1CDCE800494b79fFE4f925A504a9A9810") {
                Ok(value) => value,
                Err(error) => {
                    let err_msg = format!("Error parsing ZkPokManager address: {:?}", error);
                    error!(err_msg);
                    return;
                }
            };

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

        debug!(
            "Calldata: {:?} ZkPoK manager address: {:?}, Calldata length: {:?}",
            calldata,
            zkpok_manager_address,
            calldata.len()
        );
        // TODO: host chain ids should be a list
        // gateway chain id should be a single value
        // make sure that the `Log` contains the chain-id
        let gateway_chain_id: u64 = 54321;
        let message = ZwsRelayerEvent::SQSRelayerTransactionRequest(SQSRelayerTransactionRequest {
            request_id: input_registration_request.request_id(),
            address: zkpok_manager_address,
            chain_id: gateway_chain_id,
            calldata,
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

        match send_message_to_sqs_queue(true, &self.sqs_client, &self.tx_manager_queue_url, message)
            .await
        {
            Ok(_) => {
                debug!("Successfuly sent transaction request")
            }
            Err(error) => {
                error!(
                    "Error sending SQSRelayerTransactionRequest to {:?}: {:?}",
                    self.tx_manager_queue_url, error,
                )
            }
        }
    }
}

#[async_trait]
impl EventHandler<ZwsRelayerEvent> for ZWSRelayerHandler {
    // TODO: Make sure that we do log something about the event in the trace
    #[instrument(skip(self, event))]
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
            // TODO: We make the assumption that all events come from an EVM chain
            // we should split host and gateway events, and probably have the events be parsed in
            // the listeners instead of the handler.
            // Such that each listener implements chain-specific logic to parse the input
            ZwsRelayerEvent::BlockchainEvent(host_event) => {
                self.handle_blockchain_event(host_event, postgres_connection)
                    .await;
            }
            // Authorization response for on-host-chain request
            ZwsRelayerEvent::SQSRelayerAuthorizationResponse(authorization_response) => {
                self.handle_authorization_response(authorization_response, postgres_connection)
                    .await;
            }
            // Input registration request out of HTTP endpoint
            ZwsRelayerEvent::SQSRelayerInputRegistrationRequest(input_registration_request) => {
                info!("{:?}", input_registration_request);
                self.handle_input_registration_request(
                    input_registration_request,
                    postgres_connection,
                )
                .await;
                // TODO: implement
                // - No authorization check because query would go through the back so already
                // approved
                // - Query tx manager
                // - Wait for response back
                // - Send response back
            }
            // Transaction response
            ZwsRelayerEvent::SQSRelayerTransactionResponse(transaction_response) => {
                // info!("{:?}", transaction_response);
                self.handle_transaction_response(*transaction_response, postgres_connection)
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
