//! Relayer Binary
//!
//! Binary entry point for the relayer that handles CLI parsing and tracing initialization.

use anyhow::Context;
use clap::Parser;
use fhevm_relayer::config::settings::Settings;
use fhevm_relayer::tracing::init_tracing;
use tokio_util::sync::CancellationToken;
use tracing::info;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    config_file: Option<String>,
}

/// Main entry point for the Relayer service.
///
/// This function performs the following initialization steps:
/// 1. Parses command line arguments
/// 2. Loads and validates configuration
/// 3. Initializes logging
/// 4. Delegates to the library function
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();

    // === Initialize settings for tracing setup
    let settings =
        Settings::new(args.config_file.clone()).context("Failed to load configuration")?;

    // We need to keep the guard to force-flush on SIGINT
    let chrome_tracing_guard = init_tracing(&settings.log)?;

    // Create cancellation token and handle Ctrl+C
    let cancellation_token = CancellationToken::new();
    let cancel_on_signal = cancellation_token.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for Ctrl+C");
        info!("Received Ctrl+C signal, initiating shutdown...");
        cancel_on_signal.cancel();
    });

    let result = fhevm_relayer::run_fhevm_relayer(settings, cancellation_token, None).await;

    // Clean up tracing
    if let Some(guard) = chrome_tracing_guard {
        guard.flush();
        drop(guard);
    }

    result
}
