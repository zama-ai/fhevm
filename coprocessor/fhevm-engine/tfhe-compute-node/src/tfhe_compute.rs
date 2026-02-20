use crate::cli::Args;
use crate::context::Context;
use crate::{CiphertextInfo, ComputeError, Execution, CONSUMER_OVERHEAD};
use fhevm_engine_common::common::FheOperation;
use fhevm_engine_common::msg_broker::{create_recv_channel, extract_delivery, try_decode};
use fhevm_engine_common::protocol::messages::{ExecutablePartition, OpNode};
use fhevm_engine_common::tenant_keys::FetchTenantKeyResult;
use fhevm_engine_common::tfhe_ops::perform_fhe_operation;
use fhevm_engine_common::types::SupportedFheCiphertexts;
use futures_util::stream::StreamExt;
use lapin::message::Delivery;
use sqlx::types::Uuid;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::{Duration, Instant};
use tfhe::ReRandomizationContext;
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
        "queue_fhe_partitions",
        prefetch_count,
    )
    .await?;

    let prefetch_count = 1u16; // TODO:
    let mut local_queue = create_recv_channel(
        &args.rmq_uri,
        &node_name,
        "queue_fhe_partitions_local",
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
                let Some(partition) = try_decode::<ExecutablePartition>(&delivery).await else {
                    continue;
                };

                process_delivery(&ctx, &mut set, delivery, partition, true).await;

            },
            res = shared_queue.next() => {
                info!("Received message on shared queue");

                let delivery = extract_delivery(res, "shared")?;
                let Some(partition) = try_decode::<ExecutablePartition>(&delivery).await else {
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
    partition: ExecutablePartition,
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
                !ctx.send_partition_complete(payload).await.is_ok()
            }
            Err(err) => {
                // Errors will cause the partition execution to be retried by this or another compute-node.
                error!(error = ?err, exec = ?exec, "Error executing partition");
                true
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
    partition: ExecutablePartition,
    received_at: Instant,
) -> Result<(), ComputeError> {
    ctx.set_key_id(partition.key_id as i64).await;

    info!(
        pid = partition.id(),
        key_id = partition.key_id,
        "Set context key id for partition execution"
    );

    info!(
        pid = partition.id(),
        key_id = partition.key_id,
        "Fetched tenant keys for partition execution"
    );

    let _otel_ctx = ctx.get_otel_ctx();

    // Elapsed time since message was received until the start of execution.
    // Should be as low as possible
    let elapsed = received_at.elapsed().as_secs_f64();
    let pid = partition.id();

    info!(pid, elapsed, "Executing partition with scheduler");
    CONSUMER_OVERHEAD.observe(elapsed);

    let exec_result = execute_partition(&ctx, partition).await?;

    let count = exec_result.len();
    ctx.batch_store_ciphertexts(exec_result).await?;
    info!(pid, count, "Stored computed ciphertexts (data-layer)");

    Ok(())
}

/// Executes the computations in the partition sequentially
/// Partition is expected to be topologically sorted, so that inputs for each node are available by the time we execute it.
/// Dispatcher must guarantee that this is the case when it constructs the partition.
async fn execute_partition(
    ctx: &Context,
    partition: ExecutablePartition,
) -> Result<Vec<CiphertextInfo>, ComputeError> {
    // TODO: determine GPU index to use for this partition execution, if GPU is enabled
    let keys: FetchTenantKeyResult = ctx.get_current_key().await?;

    let mut computed_cts = Vec::new();
    for (op_node, dfg_idx, inputs) in &partition.computations {
        let handle = hex::encode(&op_node.output_handle[0..4]);
        info!(
            pid = partition.id(),
            output_handle = ?handle,
            op = ?op_node.fhe_operation,
            "Executing node in partition"
        );

        // query inputs (input ciphertexts) from either Redis or local cache,
        // or wait for them to be available if not present yet
        //
        // TODO: For backwards compatibility,
        // we should also try to fetch from Postgres if not found in Redis/cache
        let dependence_inputs = futures_util::future::try_join_all(
            inputs
                .iter()
                .map(|handle| ctx.get_or_wait_for_handle(handle)),
        )
        .await?;

        tfhe::set_server_key(keys.server_key.clone());

        // scalar operands are sent along with the partition execution message
        let scalar_inputs = op_node
            .scalar_operands
            .iter()
            .map(|s| SupportedFheCiphertexts::Scalar(s.clone()))
            .collect::<Vec<_>>();

        let mut inputs: Vec<SupportedFheCiphertexts> = dependence_inputs
            .iter()
            .map(|ci| ci.ciphertext.clone())
            .collect();

        // Re-randomise the ciphertexts before performing any operation on them.
        let rerand_seed = [
            op_node.block_info.txn_hash.as_slice(),
            &dfg_idx.index().to_be_bytes(),
        ];
        re_randomise_inputs(&mut inputs, rerand_seed, keys.pks.clone())?;

        inputs.extend(scalar_inputs);

        info!(
            pid = partition.id(),
            output_handle = ?handle,
            op = ?op_node.fhe_operation,
            inputs = ?dependence_inputs.iter().map(|i| hex::encode(&i.handle)).collect::<Vec<_>>(),
            "Fetched input ciphertexts for node in partition"
        );

        let result = try_execute_fhe_operation(op_node, inputs, false, 0)?;

        info!(
            pid = partition.id(),
            handle = ?handle,
             op = ?op_node.fhe_operation,
             "Completed execution of node in partition"
        );

        if let Some(client_key) = ctx.get_current_key().await?.client_key {
            let plaintext = result.0.decrypt(&client_key);

            // Print plaintext, useful for testing and debugging
            info!(
                pid = partition.id(),
                handle = ?handle,
                plaintext = ?plaintext,
                "Decrypted result"
            );
        }

        let ct = CiphertextInfo {
            handle: op_node.output_handle(),
            ciphertext: result.0,
        };

        ctx.cache_store(&ct).await;
        info!(
            pid = partition.id(),
            handle, "Stored computed ciphertext in cache"
        );

        computed_cts.push(ct);
    }

    Ok(computed_cts)
}

fn try_execute_fhe_operation<'a>(
    node: &OpNode,
    inputs: Vec<SupportedFheCiphertexts>,
    compress_result: bool,
    gpu_index: usize,
) -> Result<OpResult, ComputeError> {
    let opcode = node.fhe_operation as i32;

    // TODO: check_fhe_operand_types(opcode, &inputs)?;

    let result = catch_unwind(AssertUnwindSafe(|| {
        run_computation(opcode, inputs, compress_result, gpu_index)
    }));

    match result {
        Ok(value) => value,
        Err(panic_payload) => {
            // Extract panic message if possible
            let message = if let Some(s) = panic_payload.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = panic_payload.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic".to_string()
            };

            Err(ComputeError::Other(format!(
                "Panic during FHE operation execution: {}",
                message
            )))
        }
    }
}

