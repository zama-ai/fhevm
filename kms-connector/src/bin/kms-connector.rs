use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use clap::Parser;
use kms_connector::{
    core::{
        cli::{Cli, Commands},
        config::Config,
        connector::KmsCoreConnector,
    },
    error::{Error, Result},
    kms_core_adapters::service::KmsServiceImpl,
};
use std::sync::Arc;
use std::time::Duration;
use tokio::{
    signal::unix::{SignalKind, signal},
    sync::broadcast,
    task::JoinHandle,
    time::sleep,
};
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

const RETRY_DELAY: Duration = Duration::from_secs(5);

/// Keep trying to connect to the RPC endpoint until successful or shutdown signal
async fn connect_with_retry(
    config: &Config,
    mut shutdown_rx: broadcast::Receiver<()>,
) -> Result<Option<Arc<impl Provider + Clone + std::fmt::Debug + 'static>>> {
    loop {
        info!(
            "Attempting to connect to Gateway RPC endpoint: {}",
            config.gateway_url
        );
        match ProviderBuilder::new()
            .wallet(config.wallet.clone())
            .on_ws(WsConnect::new(&config.gateway_url))
            .await
        {
            Ok(provider) => {
                info!("Connected to Gateway RPC endpoint");
                return Ok(Some(Arc::new(provider)));
            }
            Err(e) => {
                error!(
                    "Failed to connect to Gateway RPC endpoint: {}, retrying in {}s...",
                    e,
                    RETRY_DELAY.as_secs()
                );

                // Wait for either the retry delay or shutdown signal
                tokio::select! {
                    _ = sleep(RETRY_DELAY) => continue,
                    _ = shutdown_rx.recv() => {
                        info!("Received shutdown signal during connection retry");
                        return Ok(None);
                    }
                }
            }
        }
    }
}

/// Run the connector with automatic reconnection
async fn run_connector(
    config: Config,
    gw_provider: Arc<impl Provider + Clone + std::fmt::Debug + 'static>,
    shutdown_rx: broadcast::Receiver<()>,
) -> Result<()> {
    // Initialize KMS service
    let kms_core_endpoint = config.kms_core_endpoint.clone();
    info!("Connecting to KMS-Core at {}", kms_core_endpoint);
    let kms_provider = Arc::new(KmsServiceImpl::new(&kms_core_endpoint, config.clone()));

    // Create and start connector
    let (mut connector, event_rx) = KmsCoreConnector::new(
        gw_provider.clone(),
        config,
        kms_provider.clone(),
        shutdown_rx.resubscribe(),
    );

    // Start the connector
    connector.start(event_rx).await?;

    // Stop the connector gracefully
    connector.stop().await?;

    Ok(())
}

async fn setup_signal_handlers(shutdown_tx: broadcast::Sender<()>) -> Result<JoinHandle<()>> {
    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sigint = signal(SignalKind::interrupt())?;

    Ok(tokio::spawn(async move {
        tokio::select! {
            _ = sigterm.recv() => {
                info!("Received SIGTERM signal");
            }
            _ = sigint.recv() => {
                info!("Received SIGINT signal");
            }
        }
        info!("Initiating graceful shutdown...");
        let _ = shutdown_tx.send(());
    }))
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup logging
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    match cli.command {
        Commands::Start { config, name } => {
            if let Some(config_path) = &config {
                info!(
                    "Starting KMS connector with config file: {}",
                    config_path.display()
                );
            } else {
                info!("Starting KMS connector using only environment variables");
            }

            // Load config and potentially override service name
            let mut config = Config::from_env_and_file(config.as_ref()).await?;
            if let Some(name) = name {
                config.service_name = name;
                info!("Using custom service name: {}", config.service_name);
            }

            // Create shutdown channel
            let (shutdown_tx, shutdown_rx) = broadcast::channel(config.channel_size);

            // Setup signal handlers for graceful shutdown
            let signal_handle = setup_signal_handlers(shutdown_tx.clone()).await?;

            // Connect to Gateway with shutdown handling
            let provider = match connect_with_retry(&config, shutdown_tx.subscribe()).await? {
                Some(provider) => provider,
                None => {
                    info!("Shutting down during connection attempt");
                    return Ok(());
                }
            };

            // Run the connector
            let connector_handle = tokio::spawn(run_connector(config, provider, shutdown_rx));

            // Wait for either the connector to finish or a shutdown signal
            tokio::select! {
                connector_result = connector_handle => {
                    match connector_result {
                        Ok(Ok(())) => info!("Connector finished successfully"),
                        Ok(Err(e)) => {
                            error!("Connector error: {}", e);
                            return Err(e);
                        }
                        Err(e) => {
                            error!("Connector task failed: {}", e);
                            return Err(Error::Channel(format!("Task join error: {}", e)));
                        }
                    }
                }
                _ = signal_handle => {
                    info!("Received shutdown signal");
                }
            }

            // Initiate shutdown
            let _ = shutdown_tx.send(());

            info!("KMS Connector stopped successfully");
        }
        Commands::List { full_path } => match Commands::list_configs(full_path) {
            Ok(configs) => {
                info!("Available configurations:");
                for config in configs {
                    info!("  {}", config.display());
                }
            }
            Err(e) => error!("Error listing configs: {}", e),
        },
        Commands::Validate { config } => match Commands::validate_config(&config).await {
            Ok(()) => info!("Configuration is valid: {}", config.display()),
            Err(e) => error!("Configuration validation failed: {}", e),
        },
    }

    Ok(())
}
