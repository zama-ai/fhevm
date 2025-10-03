use crate::{
    config::Config,
    decryption::{
        BurstResult, EVENT_LISTENER_POLLING, extract_id_from_receipt, send_tx_with_retries,
    },
};
use alloy::{
    hex,
    primitives::{Address, U256},
    providers::Provider,
    rpc::types::{Log, TransactionReceipt},
    sol_types::{self, SolEvent},
};
use anyhow::anyhow;
use fhevm_gateway_bindings::decryption::{
    Decryption::{
        self, CtHandleContractPair, DecryptionInstance, UserDecryptionResponseThresholdReached,
    },
    IDecryption::{ContractsInfo, RequestValidity},
};
use futures::{Stream, StreamExt};
use gateway_sdk::{FhevmSdk, signature::Eip712Result};
use indicatif::ProgressBar;
use std::{
    collections::HashSet,
    sync::{Arc, LazyLock},
    time::SystemTime,
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

pub type UserDecryptThresholdEvent =
    sol_types::Result<(UserDecryptionResponseThresholdReached, Log)>;

/// Sends a burst of UserDecryptionRequest.
#[allow(clippy::too_many_arguments)]
#[tracing::instrument(skip(
    config,
    decryption_contract,
    sdk,
    user_addr,
    response_listener,
    requests_pb,
    responses_pb
))]
pub async fn user_decryption_burst<P, S>(
    burst_index: usize,
    config: Config,
    decryption_contract: DecryptionInstance<P>,
    sdk: Arc<FhevmSdk>,
    user_addr: Address,
    response_listener: Arc<Mutex<S>>,
    requests_pb: ProgressBar,
    responses_pb: ProgressBar,
) -> anyhow::Result<BurstResult>
where
    P: Provider + Clone + 'static,
    S: Stream<Item = UserDecryptThresholdEvent> + Unpin + Send + 'static,
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
            send_user_decryption(
                index,
                decryption_contract.clone(),
                user_addr,
                Arc::clone(&sdk),
                config.clone(),
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

/// Sends a UserDecryptionRequest transaction to the Gateway.
#[tracing::instrument(skip(decryption_contract, user_addr, sdk, config, id_sender))]
async fn send_user_decryption<P: Provider>(
    index: u32,
    decryption_contract: DecryptionInstance<P>,
    user_addr: Address,
    sdk: Arc<FhevmSdk>,
    config: Config,
    id_sender: UnboundedSender<U256>,
) {
    if let Err(e) =
        send_user_decryption_inner(decryption_contract, user_addr, sdk, config, id_sender).await
    {
        error!("{e}");
    }
}

async fn send_user_decryption_inner<P: Provider>(
    decryption_contract: DecryptionInstance<P>,
    user_addr: Address,
    sdk: Arc<FhevmSdk>,
    config: Config,
    id_sender: UnboundedSender<U256>,
) -> anyhow::Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    let host_chain_id = config.blockchain.as_ref().unwrap().host_chain_id;
    let eip712 = generate_eip712(sdk, &config, timestamp)?;
    let decryption_call = decryption_contract
        .userDecryptionRequest(
            config
                .user_ct
                .iter()
                .map(|ct| CtHandleContractPair {
                    ctHandle: ct.handle,
                    contractAddress: config.allowed_contract,
                })
                .collect(),
            RequestValidity {
                startTimestamp: U256::from(timestamp),
                durationDays: U256::from(DURATION_DAYS),
            },
            ContractsInfo {
                chainId: U256::from(host_chain_id),
                addresses: vec![config.allowed_contract],
            },
            user_addr,
            hex::decode(RAND_PUBLIC_KEY)?.into(),
            eip712.signature.clone().unwrap(),
            EXTRA_DATA.into(),
        )
        .into_transaction_request();

    send_tx_with_retries(
        decryption_contract.provider(),
        decryption_call,
        id_sender,
        extract_user_decryption_id_from_receipt,
    )
    .await
}

fn extract_user_decryption_id_from_receipt(receipt: &TransactionReceipt) -> anyhow::Result<U256> {
    extract_id_from_receipt(
        receipt,
        Decryption::UserDecryptionRequest::SIGNATURE_HASH,
        |log| {
            Decryption::UserDecryptionRequest::decode_log_data(log)
                .map(|event| event.decryptionId)
                .map_err(|e| anyhow!("Failed to decode event data {e}"))
        },
    )
}

/// Creates the UserDecryptionResponse listener.
pub async fn init_user_decryption_response_listener<P: Provider>(
    decryption_contract: DecryptionInstance<P>,
) -> anyhow::Result<
    Arc<
        Mutex<
            impl Stream<Item = alloy::sol_types::Result<(UserDecryptionResponseThresholdReached, Log)>>
            + Unpin
            + Send,
        >,
    >,
