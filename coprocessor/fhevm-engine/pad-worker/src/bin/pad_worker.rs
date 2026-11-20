use alloy::{
    primitives::U256,
    signers::{
        aws::{aws_sdk_kms, AwsSigner},
        local::PrivateKeySigner,
    },
};
use anyhow::{anyhow, ensure, Context};
use aws_config::{timeout::TimeoutConfig, BehaviorVersion};
use aws_sdk_s3::config::{RequestChecksumCalculation, ResponseChecksumValidation};
use clap::Parser;
use fhevm_engine_common::{
    database::{connect_pool_with_options, resolve_database_url_from_option},
    db_keys::DbKeyCache,
    healthz_server::HttpServer,
    metrics_server, telemetry,
    types::{CoproSigner, SignerType},
    utils::DatabaseURL,
};
use pad_worker::{
    evaluator::TfheEvaluator, upload::Uploader, wait_for_key, Config, Health, PadWorker,
};
use sqlx::postgres::PgPoolOptions;
use std::{num::NonZeroUsize, str::FromStr, sync::Arc, time::Duration};
use tokio::signal::unix::{signal, SignalKind};
use tokio_util::sync::CancellationToken;
use tracing::{info, Level};

/// On a stop, the time for the claimed rows to complete. Below the 30 s grace period.
const DRAIN_TIMEOUT: Duration = Duration::from_secs(20);

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    /// The PRF instance of this pad-worker.
    #[arg(long, env = "PRF_INSTANCE")]
    prf_instance: u16,

    /// KMSGeneration key ID of the instance, in decimal or `0x` hex.
    #[arg(long, env = "PRF_KEY_ID", value_parser = parse_key_id)]
    prf_key_id: U256,

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

    /// Delete uploaded or failed rows (the ct128) older than this. `0s` keeps all rows.
    #[arg(long, env = "PAD_CT128_MAX_AGE", default_value = "1h", value_parser = humantime::parse_duration)]
    ct128_max_age: Duration,

    /// Delete the ct64 older than this, at least `PAD_CT128_MAX_AGE`. Not set or `0s` keeps all
    /// ct64.
    #[arg(long, env = "PAD_CT64_MAX_AGE", value_parser = humantime::parse_duration)]
    ct64_max_age: Option<Duration>,

    /// Interval of the GC and the row age metrics.
    #[arg(long, env = "PAD_GC_INTERVAL", default_value = "1m", value_parser = humantime::parse_duration)]
    gc_interval: Duration,

    /// Poll interval until the key of `--prf-key-id` is in the database.
    #[arg(long, env = "PAD_KEY_POLL_INTERVAL", default_value = "30s", value_parser = humantime::parse_duration)]
    key_poll_interval: Duration,

    /// Ciphertext bucket of the operator, for the S3 copy of the outputs.
    #[arg(long, env = "PAD_S3_BUCKET")]
    s3_bucket: String,

    #[arg(long, env = "PAD_S3_MAX_CONCURRENT_UPLOADS", default_value = "16")]
    s3_max_concurrent_uploads: NonZeroUsize,

    /// For S3 servers without checksum support.
    #[arg(long, env = "PAD_S3_DISABLE_SHA256_CHECKSUM", default_value_t = false)]
    s3_disable_sha256_checksum: bool,

    /// Interval of the read that finds failed uploads claimed before `PAD_CLAIM_TIMEOUT`.
    #[arg(long, env = "PAD_S3_RESUBMIT_INTERVAL", default_value = "30s", value_parser = humantime::parse_duration)]
    s3_resubmit_interval: Duration,

    /// Coprocessor signer. `aws-kms` reads `AWS_KEY_ID`.
    #[arg(long, value_enum, default_value = "private-key")]
    signer_type: SignerType,

    /// Signing key with `--signer-type private-key`. Prefer the env var: a flag value is visible in
    /// the process list.
    #[arg(long, env = "TX_SENDER_PRIVATE_KEY", hide_env_values = true)]
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

