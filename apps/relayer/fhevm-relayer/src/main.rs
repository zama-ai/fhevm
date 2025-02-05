use alloy::primitives::Address;
use alloy_sol_types::SolEvent;
use std::{str::FromStr, sync::Arc};
use tracing::info;
use tracing_subscriber::{fmt::SubscriberBuilder, EnvFilter};

use fhevm_relayer::{
    config::settings::{LogConfig, Settings},
    ethereum::{
        bindings::{DecryptionOracle, GatewayContract, TFHEExecutor, Transfer},
        extract_event_signature, ContractAndTopicsFilter, EthereumHostL1,
    },
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
        let ethereum_events_listener = tokio::select! {
            event = subscription.next() => match event {
                Some(event) => {
                    handle_event(event).unwrap();
                    // info!(?event, "Received event");
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
        };
        ethereum_events_listener
    }

    info!("Shutdown complete");
    Ok(())
}

pub fn handle_event(event: alloy::rpc::types::Log) -> Result<(), eyre::Error> {
    match extract_event_signature(&event)? {
        &GatewayContract::EventDecryption::SIGNATURE_HASH => {
            info!(
                "{:?} {:?}",
                GatewayContract::EventDecryption::SIGNATURE,
                event.block_number
            )
        }
        &DecryptionOracle::DecryptionRequest::SIGNATURE_HASH => {
            info!(
                "{:?} {:?}",
                DecryptionOracle::DecryptionRequest::SIGNATURE,
                event.block_number
            )
        }
        &TFHEExecutor::FheAdd::SIGNATURE_HASH => {
            info!(
                "{:?} {:?}",
                TFHEExecutor::FheAdd::SIGNATURE,
                event.block_number
            )
        }
        &TFHEExecutor::FheSub::SIGNATURE_HASH => {
            info!(
                "{:?} {:?}",
                TFHEExecutor::FheSub::SIGNATURE,
                event.block_number
            )
        }
        &Transfer::SIGNATURE_HASH => {
            info!("{:?} {:?}", Transfer::SIGNATURE, event.block_number)
        }
        _ => {
            // Ignore the event
        }
    }
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
