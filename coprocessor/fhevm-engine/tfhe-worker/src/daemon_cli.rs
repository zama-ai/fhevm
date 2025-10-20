use clap::Parser;
use tracing::Level;

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

    /// Tenant key cache size
    #[arg(long, default_value_t = 32)]
    pub tenant_key_cache_size: i32,

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
    pub database_url: Option<String>,

    /// Coprocessor private key file path.
    /// Private key is in plain text 0x1234.. format.
    #[arg(long, default_value = "./coprocessor.key")]
    pub coprocessor_private_key: String,

    /// tfhe-worker service name in OTLP traces
    #[arg(long, default_value = "tfhe-worker")]
    pub service_name: String,

    /// Log level for the application
    #[arg(
        long,
        value_parser = clap::value_parser!(Level),
        default_value_t = Level::INFO)]
    pub log_level: Level,

    #[arg(long, default_value_t = 8080)]
    pub health_check_port: u16,
}

pub fn parse_args() -> Args {
    Args::parse()
}
