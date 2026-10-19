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
use humantime::parse_duration;
use sqlx::postgres::PgPoolOptions;
use tokio_util::sync::CancellationToken;
use tracing::Level;

use fhevm_engine_common::{
    database::{
        connect_pool_with_options_and_connect_options, with_statement_timeout,
    },
    metrics_server,
    shutdown_signal::cancel_on_shutdown_signal,
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
    /// Per replica: this bound is what protects the shared database, which sees
    /// at most replicas × (this − 1) proof reads at a time.
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
    /// tx-sender's new requests are refused. Per replica: it bounds this
    /// replica's memory. DD-067 gives the sizing.
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

    /// The canonical `ProtocolConfig`, whose Active KMS contexts' tx-senders may call.
    #[arg(long)]
    protocol_config_address: Address,

    /// How often the finalized block is polled; the KMS tx-senders are read again when it is newer
    /// than the last read. A request from a signer outside the set also starts a poll.
    #[arg(long, default_value = "5s", value_parser = parse_poll_interval)]
    kms_tx_sender_poll_interval: Duration,

    /// This coprocessor's registered signer address, the audience a request must be
    /// signed for (`FhevmSig`, RFC 038).
    #[arg(long)]
    coprocessor_signer_address: Address,

    #[arg(long, default_value_t = Level::INFO)]
    log_level: Level,

    #[arg(long, default_value = "solana-merkle-proof-server")]
    service_name: String,
}

/// A zero interval would poll the RPC without pause.
fn parse_poll_interval(s: &str) -> Result<Duration, String> {
    let interval = parse_duration(s).map_err(|err| err.to_string())?;
    if interval.is_zero() {
        return Err("must be more than zero".to_owned());
    }
    Ok(interval)
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

    cancel_on_shutdown_signal(cancel.clone())
        .context("install the SIGINT and SIGTERM handlers")?;

    metrics_server::spawn(args.metrics_addr, cancel.child_token());

    let ethereum = ProviderBuilder::new()
        .connect_http(args.ethereum_rpc_url)
        .erased();
    let senders = kms_tx_senders::follow(
        ProtocolConfig::new(args.protocol_config_address, ethereum),
        args.kms_tx_sender_poll_interval,
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
