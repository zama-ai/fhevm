//! Solana host listener: reconstructs coprocessor work from confirmed Yellowstone
//! transactions and block metas and ingests it into the coprocessor database.

use std::{str::FromStr, sync::Arc};

use anyhow::{Context, Result};
use clap::Parser;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use sqlx::PgPool;
use tokio_util::sync::CancellationToken;
use tracing::{info, Level};

use fhevm_engine_common::{
    chain_id::ChainId,
    healthz_server::{
        default_get_version, HealthCheckService, HealthStatus, HttpServer,
        Version,
    },
    metrics_server, telemetry,
    utils::DatabaseURL,
};
use host_listener::{
    cmd::DEFAULT_DEPENDENCE_CACHE_SIZE,
    database::{
        solana_checkpoint::load_checkpoint, tfhe_event_propagate::Database,
    },
    solana_listener::{SolanaListenerConfig, SolanaListenerSink},
};
use solana_host_follower::{
    block_checkpoint, host::host_chain_id, run, track_confirmed_slot,
    FollowerConfig, StartPosition, SOLANA_RPC_REQUEST_TIMEOUT,
};

/// Healthy while the coprocessor database answers.
struct DatabaseHealth(PgPool);

impl HealthCheckService for DatabaseHealth {
    async fn health_check(&self) -> HealthStatus {
        let mut status = HealthStatus::default();
        status.set_db_connected(&self.0).await;
        status
    }

    async fn is_alive(&self) -> bool {
        true
    }

    fn get_version(&self) -> Version {
        default_get_version()
    }
}

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

    /// Solana JSON-RPC endpoint whose ledger history rebuilds, with `getBlock` and
    /// `getTransaction`, the slots Yellowstone can no longer replay, and reads the start block's
    /// hash. It may be another provider's. Defaults to `--url`.
    #[arg(long, env = "SOLANA_ARCHIVE_URL")]
    archive_url: Option<String>,

    /// Existing confirmed block to replay inclusively on an empty database. Must precede the host
    /// activity to reconstruct; if Yellowstone no longer retains it, the archive serves it.
    /// A saved checkpoint wins.
    #[arg(long)]
    start_slot: Option<u64>,

    /// Optional `x-token` auth metadata for the gRPC endpoint.
    #[arg(long, env = "SOLANA_GRPC_X_TOKEN")]
    grpc_x_token: Option<String>,

    /// `zama-host` program id whose instructions are reconstructed.
    #[arg(long = "program-id", alias = "acl-program-id")]
    program_id: String,

    /// Dependence-chain cache size.
    #[arg(long, default_value_t = DEFAULT_DEPENDENCE_CACHE_SIZE)]
    dependence_cache_size: u16,

    #[arg(
        long,
        default_value_t = 0,
        help = "Max dependent ops per chain before slow-lane (0 disables; startup promotes all chains to fast)"
    )]
    dependent_ops_max_per_chain: u32,

    /// Port of the HTTP health routes.
    #[arg(long, default_value_t = 8080)]
    http_port: u16,

    /// Address of the Prometheus metrics server (e.g. 0.0.0.0:9100); unset disables it.
    #[arg(long)]
    metrics_addr: Option<String>,

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
    let host_config_chain_id = host_chain_id(&rpc, &program_id).await?;
    let archive = RpcClient::new_with_timeout(
        args.archive_url.unwrap_or_else(|| args.url.clone()),
        SOLANA_RPC_REQUEST_TIMEOUT,
    );
    info!(
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

    if args.dependent_ops_max_per_chain == 0 {
        let promoted = db.promote_all_dep_chains_to_fast_priority().await?;
        if promoted > 0 {
            info!(
                count = promoted,
                "Slow-lane disabled: promoted all chains to fast on startup"
            );
        }
    }

    let pool = db.pool.read().await.clone();
    // Resume after the last block whose compute rows were committed.
    let start = match load_checkpoint(&pool).await.context("load checkpoint")? {
        Some(checkpoint) => {
            info!(
                slot = checkpoint.slot,
                "resuming after the recorded checkpoint"
            );
            StartPosition::Resume(checkpoint)
        }
        None => match args.start_slot {
            // Anchored to an actual block, so a provider silently starting at the tip is
            // rejected. The archive reads the block's hash; its transactions come from the
            // stream, or from the archive once the stream no longer retains them.
            Some(slot) => StartPosition::ReplayFrom(
                block_checkpoint(&archive, slot).await?,
            ),
            None => StartPosition::Tip,
        },
    };

    let cancel = CancellationToken::new();
    let signal_cancel = cancel.clone();
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            info!("Received ctrl-c, shutting down");
            signal_cancel.cancel();
        }
    });

    if args.metrics_addr.is_some() {
        metrics_server::spawn(args.metrics_addr, cancel.child_token());
        tokio::spawn(track_confirmed_slot(
            rpc,
            host_config_chain_id,
            cancel.child_token(),
        ));
    }

    let http_server = HttpServer::new(
        Arc::new(DatabaseHealth(pool)),
        args.http_port,
        cancel.clone(),
    );
    let http_cancel = cancel.clone();
    let http_task = tokio::spawn(async move {
        let result = http_server.start().await;
        // A dead health route must not leave the listener running unobserved.
        http_cancel.cancel();
        result
    });

    let sink = SolanaListenerSink::new(
        &db,
        SolanaListenerConfig {
            program_id,
            chain_id: host_config_chain_id,
            dependent_ops_max_per_chain: args.dependent_ops_max_per_chain,
        },
    );
    let listener_result = run(
        &sink,
        &archive,
        &FollowerConfig {
            grpc_url: args.grpc_url,
            x_token: args.grpc_x_token,
            program_id,
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
