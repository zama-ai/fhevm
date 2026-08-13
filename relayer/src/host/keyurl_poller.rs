//! Host-chain poller that keeps the `/v2/keyurl` response in sync with on-chain ProtocolConfig and KMSGeneration contracts' states.
//!
//! Polls the active key/CRS/KMS context on an interval rather than subscribing to events: the
//! activations are rare and unreliable to subscribe to, whereas periodic comparison is
//! idempotent and self-healing. State lives in memory only (repopulated from chain on
//! restart); on each change the new value is pushed into the [`tokio::sync::watch`] channel
//! that backs the endpoint and the rotation is logged at INFO.

use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use alloy::eips::BlockId;
use alloy::primitives::{hex, Address, B256, U256};
use alloy::providers::Provider as _;
use alloy::rpc::types::{BlockNumberOrTag, Filter, Log};
use alloy::sol_types::SolEvent;
use fhevm_host_bindings::i_protocol_config::IProtocolConfig;
use fhevm_host_bindings::i_protocol_config::IProtocolConfig::IProtocolConfigInstance;
use fhevm_host_bindings::ikms_generation::IKMSGeneration;
use fhevm_host_bindings::ikms_generation::IKMSGeneration::IKMSGenerationInstance;
use tokio::sync::watch;
use tracing::{error, info, warn};

use crate::config::settings::{KeyUrlConfig, ProtocolConfigSettings};
use crate::host::error_redact::{redact_alloy_error, redact_error};
use crate::host::provider::{build_host_provider, Provider};
use crate::http::endpoints::v2::types::keyurl::{KeyData, KeyUrlResponseJson};

type HostKmsGeneration = IKMSGenerationInstance<Arc<Provider>, alloy::network::AnyNetwork>;
type HostProtocolConfig = IProtocolConfigInstance<Arc<Provider>, alloy::network::AnyNetwork>;

/// Read on-chain state at the finalized block tag to avoid serving a reorged-away activation.
fn finalized() -> BlockId {
    BlockId::finalized()
}

/// A KMS node's public-storage location. `storage_prefix` (e.g. `PUB` / `PUB-p1`) isn't exposed
/// by any getter, so it's recovered from the `NewKmsContext` event.
struct KmsStorageNode {
    storage_url: String,
    storage_prefix: String,
}

/// The canonical 32-byte big-endian hex encoding of an on-chain key/CRS id, lowercase, no `0x`
/// prefix. Used both as the URL path segment and, `0x`-prefixed, as the served `dataId`.
fn id_hex(id: U256) -> String {
    hex::encode(id.to_be_bytes::<32>())
}

/// Build the full object URL the KMS Core writes to:
/// `{storage_url}/{storage_prefix}/{segment}/{id_hex}` (`segment` = `PublicKey`|`CRS`, `id_hex` =
/// 32-byte big-endian id, lowercase, no `0x`). The getters return only `storage_url`.
///
/// The key/CRS is a global artifact stored under each node's prefix, so any node's copy is
/// equivalent; the served response carries a single URL (the SDK requires exactly one), built
/// from the first context node.
fn build_object_url(node: &KmsStorageNode, segment: &str, id: U256) -> String {
    format!(
        "{}/{}/{}/{}",
        node.storage_url.trim_end_matches('/'),
        node.storage_prefix,
        segment,
        id_hex(id)
    )
}

/// Run a host-chain view call with bounded retries, redacting RPC URLs from any error.
///
/// This is a macro rather than a function because alloy's `CallBuilder::call(&self)` borrows
/// the builder: the call expression and its `.await` must live in one scope so the temporary
/// builder outlives the returned future. `$call` is re-evaluated on each attempt (view getters
/// are idempotent).
macro_rules! retry_view {
    ($retry:expr, $name:expr, $call:expr) => {{
        let retry = &$retry;
        let interval = Duration::from_millis(retry.retry_interval_ms);
        let max_attempts = retry.max_attempts;
        let mut last_error = String::new();
        let mut result = None;
        for attempt in 0..max_attempts {
            match $call.await {
                Ok(value) => {
                    result = Some(value);
                    break;
                }
                Err(e) => {
                    last_error = redact_alloy_error(&e);
                    if attempt + 1 < max_attempts {
                        warn!(
                            op = $name,
                            attempt = attempt + 1,
                            max_attempts,
                            error = %last_error,
                            "host-chain call failed, retrying"
                        );
                        tokio::time::sleep(interval).await;
                    }
                }
            }
        }
        match result {
            Some(value) => Ok(value),
            None => Err(anyhow::anyhow!(
                "host-chain call '{}' failed after {} attempts: {}",
                $name,
                max_attempts,
                last_error
            )),
        }
    }};
}

/// The set of on-chain ids that determine the served `/v2/keyurl` value. A change in any of
/// these triggers a refetch of the materials and a push to the watch channel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ActiveIds {
    key_id: U256,
    crs_id: U256,
    context_id: U256,
    epoch_id: U256,
}

