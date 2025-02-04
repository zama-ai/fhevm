use alloy::primitives::Address;
use std::{str::FromStr, sync::Arc};
use tracing::{error, info};
use tracing_subscriber::{fmt::SubscriberBuilder, EnvFilter};

use fhevm_relayer::{
    config::settings::{LogConfig, Settings},
    event::{
        processors::{tfhe_executor::TfheExecutorEventHandler, DecryptionOracleEventHandler},
        registry::EventRegistry,
    },
    service::{ContractAndTopicsFilter, EthereumHostL1},
};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let settings =
        Settings::new().map_err(|e| eyre::eyre!("Failed to load configuration: {}", e))?;

    settings
        .validate_addresses()
        .map_err(|e| eyre::eyre!("Configuration validation failed: {}", e))?;

    init_tracing(&settings.log)?;

    info!("Starting FHE Event Handler");

    let decryption_oracle_address =
        Address::from_str(&settings.contracts.decryption_oracle_address)
            .map_err(|_| eyre::eyre!("Invalid decryption oracle address"))?;

    let tfhe_executor_address = Address::from_str(&settings.contracts.tfhe_executor_address)
        .map_err(|_| eyre::eyre!("Invalid TFHE executor address"))?;

    info!(
        ?decryption_oracle_address,
        ?tfhe_executor_address,
        ?settings.network.ws_url,
        "Initialized contract addresses"
    );

    // Create and configure the event registry
    let registry = Arc::new(EventRegistry::new());
    registry.register_contract(decryption_oracle_address);
    registry.register_contract(tfhe_executor_address);

    // Create and register event processors
    let tfhe_executor = TfheExecutorEventHandler::new();
    registry.register_event(tfhe_executor_address, tfhe_executor);

    let decryption_oracle_executor = DecryptionOracleEventHandler;
    registry.register_event(decryption_oracle_address, decryption_oracle_executor);

    // Create the real event handler for WebSocket connection
    let event_handler = EthereumHostL1::new(&settings.network.ws_url)
        .await
        .map_err(|e| eyre::eyre!("Failed to create event handler: {}", e))?;
    let event_handler = Arc::new(event_handler);
    let filter = ContractAndTopicsFilter::new(
        vec![decryption_oracle_address, tfhe_executor_address],
        vec![],
    );
    let mut subscription = event_handler.new_subscription(filter, None).await?;

    // Spawn the event listener
    loop {
        tokio::select! {
            event = subscription.next() => match event {
                Some(event) => {
                    info!(?event, "Received event");
                }
                None => {
                    info!("Subscription stream ended");
                    break;
                }
            },
            _ = tokio::signal::ctrl_c() => {
                info!("Received ctrl + c signal, stopping...");
                break;
            }
        }
    }

    info!("Shutdown complete");
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
