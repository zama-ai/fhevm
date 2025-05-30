//! fhevm Relayer
//!
//! fhevm relayer service acts as a bridge between fhevm and gateway blockchains,
//! specifically handling FHE keys related operations. The service:
//!
//! 1. Listens for requests to from fhevm blockchain events to http endpoint.
//! 2. Forwards requests to the gateway blockchain for processing
//! 3. Receives responses from gateway blockchain
//! 4. Relay the result back to source (fhevm blockchain or HTTP caller).
//!
//! # Architecture
//!
//! The system consists of several key components:
//! - [`Orchestrator`]: Manages event flow and dispatch
//! - [`FhevmHandler`]: Processes fhevm blockchain events and responses
//! - [`GatewayHandler`]: Manages gateway interactions
//! - [`TransactionService`]: Handles blockchain transactions (for both fhevm and gateway)
//!
//! # Configuration
//!
//! The service is configured via:
//! - Environment variables
//! - Configuration files in the `config/` directory
//! - Command-line arguments
//!
//! See [`Settings`] for detailed configuration options.
//!
//! # Event Flow
//!
//! ```text
//! [fhevm] → [fhevm listener] → [Orchestrator] → [gateway Handler]
//!                                       ↓
//! [fhevm] ← [fhevm Handler] ← [Orchestrator] ← [gateway Listener]
//! ```

use alloy::primitives::Address;
use alloy::signers::Signer;
use std::{str::FromStr, sync::Arc};
use tracing::info;
use tracing_subscriber::{fmt::SubscriberBuilder, EnvFilter};

use fhevm_relayer::{
    blockchain::{
        ethereum::listener::{
            ethereum_listener, fhevm_event_log_converter, gateway_event_log_converter,
        },
        ethereum::{
            parse_private_key, ChainName, ContractAndTopicsFilter, EthereumJsonRPCWsClient,
        },
        InputProofGatewayHandler, PublicDecryptFhevmHandler, PublicDecryptGatewayHandler,
        UserDecryptGatewayHandler,
    },
    config::settings::{LogConfig, Settings},
    core::event::{
        GenericEventId, InputProofEventId, PublicDecryptEventId, RelayerEvent, UserDecryptEventId,
    },
    http::http_server::run_http_server,
    orchestrator::{
        hooks::{EventLoggingHook, EventPersistenceHook},
        traits::{EventHandler, HandlerRegistry, HookRegistry},
        Orchestrator, TokioEventDispatcher,
    },
    store::{key_value_db::InMemoryKVStore, EventStore},
    transaction::{TransactionService, TxConfig},
};

