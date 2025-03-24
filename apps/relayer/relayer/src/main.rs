use alloy::primitives::Address;
use dotenvy::from_path;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use fhevm_relayer::{
    blockchain::ethereum::{ChainName, ContractAndTopicsFilter, EthereumJsonRPCWsClient},
    orchestrator::{
        traits::{EventHandler, HandlerRegistry},
        Orchestrator, TokioEventDispatcher,
    },
    transaction::TransactionService,
};
use std::env;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

use zws_relayer_lib::events::*;
use zws_relayer_lib::handlers::*;
use zws_relayer_lib::listeners::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractConfig {
    name: String,
    address: Address,
}

const DECRYPTION_ORACLE_ADDRESS_ENV_KEY: &str = "DECRYPTION_ORACLE_ADDRESS";
const DECRYPTION_MANAGER_ADDRESS_ENV_KEY: &str = "DECRYPTION_MANAGER_ADDRESS";
const ZKPOK_MANAGER_ADDRESS_ENV_KEY: &str = "ZKPOK_MANAGER_ADDRESS";

pub struct ChainConfig {
    /// Chain id
    pub chain_id: u64,
    /// RPC URL
    pub rpc_url: String,
    /// Env var name that holds the private key
    pub private_key_env: String,
}

// NOTE: we should probably catch each request in a redis-db for easier debugging
// NOTE: we should also keep a request-id to properly track the flow
// TODO: Define spec for SNS/SQS messages
// TODO: Python/Rust implementation of orchestrator mock?
// TODO: add proper tracing

