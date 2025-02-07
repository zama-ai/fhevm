use alloy::primitives::Address;
use std::{str::FromStr, sync::Arc, time::Duration};
use tracing::info;
use tracing_subscriber::{fmt::SubscriberBuilder, EnvFilter};

use fhevm_relayer::{
    arbitrum_gateway_l2_handlers::ArbitrumGatewayL2Handler,
    config::settings::{LogConfig, Settings},
    ethereum::{ContractAndTopicsFilter, EthereumHostL1},
    ethereum_host_l1_handers::EthereumHostL1Handler,
    ethereum_listener::event_listener,
    orchestrator::{
        traits::{EventHandler, HandlerRegistry},
        Orchestrator, TokioEventDispatcher,
    },
    relayer_event::RelayerEvent,
    transaction::{TransactionService, TxConfig},
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // === Initialize settings
    let settings =
        Settings::new().map_err(|e| eyre::eyre!("Failed to load configuration: {}", e))?;

    init_tracing(&settings.log)?;

    settings
        .validate_addresses()
        .map_err(|e| eyre::eyre!("Configuration validation failed: {}", e))?;

    let tx_service = TransactionService::new(
        &settings.networks.fhevm.http_url,
        &settings.transaction.private_key_env,
        settings.networks.fhevm.chain_id,
    )
    .await
    .map_err(|e| eyre::eyre!("Failed to create transaction service: {}", e))?;

    let tx_service_clone = tx_service.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            tx_service_clone.cleanup_pending().await;
        }
    });

    info!("Starting FHE Event Handler");

    let decryption_oracle_address =
        Address::from_str(&settings.contracts.decryption_oracle_address)
            .map_err(|_| eyre::eyre!("Invalid decryption oracle address"))?;

    let tfhe_executor_address = Address::from_str(&settings.contracts.tfhe_executor_address)
        .map_err(|_| eyre::eyre!("Invalid TFHE executor address"))?;

    info!(
        ?decryption_oracle_address,
        ?tfhe_executor_address,
        ?settings.networks.fhevm.ws_url,
        "Initialized contract addresses"
    );

    // === Intialize the orchestrator.
    let node_id = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab];
    let dispatcher = Arc::new(TokioEventDispatcher::<RelayerEvent>::new());
    let orchestrator = Orchestrator::new(Arc::clone(&dispatcher), &node_id);

    // === Register the event handlers
    let tx_config = TxConfig::from(settings.transaction);
    let host_l1_event_log_handler: Arc<dyn EventHandler<RelayerEvent>> = Arc::new(
        EthereumHostL1Handler::new(Arc::clone(&dispatcher), tx_service.clone(), tx_config),
    );
    orchestrator.register_handler(0, Arc::clone(&host_l1_event_log_handler));
    orchestrator.register_handler(3, Arc::clone(&host_l1_event_log_handler));

    let gateway_l2_event_handler: Arc<dyn EventHandler<RelayerEvent>> =
        Arc::new(ArbitrumGatewayL2Handler::new(Arc::clone(&dispatcher)));
    orchestrator.register_handler(1, Arc::clone(&gateway_l2_event_handler));
    orchestrator.register_handler(2, Arc::clone(&gateway_l2_event_handler));

    // === Initialize Ethereum host L1 adapter
    let host_l1 = EthereumHostL1::new(&settings.networks.fhevm.ws_url)
        .await
        .map_err(|e| eyre::eyre!("Failed to create event handler: {}", e))?;
    let host_l1 = Arc::new(host_l1);

    // === Create a subscription for events and spawn a listener to listen for events from the subcription.
    // TODO: Pass the event_dispatcher to the event_listener
    let filter = ContractAndTopicsFilter::new(
        vec![decryption_oracle_address, tfhe_executor_address],
        vec![],
    );
    let subscription = host_l1.new_subscription(filter, None).await?;
    tokio::spawn(event_listener(subscription, Arc::clone(&orchestrator)));

    // === Wait for ctrl + c signal to stop the application
    tokio::signal::ctrl_c().await?;
    info!("Received ctrl + c signal, stopping...");
    Ok(())
}

/// Initialize tracing based on configuration settings
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
