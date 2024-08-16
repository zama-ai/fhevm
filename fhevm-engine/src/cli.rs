use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Run the API server
    #[arg(long)]
    pub run_server: bool,

    /// Run the background worker
    #[arg(long)]
    pub run_bg_worker: bool,

    /// Generate fhe keys and exit
    #[arg(long)]
    pub generate_fhe_keys: bool,

    /// Server maximum ciphertexts to schedule per batch
    #[arg(long, default_value_t = 5000)]
    pub server_maximum_ciphertexts_to_schedule: usize,

    /// Work items batch size
    #[arg(long, default_value_t = 10)]
    pub work_items_batch_size: i32,

    /// Tenant key cache size
    #[arg(long, default_value_t = 32)]
    pub tenant_key_cache_size: i32,

    /// Coprocessor FHE processing threads
    #[arg(long, default_value_t = 8)]
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

    /// Postgres database url. If unspecified DATABASE_URL environment variable is used
    #[arg(long)]
    pub database_url: Option<String>,
}

pub fn parse_args() -> Args {
    Args::parse()
}
