use connector_utils::{
    cli::{Cli, Subcommands},
    signal::install_signal_handlers,
};
use tokio_util::sync::CancellationToken;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use tx_sender::core::{Config, TransactionSender};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let subcommand = Cli::new("TransactionSender").parse();
    match subcommand {
        Subcommands::Validate { config } => {
            Config::from_env_and_file(Some(config)).await?;
        }
        Subcommands::Start { config, name } => {
            // Load config and potentially override service name
            let mut config = Config::from_env_and_file(config.as_ref()).await?;
            if let Some(name) = name {
                config.service_name = name;
                info!("Using custom service name: {}", config.service_name);
            }

            let cancel_token = CancellationToken::new();
            install_signal_handlers(cancel_token.clone())?;

            info!("Starting TransactionSender with config:\n{}", config);
            let tx_sender = TransactionSender::from_config(config).await?;
            tx_sender.start(cancel_token).await;
        }
    }
    Ok(())
}
