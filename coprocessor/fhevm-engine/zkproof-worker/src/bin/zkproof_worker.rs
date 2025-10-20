use clap::{command, Parser};
use fhevm_engine_common::telemetry;
use fhevm_engine_common::{healthz_server::HttpServer, metrics_server};
use humantime::parse_duration;
use std::{sync::Arc, time::Duration};
use tokio::{join, task};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, Level};
use zkproof_worker::verifier::ZkProofService;

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

    /// Postgres acquire timeout
    /// A longer timeout could affect the healthz/liveness updates
    #[arg(long, default_value = "15s", value_parser = parse_duration)]
    pub pg_timeout: Duration,

    /// Postgres diagnostics: enable auto_explain extension
    #[arg(long, value_parser = parse_duration)]
    pub pg_auto_explain_with_min_duration: Option<Duration>,

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

    /// HTTP server port for health checks
    #[arg(long, default_value_t = 8080)]
    health_check_port: u16,

    /// Prometheus metrics server address
    #[arg(long, default_value = "0.0.0.0:9100")]
    pub metrics_addr: Option<String>,
}

pub fn parse_args() -> Args {
    Args::parse()
}

#[tokio::main]
async fn main() {
    let args = parse_args();
    tracing_subscriber::fmt()
        .json()
        .with_current_span(true)
        .with_span_list(false)
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
        pg_timeout: args.pg_timeout,
        pg_auto_explain_with_min_duration: args.pg_auto_explain_with_min_duration,
    };

    if !args.service_name.is_empty() {
        if let Err(err) = telemetry::setup_otlp(&args.service_name) {
            error!(error = %err, "Failed to setup OTLP");
        }
    }

    let cancel_token = CancellationToken::new();
    let Some(service) = ZkProofService::create(conf, cancel_token.child_token()).await else {
        error!("Failed to create zkproof service");
        std::process::exit(1);
    };

    let service = Arc::new(service);

    let http_server = HttpServer::new(
        service.clone(),
        args.health_check_port,
        cancel_token.child_token(),
    );

    let http_task = task::spawn(async move {
        if let Err(err) = http_server.start().await {
            error!(
                task = "health_check",
                error = %err,
                "Error while running server"
            );
        }
        anyhow::Ok(())
    });

    // Start metrics server
    metrics_server::spawn(args.metrics_addr.clone(), cancel_token.child_token());

    let service_task = async {
        info!("Starting worker...");
        if let Err(err) = service.run().await {
            error!(error = %err, "Worker failed");
        }
        Ok::<_, anyhow::Error>(())
    };

    let (_http_result, _service_result) = join!(http_task, service_task);
}
