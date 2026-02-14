use ::tracing::{error, info};

use fhevm_engine_common::types::SupportedFheCiphertexts;
use fhevm_engine_common::{metrics_server, telemetry};
use prometheus::{register_histogram, register_int_counter, Histogram, IntCounter};
use serde::{Deserialize, Serialize};
use tokio_util::sync::CancellationToken;

use std::sync::Once;
use std::time::Instant;
use thiserror::Error;
use tokio::task::JoinSet;

pub mod cli;
pub mod context;
pub mod tfhe_compute;

#[derive(Error, Debug)]
pub enum ComputeError {
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("Postgres error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("RabbitMQ error: {0}")]
    Lapin(#[from] lapin::Error),
    #[error("Task join error: {0}")]
    Join(#[from] tokio::task::JoinError),
    #[error("Postcard serialization error: {0}")]
    Postcard(#[from] postcard::Error),
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("TFHE error: {0}")]
    Tfhe(String),
    #[error("Missing current key id in context")]
    MissingKeyId,
    #[error("Unexpected EOF: {0}")]
    UnexpectedEof(String),
    #[error("{0}")]
    Other(String),
}

#[derive(Debug, Default, Deserialize)]
struct FheTask {
    key_id: i32, // tenant id in old  impl
    partition_id: i32,
}

#[derive(Clone)]
struct CiphertextInfo {
    handle: Vec<u8>,
    ciphertext: SupportedFheCiphertexts,
}

#[derive(Debug, PartialEq)]
struct DeliveryInfo {
    inner: lapin::message::Delivery,
    received_at: Instant,
}

#[derive(Clone, Deserialize, Serialize)]
struct RedisCiphertextRecord {
    ct_type: i16,
    raw_ct: Option<Vec<u8>>,
    compressed_ct: Option<Vec<u8>>,
}

lazy_static::lazy_static! {
    /// CONSUMER_OVERHEAD is mainly due to Cache miss and Redis retrieval,
    /// which are expected to be the main contributors to the time elapsed between message received and start of execution.
    static ref CONSUMER_OVERHEAD: Histogram = register_histogram!(
        "compute_node_consumer_overhead_seconds",
         "Time elapsed between message received and start of execution"
    )
    .unwrap();

    static ref REDIS_SUB_OVERHEAD: Histogram = register_histogram!(
        "compute_node_redis_sub_overhead_seconds",
         "Time elapsed between subscribing to Redis keyspace notification and receiving the notification"
    )
    .unwrap();

    static ref REDIS_BATCH_STORE_OVERHEAD: Histogram = register_histogram!(
        "compute_node_redis_batch_store_overhead_seconds",
         "Time elapsed for batch storing computed ciphertexts in Redis"
    )
    .unwrap();

    static ref CACHE_HITS_COUNTER: IntCounter = register_int_counter!(
        "compute_node_cache_hits_total",
        "Total number of cache hits for ciphertext retrieval"
    )
    .unwrap();
    static ref REDIS_HITS_COUNTER: IntCounter = register_int_counter!(
        "compute_node_redis_hits_total",
        "Total number of redis hits for ciphertext retrieval"
    )
    .unwrap();
    static ref REDIS_SUB_COUNTER: IntCounter = register_int_counter!(
        "compute_node_redis_sub_total",
        "Total number of redis subscriptions for ciphertext retrieval"
    )
    .unwrap();


}

lazy_static::lazy_static! {
    pub static ref RUNNING_TASKS: std::sync::atomic::AtomicU64 =
        std::sync::atomic::AtomicU64::new(0);
}
// separate function for testing
pub fn start_runtime(args: cli::Args, close_recv: Option<tokio::sync::watch::Receiver<bool>>) {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(args.tokio_threads)
        // not using tokio main to specify max blocking threads
        .max_blocking_threads(args.blocking_fhe_threads)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            if let Some(mut close_recv) = close_recv {
                tokio::select! {
                    main = async_main(args) => {
                        if let Err(e) = main {
                            error!(target: "main_wchannel", { error = e }, "Runtime error");
                        }
                    }
                    _ = close_recv.changed() => {
                        info!(target: "main_wchannel", "Service stopped voluntarily");
                    }
                }
            } else if let Err(e) = async_main(args).await {
                error!(target: "main", { error = e }, "Runtime error");
            }
        })
}

// Used for testing as we would call `async_main()` multiple times.
static TRACING_INIT: Once = Once::new();

pub async fn async_main(args: cli::Args) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    TRACING_INIT.call_once(|| {
        tracing_subscriber::fmt()
            .json()
            .with_level(true)
            .with_max_level(args.log_level)
            .init();
    });

    let cancel_token = CancellationToken::new();
    info!(target: "async_main", args = ?args, "Starting runtime with args");

    if !args.service_name.is_empty() {
        if let Err(err) = telemetry::setup_otlp(&args.service_name) {
            error!(error = %err, "Failed to setup OTLP");
        }
    }

    let mut set = JoinSet::new();

    let gpu_enabled = fhevm_engine_common::utils::log_backend();
    info!(target: "async_main", gpu_enabled,  "Initializing compute-node");

    set.spawn(tfhe_compute::run_tfhe_compute(args.clone()));

    let metrics_addr = args.metrics_addr.clone();
    if let Some(fut) = metrics_server::metrics_future(metrics_addr, cancel_token.child_token()) {
        set.spawn(async {
            fut.await;
            Ok(())
        });
    }

    if set.is_empty() {
        panic!("No tasks specified to run");
    }

    while let Some(res) = set.join_next().await {
        if let Err(e) = res {
            panic!("Error background initializing worker: {:?}", e);
        }
    }

    Ok(())
}
