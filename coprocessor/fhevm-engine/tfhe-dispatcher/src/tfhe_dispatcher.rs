use fhevm_engine_common::protocol::messages as msg;

use message_broker::{
    create_default_receiver, create_default_sender, DefaultSender, MessageResult, Receiver,
};
use std::sync::{Arc, RwLock};
use tokio::time::{interval, sleep, Duration};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::{cli::Args, dispatcher::Dispatcher};

type SharedDispatcher = Arc<RwLock<Dispatcher<DefaultSender>>>;

/// Runs the main loop of the tfhe-dispatcher with retry logic and graceful shutdown handling.
pub async fn run_tfhe_dispatcher(
    args: Args,
    cancel_token: CancellationToken,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!(target: "tfhe_dispatcher", "Starting tfhe-dispatcher service");

    loop {
        if cancel_token.is_cancelled() {
            info!("Cancellation requested, stopping dispatcher");
            return Ok(());
        }

        if let Err(err) = tfhe_dispatcher_loop(&args, cancel_token.clone()).await {
            error!(
                target: "tfhe_dispatcher",
                { error = err },
                "Error in dispatcher cycle, retrying shortly"
            );
        }

        sleep(Duration::from_secs(5)).await;
    }
}

/// Main loop of the dispatcher, which listens for FHE events and partition completion messages,
/// and updates the dispatcher state accordingly. This loop will run until a cancellation is requested.
async fn tfhe_dispatcher_loop(
    args: &Args,
    cancel_token: CancellationToken,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // TODO: make this configurable with new msg_broker API is integrated
    let sender = create_default_sender(
        &args.rmq_uri,
        "shared_tfhe_queue",
        "",
        "queue_fhe_partitions",
    )
    .await;
    let state: SharedDispatcher = Arc::new(RwLock::new(Dispatcher::new(sender)));

    let mut fhe_events =
        create_default_receiver(&args.rmq_uri, &args.fhe_events_queue_name, state.clone()).await;

    let mut fhe_partition_complete = create_default_receiver(
        &args.rmq_uri,
        &args.fhe_execution_complete_queue,
        state.clone(),
    )
    .await;

    let mut tick = interval(Duration::from_secs(10));

    loop {
        tokio::select! {
            biased;
            _ = cancel_token.cancelled() => {
                info!("Cancellation requested, exiting dispatcher cycle");
                return Ok(());
            }
            res = fhe_events.recv_and_handle(handle_fhe_events) => {
                if res.is_err() {
                    info!("FHE events receiver channel closed");
                    return Ok(());
                }
            }
            res = fhe_partition_complete.recv_and_handle(handle_partition_completion) => {
                if res.is_err() {
                    info!("FHE partition complete channel closed");
                    return Ok(());
                }
            }
             _ = tick.tick() => {
                 // Periodic on-idle heartbeat indicating the dispatcher loop is still running,
                 // even when there are no FHE events or completion messages to process.
                 // TODO: Add periodic maintenance tasks here if needed ( metrics, health checks).
                 info!("heartbeat");
                 let state = state.read().unwrap();
                 state.report();

            }
        }
    }
}

async fn handle_fhe_events(
    batch: Vec<msg::FheLog>,
    _: Vec<u8>,
    state: SharedDispatcher,
) -> Result<MessageResult, Box<dyn std::error::Error + Send + Sync>> {
    info!(batch_size = batch.len(), "newmsg, received FHE logs");

    let mut state = state.write().unwrap();
    let dispatched = state.dispatch(&batch);

    info!(dispatched, "newmsg, processed FHE logs");

    Ok(MessageResult::Ack)
}

async fn handle_partition_completion(
    partition: msg::ExecutablePartition,
    _: Vec<u8>,
    state: SharedDispatcher,
) -> Result<MessageResult, Box<dyn std::error::Error + Send + Sync>> {
    info!(
        pid = partition.id(),
        "newmsg, received partition execution completion message"
    );
    let mut state = state.write().unwrap();
    state.on_partition_execution_complete(&partition);

    // Check and dispatch any new executable partitions that
    // may have become ready after this completion
    let dispatched_count = state.dispatch(&[]);

    info!(
        dispatched_count = dispatched_count,
        "newmsg, dispatched new executable partitions after completion"
    );
    Ok(MessageResult::Ack)
}
