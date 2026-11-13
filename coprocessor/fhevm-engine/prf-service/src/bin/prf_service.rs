use anyhow::anyhow;
use clap::Parser;
use coprocessor_api::MAX_PAD_LENGTH;
use fhevm_engine_common::{
    database::{connect_pool_with_options, resolve_database_url_from_option},
    healthz_server::HttpServer,
    metrics_server, telemetry,
    utils::DatabaseURL,
};
use prf_service::{Config, PrfService};
use sqlx::postgres::PgPoolOptions;
use std::{
    net::SocketAddr,
    num::{NonZeroU32, NonZeroUsize},
    sync::Arc,
    time::Duration,
};
use tokio::signal::unix::{signal, SignalKind};
use tokio_util::sync::CancellationToken;
use tracing::{info, Level};

/// The longest pad. A smaller budget refuses it forever.
const MIN_PENDING_BITS: i64 = MAX_PAD_LENGTH as i64;
const MAX_ADDS_PER_CONNECTOR: NonZeroUsize = NonZeroUsize::new(2).unwrap();
const PAD_POLL_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    /// Internal HTTP endpoint. Only the proxy connects to it, over HTTP/1.1.
    #[arg(long, env = "PRF_HTTP_ENDPOINT", default_value = "0.0.0.0:8090")]
    http_endpoint: SocketAddr,

    /// Postgres database URL. Falls back to the DATABASE_URL env var.
    #[arg(long)]
    database_url: Option<DatabaseURL>,

    /// Postgres pool size: the listener, two admissions for each connector, and reads. The default is for 13 connectors.
    #[arg(long, env = "PRF_DATABASE_POOL_SIZE", default_value_t = 32, value_parser = clap::value_parser!(u32).range(4..))]
    database_pool_size: u32,

    /// Pad bits under evaluation that one connector can trigger, in all pods.
    #[arg(
        long,
        env = "PRF_MAX_CONCURRENT_PER_CONNECTOR",
        default_value_t = 16384,
        value_parser = clap::value_parser!(i64).range(MIN_PENDING_BITS..)
    )]
    max_concurrent_per_connector: i64,

    /// Requests in progress of one connector in this pod.
    #[arg(long, env = "PRF_MAX_IN_FLIGHT_PER_CONNECTOR", default_value = "256")]
    max_in_flight_per_connector: NonZeroUsize,

    /// Bytes of response bodies in memory, in this pod. When full, a request waits 5 s, then `503`.
    #[arg(long, env = "PRF_MAX_BODY_BYTES", default_value = "67108864")]
    max_body_bytes: NonZeroU32,

    /// Bytes of response bodies in memory for one connector, in this pod. Above it: `429`. One larger
    /// body is served alone.
    #[arg(
        long,
        env = "PRF_MAX_BODY_BYTES_PER_CONNECTOR",
        default_value = "16777216"
    )]
    max_body_bytes_per_connector: NonZeroU32,

    /// Time to serve after SIGTERM, closing each connection after its response, before the stop.
    #[arg(long, env = "PRF_SHUTDOWN_DELAY", default_value = "0s", value_parser = humantime::parse_duration)]
    shutdown_delay: Duration,

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

fn install_signal_handlers(
    cancel_token: CancellationToken,
    draining: CancellationToken,
    shutdown_delay: Duration,
) -> anyhow::Result<()> {
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;
    tokio::spawn(async move {
        tokio::select! {
            _ = sigint.recv() => (),
            _ = sigterm.recv() => {
                info!("SIGTERM received, closing connections after their next response for {shutdown_delay:?}");
                draining.cancel();
                tokio::time::sleep(shutdown_delay).await;
            }
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
    info!(
        http_endpoint = %args.http_endpoint,
        database_pool_size = args.database_pool_size,
        max_concurrent_per_connector = args.max_concurrent_per_connector,
        max_in_flight_per_connector = args.max_in_flight_per_connector,
        max_adds_per_connector = MAX_ADDS_PER_CONNECTOR,
        pad_poll_interval = ?PAD_POLL_INTERVAL,
        shutdown_delay = ?args.shutdown_delay,
        health_check_port = args.health_check_port,
        metrics_addr = args.metrics_addr.as_deref().unwrap_or("none"),
        "Starting prf-service"
    );

    let cancel_token = CancellationToken::new();
    let draining = CancellationToken::new();
    install_signal_handlers(cancel_token.clone(), draining.clone(), args.shutdown_delay)?;

    let database_url = resolve_database_url_from_option(args.database_url)?;
    let (pool, _refresh) = connect_pool_with_options(
        &database_url,
        PgPoolOptions::new().max_connections(args.database_pool_size),
        Some(&cancel_token),
    )
    .await?;

    let service = PrfService::new(
        Config {
            http_endpoint: args.http_endpoint,
            max_pending_bits: args.max_concurrent_per_connector,
            max_in_flight_per_connector: args.max_in_flight_per_connector,
            max_adds_per_connector: MAX_ADDS_PER_CONNECTOR,
            pad_poll_interval: PAD_POLL_INTERVAL,
            max_body_bytes: args.max_body_bytes,
            max_body_bytes_per_connector: args.max_body_bytes_per_connector,
        },
        pool,
        cancel_token.clone(),
        draining,
    )?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refuses_an_empty_body_budget() {
        assert!(Args::try_parse_from(["prf-service", "--max-body-bytes=0"]).is_err());
        assert!(Args::try_parse_from(["prf-service", "--max-body-bytes-per-connector=0"]).is_err());
    }
}
