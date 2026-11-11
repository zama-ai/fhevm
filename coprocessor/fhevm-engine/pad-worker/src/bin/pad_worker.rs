use alloy::signers::{
    aws::{aws_sdk_kms, AwsSigner},
    local::PrivateKeySigner,
};
use anyhow::{anyhow, Context};
use aws_config::{timeout::TimeoutConfig, BehaviorVersion};
use clap::Parser;
use fhevm_engine_common::{
    database::{connect_pool_with_options, resolve_database_url_from_option},
    healthz_server::HttpServer,
    metrics_server, telemetry,
    types::{CoproSigner, SignerType},
    utils::DatabaseURL,
};
use pad_worker::{evaluator::MockEvaluator, upload::Uploader, Config, PadWorker};
use sqlx::postgres::PgPoolOptions;
use std::{num::NonZeroUsize, str::FromStr, sync::Arc, time::Duration};
use tokio::signal::unix::{signal, SignalKind};
use tokio_util::sync::CancellationToken;
use tracing::{info, warn, Level};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    /// The PRF instance of this lane.
    #[arg(long, env = "PRF_INSTANCE")]
    prf_instance: u16,

    /// Gateway key ID (`key_id_gw`) of the instance, in hex.
    #[arg(long, env = "PRF_KEY_ID", value_parser = parse_key_id)]
    prf_key_id: KeyId,

    /// Postgres database URL. Falls back to the DATABASE_URL env var.
    #[arg(long)]
    database_url: Option<DatabaseURL>,

    #[arg(long, env = "PAD_DATABASE_POOL_SIZE", default_value_t = 8)]
    database_pool_size: u32,

    /// Evaluations at the same time in this pod.
    #[arg(long, env = "PRF_MAX_CONCURRENT", default_value = "4")]
    max_concurrent: NonZeroUsize,

    /// A claim older than this is taken again. Set it above the longest evaluation.
    #[arg(long, env = "PAD_CLAIM_TIMEOUT", default_value = "5m", value_parser = humantime::parse_duration)]
    claim_timeout: Duration,

    /// Interval of the read that finds a lost `pad_work_available`.
    #[arg(long, env = "PAD_POLL_INTERVAL", default_value = "1s", value_parser = humantime::parse_duration)]
    poll_interval: Duration,

    /// Pending rows older than this are not claimed. Use the prf-service value.
    #[arg(long, env = "PAD_PENDING_MAX_AGE", default_value = "10m", value_parser = humantime::parse_duration)]
    pending_max_age: Duration,

    /// Delete uploaded or failed rows older than this. If not set, keep all rows.
    #[arg(long, env = "PRF_OUTPUT_MAX_AGE", value_parser = humantime::parse_duration)]
    output_max_age: Option<Duration>,

    /// Interval of the GC and the row age metrics.
    #[arg(long, env = "PAD_GC_INTERVAL", default_value = "1m", value_parser = humantime::parse_duration)]
    gc_interval: Duration,

    /// Interval of the reference digest read until the lane is ready.
    #[arg(long, env = "PAD_START_CHECK_INTERVAL", default_value = "30s", value_parser = humantime::parse_duration)]
    start_check_interval: Duration,

    /// Ciphertext bucket of the operator, for the S3 copy of the outputs.
    #[arg(long, env = "PAD_S3_BUCKET")]
    s3_bucket: String,

    #[arg(long, env = "PAD_S3_MAX_CONCURRENT_UPLOADS", default_value = "16")]
    s3_max_concurrent_uploads: NonZeroUsize,

    /// Interval of the read that finds failed uploads.
    #[arg(long, env = "PAD_S3_RESUBMIT_INTERVAL", default_value = "30s", value_parser = humantime::parse_duration)]
    s3_resubmit_interval: Duration,

    /// Coprocessor signer. `aws-kms` reads `AWS_KEY_ID`.
    #[arg(long, value_enum, default_value = "private-key")]
    signer_type: SignerType,

    #[arg(long)]
    private_key: Option<String>,

    /// HTTP server port for health checks
    #[arg(long, default_value_t = 8080)]
    health_check_port: u16,

    /// Prometheus metrics server address
    #[arg(long, default_value = "0.0.0.0:9100")]
    metrics_addr: Option<String>,

    #[arg(long, env = "OTEL_SERVICE_NAME", default_value = "pad-worker")]
    service_name: String,

    #[arg(
        long,
        value_parser = clap::value_parser!(Level),
        default_value_t = Level::INFO)]
    log_level: Level,

    /// Print the compiled-in coprocessor stack version and exit.
    #[arg(long)]
    stack_version: bool,
}

/// A key ID, so that clap takes one value, not a list of bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
struct KeyId(Vec<u8>);

fn parse_key_id(s: &str) -> Result<KeyId, String> {
    let bytes = hex::decode(s.strip_prefix("0x").unwrap_or(s)).map_err(|e| e.to_string())?;
    if bytes.is_empty() {
        return Err("empty key ID".to_string());
    }
    Ok(KeyId(bytes))
}