/// Polls the Ethereum host chain for the active key/CRS/context and drives the `/v2/keyurl`
/// watch channel. Exactly one instance runs per relayer.
pub struct KeyUrlPoller {
    kms_generation: HostKmsGeneration,
    protocol_config: HostProtocolConfig,
    poll_interval: Duration,
    retry: crate::config::settings::RetrySettings,
    /// Last ids pushed to the channel; `None` until the first successful poll.
    last_seen: Option<ActiveIds>,
}

impl KeyUrlPoller {
    /// Build the poller. Reads the KMSGeneration contract addressed by `keyurl` and the
    /// ProtocolConfig contract from `protocol_config`, both over the same host-chain provider.
    pub fn new(
        protocol_config: &ProtocolConfigSettings,
        keyurl: &KeyUrlConfig,
    ) -> anyhow::Result<Self> {
        let provider = build_host_provider(&protocol_config.ethereum_http_rpc_url)?;

        let kms_generation_address = Address::from_str(&keyurl.kms_generation_address)
            .map_err(|e| anyhow::anyhow!("Invalid kms_generation_address: {e}"))?;
        let protocol_config_address = Address::from_str(&protocol_config.address)
            .map_err(|e| anyhow::anyhow!("Invalid protocol_config address: {e}"))?;

        Ok(Self {
            kms_generation: IKMSGeneration::new(kms_generation_address, provider.clone()),
            protocol_config: IProtocolConfig::new(protocol_config_address, provider),
            poll_interval: Duration::from_millis(keyurl.poll_interval_ms),
            retry: protocol_config.retry.clone(),
            last_seen: None,
        })
    }

    /// Read the active key/CRS ids and the current context/epoch at the finalized block.
    async fn read_active_ids(&self) -> anyhow::Result<ActiveIds> {
        let key_id: U256 = retry_view!(
            self.retry,
            "getActiveKeyId",
            self.kms_generation
                .getActiveKeyId()
                .block(finalized())
                .call()
        )?;
        let crs_id: U256 = retry_view!(
            self.retry,
            "getActiveCrsId",
            self.kms_generation
                .getActiveCrsId()
                .block(finalized())
                .call()
        )?;
        let context_and_epoch = retry_view!(
            self.retry,
            "getCurrentKmsContextAndEpoch",
            self.protocol_config
                .getCurrentKmsContextAndEpoch()
                .block(finalized())
                .call()
        )?;

        Ok(ActiveIds {
            key_id,
            crs_id,
            context_id: context_and_epoch.contextId,
            epoch_id: context_and_epoch.epochId,
        })
    }

    /// Build the served response for the given ids. `getKeyMaterials` / `getCrsMaterials` are
    /// called only to assert the material exists (they revert otherwise); their URLs are the
    /// bucket base only, so the full object URLs are rebuilt from the KMS context nodes.
    async fn build_response(&self, ids: &ActiveIds) -> anyhow::Result<KeyUrlResponseJson> {
        retry_view!(
            self.retry,
            "getKeyMaterials",
            self.kms_generation
                .getKeyMaterials(ids.key_id)
                .block(finalized())
                .call()
        )?;
        retry_view!(
            self.retry,
            "getCrsMaterials",
            self.kms_generation
                .getCrsMaterials(ids.crs_id)
                .block(finalized())
                .call()
        )?;

        // `fetch_context_nodes` guarantees at least one node; serve the first node's copy.
        let node = &self.fetch_context_nodes(ids.context_id).await?[0];

        Ok(KeyUrlResponseJson::new(
            KeyData {
                data_id: format!("0x{}", id_hex(ids.key_id)),
                urls: vec![build_object_url(node, "PublicKey", ids.key_id)],
            },
            KeyData {
                data_id: format!("0x{}", id_hex(ids.crs_id)),
                urls: vec![build_object_url(node, "CRS", ids.crs_id)],
            },
        ))
    }

