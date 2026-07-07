use sns_worker::{Config, DBConfig, HealthCheckConfig, S3Config, S3RetryPolicy, SNSMetricsConfig};

use fhevm_engine_common::database::resolve_database_url_from_option;
use fhevm_engine_common::telemetry;
use tokio::signal::unix;
use tokio_util::sync::CancellationToken;
use tracing::error;
mod utils;

fn handle_sigint(token: CancellationToken) {
    tokio::spawn(async move {
        let mut signal = unix::signal(unix::SignalKind::interrupt()).unwrap();
        signal.recv().await;
        token.cancel();
    });
}

fn construct_config() -> Result<Config, fhevm_engine_common::database::DatabaseConnectionError> {
    let args: utils::daemon_cli::Args = utils::daemon_cli::parse_args();

    let db_url = resolve_database_url_from_option(args.database_url.clone())?;

    Ok(Config {
        service_name: args.service_name,
        metrics: SNSMetricsConfig {
            addr: args.metrics_addr,
            gauge_update_interval_secs: args.gauge_update_interval_secs,
        },
        signer_type: args.signer_type,
        private_key: args.private_key,
        db: DBConfig {
            url: db_url,
            listen_channels: args.pg_listen_channels,
            notify_channel: args.pg_notify_channel,
            batch_limit: args.work_items_batch_size,
            gc_batch_limit: args.gc_batch_size,
            polling_interval: args.pg_polling_interval,
            max_connections: args.pg_pool_connections,
            cleanup_interval: args.cleanup_interval,
            timeout: args.pg_timeout,
            lifo: args.lifo,
        },
        s3: S3Config {
            bucket_ct128: args.bucket_name_ct128,
            bucket_ct64: args.bucket_name_ct64,
            max_concurrent_uploads: args.s3_max_concurrent_uploads,
            retry_policy: S3RetryPolicy {
                max_retries_per_upload: args.s3_max_retries_per_upload,
                max_backoff: args.s3_max_backoff,
                max_retries_timeout: args.s3_max_retries_timeout,
                recheck_duration: args.s3_recheck_duration,
                regular_recheck_duration: args.s3_regular_recheck_duration,
            },
            verify_sha256_checksum: !args.s3_disable_sha256_checksum,
        },
        log_level: args.log_level,
        health_checks: HealthCheckConfig {
            liveness_threshold: args.liveness_threshold,
            port: args.health_check_port,
        },
        enable_compression: args.enable_compression,
        schedule_policy: args.schedule_policy,
        pg_auto_explain_with_min_duration: args.pg_auto_explain_with_min_duration,
        // Placeholder: overwritten in `main` via
        // `fhevm_engine_common::versioning::resolve_gcs_mode`, which is async
        // and therefore cannot be called from this sync function.
        gcs_mode: false,
        s3_migration: args.s3_migration,
        s3_migration_sleep_duration: args.s3_migration_sleep_duration,
        s3_migration_max_retries: args.s3_migration_max_retries,
    })
}

#[tokio::main]
async fn main() {
    let mut config: Config = construct_config().unwrap_or_else(|err| {
        error!(error = %err, "Invalid database configuration");
        std::process::exit(1);
    });

    let parent = CancellationToken::new();

    let _otel_guard = telemetry::init_tracing_otel_with_logs_only_fallback(
        config.log_level,
        &config.service_name,
        "otlp-layer",
    );

    // Resolved after tracing is initialized so the `resolve_gcs_mode` log is
    // captured by the subscriber.
    config.gcs_mode =
        match fhevm_engine_common::versioning::resolve_gcs_mode(config.db.url.as_str()).await {
            Ok(gcs_mode) => gcs_mode,
            Err(err) => {
                error!(error = %err, "Failed to resolve gcs_mode from versioning table");
                std::process::exit(1);
            }
        };

    // Handle SIGINIT signals
    handle_sigint(parent.clone());

    sns_worker::run_all(config, parent, None)
        .await
        .unwrap_or_else(|err| {
            error!(error = %err, "Error running SNS worker");
            telemetry::flush();
            std::process::exit(1);
        });
}
