//! Solana host listener: reconstructs coprocessor work from confirmed Yellowstone
//! sealed blocks and ingests it into the shared database.

use std::{str::FromStr, time::Duration};

use anyhow::{Context, Result};
use clap::Parser;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use tokio_util::sync::CancellationToken;
use tracing::{info, Level};

use fhevm_engine_common::{chain_id::ChainId, telemetry, utils::DatabaseURL};
use host_listener::{
    cmd::DEFAULT_DEPENDENCE_CACHE_SIZE,
    database::tfhe_event_propagate::Database,
    solana_grpc_listener::{run, SolanaGrpcListenerConfig, StartPosition},
    solana_reconstruct::{parse_host_config, HOST_CONFIG_SEED},
};

const SOLANA_RPC_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Parser, Debug, Clone)]
#[command(version, about = "Solana host listener", long_about = None)]
struct Args {
    /// PostgreSQL connection string for the coprocessor database.
    #[arg(long)]
    database_url: DatabaseURL,

    /// Solana JSON-RPC endpoint used for HostConfig and block-time reads.
    #[arg(long, default_value = "http://127.0.0.1:8899")]
    url: String,

    /// Yellowstone gRPC endpoint.
    #[arg(long, default_value = "http://127.0.0.1:10000")]
    grpc_url: String,

    /// Optional `x-token` auth metadata for the gRPC endpoint.
    #[arg(long)]
    grpc_x_token: Option<String>,

    /// `zama-host` program id whose instructions are reconstructed.
    #[arg(long = "program-id", alias = "acl-program-id")]
    program_id: String,

    /// Coprocessor host-chain id recorded against ingested handles.
    #[arg(long)]
    host_chain_id: i64,

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

    // Without a configured collector, constructing an OTLP exporter can stall the PoC ingest loop.
    let _otel_guard = if std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").is_ok() {
        telemetry::init_tracing_otel_with_logs_only_fallback(
            args.log_level,
            &args.service_name,
            "otlp-layer",
        )
    } else {
        let _ = telemetry::init_logs_only(args.log_level);
        None
    };

    let program_id = Pubkey::from_str(&args.program_id)
        .with_context(|| format!("invalid program id {}", args.program_id))?;
    // Solana host IDs set the RFC-021 high bit and arrive through CLI as the equivalent signed i64;
    // canonical-u64 conversion intentionally preserves that bit pattern.
    let database_chain_id =
        ChainId::from_canonical_u64(args.host_chain_id as u64);
    let db = Database::new(
        &args.database_url,
        database_chain_id,
        args.dependence_cache_size,
    )
    .await
    .context("connect coprocessor database")?;

    let rpc = RpcClient::new_with_timeout_and_commitment(
        args.url.clone(),
        SOLANA_RPC_REQUEST_TIMEOUT,
        CommitmentConfig::confirmed(),
    );
    let (host_config_pda, _) =
        Pubkey::find_program_address(&[HOST_CONFIG_SEED], &program_id);
    let account = rpc
        .get_account(&host_config_pda)
        .await
        .with_context(|| format!("fetch HostConfig {host_config_pda}"))?;
    let host_config_chain_id = parse_host_config(&account.data)?;
    info!(
        %host_config_pda,
        chain_id = host_config_chain_id,
        "auto-detected handle-derivation params from confirmed HostConfig"
    );

    let cancel = CancellationToken::new();
    let signal_cancel = cancel.clone();
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            info!("Received ctrl-c, shutting down");
            signal_cancel.cancel();
        }
    });

    run(
        &db,
        &SolanaGrpcListenerConfig {
            grpc_url: args.grpc_url,
            x_token: args.grpc_x_token,
            program_id: program_id.to_string(),
            chain_id: host_config_chain_id,
        },
        StartPosition::Tip,
        cancel,
    )
    .await
}