    /// Read the KMS nodes for `context_id` from its `NewKmsContext` event. The contract anchors
    /// the exact emission block (`getKmsContextAnchor`), so we fetch that single event by the
    /// `contextId` topic and decode `KmsNodeParams` — no block-range scan.
    async fn fetch_context_nodes(&self, context_id: U256) -> anyhow::Result<Vec<KmsStorageNode>> {
        let anchor = retry_view!(
            self.retry,
            "getKmsContextAnchor",
            self.protocol_config
                .getKmsContextAnchor(context_id)
                .block(finalized())
                .call()
        )?;
        let block_number: u64 = anchor.emissionBlockNumber.try_into().map_err(|e| {
            anyhow::anyhow!("KMS context {context_id} anchor block number is out of range: {e}")
        })?;
        if block_number == 0 {
            anyhow::bail!("no KMS context anchor recorded for context {context_id}");
        }

        let filter = Filter::new()
            .address(*self.protocol_config.address())
            .event_signature(IProtocolConfig::NewKmsContext::SIGNATURE_HASH)
            .topic1(B256::from(context_id.to_be_bytes::<32>()))
            .from_block(BlockNumberOrTag::Number(block_number))
            .to_block(BlockNumberOrTag::Number(block_number));

        let logs = self.get_logs_with_retry(&filter).await?;
        let log = logs.last().ok_or_else(|| {
            anyhow::anyhow!(
                "NewKmsContext event for context {context_id} not found at block {block_number}"
            )
        })?;

        let event = IProtocolConfig::NewKmsContext::decode_log_data(log.data())
            .map_err(|e| anyhow::anyhow!("failed to decode NewKmsContext event: {e}"))?;

        let nodes: Vec<KmsStorageNode> = event
            .kmsNodeParams
            .into_iter()
            .map(|node| KmsStorageNode {
                storage_url: node.storageUrl,
                storage_prefix: node.storagePrefix,
            })
            .collect();
        if nodes.is_empty() {
            anyhow::bail!("NewKmsContext for context {context_id} contains no KMS nodes");
        }
        Ok(nodes)
    }

    /// `eth_getLogs` with the same bounded retry as the view calls; a separate helper because
    /// `get_logs` returns a transport `RpcError`, not the `ContractError` `retry_view!` expects.
    async fn get_logs_with_retry(&self, filter: &Filter) -> anyhow::Result<Vec<Log>> {
        let interval = Duration::from_millis(self.retry.retry_interval_ms);
        let max_attempts = self.retry.max_attempts;
        let mut last_error = String::new();
        for attempt in 0..max_attempts {
            match self.protocol_config.provider().get_logs(filter).await {
                Ok(logs) => return Ok(logs),
                Err(e) => {
                    last_error = redact_error(&e);
                    if attempt + 1 < max_attempts {
                        warn!(
                            op = "get_logs",
                            attempt = attempt + 1,
                            max_attempts,
                            error = %last_error,
                            "host-chain call failed, retrying"
                        );
                        tokio::time::sleep(interval).await;
                    }
                }
            }
        }
        Err(anyhow::anyhow!(
            "host-chain call 'get_logs' failed after {} attempts: {}",
            max_attempts,
            last_error
        ))
    }

    /// Blocking initial fetch used to seed the watch channel at startup. The relayer gates
    /// startup on this succeeding, so the served value is always present (never a placeholder).
    pub async fn initialize(&mut self) -> anyhow::Result<KeyUrlResponseJson> {
        let ids = self.read_active_ids().await?;
        let response = self.build_response(&ids).await?;
        self.last_seen = Some(ids);
        info!(
            key_id = %ids.key_id,
            crs_id = %ids.crs_id,
            context_id = %ids.context_id,
            epoch_id = %ids.epoch_id,
            "Initialized /v2/keyurl from host chain"
        );
        Ok(response)
    }

    /// Long-running loop: poll the active ids on the configured interval and, on any change,
    /// refetch the materials and push the new value into `tx`. RPC failures are logged and
    /// retried on the next tick (the last served value is kept). Stops when the orchestrator
    /// aborts the task on shutdown.
    pub async fn run(mut self, tx: watch::Sender<KeyUrlResponseJson>) {
        let mut ticker = tokio::time::interval(self.poll_interval);
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        loop {
            ticker.tick().await;
            if let Err(e) = self.poll_once(&tx).await {
                error!(
                    error = %e,
                    "/v2/keyurl poll failed; keeping last served value, will retry next tick"
                );
            }
        }
    }

    /// One poll cycle: read ids, and if they changed, refetch materials and publish.
    async fn poll_once(&mut self, tx: &watch::Sender<KeyUrlResponseJson>) -> anyhow::Result<()> {
        let ids = self.read_active_ids().await?;
        if self.last_seen == Some(ids) {
            return Ok(());
        }

        let response = self.build_response(&ids).await?;
        let previous = self.last_seen.replace(ids);

        match previous {
            Some(prev) => info!(
                old_key_id = %prev.key_id, new_key_id = %ids.key_id,
                old_crs_id = %prev.crs_id, new_crs_id = %ids.crs_id,
                old_context_id = %prev.context_id, new_context_id = %ids.context_id,
                old_epoch_id = %prev.epoch_id, new_epoch_id = %ids.epoch_id,
                "/v2/keyurl rotation: serving new key/CRS/context/epoch"
            ),
            None => info!(
                key_id = %ids.key_id,
                crs_id = %ids.crs_id,
                context_id = %ids.context_id,
                epoch_id = %ids.epoch_id,
                "/v2/keyurl populated from host chain"
            ),
        }

        // The receiver is held by the HTTP handler for the process lifetime; a send error
        // means it was dropped (shutdown), which is benign here.
        if tx.send(response).is_err() {
            error!("Failed to update /v2/keyurl: watch receiver dropped");
        }
        Ok(())
    }
}
