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
    metrics_server, telemetry,
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
    /// Per replica: this bound is what protects the shared database, which sees
    /// at most replicas × (this − 1) proof reads.
    #[arg(long, default_value_t = 8, value_parser = clap::value_parser!(u32).range(2..))]
    database_pool_size: u32,

    /// Queried leaves per second each KMS tx-sender may ask for, in bursts of as
    /// many. A backstop against a faulty or compromised connector: a sustained
    /// load meets `--answer-cache-mib-per-kms-tx-sender` first.
    /// Per replica, so N replicas allow N times this. It keeps one tx-sender from
    /// taking this replica's reads from the others; `--database-pool-size`
    /// bounds the shared database.
    #[arg(long, default_value_t = NonZeroU32::new(4000).unwrap())]
    kms_tx_sender_leaves_per_second: NonZeroU32,

    /// MiB of each KMS tx-sender's signed requests and their answers the server
    /// remembers until it stops accepting the signatures; past it, that
    /// tx-sender's new requests are refused. A 64-leaf request with 20-hash paths
    /// holds under 48 KiB, so 16 MiB is about 22,000 leaves: 365 per second at the
    /// connector's 30 s validity plus the 30 s clock skew, and about 225 in 1-leaf
    /// requests. 13 KMS nodes in two live contexts hold at most 416 MiB.
    /// Per replica: it bounds this replica's memory.
    #[arg(long, default_value_t = 16)]
    answer_cache_mib_per_kms_tx_sender: usize,

    /// Port of the HTTP server: health routes and the Merkle proof route.
    #[arg(long, default_value_t = 8080)]
    http_port: u16,

    /// Address of the Prometheus metrics server (e.g. 0.0.0.0:9100); unset disables it.
    #[arg(long)]
    metrics_addr: Option<String>,

    /// HTTP RPC of the canonical chain that hosts `ProtocolConfig`.
    #[arg(long)]
    ethereum_rpc_url: Url,

    /// The canonical `ProtocolConfig`, whose live KMS contexts' tx-senders may call.
    #[arg(long)]
    protocol_config_address: Address,

    /// This coprocessor's registered signer address, the audience a request must be
    /// signed for (`FhevmSig`, RFC 038).
    #[arg(long)]
    coprocessor_signer_address: Address,

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

    metrics_server::spawn(args.metrics_addr, cancel.child_token());

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
        args.coprocessor_signer_address,
        args.kms_tx_sender_leaves_per_second,
        args.answer_cache_mib_per_kms_tx_sender << 20,
        args.http_port,
        cancel,
    )
    .start()
    .await
}
