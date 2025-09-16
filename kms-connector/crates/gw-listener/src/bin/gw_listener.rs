use gw_listener::{
    core::{Config, GatewayListener},
    monitoring::health::HealthStatus,
};

use connector_utils::{
    cli::{Cli, Subcommands},
    monitoring::{
        health::query_healthcheck_endpoint, otlp::init_otlp_setup, server::start_monitoring_server,
    },
    signal::install_signal_handlers,
    tasks::set_task_limit,
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
    let subcommand = Cli::new("GatewayListener").parse();
    match subcommand {
        Subcommands::Validate { config } => {
            Config::from_env_and_file(Some(config))?;
        }
        Subcommands::Health { endpoint } => {
            query_healthcheck_endpoint::<HealthStatus>(endpoint).await?;
        }
        Subcommands::Start { config } => {
            let config = Config::from_env_and_file(config.as_ref())?;
            init_otlp_setup(config.service_name.clone())?;

            let cancel_token = CancellationToken::new();
            set_task_limit(config.task_limit);
            install_signal_handlers(cancel_token.clone())?;
            let monitoring_endpoint = config.monitoring_endpoint;

            info!("Starting GatewayListener with config: {:?}", config);
            let (gw_listener, state) = GatewayListener::from_config(config).await?;
            start_monitoring_server(monitoring_endpoint, state, cancel_token.clone());
            gw_listener.start(cancel_token).await
        }
    }
    Ok(())
}
