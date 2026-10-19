//! The callers the Merkle proof route accepts: the tx-sender address of every KMS node in an Active
//! KMS context of the canonical `ProtocolConfig` (DD-076). The KMS connector signs its requests with
//! that tx-sender key. A tx-sender only in a Pending or Created context is refused until that
//! context is activated; the previous context stays Active, and accepted, until it is destroyed.
//!
//! A background task polls the finalized block every `poll_interval` and, when it is newer than the
//! last read, reads the set again at that block; requests only read the last set. A request whose signer is not in
//! the set wakes the task and waits, for a bounded time, for a poll that started after it, so a
//! context activated since the last poll is accepted at once. Woken polls start at least
//! [`WOKEN_POLL_SPACING`] apart, so unknown signers cost at most one poll a second. A failed poll
//! keeps the last set. Until the first read succeeds the route refuses every request.

use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, LazyLock},
    time::Duration,
};

use alloy::{
    eips::{BlockId, BlockNumberOrTag},
    primitives::{Address, U256},
    providers::Provider,
    transports::http::reqwest,
};
use anyhow::{anyhow, Context};
use fhevm_host_bindings::protocol_config::ProtocolConfig::ProtocolConfigInstance;
use kms_context::KMS_CONTEXT_COUNTER_BASE;
use prometheus::{register_int_gauge, IntGauge};
use request_authorization::KeyRegistry;
use solana_host_follower::url_origin;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use crate::{gauge_value, unix_now_secs};

/// The shortest time between the start of a poll and the start of a poll woken after it.
const WOKEN_POLL_SPACING: Duration = Duration::from_secs(1);
/// The RPC client has no request timeout, so a hung call would stall every later poll.
const READ_TIMEOUT: Duration = Duration::from_secs(30);

/// The accepted callers and the registry their signatures name, once read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KmsTxSenderSet {
    pub registry: KeyRegistry,
    pub senders: HashSet<Address>,
}

static KMS_TX_SENDERS: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "solana_merkle_proof_server_kms_tx_senders",
        "KMS tx-senders the last successful read of ProtocolConfig allows"
    )
    .unwrap()
});

static KMS_TX_SENDERS_READ_TIMESTAMP: LazyLock<IntGauge> = LazyLock::new(
    || {
        register_int_gauge!(
        "solana_merkle_proof_server_kms_tx_senders_read_timestamp_seconds",
        "Unix time of the last successful poll of the KMS tx-senders from ProtocolConfig"
    )
    .unwrap()
    },
);

/// The follower's polls so far: how many started, and how many finished.
#[derive(Clone, Copy, Debug, Default)]
struct Polls {
    started: u64,
    finished: u64,
}

/// The latest [`KmsTxSenderSet`]; `None` until the first read succeeds.
#[derive(Clone)]
pub struct KmsTxSenders {
    set: watch::Receiver<Option<Arc<KmsTxSenderSet>>>,
    polls: watch::Receiver<Polls>,
    /// The number of the poll a request waits for; the follower polls early while it has not
    /// started that one.
    wanted: Arc<watch::Sender<u64>>,
}

impl KmsTxSenders {
    /// No follower publishes polls, so [`Self::refresh_after_miss`] returns at once.
    fn without_follower(set: Option<KmsTxSenderSet>) -> Self {
        Self {
            set: watch::channel(set.map(Arc::new)).1,
            polls: watch::channel(Polls::default()).1,
            wanted: Arc::new(watch::channel(0).0),
        }
    }

    /// A set that never changes.
    pub fn fixed(set: KmsTxSenderSet) -> Self {
        Self::without_follower(Some(set))
    }

    /// A set that is never read.
    #[cfg(test)]
    pub(crate) fn unread() -> Self {
        Self::without_follower(None)
    }

    pub fn current(&self) -> Option<Arc<KmsTxSenderSet>> {
        self.set.borrow().clone()
    }

    pub fn is_loaded(&self) -> bool {
        self.set.borrow().is_some()
    }

