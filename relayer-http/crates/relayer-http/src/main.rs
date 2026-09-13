#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

use std::process::ExitCode;

use relayer_http::{App, logging, settings::Settings};
use tokio::signal::unix::{SignalKind, signal};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

#[tokio::main]
async fn main() -> ExitCode {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config/config.yaml".to_owned());
    let settings = match Settings::load(&path) {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!("invalid configuration: {e}");
            return ExitCode::FAILURE;
        }
    };
    logging::init(&settings.log);
    let shutdown = CancellationToken::new();
    let _app = match App::new(&settings, shutdown.clone()) {
        Ok(app) => app,
        Err(e) => {
            error!(error = %e, "startup failed");
            return ExitCode::FAILURE;
        }
    };
    info!(
        name = %settings.name,
        nodes = settings.kms_aggregator.endpoints.len(),
        "relayer-http started"
    );

    // HTTP handlers come in the next iteration; until then the process only waits for SIGINT or SIGTERM.
    let mut sigterm = match signal(SignalKind::terminate()) {
        Ok(sigterm) => sigterm,
        Err(e) => {
            error!(error = %e, "signal handler failed");
            return ExitCode::FAILURE;
        }
    };
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {}
        _ = sigterm.recv() => {}
    }
    // Every in-flight aggregation ends with `AggregationError::Cancelled`.
    shutdown.cancel();
    info!("relayer-http stopped");
    ExitCode::SUCCESS
}
