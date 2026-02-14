use crate::cli::Args;
use crate::context::Context;
use crate::{
    CiphertextInfo, ComputeError, DeliveryInfo, FheTask, CONSUMER_OVERHEAD, RUNNING_TASKS,
};
use fhevm_engine_common::tenant_keys::FetchTenantKeyResult;
use fhevm_engine_common::utils::create_recv_channel;
use futures_util::stream::StreamExt;
use lapin::options::*;
use scheduler::dfg::scheduler;
use sqlx::types::Uuid;
use std::sync::atomic::Ordering;
use std::time::Instant;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

pub async fn run_tfhe_compute(args: Args) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Determine worker ID to use for the lifetime of this process
    // In case of a failure in tfhe_compute_cycle, the same id must be reused to quickly unlock any held locks
    let id = args.worker_id.unwrap_or(Uuid::new_v4());
    info!( id = %id, "Starting tfhe-compute-node service");

    loop {
        // here we log the errors and make sure we retry
        if let Err(cycle_error) = tfhe_compute_cycle(&args, id).await {
            error!( { error = ?cycle_error }, "Error in background worker, retrying shortly");
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    }
}

/// The main compute cycle for the TFHE compute node worker
async fn tfhe_compute_cycle(args: &Args, worker_id: Uuid) -> Result<(), ComputeError> {
    let db_url = args.database_url.clone().unwrap_or_default();

    let ctx = Context::create(
        db_url.as_str(),
        &args.redis_url,
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
    let cancel_token = CancellationToken::new(); // TODO:

    'outer: loop {
        let mut pending_delivery = None;
        let mut is_local = false;
        let received_at;

        tokio::select! {
            biased;
            // prioritize local queue
            d = local_queue.next() => {
                received_at = Instant::now();
                if let Some(Ok(d)) = d {
                    is_local = true;
                    pending_delivery = Some(d);
                }
            },
            d = shared_queue.next() => {
                received_at = Instant::now();
                if let Some(Ok(d)) = d {
                    pending_delivery = Some(d);
                }
            },
            _ = cancel_token.cancelled() => {
                info!( "Cancellation requested, shutting down compute cycle");
                break 'outer;
            },
            // TODO idle timeout to do health check ping
        };

        let delivery = match pending_delivery {
            Some(delivery) => DeliveryInfo {
                inner: delivery,
                received_at,
            },
            None => {
                // No job received, continue to next cycle
                continue;
            }
        };

        info!(delivery = ?delivery, is_local, "Received FHE task msg");

        // Spawn blocking task to execute the partition
        if let Err(e) = spawn_partition_execution(ctx.clone(), &mut set, delivery).await {
            error!(error = ?e, "Failed to spawn partition execution");
        }
    }

    set.shutdown().await;
    Ok(())
}

async fn spawn_partition_execution(
    ctx: Context,
    set: &mut JoinSet<()>,
    delivery: DeliveryInfo,
) -> Result<(), ComputeError> {
    // Deserialize the message to get the partition information
    let task: FheTask = serde_json::from_slice::<FheTask>(&delivery.inner.data)?;
    ctx.set_key_id(task.key_id as i64).await;

    let keys: FetchTenantKeyResult = ctx.get_current_key().await?;
    let partition_id = task.partition_id;

    let _ = set.spawn( async move {
        // load atomic running_tasks counter
        let task_counter = RUNNING_TASKS.fetch_add(1,  Ordering::Relaxed);
        info!(target: "compute_node", partition_id, task_counter, "Starting FHE task execution");

        let otel_ctx = ctx.get_otel_ctx();
        let task_id = daggy::NodeIndex::new(partition_id as usize); // TODO:

        // query inputs (input ciphertexts) from either Redis or local cache,
        // or wait for them to be available if not present yet
        let inputs_handles = vec![]; // TODO: get from task
        futures_util::future::try_join_all(
            inputs_handles.iter().map(|handle | ctx.get_or_wait_for_handle(handle))).await.unwrap(); // TODO: handle errors properly

        let outputs = tokio::task::spawn_blocking(move || {
            let graph_set = vec![]; // TODO:

            // Elapsed time since message was received until the start of execution. 
            // Should be as low as possible
            let elapsed = delivery.received_at.elapsed().as_secs_f64();
            info!( partition_id, elapsed, "Executing partition with scheduler");
            CONSUMER_OVERHEAD.observe(elapsed);

            scheduler::execute_partition(
                graph_set,
                task_id,
                0,
                keys.server_key.clone(),
                keys.pks.clone(),
                otel_ctx,
            )
        }).await;

        let outputs =  match outputs {
            Ok(res) => res,
            Err(e) => {
                error!( partition_id, error = %e, "Task panicked during FHE computation");
                return;
            }
        };
       let outputs_count = outputs.0.len();
       let mut output_cts = Vec::new(); // TODO: convert outputs to ciphertexts
        for (handle, output) in outputs.0.into_iter() {
            let result = match output {
                Ok(ct) => ct,
                Err(e) => {
                    error!( partition_id, handle = %hex::encode(&handle), error = %e, "Error executing node in partition");
                    continue;
                }
            };

            let ct = CiphertextInfo { handle: handle.clone(), ciphertext : result.ct};
            ctx.cache_store(&ct).await;
            output_cts.push(ct);
            info!( partition_id, handle = %hex::encode(&handle), "Stored computed ciphertext in cache");
        }

        // batch store to Redis for better performance
        // TODO: Ensure redis always stores all outputs
        // TODO: Consider retries or dead-letter queue on failure
        ctx.redis_batch_store(output_cts).await.unwrap(); // TODO: handle errors properly

        info!( partition_id, "Stored computed ciphertext in redis");

        // Once acknowledged, we can continue to process the next message
        if let Err(err) = delivery.inner.ack(BasicAckOptions::default()).await {
            error!(
                target: "tfhe_worker",
                error = %err,
                "Failed to acknowledge FHE task message"
            );
        }

        info!(target: "compute_node", partition_id, outputs_count, "Completed FHE task execution");
        RUNNING_TASKS.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
    });

    Ok(())
}
