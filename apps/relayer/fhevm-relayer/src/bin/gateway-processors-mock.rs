//! Gateway processors mock
//!
//! This mock service consumes request events from gateway contracts and sends a mock response.

use alloy::{
    primitives::Address,
    signers::{local::PrivateKeySigner, Signer},
};
use std::{str::FromStr, sync::Arc};
use tracing::info;
use tracing_subscriber::{fmt::SubscriberBuilder, EnvFilter};

use fhevm_relayer::{
    blockchain::ethereum::{ChainName, ContractAndTopicsFilter, EthereumJsonRPCWsClient},
    config::settings::{LogConfig, Settings},
    gateway_processors_mock::{
        event_listener_gateway, GatewayProcessorsEvent, GatewayProcessorsHandler,
    },
    orchestrator::{
        hooks::EventLoggingHook,
        traits::{EventHandler, HandlerRegistry, HookRegistry},
        Orchestrator, TokioEventDispatcher,
    },
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

    let gateway_settings = settings
        .get_network("gateway")
        .cloned()
        .map_err(|e| eyre::eyre!("Failed to get gateway settings: {}", e))?;

    let mut signer_gateway: PrivateKeySigner =
        "e746bc71f6bee141a954e6a49bc9384d334e393a7ea1e70b50241cb2e78e9e4c".parse()?;
    signer_gateway.set_chain_id(Some(gateway_settings.chain_id));

    // Prepare tx service for gateway
    let tx_service_gateway =
        TransactionService::new(&gateway_settings.ws_url, Arc::new(signer_gateway))
            .await
            .map_err(|e| eyre::eyre!("Failed to create transaction service: {}", e))?;

    Arc::clone(&tx_service_gateway).spawn_maintenance_tasks();

    info!("Starting mock event handler");

    let decryption_address = Address::from_str(&settings.contracts.decryption_address)
        .map_err(|_| eyre::eyre!("Invaliddecryption contract address"))?;

    let input_verification_address =
        Address::from_str(&settings.contracts.input_verification_address)
            .map_err(|_| eyre::eyre!("InvalidInputVerification address"))?;

    info!(
        ?decryption_address,
        ?input_verification_address,
        ?settings.networks.fhevm.ws_url,
        "Initialized contract addresses"
    );

    // === Intialize the orchestrator.
    let orchestrator = Orchestrator::new(Arc::new(
        TokioEventDispatcher::<GatewayProcessorsEvent>::new(),
    ));

    orchestrator.register_pre_dispatch_hook(EventLoggingHook::new(
        "Received gateway processor mock event".to_string(),
    ));

    // === Register the event handlers
    let tx_config = TxConfig::from(settings.transaction);

    let gateway_processors_handler: Arc<dyn EventHandler<GatewayProcessorsEvent>> =
        Arc::new(GatewayProcessorsHandler::new(
            tx_service_gateway.clone(),
            tx_config.clone(),
            settings.contracts,
        ));

    // Register input event handlers

    // Event type: InputEventData::EventLogRequestFromGw
    orchestrator.register_handler(5, Arc::clone(&gateway_processors_handler));
    // Event type UserDecryptionEventData::EventLogRequestFromGw
    orchestrator.register_handler(4, Arc::clone(&gateway_processors_handler));
    // Event type PublicDecryptionEventData::EventLogRequestFromGw
    orchestrator.register_handler(3, Arc::clone(&gateway_processors_handler));

    // === Create a subscription for events and spawn a listener to listen for events from the subcription.

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
    tokio::spawn(event_listener_gateway(
        subscription_gateway,
        Arc::clone(&orchestrator),
    ));

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
