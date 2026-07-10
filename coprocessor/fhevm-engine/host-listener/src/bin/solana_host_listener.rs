//! Live Solana host listener: polls a validator for `zama-host` CPI events and
//! ingests them into the coprocessor database. Solana counterpart of the
//! `host_listener` binary, sharing decode/insert logic via `solana_adapter`.

use std::str::FromStr;
use std::time::Duration;

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
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

const SOLANA_RPC_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

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

    /// Event transport: `rpc` (poll getSignaturesForAddress) or `grpc`
    /// (subscribe to a Yellowstone gRPC endpoint). `grpc` requires building
    /// with `--features solana-grpc`.
    #[arg(long, value_enum, default_value_t = Transport::Rpc)]
    transport: Transport,

    /// Yellowstone gRPC endpoint (used when --transport=grpc).
    #[arg(long, default_value = "http://127.0.0.1:10000")]
    grpc_url: String,

    /// Optional `x-token` auth metadata for the gRPC endpoint.
    #[arg(long)]
    grpc_x_token: Option<String>,

    /// Ingest events REBUILT off-chain from instructions instead of emit-decoded
    /// events (the reconstruction swap). Handle-derivation params (chain_id and
    /// the zero-birth-entropy rule) are auto-detected from the on-chain HostConfig.
    /// Without it, ingestion stays purely emit-based (the Phase 1 transport).
    /// Requires `--features solana-grpc,solana-reconstruct`.
    #[arg(long, default_value_t = false)]
    reconstruct: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, ValueEnum)]
enum Transport {
    Rpc,
    Grpc,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Only stand up the OTLP exporter when a collector is actually configured.
    // With no collector (CI / the Solana e2e) a doomed exporter can stall the
    // ingest loop's runtime, so fall back to JSON-logs-only there.
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
    // A Solana host id carries the RFC-021 chain-type high bit and is passed as the
    // negative i64 bit pattern, which the strict ChainId::try_from rejects;
    // from_canonical_u64 accepts the full u64 host id for EVM and Solana alike.
    let chain_id = ChainId::from_canonical_u64(args.host_chain_id as u64);

    let db =
        Database::new(&args.database_url, chain_id, args.dependence_cache_size)
            .await
            .context("connect coprocessor database")?;

    let cancel = CancellationToken::new();
    let signal_cancel = cancel.clone();
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            info!("Received ctrl-c, shutting down");
            signal_cancel.cancel();
        }
    });

    match args.transport {
        Transport::Rpc => {
            let commitment = CommitmentConfig::confirmed();
            let rpc = RpcClient::new_with_timeout_and_commitment(
                args.url.clone(),
                SOLANA_RPC_REQUEST_TIMEOUT,
                commitment,
            );
            let config = SolanaListenerConfig {
                rpc_url: args.url,
                program_id,
                poll_interval: Duration::from_millis(args.poll_interval_ms),
                commitment,
            };
            run(&db, &rpc, &config, cancel).await
        }
        Transport::Grpc => run_grpc(&db, &args, program_id, cancel).await,
    }
}

#[cfg(feature = "solana-grpc")]
async fn run_grpc(
    db: &Database,
    args: &Args,
    program_id: Pubkey,
    cancel: CancellationToken,
) -> Result<()> {
    use host_listener::solana_grpc_listener::{
        run as grpc_run, SolanaGrpcListenerConfig,
    };
    use yellowstone_grpc_proto::prelude::CommitmentLevel;

    #[allow(unused_mut)]
    let mut config = SolanaGrpcListenerConfig {
        grpc_url: args.grpc_url.clone(),
        x_token: args.grpc_x_token.clone(),
        rpc_fallback_url: args.url.clone(),
        program_id: program_id.to_string(),
        commitment: CommitmentLevel::Finalized,
        // Auto-detected from the on-chain HostConfig below (reconstruct mode only).
        chain_id: 0,
        zero_birth_entropy: false,
        reconstruct: args.reconstruct,
    };

    // In reconstruct mode, source the handle-derivation params (chain_id and the
    // zero-birth-entropy rule) from the on-chain HostConfig instead of CLI flags.
    #[cfg(feature = "solana-reconstruct")]
    if args.reconstruct {
        use host_listener::solana_reconstruct::{
            parse_host_config, HOST_CONFIG_SEED,
        };
        // Match the gRPC subscription finality: the lineage tracker is only safe
        // when reconstruction is driven by finalized data.
        let rpc = RpcClient::new_with_timeout_and_commitment(
            args.url.clone(),
            SOLANA_RPC_REQUEST_TIMEOUT,
            CommitmentConfig::finalized(),
        );
        let (host_config_pda, _) =
            Pubkey::find_program_address(&[HOST_CONFIG_SEED], &program_id);
        let account = rpc
            .get_account(&host_config_pda)
            .await
            .with_context(|| format!("fetch HostConfig {host_config_pda}"))?;
        let (chain_id, zero_birth_entropy) = parse_host_config(&account.data)?;
        info!(
            %host_config_pda,
            chain_id,
            zero_birth_entropy,
            "auto-detected handle-derivation params from HostConfig"
        );
        config.chain_id = chain_id;
        config.zero_birth_entropy = zero_birth_entropy;
    }

    grpc_run(db, &config, cancel).await
}

#[cfg(not(feature = "solana-grpc"))]
async fn run_grpc(
    _db: &Database,
    _args: &Args,
    _program_id: Pubkey,
    _cancel: CancellationToken,
) -> Result<()> {
    anyhow::bail!(
        "--transport=grpc requires building host-listener with --features solana-grpc"
    )
}
