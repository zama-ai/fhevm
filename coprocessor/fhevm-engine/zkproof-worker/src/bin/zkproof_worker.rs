use clap::{command, Parser};
use fhevm_engine_common::telemetry;
use tracing::{error, info, Level};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// NOTIFY/LISTEN channel for database that the worker listen to
    #[arg(long)]
    pub pg_listen_channel: String,

    /// NOTIFY/LISTEN channel for database that the worker notify to
    #[arg(long)]
    pub pg_notify_channel: String,

    /// Polling interval in seconds
    #[arg(long, default_value_t = 60)]
    pub pg_polling_interval: u32,

    /// Postgres pool connections
    #[arg(long, default_value_t = 5)]
    pub pg_pool_connections: u32,

    /// Postgres database url. If unspecified DATABASE_URL environment variable
    /// is used
    #[arg(long)]
    pub database_url: Option<String>,

    /// Number of zkproof workers to process proofs in parallel
    #[arg(long, default_value_t = 8)]
    pub worker_thread_count: u32,

    /// Zkproof-worker service name in OTLP traces
    #[arg(long, default_value = "zkproof-worker")]
    pub service_name: String,

    /// Log level for the worker
    #[arg(
        long,
        value_parser = clap::value_parser!(Level),
        default_value_t = Level::INFO)]
    pub log_level: Level,
}

pub fn parse_args() -> Args {
    Args::parse()
}

#[tokio::main]
async fn main() {
    let args = parse_args();
    tracing_subscriber::fmt()
        .json()
        .with_level(true)
        .with_max_level(args.log_level)
        .init();

    let database_url = args
        .database_url
        .clone()
        .unwrap_or_else(|| std::env::var("DATABASE_URL").expect("DATABASE_URL is undefined"));

    let conf = zkproof_worker::Config {
        database_url,
        listen_database_channel: args.pg_listen_channel,
        notify_database_channel: args.pg_notify_channel,
        pg_pool_connections: args.pg_pool_connections,
        pg_polling_interval: args.pg_polling_interval,
        worker_thread_count: args.worker_thread_count,
    };

    if let Err(err) = telemetry::setup_otlp(&args.service_name) {
        error!("Error while initializing tracing: {:?}", err);
        std::process::exit(1);
    }

    info!("Starting zkProof worker...");
    if let Err(err) = zkproof_worker::verifier::execute_verify_proofs_loop(&conf).await {
        error!("Worker failed: {:?}", err);
    }
}
