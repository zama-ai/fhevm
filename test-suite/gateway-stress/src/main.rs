mod bench;
mod blockchain;
mod cli;
mod config;
mod db;
mod decryption;

use crate::{
    blockchain::GatewayTestManager,
    cli::{Cli, Subcommands},
    config::Config,
    db::manager::DatabaseTestManager,
};
use clap::Parser;
use std::process::ExitCode;
use tracing::{debug, error};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> ExitCode {
    init_tracing();
    if let Err(e) = run().await {
        error!("{e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut config = Config::from_env_and_file(&cli.config)?;
    update_config_from_cli(&mut config, &cli);

    debug!("Config: {config:?}");
    match cli.subcommand {
        Subcommands::Gw(args) => {
            let test_manager = GatewayTestManager::connect(config).await?;
            test_manager.stress_test(args).await?
        }
        Subcommands::BenchGw(args) => {
            let test_manager = GatewayTestManager::connect(config).await?;
            test_manager.decryption_benchmark(args).await?
        }
        Subcommands::Db(args) => {
            let test_manager = DatabaseTestManager::connect(config).await?;
            test_manager.stress_test(args).await?
        }
        Subcommands::BenchDb(args) => {
            let test_manager = DatabaseTestManager::connect(config).await?;
            test_manager.decryption_benchmark(args).await?
        }
    }

    Ok(())
}

fn update_config_from_cli(config: &mut Config, cli: &Cli) {
    if cli.sequential {
        config.sequential = cli.sequential;
    }
    if let Some(parallel) = cli.parallel {
        config.parallel_requests = parallel;
    }
    if let Some(duration) = cli.duration {
        config.tests_duration = duration;
    }
    if let Some(interval) = cli.interval {
        config.tests_interval = interval;
    }
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                EnvFilter::new(format!("none,{}=info", env!("CARGO_CRATE_NAME")))
            }),
        )
        .init();
}