/// Main public-decryption relayer service
///
/// 3 Listeners
/// - HTTPZ chain listener
/// - Gateway chain listerner
/// - SQS listener
///
/// Input adder:
/// - Event is caught by the SQS listener
/// - No check to do since authorization already done
/// - Emit event to TX manager
/// - tx fulfilled caught by SQS listener
/// - We push the response to orchestrator SNS topic
/// - Caught by orchestrator
///
/// Public Decryption Flow:
/// - Event is caught by l1-event-listener
/// - A check for said contract caller is emitted on SNS
/// - A response to that call is caught by the SQS listener
///     - If success we proceed, else we don't do anything
///     - An event could probably be propagated through SQS in case a new contract is whitelisted
///     in this case we would un-block all matching transactions (i.e. re-ask for approval for
///     them)
/// - A tx is emitted to the tx manager
/// - tx fulfilled emitted to SNS
/// - tx fulfilled caught by SQS listener
/// - Response event is caught by the l2-event-listener
///     - We filter if the kms-request-id matches one that we are expecting
///     if so we continue else we return
///     - Should we implement a check at this point for registration too?
/// - Callback tx emitted to the tx manager
/// - wait for fulfilled or error
///
/// Private Decryption Flow:
/// - Event is caught by the SQS listener
/// - No check to do since authorization already done
/// - Emit event to TX manager
/// - tx fulfilled caught by SQS listener
/// - Response event is caught by the l2-event-listener
///     - We filter if the kms-request-id matches one that we are expecting
///     if so we continue else we return
/// - We push the response to tx-manager SNS topic
/// - wait for fulfilled or error
///
#[tokio::main]
async fn main() {
    // ############################################################################################
    // Configuration
    // ############################################################################################

    match from_path(Path::new(".env")).ok() {
        Some(_) => {
            println!("Properly loaded .env file");
        }
        None => {
            println!("Didn't load .env");
        }
    }

    // TODO: create proper struct to handle configuration

    let gateway_ws_url =
        env::var("GATEWAY_WEBSOCKET").unwrap_or(String::from("ws://localhost:8546"));
    let host_ws_url = env::var("HOST_WEBSOCKET").unwrap_or(String::from("ws://localhost:8545"));
    let decryption_oracle_address = Address::from_str(
        &env::var(DECRYPTION_ORACLE_ADDRESS_ENV_KEY)
            .expect("Couldn't find DECRYPTION_ORACLE_ADDRESS from "),
    )
    .expect("Invalid Ethereum address");
    let decryption_manager_address = Address::from_str(
        &env::var(DECRYPTION_MANAGER_ADDRESS_ENV_KEY)
            .expect("Couldn't find DECRYPTION_ORACLE_ADDRESS from "),
    )
    .expect("Invalid Ethereum address");
    let zkpok_manager_address = Address::from_str(
        &env::var(ZKPOK_MANAGER_ADDRESS_ENV_KEY)
            .expect("Couldn't find DECRYPTION_ORACLE_ADDRESS from "),
    )
    .expect("Invalid Ethereum address");

    // NOTE: Should probably come from .env
    let node_id = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab]; // Used to generate uuid

    // TODO: Pass the event_dispatcher to the event_listener
    let config = aws_config::from_env().load().await;
    let sqs_client = aws_sdk_sqs::Client::new(&config);
    let default_relayer_sqs_endpoint = String::from(
        "http://sqs.eu-central-1.localhost.localstack.cloud:4566/000000000000/relayer-queue",
    );
    let default_console_sqs_endpoint = String::from(
        "http://sqs.eu-central-1.localhost.localstack.cloud:4566/000000000000/orchestrator-queue",
    );
    let default_tx_manager_sqs_endpoint = String::from(
        "http://sqs.eu-central-1.localhost.localstack.cloud:4566/000000000000/tx-manager-queue",
    );
    // ############################################################################################
    // Observability
    // ############################################################################################
    // https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#example-syntax
    let filter = tracing_subscriber::EnvFilter::from_default_env();
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    info!(
        "Decryption Oracle Contract address {:?}",
        decryption_oracle_address
    );

    // ############################################################################################
    // Orchestrator
    // ############################################################################################

    let dispatcher = Arc::new(TokioEventDispatcher::<ZwsRelayerEvent>::new());
    let orchestrator = Orchestrator::new(Arc::clone(&dispatcher), &node_id);

    let gateway_chain_id: u64 = 54321;
    let gateway_private_key_env = "GATEWAY_PRIVATE_KEY".to_string();

    let gateway_rpc_url = "http://localhost:8546";
    let gateway_tx_service =
        match TransactionService::new(gateway_rpc_url, &gateway_private_key_env, gateway_chain_id)
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

    let host_chain_private_key_env = "HTTPZ_PRIVATE_KEY".to_string();

    let host_chains: Vec<ChainConfig> = vec![ChainConfig {
        chain_id: 12345,
        rpc_url: "http://localhost:8545".to_string(),
        private_key_env: host_chain_private_key_env,
    }];

    for host_chain in host_chains {
        let tx_service = match TransactionService::new(
            &host_chain.rpc_url,
            &host_chain.rpc_url,
            host_chain.chain_id,
        )
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
        tx_services.insert(host_chain.chain_id, tx_service);
    }

    // Register the event handlers
    // NOTE: we could also set the dispatcher in the Handler, but we use a SNS topic instead
    // here

    // TODO: add tx-manager queue, and properly setup all listeners
    // TODO: http listener should send message to SQS instead of internal orchestrator

    let zws_handler: Arc<dyn EventHandler<ZwsRelayerEvent>> = Arc::new(
        ZWSRelayerHandler::new(
            default_console_sqs_endpoint.to_owned(),
            default_tx_manager_sqs_endpoint.to_owned(),
            Arc::clone(&orchestrator),
        )
        .await,
    );

    let console_handler: Arc<dyn EventHandler<ZwsRelayerEvent>> =
        Arc::new(ZWSConsoleMockHandler::new(default_relayer_sqs_endpoint.to_owned()).await);

    let tx_manager_handler: Arc<dyn EventHandler<ZwsRelayerEvent>> = Arc::new(
        ZWSTransactionManagerMockHandler::new(default_relayer_sqs_endpoint.to_owned(), tx_services)
            .await,
    );

    // Register handler for all events
    // Public decryption request
    orchestrator.register_handler(BlockchainEvent::event_id(), Arc::clone(&zws_handler));
    // Console authorization response
    orchestrator.register_handler(
        SQSRelayerAuthorizationResponse::event_id(),
        Arc::clone(&zws_handler),
    );
    // HTTPZ-Gateway response
    orchestrator.register_handler(HTTPZGatewayEvent::event_id(), Arc::clone(&zws_handler));
    // Transaction response
    orchestrator.register_handler(
        SQSRelayerTransactionResponse::event_id(),
        Arc::clone(&zws_handler),
    );
    orchestrator.register_handler(
        SQSRelayerInputRegistrationRequest::event_id(),
        Arc::clone(&zws_handler),
    );

    // NOTE: used for debugging mostly

    // Transaction response
    orchestrator.register_handler(
        SQSRelayerAuthorizationRequest::event_id(),
        Arc::clone(&console_handler),
    );
    orchestrator.register_handler(
        SQSRelayerTransactionRequest::event_id(),
        Arc::clone(&tx_manager_handler),
    );
    // TODO: Implement missing:
    // SQS private decryption request
    // SQS private decryption response
    // INPUT ??? -> check input flow

    // Initialize Ethereum host L1 adapter
    let host_l1 = EthereumJsonRPCWsClient::new(ChainName::Httpz, host_ws_url.as_str())
        .await
        .expect("Couldn't connect to websocket of Host L1 blockchain ");
    let host_l1 = Arc::new(host_l1);
    // Initialize Gateway L2 adapter
    let rollup_l2 = EthereumJsonRPCWsClient::new(ChainName::Gateway, gateway_ws_url.as_str())
        .await
        .expect("Couldn't connect to websocket of Gateway L2 blockchain ");
    let rollup_l2 = Arc::new(rollup_l2);

    let relayer_sqs_endpoint: &'static str = Box::leak(
        env::var("RELAYER_SQS_ENDPOINT")
            .unwrap_or(default_relayer_sqs_endpoint)
            .into_boxed_str(),
    );
    let console_sqs_endpoint: &'static str = Box::leak(
        env::var("CONSOLE_SQS_ENDPOINT")
            .unwrap_or(default_console_sqs_endpoint)
            .into_boxed_str(),
    );
    let tx_manager_sqs_endpoint: &'static str = Box::leak(
        env::var("TX_MANAGER_SQS_ENDPOINT")
            .unwrap_or(default_tx_manager_sqs_endpoint)
            .into_boxed_str(),
    );

    let filter_httpz_host = ContractAndTopicsFilter::new(vec![decryption_oracle_address], vec![]);
    let subscription_httpz_host = host_l1
        .new_subscription(filter_httpz_host, None)
        .await
        .expect("Subscription to L1 failed");

    let filter_httpz_gateway = ContractAndTopicsFilter::new(
        vec![decryption_manager_address, zkpok_manager_address],
        vec![],
    );
    let subscription_httpz_gateway = rollup_l2
        .new_subscription(filter_httpz_gateway, None)
        .await
        .expect("Subscription to Gateway failed");

    // TODO: Add http listeners for un-metered relayer

    // HTTP listener (queries should come from the backend via SQS but to be able to run as
    // standalone software we need this)
    tokio::spawn(http_listener(
        sqs_client.clone(),
        relayer_sqs_endpoint,
        Arc::clone(&orchestrator),
    ));

    // Relayer SQS event listener
    tokio::spawn(sqs_listener(
        sqs_client.clone(),
        relayer_sqs_endpoint,
        Arc::clone(&orchestrator),
    ));

    // TX-Manager SQS event listener
    tokio::spawn(sqs_listener(
        sqs_client.clone(),
        tx_manager_sqs_endpoint,
        Arc::clone(&orchestrator),
    ));

    // Blockchain event listener
    tokio::spawn(blockchain_event_listener(
        subscription_httpz_gateway,
        Arc::clone(&orchestrator),
        "Gateway".to_owned(),
    ));

    // Native chain event listener
    tokio::spawn(blockchain_event_listener(
        subscription_httpz_host,
        Arc::clone(&orchestrator),
        "Host".to_owned(),
    ));

    // Wait for ctrl + c signal to stop the application
    tokio::signal::ctrl_c().await.expect("Crtl-C Error");
    info!("Received ctrl + c signal, stopping...");
}
