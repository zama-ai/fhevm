use anyhow::anyhow;
use clap::Parser;
use fhevm_engine_common::{
    database::{connect_pool_with_options, resolve_database_url_from_option},
    healthz_server::HttpServer,
    metrics_server, telemetry,
    utils::DatabaseURL,
};
use pad_worker::{Config, PadWorker};
use sqlx::postgres::PgPoolOptions;
use std::{num::NonZeroUsize, sync::Arc, time::Duration};
use tokio::signal::unix::{signal, SignalKind};
use tokio_util::sync::CancellationToken;
use tracing::{info, Level};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    /// The PRF instance of this lane.
    #[arg(long, env = "PRF_INSTANCE")]
    prf_instance: u16,

    /// Gateway key ID (`key_id_gw`) of the instance, in hex.
    #[arg(long, env = "PRF_KEY_ID", value_parser = parse_key_id)]
    prf_key_id: KeyId,

    /// Postgres database URL. Falls back to the DATABASE_URL env var.
    #[arg(long)]
    database_url: Option<DatabaseURL>,

    #[arg(long, env = "PAD_DATABASE_POOL_SIZE", default_value_t = 8)]
    database_pool_size: u32,

    /// Evaluations at the same time in this pod.
    #[arg(long, env = "PRF_MAX_CONCURRENT", default_value = "4")]
    max_concurrent: NonZeroUsize,

    /// A claim older than this is taken again. Set it above the longest evaluation.
    #[arg(long, env = "PAD_CLAIM_TIMEOUT", default_value = "5m", value_parser = humantime::parse_duration)]
    claim_timeout: Duration,

    /// Failed tries before a row fails for good.
    #[arg(long, env = "PAD_MAX_ATTEMPTS", default_value_t = 3)]
    max_attempts: u32,

    /// Interval of the read that finds a lost `pad_work_available`.
    #[arg(long, env = "PAD_POLL_INTERVAL", default_value = "1s", value_parser = humantime::parse_duration)]
    poll_interval: Duration,

    /// Pending rows older than this are not claimed. Use the prf-service value.
    #[arg(long, env = "PAD_PENDING_MAX_AGE", default_value = "10m", value_parser = humantime::parse_duration)]
    pending_max_age: Duration,

    /// HTTP server port for health checks
    #[arg(long, default_value_t = 8080)]
    health_check_port: u16,

    /// Prometheus metrics server address
    #[arg(long, default_value = "0.0.0.0:9100")]
    metrics_addr: Option<String>,

    #[arg(long, env = "OTEL_SERVICE_NAME", default_value = "pad-worker")]
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

/// A key ID, so that clap takes one value, not a list of bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
struct KeyId(Vec<u8>);

fn parse_key_id(s: &str) -> Result<KeyId, String> {
    let bytes = hex::decode(s.strip_prefix("0x").unwrap_or(s)).map_err(|e| e.to_string())?;
    if bytes.is_empty() {
        return Err("empty key ID".to_string());
    }
    Ok(KeyId(bytes))
}

fn install_signal_handlers(cancel_token: CancellationToken) -> anyhow::Result<()> {
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;
    tokio::spawn(async move {
        tokio::select! {
            _ = sigint.recv() => (),
            _ = sigterm.recv() => (),
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
        prf_instance = args.prf_instance,
        prf_key_id = hex::encode(&args.prf_key_id.0),
        database_pool_size = args.database_pool_size,
        max_concurrent = args.max_concurrent,
        claim_timeout = ?args.claim_timeout,
        max_attempts = args.max_attempts,
        poll_interval = ?args.poll_interval,
        pending_max_age = ?args.pending_max_age,
        health_check_port = args.health_check_port,
        metrics_addr = args.metrics_addr.as_deref().unwrap_or("none"),
        "Starting pad-worker"
    );

    let cancel_token = CancellationToken::new();
    install_signal_handlers(cancel_token.clone())?;

    let database_url = resolve_database_url_from_option(args.database_url)?;
    let (pool, _refresh) = connect_pool_with_options(
        &database_url,
        PgPoolOptions::new().max_connections(args.database_pool_size),
        Some(&cancel_token),
    )
    .await?;

    let worker = PadWorker::new(
        Config {
            prf_instance: args.prf_instance,
            key_id: args.prf_key_id.0,
            max_concurrent: args.max_concurrent,
            claim_timeout: args.claim_timeout,
            max_attempts: args.max_attempts,
            poll_interval: args.poll_interval,
            pending_max_age: args.pending_max_age,
        },
        pool,
        cancel_token.clone(),
    );

    // The health server runs until the process exits, so a stop does not look like a crash.
    let health_server = HttpServer::new(
        Arc::new(worker.health()),
        args.health_check_port,
        CancellationToken::new(),
    );
    let health_task = tokio::spawn(async move { health_server.start().await });
    metrics_server::spawn(args.metrics_addr, cancel_token.child_token());

    let result = tokio::select! {
        res = worker.run() => res,
        res = health_task => Err(anyhow!("Health-check server exited: {res:?}")),
    };
    telemetry::flush();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_settings() {
        let args = Args::try_parse_from([
            "pad-worker",
            "--prf-instance=1",
            "--prf-key-id=0xab01",
            "--claim-timeout=2m",
        ])
        .unwrap();
        assert_eq!(args.prf_instance, 1);
        assert_eq!(args.prf_key_id, KeyId(vec![0xab, 0x01]));
        assert_eq!(args.claim_timeout, Duration::from_secs(120));
        assert_eq!(args.pending_max_age, Duration::from_secs(600));
    }

    #[test]
    fn refuses_a_missing_or_bad_key_id() {
        assert!(Args::try_parse_from(["pad-worker", "--prf-instance=1"]).is_err());
        assert!(
            Args::try_parse_from(["pad-worker", "--prf-instance=1", "--prf-key-id=zz"]).is_err()
        );
        assert!(Args::try_parse_from(["pad-worker", "--prf-instance=1", "--prf-key-id="]).is_err());
    }
}
