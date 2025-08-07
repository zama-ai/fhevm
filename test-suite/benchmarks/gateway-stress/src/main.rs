mod app;
mod blockchain;
mod cli;
mod config;
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
    if let Err(e) = run().await {
        error!("{e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

async fn run() -> anyhow::Result<()> {
    init_tracing();
    let cli = Cli::parse();
    let config = Config::from_env_and_file(cli.config)?;
    info!("Config: {config:?}");

    let app = App::connect(config.clone()).await?;
    match cli.subcommand {
        Subcommands::Public => app.public_decryption_stress_test().await?,
        Subcommands::User => app.user().await?,
        _ => todo!(),
    }

    Ok(())
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();
}
