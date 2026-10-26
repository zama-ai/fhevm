use clap::Parser;
use coprocessor_proxy::core::{Config, Proxy};
use fhevm_engine_common::{healthz_server, metrics_server, telemetry};
use std::{process::ExitCode, sync::Arc};
use tokio_util::sync::CancellationToken;
use tracing::{Level, Metadata, error};

/// Where Pingora logs whole requests, signed `Authorization` included, and response bodies, at DEBUG
/// and TRACE.
const PINGORA_MESSAGE_TARGETS: [&str; 2] = ["pingora_proxy", "pingora_core::protocols::http"];

fn log_filter(metadata: &Metadata<'_>) -> bool {
    let pingora_message = *metadata.level() > Level::INFO
        && PINGORA_MESSAGE_TARGETS
            .iter()
            .any(|target| metadata.target().starts_with(target));
    telemetry::transport_dependency_filter(metadata) && !pingora_message
}

fn main() -> ExitCode {
    if let Err(err) = run() {
        error!("{err:#}");
        eprintln!("{err:#}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

// Unlike the other coprocessor services, the proxy does not run on `#[tokio::main]`: Pingora
// manages its own runtimes and blocks the main thread.
fn run() -> anyhow::Result<()> {
    fhevm_engine_common::handle_stack_version_flag();
    let config = Config::parse();

    // The OTLP exporter spawns on the entered runtime during setup. Keep it alive alongside
    // Pingora's runtimes until shutdown.
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()?;
    let _guard = runtime.enter();
    let _otel_guard = telemetry::init_tracing_otel_with_logs_only_fallback_and_filter(
        config.log_level,
        &config.service_name,
        "otlp-layer",
        log_filter,
    );

    // The monitoring servers run on the tokio runtime, next to Pingora's own.
    let healthcheck_port = config.healthcheck_port;
    let metrics_endpoint = config.metrics_endpoint.to_string();
    let (proxy, state) = Proxy::from_config(config)?;
    let cancel_token = CancellationToken::new();
    let health_server = healthz_server::HttpServer::new(
        Arc::new(state),
        healthcheck_port,
        cancel_token.child_token(),
    );
    let health_server_task = tokio::spawn(async move {
        if let Err(e) = health_server.start().await {
            error!(error = %e, "Health check server failed");
        }
    });
    let metrics_server_task =
        metrics_server::spawn(Some(metrics_endpoint), cancel_token.child_token());

    proxy.run()?;

    cancel_token.cancel();
    runtime.block_on(async {
        let _ = health_server_task.await;
        if let Some(task) = metrics_server_task {
            let _ = task.await;
        }
    });
    Ok(())
}