/// Main entry point for the FHE Event Relayer service.
///
/// This function performs the following initialization steps:
/// 1. Loads and validates configuration
/// 2. Initializes logging
/// 3. Sets up transaction services for fhevm and gateway
/// 4. Creates and configures event handlers
/// 5. Starts event listeners
/// 6. Waits for shutdown signal
#[tokio::main]
async fn main() -> eyre::Result<()> {
    // === Initialize settings
    let settings =
        Settings::new().map_err(|e| eyre::eyre!("Failed to load configuration: {}", e))?;

    init_tracing(&settings.log)?;

    settings
        .validate_addresses()
        .map_err(|e| eyre::eyre!("Configuration validation failed: {}", e))?;

    let mut fhevm_signer = parse_private_key(&settings.transaction.private_key_fhevm_env)?;

    fhevm_signer.set_chain_id(Some(settings.networks.fhevm.chain_id));

    // Prepare tx service for fhevm
    let tx_service =
        TransactionService::new(&settings.networks.fhevm.ws_url, Arc::new(fhevm_signer))
            .await
            .map_err(|e| eyre::eyre!("Failed to create transaction service: {}", e))?;

    Arc::clone(&tx_service).spawn_maintenance_tasks();

    let gateway_settings = settings
        .get_network("gateway")
        .cloned()
        .map_err(|e| eyre::eyre!("Failed to get gateway settings: {}", e))?;

    let mut gateway_signer = parse_private_key(&settings.transaction.private_key_gateway_env)?;

    gateway_signer.set_chain_id(Some(gateway_settings.chain_id));

    // Prepare tx service for gateway
    let tx_service_gateway =
        TransactionService::new(&gateway_settings.ws_url, Arc::new(gateway_signer))
            .await
            .map_err(|e| eyre::eyre!("Failed to create transaction service: {}", e))?;

    Arc::clone(&tx_service_gateway).spawn_maintenance_tasks();

    info!("Starting FHE Event Handler");

    let decryption_oracle_address =
        Address::from_str(&settings.contracts.decryption_oracle_address)
            .map_err(|_| eyre::eyre!("Invalid decryption oracle address"))?;

    let decryption_address = Address::from_str(&settings.contracts.decryption_address)
        .map_err(|_| eyre::eyre!("Invalid decryption contract address"))?;

    let input_verification_address =
        Address::from_str(&settings.contracts.input_verification_address)
            .map_err(|_| eyre::eyre!("Invalid InputVerification address"))?;

    info!(
        ?decryption_oracle_address,
        ?decryption_address,
        ?input_verification_address,
        ?settings.networks.fhevm.ws_url,
        "Initialized contract addresses"
    );

    // === Intialize the orchestrator.
    let orchestrator = Orchestrator::new(Arc::new(TokioEventDispatcher::<RelayerEvent>::new()));

    // Create the storage components for event persistence
    let kv_store = Arc::new(InMemoryKVStore::default());
    let event_store = Arc::new(EventStore::<RelayerEvent>::new(kv_store.clone()));

    // Register event logging hook to capture all events
    orchestrator
        .register_pre_dispatch_hook(EventLoggingHook::new("Received relayer event".to_string()));

    // Register event persistence hook
    orchestrator.register_pre_dispatch_hook(EventPersistenceHook::<RelayerEvent>::new(
        event_store.clone(),
    ));

    // === Register the event handlers
    let tx_config = TxConfig::from(settings.transaction.clone());
    let fhevm_event_log_handler: Arc<dyn EventHandler<RelayerEvent>> =
        Arc::new(PublicDecryptFhevmHandler::new(
            Arc::clone(&orchestrator),
            tx_service.clone(),
            tx_config.clone(),
        ));

    let input_proof_gw_handler: Arc<dyn EventHandler<RelayerEvent>> =
        Arc::new(InputProofGatewayHandler::new(
            Arc::clone(&orchestrator),
            tx_service_gateway.clone(),
            tx_config.clone(),
            settings.contracts.clone(),
        ));

    // Register input event handlers
    orchestrator.register_handler(
        InputProofEventId::ReqRcvdFromUser.into(),
        Arc::clone(&input_proof_gw_handler),
    );
    orchestrator.register_handler(
        InputProofEventId::ReqSentToGw.into(),
        Arc::clone(&input_proof_gw_handler),
    );
    orchestrator.register_handler(
        InputProofEventId::RespRcvdFromGw.into(),
        Arc::clone(&input_proof_gw_handler),
    );
    orchestrator.register_handler(
        GenericEventId::EventLogRcvdFromGw.into(),
        Arc::clone(&input_proof_gw_handler),
    );

    // Register public decryption events
    orchestrator.register_handler(
        GenericEventId::EventLogRcvdFromFhevm.into(),
        Arc::clone(&fhevm_event_log_handler),
    );
    orchestrator.register_handler(
        PublicDecryptEventId::RespRcvdFromGw.into(),
        Arc::clone(&fhevm_event_log_handler),
    );
    orchestrator.register_handler(
        PublicDecryptEventId::RespSentToFhevm.into(),
        Arc::clone(&fhevm_event_log_handler),
    );
    orchestrator.register_handler(
        UserDecryptEventId::RespSentToUser.into(),
        Arc::clone(&fhevm_event_log_handler),
    );

    let public_decrypt_gateway_handler: Arc<dyn EventHandler<RelayerEvent>> =
        Arc::new(PublicDecryptGatewayHandler::new(
            Arc::clone(&orchestrator),
            tx_service_gateway.clone(),
            tx_config.clone(),
            settings.contracts.clone(),
            gateway_settings.http_url.clone(),
            settings.transaction.clone().ciphertext_check_retry.clone(),
        ));

    let user_decrypt_gateway_handler: Arc<dyn EventHandler<RelayerEvent>> =
        Arc::new(UserDecryptGatewayHandler::new(
            Arc::clone(&orchestrator),
            tx_service_gateway,
            tx_config,
            settings.contracts,
            gateway_settings.http_url.clone(),
            settings.transaction.clone().ciphertext_check_retry.clone(),
        ));

    orchestrator.register_handler(
        PublicDecryptEventId::ReqRcvdFromUser.into(),
        Arc::clone(&public_decrypt_gateway_handler),
    );

    orchestrator.register_handler(
        PublicDecryptEventId::ReqSentToGw.into(),
        Arc::clone(&public_decrypt_gateway_handler),
    );

    // Register user decryption events

    orchestrator.register_handler(
        UserDecryptEventId::ReqRcvdFromUser.into(),
        Arc::clone(&user_decrypt_gateway_handler),
    );

    orchestrator.register_handler(
        UserDecryptEventId::ReqSentToGw.into(),
        Arc::clone(&user_decrypt_gateway_handler),
    );

    orchestrator.register_handler(
        GenericEventId::EventLogRcvdFromGw.into(),
        Arc::clone(&public_decrypt_gateway_handler),
    );

    orchestrator.register_handler(
        GenericEventId::EventLogRcvdFromGw.into(),
        Arc::clone(&user_decrypt_gateway_handler),
    );

    // === Initialize fhevm adapter
    let fhevm = EthereumJsonRPCWsClient::new(ChainName::Fhevm, &settings.networks.fhevm.ws_url)
        .await
        .map_err(|e| eyre::eyre!("Failed to create event handler: {}", e))?;
    let fhevm = Arc::new(fhevm);

    // === Create a subscription for events and spawn a listener to listen for events from the subcription.
    // TODO: Pass the event_dispatcher to the event_listener
    let filter_fhevm = ContractAndTopicsFilter::new(vec![decryption_oracle_address], vec![]);
    let subscription_fhevm = fhevm.new_subscription(filter_fhevm, None).await?;
    tokio::spawn(ethereum_listener(
        subscription_fhevm,
        fhevm_event_log_converter,
        Arc::clone(&orchestrator),
    ));

    // === Initialize gateway adapter
    let gateway = EthereumJsonRPCWsClient::new(ChainName::Gateway, &gateway_settings.ws_url)
        .await
        .map_err(|e| eyre::eyre!("Failed to create event handler for gateway: {}", e))?;
    let gateway = Arc::new(gateway);

    // === Create a subscription for events and spawn a listener to listen for events from the subcription.
    // TODO: Pass the event_dispatcher to the event_listener
    let filter_gateway =
        ContractAndTopicsFilter::new(vec![decryption_address, input_verification_address], vec![]);
    let subscription_gateway = gateway.new_subscription(filter_gateway, None).await?;
    tokio::spawn(ethereum_listener(
        subscription_gateway,
        gateway_event_log_converter,
        Arc::clone(&orchestrator),
    ));

    tokio::spawn(run_http_server(Arc::clone(&orchestrator)));

    // === Wait for ctrl + c signal to stop the application
    tokio::signal::ctrl_c().await?;
    info!("Received ctrl + c signal, stopping...");
    Ok(())
}

