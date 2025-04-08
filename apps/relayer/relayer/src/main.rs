use alloy::primitives::Address;
use alloy::signers::Signer;
use clap::Parser;
use config::{Config, Environment, File}; // ConfigError
use dotenvy::from_path;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use fhevm_relayer::{
    blockchain::ethereum::ChainName,
    config::settings::KeyUrl,
    orchestrator::{
        traits::{EventHandler, HandlerRegistry},
        Orchestrator, TokioEventDispatcher,
    },
    transaction::TransactionService,
};
use std::path::Path;
use std::sync::Arc;
use tracing::{debug, error, info, warn};
use tracing_subscriber::FmtSubscriber;

use zws_relayer_lib::events::*;
use zws_relayer_lib::handlers::*;
use zws_relayer_lib::listeners::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContractConfig {
    name: String,
    address: Address,
}

// TODO: use rust-url to validate url inputs?

// TODO: rethink the private-key-env to fit the new signer paradigm
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChainConfig {
    /// Chain id
    pub chain_id: u64,
    /// WebSocket endpoint URL
    pub ws_url: String,
    /// HTTP endpoint URL
    pub http_url: String,
    /// Signer configuration
    pub signer_config: SignerConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HostChainConfig {
    pub chain_config: ChainConfig,
    pub decryption_oracle: Address,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GatewayChainConfig {
    pub chain_config: ChainConfig,
    pub zkpok_manager: Address,
    pub decryption_manager: Address,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalSignerConfig {
    /// Env var name that holds the private key
    pub private_key_env: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AWSKMSSignerConfig {
    /// Env var name that holds the private key
    pub key_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SignerConfig {
    AWSKMS(AWSKMSSignerConfig),
    Local(LocalSignerConfig),
}

/// Top-level configuration structure.
///
/// Contains all configuration settings for the relayer service.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RelayerConfiguration {
    /// Network configurations
    pub host_chains: Vec<HostChainConfig>,
    pub gateway_chain: GatewayChainConfig,
    pub queues: SQSConfiguration,
    pub key_url: Option<KeyUrl>, // Optional because only require in debug mode
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SQSConfiguration {
    pub console_queue: String,
    pub relayer_queue: String,
    pub transaction_queue: String,
}

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[arg(long = "config-file")]
    pub config_file: Option<String>,
}

impl RelayerConfiguration {
    pub fn new(config_file: Option<String>) -> Result<Self, String> {
        // First get base config from files
        let s = match Config::builder()
            .add_source(
                File::with_name(&config_file.unwrap_or("config.toml".to_string())).required(false),
            )
            // Change how we specify environment variables
            // Env takes precedence over other sources
            .add_source(
                Environment::with_prefix("RELAYER")
                    .separator("__") // Use double underscore
                    .prefix_separator("_"), // Separator between RELAYER and the rest
            )
            .build()
        {
            Ok(value) => value,
            Err(error) => {
                error!("{:?}", error);
                return Err("".to_string());
            }
        };

        let settings: Self = match s.try_deserialize() {
            Ok(value) => value,
            Err(error) => {
                error!("{:?}", error);
                return Err("".to_string());
            }
        };

        Ok(settings)
    }

    pub fn get_host_signer(
        &self,
        _chain_id: u64,
    ) -> Result<Box<dyn Signer>, alloy::signers::local::LocalSignerError> {
        match "".parse::<alloy::signers::local::PrivateKeySigner>() {
            Ok(value) => Ok(Box::new(value)),
            Err(error) => Err(error),
        }
    }

    pub fn get_gateway_signer(
        &self,
    ) -> Result<Box<dyn Signer>, alloy::signers::local::LocalSignerError> {
        match "".parse::<alloy::signers::local::PrivateKeySigner>() {
            Ok(value) => Ok(Box::new(value)),
            Err(error) => Err(error),
        }
    }
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
    // Load .env if it exists, tracing not setup yet so println log
    match from_path(Path::new(".env")).ok() {
        Some(_) => {
            println!("Properly loaded .env file");
        }
        None => {
            println!("Didn't load .env");
        }
    }

    let args = CliArgs::parse();

    // Check if the config file exists
    if let Some(conf) = &args.config_file {
        if !Path::new(&conf).exists() {
            eprintln!("Config file not found: {}", conf);
            std::process::exit(1);
        }
    }

    // Observability
    // https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#example-syntax
    let filter = tracing_subscriber::EnvFilter::from_default_env();
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    // Settings
    // TODO: add cli arg to specify path to config file with default
    let settings = match RelayerConfiguration::new(args.config_file) {
        Ok(value) => value,
        Err(error) => {
            error!(
                "Unrecoverable error parsing relayer configuration: {:?}",
                error
            );
            panic!("Error: {:?}", error)
        }
    };

    info!("Configuration {:?}", settings);

    // SQS
    let config = aws_config::from_env().load().await;
    let sqs_client = aws_sdk_sqs::Client::new(&config);
    let kms_client = aws_sdk_kms::Client::new(&config);

    // Orchestrator
    // TODO: Should probably come from configuration
    let node_id = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab]; // Used to generate uuid
    let dispatcher = Arc::new(TokioEventDispatcher::<ZwsRelayerEvent>::new());
    let orchestrator = Orchestrator::new(Arc::clone(&dispatcher), &node_id);

    // Transaction services
    let mut tx_services = HashMap::new();

    match settings.gateway_chain.chain_config.signer_config {
        SignerConfig::Local(signer_config) => {
            let gateway_tx_service = match TransactionService::new(
                &settings.gateway_chain.chain_config.http_url,
                &signer_config.private_key_env,
                settings.gateway_chain.chain_config.chain_id,
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
            tx_services.insert(
                settings.gateway_chain.chain_config.chain_id,
                gateway_tx_service,
            );
        }
        SignerConfig::AWSKMS(signer_config) => {
            let _signer = alloy::signers::aws::AwsSigner::new(
                kms_client,
                signer_config.key_id,
                Some(settings.gateway_chain.chain_config.chain_id),
            )
            .await
            .unwrap();
            // TODO: add support for arbitrary signers in fhevm-relayer
            let error_message = "AWS KMS isn't properly supported in fhevm-relayer yet!";
            error!(error_message);
            panic!("{}", error_message);
        }
    }

    for host_chain in settings.host_chains.clone() {
        match host_chain.chain_config.signer_config {
            SignerConfig::Local(signer_config) => {
                let tx_service = match TransactionService::new(
                    &host_chain.chain_config.http_url,
                    &signer_config.private_key_env,
                    host_chain.chain_config.chain_id,
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
                tx_services.insert(host_chain.chain_config.chain_id, tx_service);
            }
            _ => {
                error!("Not supported signer");
                panic!("Unsupported signer");
            }
        }
    }

    // Event handlers
    let zws_handler: Arc<dyn EventHandler<ZwsRelayerEvent>> = Arc::new(
        ZWSRelayerHandler::new(
            settings.queues.console_queue.to_owned(),
            settings.queues.transaction_queue.to_owned(),
            Arc::clone(&orchestrator),
        )
        .await,
    );

    // NOTE: for now the transaction manager is part of the relayer but communication is already
    // done through SQS
    // NOTE: we could probably tweak the orchestrator to add SQS communication in the dispatch
    // event method
    let tx_manager_handler: Arc<dyn EventHandler<ZwsRelayerEvent>> = Arc::new(
        ZWSTransactionManagerMockHandler::new(
            settings.queues.relayer_queue.to_owned(),
            tx_services,
        )
        .await,
    );

    // Register handler for all events
    // Relayer handler
    orchestrator.register_handler(BlockchainEvent::event_id(), Arc::clone(&zws_handler));
    orchestrator.register_handler(
        SQSRelayerAuthorizationResponse::event_id(),
        Arc::clone(&zws_handler),
    );
    orchestrator.register_handler(HTTPZGatewayEvent::event_id(), Arc::clone(&zws_handler));
    orchestrator.register_handler(
        SQSRelayerTransactionResponse::event_id(),
        Arc::clone(&zws_handler),
    );
    orchestrator.register_handler(
        SQSRelayerInputRegistrationRequest::event_id(),
        Arc::clone(&zws_handler),
    );

    // Transaction handler
    orchestrator.register_handler(
        SQSRelayerTransactionRequest::event_id(),
        Arc::clone(&tx_manager_handler),
    );

    // Optional Console Mock
    // This is for testing purposes only
    if let Some(key_url) = settings.key_url {
        warn!("MOCKING CONSOLE! DEVELOPMENT PURPOSES ONLY");
        // Authorization handler
        let console_handler: Arc<dyn EventHandler<ZwsRelayerEvent>> =
            Arc::new(ZWSConsoleMockHandler::new(settings.queues.relayer_queue.to_owned()).await);
        orchestrator.register_handler(
            SQSRelayerAuthorizationRequest::event_id(),
            Arc::clone(&console_handler),
        );

        // HTTP listener
        tokio::spawn(http_listener(
            sqs_client.clone(),
            settings.queues.relayer_queue.to_string(),
            settings.queues.console_queue.to_string(),
            key_url,
            Arc::clone(&orchestrator),
        ));
    }

    // Initialize EVM Host adapters
    for host_chain in settings.host_chains.clone() {
        tokio::spawn(blockchain_event_listener(
            ChainName::Httpz,
            host_chain.chain_config.ws_url,
            vec![host_chain.decryption_oracle],
            Some(1000),
            Arc::clone(&orchestrator),
            "Host".to_owned(),
        ));
    }

    // Relayer SQS event listener
    tokio::spawn(sqs_listener(
        sqs_client.clone(),
        settings.queues.relayer_queue,
        Some(1000),
        Arc::clone(&orchestrator),
    ));

    // TX-Manager SQS event listener
    tokio::spawn(sqs_listener(
        sqs_client.clone(),
        settings.queues.transaction_queue,
        Some(1000),
        Arc::clone(&orchestrator),
    ));

    // Blockchain event listener
    tokio::spawn(blockchain_event_listener(
        ChainName::Gateway,
        settings.gateway_chain.chain_config.ws_url,
        vec![
            settings.gateway_chain.zkpok_manager,
            settings.gateway_chain.decryption_manager,
        ],
        Some(1000),
        Arc::clone(&orchestrator),
        "Gateway".to_owned(),
    ));

    debug!("All listeners started in their own tokio task");

    // Wait for ctrl + c signal to stop the application
    tokio::signal::ctrl_c().await.expect("Crtl-C Error");
    info!("Received ctrl + c signal, stopping...");
}
