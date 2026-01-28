use clap::Parser;
use fhevm_engine_common::telemetry::MetricsConfig;
use fhevm_engine_common::utils::DatabaseURL;
use tracing::Level;
use uuid::Uuid;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Run the API server
    #[arg(long)]
    pub run_server: bool,

    /// Run the background worker
    #[arg(long)]
    pub run_bg_worker: bool,

    /// Polling interval for the background worker to fetch jobs
    #[arg(long, default_value_t = 1000)]
    pub worker_polling_interval_ms: u64,

    /// Generate fhe keys and exit
    #[arg(long)]
    pub generate_fhe_keys: bool,

    /// Server maximum ciphertexts to schedule per batch
    #[arg(long, default_value_t = 5000)]
    pub server_maximum_ciphertexts_to_schedule: usize,

    /// Server maximum ciphertexts to serve on get_cihpertexts endpoint
    #[arg(long, default_value_t = 5000)]
    pub server_maximum_ciphertexts_to_get: usize,

    /// Work items batch size
    #[arg(long, default_value_t = 100)]
    pub work_items_batch_size: i32,

    /// Number of dependence chains to fetch per worker
    #[arg(long, default_value_t = 20)]
    pub dependence_chains_per_batch: i32,

    /// Key cache size
    #[arg(long, default_value_t = 32, alias = "tenant-key-cache-size")]
    pub key_cache_size: usize,

    /// Maximum compact inputs to upload
    #[arg(long, default_value_t = 10)]
    pub maximum_compact_inputs_upload: usize,

    /// Maximum compact inputs to upload
    #[arg(long, default_value_t = 255)]
    pub maximum_handles_per_input: u8,

    /// Coprocessor FHE processing threads
    #[arg(long, default_value_t = 32)]
    pub coprocessor_fhe_threads: usize,

    /// Tokio Async IO threads
    #[arg(long, default_value_t = 4)]
    pub tokio_threads: usize,

    /// Postgres pool max connections
    #[arg(long, default_value_t = 10)]
    pub pg_pool_max_connections: u32,

    /// Server socket address
    #[arg(long, default_value = "127.0.0.1:50051")]
    pub server_addr: String,

    /// Prometheus metrics server address
    #[arg(long, default_value = "0.0.0.0:9100")]
    pub metrics_addr: Option<String>,

    /// Postgres database url. If unspecified DATABASE_URL environment variable is used
    #[arg(long)]
    pub database_url: Option<DatabaseURL>,

    /// Coprocessor private key file path.
    /// Private key is in plain text 0x1234.. format.
    #[arg(long, default_value = "./coprocessor.key")]
    pub coprocessor_private_key: String,

    /// tfhe-worker service name in OTLP traces
    #[arg(long, default_value = "tfhe-worker")]
    pub service_name: String,

    /// Worker/replica ID for this worker instance
    /// If not provided, a random UUID will be generated
    /// Used to identify the worker in the dependence_chain table
    #[arg(long, value_parser = clap::value_parser!(Uuid))]
    pub worker_id: Option<Uuid>,

    /// Time-to-live in seconds for dependence chain locks
    /// Defaults to 30 seconds if not provided
    #[arg(long, value_parser = clap::value_parser!(u32), default_value_t = 30)]
    pub dcid_ttl_sec: u32,

    /// If set to true, disable dependence chain ID locking mechanism
    /// Enabling this may lead to multiple workers processing the same dependence chain simultaneously
    /// Useful for fallbacking to non-locking behavior in case of issues with the locking mechanism
    #[arg(long, value_parser = clap::value_parser!(bool), default_value_t = false)]
    pub disable_dcid_locking: bool,

    /// Time slice in seconds for processing each dependence chain
    /// If a worker exceeds this time while processing a dependence chain,
    /// it will release the lock and allow other workers to acquire it
    #[arg(long, default_value_t = 90)]
    pub dcid_timeslice_sec: u32,

    /// Time-to-live in seconds for processed dependence chains
    /// Processed dependence chains older than this TTL will be deleted during idle time
    #[arg(long, default_value_t = 48*60*60)] // Keep dcid not older than 48 hours
    pub processed_dcid_ttl_sec: u32,

    /// Interval in seconds for cleaning up expired dependence chain locks
    #[arg(long, default_value_t = 3600)]
    pub dcid_cleanup_interval_sec: u32,

    /// Maximum number of worker cycles allowed without progress on a
    /// dependence chain
    #[arg(long, value_parser = clap::value_parser!(u32), default_value_t = 2)]
    pub dcid_max_no_progress_cycles: u32,

    /// Log level for the application
    #[arg(
        long,
        value_parser = clap::value_parser!(Level),
        default_value_t = Level::INFO)]
    pub log_level: Level,

    #[arg(long, default_value_t = 8080)]
    pub health_check_port: u16,

    /// Prometheus metrics: coprocessor_rerand_batch_latency_seconds
    #[arg(long, default_value = "0.1:5.0:0.01", value_parser = clap::value_parser!(MetricsConfig))]
    pub metric_rerand_batch_latency: MetricsConfig,

    /// Prometheus metrics: coprocessor_fhe_batch_latency_seconds
    #[arg(long, default_value = "0.2:5.0:0.05", value_parser = clap::value_parser!(MetricsConfig))]
    pub metric_fhe_batch_latency: MetricsConfig,
}

pub fn parse_args() -> Args {
    let args = Args::parse();
    // Set global configs from args
    let _ = scheduler::RERAND_LATENCY_BATCH_HISTOGRAM_CONF.set(args.metric_rerand_batch_latency);
    let _ = scheduler::FHE_BATCH_LATENCY_HISTOGRAM_CONF.set(args.metric_fhe_batch_latency);
    args
}
