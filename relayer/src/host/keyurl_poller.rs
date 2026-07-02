//! Host-chain poller that keeps the `/v2/keyurl` response in sync with on-chain ProtocolConfig and KMSGeneration contracts' states.
//!
//! Polls the active key/CRS/context on an interval rather than subscribing to events: the
//! activations are rare and unreliable to subscribe to, whereas periodic comparison is
//! idempotent and self-healing. State lives in memory only (repopulated from chain on
//! restart); on each change the new value is pushed into the [`tokio::sync::watch`] channel
//! that backs the endpoint and the rotation is logged at INFO.

use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use alloy::eips::BlockId;
use alloy::primitives::{Address, U256};
use fhevm_host_bindings::i_protocol_config::IProtocolConfig;
use fhevm_host_bindings::i_protocol_config::IProtocolConfig::IProtocolConfigInstance;
use fhevm_host_bindings::kms_generation::KMSGeneration;
use fhevm_host_bindings::kms_generation::KMSGeneration::KMSGenerationInstance;
use tokio::sync::watch;
use tracing::{error, info, warn};

use crate::config::settings::ProtocolConfigSettings;
use crate::host::error_redact::redact_alloy_error;
use crate::host::provider::{build_host_provider, Provider};
use crate::http::endpoints::v2::types::keyurl::{KeyData, KeyUrlResponseJson};

type HostKmsGeneration = KMSGenerationInstance<Arc<Provider>, alloy::network::AnyNetwork>;
type HostProtocolConfig = IProtocolConfigInstance<Arc<Provider>, alloy::network::AnyNetwork>;

/// Read on-chain state at the finalized block tag to avoid serving a reorged-away activation.
fn finalized() -> BlockId {
    BlockId::finalized()
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
    /// Build the poller from the protocol-config settings.
    pub fn new(settings: &ProtocolConfigSettings) -> anyhow::Result<Self> {
        let provider = build_host_provider(&settings.ethereum_http_rpc_url)?;

        let kms_generation_address = Address::from_str(&settings.kms_generation_address)
            .map_err(|e| anyhow::anyhow!("Invalid kms_generation_address: {e}"))?;
        let protocol_config_address = Address::from_str(&settings.address)
            .map_err(|e| anyhow::anyhow!("Invalid protocol_config address: {e}"))?;

        Ok(Self {
            kms_generation: KMSGeneration::new(kms_generation_address, provider.clone()),
            protocol_config: IProtocolConfig::new(protocol_config_address, provider),
            poll_interval: Duration::from_millis(settings.keyurl_poll_interval_ms),
            retry: settings.retry.clone(),
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

    /// Fetch the key/CRS materials for the given ids and build the served response.
    async fn build_response(&self, ids: &ActiveIds) -> anyhow::Result<KeyUrlResponseJson> {
        let key_materials = retry_view!(
            self.retry,
            "getKeyMaterials",
            self.kms_generation
                .getKeyMaterials(ids.key_id)
                .block(finalized())
                .call()
        )?;
        let crs_materials = retry_view!(
            self.retry,
            "getCrsMaterials",
            self.kms_generation
                .getCrsMaterials(ids.crs_id)
                .block(finalized())
                .call()
        )?;

        Ok(KeyUrlResponseJson::new(
            KeyData {
                data_id: ids.key_id.to_string(),
                urls: key_materials._0,
            },
            KeyData {
                data_id: ids.crs_id.to_string(),
                urls: crs_materials._0,
            },
            ids.context_id.to_string(),
            ids.epoch_id.to_string(),
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
