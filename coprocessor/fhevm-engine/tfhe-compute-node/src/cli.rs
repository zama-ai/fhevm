use clap::Parser;
use fhevm_engine_common::utils::DatabaseURL;
use tracing::Level;
use uuid::Uuid;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Postgres database url. If unspecified DATABASE_URL environment variable is used
    #[arg(long)]
    pub database_url: Option<DatabaseURL>,

    /// Redis URL  
    #[arg(long, default_value = "redis://localhost:6379")]
    pub redis_url: String,

    /// RabbitMQ URI
    #[arg(long, default_value = "amqp://admin:admin@localhost:5672/%2f")]
    pub rmq_uri: String,

    /// Queue name for receiving FHE partitions to execute
    #[arg(long, default_value = "queue_fhe_partitions")]
    pub queue_fhe_partitions: String,

    /// Tenant key cache size
    #[arg(long, default_value_t = 32)]
    pub tenant_key_cache_size: i32,

    /// Ciphertext cache size
    #[arg(long, default_value_t = 1000)]
    pub ciphertext_cache_size: i32,

    /// tfhe-worker service name in OTLP traces
    #[arg(long, default_value = "tfhe-compute-node")]
    pub service_name: String,

    /// Prometheus metrics server address
    #[arg(long, default_value = "0.0.0.0:9100")]
    pub metrics_addr: Option<String>,

    /// Tokio processing threads
    #[arg(long, default_value_t = 32)]
    pub blocking_fhe_threads: usize,

    /// Tokio Async IO threads
    #[arg(long, default_value_t = 4)]
    pub tokio_threads: usize,

    /// Log level for the application
    #[arg(
        long,
        value_parser = clap::value_parser!(Level),
        default_value_t = Level::INFO)]
    pub log_level: Level,

    /// Worker/replica ID for this worker instance
    /// If not provided, a random UUID will be generated
    #[arg(long, value_parser = clap::value_parser!(Uuid))]
    pub worker_id: Option<Uuid>,
}

pub fn parse() -> Args {
    Args::parse()
}
