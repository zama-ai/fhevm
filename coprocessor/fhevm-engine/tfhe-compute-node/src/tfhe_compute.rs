use crate::cli::Args;
use crate::context::Context;
use crate::{CiphertextInfo, ComputeError, Execution, CONSUMER_OVERHEAD};
use fhevm_engine_common::common::FheOperation;
use fhevm_engine_common::protocol::messages::{ExecutablePartition, OpNode};
use fhevm_engine_common::tenant_keys::FetchTenantKeyResult;
use fhevm_engine_common::tfhe_ops;
use fhevm_engine_common::types::{Handle, SupportedFheCiphertexts, SupportedFheOperations};

use message_broker::{MessageResult, Receiver};
use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::{Duration, Instant};
use tfhe::ReRandomizationContext;
use tokio::task::JoinSet;

use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

pub async fn run_tfhe_compute(
    args: Args,
    cancel_token: CancellationToken,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Starting tfhe-compute-node service");

    loop {
        if let Err(cycle_error) = tfhe_compute_cycle(&args, cancel_token.clone()).await {
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

            res = receiver.recv_and_handle(async |partition: ExecutablePartition, payload_raw, _state| {
                info!("Received message on local queue");
                if let Err(_) = process_delivery(&ctx, partition, payload_raw).await {
                    // TODO: Distinguish between retryable and non-retryable errors
                    Ok(MessageResult::Nack(true, 1)) // requeue the message for retry
                } else {
                    Ok(MessageResult::Ack)
                }
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
    let consumer_tag = format!("tfhe_compute_node_{}", uuid::Uuid::new_v4());
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

async fn process_delivery(
    ctx: &Context,
    partition: ExecutablePartition,
    payload: Vec<u8>,
) -> Result<(), ComputeError> {
    let exec = Execution {
        received_at: Instant::now(),
        partition_id: hex::encode(partition.hash),
    };

    info!(delivery = ?exec, "Received FHE partition for execution");

    prepare_and_execute(ctx.clone(), partition, exec.received_at)
        .await
        .inspect_err(|err| {
            error!(error = ?err, exec = ?exec, "Error executing partition");
        })?;

    ctx.send_partition_complete(payload).await.map_err(|err| {
        error!(error = ?err, exec = ?exec, "Error sending partition completion");
        ComputeError::Other(format!(
            "Failed to send partition completion message: {err}"
        ))
    })?;

    Ok(())
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
    let pid = partition.id();
    let _otel_ctx = ctx.get_otel_ctx();

    // Elapsed time since message was received until the start of execution.
    // Should be as low as possible
    let elapsed = received_at.elapsed().as_secs_f64();

    info!(pid, elapsed, "Executing partition with scheduler");
    CONSUMER_OVERHEAD.observe(elapsed);

    let cts = execute_partition(&ctx, partition).await?;

    let count = cts.len();
    ctx.batch_store_ciphertexts(cts).await?;
    info!(pid, count, "Stored computed ciphertexts (data-layer)");

    Ok(())
}

/// Checks the operand types for the given operation node and its dependent inputs.
/// This is a wrapper around tfhe_ops::check_fhe_operand_types
fn check_fhe_operand_types(op_node: &OpNode, inputs: &[Vec<u8>]) -> Result<(), ComputeError> {
    let mut this_comp_inputs: Vec<Vec<u8>> = Vec::new();
    let mut is_scalar_op_vec: Vec<bool> = Vec::new();

    let non_scalar_inputs: Vec<(fhevm_engine_common::types::Handle, Vec<u8>)> = inputs
        .iter()
        .map(|handle| (handle.clone(), Vec::new()))
        .collect();

    let scalar_inputs = op_node
        .scalar_operands
        .iter()
        .map(|s| (vec![0u8; 32], s.clone()))
        .collect::<Vec<_>>();
    let is_scalar = !scalar_inputs.is_empty();

    let mut inputs = non_scalar_inputs;
    inputs.extend_from_slice(&scalar_inputs);

    let fhe_op = op_node.fhe_operation;
    for (idx, (handle, dh)) in inputs.iter().enumerate() {
        let is_operand_scalar = is_scalar && idx == 1 || fhe_op.does_have_more_than_one_scalar();

        is_scalar_op_vec.push(is_operand_scalar);
        if is_operand_scalar {
            this_comp_inputs.push(dh.clone());
        } else {
            this_comp_inputs.push(handle.clone());
        }
    }

    tfhe_ops::check_fhe_operand_types(fhe_op as i32, &this_comp_inputs, &is_scalar_op_vec)
        .map_err(|err| ComputeError::Other(format!("Operand type check failed: {err}")))?;

    Ok(())
}

/// Concurrently retrieves input ciphertexts for all FheGetInputCiphertext nodes in the partition and stores them in the local cache.
async fn get_input_ciphertexts(
    ctx: &Context,
    partition: &ExecutablePartition,
) -> Result<HashMap<Handle, CiphertextInfo>, ComputeError> {
    let mut tasks = JoinSet::new();
    let pid = partition.id();

    for (op_node, _, _) in partition.computations.iter() {
        // Process only SupportedFheOperations::FheGetInputCiphertext
        if op_node.fhe_operation != SupportedFheOperations::FheGetInputCiphertext {
            continue;
        }

        let ctx = ctx.clone();
        let output_handle = op_node.output_handle.clone();
        let op = op_node.fhe_operation;
        let pid = pid.clone();

        tasks.spawn(async move {
            let handle = hex_handle(&output_handle);

            info!(
                pid = pid,
                output_handle = ?handle,
                op = ?op,
                tag = "operation",
                "Fetching FheGetInputCiphertext input"
            );

            let ct = ctx.retrieve_ciphertext(&output_handle, Some(op)).await?;
            ctx.cache_store(&ct).await;

            info!(
                pid = pid,
                output_handle = ?handle,
                "Stored input ciphertext in local cache"
            );

            Ok::<CiphertextInfo, ComputeError>(ct)
        });
    }

    let mut cts = HashMap::new();

    while let Some(join_result) = tasks.join_next().await {
        let ct = join_result.map_err(ComputeError::from)??;
        cts.insert(ct.handle.clone(), ct);
    }

    Ok(cts)
}

/// Executes the computations in the partition sequentially
/// Partition is expected to be topologically sorted, so that inputs for each node are available by the time we execute it.
/// Dispatcher must guarantee that this is the case when it constructs the partition.
async fn execute_partition(
    ctx: &Context,
    partition: ExecutablePartition,
) -> Result<HashMap<Handle, CiphertextInfo>, ComputeError> {
    let keys: FetchTenantKeyResult = ctx.get_current_key().await?;
    let pid = partition.id();

    // Validate operand types for every node in the partition before executing any of
    // them, so we fail fast on unexpected input types.
    partition
        .computations
        .iter()
        .try_for_each(|(op_node, _, inputs)| {
            let handle = hex_handle(&op_node.output_handle);

            info!(
                pid = pid,
                output_handle = ?handle,
                op = ?op_node.fhe_operation,
                "Checking FHE operands"
            );

            check_fhe_operand_types(op_node, inputs).map_err(|err| {
                error!(
                    pid = pid,
                    output_handle = ?handle,
                    op = ?op_node.fhe_operation,
                    error = ?err,
                    "Operand type check failed for node in partition"
                );
                err
            })
        })?;

    // Make sure all input ciphertexts for FheGetInputCiphertext nodes are retrieved and stored in the local cache before
    // executing any of the nodes in the partition.
    let mut cts = get_input_ciphertexts(ctx, &partition).await?;

    // Compute all fhe operations in the partition sequentially as they are expected to be topologically sorted
    // Later, this should be a ComputeGraph instead without assuming any particular order and with more parallelism within the partition execution.
    for (op_node, dfg_idx, inputs) in &partition.computations {
        // Skip FheGetInputCiphertext nodes, their ciphertexts were already fetched into the cache
        if op_node.fhe_operation == SupportedFheOperations::FheGetInputCiphertext {
            continue;
        }

        let handle_as_hex = hex_handle(&op_node.output_handle);
        info!(
            pid = pid,
            output_handle = ?handle_as_hex,
            op = ?op_node.fhe_operation,
            "Executing fhe operation"
        );

        // retrieve inputs (input ciphertexts) from either local cache or DataLayer
        let dependence_inputs = futures_util::future::try_join_all(
            inputs
                .iter()
                .map(|handle| ctx.retrieve_ciphertext(handle, Some(op_node.fhe_operation))),
        )
        .await?;

        // scalar operands are sent along with the partition execution message
        let scalar_inputs = op_node
            .scalar_operands
            .iter()
            .map(|s| SupportedFheCiphertexts::Scalar(s.clone()))
            .collect::<Vec<_>>();

        let mut inputs: Vec<SupportedFheCiphertexts> = dependence_inputs
            .iter()
            .map(|ci| ci.ciphertext.clone())
            .collect::<Vec<_>>();

        // Re-randomise the ciphertexts before performing any operation on them.
        let rerand_seed = [
            op_node.block_info.txn_hash.as_slice(),
            &dfg_idx.index().to_be_bytes(),
        ];

        tfhe::set_server_key(keys.server_key.clone());
        re_randomise_inputs(&mut inputs, rerand_seed, keys.pks.clone())?;

        inputs.extend(scalar_inputs);

        info!(
            pid = pid,
            output_handle = ?handle_as_hex,
            op = ?op_node.fhe_operation,
            inputs = ?dependence_inputs.iter().map(|i| hex::encode(&i.handle)).collect::<Vec<_>>(),
            "Fetched input ciphertexts for node in partition"
        );

        let ct = try_run_computation(
            pid.as_str(),
            op_node,
            inputs,
            false,
            0,
            keys.client_key.clone(),
        )?;

        ctx.cache_store(&ct).await;
        info!(
            pid = pid,
            handle_as_hex, "Stored computed ciphertext in cache"
        );

        cts.insert(ct.handle.clone(), ct);
    }

    Ok(cts)
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
        pid = partition_id,
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
        handle: op_node.output_handle().clone(),
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

    info!(op_name, gpu_idx, tag = "operation", "Compute FHE operation");
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

#[inline]
fn hex_handle(handle: &[u8]) -> String {
    hex::encode(&handle[..handle.len().min(4)])
}
