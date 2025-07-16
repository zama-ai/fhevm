use tx_sender::core::{Config, TransactionSender};

use connector_utils::{
    cli::{Cli, Subcommands},
    otlp::init_otlp_setup,
    signal::install_signal_handlers,
};
use std::process::ExitCode;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

#[tokio::main]
async fn main() -> ExitCode {
    if let Err(err) = run().await {
        error!("{err}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

async fn run() -> anyhow::Result<()> {
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
            init_otlp_setup(
                config.service_name.clone(),
                config.metrics_endpoint,
                cancel_token.clone(),
            )?;

            info!("Starting TransactionSender with config: {:?}", config);
            let tx_sender = TransactionSender::from_config(config).await?;
            tx_sender.start(cancel_token).await;
        }
    }
    Ok(())
}
