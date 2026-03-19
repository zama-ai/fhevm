use crate::cli::Args;
use crate::context::Context;
use crate::{CiphertextInfo, ComputeError, Execution, CONSUMER_OVERHEAD};
use fhevm_engine_common::common::FheOperation;
use fhevm_engine_common::protocol::messages::{ExecutablePartition, OpNode};
use fhevm_engine_common::tenant_keys::FetchTenantKeyResult;
use fhevm_engine_common::tfhe_ops;
use fhevm_engine_common::types::{SupportedFheCiphertexts, SupportedFheOperations};

use message_broker::{MessageResult, Receiver};
use sqlx::types::Uuid;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::{Duration, Instant};
use tfhe::ReRandomizationContext;

use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

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
    _worker_id: Uuid,
    _cancel_token: CancellationToken,
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

    let mut receiver = create_receiver(args, "queue_fhe_partitions", ctx.clone()).await;
    loop {
        tokio::select! {
             _ = _cancel_token.cancelled() => {
                info!("Cancellation requested, stopping compute cycle");
                break;
            }

            res = receiver.recv_and_handle(async |msg: ExecutablePartition, payload_raw, _state| {
                info!("Received message on local queue");
                process_delivery(&ctx, msg.clone(), payload_raw).await;
                Ok(MessageResult::Ack)
            }) => {
                if let Err(e) = res {
                    error!(error = ?e, "Error receiving message from RabbitMQ");
                    // In case of an error in receiving messages, we break the loop to restart the connection and consumer channel
                    break;
                }
            }
        }
    }

    Ok(())
}

#[cfg(feature = "rabbitmq")]
async fn create_receiver(
    args: &Args,
    queue_name: &str,
    ctx: Context,
) -> impl Receiver<ExecutablePartition, Context> {
    let consumer_tag = format!("tfhe_compute_node_{}", Uuid::new_v4());
    message_broker::rabbitmq::RabbitMQReceiver::new(&args.rmq_uri, queue_name, &consumer_tag, ctx)
        .await
}

#[cfg(feature = "redis_stream")]
async fn create_receiver(
    args: &Args,
    queue_name: &str,
    ctx: Context,
) -> impl Receiver<ExecutablePartition, Context> {
    message_broker::redis_stream::RedisStreamReceiver::new(&args.redis_url, queue_name).await
}

async fn process_delivery(ctx: &Context, partition: ExecutablePartition, payload: Vec<u8>) {
    let exec = Execution {
        received_at: Instant::now(),
        partition_id: hex::encode(partition.hash),
    };

    info!(delivery = ?exec, "Received FHE partition for execution");

    let ctx = ctx.clone();

    let _res = match prepare_and_execute(ctx.clone(), partition, exec.received_at).await {
        Ok(_) => !ctx.send_partition_complete(payload).await.is_ok(),
        Err(err) => {
            // Errors will cause the partition execution to be retried by this or another compute-node.
            error!(error = ?err, exec = ?exec, "Error executing partition");
            true
        }
    };
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

        // query inputs (input ciphertexts) from either DataLayer or local cache,
        let dependence_inputs = futures_util::future::try_join_all(
            inputs
                .iter()
                .map(|handle| ctx.get_or_wait_for_ciphertext(handle)),
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

        let ct = match op_node.fhe_operation {
            SupportedFheOperations::FheGetInputCiphertext => {
                ctx.get_or_wait_for_ciphertext(&op_node.output_handle)
                    .await?
            }
            _ => try_run_computation(
                partition.id().as_str(),
                op_node,
                inputs,
                false,
                0,
                ctx.get_current_key().await?.client_key,
            )?,
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

/// Executes the given FHE operation node with the provided input ciphertexts.
fn try_run_computation(
    partition_id: &str,
    op_node: &OpNode,
    inputs: Vec<SupportedFheCiphertexts>,
    compress_result: bool,
    gpu_index: usize,
    client_key: Option<tfhe::ClientKey>,
) -> Result<CiphertextInfo, ComputeError> {
    let opcode = op_node.fhe_operation as i32;
    let handle = hex::encode(&op_node.output_handle[0..4]);

    // TODO: check_fhe_operand_types(opcode, &inputs)?
    let result = catch_unwind(AssertUnwindSafe(|| {
        run_computation(opcode, inputs, compress_result, gpu_index)
    }));

    let ciphertext_result = match result {
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
    }?;

    info!(
        pid  = partition_id,
        handle = ?handle,
         op = ?op_node.fhe_operation,
         "Completed execution of node in partition"
    );

    if let Some(client_key) = client_key {
        let plaintext = ciphertext_result.0.decrypt(&client_key);

        // Print plaintext, useful for testing and debugging
        info!(
            pid =  partition_id,
            handle = ?handle,
            plaintext = ?plaintext,
            "Decrypted result"
        );
    }

    Ok(CiphertextInfo {
        handle: op_node.output_handle(),
        ciphertext: ciphertext_result.0,
    })
}

pub type OpResult = (SupportedFheCiphertexts, Option<(i16, Vec<u8>)>);
fn run_computation(
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
            warn!("FheGetCiphertext operation should be handled separately");
            return Err(ComputeError::Other(
                "FheGetCiphertext not supported".to_string(),
            ));
        }

        Ok(_) => match tfhe_ops::perform_fhe_operation(operation as i16, &inputs, gpu_idx) {
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
