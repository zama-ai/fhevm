//! The callers the Merkle proof route accepts: the tx-sender address of every KMS node in a live
//! KMS context of the canonical `ProtocolConfig`. The KMS connector signs its requests with that
//! tx-sender key.
//!
//! A background task reads the set at the finalized block and replaces it; requests only read the
//! last set, so a forged signature costs no RPC call. A failed refresh keeps the last set. Until
//! the first read succeeds the route refuses every request.

use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, LazyLock},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use alloy::{
    eips::BlockId,
    primitives::{Address, B256, U256},
    providers::Provider,
    rpc::types::Filter,
    sol_types::SolEvent,
};
use anyhow::{anyhow, Context};
use fhevm_host_bindings::protocol_config::ProtocolConfig::{
    NewKmsContext, ProtocolConfigInstance,
};
use prometheus::{register_int_gauge, IntGauge};
use request_authorization::KeyRegistry;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

const REFRESH_INTERVAL: Duration = Duration::from_secs(60);
const RETRY_INTERVAL: Duration = Duration::from_secs(5);
/// The RPC client has no request timeout, so a hung call would stall every later refresh.
const READ_TIMEOUT: Duration = Duration::from_secs(30);

/// Mirrors `KMS_CONTEXT_COUNTER_BASE` from `host-contracts/contracts/shared/Constants.sol`:
/// `0x07 << 248`, the top byte of the highest limb.
const KMS_CONTEXT_COUNTER_BASE: U256 = U256::from_limbs([0, 0, 0, 0x07 << 56]);

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