    /// Wakes the follower, and waits at most `wait` for a poll that started after this call to
    /// finish. Many concurrent callers share that poll.
    pub async fn refresh_after_miss(&self, wait: Duration) {
        let mut polls = self.polls.clone();
        let needed = polls.borrow().started + 1;
        self.wanted.send_if_modified(|wanted| {
            let earlier = *wanted < needed;
            if earlier {
                *wanted = needed;
            }
            earlier
        });
        // Ends at once when no follower publishes polls.
        let _ = tokio::time::timeout(
            wait,
            polls.wait_for(|polls| polls.finished >= needed),
        )
        .await;
    }
}

/// Polls `protocol_config` every `poll_interval`, and early when a request waits for a poll, until
/// `cancel` fires or no [`KmsTxSenders`] remains.
pub fn follow<P: Provider + 'static>(
    protocol_config: ProtocolConfigInstance<P>,
    poll_interval: Duration,
    cancel: CancellationToken,
) -> KmsTxSenders {
    let (publish, set) = watch::channel(None);
    let (publish_polls, polls) = watch::channel(Polls::default());
    let (wanted, mut wanted_poll) = watch::channel(0);
    // An allowlist never read shows as no tx-senders, not as a missing series.
    KMS_TX_SENDERS.set(0);
    tokio::spawn(async move {
        let mut reader = KmsTxSenderReader::new(protocol_config);
        let mut count = Polls::default();
        loop {
            count.started += 1;
            publish_polls.send_replace(count);
            let started = tokio::time::Instant::now();
            let poll = tokio::time::timeout(READ_TIMEOUT, reader.poll())
                .await
                .unwrap_or_else(|_| {
                    Err(anyhow!("no answer within {READ_TIMEOUT:?}"))
                });
            match poll {
                Ok(read) => {
                    if let Some(set) = read {
                        if publish.borrow().as_deref() != Some(&set) {
                            info!(
                                senders = set.senders.len(),
                                "KMS tx-sender set updated"
                            );
                        }
                        KMS_TX_SENDERS
                            .set(gauge_value(set.senders.len() as u64));
                        publish.send_replace(Some(Arc::new(set)));
                    }
                    KMS_TX_SENDERS_READ_TIMESTAMP
                        .set(gauge_value(unix_now_secs()));
                }
                Err(err) => {
                    warn!(error = %without_rpc_url_secrets(&err), "KMS tx-sender read failed; keeping the last set");
                }
            }
            count.finished = count.started;
            publish_polls.send_replace(count);
            tokio::select! {
                _ = cancel.cancelled() => return,
                _ = tokio::time::sleep(poll_interval) => {}
                woken = async {
                    wanted_poll.wait_for(|&wanted| wanted > count.started).await.is_ok()
                } => {
                    if !woken {
                        return;
                    }
                    tokio::select! {
                        _ = cancel.cancelled() => return,
                        _ = tokio::time::sleep_until(started + WOKEN_POLL_SPACING) => {}
                    }
                }
            }
        }
    });
    KmsTxSenders {
        set,
        polls,
        wanted: Arc::new(wanted),
    }
}

/// `err` as text, with each request URL cut to its [`url_origin`]: `reqwest` prints the full URL.
fn without_rpc_url_secrets(err: &anyhow::Error) -> String {
    let mut text = format!("{err:#}");
    for url in err
        .chain()
        .filter_map(|cause| cause.downcast_ref::<reqwest::Error>())
        .filter_map(reqwest::Error::url)
    {
        text = text.replace(url.as_str(), &url_origin(url.as_str()));
    }
    text
}

struct KmsTxSenderReader<P> {
    protocol_config: ProtocolConfigInstance<P>,
    /// A context's nodes never change once it is created.
    nodes_by_context: HashMap<U256, Vec<Address>>,
    /// The finalized block of the last successful read.
    read_at: Option<u64>,
}

impl<P: Provider> KmsTxSenderReader<P> {
    fn new(protocol_config: ProtocolConfigInstance<P>) -> Self {
        Self {
            protocol_config,
            nodes_by_context: HashMap::new(),
            read_at: None,
        }
    }

