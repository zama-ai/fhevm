use std::process::ExitCode;

use kms_worker::core::{Config, KmsWorker};

use connector_utils::{
    cli::{Cli, Subcommands},
    signal::install_signal_handlers,
};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> ExitCode {
    if let Err(err) = run().await {
        error!("{err}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

async fn run() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let subcommand = Cli::new("KmsWorker").parse();
    match subcommand {
        Subcommands::Validate { config } => {
            Config::from_env_and_file(Some(config))?;
        }
        Subcommands::Start { config, name } => {
            // Load config and potentially override service name
            let mut config = Config::from_env_and_file(config.as_ref())?;
            if let Some(name) = name {
                config.service_name = name;
                info!("Using custom service name: {}", config.service_name);
            }

            let cancel_token = CancellationToken::new();
            install_signal_handlers(cancel_token.clone())?;

            info!("Starting KmsWorker with config:\n{}", config);
            let kms_worker = KmsWorker::from_config(config).await?;
            kms_worker.start(cancel_token).await;
        }
    }
    Ok(())
}
