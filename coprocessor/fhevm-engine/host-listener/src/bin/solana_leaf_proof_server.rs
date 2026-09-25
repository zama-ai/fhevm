//! Solana leaf-proof server: answers the KMS connector's leaf inclusion proofs from the leaf
//! record that `solana_host_listener` ingests. It runs as its own deployment with its own
//! database pool, so it keeps serving while ingestion is stopped or behind. A proof from a record
//! that is behind still verifies against the chain's peaks until a later append to the Store
//! merges that leaf's mountain.

use std::time::Duration;

use anyhow::{Context, Result};
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use tokio_util::sync::CancellationToken;
use tracing::{info, Level};

use fhevm_engine_common::{
    database::{
        connect_pool_with_options_and_connect_options, with_statement_timeout,
    },
    telemetry,
    utils::DatabaseURL,
};
use host_listener::http_server::HttpServer;

#[derive(Parser, Debug, Clone)]
#[command(version, about = "Solana leaf-proof server", long_about = None)]
struct Args {
    /// PostgreSQL connection string for the coprocessor database.
    #[arg(long)]
    database_url: DatabaseURL,

    /// Most connections the server's pool holds.
    #[arg(long, default_value_t = 8)]
    database_pool_size: u32,

    /// Port of the HTTP server: health routes and the leaf-proof route.
    #[arg(long, default_value_t = 8080)]
    http_port: u16,

    /// Bearer API key the leaf-proof route requires.
    #[arg(long, env = "SOLANA_PROOF_API_KEY")]
    proof_api_key: String,

    #[arg(long, default_value_t = Level::INFO)]
    log_level: Level,

    #[arg(long, default_value = "solana-leaf-proof-server")]
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

    let cancel = CancellationToken::new();
    let (pool, _refresh) = connect_pool_with_options_and_connect_options(
        &args.database_url,
        PgPoolOptions::new()
            .max_connections(args.database_pool_size)
            .acquire_timeout(Duration::from_secs(5)),
        Some(&cancel),
        // The KMS connector gives up on a proof request after its `host_rpc_call_timeout`,
        // 10 seconds by default, so a longer statement only holds a connection.
        |options| with_statement_timeout(options, Duration::from_secs(10)),
    )
    .await
    .context("connect coprocessor database")?;

    let signal_cancel = cancel.clone();
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            info!("Received ctrl-c, shutting down");
            signal_cancel.cancel();
        }
    });

    HttpServer::leaf_proofs(pool, args.proof_api_key, args.http_port, cancel)
        .start()
        .await
}
