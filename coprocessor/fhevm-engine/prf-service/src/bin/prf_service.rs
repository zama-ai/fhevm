use anyhow::anyhow;
use clap::Parser;
use fhevm_engine_common::{healthz_server::HttpServer, metrics_server, telemetry};
use prf_service::{Config, Health, PrfService};
use std::{net::SocketAddr, sync::Arc};
use tokio::signal::unix::{signal, SignalKind};
use tokio_util::sync::CancellationToken;
use tracing::Level;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    /// Internal HTTP endpoint. Only the proxy connects to it.
    #[arg(long, env = "PRF_HTTP_ENDPOINT", default_value = "0.0.0.0:8090")]
    http_endpoint: SocketAddr,

    /// HTTP server port for health checks
    #[arg(long, default_value_t = 8080)]
    health_check_port: u16,

    /// Prometheus metrics server address
    #[arg(long, default_value = "0.0.0.0:9100")]
    metrics_addr: Option<String>,

    #[arg(long, env = "OTEL_SERVICE_NAME", default_value = "prf-service")]
    service_name: String,

    #[arg(
        long,
        value_parser = clap::value_parser!(Level),
        default_value_t = Level::INFO)]
    log_level: Level,

    /// Print the compiled-in coprocessor stack version and exit.
    #[arg(long)]
    stack_version: bool,
}

fn install_signal_handlers(cancel_token: CancellationToken) -> anyhow::Result<()> {
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;
    tokio::spawn(async move {
        tokio::select! {
            _ = sigint.recv() => (),
            _ = sigterm.recv() => ()
        }
        cancel_token.cancel();
    });
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    fhevm_engine_common::handle_stack_version_flag();
    let args = Args::parse();

    let _otel_guard = telemetry::init_tracing_otel_with_logs_only_fallback(
        args.log_level,
        &args.service_name,
        "otlp-layer",
    );

    let cancel_token = CancellationToken::new();
    install_signal_handlers(cancel_token.clone())?;

    let service = PrfService::new(
        Config {
            http_endpoint: args.http_endpoint,
        },
        cancel_token.clone(),
    )?;

    // The health server runs until the process exits, so a stop does not look like a crash.
    let health_server = HttpServer::new(
        Arc::new(Health),
        args.health_check_port,
        CancellationToken::new(),
    );
    let health_task = tokio::spawn(async move { health_server.start().await });
    metrics_server::spawn(args.metrics_addr, cancel_token.child_token());

    let result = tokio::select! {
        res = service.start() => res,
        res = health_task => Err(anyhow!("Health-check server exited: {res:?}")),
    };
    telemetry::flush();
    result
}
