use ::tracing::{error, info};
use fhevm_engine_common::types::Handle;
use lapin::options::BasicRejectOptions;
use tracing::debug;

use fhevm_engine_common::telemetry;
use fhevm_engine_common::types::{FhevmError, SupportedFheCiphertexts};
use prometheus::{Histogram, IntCounter, register_histogram, register_int_counter};
use serde::{Deserialize, Serialize};
use tokio_util::sync::CancellationToken;

use std::sync::Once;
use std::sync::atomic::Ordering;
use std::time::Instant;
use thiserror::Error;

pub mod cli;
pub mod context;
pub mod tfhe_compute;

#[derive(Error, Debug)]
pub enum ComputeError {
    #[error("FHEVM error: {0}")]
    Fhevm(#[from] FhevmError),
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
    #[error("Rerand error: {0}")]
    Rerand(String),
    #[error("Missing current key id in context")]
    MissingKeyId,
    #[error("Unexpected EOF: {0}")]
    UnexpectedEof(String),
    #[error("{0}")]
    Other(String),
}

#[derive(Clone)]
struct CiphertextInfo {
    handle: Handle,
    ciphertext: SupportedFheCiphertexts,
}

#[derive(Debug, PartialEq)]
struct Execution {
    partition_id: String,

    // Delivery info from lapin, needed for ack/nack
    delivery: lapin::message::Delivery,
    received_at: Instant,
    is_local: bool,
}

impl Execution {
    async fn ack(&self) -> Result<bool, lapin::Error> {
        let res = self
            .delivery
            .ack(lapin::options::BasicAckOptions::default())
            .await?;

        debug!(
            routing_key = ?self.delivery.routing_key,
            "Acknowledged message"
        );

        Ok(res)
    }

    fn begin(&self) {
        let tc = RUNNING_TASKS.fetch_add(1, Ordering::Relaxed);
        let pid = self.partition_id.clone();
        info!(pid = pid, tc = tc, "Starting FHE partition execution");
    }

    async fn end(&self, transient_error: bool) {
        if transient_error {
            let _headers = self.delivery.properties.headers();
            // TODO: use headers to decrement a retry counter
            let _ = self
                .delivery
                .reject(BasicRejectOptions {
                    requeue: false, /* TODO true */
                })
                .await;
        } else {
            let _ = self.ack().await;
        }

        let pid = self.partition_id.clone();
        if transient_error {
            error!(pid = pid, "FHE partition execution failed");
        } else {
            info!(pid = pid, "Completed FHE partition execution");
        }

        RUNNING_TASKS.fetch_sub(1, Ordering::SeqCst);
    }
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

// Used for testing as we would call `async_main()` multiple times.
static TRACING_INIT: Once = Once::new();

pub async fn async_main(
    args: cli::Args,
    cancel_token: CancellationToken,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    TRACING_INIT.call_once(|| {
        tracing_subscriber::fmt()
            .json()
            .with_level(true)
            .with_max_level(args.log_level)
            .init();
    });

    info!(target: "async_main", args = ?args, "Starting runtime with args");

    if !args.service_name.is_empty() {
        if let Err(err) = telemetry::setup_otlp(&args.service_name) {
            error!(error = %err, "Failed to setup OTLP");
        }
    }

    let gpu_enabled = fhevm_engine_common::utils::log_backend();
    info!(target: "async_main", gpu_enabled,  "Initializing tfhe-compute-node");

    tfhe_compute::run_tfhe_compute(args.clone(), cancel_token).await?;
    Ok(())
}