/// Initialize tracing based on configuration settings.
///
/// # Arguments
/// * `log_config` - The [`LogConfig`] containing logging preferences
///
/// # Returns
/// * `Ok(())` - If logging was successfully initialized
/// * `Err(`[`eyre::Error`]`)` - If initialization failed
///
/// # Configuration Options
/// - Log level (trace, debug, info, warn, error)
/// - Log format (compact, pretty, json)
/// - File and line number display
/// - Thread ID display
fn init_tracing(log_config: &LogConfig) -> eyre::Result<()> {
    let env_filter = match log_config.level.as_str() {
        "trace" => EnvFilter::new("trace"),
        "debug" => EnvFilter::new("debug"),
        "info" => EnvFilter::new("info"),
        "warn" => EnvFilter::new("warn"),
        "error" => EnvFilter::new("error"),
        _ => EnvFilter::from_default_env(), // Fallback to env if invalid level
    };

    // Build subscriber with common settings
    let builder = SubscriberBuilder::default()
        .with_env_filter(env_filter)
        .with_ansi(true)
        .with_file(log_config.show_file_line)
        .with_line_number(log_config.show_file_line)
        .with_thread_ids(log_config.show_thread_ids)
        .with_target(false);

    // Try to initialize the subscriber
    builder
        .try_init()
        .map_err(|e| eyre::eyre!("Failed to initialize tracing: {}", e))?;

    info!(
        level = ?log_config.level,
        format = ?log_config.format,
        show_file_line = ?log_config.show_file_line,
        show_thread_ids = ?log_config.show_thread_ids,
        "Tracing initialized successfully"
    );

    Ok(())
}