    /// Reads the set at the finalized block, or nothing when that block is not newer than the one
    /// last read: at the same block the set cannot have changed, and an older one, from an RPC node
    /// that lags, would give an older set.
    async fn poll(&mut self) -> anyhow::Result<Option<KmsTxSenderSet>> {
        let finalized = self
            .protocol_config
            .provider()
            .get_block_by_number(BlockNumberOrTag::Finalized)
            .await
            .context("finalized block")?
            .ok_or_else(|| anyhow!("no finalized block"))?
            .header
            .number;
        if self.read_at.is_some_and(|read_at| finalized <= read_at) {
            return Ok(None);
        }
        let set = self.read(BlockId::number(finalized)).await?;
        self.read_at = Some(finalized);
        Ok(Some(set))
    }

    /// Every `ProtocolConfig` read of a poll uses the one block `at`. Read at different blocks, the
    /// counter could predate a new context while `isValidKmsContext` already shows its predecessor
    /// destroyed.
    async fn read(&mut self, at: BlockId) -> anyhow::Result<KmsTxSenderSet> {
        let chain_id = self
            .protocol_config
            .provider()
            .get_chain_id()
            .await
            .context("eth_chainId")?;
        let latest = self
            .protocol_config
            .getCurrentKmsContextIdCounter()
            .block(at)
            .call()
            .await
            .context("getCurrentKmsContextIdCounter")?;
        let mut senders = HashSet::new();
        let mut context_id = KMS_CONTEXT_COUNTER_BASE + U256::from(1);
        while context_id <= latest {
            let active = self
                .protocol_config
                .isValidKmsContext(context_id)
                .block(at)
                .call()
                .await
                .with_context(|| format!("isValidKmsContext({context_id})"))?;
            if active {
                if !self.nodes_by_context.contains_key(&context_id) {
                    let nodes = self
                        .protocol_config
                        .getKmsNodesForContext(context_id)
                        .block(at)
                        .call()
                        .await
                        .with_context(|| {
                            format!("getKmsNodesForContext({context_id})")
                        })?;
                    self.nodes_by_context.insert(
                        context_id,
                        nodes.iter().map(|node| node.txSenderAddress).collect(),
                    );
                }
                senders.extend(&self.nodes_by_context[&context_id]);
            }
            context_id += U256::from(1);
        }
        Ok(KmsTxSenderSet {
            registry: KeyRegistry {
                chain_id,
                contract: *self.protocol_config.address(),
            },
            senders,
        })
    }
}

/// A node whose `ProtocolConfig` answers as the contract does, for this module's tests and the
/// server's.
#[cfg(test)]
pub(crate) mod stub {
    use super::*;
    use alloy::{
        primitives::Bytes,
        providers::{mock::Asserter, ProviderBuilder},
        rpc::{
            client::RpcClient,
            json_rpc::{RequestPacket, ResponsePacket, SerializedRequest},
            types::Block,
        },
        sol_types::{SolCall, SolInterface},
        transports::{mock::MockTransport, TransportError, TransportFut},
    };
    use fhevm_host_bindings::protocol_config::ProtocolConfig::{
        self, getCurrentKmsContextIdCounterCall, getKmsNodesForContextCall,
        isValidKmsContextCall, KmsNode, ProtocolConfigCalls,
    };
    use std::sync::Mutex;

    pub(crate) const CONTRACT: Address = Address::repeat_byte(0xC0);
    /// The finalized block before any [`Ledger::finalize`].
    pub(crate) const FINALIZED: u64 = 0x7b;

    #[derive(Clone, Copy, PartialEq)]
    pub(crate) enum State {
        Pending,
        Created,
        Active,
        Destroyed,
    }

    /// What the stub node answers from.
    pub(crate) struct Ledger {
        pub finalized: u64,
        /// The KMS contexts from context 1 on: each one's state and its nodes' tx-senders.
        pub contexts: Vec<(State, Vec<Address>)>,
        /// While false, every call stays unanswered.
        pub answering: bool,
    }

    impl Ledger {
        /// Applies `change` to the contexts in a new finalized block.
        pub(crate) fn finalize(
            &mut self,
            change: impl FnOnce(&mut Vec<(State, Vec<Address>)>),
        ) {
            change(&mut self.contexts);
            self.finalized += 1;
        }
    }

    pub(crate) type Requests = Arc<Mutex<Vec<SerializedRequest>>>;

