use fhevm_engine_common::utils::{create_recv_channel, create_send_channel};
use tracing::{error, info};

use crate::dispatcher::Dispatcher;
use futures_util::stream::StreamExt;

pub async fn run_tfhe_dispatcher(
    args: crate::cli::Args,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!(target: "tfhe_dispatcher", "Starting tfhe-dispatcher service");
    loop {
        // here we log the errors and make sure we retry
        if let Err(cycle_error) = tfhe_dispatcher_cycle(&args).await {
            error!(target: "tfhe_dispatcher", { error = cycle_error }, "Error in background dispatcher, retrying shortly");
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    }
}

async fn tfhe_dispatcher_cycle(
    args: &crate::cli::Args,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut dispatcher = Dispatcher::default();

    let prefetch_count = 4u16; // TODO:

    // Queue for receiving batches of FHE events (logs) to process
    // Host-listener will push to this queue
    let mut fhe_events_rcv_chan = create_recv_channel(
        &args.rmq_uri,
        "evm_fhe_dispatcher",
        "batch_fhe_events_queue",
        prefetch_count,
    )
    .await?;

    // Responses from workers about completed partitions
    // tfhe-compute-nodes will push to this queue after completing execution of a partition
    let mut fhe_partition_complete_rcv_chan = create_recv_channel(
        &args.rmq_uri,
        "evm_fhe_dispatcher",
        "fhe_partition_execution_complete_queue",
        prefetch_count,
    )
    .await?;

    // Queue to send executable partitions to workers
    let sender_channel = create_send_channel(&args.rmq_uri, "shared_tfhe_queue").await?;
    sender_channel.confirm_select(Default::default()).await?;

    loop {
        tokio::select! {
            biased;
            _ = fhe_events_rcv_chan.next() => {
                let batch = vec![]; // TODO: fetch logs from message payload

                dispatcher.dispatch_fhe_partitions(&batch, sender_channel.clone() );

            },
            msg = fhe_partition_complete_rcv_chan.next() => {
                let partition_id = [0u8; 32]; // TODO: fetch partition id from message payload

                dispatcher.on_partition_execution_complete(&partition_id);
                dispatcher.dispatch_fhe_partitions(&[], sender_channel.clone() );
            }
        }
    }
}