fn parse_key_id(s: &str) -> Result<U256, String> {
    if s.is_empty() {
        return Err("empty key ID".to_string());
    }
    s.parse().map_err(|e| format!("{e}"))
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

/// Returns the age of a GC, or `None` for `0s`: the GC is off.
fn gc_age(age: Duration) -> Option<Duration> {
    Some(age).filter(|age| !age.is_zero())
}

/// The ct64 stays at least as long as the ct128.
fn check_ages(args: &Args) -> anyhow::Result<()> {
    if let Some(ct64) = args.ct64_max_age.and_then(gc_age) {
        ensure!(
            gc_age(args.ct128_max_age).is_some_and(|ct128| ct64 >= ct128),
            "--ct64-max-age must be at least --ct128-max-age, and --ct128-max-age must not be 0s"
        );
    }
    Ok(())
}

async fn uploader(args: &Args) -> Uploader {
    let aws_conf = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let mut config = aws_sdk_s3::config::Builder::from(&aws_conf).timeout_config(
        TimeoutConfig::builder()
            .connect_timeout(Duration::from_secs(10))
            .operation_attempt_timeout(Duration::from_secs(60))
            .build(),
    );
    if args.s3_disable_sha256_checksum {
        // Also turn off the default checksum of the SDK.
        config = config
            .request_checksum_calculation(RequestChecksumCalculation::WhenRequired)
            .response_checksum_validation(ResponseChecksumValidation::WhenRequired);
    }
    let uploader = Uploader::new(
        aws_sdk_s3::Client::from_conf(config.build()),
        args.s3_bucket.clone(),
        args.s3_max_concurrent_uploads,
        args.s3_resubmit_interval,
    );
    if args.s3_disable_sha256_checksum {
        return uploader.without_sha256_checksum();
    }
    uploader
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
    check_ages(&args)?;

    let _otel_guard = telemetry::init_tracing_otel_with_logs_only_fallback(
        args.log_level,
        &args.service_name,
        "otlp-layer",
    );
    info!(
        prf_instance = args.prf_instance,
        prf_key_id = %args.prf_key_id,
        database_pool_size = args.database_pool_size,
        max_concurrent = args.max_concurrent,
        claim_timeout = ?args.claim_timeout,
        poll_interval = ?args.poll_interval,
        pending_max_age = ?args.pending_max_age,
        ct128_max_age = ?args.ct128_max_age,
        ct64_max_age = ?args.ct64_max_age,
        gc_interval = ?args.gc_interval,
        key_poll_interval = ?args.key_poll_interval,
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
    let uploader = uploader(&args).await;

    let database_url = resolve_database_url_from_option(args.database_url)?;
    let (pool, _refresh) = connect_pool_with_options(
        &database_url,
        PgPoolOptions::new().max_connections(args.database_pool_size),
        Some(&cancel_token),
    )
    .await?;

    // Start before the key load, so the liveness probe answers.
    let health = Arc::new(Health::new(pool.clone()));
    // The health server runs until the process exits, so a stop does not look like a crash.
    let health_server = HttpServer::new(
        health.clone(),
        args.health_check_port,
        CancellationToken::new(),
    );
    let health_task = tokio::spawn(async move { health_server.start().await });
    metrics_server::spawn(args.metrics_addr, cancel_token.child_token());

    let key_id = args.prf_key_id.to_be_bytes::<32>().to_vec();
    if !wait_for_key(&pool, &key_id, args.key_poll_interval, &cancel_token).await? {
        return Ok(());
    }
    let key = DbKeyCache::new_keeping_noise_squashing(1)?
        .fetch(&key_id, &pool)
        .await
        .context("load the key of --prf-key-id")?;
    info!(prf_key_id = %args.prf_key_id, "Key loaded");
    let evaluator = Arc::new(TfheEvaluator::new(key.sks));

    let worker = PadWorker::new(
        Config {
            prf_instance: args.prf_instance,
            key_id,
            max_concurrent: args.max_concurrent,
            claim_timeout: args.claim_timeout,
            poll_interval: args.poll_interval,
            pending_max_age: args.pending_max_age,
            ct128_max_age: gc_age(args.ct128_max_age),
            ct64_max_age: args.ct64_max_age.and_then(gc_age),
            gc_interval: args.gc_interval,
            drain_timeout: DRAIN_TIMEOUT,
        },
        pool,
        evaluator,
        signer,
        cancel_token.clone(),
    )
    .with_uploader(uploader)
    .with_health(&health);

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
            "--prf-key-id=0xab",
            "--s3-bucket=b",
            "--claim-timeout=2m",
        ])
        .unwrap();
        assert_eq!(args.prf_instance, 1);
        assert_eq!(args.prf_key_id, U256::from(0xab));
        assert_eq!(args.claim_timeout, Duration::from_secs(120));
        assert_eq!(args.pending_max_age, Duration::from_secs(600));
        assert_eq!(args.ct128_max_age, Duration::from_secs(3600));
        assert_eq!(args.ct64_max_age, None);
    }

    #[test]
    fn refuses_a_ct64_age_below_the_ct128_age() {
        let check = |ages: &[&str]| {
            let mut args = vec![
                "pad-worker",
                "--prf-instance=1",
                "--prf-key-id=0xab",
                "--s3-bucket=b",
            ];
            args.extend(ages);
            check_ages(&Args::try_parse_from(args).unwrap())
        };
        assert!(check(&[]).is_ok());
        assert!(check(&["--ct64-max-age=0s"]).is_ok());
        assert!(check(&["--ct128-max-age=0s", "--ct64-max-age=0s"]).is_ok());
        assert!(check(&["--ct64-max-age=1h"]).is_ok());
        assert!(check(&["--ct64-max-age=59m"]).is_err());
        // The GC keeps every ct128.
        assert!(check(&["--ct128-max-age=0s", "--ct64-max-age=1d"]).is_err());
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
        // Above 2^256.
        let too_large = format!("--prf-key-id=0x1{}", "0".repeat(64));
        assert!(parse(Some(&too_large)).is_err());
    }
}
