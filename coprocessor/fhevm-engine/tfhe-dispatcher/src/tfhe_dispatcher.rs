use crate::scheduler::messages::{self as msg};
use fhevm_engine_common::msg_broker::{
    create_recv_channel, create_send_channel, extract_delivery, try_decode,
};
use lapin::options::BasicAckOptions;

use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::{
    cli::Args,
    dispatcher::{Dispatcher, LapinChannel},
};
use futures_util::stream::StreamExt;

pub async fn run_tfhe_dispatcher(
    args: crate::cli::Args,
    cancel_token: CancellationToken,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!(target: "tfhe_dispatcher", "Starting tfhe-dispatcher service");
    loop {
        let cancel = cancel_token.clone();
        if let Err(cycle_error) = tfhe_dispatcher_cycle(&args, cancel).await {
            error!(target: "tfhe_dispatcher", { error = cycle_error }, "Error in background dispatcher, retrying shortly");
        }

        if cancel_token.is_cancelled() {
            info!("Cancellation requested, not restarting dispatcher cycle");
            return Ok(());
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    }
}

async fn tfhe_dispatcher_cycle(
    args: &crate::cli::Args,
    cancel_token: CancellationToken,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let prefetch_count = 4u16; // TODO:

    // Queue for receiving batches of FHE events (logs) to process
    // Host-listener will push to this queue
    let mut fhe_events_recv_chan = create_recv_channel(
        &args.rmq_uri,
        "evm_fhe_dispatcher",
        "batch_fhe_events_queue",
        prefetch_count,
    )
    .await?;

    // Responses from workers about completed partitions
    // tfhe-compute-nodes will push to this queue after completing execution of a partition
    let mut fhe_partition_complete_recv_chan = create_recv_channel(
        &args.rmq_uri,
        "evm_fhe_dispatcher",
        "fhe_partition_execution_complete_queue",
        prefetch_count,
    )
    .await?;

    let mut dsp = setup_dispatcher(args).await?;

    loop {
        tokio::select! {
            biased;
            res = fhe_events_recv_chan.next() => {
                let d = extract_delivery(res, "fhe_events")?;
                let Some(batch) = try_decode::<Vec<msg::FheLog>>(&d).await else {
                    continue;
                };

                dsp.dispatch_fhe_partitions(&batch);

                let _ = d.ack(BasicAckOptions::default()).await;
            },
            res = fhe_partition_complete_recv_chan.next() => {
                let d = extract_delivery(res, "fhe_partition_complete")?;
                let Some(partition) = try_decode::<msg::ExecutablePartition>(&d).await else {
                    continue;
                };

                dsp.on_partition_execution_complete(&partition);
                dsp.dispatch_fhe_partitions(&[]);

                let _ = d.ack(BasicAckOptions::default()).await;
            }
            _ = cancel_token.cancelled() => {
                info!("Cancellation requested, exiting dispatcher cycle");
                return Ok(());
            }
        }
    }
}

async fn setup_dispatcher(
    args: &Args,
) -> Result<Dispatcher<LapinChannel>, Box<dyn std::error::Error + Send + Sync>> {
    // Queue to send executable partitions to workers
    let sender_channel = create_send_channel(&args.rmq_uri, "shared_tfhe_queue").await?;

    sender_channel.confirm_select(Default::default()).await?;

    let default_channel = LapinChannel::new(
        sender_channel,
        "shared_tfhe_queue".to_string(),
        "shared_tfhe_queue".to_string(),
    );

    Ok(Dispatcher::new(default_channel))
}
