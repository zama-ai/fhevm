use crate::{config::Config, decryption::extract_id_from_receipt};
use alloy::{
    primitives::{FixedBytes, U256},
    providers::Provider,
    rpc::types::{Log, TransactionReceipt},
    sol_types::{self, SolEvent},
};
use anyhow::anyhow;
use fhevm_gateway_rust_bindings::decryption::Decryption::{
    self, DecryptionInstance, PublicDecryptionResponse,
};
use futures::{Stream, StreamExt};
use indicatif::ProgressBar;
use std::{
    collections::HashSet,
    sync::{Arc, LazyLock},
    time::Duration,
};
use tokio::{
    sync::{
        Mutex,
        mpsc::{self, UnboundedReceiver, UnboundedSender},
    },
    task::JoinSet,
    time::Instant,
};
use tracing::{debug, error, trace};

/// Sends a burst of PublicDecryptionRequest.
pub async fn public_decryption_burst<P, S>(
    burst_index: usize,
    config: Config,
    decryption_contract: DecryptionInstance<(), P>,
    response_listener: Arc<Mutex<S>>,
    requests_pb: ProgressBar,
    responses_pb: ProgressBar,
) where
    P: Provider + Clone + 'static,
    S: Stream<Item = sol_types::Result<(PublicDecryptionResponse, Log)>> + Unpin + Send + 'static,
{
    let (id_sender, id_receiver) = mpsc::unbounded_channel();
    let wait_response_task = tokio::spawn(wait_for_burst_responses(
        burst_index,
        response_listener,
        id_receiver,
        config.clone(),
        responses_pb,
    ));

    let mut requests_tasks = JoinSet::new();
    for index in 0..config.parallel_requests {
        requests_tasks.spawn(send_public_decryption(
            index,
            decryption_contract.clone(),
            config.ct_handles.clone(),
            id_sender.clone(),
        ));
    }

    for _ in 0..config.parallel_requests {
        requests_tasks.join_next().await;
        requests_pb.inc(1);
    }
    requests_pb.finish_with_message("All requests were sent!");

    drop(id_sender); // Dropping last sender so `wait_for_responses` can exit properly
    if let Err(e) = wait_response_task.await {
        error!("{e}");
    }
}

/// Sends a PublicDecryptionRequest transaction to the Gateway.
#[tracing::instrument(skip(decryption_contract, handles, id_sender))]
async fn send_public_decryption<P: Provider>(
    index: u32,
    decryption_contract: DecryptionInstance<(), P>,
    handles: Vec<FixedBytes<32>>,
    id_sender: UnboundedSender<U256>,
) {
    if let Err(e) = send_public_decryption_inner(decryption_contract, handles, id_sender).await {
        error!("{e}");
    }
}

async fn send_public_decryption_inner<P: Provider>(
    decryption_contract: DecryptionInstance<(), P>,
    handles: Vec<FixedBytes<32>>,
    id_sender: UnboundedSender<U256>,
) -> anyhow::Result<()> {
    let decryption_call = decryption_contract
        .publicDecryptionRequest(handles)
        .send()
        .await
        .map_err(|e| anyhow!("Failed to send transaction: {e}"))?;
    let receipt = decryption_call
        .get_receipt()
        .await
        .map_err(|e| anyhow!("Failed to get receipt: {e}"))?;
    debug!("PublicDecryptionRequest successfully sent!");

    let id = extract_public_decryption_id_from_receipt(&receipt)?;
    id_sender.send(id)?;

    Ok(())
}

fn extract_public_decryption_id_from_receipt(receipt: &TransactionReceipt) -> anyhow::Result<U256> {
    extract_id_from_receipt(
        receipt,
        Decryption::PublicDecryptionRequest::SIGNATURE_HASH,
        |log| {
            Decryption::PublicDecryptionRequest::decode_log_data(log, true)
                .map(|event| event.decryptionId)
                .map_err(|e| anyhow!("Failed to decode event data {e}"))
        },
    )
}

/// Creates the PublicDecryptionResponse listener.
pub async fn init_public_decryption_response_listener<P: Provider>(
    decryption_contract: DecryptionInstance<(), P>,
) -> anyhow::Result<
    Arc<
        Mutex<
            impl Stream<Item = alloy::sol_types::Result<(PublicDecryptionResponse, Log)>> + Unpin + Send,
        >,
    >,
> {
    debug!("Subcribing to PublicDecryptionResponse events...");
    let mut response_filter = decryption_contract
        .PublicDecryptionResponse_filter()
        .watch()
        .await
        .map_err(|e| anyhow!("Failed to subscribe to PublicDecryptionResponse {e}"))?;
    debug!(
        "Subcribed to PublicDecryptionResponse events! Can start sending PublicDecryptionRequests..."
    );

    response_filter.poller = response_filter
        .poller
        .with_poll_interval(Duration::from_millis(500));

    Ok(Arc::new(Mutex::new(response_filter.into_stream())))
}

/// Waits for all the responses of a requests burst.
async fn wait_for_burst_responses<S>(
    burst_index: usize,
    response_listener: Arc<Mutex<S>>,
    id_receiver: UnboundedReceiver<U256>,
    config: Config,
    progress_bar: ProgressBar,
) where
    S: Stream<Item = sol_types::Result<(PublicDecryptionResponse, Log)>> + Unpin,
{
    if let Err(e) = wait_for_burst_responses_inner(
        burst_index,
        response_listener,
        id_receiver,
        config,
        progress_bar,
    )
    .await
    {
        error!("{e}");
    }
}

async fn wait_for_burst_responses_inner<S>(
    burst_index: usize,
    response_listener: Arc<Mutex<S>>,
    mut id_receiver: UnboundedReceiver<U256>,
    config: Config,
    progress_bar: ProgressBar,
) -> anyhow::Result<()>
where
    S: Stream<Item = sol_types::Result<(PublicDecryptionResponse, Log)>> + Unpin,
{
    let burst_start = Instant::now();

    let mut received_id_guard = RECEIVED_RESPONSES_IDS.lock().await;
    let mut listener_guard = response_listener.lock().await;
    for _ in 0..config.parallel_requests {
        let Some(id) = id_receiver.recv().await else {
            return Err(anyhow!("Request id channel was closed unexpectedly"));
        };

        debug!(
            "PublicDecryptionRequest #{id} was sent. Waiting for PublicDecryptionResponse #{id}..."
        );

        while !received_id_guard.remove(&id) {
            match listener_guard.next().await {
                Some(Ok((response, _))) => {
                    let response_id = response.decryptionId;
                    trace!("Received PublicDecryptionResponse #{response_id}");
                    received_id_guard.insert(response_id);
                    progress_bar.inc(1);
                }
                Some(Err(e)) => return Err(anyhow!("Failed to retrieve next event: {e}")),
                None => return Err(anyhow!("No more events to receive!")),
            }
        }
        debug!("PublicDecryptionResponse #{id} was successfully received!");
    }
    drop(received_id_guard);
    drop(listener_guard);

    let elapsed = burst_start.elapsed().as_secs_f64();
    progress_bar.finish_with_message(format!(
        "Handled burst #{} of {} in {:.2}s. Throughput: {:.2} tps",
        burst_index,
        config.parallel_requests,
        elapsed,
        config.parallel_requests as f64 / elapsed
    ));

    Ok(())
}

static RECEIVED_RESPONSES_IDS: LazyLock<Arc<Mutex<HashSet<U256>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(HashSet::new())));