    pub(crate) fn ledger(
        contexts: &[(State, &[Address])],
    ) -> Arc<Mutex<Ledger>> {
        Arc::new(Mutex::new(Ledger {
            finalized: FINALIZED,
            contexts: contexts
                .iter()
                .map(|(state, tx_senders)| (*state, tx_senders.to_vec()))
                .collect(),
            answering: true,
        }))
    }

    pub(crate) fn context(n: u64) -> U256 {
        KMS_CONTEXT_COUNTER_BASE + U256::from(n)
    }

    /// `isValidKmsContext` is true only for an Active context, and `getKmsNodesForContext` reverts
    /// for any other. It keeps every request.
    #[derive(Clone)]
    struct Node {
        ledger: Arc<Mutex<Ledger>>,
        answers: MockTransport,
        requests: Requests,
    }

    impl Node {
        /// Queues the answer to `request` for the mock transport.
        fn answer(&self, ledger: &Ledger, request: &SerializedRequest) {
            let asserter = self.answers.asserter();
            match request.method() {
                "eth_chainId" => asserter.push_success(&"0x3039"),
                "eth_getBlockByNumber" => {
                    let mut block = Block::<()>::default();
                    block.header.inner.number = ledger.finalized;
                    asserter.push_success(&block);
                }
                "eth_call" => {
                    let contexts = &ledger.contexts;
                    let get = |id: U256| {
                        let index: usize = (id - context(1)).to();
                        &contexts[index]
                    };
                    let returned = match ProtocolConfigCalls::abi_decode(
                        &call_input(request),
                    )
                    .unwrap()
                    {
                        ProtocolConfigCalls::getCurrentKmsContextIdCounter(_) => {
                            Some(getCurrentKmsContextIdCounterCall::abi_encode_returns(
                                &context(contexts.len() as u64),
                            ))
                        }
                        ProtocolConfigCalls::isValidKmsContext(call) => {
                            Some(isValidKmsContextCall::abi_encode_returns(
                                &(get(call.kmsContextId).0 == State::Active),
                            ))
                        }
                        ProtocolConfigCalls::getKmsNodesForContext(call) => {
                            let (state, tx_senders) = get(call.kmsContextId);
                            (*state == State::Active).then(|| {
                                let nodes: Vec<KmsNode> = tx_senders
                                    .iter()
                                    .map(|&tx_sender| KmsNode {
                                        txSenderAddress: tx_sender,
                                        ..Default::default()
                                    })
                                    .collect();
                                getKmsNodesForContextCall::abi_encode_returns(&nodes)
                            })
                        }
                        call => panic!("unexpected call {:?}", call.selector()),
                    };
                    match returned {
                        Some(returned) => {
                            asserter.push_success(&Bytes::from(returned))
                        }
                        None => asserter.push_failure_msg(
                            "execution reverted: InvalidKmsContext",
                        ),
                    }
                }
                method => panic!("unexpected RPC method {method}"),
            }
        }
    }

    impl tower::Service<RequestPacket> for Node {
        type Response = ResponsePacket;
        type Error = TransportError;
        type Future = TransportFut<'static>;

        fn poll_ready(
            &mut self,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Result<(), Self::Error>> {
            self.answers.poll_ready(cx)
        }

        fn call(&mut self, request: RequestPacket) -> Self::Future {
            let ledger = self.ledger.lock().unwrap();
            for each in request.requests() {
                self.requests.lock().unwrap().push(each.clone());
                if ledger.answering {
                    self.answer(&ledger, each);
                }
            }
            if !ledger.answering {
                return Box::pin(std::future::pending());
            }
            self.answers.call(request)
        }
    }

    pub(crate) fn params(request: &SerializedRequest) -> serde_json::Value {
        serde_json::from_str(request.params().unwrap().get()).unwrap()
    }

    pub(crate) fn call_input(request: &SerializedRequest) -> Bytes {
        serde_json::from_value(params(request)[0]["input"].clone()).unwrap()
    }

