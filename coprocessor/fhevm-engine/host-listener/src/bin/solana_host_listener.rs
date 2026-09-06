//! Solana host listener: reconstructs coprocessor work and the RFC 035 leaf record
//! from confirmed Yellowstone sealed blocks, ingests both into the shared database,
//! and serves leaf inclusion proofs over HTTP.

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
    database::{
        solana_leaves::load_checkpoint, tfhe_event_propagate::Database,
    },
    http_server::HttpServer,
    solana_grpc_listener::{
        run, BlockCheckpoint, SolanaGrpcListenerConfig, StartPosition,
    },
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

    /// Dependence-chain cache size.
    #[arg(long, default_value_t = DEFAULT_DEPENDENCE_CACHE_SIZE)]
    dependence_cache_size: u16,

    /// Port of the HTTP server: health routes and the leaf-proof route.
    #[arg(long, default_value_t = 8080)]
    http_port: u16,

    /// Bearer API key the leaf-proof route requires.
    #[arg(long, env = "SOLANA_PROOF_API_KEY")]
    proof_api_key: String,

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

    // The deployment's HostConfig is the one source of the chain id: it derives handles and
    // names the database partition they are filed under, so the two can never disagree.
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

    let db = Database::new(
        &args.database_url,
        ChainId::from_canonical_u64(host_config_chain_id),
        args.dependence_cache_size,
    )
    .await
    .context("connect coprocessor database")?;

    let pool = db.pool.read().await.clone();
    // Resume after the last block whose compute rows and leaves were committed; the
    // leaf record can only be continued from where it stopped.
    let start = match load_checkpoint(&pool).await.context("load checkpoint")? {
        Some(checkpoint) => {
            info!(
                slot = checkpoint.slot,
                "resuming after the recorded checkpoint"
            );
            StartPosition::Resume(BlockCheckpoint {
                slot: checkpoint.slot,
                block_hash: checkpoint.block_hash,
            })
        }
        None => StartPosition::Tip,
    };

    let cancel = CancellationToken::new();
    let signal_cancel = cancel.clone();
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            info!("Received ctrl-c, shutting down");
            signal_cancel.cancel();
        }
    });

    let http_server = HttpServer::new(
        pool,
        args.proof_api_key,
        args.http_port,
        cancel.clone(),
    );
    let http_cancel = cancel.clone();
    let http_task = tokio::spawn(async move {
        let result = http_server.start().await;
        // A dead proof route must not leave the listener running silently.
        http_cancel.cancel();
        result
    });

    let listener_result = run(
        &db,
        &SolanaGrpcListenerConfig {
            grpc_url: args.grpc_url,
            x_token: args.grpc_x_token,
            program_id: program_id.to_string(),
            chain_id: host_config_chain_id,
        },
        start,
        cancel.clone(),
    )
    .await;
    cancel.cancel();
    http_task.await.context("join HTTP server")??;
    listener_result
}
