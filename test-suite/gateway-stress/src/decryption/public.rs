use crate::{
    blockchain::manager::AppProvider,
    config::Config,
    decryption::{
        BurstResult, EVENT_LISTENER_POLLING, extract_id_from_receipt, send_tx_with_retries,
    },
};
use alloy::{
    primitives::{FixedBytes, U256},
    providers::Provider,
    rpc::types::{Log, TransactionReceipt},
    sol_types::{self, SolEvent},
};
use anyhow::anyhow;
use fhevm_gateway_bindings::decryption::Decryption::{
    self, DecryptionInstance, PublicDecryptionResponse,
};
use futures::{Stream, StreamExt};
use indicatif::ProgressBar;
use std::{
    collections::HashSet,
    sync::{Arc, LazyLock},
};
use tokio::{
    sync::{
        Mutex,
        mpsc::{self, UnboundedReceiver, UnboundedSender},
    },
    task::JoinSet,
    time::Instant,
};
use tracing::{Instrument, debug, error, trace};

pub type PublicDecryptThresholdEvent = sol_types::Result<(PublicDecryptionResponse, Log)>;

/// Sends a burst of PublicDecryptionRequest.
#[tracing::instrument(skip(
    config,
    decryption_contract,
    response_listener,
    requests_pb,
    responses_pb
))]
pub async fn public_decryption_burst<S>(
    burst_index: usize,
    config: Config,
    decryption_contract: DecryptionInstance<AppProvider>,
    response_listener: Arc<Mutex<S>>,
    requests_pb: ProgressBar,
    responses_pb: ProgressBar,
) -> anyhow::Result<BurstResult>
where
    S: Stream<Item = sol_types::Result<(PublicDecryptionResponse, Log)>> + Unpin + Send + 'static,
{
    debug!("Start of the burst...");
    let (id_sender, id_receiver) = mpsc::unbounded_channel();
    let wait_response_task = tokio::spawn(
        wait_for_burst_responses(
            burst_index,
            response_listener,
            id_receiver,
            config.clone(),
            responses_pb,
        )
        .in_current_span(),
    );

    let mut requests_tasks = JoinSet::new();
    for index in 0..config.parallel_requests {
        requests_tasks.spawn(
            send_public_decryption(
                index,
                decryption_contract.clone(),
                config.public_ct.iter().map(|ct| ct.handle).collect(),
                id_sender.clone(),
            )
            .in_current_span(),
        );
    }

    for _ in 0..config.parallel_requests {
        requests_tasks.join_next().await;
        requests_pb.inc(1);
    }
    requests_pb.finish_with_message("All requests were sent!");
    debug!("All requests of the burst have been sent! Waiting for responses...");

    drop(id_sender); // Dropping last sender so `wait_for_responses` can exit properly
    let res = wait_response_task
        .await
        .inspect_err(|e| error!("{e}"))?
        .inspect_err(|e| error!("{e}"))?;
    debug!("Successfully received all responses of the burst!");
    Ok(res)
}

/// Sends a PublicDecryptionRequest transaction to the Gateway.
#[tracing::instrument(skip(decryption_contract, handles, id_sender))]
async fn send_public_decryption(
    index: u32,
    decryption_contract: DecryptionInstance<AppProvider>,
    handles: Vec<FixedBytes<32>>,
    id_sender: UnboundedSender<U256>,
) {
    if let Err(e) = send_public_decryption_inner(decryption_contract, handles, id_sender).await {
        error!("{e}");
    }
}

async fn send_public_decryption_inner(
    decryption_contract: DecryptionInstance<AppProvider>,
    handles: Vec<FixedBytes<32>>,
    id_sender: UnboundedSender<U256>,
) -> anyhow::Result<()> {
    let decryption_call = decryption_contract
        .publicDecryptionRequest(handles, vec![].into())
        .into_transaction_request();

    send_tx_with_retries(
        decryption_contract.provider(),
        decryption_call,
        id_sender,
        extract_public_decryption_id_from_receipt,
    )
    .await
}

fn extract_public_decryption_id_from_receipt(receipt: &TransactionReceipt) -> anyhow::Result<U256> {
    extract_id_from_receipt(
        receipt,
        Decryption::PublicDecryptionRequest::SIGNATURE_HASH,
        |log| {
            Decryption::PublicDecryptionRequest::decode_log_data(log)
                .map(|event| event.decryptionId)
                .map_err(|e| anyhow!("Failed to decode event data {e}"))
        },
    )
}

/// Creates the PublicDecryptionResponse listener.
pub async fn init_public_decryption_response_listener<P: Provider>(
    decryption_contract: DecryptionInstance<P>,
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
        .with_poll_interval(EVENT_LISTENER_POLLING);

    Ok(Arc::new(Mutex::new(response_filter.into_stream())))
}

/// Waits for all the responses of a requests burst.
async fn wait_for_burst_responses<S>(
    burst_index: usize,
    response_listener: Arc<Mutex<S>>,
    mut id_receiver: UnboundedReceiver<U256>,
    config: Config,
    progress_bar: ProgressBar,
) -> anyhow::Result<BurstResult>
where
    S: Stream<Item = sol_types::Result<(PublicDecryptionResponse, Log)>> + Unpin,
{
    let burst_start = Instant::now();

    let mut received_id_guard = RECEIVED_RESPONSES_IDS.lock().await;
    let mut listener_guard = response_listener.lock().await;
    for _ in 0..config.parallel_requests {
        let Some(id) = id_receiver.recv().await else {
            return Err(anyhow!(
                "Request id channel #{burst_index} was closed unexpectedly"
            ));
        };

        trace!(
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

    let latency = burst_start.elapsed().as_secs_f64();
    let result = BurstResult {
        latency,
        throughput: config.parallel_requests as f64 / latency,
    };
    progress_bar.finish_with_message(format!(
        "Handled burst #{} of {} in {:.2}s. Throughput: {:.2} tps",
        burst_index, config.parallel_requests, result.latency, result.throughput
    ));

    Ok(result)
}

static RECEIVED_RESPONSES_IDS: LazyLock<Arc<Mutex<HashSet<U256>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(HashSet::new())));