    /// The `ProtocolConfig` at [`CONTRACT`] of a node answering from `ledger`, and the requests
    /// it receives.
    pub(crate) fn protocol_config(
        ledger: &Arc<Mutex<Ledger>>,
    ) -> (ProtocolConfigInstance<impl Provider>, Requests) {
        let requests = Requests::default();
        let node = Node {
            ledger: Arc::clone(ledger),
            answers: MockTransport::new(Asserter::new()),
            requests: Arc::clone(&requests),
        };
        let provider =
            ProviderBuilder::new().connect_client(RpcClient::new(node, true));
        (ProtocolConfig::new(CONTRACT, provider), requests)
    }

    /// A follower of `ledger` polling every `poll_interval`, once its first read is published.
    pub(crate) async fn follower(
        ledger: &Arc<Mutex<Ledger>>,
        poll_interval: Duration,
    ) -> (KmsTxSenders, Requests, CancellationToken) {
        let (protocol_config, requests) = protocol_config(ledger);
        let cancel = CancellationToken::new();
        let senders = follow(protocol_config, poll_interval, cancel.clone());
        while !senders.is_loaded() {
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
        (senders, requests, cancel)
    }

    /// How many requests in `requests` call `selector`.
    pub(crate) fn calls(requests: &Requests, selector: [u8; 4]) -> usize {
        requests
            .lock()
            .unwrap()
            .iter()
            .filter(|request| {
                request.method() == "eth_call"
                    && call_input(request).starts_with(&selector)
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::{stub::*, *};
    use alloy::{providers::ProviderBuilder, sol_types::SolCall};
    use fhevm_host_bindings::protocol_config::ProtocolConfig::{
        self, getCurrentKmsContextIdCounterCall, getKmsNodesForContextCall,
    };
    use std::sync::Mutex;

    const PREVIOUS: Address = Address::repeat_byte(0xA1);
    const NEXT: Address = Address::repeat_byte(0xB2);
    /// The binary's default.
    const POLL_INTERVAL: Duration = Duration::from_secs(5);
    const WAIT: Duration = Duration::from_secs(3);

    fn reader(
        ledger: &Arc<Mutex<Ledger>>,
    ) -> (KmsTxSenderReader<impl Provider>, Requests) {
        let (protocol_config, requests) = protocol_config(ledger);
        (KmsTxSenderReader::new(protocol_config), requests)
    }

    async fn read<P: Provider>(
        reader: &mut KmsTxSenderReader<P>,
    ) -> HashSet<Address> {
        reader
            .poll()
            .await
            .expect("poll")
            .expect("a new finalized block")
            .senders
    }

    fn accepts(senders: &KmsTxSenders, tx_sender: Address) -> bool {
        senders.current().unwrap().senders.contains(&tx_sender)
    }

    /// An Active context's tx-senders are accepted. Its nodes are read once; the next read
    /// reuses them.
    #[tokio::test]
    async fn accepts_an_active_contexts_tx_senders_and_reads_its_nodes_once() {
        let ledger = ledger(&[(State::Active, &[PREVIOUS, NEXT])]);
        let (mut reader, requests) = reader(&ledger);
        let set = reader.poll().await.expect("poll").expect("first read");
        assert_eq!(
            set.registry,
            KeyRegistry {
                chain_id: 12345,
                contract: CONTRACT
            }
        );
        assert_eq!(set.senders, HashSet::from([PREVIOUS, NEXT]));
        ledger.lock().unwrap().finalize(|_| {});
        assert_eq!(read(&mut reader).await, HashSet::from([PREVIOUS, NEXT]));
        assert_eq!(calls(&requests, getKmsNodesForContextCall::SELECTOR), 1);
    }

    /// A Pending context, defined by governance, and a Created one, whose nodes have not attested
    /// their keys, are refused while the previous context stays accepted. Once the new context is
    /// Active, both are accepted.
    #[tokio::test]
    async fn refuses_a_pending_or_created_contexts_tx_senders() {
        let ledger =
            ledger(&[(State::Active, &[PREVIOUS]), (State::Pending, &[NEXT])]);
        let (mut reader, _) = reader(&ledger);
        assert_eq!(read(&mut reader).await, HashSet::from([PREVIOUS]));

        ledger
            .lock()
            .unwrap()
            .finalize(|contexts| contexts[1].0 = State::Created);
        assert_eq!(read(&mut reader).await, HashSet::from([PREVIOUS]));

        ledger
            .lock()
            .unwrap()
            .finalize(|contexts| contexts[1].0 = State::Active);
        assert_eq!(read(&mut reader).await, HashSet::from([PREVIOUS, NEXT]));
    }

    /// Destroying a context refuses its tx-senders, although its nodes were read before.
    #[tokio::test]
    async fn refuses_a_destroyed_contexts_tx_senders() {
        let ledger =
            ledger(&[(State::Active, &[PREVIOUS]), (State::Active, &[NEXT])]);
        let (mut reader, _) = reader(&ledger);
        assert_eq!(read(&mut reader).await, HashSet::from([PREVIOUS, NEXT]));

        ledger
            .lock()
            .unwrap()
            .finalize(|contexts| contexts[0].0 = State::Destroyed);
        assert_eq!(read(&mut reader).await, HashSet::from([NEXT]));
    }

    /// The counter, the Active flag and the context's nodes are all read at the finalized block
    /// the poll resolved first.
    #[tokio::test]
    async fn a_refresh_reads_every_contract_value_at_one_block() {
        let ledger = ledger(&[(State::Active, &[PREVIOUS])]);
        let (mut reader, requests) = reader(&ledger);
        read(&mut reader).await;

        let blocks: Vec<String> = requests
            .lock()
            .unwrap()
            .iter()
            .filter(|request| request.method() == "eth_call")
            .map(|request| params(request)[1].to_string())
            .collect();
        assert_eq!(blocks, vec![format!("\"{FINALIZED:#x}\""); 3]);
    }

    /// A poll that finds the finalized block of the last read reads nothing else.
    #[tokio::test]
    async fn an_unchanged_finalized_block_reads_no_contract_state() {
        let ledger = ledger(&[(State::Active, &[PREVIOUS])]);
        let (mut reader, requests) = reader(&ledger);
        read(&mut reader).await;
        let before = requests.lock().unwrap().len();

        assert_eq!(reader.poll().await.expect("poll"), None);
        let methods: Vec<String> = requests.lock().unwrap()[before..]
            .iter()
            .map(|request| request.method().to_owned())
            .collect();
        assert_eq!(methods, ["eth_getBlockByNumber"]);
    }

    /// An older finalized block, from an RPC node behind the one last read, reads nothing, so the
    /// set it would give is not published.
    #[tokio::test]
    async fn an_older_finalized_block_reads_nothing() {
        let ledger = ledger(&[(State::Active, &[PREVIOUS])]);
        let (mut reader, _) = reader(&ledger);
        read(&mut reader).await;
        ledger
            .lock()
            .unwrap()
            .finalize(|contexts| contexts.push((State::Active, vec![NEXT])));
        assert_eq!(read(&mut reader).await, HashSet::from([PREVIOUS, NEXT]));

        // The lagging node answers with the block before, and the state there.
        {
            let mut lagging = ledger.lock().unwrap();
            lagging.finalized -= 1;
            lagging.contexts.pop();
        }
        assert_eq!(reader.poll().await.expect("poll"), None);
    }

    /// A context activated in a new finalized block is accepted at the next poll.
    #[tokio::test(start_paused = true)]
    async fn accepts_a_context_activated_in_a_new_finalized_block_at_the_next_poll(
    ) {
        let ledger = ledger(&[(State::Active, &[PREVIOUS])]);
        let (senders, _, cancel) = follower(&ledger, POLL_INTERVAL).await;
        ledger
            .lock()
            .unwrap()
            .finalize(|contexts| contexts.push((State::Active, vec![NEXT])));

        tokio::time::sleep(POLL_INTERVAL + Duration::from_millis(1)).await;
        assert!(accepts(&senders, NEXT));
        cancel.cancel();
    }

    /// Concurrent misses before a poll starts share that one poll.
    #[tokio::test(start_paused = true)]
    async fn concurrent_misses_share_one_read() {
        let ledger = ledger(&[(State::Active, &[PREVIOUS])]);
        let (senders, requests, cancel) =
            follower(&ledger, Duration::from_secs(3600)).await;
        ledger
            .lock()
            .unwrap()
            .finalize(|contexts| contexts.push((State::Active, vec![NEXT])));

        let mut misses = tokio::task::JoinSet::new();
        for _ in 0..10 {
            let senders = senders.clone();
            misses.spawn(async move { senders.refresh_after_miss(WAIT).await });
        }
        misses.join_all().await;
        assert!(accepts(&senders, NEXT));
        // Long enough for any further woken poll.
        tokio::time::sleep(Duration::from_secs(60)).await;
        let polls = requests
            .lock()
            .unwrap()
            .iter()
            .filter(|request| request.method() == "eth_getBlockByNumber")
            .count();
        assert_eq!(polls, 2);
        assert_eq!(
            calls(&requests, getCurrentKmsContextIdCounterCall::SELECTOR),
            2
        );
        cancel.cancel();
    }

    /// A woken poll starts at least `WOKEN_POLL_SPACING` after the poll before it started.
    #[tokio::test(start_paused = true)]
    async fn woken_polls_start_at_least_a_second_apart() {
        let ledger = ledger(&[(State::Active, &[PREVIOUS])]);
        let (senders, _, cancel) =
            follower(&ledger, Duration::from_secs(3600)).await;
        // Each miss below comes as the poll before it starts.
        senders.refresh_after_miss(WAIT).await;
        for _ in 0..2 {
            let missed = tokio::time::Instant::now();
            senders.refresh_after_miss(WAIT).await;
            assert!(missed.elapsed() >= WOKEN_POLL_SPACING);
        }
        cancel.cancel();
    }

    /// A miss waits at most its bound, even when the poll it woke never ends.
    #[tokio::test(start_paused = true)]
    async fn a_miss_waits_at_most_its_bound() {
        let ledger = ledger(&[(State::Active, &[PREVIOUS])]);
        let (senders, _, cancel) =
            follower(&ledger, Duration::from_secs(3600)).await;
        ledger.lock().unwrap().answering = false;

        let missed = tokio::time::Instant::now();
        tokio::time::timeout(WAIT * 2, senders.refresh_after_miss(WAIT))
            .await
            .expect("the miss stopped waiting");
        assert!(missed.elapsed() >= WAIT);
        cancel.cancel();
    }

    #[derive(Clone, Default)]
    struct Logs(std::sync::Arc<std::sync::Mutex<Vec<u8>>>);

    impl std::io::Write for Logs {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for Logs {
        type Writer = Self;
        fn make_writer(&'a self) -> Self {
            self.clone()
        }
    }

    impl Logs {
        fn text(&self) -> String {
            String::from_utf8(self.0.lock().unwrap().clone()).unwrap()
        }
    }

    /// The read failure is logged from the follower's own task, so the test runs on one thread
    /// and captures that thread's events: `tracing_test` keeps only the test's own span.
    #[tokio::test(flavor = "current_thread")]
    async fn a_failed_read_logs_the_rpc_origin_without_its_keys() {
        let logs = Logs::default();
        let _subscriber = tracing::subscriber::set_default(
            tracing_subscriber::fmt()
                .with_writer(logs.clone())
                .with_ansi(false)
                .finish(),
        );
        // Nothing listens there, so the first call fails in the HTTP transport.
        let rpc = "http://USER-KEY:PASSWORD-KEY@127.0.0.1:1/v2/PATH-KEY?apikey=QUERY-KEY";
        let provider =
            ProviderBuilder::new().connect_http(rpc.parse().unwrap());
        let cancel = CancellationToken::new();
        let _senders = follow(
            ProtocolConfig::new(CONTRACT, provider),
            POLL_INTERVAL,
            cancel.clone(),
        );

        let deadline =
            tokio::time::Instant::now() + std::time::Duration::from_secs(10);
        while !logs.text().contains("KMS tx-sender read failed") {
            assert!(
                tokio::time::Instant::now() < deadline,
                "no failed read logged"
            );
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }
        cancel.cancel();

        let text = logs.text();
        assert!(text.contains("http://127.0.0.1:1"), "{text}");
        for secret in ["USER-KEY", "PASSWORD-KEY", "PATH-KEY", "QUERY-KEY"] {
            assert!(!text.contains(secret), "{secret} in {text}");
        }
    }
}