static KMS_TX_SENDERS_READ: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "solana_merkle_proof_server_kms_tx_senders_read_timestamp_seconds",
        "Unix time of the last successful read of the KMS tx-senders from ProtocolConfig"
    )
    .unwrap()
});

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
                    KMS_TX_SENDERS.set(
                        i64::try_from(set.senders.len()).unwrap_or(i64::MAX),
                    );
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map_or(0, |elapsed| elapsed.as_secs());
                    KMS_TX_SENDERS_READ
                        .set(i64::try_from(now).unwrap_or(i64::MAX));
                    publish.send_replace(Some(Arc::new(set)));
                    REFRESH_INTERVAL
                }
                Err(err) => {
                    warn!(error = %format!("{err:#}"), "KMS tx-sender read failed; keeping the last set");
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
        let latest = self
            .protocol_config
            .getCurrentKmsContextIdCounter()
            .block(BlockId::finalized())
            .call()
            .await
            .context("getCurrentKmsContextIdCounter")?;
        let mut senders = HashSet::new();
        let mut context_id = KMS_CONTEXT_COUNTER_BASE + U256::from(1);
        while context_id <= latest {
            let live = self
                .protocol_config
                .isLiveKmsContext(context_id)
                .block(BlockId::finalized())
                .call()
                .await
                .with_context(|| format!("isLiveKmsContext({context_id})"))?;
            if live {
                if !self.nodes_by_context.contains_key(&context_id) {
                    let nodes = self.read_tx_senders(context_id).await?;
                    self.nodes_by_context.insert(context_id, nodes);
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

    /// Reads the context's nodes from its `NewKmsContext` event, which also covers a Pending or
    /// Created context that `getKmsNodesForContext` refuses.
    async fn read_tx_senders(
        &self,
        context_id: U256,
    ) -> anyhow::Result<Vec<Address>> {
        let anchor = self
            .protocol_config
            .getKmsContextAnchor(context_id)
            .block(BlockId::finalized())
            .call()
            .await
            .with_context(|| format!("getKmsContextAnchor({context_id})"))?;
        let block: u64 = anchor.emissionBlockNumber.saturating_to();
        let filter = Filter::new()
            .address(*self.protocol_config.address())
            .event_signature(NewKmsContext::SIGNATURE_HASH)
            .topic1(B256::from(context_id))
            .from_block(block)
            .to_block(block);
        let logs = self
            .protocol_config
            .provider()
            .get_logs(&filter)
            .await
            .with_context(|| format!("NewKmsContext({context_id}) logs"))?;
        let [log] = logs.as_slice() else {
            return Err(anyhow!(
                "{} NewKmsContext({context_id}) events at its anchor block {block}",
                logs.len()
            ));
        };
        let event = NewKmsContext::decode_log(&log.inner)
            .with_context(|| format!("decode NewKmsContext({context_id})"))?;
        Ok(event
            .data
            .kmsNodeParams
            .iter()
            .map(|node| node.txSenderAddress)
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        primitives::{Bytes, Log as PrimitiveLog},
        providers::{mock::Asserter, ProviderBuilder},
        rpc::types::Log,
        sol_types::SolValue,
    };
    use fhevm_host_bindings::protocol_config::{
        IProtocolConfig::KmsThresholds,
        ProtocolConfig::{self, KmsNodeParams},
    };

    const CONTRACT: Address = Address::repeat_byte(0xC0);

    fn context(n: u64) -> U256 {
        KMS_CONTEXT_COUNTER_BASE + U256::from(n)
    }

    fn node(tx_sender: Address) -> KmsNodeParams {
        KmsNodeParams {
            txSenderAddress: tx_sender,
            signerAddress: Address::repeat_byte(0x51),
            ..Default::default()
        }
    }

    fn new_context_log(context_id: U256, tx_senders: &[Address]) -> Log {
        let event = NewKmsContext {
            contextId: context_id,
            previousContextId: context_id - U256::from(1),
            kmsNodeParams: tx_senders.iter().copied().map(node).collect(),
            thresholds: KmsThresholds::default(),
            softwareVersion: String::new(),
            pcrValues: vec![],
        };
        Log {
            inner: PrimitiveLog {
                address: CONTRACT,
                data: event.encode_log_data(),
            },
            ..Default::default()
        }
    }

    fn reader(asserter: &Asserter) -> KmsTxSenderReader<impl Provider> {
        let provider =
            ProviderBuilder::new().connect_mocked_client(asserter.clone());
        KmsTxSenderReader::new(ProtocolConfig::new(CONTRACT, provider))
    }

    fn chain_id(asserter: &Asserter) {
        asserter.push_success(&"0x3039");
    }

    fn anchor(asserter: &Asserter, block: u64) {
        asserter.push_success(&Bytes::from(
            (U256::from(block), B256::ZERO).abi_encode_sequence(),
        ));
    }

    fn abi(asserter: &Asserter, value: impl SolValue) {
        asserter.push_success(&Bytes::from(value.abi_encode()));
    }

    /// Context 1 was destroyed; context 2 is Active and context 3 Pending. The second read finds
    /// context 2 destroyed and reuses context 3's cached nodes.
    #[tokio::test]
    async fn reads_every_live_context_and_caches_its_nodes() {
        let (a, b, c) = (
            Address::repeat_byte(0xA1),
            Address::repeat_byte(0xB2),
            Address::repeat_byte(0xC3),
        );
        let asserter = Asserter::new();
        let mut reader = reader(&asserter);

        chain_id(&asserter);
        abi(&asserter, context(3));
        abi(&asserter, false);
        abi(&asserter, true);
        anchor(&asserter, 20);
        asserter.push_success(&vec![new_context_log(context(2), &[a, b])]);
        abi(&asserter, true);
        anchor(&asserter, 30);
        asserter.push_success(&vec![new_context_log(context(3), &[c])]);
        let set = reader.read().await.expect("first read");
        assert_eq!(
            set.registry,
            KeyRegistry {
                chain_id: 12345,
                contract: CONTRACT
            }
        );
        assert_eq!(set.senders, HashSet::from([a, b, c]));

        chain_id(&asserter);
        abi(&asserter, context(3));
        abi(&asserter, false);
        abi(&asserter, false);
        abi(&asserter, true);
        let set = reader.read().await.expect("second read");
        assert_eq!(set.senders, HashSet::from([c]));
        assert!(asserter.read_q().is_empty(), "no further RPC call");
    }

    #[tokio::test]
    async fn refuses_an_anchor_without_its_event() {
        let asserter = Asserter::new();
        let mut reader = reader(&asserter);
        chain_id(&asserter);
        abi(&asserter, context(1));
        abi(&asserter, true);
        anchor(&asserter, 20);
        asserter.push_success(&Vec::<Log>::new());
        assert!(reader.read().await.is_err());
    }
}
