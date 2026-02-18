use clap::Parser;
use tracing::Level;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// RabbitMQ URI
    #[arg(long, default_value = "amqp://admin:admin@localhost:5672/%2f")]
    pub rmq_uri: String,

    /// Coprocessor FHE processing threads
    #[arg(long, default_value_t = 32)]
    pub coprocessor_fhe_threads: usize,

    /// Tokio Async IO threads
    #[arg(long, default_value_t = 4)]
    pub tokio_threads: usize,

    /// tfhe-worker service name in OTLP traces
    #[arg(long, default_value = "tfhe-worker")]
    pub service_name: String,

    /// Log level for the application
    #[arg(
        long,
        value_parser = clap::value_parser!(Level),
        default_value_t = Level::INFO)]
    pub log_level: Level,
}

pub fn parse_args() -> Args {
    Args::parse()
}
