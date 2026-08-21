use std::{str::FromStr, sync::Arc, time::Duration};

use alloy::primitives::Address;
use alloy::providers::{ProviderBuilder, WsConnect};
use alloy::signers::{
    aws::{aws_sdk_kms, AwsSigner},
    local::PrivateKeySigner,
};
use alloy::transports::http::reqwest::Url;
use anyhow::Context;
use clap::Parser;
use consensus_detector::Config;
use fhevm_engine_common::{
    database::{
        apply_gcs_mode_search_path, connect_pool_with_options_and_connect_options,
        resolve_database_url_from_option,
    },
    types::{CoproSigner, SignerType},
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

    /// This operator's S3 bucket, or `none` to explicitly disable uploads.
    #[arg(long)]
    my_bucket: String,

    /// S3 endpoint override (e.g. `http://minio:9000`).
    #[arg(long)]
    s3_endpoint: Option<String>,

    /// Max pending blocks processed per state_hash pass.
    #[arg(long, default_value_t = 256)]
    state_hash_batch_limit: i64,

    /// Delay between local manifest publication retries.
    #[arg(long, default_value = "1m", value_parser = parse_duration)]
    consensus_publication_retry_delay: Duration,

    /// Additional publication attempts after the initial attempt, including
    /// transient S3 failures.
    #[arg(long, default_value_t = 30)]
    consensus_publication_retry_count: u32,

    /// Manifest signer implementation.
    #[arg(long, value_enum, default_value = "private-key")]
    signer_type: SignerType,

    /// Manifest signing key when `--signer-type private-key` is selected.
    #[arg(long)]
    private_key: Option<String>,

    #[arg(
        long,
        value_parser = clap::value_parser!(Level),
        default_value_t = Level::INFO,
    )]
    log_level: Level,

    /// Address for the Prometheus metrics server.
    #[arg(long)]
    metrics_addr: Option<String>,

    /// Manifest gauge refresh interval in seconds.
    #[arg(long, value_parser = clap::value_parser!(u32).range(1..))]
    gauge_update_interval_secs: Option<u32>,

    /// Print the compiled-in coprocessor stack version and exit.
    #[arg(long)]
    stack_version: bool,
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

    fhevm_engine_common::handle_stack_version_flag();
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(args.log_level)
        .init();

    let my_bucket = match args.my_bucket.as_str() {
        "none" => None,
        "" => anyhow::bail!("--my-bucket must name a bucket or be the literal `none`"),
        bucket => Some(bucket.to_owned()),
    };
    let manifest_signer: Option<CoproSigner> = match my_bucket.as_ref() {
        Some(_) => Some(match args.signer_type {
            SignerType::PrivateKey => {
                let private_key = args.private_key.as_deref().context(
                    "--private-key is required when manifest publication uses private-key signing",
                )?;
                Arc::new(PrivateKeySigner::from_str(private_key.trim())?)
            }
            SignerType::AwsKms => {
                let key_id = std::env::var("AWS_KEY_ID")
                    .context("AWS_KEY_ID is required when manifest publication uses AWS KMS")?;
                let aws_config =
                    aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
                let client = aws_sdk_kms::Client::new(&aws_config);
                Arc::new(AwsSigner::new(client, key_id, None).await?)
            }
        }),
        None => None,
    };

    let database_url = resolve_database_url_from_option(args.database_url.clone())?;
    let gcs_mode = fhevm_engine_common::versioning::resolve_gcs_mode(database_url.as_str()).await?;

    let config = Config {
        service_name: args.service_name.clone(),
        database_url,
        database_pool_size: args.database_pool_size,
        gcs_mode,
        gateway_config_address: args.gateway_config_address,
        log_level: args.log_level,
        metrics_addr: args.metrics_addr,
        gauge_update_interval_secs: args.gauge_update_interval_secs,
        poll_interval: Duration::from_secs(args.poll_interval_secs),
        commitment_poll_interval: args.commitment_poll_interval,
        commitment_timeout: args.commitment_timeout,
        my_bucket,
        s3_endpoint: args.s3_endpoint.clone(),
        state_hash_batch_limit: args.state_hash_batch_limit,
        manifest_consensus: consensus_detector::manifest_consensus::Config {
            publication_retry_delay: args.consensus_publication_retry_delay,
            publication_retry_count: args.consensus_publication_retry_count,
        },
        manifest_signer,
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

    // Each Blue/Green instance uses the schema selected from its compiled stack
    // version. Green resolves GCS copies first; Blue stays in public.
    let (pool, _refresh) = connect_pool_with_options_and_connect_options(
        &config.database_url,
        PgPoolOptions::new().max_connections(config.database_pool_size),
        Some(&cancel),
        apply_gcs_mode_search_path(config.gcs_mode),
    )
    .await?;

    if let Err(e) = consensus_detector::run(config, pool, provider, cancel.clone()).await {
        error!(error = %e, "consensus-detector exited with error");
        return Err(e);
    }

    info!("consensus-detector stopped cleanly");
    Ok(())
}
