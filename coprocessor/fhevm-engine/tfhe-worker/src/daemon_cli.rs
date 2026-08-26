use clap::Parser;
use fhevm_engine_common::drift_revert::WatcherTimeouts;
use fhevm_engine_common::telemetry::MetricsConfig;
use fhevm_engine_common::utils::DatabaseURL;
use tracing::Level;
use uuid::Uuid;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Run the background worker
    #[arg(long)]
    pub run_bg_worker: bool,

    /// Polling interval for the background worker to fetch jobs
    #[arg(long, default_value_t = 1000)]
    pub worker_polling_interval_ms: u64,

    /// Polling interval (ms) for the confidential bridge worker to associate
    /// bridged handles. Minimum 10ms: a smaller value would busy-spin the idle
    /// loop with empty readiness queries.
    #[arg(long, value_parser = clap::value_parser!(u64).range(10..), default_value_t = 1000)]
    pub bridge_polling_interval_ms: u64,

    /// Max bridged-handle pairs the confidential bridge worker associates per transaction
    #[arg(long, value_parser = clap::value_parser!(i64).range(1..), default_value_t = 128)]
    pub bridge_associate_batch_size: i64,

    /// Generate fhe keys and exit
    #[arg(long)]
    pub generate_fhe_keys: bool,

    /// Work items batch size
    #[arg(long, default_value_t = 100)]
    pub work_items_batch_size: i32,

    /// Number of dependence chains to fetch per worker
    #[arg(long, default_value_t = 20)]
    pub dependence_chains_per_batch: i32,

    /// Acquire and execute multiple independent dependence chains in one
    /// worker schedule. Disabled by default to retain the production
    /// single-DCID lifecycle; the reportable benchmark enables it explicitly.
    #[arg(
        long,
        env = "FHEVM_BENCH_DCID_BATCH_EXECUTION",
        default_value_t = false
    )]
    pub dcid_batch_execution: bool,

    /// Key cache size
    #[arg(long, default_value_t = 32, alias = "tenant-key-cache-size")]
    pub key_cache_size: usize,

    /// Coprocessor FHE processing threads
    #[arg(long, default_value_t = 32)]
    pub coprocessor_fhe_threads: usize,

    /// Maximum time a GPU operation may wait for memory capacity before it
    /// fails (and its batch retries) instead of spinning while holding
    /// resources. CPU builds ignore this setting.
    #[arg(
        long,
        env = "FHEVM_GPU_MEMORY_RESERVATION_TIMEOUT_MS",
        default_value_t = 300_000
    )]
    pub gpu_memory_reservation_timeout_ms: u64,

    /// Maximum number of concurrent CUDA streams allocated per visible GPU.
    /// CPU builds ignore this setting. Single-stream execution serializes
    /// partition dispatch and measurably regresses block-scoped workloads
    /// versus unbounded main; 16 is the measured plateau on H100.
    #[arg(long, env = "FHEVM_GPU_STREAMS_PER_DEVICE", value_parser = parse_nonzero_usize, default_value_t = 16)]
    pub gpu_streams_per_device: usize,

    /// Tokio Async IO threads
    #[arg(long, default_value_t = 4)]
    pub tokio_threads: usize,

    /// Postgres pool max connections
    #[arg(long, default_value_t = 10)]
    pub pg_pool_max_connections: u32,

    /// Prometheus metrics server address
    #[arg(long, default_value = "0.0.0.0:9100")]
    pub metrics_addr: Option<String>,

    /// Postgres database url. If unspecified DATABASE_URL environment variable is used
    #[arg(long)]
    pub database_url: Option<DatabaseURL>,

    /// tfhe-worker service name in OTLP traces
    #[arg(long, env = "OTEL_SERVICE_NAME", default_value = "tfhe-worker")]
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

    /// Number of no-progress DCID releases before ignoring dependence counter
    #[arg(long, value_parser = clap::value_parser!(u32), default_value_t = 100)]
    pub dcid_ignore_dependency_count_threshold: u32,

    /// Minimum age (seconds) of an unowned, dependency-gated dependence
    /// chain before the idle-time repair acquisition may pick it up. The
    /// repair path only takes chains whose gate is provably stale (every
    /// producer chain processed or gone, count never decremented); this age
    /// gate additionally keeps it from racing a listener transaction
    /// mid-arm.
    #[arg(long, default_value_t = 300.0)]
    pub dcid_stale_gate_age_secs: f64,

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

    /// Liveness override budget for one in-flight worker batch, in seconds:
    /// a batch older than this stops keeping the pod alive, so a genuinely
    /// wedged execution is eventually restarted while per-op progress ticks
    /// keep legitimate long batches healthy.
    #[arg(long, env = "TFHE_WORKER_MAX_BATCH_TTL_SECS", default_value_t = 300)]
    pub max_batch_ttl_secs: u64,

    /// Print the compiled-in coprocessor stack version and exit.
    #[arg(long)]
    pub stack_version: bool,

    /// Not exposed via CLI — `#[arg(skip)]` initializes the field to `WatcherTimeouts::default()`
    /// on `Args::parse()`.
    #[arg(skip)]
    pub drift_revert_watcher_timeouts: WatcherTimeouts,
}

fn parse_nonzero_usize(value: &str) -> Result<usize, String> {
    let parsed = value
        .parse::<usize>()
        .map_err(|error| format!("must be a positive integer: {error}"))?;
    if parsed == 0 {
        Err("must be at least 1".to_owned())
    } else {
        Ok(parsed)
    }
}

pub fn parse_args() -> Args {
    fhevm_engine_common::handle_stack_version_flag();
    let args = Args::parse();
    // Set global configs from args
    let _ = scheduler::RERAND_LATENCY_BATCH_HISTOGRAM_CONF.set(args.metric_rerand_batch_latency);
    let _ = scheduler::FHE_BATCH_LATENCY_HISTOGRAM_CONF.set(args.metric_fhe_batch_latency);
    args
}
