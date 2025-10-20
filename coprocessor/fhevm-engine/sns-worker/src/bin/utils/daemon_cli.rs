use std::time::Duration;

use clap::{command, Parser};
use humantime::parse_duration;
use sns_worker::SchedulePolicy;
use tracing::Level;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Tenant API key
    #[arg(long)]
    pub tenant_api_key: String,

    /// Work items batch size
    #[arg(long, default_value_t = 4)]
    pub work_items_batch_size: u32,

    /// Garbage collection batch size
    #[arg(long, default_value_t = 80)]
    pub gc_batch_size: u32,

    /// NOTIFY/LISTEN channels for database that the worker listen to
    #[arg(long, num_args(1..))]
    pub pg_listen_channels: Vec<String>,

    /// NOTIFY/LISTEN channel for database that the worker notify to
    #[arg(long)]
    pub pg_notify_channel: String,

    /// Polling interval in seconds
    #[arg(long, default_value_t = 60)]
    pub pg_polling_interval: u32,

    /// Postgres pool connections
    #[arg(long, default_value_t = 10)]
    pub pg_pool_connections: u32,

    /// Postgres acquire timeout
    #[arg(long, default_value = "15s", value_parser = parse_duration)]
    pub pg_timeout: Duration,

    /// Postgres diagnostics: enable auto_explain extension
    #[arg(long, value_parser = parse_duration)]
    pub pg_auto_explain_with_min_duration: Option<Duration>,

    /// Postgres database url. If unspecified DATABASE_URL environment variable
    /// is used
    #[arg(long)]
    pub database_url: Option<String>,

    /// KeySet file. If unspecified the the keys are read from the database
    #[arg(long)]
    pub keys_file_path: Option<String>,

    /// sns-executor service name in OTLP traces
    #[arg(long, default_value = "sns-executor")]
    pub service_name: String,

    /// S3 bucket name for ct128 ciphertexts
    /// See also: general purpose buckets naming rules
    #[arg(long, default_value = "ct128")]
    pub bucket_name_ct128: String,

    /// S3 bucket name for ct64 ciphertexts
    /// See also: general purpose buckets naming rules
    #[arg(long, default_value = "ct64")]
    pub bucket_name_ct64: String,

    /// Maximum number of concurrent uploads to S3
    #[arg(long, default_value_t = 100)]
    pub s3_max_concurrent_uploads: u32,

    #[arg(long, default_value_t = 100)]
    pub s3_max_retries_per_upload: u32,

    #[arg(long, default_value = "10s", value_parser = parse_duration)]
    pub s3_max_backoff: Duration,

    #[arg(long, default_value = "120s", value_parser = parse_duration)]
    pub s3_max_retries_timeout: Duration,

    #[arg(long, default_value = "2s", value_parser = parse_duration)]
    pub s3_recheck_duration: Duration,

    #[arg(long, default_value = "120s", value_parser = parse_duration)]
    pub s3_regular_recheck_duration: Duration,

    #[arg(long, default_value = "120s", value_parser = parse_duration)]
    pub cleanup_interval: Duration,

    #[arg(
        long,
        value_parser = clap::value_parser!(Level),
        default_value_t = Level::INFO)]
    pub log_level: Level,

    /// HTTP server port for health checks
    #[arg(long, default_value_t = 8080)]
    pub health_check_port: u16,

    /// Prometheus metrics server address
    #[arg(long, default_value = "0.0.0.0:9100")]
    pub metrics_addr: Option<String>,

    /// Liveness threshold for health checks
    /// Exceeding this threshold means that the worker is stuck
    /// and will be restarted by the orchestrator
    #[arg(long, default_value = "70s", value_parser = parse_duration)]
    pub liveness_threshold: Duration,

    /// LIFO (Last In, First Out) processing
    /// If true, the worker will process the most recent tasks
    /// if false, default FIFO (First In, First Out) processing is used
    #[arg(long, default_value_t = false)]
    pub lifo: bool,

    /// Enable compression of big ciphertexts before uploading to S3
    #[arg(long, default_value_t = true)]
    pub enable_compression: bool,

    /// Schedule policy for processing tasks
    #[arg(long, default_value = "rayon_parallel", value_parser = clap::value_parser!(SchedulePolicy))]
    pub schedule_policy: SchedulePolicy,
}

pub fn parse_args() -> Args {
    Args::parse()
}
