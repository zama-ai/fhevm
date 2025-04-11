use alloy::providers::Provider;
use alloy_provider::{ProviderBuilder, WsConnect};
use clap::Parser;
use kms_connector::{
    core::{
        cli::{Cli, Commands},
        config::Config,
        connector::KmsCoreConnector,
        utils::wallet::KmsWallet,
    },
    error::{Error, Result},
    kms_core_adapters::service::KmsServiceImpl,
};
use std::sync::Arc;
use std::time::Duration;
use tokio::{
    signal::unix::{signal, SignalKind},
    sync::broadcast,
    task::JoinHandle,
    time::sleep,
};
use tracing::{error, info};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

const RETRY_DELAY: Duration = Duration::from_secs(5);
const DEFAULT_CHANNEL_SIZE: usize = 1000;

/// Keep trying to connect to the RPC endpoint until successful or shutdown signal
async fn connect_with_retry(
    rpc_url: &str,
    mut shutdown_rx: broadcast::Receiver<()>,
) -> Result<Option<Arc<impl Provider + Clone + std::fmt::Debug + 'static>>> {
    loop {
        info!(
            "Attempting to connect to Gateway L2 RPC endpoint: {}",
            rpc_url
        );
        let ws = WsConnect::new(rpc_url);
        match ProviderBuilder::new().on_ws(ws).await {
            Ok(provider) => {
                info!("Connected to Gateway L2 RPC endpoint");
                return Ok(Some(Arc::new(provider)));
            }
            Err(e) => {
                error!(
                    "Failed to connect to Gateway L2 RPC endpoint: {}, retrying in {}s...",
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
    // Initialize wallet based on configuration
    let wallet = if let Some(aws_kms_config) = &config.aws_kms_config {
        info!(
            "Using AWS KMS for signing with key ID: {}",
            aws_kms_config.key_id
        );
        KmsWallet::from_aws_kms(
            aws_kms_config.key_id.clone(),
            aws_kms_config.region.clone(),
            aws_kms_config.endpoint.clone(),
            Some(config.chain_id),
        )
        .await?
    } else if let Some(signing_key_path) = &config.signing_key_path {
        info!("Using signing key from file: {}", signing_key_path);
        KmsWallet::from_signing_key_file(Some(signing_key_path), Some(config.chain_id))?
    } else if let Some(private_key) = &config.private_key {
        info!("Using private key from configuration");
        KmsWallet::from_private_key_str(private_key, Some(config.chain_id))?
    } else {
        // Initialize wallet with account index derived from service name
        let account_index = config.get_account_index();
        info!("Using mnemonic with account index: {}", account_index);
        KmsWallet::from_mnemonic_with_index(&config.mnemonic, account_index, Some(config.chain_id))?
    };

    info!(
        "Wallet created successfully with address: {:#x}",
        wallet.address()
    );

    info!(
        "Using contracts for EVENTS subscription:\n\tDecryptionManager: {}\n\tHttpz: {}",
        config.decryption_manager_address, config.httpz_address
    );

    // Initialize KMS service
    let kms_core_endpoint = config.kms_core_endpoint.clone();
    info!("Connecting to KMS-Core at {}", kms_core_endpoint);
    let kms_provider = Arc::new(KmsServiceImpl::new(&kms_core_endpoint, config.clone()));

    // Create and start connector
    let (mut connector, event_rx) = KmsCoreConnector::new(
        gw_provider.clone(),
        wallet.clone(),
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
            let mut config = Config::from_env_and_file(config.as_ref())?;
            if let Some(name) = name {
                config.service_name = name;
                info!("Using custom service name: {}", config.service_name);
            }

            // Create shutdown channel
            let (shutdown_tx, shutdown_rx) =
                broadcast::channel(config.channel_size.unwrap_or(DEFAULT_CHANNEL_SIZE));

            // Setup signal handlers for graceful shutdown
            let signal_handle = setup_signal_handlers(shutdown_tx.clone()).await?;

            // Connect to L2 gateway with shutdown handling
            let provider =
                match connect_with_retry(&config.gwl2_url, shutdown_tx.subscribe()).await? {
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
        Commands::Validate { config } => match Commands::validate_config(&config) {
            Ok(()) => info!("Configuration is valid: {}", config.display()),
            Err(e) => error!("Configuration validation failed: {}", e),
        },
    }

    Ok(())
}
