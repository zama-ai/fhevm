//! The KMS tx-senders that `ProtocolConfig` lists: the callers whose `FhevmSig` requests a
//! coprocessor accepts.
//!
//! A tx-sender is accepted only while its KMS context is Active, from the context's activation
//! until it is destroyed: the contexts for which `getKmsNodesForContext` returns the nodes. Active
//! is the first state in which a context's nodes can serve a decryption: every node has attested
//! the same keys and CRS on chain (`confirmEpochActivation`, unanimous). The requests a coprocessor
//! authenticates this way are made only to serve a decryption, so a Pending or Created context has
//! none to make, and its nodes cannot make a coprocessor spend work. The outgoing context stays
//! Active until it is destroyed, so both node sets are accepted during a switch. The contract's
//! own `isKmsTxSenderForContext` also accepts Created contexts, because their nodes send
//! confirmation transactions during resharing. A request to a coprocessor is not part of the
//! switch.
//!
//! A [`Follower`] polls every poll interval: it reads the number of the latest finalized block and,
//! when that block is newer than the one of the last successful read, reads the set at that block
//! and replaces the content of a [`RefreshableSigners`]. An older block comes from a lagging or
//! load-balanced RPC node, and reading at it could bring back an older set. A context switch takes
//! effect within one poll interval of its block being finalized. A failed poll keeps the last set.
//! Until the first read succeeds the set is empty and [`Loaded::is_loaded`] is false.
//!
//! A KMS connector serves a new context as soon as its activation is finalized, and treats a
//! refusal as final. So a request signed by an unknown tx-sender wakes the loop
//! ([`Follower::refresh_after_miss`]) and is checked again once a poll that started after it has
//! finished, or once the caller's wait has passed, whichever comes first.
//! Any header whose signature recovers an address can cause such a wake: a signature made for
//! another audience, path or body recovers an unrelated address, and the body is read only after
//! authentication. Woken polls start at least [`WOKEN_POLL_SPACING`] after the previous poll
//! started, so unknown signers cost at most one finalized-block query a second. What remains: the
//! first requests of a new context are refused when this RPC node sees the finalized block later
//! than the connector's node by more than the caller's wait, or when the woken poll fails or takes
//! longer than that wait.

use alloy::{
    eips::{BlockId, BlockNumberOrTag},
    primitives::{Address, U256},
    providers::Provider,
    transports::{RpcError, TransportError, TransportErrorKind},
};
use anyhow::{anyhow, Context};
use fhevm_host_bindings::i_protocol_config_base::IProtocolConfigBase::{
    self, IProtocolConfigBaseInstance,
};
use fhevm_sig_auth::RefreshableSigners;
use prometheus::{register_int_gauge, IntGauge};
use std::{
    collections::{HashMap, HashSet},
    future::Future,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, LazyLock,
    },
    time::Duration,
};
use tokio::{
    sync::{watch, Mutex},
    time::Instant,
};
use tracing::{info, warn};

/// The RPC client has no request timeout, so a hung call would stall every later poll.
const READ_TIMEOUT: Duration = Duration::from_secs(30);
/// The least time from the start of a poll to the start of a poll woken by a miss.
pub const WOKEN_POLL_SPACING: Duration = Duration::from_secs(1);

static KMS_TX_SENDERS: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "coprocessor_kms_tx_senders",
        "KMS tx-senders the last successful read of ProtocolConfig allows"
    )
    .unwrap()
});

static KMS_TX_SENDERS_READ_TIMESTAMP: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "coprocessor_kms_tx_senders_read_timestamp_seconds",
        "Unix time the KMS tx-sender set was last confirmed current at the finalized block"
    )
    .unwrap()
});

/// Whether the first read of the set has succeeded.
#[derive(Clone, Debug, Default)]
pub struct Loaded(Arc<AtomicBool>);

