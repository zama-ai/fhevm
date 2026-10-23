use anyhow::anyhow;
use clap::Parser;
use fhevm_engine_common::{healthz_server::HttpServer, metrics_server, telemetry};
use alloy::primitives::Address;
use prf_service::{AuthConfig, Config, PrfService};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::signal::unix::{signal, SignalKind};
use tokio_util::sync::CancellationToken;
use tracing::Level;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    /// Internal HTTP endpoint. Only the proxy connects to it, over HTTP/1.1.
    #[arg(long, env = "PRF_HTTP_ENDPOINT", default_value = "0.0.0.0:8090")]
    http_endpoint: SocketAddr,

    /// Connection string of the `prf_service` role, which can only add rows to `pads` and read them.
    #[arg(long, env = "DATABASE_URL")]
    database_url: String,

    #[arg(long, default_value_t = 16)]
    database_pool_size: u32,

    /// RFC-038 `PRF_MAX_CONCURRENT_PER_CONNECTOR`, in pad bits.
    #[arg(long, env = "PRF_MAX_CONCURRENT_PER_CONNECTOR", default_value_t = 65536)]
    max_pending_bits_per_connector: i64,

    #[arg(long, default_value_t = 4)]
    max_adds_per_connector: usize,

    #[arg(long, value_parser = humantime::parse_duration, default_value = "1s")]
    pad_poll_interval: Duration,

    #[arg(long, value_parser = humantime::parse_duration, default_value = "1s")]
    retry_after: Duration,

    /// The `FhevmSig` domain and audience, the same as the proxy's.
    #[arg(long, env = "PRF_ETHEREUM_CHAIN_ID")]
    ethereum_chain_id: u64,

    #[arg(long, env = "PRF_PROTOCOL_CONFIG_ADDRESS")]
    protocol_config_address: Address,

    #[arg(long, env = "PRF_COPROCESSOR_SIGNER_ADDRESS")]
    coprocessor_signer_address: Address,

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
            database_url: args.database_url,
            database_pool_size: args.database_pool_size,
            max_pending_bits_per_connector: args.max_pending_bits_per_connector,
            max_adds_per_connector: args.max_adds_per_connector,
            pad_poll_interval: args.pad_poll_interval,
            retry_after: args.retry_after,
            auth: AuthConfig {
                ethereum_chain_id: args.ethereum_chain_id,
                protocol_config_address: args.protocol_config_address,
                coprocessor_signer_address: args.coprocessor_signer_address,
            },
        },
        cancel_token.clone(),
    )
    .await?;

    // The health server runs until the process exits, so a stop does not look like a crash.
    let health_server = HttpServer::new(
        Arc::new(service.health()),
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
