#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

use std::process::ExitCode;

use relayer_http::{App, endpoint, logging, settings::Settings};
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
    let app = match App::new(&settings, shutdown.clone()) {
        Ok(app) => app,
        Err(e) => {
            error!(error = %e, "startup failed");
            return ExitCode::FAILURE;
        }
    };
    info!(
        name = %settings.name,
        endpoint = %settings.http.endpoint,
        nodes = settings.kms_aggregator.endpoints.len(),
        "relayer-http started"
    );

    // Serves until `shutdown` is cancelled, then drains the in-flight requests (bounded by call.timeout).
    let mut server = tokio::spawn(endpoint::serve(app));
    tokio::select! {
        result = &mut server => {
            error!(result = ?result, "server stopped unexpectedly");
            return ExitCode::FAILURE;
        }
        () = wait_for_signal() => {
            shutdown.cancel();
            if let Err(e) = server.await {
                error!(error = %e, "server task failed");
                return ExitCode::FAILURE;
            }
        }
    }
    info!("relayer-http stopped");
    ExitCode::SUCCESS
}

/// SIGINT (Ctrl-C) or SIGTERM (Kubernetes). A failure to install the handler is logged and treated as a signal,
/// so the process never runs without a way to stop.
async fn wait_for_signal() {
    let mut sigterm = match signal(SignalKind::terminate()) {
        Ok(sigterm) => sigterm,
        Err(e) => {
            error!(error = %e, "signal handler failed");
            return;
        }
    };
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {}
        _ = sigterm.recv() => {}
    }
}
