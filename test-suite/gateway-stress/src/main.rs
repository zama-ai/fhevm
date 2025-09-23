mod app;
mod blockchain;
mod cli;
mod config;
mod db_manager;
mod decryption;

use crate::{
    app::App,
    cli::{Cli, Subcommands},
    config::Config,
};
use clap::Parser;
use std::process::ExitCode;
use tracing::{error, info};
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
    
    // Use default config path if none specified
    let config_path = cli.config.or_else(|| {
        std::path::PathBuf::from("config/config.toml").exists()
            .then_some(std::path::PathBuf::from("config/config.toml"))
    });
    
    let mut config = Config::from_env_and_file(config_path)?;

    // Override some fields of the config by the CLI
    if cli.sequential {
        config.sequential = cli.sequential;
    }
    if let Some(parallel) = cli.parallel {
        config.parallel_requests = Some(parallel);
    }

    info!("Config: {config:?}");
    
    match cli.subcommand {
        Subcommands::DbConnector(args) => {
            // For DB connector testing, we don't need to connect to the gateway
            App::db_connector_stress_test_standalone(config, args).await?
        }
        _ => {
            // For blockchain testing, we need gateway connection
            let app = App::connect(config).await?;
            match cli.subcommand {
                Subcommands::Public => app.public_decryption_stress_test().await?,
                Subcommands::User => app.user_decryption_stress_test().await?,
                Subcommands::Mixed => todo!("Mixed mode not implemented yet"),
                _ => unreachable!(),
            }
        }
    }

    Ok(())
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
