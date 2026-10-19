//! The callers the Merkle proof route accepts: the tx-sender address of every KMS node in an Active
//! KMS context of the canonical `ProtocolConfig` (DD-076). The KMS connector signs its requests with
//! that tx-sender key. A tx-sender only in a Pending or Created context is refused until that
//! context is activated; the previous context stays Active, and accepted, until it is destroyed.
//!
//! A background task reads the set at one finalized block and replaces it; requests only read the
//! last set, so a forged signature costs no RPC call. A failed refresh keeps the last set. Until
//! the first read succeeds the route refuses every request.

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

const REFRESH_INTERVAL: Duration = Duration::from_secs(60);
const RETRY_INTERVAL: Duration = Duration::from_secs(5);
/// The RPC client has no request timeout, so a hung call would stall every later refresh.
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
        "Unix time of the last successful read of the KMS tx-senders from ProtocolConfig"
    )
    .unwrap()
    },
);

/// The latest [`KmsTxSenderSet`]; `None` until the first read succeeds.
#[derive(Clone)]
pub struct KmsTxSenders(watch::Receiver<Option<Arc<KmsTxSenderSet>>>);

impl KmsTxSenders {
    /// A set that never changes.
    pub fn fixed(set: KmsTxSenderSet) -> Self {
        Self(watch::channel(Some(Arc::new(set))).1)
    }

    /// A set that is never read.
    #[cfg(test)]
    pub(crate) fn unread() -> Self {
        Self(watch::channel(None).1)
    }

    pub fn current(&self) -> Option<Arc<KmsTxSenderSet>> {
        self.0.borrow().clone()
    }

    pub fn is_loaded(&self) -> bool {
        self.0.borrow().is_some()
    }
}

/// Reads the set from `protocol_config` every [`REFRESH_INTERVAL`] until `cancel` fires.
pub fn follow<P: Provider + 'static>(
    protocol_config: ProtocolConfigInstance<P>,
    cancel: CancellationToken,
) -> KmsTxSenders {
    let (publish, senders) = watch::channel(None);
    // An allowlist never read shows as no tx-senders, not as a missing series.
    KMS_TX_SENDERS.set(0);
    tokio::spawn(async move {
        let mut reader = KmsTxSenderReader::new(protocol_config);
        loop {
            let read = tokio::time::timeout(READ_TIMEOUT, reader.read())
                .await
                .unwrap_or_else(|_| {
                    Err(anyhow!("no answer within {READ_TIMEOUT:?}"))
                });
            let delay = match read {
                Ok(set) => {
                    if publish.borrow().as_deref() != Some(&set) {
                        info!(
                            senders = set.senders.len(),
                            "KMS tx-sender set updated"
                        );
                    }
                    KMS_TX_SENDERS.set(gauge_value(set.senders.len() as u64));
                    KMS_TX_SENDERS_READ_TIMESTAMP
                        .set(gauge_value(unix_now_secs()));
                    publish.send_replace(Some(Arc::new(set)));
                    REFRESH_INTERVAL
                }
                Err(err) => {
                    warn!(error = %without_rpc_url_secrets(&err), "KMS tx-sender read failed; keeping the last set");
                    RETRY_INTERVAL
                }
            };
            tokio::select! {
                _ = cancel.cancelled() => return,
                _ = tokio::time::sleep(delay) => {}
            }
        }
    });
    KmsTxSenders(senders)
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
}

impl<P: Provider> KmsTxSenderReader<P> {
    fn new(protocol_config: ProtocolConfigInstance<P>) -> Self {
        Self {
            protocol_config,
            nodes_by_context: HashMap::new(),
        }
    }

