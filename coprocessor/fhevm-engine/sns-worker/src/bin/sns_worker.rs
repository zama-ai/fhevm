use sns_worker::{Config, DBConfig, HealthCheckConfig, S3Config, S3RetryPolicy};

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

fn construct_config() -> Config {
    let args: utils::daemon_cli::Args = utils::daemon_cli::parse_args();

    let db_url = args.database_url.clone().unwrap_or_default();

    Config {
        tenant_api_key: args.tenant_api_key,
        service_name: args.service_name,
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
        },
        log_level: args.log_level,
        health_checks: HealthCheckConfig {
            liveness_threshold: args.liveness_threshold,
            port: args.health_check_port,
        },
        enable_compression: args.enable_compression,
        schedule_policy: args.schedule_policy,
        pg_auto_explain_with_min_duration: args.pg_auto_explain_with_min_duration,
    }
}

#[tokio::main]
async fn main() {
    let config: Config = construct_config();
    let parent = CancellationToken::new();

    tracing_subscriber::fmt()
        .json()
        .with_target(false) // drop "target" field so the logs are not too verbose. Instead, span names are used.
        .with_current_span(true) // keep "span"
        .with_span_list(false) // drop "spans"
        .with_level(true)
        .with_max_level(config.log_level)
        .init();

    // Handle SIGINIT signals
    handle_sigint(parent.clone());

    sns_worker::run_all(config, parent, None)
        .await
        .unwrap_or_else(|err| {
            error!(error = %err, "Error running SNS worker");
            std::process::exit(1);
        });
}
