use std::time::Duration;

use anyhow::ensure;
use clap::Parser;
use tokio_util::sync::CancellationToken;
use tracing::Level;

use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::utils::DatabaseURL;
use fhevm_engine_common::{metrics_server, telemetry};
use host_listener::cmd::DEFAULT_DEPENDENCE_CACHE_SIZE;
use host_listener::database::tfhe_event_propagate::Database;
use host_listener::solana_finalized_account_fetcher::{
    run_solana_finalized_account_fetcher, SolanaFinalizedAccountFetcherConfig,
};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        long = "url",
        alias = "rpc-url",
        help = "Solana HTTP JSON-RPC endpoint"
    )]
    url: String,

    #[arg(long, help = "PostgreSQL connection URL")]
    database_url: DatabaseURL,

    #[arg(
        long,
        default_value_t = 100,
        help = "Maximum queued finalized-account fetches to claim per batch"
    )]
    batch_size: i64,

    #[arg(
        long,
        default_value_t = 1000,
        help = "Sleep duration between empty queue polls in milliseconds"
    )]
    poll_interval_ms: u64,

    #[arg(
        long,
        default_value_t = 1000,
        help = "Backoff between retry attempts for RPC/DB failures in milliseconds"
    )]
    retry_interval_ms: u64,

    #[arg(
        long,
        help = "Address for Prometheus metrics HTTP server (e.g. 0.0.0.0:9100); if unset, metrics server is disabled"
    )]
    metrics_addr: Option<String>,

    #[arg(
        long,
        value_parser = clap::value_parser!(Level),
        default_value_t = Level::INFO
    )]
    log_level: Level,

    #[arg(long, default_value = "solana-finalized-account-fetcher")]
    service_name: String,

    #[arg(
        long,
        default_value_t = DEFAULT_DEPENDENCE_CACHE_SIZE,
        help = "Pre-computation dependence chain cache size"
    )]
    dependence_cache_size: u16,

    #[arg(
        long,
        default_value_t = 0,
        help = "Database chain id label used by the shared host-listener database type"
    )]
    chain_id: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    ensure!(args.batch_size > 0, "--batch-size must be positive");

    let _otel_guard = telemetry::init_tracing_otel_with_logs_only_fallback(
        args.log_level,
        &args.service_name,
        "otlp-layer",
    );

    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

    let cancel_token = CancellationToken::new();
    metrics_server::spawn(
        args.metrics_addr.clone(),
        cancel_token.child_token(),
    );

    let db = Database::new(
        &args.database_url,
        ChainId::try_from(args.chain_id)?,
        args.dependence_cache_size,
    )
    .await?;
    let config = SolanaFinalizedAccountFetcherConfig {
        rpc_url: args.url,
        batch_size: args.batch_size,
        poll_interval: Duration::from_millis(args.poll_interval_ms),
        retry_interval: Duration::from_millis(args.retry_interval_ms),
    };

    run_solana_finalized_account_fetcher(db, config).await
}
