use endpoint::core::{Config, Endpoint};

use connector_utils::{
    cli::{Cli, Subcommands},
    config::DeserializeConfig,
    monitoring::otlp::init_otlp_setup,
    signal::install_signal_handlers,
    tasks::set_task_limit,
};
use std::process::ExitCode;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error};

#[tokio::main]
async fn main() -> ExitCode {
    if let Err(err) = run().await {
        error!("{err}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

async fn run() -> anyhow::Result<()> {
    let subcommand = Cli::new("Endpoint").parse();
    match subcommand {
        Subcommands::Validate { config } => {
            Config::from_env_and_file(Some(config))?;
        }
        Subcommands::Health { endpoint: _ } => {
            todo!("Endpoint healthcheck is not implemented yet")
        }
        Subcommands::Start { config } => {
            let config = Config::from_env_and_file(config.as_ref())?;
            debug!("{config:?}");
            init_otlp_setup(config.service_name.clone())?;

            let cancel_token = CancellationToken::new();
            set_task_limit(config.task_limit);
            install_signal_handlers(cancel_token.clone())?;

            let endpoint = Endpoint::from_config(config).await?;
            endpoint.start(cancel_token).await?;
        }
    }
    Ok(())
}
