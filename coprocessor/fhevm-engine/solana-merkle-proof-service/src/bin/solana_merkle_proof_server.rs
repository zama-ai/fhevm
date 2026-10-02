//! Solana Merkle proof server: answers the KMS connector's leaf inclusion proofs from the leaf
//! record that `solana_merkle_indexer` writes. It runs as its own deployment with its own
//! database pool, so it keeps serving while the indexer is stopped or behind. A proof from a
//! record that is behind still verifies against the chain's peaks until a later append to the
//! store merges that leaf's mountain. It answers only the KMS connectors that the canonical
//! `ProtocolConfig` lists, which it reads from an Ethereum RPC.

use std::{num::NonZeroU32, time::Duration};

use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    transports::http::reqwest::Url,
};
use anyhow::{Context, Result};
use clap::Parser;
use fhevm_host_bindings::protocol_config::ProtocolConfig;
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
use solana_merkle_proof_service::{kms_tx_senders, server::HttpServer};

#[derive(Parser, Debug, Clone)]
#[command(version, about = "Solana Merkle proof server", long_about = None)]
struct Args {
    /// PostgreSQL connection string for the Merkle record's database.
    #[arg(long)]
    database_url: DatabaseURL,

    /// Most connections the server's pool holds. All but one serve requests
    /// reading the record, one at a time each; `/healthz` keeps the last.
    #[arg(long, default_value_t = 8, value_parser = clap::value_parser!(u32).range(2..))]
    database_pool_size: u32,

    /// Queried leaves per second each KMS tx-sender may ask for, in bursts of as
    /// many. A backstop against a faulty or compromised connector.
    #[arg(long, default_value_t = NonZeroU32::new(4000).unwrap())]
    kms_tx_sender_leaves_per_second: NonZeroU32,

    /// MiB of signed requests and their answers the server remembers until the
    /// signatures expire; past it, new requests are refused. A request holds
    /// about 1.3 KiB with its answer, so 128 MiB is about 100,000 requests:
    /// 3,300 per second at the connector's 30 s validity, 1,700 at the 60 s
    /// maximum.
    #[arg(long, default_value_t = 128)]
    answer_cache_mib: usize,

    /// Port of the HTTP server: health routes and the Merkle proof route.
    #[arg(long, default_value_t = 8080)]
    http_port: u16,

    /// HTTP RPC of the canonical chain that hosts `ProtocolConfig`.
    #[arg(long)]
    ethereum_rpc_url: Url,

    /// The canonical `ProtocolConfig`, whose live KMS contexts' tx-senders may call.
    #[arg(long)]
    protocol_config_address: Address,

    #[arg(long, default_value_t = Level::INFO)]
    log_level: Level,

    #[arg(long, default_value = "solana-merkle-proof-server")]
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
        // The KMS connector gives up on a proof request after its `host_rpc_call_timeout`, so a
        // longer statement only holds a connection.
        |options| with_statement_timeout(options, Duration::from_secs(10)),
    )
    .await
    .context("connect Merkle record database")?;

    let signal_cancel = cancel.clone();
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            info!("Received ctrl-c, shutting down");
            signal_cancel.cancel();
        }
    });

    let ethereum = ProviderBuilder::new()
        .connect_http(args.ethereum_rpc_url)
        .erased();
    let senders = kms_tx_senders::follow(
        ProtocolConfig::new(args.protocol_config_address, ethereum),
        cancel.clone(),
    );

    HttpServer::merkle_proofs(
        pool,
        senders,
        args.kms_tx_sender_leaves_per_second,
        args.answer_cache_mib << 20,
        args.http_port,
        cancel,
    )
    .start()
    .await
}
