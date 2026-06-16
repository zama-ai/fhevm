//! Live Solana host listener: polls a validator for `zama-host` CPI events and
//! ingests them into the coprocessor database. Solana counterpart of the
//! `host_listener` binary, sharing decode/insert logic via `solana_adapter`.

use std::str::FromStr;
use std::time::Duration;

use anyhow::{Context, Result};
use clap::Parser;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use tokio_util::sync::CancellationToken;
use tracing::{info, Level};

use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::telemetry;
use fhevm_engine_common::utils::DatabaseURL;

use host_listener::cmd::DEFAULT_DEPENDENCE_CACHE_SIZE;
use host_listener::database::tfhe_event_propagate::Database;
use host_listener::solana_listener::{run, SolanaListenerConfig};

#[derive(Parser, Debug, Clone)]
#[command(version, about = "Solana host listener", long_about = None)]
struct Args {
    /// PostgreSQL connection string for the coprocessor database.
    #[arg(long)]
    database_url: DatabaseURL,

    /// Solana validator JSON-RPC endpoint.
    #[arg(long, default_value = "http://127.0.0.1:8899")]
    url: String,

    /// `zama-host` program id whose CPI events are ingested.
    #[arg(long = "program-id", alias = "acl-program-id")]
    program_id: String,

    /// Coprocessor host-chain id recorded against ingested handles.
    #[arg(long)]
    host_chain_id: i64,

    /// Delay between polls, in milliseconds.
    #[arg(long, default_value_t = 1_000)]
    poll_interval_ms: u64,

    /// Dependence-chain cache size.
    #[arg(long, default_value_t = DEFAULT_DEPENDENCE_CACHE_SIZE)]
    dependence_cache_size: u16,

    #[arg(long, default_value_t = Level::INFO)]
    log_level: Level,

    #[arg(long, default_value = "solana-host-listener")]
    service_name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let _otel_guard = telemetry::init_tracing_otel_with_logs_only_fallback(
        args.log_level,
        &args.service_name,
        "otlp-layer",
    );

    let program_id = Pubkey::from_str(&args.program_id)
        .with_context(|| format!("invalid program id {}", args.program_id))?;
    // A Solana host id carries the RFC-021 chain-type high bit and is passed as the
    // negative i64 bit pattern, which the strict ChainId::try_from rejects;
    // from_canonical_u64 accepts the full u64 host id for EVM and Solana alike.
    let chain_id = ChainId::from_canonical_u64(args.host_chain_id as u64);

    let db =
        Database::new(&args.database_url, chain_id, args.dependence_cache_size)
            .await
            .context("connect coprocessor database")?;

    let commitment = CommitmentConfig::confirmed();
    let rpc = RpcClient::new_with_commitment(args.url.clone(), commitment);

    let config = SolanaListenerConfig {
        rpc_url: args.url,
        program_id,
        poll_interval: Duration::from_millis(args.poll_interval_ms),
        commitment,
    };

    let cancel = CancellationToken::new();
    let signal_cancel = cancel.clone();
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            info!("Received ctrl-c, shutting down");
            signal_cancel.cancel();
        }
    });

    run(&db, &rpc, &config, cancel).await
}