> {
    debug!("Subcribing to UserDecryptionResponseThresholdReached events...");
    let mut response_filter = decryption_contract
        .UserDecryptionResponseThresholdReached_filter()
        .watch()
        .await
        .map_err(|e| {
            anyhow!("Failed to subscribe to UserDecryptionResponseThresholdReached {e}")
        })?;
    debug!(
        "Subcribed to UserDecryptionResponseThresholdReached events! \
        Can start sending UserDecryptionRequests..."
    );

    response_filter.poller = response_filter
        .poller
        .with_poll_interval(EVENT_LISTENER_POLLING);

    Ok(Arc::new(Mutex::new(response_filter.into_stream())))
}

pub fn generate_eip712(
    sdk: Arc<FhevmSdk>,
    config: &Config,
    timestamp: u64,
) -> anyhow::Result<Eip712Result> {
    let allowed_contract = config.allowed_contract;
    let private_key = config
        .blockchain
        .as_ref()
        .unwrap()
        .private_key
        .clone()
        .unwrap();

    // Spawn in new thread otherwise panic because it blocks the async runtime
    std::thread::spawn(move || {
        sdk.create_eip712_signature_builder()
            .with_public_key(RAND_PUBLIC_KEY)
            .with_contract(allowed_contract)
            .unwrap()
            .with_verification(true)
            .with_validity_period(timestamp, DURATION_DAYS)
            .with_private_key(&private_key)
            .with_extra_data(EXTRA_DATA.to_vec())
            .generate_and_sign()
    })
    .join()
    .map_err(|e| anyhow!("{e:?}"))?
    .map_err(|e| anyhow!("{e}"))
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
    S: Stream<Item = UserDecryptThresholdEvent> + Unpin,
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
            "UserDecryptionRequest #{id} was sent. \
            Waiting for UserDecryptionResponseThresholdReached #{id}..."
        );

        while !received_id_guard.remove(&id) {
            match listener_guard.next().await {
                Some(Ok((response, _))) => {
                    let response_id = response.decryptionId;
                    trace!("Received UserDecryptionResponseThresholdReached #{response_id}");
                    received_id_guard.insert(response_id);
                    progress_bar.inc(1);
                }
                Some(Err(e)) => return Err(anyhow!("Failed to retrieve next event: {e}")),
                None => return Err(anyhow!("No more events to receive!")),
            }
        }
        debug!("UserDecryptionResponseThresholdReached #{id} was successfully received!");
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

pub const DURATION_DAYS: u64 = 10;
pub const EXTRA_DATA: [u8; 1] = [0];
pub const RAND_PUBLIC_KEY: &str = "0x0300000000000000302e35000000000300000000000000302e311300000000000000556e69666965645075626c6963456e634b65790000000000000000200300000000000014ea3e413024ff622b2a29974a7418a0c0b5d3fb5d93f83a414384386701e7895646cc83ff289216007c4a7821ec4291d4e92e5fa3a8e96cbd11491696407336d6a03bb971a4207f90b35958191ac69599d8337fa114cb607552e4113253c875f824c027e15e5ee2c73f5c4885d66f5324915f94c1dd1a0c4d226f9967c56db99bf81c3bfbb273e1fcc41f01639f754f13f8c62ec4cb6af14c373bc2d97955b1f50b66124ea38757b41c21191135c304b2b9223e2067cd4691c5ee840bbc3a382037c0ea611f8e0a6f6cb22afdf0a61f6aa11dc28691cb41d9b5a13eb94a3aea4e8e7c07c9f467949acac6bca98f720754485de7b95568495e0c90c9e73175db850a63d3cf45189846d20894e633636621b8107dbc132b9a281dbf382231697f67249a6b177db34c5b4639bc605424ab23cc186b00d0430e9efba3b6e8c07c615df7e0cf2c8082ffe640f6d4173b2c0b42f0c57e0a7b29a886b0808427f46cd0359e68a54bc783a684b95fe1859373d636e812524ee42a4ac8c8bab03a421cb8c8734ede914d8d221a4e258485f5415fe15d8aebcf0225ce87451402170d97a21313789750f794ca064129b1b3c8fb452c2b1d8d7a5bc514638362b952362c06536906000b08551e3087336869c2cb2190741a58c6171e6a562e83389042cb844cd3c2bf8365e36168fbe1bcfc2b1b281863dbe62fbb620f4683c3dcd34f22647f6d583950f834bf69cb3a9c790c049a7ec2cdfcfcaefd2b0281969913ca2ed134acbb917f36e3bf8fa8818a8a146a518ef00346d91469de68b495200a50d205735a781c650319f953c62255c8f93cd931387da3b2eb474f3bd9431641c2f62b6ddae95e34d01dc8510382e925610228508b30d5271ec74a01b9b0c6ceb07250565055369af2577bbae08b9bb61c38acac6ac6bb65494d2b5846fe345556252deecb6c8c2629e8a67585fcbb9c992266f6a739fc8d45269acb0c5250f4990f899982493a50d23d9228cdbf726bdcf250d19705f5991ee966077e58b04cb0023a9984431828e9085b34c959cfeb838d4507d4d1b8704c5f6ce7b1298c89a984196fec71df209828e693510719ef86d5740906b94a86aa52b57b955b4ffb9c1cc5";