impl Loaded {
    pub fn is_loaded(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

/// The polls of a [`Follower`], numbered from 1.
#[derive(Clone, Copy, Debug, Default)]
struct Polls {
    /// The number of the last poll started.
    started: u64,
    /// The number of the last poll finished.
    finished: u64,
    /// The number of the poll a miss is waiting for: it starts as soon as the spacing allows.
    wanted: u64,
}

/// Keeps a [`RefreshableSigners`] equal to the set `ProtocolConfig` lists.
pub struct Follower<P> {
    reader: Mutex<Reader<P>>,
    signers: RefreshableSigners,
    poll_interval: Duration,
    loaded: Loaded,
    polls: watch::Sender<Polls>,
}

impl<P: Provider> Follower<P> {
    /// Polls the `ProtocolConfig` contract at `protocol_config` through `provider` every
    /// `poll_interval`.
    pub fn new(
        provider: P,
        protocol_config: Address,
        signers: RefreshableSigners,
        poll_interval: Duration,
    ) -> Self {
        Self {
            reader: Mutex::new(Reader::new(IProtocolConfigBase::new(
                protocol_config,
                provider,
            ))),
            signers,
            poll_interval,
            loaded: Loaded::default(),
            polls: watch::Sender::new(Polls::default()),
        }
    }

    pub fn loaded(&self) -> Loaded {
        self.loaded.clone()
    }

    /// Polls every `poll_interval`, and sooner for a miss, until `stop` completes.
    pub async fn run(&self, stop: impl Future<Output = ()>) {
        // A set never read shows as no tx-senders, not as a missing series.
        KMS_TX_SENDERS.set(0);
        let mut reader = self.reader.lock().await;
        let mut polls = self.polls.subscribe();
        let mut last = None;
        tokio::pin!(stop);
        loop {
            let started_at = Instant::now();
            self.polls.send_modify(|polls| polls.started += 1);
            let poll = tokio::time::timeout(READ_TIMEOUT, reader.poll())
                .await
                .unwrap_or_else(|_| Err(anyhow!("no answer within {READ_TIMEOUT:?}")));
            match poll {
                Ok(read) => {
                    if let Some(senders) = read {
                        if last.as_ref() != Some(&senders) {
                            info!(senders = senders.len(), "KMS tx-sender set updated");
                        }
                        KMS_TX_SENDERS.set(senders.len() as i64);
                        self.signers.replace(senders.iter().copied());
                        self.loaded.0.store(true, Ordering::Relaxed);
                        last = Some(senders);
                    }
                    KMS_TX_SENDERS_READ_TIMESTAMP.set(fhevm_sig_auth::unix_now() as i64);
                }
                Err(err) => warn!(
                    error = describe_read_error(&err),
                    "KMS tx-sender read failed; keeping the last set"
                ),
            }
            self.polls
                .send_modify(|polls| polls.finished = polls.started);
            tokio::select! {
                _ = &mut stop => return,
                _ = tokio::time::sleep(self.poll_interval) => {}
                _ = async {
                    let _ = polls.wait_for(|polls| polls.wanted > polls.started).await;
                    tokio::time::sleep_until(started_at + WOKEN_POLL_SPACING).await;
                } => {}
            }
        }
    }

    /// Wakes the poll loop for a request signed by an unknown tx-sender, and waits at most `wait`
    /// for a poll that started after this call to finish. Concurrent misses share that poll.
    pub async fn refresh_after_miss(&self, wait: Duration) {
        let mut polls = self.polls.subscribe();
        let mut after = 0;
        // Only the first miss for the next poll notifies: each notification wakes every waiter.
        self.polls.send_if_modified(|polls| {
            after = polls.started;
            let first = polls.wanted <= polls.started;
            polls.wanted = polls.started + 1;
            first
        });
        let _ = tokio::time::timeout(wait, async {
            let _ = polls.wait_for(|polls| polls.finished > after).await;
        })
        .await;
    }
}

struct Reader<P> {
    protocol_config: IProtocolConfigBaseInstance<P>,
    /// A context's nodes never change once it is created.
    nodes_by_context: HashMap<U256, Vec<Address>>,
    /// The finalized block of the last successful read.
    read_at: Option<u64>,
}

impl<P: Provider> Reader<P> {
    fn new(protocol_config: IProtocolConfigBaseInstance<P>) -> Self {
        Self {
            protocol_config,
            nodes_by_context: HashMap::new(),
            read_at: None,
        }
    }

    /// The set at the latest finalized block, or `None` when that block is not newer than the one
    /// last read.
    async fn poll(&mut self) -> anyhow::Result<Option<HashSet<Address>>> {
        let finalized = self
            .protocol_config
            .provider()
            .get_block_by_number(BlockNumberOrTag::Finalized)
            .await
            .context("finalized block")?
            .context("no finalized block")?
            .header
            .number;
        if self.read_at.is_some_and(|read_at| finalized <= read_at) {
            return Ok(None);
        }
        let senders = self.read(finalized).await?;
        self.read_at = Some(finalized);
        Ok(Some(senders))
    }

    async fn read(&mut self, finalized: u64) -> anyhow::Result<HashSet<Address>> {
        // Every call reads the same block, so the set is one state of `ProtocolConfig`.
        let block = BlockId::number(finalized);
        let counter = self
            .protocol_config
            .getCurrentKmsContextIdCounter()
            .block(block)
            .call()
            .await
            .context("getCurrentKmsContextIdCounter")?;
        let mut senders = HashSet::new();
        let Some(mut context_id) = first_context_id(counter) else {
            return Ok(senders);
        };
        while context_id <= counter {
            let active = self
                .protocol_config
                .isValidKmsContext(context_id)
                .block(block)
                .call()
                .await
                .with_context(|| format!("isValidKmsContext({context_id})"))?;
            if active {
                if !self.nodes_by_context.contains_key(&context_id) {
                    let nodes = self
                        .protocol_config
                        .getKmsNodesForContext(context_id)
                        .block(block)
                        .call()
                        .await
                        .with_context(|| format!("getKmsNodesForContext({context_id})"))?;
                    self.nodes_by_context.insert(
                        context_id,
                        nodes.iter().map(|node| node.txSenderAddress).collect(),
                    );
                }
                senders.extend(&self.nodes_by_context[&context_id]);
            }
            context_id += U256::ONE;
        }
        Ok(senders)
    }
}

/// A failed read as logged: the step that failed, which is our own text, and the kind of RPC
/// failure. Nothing the RPC side wrote is kept: the RPC URL often carries the provider's API key,
/// and an error body or message can echo it, with or without the scheme.
fn describe_read_error(err: &anyhow::Error) -> String {
    let rpc_failure = err.chain().find_map(|cause| {
        if let Some(error) = cause.downcast_ref::<TransportError>() {
            return Some(describe_transport_error(error));
        }
        match cause.downcast_ref::<alloy::contract::Error>()? {
            alloy::contract::Error::TransportError(error) => Some(describe_transport_error(error)),
            // The usual cause: a wrong ProtocolConfig address or chain.
            alloy::contract::Error::ZeroData(..) => Some("no contract at the address".into()),
            _ => Some("undecodable contract answer".into()),
        }
    });
    match rpc_failure {
        Some(failure) => format!("{err}: {failure}"),
        None => err.to_string(),
    }
}

fn describe_transport_error(error: &TransportError) -> String {
    match error {
        RpcError::ErrorResp(payload) => format!("JSON-RPC error {}", payload.code),
        RpcError::Transport(TransportErrorKind::HttpError(error)) => {
            format!("HTTP status {}", error.status)
        }
        RpcError::DeserError { .. } => "invalid JSON-RPC answer".into(),
        RpcError::NullResp => "null JSON-RPC answer".into(),
        _ => "RPC transport failure".into(),
    }
}

/// The first KMS context id under `counter`, the last id `ProtocolConfig` allocated. Ids are
/// `KMS_CONTEXT_COUNTER_BASE` (host-contracts `shared/Constants.sol`: a type tag in the top byte)
/// plus a counter from 1, so the first id keeps the counter's top byte. `None` before any context.
fn first_context_id(counter: U256) -> Option<U256> {
    const TYPE_TAG_SHIFT: usize = 248;
    (!counter.is_zero()).then(|| ((counter >> TYPE_TAG_SHIFT) << TYPE_TAG_SHIFT) + U256::ONE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        primitives::Bytes,
        providers::ProviderBuilder,
        rpc::{
            client::ClientBuilder,
            json_rpc::{RequestPacket, ResponsePacket},
            types::Block,
        },
        sol_types::SolCall,
        transports::{
            mock::{Asserter, MockTransport},
            TransportError, TransportFut,
        },
    };
    use fhevm_host_bindings::i_protocol_config_base::IProtocolConfigBase::{
        getCurrentKmsContextIdCounterCall, getKmsNodesForContextCall, isValidKmsContextCall,
        KmsNode,
    };
    use fhevm_sig_auth::SignerAllowlist;
    use std::{
        sync::Arc,
        task::{Context, Poll},
    };

    fn context_id(n: u64) -> U256 {
        (U256::from(7) << 248) + U256::from(n)
    }

    fn finalized_at(number: u64) -> Block {
        let mut block: Block = Block::default();
        block.header.inner.number = number;
        block
    }

    fn finalized() -> Block {
        finalized_at(100)
    }

    fn counter(n: u64) -> Bytes {
        getCurrentKmsContextIdCounterCall::abi_encode_returns(&context_id(n)).into()
    }

    fn valid(active: bool) -> Bytes {
        isValidKmsContextCall::abi_encode_returns(&active).into()
    }

    fn nodes(tx_senders: &[Address]) -> Bytes {
        let nodes: Vec<KmsNode> = tx_senders
            .iter()
            .map(|&tx_sender| KmsNode {
                txSenderAddress: tx_sender,
                ..Default::default()
            })
            .collect();
        getKmsNodesForContextCall::abi_encode_returns(&nodes).into()
    }

    /// A provider whose RPC answers the finalized block, then `answers` in order: the mock does not
    /// match calls, so a call the reader should not make consumes an answer meant for another and
    /// fails to decode.
    fn provider(answers: &[Bytes]) -> (impl Provider + use<>, Asserter) {
        let asserter = Asserter::new();
        if !answers.is_empty() {
            asserter.push_success(&finalized());
        }
        for answer in answers {
            asserter.push_success(answer);
        }
        let provider = ProviderBuilder::new().connect_mocked_client(asserter.clone());
        (provider, asserter)
    }

    fn reader(answers: &[Bytes]) -> (Reader<impl Provider + use<>>, Asserter) {
        let (provider, asserter) = provider(answers);
        let reader = Reader::new(IProtocolConfigBase::new(Address::ZERO, provider));
        (reader, asserter)
    }

    #[tokio::test]
    async fn only_active_contexts_count_and_their_nodes_are_read_once() {
        let (a, b, c) = (
            Address::repeat_byte(1),
            Address::repeat_byte(2),
            Address::repeat_byte(3),
        );
        // Context 1 destroyed, 2 and 3 Active (a switch in progress), 4 Created.
        let (mut reader, asserter) = reader(&[
            counter(4),
            valid(false),
            valid(true),
            nodes(&[a, b]),
            valid(true),
            nodes(&[b, c]),
            valid(false),
        ]);
        assert_eq!(
            reader.poll().await.unwrap().unwrap(),
            HashSet::from([a, b, c])
        );
        assert!(asserter.read_q().is_empty());

        // Context 2 destroyed since: its nodes leave the set, and no node list is read again.
        asserter.push_success(&finalized_at(101));
        asserter.push_success(&counter(4));
        for active in [false, false, true, false] {
            asserter.push_success(&valid(active));
        }
        assert_eq!(reader.poll().await.unwrap().unwrap(), HashSet::from([b, c]));
        assert!(asserter.read_q().is_empty());
    }

    #[tokio::test]
    async fn no_context_is_no_tx_sender() {
        let (mut reader, asserter) =
            reader(&[getCurrentKmsContextIdCounterCall::abi_encode_returns(&U256::ZERO).into()]);
        assert!(reader.poll().await.unwrap().unwrap().is_empty());
        assert!(asserter.read_q().is_empty());
    }

    #[tokio::test]
    async fn a_failed_call_fails_the_whole_read() {
        let (mut reader, asserter) = reader(&[counter(1)]);
        asserter.push_failure_msg("execution reverted");
        assert!(reader.poll().await.is_err());
    }

    /// The mocked RPC, recording the parameters of each `eth_call` it answers.
    #[derive(Clone)]
    struct RecordedCalls {
        transport: MockTransport,
        params: Arc<std::sync::Mutex<Vec<String>>>,
    }

    impl tower::Service<RequestPacket> for RecordedCalls {
        type Response = ResponsePacket;
        type Error = TransportError;
        type Future = TransportFut<'static>;

        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), TransportError>> {
            self.transport.poll_ready(cx)
        }

        fn call(&mut self, request: RequestPacket) -> Self::Future {
            if let RequestPacket::Single(call) = &request {
                if call.method() == "eth_call" {
                    let params = call
                        .params()
                        .map_or_else(String::new, |p| p.get().to_owned());
                    self.params.lock().unwrap().push(params);
                }
            }
            self.transport.call(request)
        }
    }

    /// A provider answering `asserter`'s answers in order, and the record of its `eth_call`s.
    fn recorded_provider(asserter: Asserter) -> (impl Provider + use<>, RecordedCalls) {
        let recorded = RecordedCalls {
            transport: MockTransport::new(asserter),
            params: Arc::default(),
        };
        let client = ClientBuilder::default().transport(recorded.clone(), true);
        (ProviderBuilder::new().connect_client(client), recorded)
    }

    /// Answers a first poll that finds context 1 Active at block 100, with the node `a`.
    fn first_read(a: Address) -> Asserter {
        let asserter = Asserter::new();
        asserter.push_success(&finalized());
        for answer in [counter(1), valid(true), nodes(&[a])] {
            asserter.push_success(&answer);
        }
        asserter
    }

    #[tokio::test]
    async fn every_call_reads_the_finalized_block() {
        let a = Address::repeat_byte(1);
        let (provider, recorded) = recorded_provider(first_read(a));
        let mut reader = Reader::new(IProtocolConfigBase::new(Address::ZERO, provider));

        assert_eq!(reader.poll().await.unwrap().unwrap(), HashSet::from([a]));
        let params = recorded.params.lock().unwrap();
        assert_eq!(params.len(), 3, "{params:?}");
        // The finalized block is number 100.
        assert!(
            params.iter().all(|p| p.ends_with(r#","0x64"]"#)),
            "{params:?}"
        );
    }

    const POLL_INTERVAL: Duration = Duration::from_millis(20);

    /// Runs `follower` for `polls` polls, in paused time: no answer takes time.
    async fn run_polls<P: Provider>(follower: &Follower<P>, polls: u32) {
        let stop = POLL_INTERVAL * (polls - 1) + POLL_INTERVAL / 2;
        follower.run(tokio::time::sleep(stop)).await;
    }

    fn follower<P: Provider>(provider: P, signers: &RefreshableSigners) -> Follower<P> {
        Follower::new(provider, Address::ZERO, signers.clone(), POLL_INTERVAL)
    }

    #[tokio::test(start_paused = true)]
    async fn a_context_activated_at_a_new_finalized_block_is_read_at_the_next_poll() {
        let (a, b) = (Address::repeat_byte(1), Address::repeat_byte(2));
        let asserter = first_read(a);
        // Context 2 is Active at block 101; context 1 still is, its nodes already known.
        asserter.push_success(&finalized_at(101));
        for answer in [counter(2), valid(true), valid(true), nodes(&[b])] {
            asserter.push_success(&answer);
        }
        let provider = ProviderBuilder::new().connect_mocked_client(asserter.clone());
        let signers = RefreshableSigners::default();

        run_polls(&follower(provider, &signers), 2).await;

        assert!(signers.allows(a));
        assert!(signers.allows(b));
    }

    #[tokio::test(start_paused = true)]
    async fn an_unchanged_finalized_block_reads_no_contract_state() {
        let a = Address::repeat_byte(1);
        let asserter = first_read(a);
        for _ in 0..3 {
            asserter.push_success(&finalized());
        }
        let (provider, recorded) = recorded_provider(asserter.clone());
        let signers = RefreshableSigners::default();

        run_polls(&follower(provider, &signers), 4).await;

        // The first poll read the set, the three others only the block.
        assert!(asserter.read_q().is_empty());
        assert_eq!(recorded.params.lock().unwrap().len(), 3);
        assert!(signers.allows(a));
    }

    #[tokio::test(start_paused = true)]
    async fn an_older_finalized_block_reads_no_contract_state() {
        let a = Address::repeat_byte(1);
        let asserter = first_read(a);
        // A lagging RPC node, at a block before the context's activation.
        asserter.push_success(&finalized_at(99));
        for answer in [counter(1), valid(false)] {
            asserter.push_success(&answer);
        }
        let (provider, recorded) = recorded_provider(asserter);
        let signers = RefreshableSigners::default();

        run_polls(&follower(provider, &signers), 2).await;

        assert_eq!(recorded.params.lock().unwrap().len(), 3);
        assert!(signers.allows(a));
    }

    #[tokio::test(start_paused = true)]
    async fn a_read_that_fails_at_a_new_block_is_retried_at_that_block() {
        let (a, b) = (Address::repeat_byte(1), Address::repeat_byte(2));
        let asserter = first_read(a);
        // Block 101, where context 2 is Active: the first read of it fails after the block number.
        asserter.push_success(&finalized_at(101));
        asserter.push_failure_msg("connection refused");
        asserter.push_success(&finalized_at(101));
        for answer in [counter(2), valid(true), valid(true), nodes(&[b])] {
            asserter.push_success(&answer);
        }
        let provider = ProviderBuilder::new().connect_mocked_client(asserter.clone());
        let signers = RefreshableSigners::default();

        run_polls(&follower(provider, &signers), 3).await;

        assert!(asserter.read_q().is_empty());
        assert!(signers.allows(b));
    }

    #[tokio::test(start_paused = true)]
    async fn a_failed_read_keeps_the_last_set() {
        let a = Address::repeat_byte(1);
        let (provider, asserter) = provider(&[counter(1), valid(true), nodes(&[a])]);
        asserter.push_failure_msg("connection refused");
        let signers = RefreshableSigners::default();
        let follower = follower(provider, &signers);

        run_polls(&follower, 2).await;

        assert!(follower.loaded().is_loaded());
        assert!(signers.allows(a));
    }

    #[tokio::test(start_paused = true)]
    async fn nothing_is_loaded_before_the_first_successful_read() {
        let (provider, asserter) = provider(&[]);
        asserter.push_failure_msg("connection refused");
        let signers = RefreshableSigners::default();
        let follower = follower(provider, &signers);

        run_polls(&follower, 1).await;

        assert!(!follower.loaded().is_loaded());
        assert!(!signers.allows(Address::repeat_byte(1)));
    }

    #[tokio::test]
    async fn read_errors_leave_out_what_the_rpc_wrote() {
        let echo = "error for https://rpc.example/v3/SECRET-KEY: /v3/SECRET-KEY?apikey=SECRET-KEY";

        let (mut finalized_fails, asserter) = reader(&[]);
        asserter.push_failure_msg(echo);
        let err = finalized_fails.poll().await.unwrap_err();
        assert_eq!(
            describe_read_error(&err),
            "finalized block: JSON-RPC error -32603"
        );

        // A contract call wraps the transport error in `alloy::contract::Error`.
        let (mut counter_fails, asserter) = reader(&[]);
        asserter.push_success(&finalized());
        asserter.push_failure_msg(echo);
        let err = counter_fails.poll().await.unwrap_err();
        assert_eq!(
            describe_read_error(&err),
            "getCurrentKmsContextIdCounter: JSON-RPC error -32603"
        );

        // No contract at the address: the call returns no data.
        let (mut no_contract, _asserter) = reader(&[Bytes::new()]);
        let err = no_contract.poll().await.unwrap_err();
        assert_eq!(
            describe_read_error(&err),
            "getCurrentKmsContextIdCounter: no contract at the address"
        );

        for (error, described) in [
            (
                TransportErrorKind::http_error(500, echo.to_owned()),
                "finalized block: HTTP status 500",
            ),
            (
                TransportErrorKind::custom_str(echo),
                "finalized block: RPC transport failure",
            ),
        ] {
            let err = anyhow::Error::from(error).context("finalized block");
            assert_eq!(describe_read_error(&err), described);
        }
    }

    #[test]
    fn the_first_context_id_follows_the_counters_type_tag() {
        // `getCurrentKmsContextIdCounter` after the bootstrap context and one switch.
        let counter = U256::from_be_bytes({
            let mut id = [0u8; 32];
            id[0] = 0x07;
            id[31] = 2;
            id
        });
        let mut first = [0u8; 32];
        first[0] = 0x07;
        first[31] = 1;
        assert_eq!(first_context_id(counter), Some(U256::from_be_bytes(first)));
        assert_eq!(first_context_id(U256::ZERO), None);
    }
}
