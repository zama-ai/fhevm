use fhevm_engine_common::telemetry;
use sns_executor::{compute_128bit_ct, process_s3_uploads, Config, DBConfig, HandleItem, S3Config};
use tokio::{signal::unix, spawn, sync::mpsc};
use tokio_util::sync::CancellationToken;
use tracing::{error, Level};
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

    let db_url = args
        .database_url
        .clone()
        .unwrap_or_else(|| std::env::var("DATABASE_URL").expect("DATABASE_URL is undefined"));

    Config {
        tenant_api_key: args.tenant_api_key,
        service_name: args.service_name,
        db: DBConfig {
            url: db_url,
            listen_channels: args.pg_listen_channels,
            notify_channel: args.pg_notify_channel,
            batch_limit: args.work_items_batch_size,
            polling_interval: args.pg_polling_interval,
            max_connections: args.pg_pool_connections,
        },
        s3: S3Config {
            bucket_ct128: args.bucket_name_ct128,
            bucket_ct64: args.bucket_name_ct64,
            max_concurrent_uploads: args.max_concurrent_uploads,
            retry_policy: S3Config::default().retry_policy, // TODO:
        },
    }
}

#[tokio::main]
async fn main() {
    let config: Config = construct_config();
    let parent = CancellationToken::new();

    tracing_subscriber::fmt()
        .json()
        .with_level(true)
        .with_max_level(Level::INFO)
        .init();

    // Handle SIGINIT signals
    handle_sigint(parent.clone());

    // Queue of tasks to upload ciphertexts is 10 times the number of concurrent uploads
    // to avoid blocking the worker
    // and to allow for some burst of uploads
    let (uploads_tx, uploads_rx) =
        mpsc::channel::<HandleItem>(10 * config.s3.max_concurrent_uploads as usize);

    if let Err(err) = telemetry::setup_otlp(&config.service_name) {
        panic!("Error while initializing tracing: {:?}", err);
    }

    let conf = config.clone();
    let token = parent.child_token();
    let tx = uploads_tx.clone();
    spawn(async move {
        if let Err(err) = process_s3_uploads(&conf, uploads_rx, tx, token).await {
            error!("Failed to run the upload-worker : {:?}", err);
        }
    });

    // Start the SnS worker

    let conf = config.clone();
    let token = parent.child_token();
    if let Err(err) = compute_128bit_ct(&conf, uploads_tx, token).await {
        error!("SnS worker failed: {:?}", err);
    }
}
