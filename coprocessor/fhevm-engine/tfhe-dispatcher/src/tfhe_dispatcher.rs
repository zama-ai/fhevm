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

pub async fn run_tfhe_dispatcher(
    args: Args,
    cancel_token: CancellationToken,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!(target: "tfhe_dispatcher", "Starting tfhe-dispatcher service");

    loop {
        let cancel = cancel_token.clone();

        if let Err(err) = tfhe_dispatcher_loop(&args, cancel).await {
            error!(
                target: "tfhe_dispatcher",
                { error = err },
                "Error in dispatcher cycle, retrying shortly"
            );
        }

        if cancel_token.is_cancelled() {
            info!("Cancellation requested, stopping dispatcher");
            return Ok(());
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
            res = fhe_events.recv_and_handle(|batch: Vec<msg::FheLog>, _, state| async move {
                info!(batch_size = batch.len(), "msg: received batch of FHE log messages");
                let mut state = state.write().unwrap();
                let dispatched_count = state.dispatch(&batch);

                info!(dispatched_count = dispatched_count, "msg: processed batch of FHE log messages");
                Ok(MessageResult::Ack)
            }) => {
                if res.is_err() {
                    return Ok(());
                }
            }
            res = fhe_partition_complete.recv_and_handle(|partition: msg::ExecutablePartition, _, state| async move {
                info!(
                    pid = %partition.id(),
                    "msg: received partition execution completion message"
                );
                let mut state = state.write().unwrap();
                state.on_partition_execution_complete(&partition);

                // Check and dispatch any new executable partitions that
                // may have become ready after this completion
                let dispatched_count = state.dispatch(&[]);

                info!(dispatched_count = dispatched_count, "msg: dispatched new executable partitions after completion");
                Ok(MessageResult::Ack)
            }) => {
                if res.is_err() {
                    return Ok(());
                }
            }
             _ = tick.tick() => {
                 info!("on-idle event");
            }
        }
    }
}