async fn signer(args: &Args) -> anyhow::Result<CoproSigner> {
    Ok(match args.signer_type {
        SignerType::PrivateKey => {
            let key = args
                .private_key
                .as_deref()
                .context("--private-key is required for the private-key signer")?;
            Arc::new(PrivateKeySigner::from_str(key.trim())?)
        }
        SignerType::AwsKms => {
            let key_id =
                std::env::var("AWS_KEY_ID").context("AWS_KEY_ID is required for aws-kms")?;
            let aws_conf = aws_config::load_defaults(BehaviorVersion::latest()).await;
            let client = aws_sdk_kms::Client::new(&aws_conf);
            Arc::new(AwsSigner::new(client, key_id, None).await?)
        }
    })
}

async fn uploader(args: &Args) -> Uploader {
    let aws_conf = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let config = aws_sdk_s3::config::Builder::from(&aws_conf)
        .timeout_config(
            TimeoutConfig::builder()
                .connect_timeout(Duration::from_secs(10))
                .operation_attempt_timeout(Duration::from_secs(60))
                .build(),
        )
        .build();
    Uploader::new(
        aws_sdk_s3::Client::from_conf(config),
        args.s3_bucket.clone(),
        args.s3_max_concurrent_uploads,
        args.s3_resubmit_interval,
    )
}

fn install_signal_handlers(cancel_token: CancellationToken) -> anyhow::Result<()> {
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;
    tokio::spawn(async move {
        tokio::select! {
            _ = sigint.recv() => (),
            _ = sigterm.recv() => (),
        }
        cancel_token.cancel();
    });
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    fhevm_engine_common::handle_stack_version_flag();
    let args = Args::parse();

    let _otel_guard = telemetry::init_tracing_otel_with_logs_only_fallback(
        args.log_level,
        &args.service_name,
        "otlp-layer",
    );
    info!(
        prf_instance = args.prf_instance,
        prf_key_id = hex::encode(&args.prf_key_id.0),
        database_pool_size = args.database_pool_size,
        max_concurrent = args.max_concurrent,
        claim_timeout = ?args.claim_timeout,
        poll_interval = ?args.poll_interval,
        pending_max_age = ?args.pending_max_age,
        output_max_age = ?args.output_max_age,
        gc_interval = ?args.gc_interval,
        start_check_interval = ?args.start_check_interval,
        s3_bucket = args.s3_bucket,
        s3_max_concurrent_uploads = args.s3_max_concurrent_uploads,
        s3_resubmit_interval = ?args.s3_resubmit_interval,
        health_check_port = args.health_check_port,
        metrics_addr = args.metrics_addr.as_deref().unwrap_or("none"),
        "Starting pad-worker"
    );

    let cancel_token = CancellationToken::new();
    install_signal_handlers(cancel_token.clone())?;
    let signer = signer(&args).await?;
    info!(signer = %signer.address(), "Signer loaded");
    warn!("Mock evaluator: the outputs are not PRF outputs");
    let uploader = uploader(&args).await;

    let database_url = resolve_database_url_from_option(args.database_url)?;
    let (pool, _refresh) = connect_pool_with_options(
        &database_url,
        PgPoolOptions::new().max_connections(args.database_pool_size),
        Some(&cancel_token),
    )
    .await?;

    let worker = PadWorker::new(
        Config {
            prf_instance: args.prf_instance,
            key_id: args.prf_key_id.0,
            max_concurrent: args.max_concurrent,
            claim_timeout: args.claim_timeout,
            poll_interval: args.poll_interval,
            pending_max_age: args.pending_max_age,
            output_max_age: args.output_max_age,
            gc_interval: args.gc_interval,
            start_check_interval: args.start_check_interval,
        },
        pool,
        Arc::new(MockEvaluator),
        signer,
        cancel_token.clone(),
    )
    .with_uploader(uploader);

    // The health server runs until the process exits, so a stop does not look like a crash.
    let health_server = HttpServer::new(
        Arc::new(worker.health()),
        args.health_check_port,
        CancellationToken::new(),
    );
    let health_task = tokio::spawn(async move { health_server.start().await });
    metrics_server::spawn(args.metrics_addr, cancel_token.child_token());

    let result = tokio::select! {
        res = worker.run() => res,
        res = health_task => Err(anyhow!("Health-check server exited: {res:?}")),
    };
    telemetry::flush();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_settings() {
        let args = Args::try_parse_from([
            "pad-worker",
            "--prf-instance=1",
            "--prf-key-id=0xab01",
            "--s3-bucket=b",
            "--claim-timeout=2m",
        ])
        .unwrap();
        assert_eq!(args.prf_instance, 1);
        assert_eq!(args.prf_key_id, KeyId(vec![0xab, 0x01]));
        assert_eq!(args.claim_timeout, Duration::from_secs(120));
        assert_eq!(args.pending_max_age, Duration::from_secs(600));
    }

    #[test]
    fn refuses_a_missing_or_bad_key_id() {
        let parse = |key_id: Option<&str>| {
            let mut args = vec!["pad-worker", "--prf-instance=1", "--s3-bucket=b"];
            args.extend(key_id);
            Args::try_parse_from(args)
        };
        assert!(parse(None).is_err());
        assert!(parse(Some("--prf-key-id=zz")).is_err());
        assert!(parse(Some("--prf-key-id=")).is_err());
    }
}
