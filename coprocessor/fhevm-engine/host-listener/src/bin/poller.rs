use std::time::Duration;

use alloy::primitives::Address;
use clap::Parser;
use tokio_util::sync::CancellationToken;
use tracing::Level;

use fhevm_engine_common::metrics_server;
use fhevm_engine_common::utils::DatabaseURL;
use host_listener::cmd::{
    DEFAULT_DEPENDENCE_BY_CONNEXITY, DEFAULT_DEPENDENCE_CACHE_SIZE,
    DEFAULT_DEPENDENCE_CROSS_BLOCK,
};
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
        default_value_t = 6000, // half block time ~6s for Ethereum
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
        default_value_t = 45,
        help = "Maximum number of HTTP/RPC retry attempts (in addition to the initial attempt) before failing an operation"
    )]
    max_http_retries: u32,

    #[arg(
        long,
        default_value_t = 1000,
        help = "Rate limiting budget for RPC calls during block catchup (compute units per second). Higher values = less throttling"
    )]
    rpc_compute_units_per_second: u64,

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

    #[arg(
        long,
        default_value_t = DEFAULT_DEPENDENCE_CACHE_SIZE,
        help = "Pre-computation dependence chain cache size"
    )]
    pub dependence_cache_size: u16,

    #[arg(
        long,
        default_value_t = DEFAULT_DEPENDENCE_BY_CONNEXITY,
        help = "Dependence chain are connected components"
    )]
    pub dependence_by_connexity: bool,

    #[arg(
        long,
        default_value_t = DEFAULT_DEPENDENCE_CROSS_BLOCK,
        help = "Dependence chain are across blocks"
    )]
    pub dependence_cross_block: bool,

    #[arg(
        long,
        default_value_t = 0,
        help = "Max dependent ops per chain before slow-lane (0 disables; startup promotes all chains to fast)"
    )]
    pub dependent_ops_max_per_chain: u32,
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

    let config = PollerConfig {
        url: args.url,
        acl_address: args.acl_contract_address,
        tfhe_address: args.tfhe_contract_address,
        database_url: args.database_url,
        finality_lag: args.finality_lag,
        batch_size: args.batch_size,
        poll_interval: Duration::from_millis(args.poll_interval_ms),
        retry_interval: Duration::from_millis(args.retry_interval_ms),
        service_name: args.service_name,
        max_http_retries: args.max_http_retries,
        rpc_compute_units_per_second: args.rpc_compute_units_per_second,
        health_port: args.health_port,
        dependence_cache_size: args.dependence_cache_size,
        dependence_by_connexity: args.dependence_by_connexity,
        dependence_cross_block: args.dependence_cross_block,
        dependent_ops_max_per_chain: args.dependent_ops_max_per_chain,
    };

    run_poller(config).await
}
