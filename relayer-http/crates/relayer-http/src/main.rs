#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

use std::process::ExitCode;

use relayer_http::{App, config::RelayerConfig, endpoint, logging, metrics};
use tokio::net::TcpListener;
use tokio::signal::unix::{SignalKind, signal};
use tokio::task::JoinError;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

#[tokio::main]
async fn main() -> ExitCode {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "config/config.yaml".to_owned());
    let config = match RelayerConfig::load(&path) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("invalid configuration: {e}");
            return ExitCode::FAILURE;
        }
    };
    logging::init(&config.log);
    logging::init_panic_hook();
    let shutdown = CancellationToken::new();
    let app = match App::new(&config, shutdown.clone()).await {
        Ok(app) => app,
        Err(e) => {
            error!(error = %e, "startup failed");
            return ExitCode::FAILURE;
        }
    };
    info!(
        name = %config.name,
        endpoint = %config.http.endpoint,
        metrics_endpoint = %config.metrics.endpoint,
        nodes = config.kms_aggregator.endpoints.len(),
        "relayer-http started"
    );

    // Serves `/metrics` until the API has drained, so the last requests are still scraped. Its failure stops the
    // process: a pod without metrics is blind to on-call.
    let metrics_stop = CancellationToken::new();
    let listener = match TcpListener::bind(config.metrics.endpoint).await {
        Ok(listener) => listener,
        Err(e) => {
            error!(server = "metrics server", error = %e, "server stopped unexpectedly");
            return ExitCode::FAILURE;
        }
    };
    let mut metrics_server = tokio::spawn(metrics::serve(
        listener,
        app.metrics.clone(),
        metrics_stop.clone(),
    ));

    // Serves until `shutdown` is cancelled, then drains the in-flight requests, which run to their own end
    // (bounded by http.body_read_timeout plus call.timeout).
    let mut server = tokio::spawn(endpoint::serve(app));
    tokio::select! {
        result = &mut server => {
            log_stopped("server", result);
            return ExitCode::FAILURE;
        }
        result = &mut metrics_server => {
            log_stopped("metrics server", result);
            return ExitCode::FAILURE;
        }
        () = wait_for_signal() => {
            shutdown.cancel();
            // Both layers: the task (panic, abort) and the server itself (bind or serve error).
            match server.await {
                Ok(Ok(())) => {}
                Ok(Err(e)) => {
                    error!(error = %e, "server failed");
                    return ExitCode::FAILURE;
                }
                Err(e) => {
                    error!(error = %e, "server task failed");
                    return ExitCode::FAILURE;
                }
            }
        }
    }
    metrics_stop.cancel();
    // Stopped by `metrics_stop`, it returns `Ok(())`; anything else is a failure of the process.
    match metrics_server.await {
        Ok(Ok(())) => {}
        Ok(Err(e)) => {
            error!(error = %e, "metrics server failed");
            return ExitCode::FAILURE;
        }
        Err(e) => {
            error!(server = "metrics server", error = %e, "server task failed");
            return ExitCode::FAILURE;
        }
    }
    info!("relayer-http stopped");
    ExitCode::SUCCESS
}

/// A server that ended before the shutdown: both layers, the server's own error and the task's (panic, abort).
fn log_stopped(name: &str, result: Result<std::io::Result<()>, JoinError>) {
    match result {
        Ok(Ok(())) => error!(server = name, "server stopped unexpectedly"),
        Ok(Err(e)) => error!(server = name, error = %e, "server stopped unexpectedly"),
        Err(e) => error!(server = name, error = %e, "server task failed"),
    }
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
