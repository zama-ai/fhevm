//! Solana Merkle indexer: rebuilds the leaf record of every encrypted store from confirmed
//! Yellowstone blocks into its own database. `solana_merkle_proof_server` serves the inclusion
//! proofs from that database.

use std::time::Duration;

use anyhow::{Context, Result};
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use tokio_util::sync::CancellationToken;
use tracing::{info, Level};

use fhevm_engine_common::{
    database::connect_pool_with_options, metrics_server, telemetry,
    utils::DatabaseURL,
};
use solana_host_follower::{
    block_checkpoint, run, track_confirmed_slot, Follower, FollowerArgs,
    StartPosition,
};
use solana_merkle_proof_service::{
    indexer::{IndexerStart, MerkleIndexerSink},
    server::HttpServer,
    store::load_checkpoint,
    store_check::run_store_checks,
    MIGRATOR,
};

#[derive(Parser, Debug, Clone)]
#[command(version, about = "Solana Merkle indexer", long_about = None)]
struct Args {
    /// PostgreSQL connection string for the Merkle record's own database, never the coprocessor's.
    #[arg(long)]
    database_url: DatabaseURL,

    #[command(flatten)]
    follower: FollowerArgs,

    /// Confirmed block to replay inclusively on an empty record: a slot before the first
    /// encrypted store was created, such as the zama-host deployment slot. Required on an empty
    /// record; a saved checkpoint wins.
    #[arg(long, env = "SOLANA_MERKLE_START_SLOT")]
    start_slot: Option<u64>,

    /// Port of the HTTP health routes.
    #[arg(long, default_value_t = 8080)]
    http_port: u16,

    /// Address of the Prometheus metrics server (e.g. 0.0.0.0:9100); unset disables it.
    #[arg(long)]
    metrics_addr: Option<String>,

    /// Seconds between two store checks, which compare every recorded store with its account
    /// on chain. The first runs at start.
    #[arg(long, default_value_t = 600)]
    store_check_interval_secs: u64,

    #[arg(long, default_value_t = Level::INFO)]
    log_level: Level,

    #[arg(long, default_value = "solana-merkle-indexer")]
    service_name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

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

    let Follower {
        config,
        live,
        archive,
    } = args.follower.connect().await?;

    let cancel = CancellationToken::new();
    let (pool, _refresh) = connect_pool_with_options(
        &args.database_url,
        PgPoolOptions::new()
            .max_connections(2)
            .acquire_timeout(Duration::from_secs(5)),
        Some(&cancel),
    )
    .await
    .context("connect Merkle record database")?;
    // Refuses a database that holds another schema's migrations, such as the coprocessor's.
    MIGRATOR
        .run(&pool)
        .await
        .context("migrate Merkle record database")?;

    let start = match IndexerStart::resolve(
        load_checkpoint(&pool).await.context("load checkpoint")?,
        args.start_slot,
    )? {
        IndexerStart::Resume(checkpoint) => {
            info!(
                slot = checkpoint.slot,
                "resuming after the recorded checkpoint"
            );
            StartPosition::Resume(checkpoint)
        }
        // Anchored to an actual block, so a provider silently starting at the tip is rejected.
        // The start slot is usually older than the live endpoint's ledger.
        IndexerStart::From(slot) => {
            info!(slot, "building the record from the start slot");
            StartPosition::ReplayFrom(block_checkpoint(&archive, slot).await?)
        }
    };

    let signal_cancel = cancel.clone();
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            info!("Received ctrl-c, shutting down");
            signal_cancel.cancel();
        }
    });

    tokio::spawn(run_store_checks(
        pool.clone(),
        args.follower.live_rpc(),
        config.program_id,
        config.chain_id,
        Duration::from_secs(args.store_check_interval_secs),
        cancel.child_token(),
    ));

    if args.metrics_addr.is_some() {
        metrics_server::spawn(args.metrics_addr, cancel.child_token());
        tokio::spawn(track_confirmed_slot(
            live,
            config.chain_id,
            cancel.child_token(),
        ));
    }

    let http_server =
        HttpServer::health(pool.clone(), args.http_port, cancel.clone());
    let http_cancel = cancel.clone();
    let http_task = tokio::spawn(async move {
        let result = http_server.start().await;
        // A dead health route must not leave the indexer running unobserved.
        http_cancel.cancel();
        result
    });

    let indexer_result = run(
        &MerkleIndexerSink::new(pool),
        &archive,
        &config,
        start,
        cancel.clone(),
    )
    .await;
    cancel.cancel();
    http_task.await.context("join HTTP server")??;
    indexer_result
}
