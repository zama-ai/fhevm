use alloy::primitives::Address;
use alloy::transports::TransportError;
use eyre::Result;
use fhevm_relayer::common::provider::{
    DECRYPTION_ORACLE_EVENT_SIGNATURE, TFHE_EXECUTOR_FHE_ADD_EVENT_SIGNATURE,
};
use fhevm_relayer::config::settings::LogConfig;
use fhevm_relayer::config::settings::Settings;

use fhevm_relayer::event::registry::EventRegistry;
use fhevm_relayer::RealEventHandler;
use std::str::FromStr;
use std::sync::Arc;
use thiserror::Error;
use tracing::{error, info};
use tracing_subscriber::fmt::SubscriberBuilder;
use tracing_subscriber::EnvFilter;

#[derive(Error, Debug)]
pub enum EventHandlerError {
    #[error("ABI decode error: {0}")]
    AbiError(#[from] alloy_sol_types::Error),

    #[error("Transport error: {0}")]
    TransportError(#[from] TransportError),

    #[error("Event processing failed: {0}")]
    ProcessingError(String),

    #[error("Task failed: {0}")]
    TaskError(#[from] tokio::task::JoinError),
}

#[tokio::main]
async fn main() -> Result<()> {
    let settings =
        Settings::new().map_err(|e| eyre::eyre!("Failed to load configuration: {}", e))?;

    // Validate contract addresses
    settings
        .validate_addresses()
        .map_err(|e| eyre::eyre!("Configuration validation failed: {}", e))?;

    init_tracing(&settings.log)?;

    info!("--- Real Event Handler ---");

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

    let registry = Arc::new(EventRegistry::new());
    registry.register_contract(decryption_oracle_address);
    registry.register_contract(tfhe_executor_address);

    let real_event_handler =
        RealEventHandler::new(&settings.network.ws_url, registry.clone()).await?;
    let real_event_handler = Arc::new(real_event_handler);

    registry.register_event(
        decryption_oracle_address,
        DECRYPTION_ORACLE_EVENT_SIGNATURE,
        real_event_handler.clone(),
    );

    registry.register_event(
        tfhe_executor_address,
        TFHE_EXECUTOR_FHE_ADD_EVENT_SIGNATURE,
        real_event_handler.clone(),
    );

    let (shutdown_tx, mut shutdown_rx) = tokio::sync::broadcast::channel(1);
    let shutdown_handle = shutdown_tx.clone();

    let listener_handle = tokio::spawn({
        let real_event_handler = real_event_handler.clone();
        async move {
            match real_event_handler.listen_for_contract_events().await {
                Ok(()) => Ok(()),
                Err(e) => {
                    error!(?e, "Event listener error");
                    Err(EventHandlerError::ProcessingError(e.to_string()))
                }
            }
        }
    });

    tokio::select! {
        result = listener_handle => {
            match result {
                Ok(Ok(())) => info!("Event listener completed successfully"),
                Ok(Err(e)) => {
                    error!(?e, "Event listener failed");
                    let _ = shutdown_handle.send(());
                }
                Err(e) => {
                    error!(?e, "Event listener task panicked");
                    let _ = shutdown_handle.send(());
                }
            }
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Received interrupt signal, initiating graceful shutdown...");
            let _ = shutdown_handle.send(());
        }
        _ = shutdown_rx.recv() => {
            info!("Received shutdown request, stopping...");
        }
    }

    info!("Shutdown complete");
    Ok(())
}

/// Initialize tracing based on configuration settings
fn init_tracing(log_config: &LogConfig) -> Result<()> {
    let env_filter = match log_config.level.as_str() {
        "trace" => EnvFilter::new("trace"),
        "debug" => EnvFilter::new("debug"),
        "info" => EnvFilter::new("info"),
        "warn" => EnvFilter::new("warn"),
        "error" => EnvFilter::new("error"),
        _ => EnvFilter::from_default_env(), // Fallback to env if invalid level
    };

    // Build base subscriber with common settings
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

    // Log the initialization success with current settings
    tracing::info!(
        level = log_config.level,
        format = log_config.format,
        show_file_line = log_config.show_file_line,
        show_thread_ids = log_config.show_thread_ids,
        "Tracing initialized successfully"
    );

    Ok(())
}
