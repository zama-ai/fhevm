//! fhEVM Relayer
//!
//! This relayer service acts as a bridge between Ethereum L1 and a rollup L2 network,
//! specifically handling FHE keys related operations. The service:
//!
//! 1. Listens for decryption events on Ethereum L1
//! 2. Forwards requests to the L2 for processing
//! 3. Receives responses from L2
//! 4. Sends results back to L1
//!
//! # Architecture
//!
//! The system consists of several key components:
//! - [`Orchestrator`]: Manages event flow and dispatch
//! - [`EthereumHostL1Handler`]: Processes L1 events and responses
//! - [`ArbitrumGatewayL2Handler`]: Manages L2 interaction
//! - [`TransactionService`]: Handles blockchain transactions
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
//! [Ethereum L1] → [L1 Listener] → [Orchestrator] → [L2 Handler]
//!                                       ↓
//! [Ethereum L1] ← [L1 Handler] ← [Orchestrator] ← [L2 Listener]
//! ```

use alloy::primitives::Address;
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
        traits::{EventHandler, HandlerRegistry},
        Orchestrator, TokioEventDispatcher,
    },
    transaction::{TransactionService, TxConfig},
};

/// Main entry point for the FHE Event Relayer service.
///
/// This function performs the following initialization steps:
/// 1. Loads and validates configuration
/// 2. Initializes logging
/// 3. Sets up transaction services for L1 and L2
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

    let rollup_settings = settings
        .get_network("rollup")
        .cloned()
        .map_err(|e| eyre::eyre!("Failed to get rollup settings: {}", e))?;

    // Prepare tx service for rollup
    let tx_service_rollup = TransactionService::new(
        &rollup_settings.http_url,
        &settings.transaction.private_key_gateway_env,
        rollup_settings.chain_id,
    )
    .await
    .map_err(|e| eyre::eyre!("Failed to create transaction service: {}", e))?;

    Arc::clone(&tx_service_rollup).spawn_maintenance_tasks();

    info!("Starting KMS Connector FHE Event Handler");

    let decryption_manager_address =
        Address::from_str(&settings.contracts.decryption_manager_address)
            .map_err(|_| eyre::eyre!("Invalid decryption manager address"))?;

    // Update the L2 filter to include the ZKPoK contract
    let zkpok_manager_address = Address::from_str(&settings.contracts.zkpok_manager_address)
        .map_err(|_| eyre::eyre!("Invalid ZKPoK manager address"))?;

    info!(
        ?decryption_manager_address,
        ?zkpok_manager_address,
        ?settings.networks.fhevm.ws_url,
        "Initialized contract addresses"
    );

    // === Intialize the orchestrator.
    let node_id = [0x02, 0x23, 0x45, 0x67, 0x89, 0xab];
    let dispatcher = Arc::new(TokioEventDispatcher::<GatewayProcessorsEvent>::new());
    let orchestrator = Orchestrator::new(Arc::clone(&dispatcher), &node_id);

    // === Register the event handlers
    let tx_config = TxConfig::from(settings.transaction);

    let gateway_processors_handler: Arc<dyn EventHandler<GatewayProcessorsEvent>> =
        Arc::new(GatewayProcessorsHandler::new(
            Arc::clone(&dispatcher),
            tx_service_rollup.clone(),
            tx_config.clone(),
            settings.contracts,
        ));

    // Register input event handlers

    // Event type: InputEventData::EventLogRequestFromGwL2
    orchestrator.register_handler(5, Arc::clone(&gateway_processors_handler));
    // Event type UserDecryptionEventData::EventLogRequestFromGwL2
    orchestrator.register_handler(4, Arc::clone(&gateway_processors_handler));

    // === Create a subscription for events and spawn a listener to listen for events from the subcription.

    // === Initialize Rollup L2 adapter
    let rollup_l2 = EthereumJsonRPCWsClient::new(ChainName::Gateway, &rollup_settings.ws_url)
        .await
        .map_err(|e| eyre::eyre!("Failed to create event handler for Rollup L2: {}", e))?;
    let rollup_l2 = Arc::new(rollup_l2);

    // === Create a subscription for events and spawn a listener to listen for events from the subcription.
    // TODO: Pass the event_dispatcher to the event_listener
    let filter_rollup = ContractAndTopicsFilter::new(
        vec![decryption_manager_address, zkpok_manager_address],
        vec![],
    );
    let subscription_rollup = rollup_l2.new_subscription(filter_rollup, None).await?;
    tokio::spawn(event_listener_gateway(
        subscription_rollup,
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
