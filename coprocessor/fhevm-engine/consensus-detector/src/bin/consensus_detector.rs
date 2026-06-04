use std::time::Duration;

use alloy::primitives::Address;
use alloy::providers::{ProviderBuilder, WsConnect};
use alloy::transports::http::reqwest::Url;
use clap::Parser;
use consensus_detector::Config;
use fhevm_engine_common::{
    database::{connect_pool_with_options, resolve_database_url_from_option},
    utils::DatabaseURL,
};
use humantime::parse_duration;
use sqlx::postgres::PgPoolOptions;
use tokio::signal::unix::{signal, SignalKind};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, Level};

#[derive(Parser, Debug, Clone)]
#[command(version, about = "Coprocessor unanimity consensus detector", long_about = None)]
struct Args {
    /// Service name (used by the logger and OTLP traces).
    #[arg(long, env = "OTEL_SERVICE_NAME", default_value = "consensus-detector")]
    service_name: String,

    /// Postgres database URL. Falls back to DATABASE_URL env var.
    #[arg(long)]
    database_url: Option<DatabaseURL>,

    /// Postgres pool size.
    #[arg(long, default_value_t = 4)]
    database_pool_size: u32,

    /// Gateway RPC URL (websocket).
    #[arg(long)]
    gw_url: Url,

    /// On-chain `GatewayConfig` contract address.
    #[arg(long)]
    gateway_config_address: Address,

    /// Provider reconnect attempts.
    #[arg(long, default_value_t = u32::MAX)]
    provider_max_retries: u32,

    /// Delay between provider reconnect attempts.
    #[arg(long, default_value = "4s", value_parser = parse_duration)]
    provider_retry_interval: Duration,

    /// Fallback poll interval used while waiting for notifications.
    #[arg(long, default_value_t = 30)]
    poll_interval_secs: u64,

    /// How often to call `fetch_state_commitments` while waiting for unanimity.
    #[arg(long, default_value = "5s", value_parser = parse_duration)]
    commitment_poll_interval: Duration,

    /// Hard cap on the unanimity poll before falling back to
    /// `unanimity_consensus_timeout`.
    #[arg(long, default_value = "60s", value_parser = parse_duration)]
    commitment_timeout: Duration,

    /// This operator's S3 bucket. Omit to disable GCS uploads.
    #[arg(long)]
    my_bucket: Option<String>,

    /// S3 endpoint override (e.g. `http://minio:9000`).
    #[arg(long)]
    s3_endpoint: Option<String>,

    /// Max pending blocks processed per state_hash sweep.
    #[arg(long, default_value_t = 256)]
    state_hash_batch_limit: i64,

    #[arg(
        long,
        value_parser = clap::value_parser!(Level),
        default_value_t = Level::INFO,
    )]
    log_level: Level,
}

fn install_signal_handlers(cancel: CancellationToken) -> anyhow::Result<()> {
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;
    tokio::spawn(async move {
        tokio::select! {
            _ = sigint.recv() => {}
            _ = sigterm.recv() => {}
        }
        cancel.cancel();
    });
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(args.log_level)
        .init();

    let config = Config {
        service_name: args.service_name.clone(),
        database_url: resolve_database_url_from_option(args.database_url.clone())?,
        database_pool_size: args.database_pool_size,
        gateway_config_address: args.gateway_config_address,
        log_level: args.log_level,
        poll_interval: Duration::from_secs(args.poll_interval_secs),
        commitment_poll_interval: args.commitment_poll_interval,
        commitment_timeout: args.commitment_timeout,
        my_bucket: args.my_bucket.clone(),
        s3_endpoint: args.s3_endpoint.clone(),
        state_hash_batch_limit: args.state_hash_batch_limit,
    };

    info!(
        service_name = %config.service_name,
        gateway_config_address = %config.gateway_config_address,
        gw_url = %args.gw_url,
        pool_size = config.database_pool_size,
        "consensus-detector starting"
    );

    let cancel = CancellationToken::new();
    install_signal_handlers(cancel.clone())?;

    let provider = loop {
        match ProviderBuilder::new()
            .connect_ws(
                WsConnect::new(args.gw_url.clone())
                    .with_max_retries(args.provider_max_retries)
                    .with_retry_interval(args.provider_retry_interval),
            )
            .await
        {
            Ok(p) => {
                info!(gw_url = %args.gw_url, "connected to Gateway");
                break p;
            }
            Err(e) => {
                error!(
                    gw_url = %args.gw_url,
                    error = %e,
                    retry_interval = ?args.provider_retry_interval,
                    "failed to connect to Gateway, retrying"
                );
                tokio::time::sleep(args.provider_retry_interval).await;
            }
        }
    };

    let (pool, _refresh) = connect_pool_with_options(
        &config.database_url,
        PgPoolOptions::new().max_connections(config.database_pool_size),
        Some(&cancel),
    )
    .await?;

    if let Err(e) = consensus_detector::run(config, pool, provider, cancel.clone()).await {
        error!(error = %e, "consensus-detector exited with error");
        return Err(e);
    }

    info!("consensus-detector stopped cleanly");
    Ok(())
}