    async fn read(&mut self) -> anyhow::Result<KmsTxSenderSet> {
        let chain_id = self
            .protocol_config
            .provider()
            .get_chain_id()
            .await
            .context("eth_chainId")?;
        // Every ProtocolConfig read of a refresh uses this one block. Read at different blocks, the
        // counter could predate a new context while `isValidKmsContext` already shows its
        // predecessor destroyed.
        let finalized = self
            .protocol_config
            .provider()
            .get_block_by_number(BlockNumberOrTag::Finalized)
            .await
            .context("finalized block")?
            .ok_or_else(|| anyhow!("no finalized block"))?
            .header
            .number;
        let at = BlockId::number(finalized);
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

#[cfg(test)]
mod tests {
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

    const CONTRACT: Address = Address::repeat_byte(0xC0);
    /// The finalized block every refresh resolves.
    const FINALIZED: u64 = 0x7b;
    const PREVIOUS: Address = Address::repeat_byte(0xA1);
    const NEXT: Address = Address::repeat_byte(0xB2);

    #[derive(Clone, Copy, PartialEq)]
    enum State {
        Pending,
        Created,
        Active,
        Destroyed,
    }

    /// The KMS contexts of the stub `ProtocolConfig`, from context 1 on: each one's state and its
    /// nodes' tx-senders.
    type Contexts = Arc<Mutex<Vec<(State, Vec<Address>)>>>;

    fn contexts(contexts: &[(State, &[Address])]) -> Contexts {
        Arc::new(Mutex::new(
            contexts
                .iter()
                .map(|(state, tx_senders)| (*state, tx_senders.to_vec()))
                .collect(),
        ))
    }

    fn context(n: u64) -> U256 {
        KMS_CONTEXT_COUNTER_BASE + U256::from(n)
    }

    /// A node whose `ProtocolConfig` answers each call as the contract does for `contexts`:
    /// `isValidKmsContext` is true only for an Active context, and `getKmsNodesForContext` reverts
    /// for any other. It keeps every request.
    #[derive(Clone)]
    struct Chain {
        contexts: Contexts,
        answers: MockTransport,
        requests: Arc<Mutex<Vec<SerializedRequest>>>,
    }

    impl Chain {
        /// Queues the answer to `request` for the mock transport.
        fn answer(&self, request: &SerializedRequest) {
            let asserter = self.answers.asserter();
            match request.method() {
                "eth_chainId" => asserter.push_success(&"0x3039"),
                "eth_getBlockByNumber" => {
                    let mut block = Block::<()>::default();
                    block.header.inner.number = FINALIZED;
                    asserter.push_success(&block);
                }
                "eth_call" => {
                    let contexts = self.contexts.lock().unwrap();
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

    impl tower::Service<RequestPacket> for Chain {
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
            for each in request.requests() {
                self.answer(each);
                self.requests.lock().unwrap().push(each.clone());
            }
            self.answers.call(request)
        }
    }

    fn params(request: &SerializedRequest) -> serde_json::Value {
        serde_json::from_str(request.params().unwrap().get()).unwrap()
    }

    fn call_input(request: &SerializedRequest) -> Bytes {
        serde_json::from_value(params(request)[0]["input"].clone()).unwrap()
    }

    fn reader(
        contexts: &Contexts,
    ) -> (
        KmsTxSenderReader<impl Provider>,
        Arc<Mutex<Vec<SerializedRequest>>>,
    ) {
        let requests = Arc::default();
        let chain = Chain {
            contexts: Arc::clone(contexts),
            answers: MockTransport::new(Asserter::new()),
            requests: Arc::clone(&requests),
        };
        let provider =
            ProviderBuilder::new().connect_client(RpcClient::new(chain, true));
        (
            KmsTxSenderReader::new(ProtocolConfig::new(CONTRACT, provider)),
            requests,
        )
    }

    /// An Active context's tx-senders are accepted. Its nodes are read once; the next refresh
    /// reuses them.
    #[tokio::test]
    async fn accepts_an_active_contexts_tx_senders_and_reads_its_nodes_once() {
        let contexts = contexts(&[(State::Active, &[PREVIOUS, NEXT])]);
        let (mut reader, requests) = reader(&contexts);
        let set = reader.read().await.expect("first read");
        assert_eq!(
            set.registry,
            KeyRegistry {
                chain_id: 12345,
                contract: CONTRACT
            }
        );
        assert_eq!(set.senders, HashSet::from([PREVIOUS, NEXT]));
        let set = reader.read().await.expect("second read");
        assert_eq!(set.senders, HashSet::from([PREVIOUS, NEXT]));
        let node_reads = requests
            .lock()
            .unwrap()
            .iter()
            .filter(|request| {
                request.method() == "eth_call"
                    && call_input(request)
                        .starts_with(&getKmsNodesForContextCall::SELECTOR)
            })
            .count();
        assert_eq!(node_reads, 1);
    }

    /// A Pending context, defined by governance, and a Created one, whose nodes have not attested
    /// their keys, are refused while the previous context stays accepted. Once the new context is
    /// Active, both are accepted.
    #[tokio::test]
    async fn refuses_a_pending_or_created_contexts_tx_senders() {
        let contexts = contexts(&[
            (State::Active, &[PREVIOUS]),
            (State::Pending, &[NEXT]),
        ]);
        let (mut reader, _) = reader(&contexts);
        let set = reader.read().await.expect("read with a Pending context");
        assert_eq!(set.senders, HashSet::from([PREVIOUS]));

        contexts.lock().unwrap()[1].0 = State::Created;
        let set = reader.read().await.expect("read with a Created context");
        assert_eq!(set.senders, HashSet::from([PREVIOUS]));

        contexts.lock().unwrap()[1].0 = State::Active;
        let set = reader.read().await.expect("read after the activation");
        assert_eq!(set.senders, HashSet::from([PREVIOUS, NEXT]));
    }

    /// Destroying a context refuses its tx-senders, although its nodes were read before.
    #[tokio::test]
    async fn refuses_a_destroyed_contexts_tx_senders() {
        let contexts =
            contexts(&[(State::Active, &[PREVIOUS]), (State::Active, &[NEXT])]);
        let (mut reader, _) = reader(&contexts);
        let set = reader.read().await.expect("read before the destruction");
        assert_eq!(set.senders, HashSet::from([PREVIOUS, NEXT]));

        contexts.lock().unwrap()[0].0 = State::Destroyed;
        let set = reader.read().await.expect("read after the destruction");
        assert_eq!(set.senders, HashSet::from([NEXT]));
    }

    /// The counter, the Active flag and the context's nodes are all read at the finalized block
    /// the refresh resolved first.
    #[tokio::test]
    async fn a_refresh_reads_every_contract_value_at_one_block() {
        let contexts = contexts(&[(State::Active, &[PREVIOUS])]);
        let (mut reader, requests) = reader(&contexts);
        reader.read().await.expect("read");

        let blocks: Vec<String> = requests
            .lock()
            .unwrap()
            .iter()
            .filter(|request| request.method() == "eth_call")
            .map(|request| params(request)[1].to_string())
            .collect();
        assert_eq!(blocks, vec![format!("\"{FINALIZED:#x}\""); 3]);
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
        let _senders =
            follow(ProtocolConfig::new(CONTRACT, provider), cancel.clone());

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