pub type OpResult = (SupportedFheCiphertexts, Option<(i16, Vec<u8>)>);
pub fn run_computation(
    operation: i32,
    inputs: Vec<SupportedFheCiphertexts>,
    compress_result: bool,
    gpu_idx: usize,
) -> Result<OpResult, ComputeError> {
    let op = FheOperation::try_from(operation);

    let op_name = op
        .as_ref()
        .map(|o| format!("{:?}", o))
        .unwrap_or_else(|_| format!("Unknown({})", operation));

    info!(op_name, gpu_idx, "Performing FHE operation");
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

const TRANSACTION_RERANDOMISATION_DOMAIN_SEPARATOR: [u8; 8] = *b"TFHE_Rrd";
const COMPACT_PUBLIC_ENCRYPTION_DOMAIN_SEPARATOR: [u8; 8] = *b"TFHE_Enc";
fn re_randomise_inputs<'a>(
    inputs: &mut Vec<SupportedFheCiphertexts>,
    seed: impl IntoIterator<Item = &'a [u8]>,
    cpk: tfhe::CompactPublicKey,
) -> Result<(), ComputeError> {
    if inputs.is_empty() {
        return Ok(());
    }

    let mut re_rand_context = ReRandomizationContext::new(
        TRANSACTION_RERANDOMISATION_DOMAIN_SEPARATOR,
        seed,
        COMPACT_PUBLIC_ENCRYPTION_DOMAIN_SEPARATOR,
    );
    for input in inputs.iter() {
        input.add_to_re_randomization_context(&mut re_rand_context);
    }
    let mut seed_gen = re_rand_context.finalize();
    for input in inputs.iter_mut() {
        input.re_randomise(
            &cpk,
            seed_gen
                .next_seed()
                .map_err(|e| ComputeError::Rerand(e.to_string()))?,
        )?;
    }
    Ok(())
}
