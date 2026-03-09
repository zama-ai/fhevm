use std::time::SystemTime;

use fhevm_engine_common::{
    protocol::messages::{self, BlockContext, Dependence},
    types::{Handle, SupportedFheOperations},
};
use lapin::options::BasicPublishOptions;
use message_broker::rabbitmq::create_send_channel;
use tokio::signal::unix::{self, SignalKind};
use tokio_util::sync::CancellationToken;
use tracing::info;

#[tokio::main]
async fn main() {
    let args = tfhe_dispatcher::cli::parse_args();
    let cancel = CancellationToken::new();
    install_signal_handlers(cancel.clone());

    tracing_subscriber::fmt()
        .json()
        .with_level(true)
        .with_max_level(args.log_level)
        .init();

    let queue = &args.fhe_events_queue_name;
    let sender_channel = create_send_channel(&args.rmq_uri, queue).await.unwrap();

    sender_channel
        .confirm_select(Default::default())
        .await
        .unwrap();

    info!(target: "main", args = ?args, "Starting client with args");

    publish_batch(&sender_channel, queue).await;

    // Insert computations
}

static HANDLE_COUNTER: std::sync::atomic::AtomicU16 = std::sync::atomic::AtomicU16::new(0);
fn next_handle() -> Handle {
    handle(HANDLE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst))
}

pub fn handle(id: u16) -> Handle {
    if id > HANDLE_COUNTER.load(std::sync::atomic::Ordering::SeqCst) {
        panic!("Handle ID {} is out of bounds", id);
    }
    let mut h = Vec::from([0u8; 32]);
    h[..2].copy_from_slice(&id.to_be_bytes());
    h
}

/// Publishes a batch of FHE log messages to the queue
async fn publish_batch(sender_channel: &lapin::Channel, queue: &str) {
    let scalar_type = Vec::from(2u16.to_be_bytes());
    let scalar_pt: Vec<u8> = Vec::from(20u64.to_be_bytes());

    let mut batch = Vec::new();
    let _ = add_log_event(
        &mut batch,
        SupportedFheOperations::FheTrivialEncrypt,
        next_handle(),
        vec![
            Dependence::Scalar(scalar_pt),
            Dependence::Scalar(scalar_type),
        ],
    );

    let _ = add_log_event(
        &mut batch,
        SupportedFheOperations::FheAdd,
        next_handle(),
        vec![
            Dependence::Reference(handle(0)),
            Dependence::Reference(handle(0)),
        ],
    );

    let _ = add_log_event(
        &mut batch,
        SupportedFheOperations::FheMul,
        next_handle(),
        vec![
            Dependence::Reference(handle(0)),
            Dependence::Reference(handle(0)),
        ],
    );

    let scalar_pt = Vec::from(10u64.to_be_bytes());

    let _ = add_log_event(
        &mut batch,
        SupportedFheOperations::FheAdd,
        next_handle(),
        vec![
            Dependence::Reference(handle(2)),
            Dependence::Scalar(scalar_pt),
        ],
    );

    let scalar_pt = Vec::from(10u64.to_be_bytes());
    let _ = add_log_event(
        &mut batch,
        SupportedFheOperations::FheAdd,
        next_handle(),
        vec![
            Dependence::Reference(handle(3)),
            Dependence::Scalar(scalar_pt),
        ],
    );

    let _ = add_log_event(
        &mut batch,
        SupportedFheOperations::FheMul,
        next_handle(),
        vec![
            Dependence::Reference(handle(0)),
            Dependence::Reference(handle(4)),
        ],
    );

    let _ = add_log_event(
        &mut batch,
        SupportedFheOperations::FheMul,
        next_handle(),
        vec![
            Dependence::Reference(handle(0)),
            Dependence::Reference(handle(4)),
        ],
    );

    let _ = add_log_event(
        &mut batch,
        SupportedFheOperations::FheMul,
        next_handle(),
        vec![
            Dependence::Reference(handle(4)),
            Dependence::Reference(handle(0)),
        ],
    );

    let _ = add_log_event(
        &mut batch,
        SupportedFheOperations::FheDiv,
        next_handle(),
        vec![
            Dependence::Reference(handle(1)),
            Dependence::Reference(handle(2)),
        ],
    );

    let scalar_pt = Vec::from(100u64.to_be_bytes());

    let _ = add_log_event(
        &mut batch,
        SupportedFheOperations::FheDiv,
        next_handle(),
        vec![
            Dependence::Reference(handle(5)),
            Dependence::Scalar(scalar_pt),
        ],
    );

    // Partition with computed reference (Dependence::Reference(handle(4)), //TODO: handle(0))

    let payload: Vec<u8> = postcard::to_allocvec(&batch).unwrap();

    let confirm = sender_channel
        .basic_publish(
            "",
            queue,
            BasicPublishOptions::default(),
            &payload,
            lapin::BasicProperties::default(),
        )
        .await
        .unwrap();

    let confirm = confirm.await.unwrap();
    info!(confirm = ?confirm, "Sent FHE log message to the queue");
}

fn install_signal_handlers(cancel_token: CancellationToken) {
    let mut sigint = unix::signal(SignalKind::interrupt()).unwrap();
    let mut sigterm = unix::signal(SignalKind::terminate()).unwrap();
    tokio::spawn(async move {
        tokio::select! {
            _ = sigint.recv() => { info!("received SIGINT"); },
            _ = sigterm.recv() => { info!("received SIGTERM"); },
        }
        cancel_token.cancel();
        info!("Cancellation signal sent over the token");
    });
}

fn add_log_event(
    batch: &mut Vec<messages::FheLog>,
    fhe_operation: SupportedFheOperations,
    output_handle: Handle,
    dependencies: Vec<messages::Dependence>,
) -> messages::FheLog {
    let log = messages::FheLog {
        output_handle,
        dependencies,
        fhe_operation,
        is_scalar: true,
        is_allowed: false,
        created_at: SystemTime::now(),
        block_info: BlockContext {
            txn_hash: [1u8; 32],
            block_number: 1,
            block_hash: [1u8; 32],
        },
    };

    batch.push(log.clone());
    log
}
