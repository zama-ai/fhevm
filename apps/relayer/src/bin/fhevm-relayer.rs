//! fhevm Relayer Binary
//!
//! Binary entry point for the fhevm-relayer that handles CLI parsing and tracing initialization.

use clap::Parser;
use fhevm_relayer::config::settings::{LogConfig, Settings};
use tokio_util::sync::CancellationToken;
use tracing::info;
#[cfg(feature = "tracing-chrome")]
use tracing_chrome::{ChromeLayerBuilder, FlushGuard};
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

#[cfg(not(feature = "tracing-chrome"))]
struct FlushGuard {}

#[cfg(not(feature = "tracing-chrome"))]
impl FlushGuard {
    fn flush(&self) {}
}

#[cfg(not(feature = "tracing-chrome"))]
impl Drop for FlushGuard {
    fn drop(&mut self) {}
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    config_file: Option<String>,
}

/// Main entry point for the FHE Event Relayer service.
///
/// This function performs the following initialization steps:
/// 1. Parses command line arguments
/// 2. Loads and validates configuration
/// 3. Initializes logging
/// 4. Delegates to the library function
#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args: Args = Args::parse();

    // === Initialize settings for tracing setup
    let settings = Settings::new(args.config_file.clone())
        .map_err(|e| eyre::eyre!("Failed to load configuration: {}", e))?;

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

    let result = fhevm_relayer::run_fhevm_relayer(settings, cancellation_token).await;

    // Clean up tracing
    if let Some(guard) = chrome_tracing_guard {
        guard.flush();
        drop(guard);
    }

    result
}

/// Initialize tracing based on configuration settings.
///
/// # Arguments
/// * `log_config` - The [`LogConfig`] containing logging preferences
///
/// # Returns
/// * `Ok(())` - If logging was successfully initialized
/// * `Err(`[`eyre::Error`]`)` - If initialization failed
///
/// # Configuration Options
/// - Log level (trace, debug, info, warn, error)
/// - Log format (compact, pretty, json)
/// - File and line number display
/// - Thread ID display
fn init_tracing(log_config: &LogConfig) -> eyre::Result<Option<FlushGuard>> {
    // Env filter allows for more control on per-crate log-level
    let env_filter = EnvFilter::from_default_env();

    // TODO: hide this behing a tracing-chrome feature
    // Build subscriber with common settings

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_file(log_config.show_file_line)
        .with_line_number(log_config.show_file_line)
        .with_thread_ids(log_config.show_thread_ids)
        .with_target(false);

    let tracing_subscriber_builder = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer);

    let optional_chrome_guard: Option<FlushGuard>;
    #[cfg(feature = "tracing-chrome")]
    {
        // The TODO is now addressed: this block is conditional.
        let (chrome_layer, chrome_tracing_guard) = ChromeLayerBuilder::new()
            .trace_style(tracing_chrome::TraceStyle::Async)
            .build();
        tracing_subscriber_builder.with(chrome_layer).init(); // Initialize with the Chrome layer
        optional_chrome_guard = Some(chrome_tracing_guard);
    }
    #[cfg(not(feature = "tracing-chrome"))]
    {
        tracing_subscriber_builder.init(); // Initialize without the Chrome layer
        optional_chrome_guard = None;
    }

    info!(
        format = ?log_config.format,
        show_file_line = ?log_config.show_file_line,
        show_thread_ids = ?log_config.show_thread_ids,
        "Tracing initialized successfully"
    );

    Ok(optional_chrome_guard)
}
