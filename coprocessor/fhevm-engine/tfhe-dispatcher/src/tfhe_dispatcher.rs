use fhevm_engine_common::protocol::messages as msg;

#[cfg(feature = "rabbitmq")]
use message_broker::rabbitmq::{RabbitMQReceiver, RabbitMQSender};

use message_broker::{MessageResult, Receiver};
use std::sync::{Arc, RwLock};
use tokio::time::{sleep, Duration};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::{cli::Args, dispatcher::Dispatcher};

type SharedDispatcher = Arc<RwLock<Dispatcher<RabbitMQSender>>>;

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
    let sender = create_sender(args, "shared_tfhe_queue").await;
    let state: SharedDispatcher = Arc::new(RwLock::new(Dispatcher::new(sender)));

    let mut fhe_events = create_receiver(args, &args.fhe_events_queue_name, state.clone()).await;

    let mut fhe_partition_complete =
        create_receiver(args, &args.fhe_execution_complete_queue, state.clone()).await;

    loop {
        tokio::select! {
            biased;

            _ = cancel_token.cancelled() => {
                info!("Cancellation requested, exiting dispatcher cycle");
                return Ok(());
            }

            res = fhe_events.recv_and_handle(|batch: Vec<msg::FheLog>, _, state| async move {
                let mut state = state.write().unwrap();
                state.dispatch(&batch);
                Ok(MessageResult::Ack)
            }) => {
                if res.is_err() {
                    return Ok(());
                }
            }

            res = fhe_partition_complete.recv_and_handle(|partition: msg::ExecutablePartition, _, state| async move {
                let mut state = state.write().unwrap();
                state.on_partition_execution_complete(&partition);

                // Check if new partitions became executable
                state.dispatch(&[]);

                Ok(MessageResult::Ack)
            }) => {
                if res.is_err() {
                    return Ok(());
                }
            }
        }
    }
}

#[cfg(feature = "rabbitmq")]
async fn create_sender(args: &Args, queue_name: &str) -> RabbitMQSender {
    RabbitMQSender::new(&args.rmq_uri, queue_name, "", "queue_fhe_partitions").await
}

#[cfg(feature = "rabbitmq")]
async fn create_receiver(
    args: &Args,
    queue_name: &str,
    state: SharedDispatcher,
) -> RabbitMQReceiver<SharedDispatcher> {
    RabbitMQReceiver::new(&args.rmq_uri, queue_name, &args.consumer_tag_prefix, state).await
}

#[cfg(feature = "redis_stream")]
async fn create_sender(args: &Args, queue_name: &str) -> impl Sender<Vec<u8>> {
    message_broker::redis_stream::RedisStreamSender::new(&args.rmq_uri, queue_name).await
}
