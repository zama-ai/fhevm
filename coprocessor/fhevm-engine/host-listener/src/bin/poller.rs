use std::time::Duration;

use alloy::primitives::Address;
use anyhow::Context;
use clap::Parser;
use sqlx::types::Uuid;
use tokio_util::sync::CancellationToken;
use tracing::Level;

use fhevm_engine_common::metrics_server;
use fhevm_engine_common::utils::DatabaseURL;
use host_listener::poller::{run_poller, PollerConfig};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        long = "url",
        alias = "rpc-url",
        help = "L1 node HTTP JSON-RPC endpoint (HTTP only; ws not supported)"
    )]
    url: String,

    #[arg(long, help = "ACL contract address to monitor")]
    acl_contract_address: Address,

    #[arg(long, help = "TFHE contract address to monitor")]
    tfhe_contract_address: Address,

    #[arg(long, help = "PostgreSQL connection URL")]
    database_url: DatabaseURL,

    #[arg(long, help = "Coprocessor API key")]
    coprocessor_api_key: Option<Uuid>,

    #[arg(
        long,
        default_value_t = 15,
        help = "Depth behind the head considered final (in blocks)"
    )]
    finality_lag: u64,

    #[arg(
        long,
        default_value_t = 100,
        help = "Maximum number of blocks to process per iteration"
    )]
    batch_size: u64,

    #[arg(
        long,
        default_value_t = 1000,
        help = "Sleep duration between iterations in milliseconds"
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
        default_value_t = 10,
        help = "Maximum number of HTTP/RPC retry attempts (in addition to the initial attempt) before failing an operation"
    )]
    max_http_retries: u64,

    #[arg(
        long,
        help = "Address for Prometheus metrics HTTP server (e.g. 0.0.0.0:9100); if unset, metrics server is disabled"
    )]
    metrics_addr: Option<String>,

    #[arg(long, default_value_t = 8080, help = "Health check port")]
    health_port: u16,

    #[arg(
        long,
        value_parser = clap::value_parser!(Level),
        default_value_t = Level::INFO
    )]
    log_level: Level,

    #[arg(long, default_value = "host-listener-poller")]
    service_name: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .json()
        .with_level(true)
        .with_max_level(args.log_level)
        .init();

    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

    let cancel_token = CancellationToken::new();
    metrics_server::spawn(
        args.metrics_addr.clone(),
        cancel_token.child_token(),
    );

    let coprocessor_api_key = args
        .coprocessor_api_key
        .context("A Coprocessor API key is required to access the database")?;

    let config = PollerConfig {
        url: args.url,
        acl_address: args.acl_contract_address,
        tfhe_address: args.tfhe_contract_address,
        database_url: args.database_url,
        coprocessor_api_key,
        finality_lag: args.finality_lag,
        batch_size: args.batch_size,
        poll_interval: Duration::from_millis(args.poll_interval_ms),
        retry_interval: Duration::from_millis(args.retry_interval_ms),
        service_name: args.service_name,
        max_http_retries: args.max_http_retries,
        health_port: args.health_port,
    };

    run_poller(config).await
}
