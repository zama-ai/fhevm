use crate::cli::Args;
use crate::context::Context;
use crate::{CiphertextInfo, ComputeError, Execution, CONSUMER_OVERHEAD};
use fhevm_engine_common::common::FheOperation;
use fhevm_engine_common::rmq_utils::{create_recv_channel, extract_delivery, try_decode};
use fhevm_engine_common::tenant_keys::FetchTenantKeyResult;
use fhevm_engine_common::tfhe_ops::perform_fhe_operation;
use fhevm_engine_common::types::{SupportedFheCiphertexts, SupportedFheOperations};
use futures_util::stream::StreamExt;
use lapin::message::Delivery;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use std::time::{Duration, Instant, SystemTime};
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

pub async fn run_tfhe_compute(
    args: Args,
    cancel_token: CancellationToken,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Determine worker ID to use for the lifetime of this process
    // In case of a failure in tfhe_compute_cycle, the same id must be reused to quickly unlock any held locks
    let id = args.worker_id.unwrap_or(Uuid::new_v4());
    info!( id = %id, "Starting tfhe-compute-node service");

    loop {
        // here we log the errors and make sure we retry
        if let Err(cycle_error) = tfhe_compute_cycle(&args, id, cancel_token.clone()).await {
            error!( { error = ?cycle_error }, "Error in background worker, retrying shortly");
        }

        if cancel_token.is_cancelled() {
            info!("Cancellation requested, not restarting compute cycle");
            return Ok(());
        }

        tokio::time::sleep(Duration::from_millis(5000)).await;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Pending { remaining_deps: usize },
    Computing { started_at: SystemTime },
    Computed { finished_at: SystemTime },
    Malformed { error_code: u8 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockContext {
    pub txn_hash: [u8; 32],
    pub block_number: u64,
    pub block_hash: [u8; 32],
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ComputationNode {
    pub key_id: u64,
    pub output_handle: Vec<u8>,
    pub fhe_operation: SupportedFheOperations,
    pub is_scalar: bool,
    pub created_at: SystemTime,
    pub status: Status,
    pub block_info: BlockContext,
}

// TODO:
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PartitionMessage {
    pub hash: [u8; 32],
    pub exec_node_idx: usize,
    pub created_at: usize,
    pub key_id: i32,

    pub input_handles: Vec<(i16, Vec<u8>)>,
    pub computations: Vec<ComputationNode>,
}

impl PartitionMessage {
    fn id(&self) -> String {
        hex::encode(self.hash)
    }
}

/// The main compute cycle for the TFHE compute node worker
async fn tfhe_compute_cycle(
    args: &Args,
    worker_id: Uuid,
    cancel_token: CancellationToken,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db_url = args.database_url.clone().unwrap_or_default();

    let ctx = Context::create(
        db_url.as_str(),
        &args.redis_url,
        args.rmq_uri.as_str(),
        args.tenant_key_cache_size,
        args.ciphertext_cache_size,
    )
    .await?;

    let node_name = format!("tfhe_compute_node_{}", worker_id);

    // Never deliver more than prefetch_count un-acked messages to that worker
    // Thus, automatically route jobs to workers with available capacity
    let prefetch_count = 4u16; // TODO:
    let mut shared_queue = create_recv_channel(
        &args.rmq_uri,
        &node_name,
        "shared_tfhe_queue",
        prefetch_count,
    )
    .await?;

    let prefetch_count = 1u16; // TODO:
    let mut local_queue = create_recv_channel(
        &args.rmq_uri,
        &node_name,
        "local_tfhe_queue",
        prefetch_count,
    )
    .await?;

    let mut set: JoinSet<()> = JoinSet::new();

    'outer: loop {
        tokio::select! {
            biased;
            // prioritize local queue
            res = local_queue.next() => {
                let delivery = extract_delivery(res, "local")?;
                let Some(partition) = try_decode::<PartitionMessage>(&delivery).await else {
                    continue;
                };

                process_delivery(&ctx, &mut set, delivery, partition, true).await;

            },
            res = shared_queue.next() => {
                let delivery = extract_delivery(res, "shared")?;
                let Some(partition) = try_decode::<PartitionMessage>(&delivery).await else {
                    continue;
                };
                process_delivery(&ctx, &mut set, delivery, partition, false).await;
            },
            _ = cancel_token.cancelled() => {
                info!( "Cancellation requested, shutting down compute cycle");
                break 'outer;
            },
            // TODO idle timeout to do health check ping
        };
    }

    set.shutdown().await;
    Ok(())
}

async fn process_delivery(
    ctx: &Context,
    set: &mut JoinSet<()>,
    delivery: Delivery,
    partition: PartitionMessage,
    is_local: bool,
) {
    let exec = Execution {
        delivery,
        received_at: Instant::now(),
        is_local,
        partition_id: hex::encode(partition.hash),
    };

    info!(delivery = ?exec, is_local, "Received FHE partition for execution");

    let ctx = ctx.clone();
    set.spawn(async move {
        exec.begin();

        let res = match prepare_and_execute(ctx.clone(), partition, exec.received_at).await {
            Ok(_) => {
                let payload = exec.delivery.data.clone();
                ctx.send_partition_complete(payload).await.is_ok()
            }
            Err(err) => {
                // Errors will cause the partition execution to be retried by this or another compute-node.
                error!(error = ?err, exec = ?exec, "Error executing partition");
                false
            }
        };

        exec.end(res).await;
    });
}

/// Prepares the context and executes the given partition, handling caching and Redis storage of results.
///
/// Returns error only on transient failures.
async fn prepare_and_execute(
    ctx: Context,
    partition: PartitionMessage,
    received_at: Instant,
) -> Result<(), ComputeError> {
    ctx.set_key_id(partition.key_id as i64).await;

    let keys: FetchTenantKeyResult = ctx.get_current_key().await?;
    let pid = partition.id();
    let _otel_ctx = ctx.get_otel_ctx();

    // query inputs (input ciphertexts) from either Redis or local cache,
    // or wait for them to be available if not present yet
    //
    // TODO: For backwards compatibility,
    // we should also try to fetch from Postgres if not found in Redis/cache
    let input_handles = partition.input_handles.clone();
    futures_util::future::try_join_all(
        input_handles
            .iter()
            .map(|(_ct_type, ct_data)| ctx.get_or_wait_for_handle(ct_data)),
    )
    .await?;

    // convert input handles to ciphertexts for execution
    let mut inputs = Vec::new();
    for (ct_type, ct_data) in &partition.input_handles {
        let ct = SupportedFheCiphertexts::deserialize(*ct_type, ct_data).unwrap();
        inputs.push(ct);
    }

    tfhe::set_server_key(keys.server_key);

    let exec_result = tokio::task::spawn_blocking(move || {
        // Elapsed time since message was received until the start of execution.
        // Should be as low as possible
        let elapsed = received_at.elapsed().as_secs_f64();
        let pid = partition.id();

        info!(pid, elapsed, "Executing partition with scheduler");
        CONSUMER_OVERHEAD.observe(elapsed);

        execute_partition(partition, inputs).unwrap()
    })
    .await?;

    try_store_ciphertext(&ctx, pid, exec_result).await?;

    Ok(())
}

async fn try_store_ciphertext(
    ctx: &Context,
    pid: String,
    ciphertexts: Vec<CiphertextInfo>,
) -> Result<(), ComputeError> {
    for ct in ciphertexts.iter() {
        ctx.cache_store(&ct).await;
        info!(pid, handle = %hex::encode(&ct.handle), "Stored computed ciphertext in cache");
    }

    // batch store
    let count = ciphertexts.len();
    ctx.batch_store_ciphertexts(ciphertexts).await?;

    info!(pid, count, "Stored computed ciphertexts (data-layer)");

    Ok(())
}

/// Executes the computations in the partition sequentially
/// Partition is expected to be topologically sorted, so that inputs for each node are available by the time we execute it.
/// Dispatcher must guarantee that this is the case when it constructs the partition.
fn execute_partition(
    partition: PartitionMessage,
    mut inputs: Vec<SupportedFheCiphertexts>,
) -> Result<Vec<CiphertextInfo>, ComputeError> {
    // TODO: rerandomization will be implemented pending clarification

    let mut computed_cts = Vec::new();

    let gpu_index = 0; // TODO: determine GPU index to use for this partition execution, if GPU is enabled

    for node in &partition.computations {
        info!(
            pid = partition.id(),
            handle = %hex::encode(&node.output_handle),
            op = ?node.fhe_operation,
            "Executing node in partition"
        );

        let result = try_execute_fhe_operation(node, inputs.clone(), false, gpu_index)?;
        let ct = result.0.clone();
        inputs.push(ct);

        info!(
            pid = partition.id(),
            handle = %hex::encode(&node.output_handle),
            "Executed node in partition"
        );

        computed_cts.push(CiphertextInfo {
            handle: node.output_handle.clone(),
            ciphertext: result.0,
        });
    }

    Ok(computed_cts)
}

fn try_execute_fhe_operation(
    node: &ComputationNode,
    inputs: Vec<SupportedFheCiphertexts>,
    compress_result: bool,
    gpu_index: usize,
) -> Result<OpResult, ComputeError> {
    let opcode = node.fhe_operation as i32;
    run_computation(opcode, inputs, compress_result, gpu_index)
}

pub type OpResult = (SupportedFheCiphertexts, Option<(i16, Vec<u8>)>);
pub fn run_computation(
    operation: i32,
    inputs: Vec<SupportedFheCiphertexts>,
    compress_result: bool,
    gpu_idx: usize,
) -> Result<OpResult, ComputeError> {
    let op = FheOperation::try_from(operation);

    match op {
        Ok(FheOperation::FheGetCiphertext) => {
            let (ct_type, ct_bytes) = inputs[0].compress();
            Ok((inputs[0].clone(), Some((ct_type, ct_bytes))))
        }

        Ok(_) => match perform_fhe_operation(operation as i16, &inputs, gpu_idx) {
            Ok(result) => {
                if compress_result {
                    let (ct_type, ct_bytes) = result.compress();
                    Ok((result, Some((ct_type, ct_bytes))))
                } else {
                    Ok((result, None))
                }
            }
            Err(e) => Err(e.into()),
        },

        Err(e) => Err(ComputeError::Other(e.to_string())),
    }
}
