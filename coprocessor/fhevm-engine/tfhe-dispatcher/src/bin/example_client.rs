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

    publish_transfer(&sender_channel, queue).await;

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
/*
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
*/
/// Publishes a batch of FHE log messages to the queue
async fn publish_transfer(sender_channel: &lapin::Channel, queue: &str) {
    let scalar_type = Vec::from(5u16.to_be_bytes());
    let scalar_pt: Vec<u8> = Vec::from(100u64.to_be_bytes());

    // Alice balance = 100
    let mut batch = Vec::new();
    let alice_balance = add_log_event_other_op(
        &mut batch,
        SupportedFheOperations::FheTrivialEncrypt,
        vec![
            Dependence::Scalar(scalar_pt),
            Dependence::Scalar(scalar_type.clone()),
        ],
    );

    let scalar_pt: Vec<u8> = Vec::from(11u64.to_be_bytes());

    // Alice wants to transfer 11 tokens to Bob
    let alice_spent = add_log_event_other_op(
        &mut batch,
        SupportedFheOperations::FheTrivialEncrypt,
        vec![
            Dependence::Scalar(scalar_pt),
            Dependence::Scalar(scalar_type.clone()),
        ],
    );

    // Bob balance = 22

    let scalar_pt: Vec<u8> = Vec::from(22u64.to_be_bytes());
    let bob_balance = add_log_event_other_op(
        &mut batch,
        SupportedFheOperations::FheTrivialEncrypt,
        vec![
            Dependence::Scalar(scalar_pt),
            Dependence::Scalar(scalar_type.clone()),
        ],
    );

    // Bob receives 11 tokens from Alice
    let bob_updated_balance = add_log_event_binary_op(
        &mut batch,
        SupportedFheOperations::FheAdd,
        Dependence::Reference(bob_balance.output_handle),
        Dependence::Reference(alice_spent.output_handle.clone()),
    );

    // Alice balance is reduced by 10 tokens
    let alice_updated_balance = add_log_event_binary_op(
        &mut batch,
        SupportedFheOperations::FheSub,
        Dependence::Reference(alice_balance.output_handle), // lhs
        Dependence::Reference(alice_spent.output_handle),   // rhs
    );

    let scalar_pt: Vec<u8> = Vec::from(3u64.to_be_bytes());

    // Shareholders
    let shareholders_num = add_log_event_other_op(
        &mut batch,
        SupportedFheOperations::FheTrivialEncrypt,
        vec![
            Dependence::Scalar(scalar_pt),
            Dependence::Scalar(scalar_type.clone()),
        ],
    );

    // Alice balance is reduced by 10 tokens
    let divident_by_3 = add_log_event_binary_op(
        &mut batch,
        SupportedFheOperations::FheDiv,
        Dependence::Reference(bob_updated_balance.output_handle), // lhs
        Dependence::Reference(shareholders_num.output_handle),    // rhs
    );

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

fn add_log_event_binary_op(
    batch: &mut Vec<messages::FheLog>,
    fhe_operation: SupportedFheOperations,
    lhs: messages::Dependence,
    rhs: messages::Dependence,
) -> messages::FheLog {
    let mut dependencies = Vec::new();
    dependencies.push(rhs); //TODO: why reverse order?
    dependencies.push(lhs);

    add_log_event_other_op(batch, fhe_operation, dependencies)
}

fn add_log_event_other_op(
    batch: &mut Vec<messages::FheLog>,
    fhe_operation: SupportedFheOperations,
    dependencies: Vec<messages::Dependence>,
) -> messages::FheLog {
    let output_handle = next_handle();

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
