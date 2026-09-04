use tracing::{debug, error, info, warn};

use crate::{
    config::settings::GatewayConfig,
    gateway::arbitrum::bindings::{gateway_chain_event_for_log, gateway_chain_event_signatures},
    gateway::handled_events::{EventKey, HandledEvents, ObservedEvent, RangeObserved},
    logging::ListenerStep,
    orchestrator::{DispatchGate, HealthCheck},
    store::sql::repositories::chain_cursor_repo::ChainCursorRepository,
};
use alloy::{
    network::AnyNetwork,
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    rpc::types::{BlockNumberOrTag, Filter, Log},
};
use async_trait::async_trait;
use std::{str::FromStr, sync::Arc, time::Duration};
use tokio_util::sync::CancellationToken;

/// The last block a single `eth_getLogs` should cover, `max_blocks` counted inclusively.
fn catch_up_to_block(from_block: u64, head: u64, max_blocks: u64) -> u64 {
    head.min(from_block.saturating_add(max_blocks.saturating_sub(1)))
}

/// HTTP polling listener that uses eth_getLogs at configurable intervals
pub struct PollingListener {
    gateway_config: GatewayConfig,
    chain_cursor_repo: Arc<ChainCursorRepository>,
    handled_events: Arc<HandledEvents>,
    /// Position in the whole listener pool, counting polling and subscription listeners alike.
    /// Keys `HandledEvents`' per-listener pending-range queues and labels this listener's logs
    /// and metrics, so two listeners in one pool must never collide on it.
    pool_index: usize,
    /// HTTP URL for this listener
    http_url: String,
    /// Open only while this pod is the confirmed dispatcher. A non-holder
    /// must neither handle gateway responses nor advance the shared chain cursor, so the poll
    /// loop waits here rather than fetching.
    gate: DispatchGate,
    shutdown: CancellationToken,
}

impl PollingListener {
    pub fn new(
        gateway_config: GatewayConfig,
        chain_cursor_repo: Arc<ChainCursorRepository>,
        handled_events: Arc<HandledEvents>,
        pool_index: usize,
        http_url: String,
        gate: DispatchGate,
        shutdown: CancellationToken,
    ) -> anyhow::Result<Self> {
        // Enforce HTTP URL - polling listener requires HTTP, not WebSocket
        if !http_url.starts_with("http://") && !http_url.starts_with("https://") {
            return Err(anyhow::anyhow!(
                "Polling listener {} requires HTTP URL (http:// or https://), got: {}",
                pool_index,
                http_url
            ));
        }

        Ok(Self {
            gateway_config,
            chain_cursor_repo,
            handled_events,
            pool_index,
            http_url,
            gate,
            shutdown,
        })
    }

    async fn interruptible_sleep(&self, dur: Duration) {
        tokio::select! {
            _ = tokio::time::sleep(dur) => {}
            _ = self.shutdown.cancelled() => {}
        }
    }

    /// Wait until this pod is the dispatcher. Returns `true` if shutdown came first.
    async fn wait_for_gate_or_shutdown(&self) -> bool {
        if self.gate.is_open() {
            return false;
        }
        info!(
            instance_id = self.pool_index,
            "Polling listener idle: not the dispatcher"
        );
        tokio::select! {
            _ = self.gate.wait_open() => {
                info!(
                    instance_id = self.pool_index,
                    "Polling listener resuming: this pod is now the dispatcher"
                );
                false
            }
            _ = self.shutdown.cancelled() => true,
        }
    }

    /// The block to resume polling after. Nothing is written here: the first range this
    /// listener completes records the position, so a start that handles nothing leaves the
    /// cursor where it was.
    async fn resolve_starting_block(
        &self,
        provider: &Arc<dyn Provider<AnyNetwork> + Send + Sync>,
    ) -> anyhow::Result<u64> {
        if let Some(from_config) = self.gateway_config.listener_pool.last_block_number {
            info!(
                instance_id = self.pool_index,
                "Starting from config block {} (overriding any recorded cursor)", from_config
            );
            return Ok(from_config);
        }

        if let Some(recorded) = self.chain_cursor_repo.get().await? {
            info!(
                instance_id = self.pool_index,
                "Starting from recorded block {} (resuming)", recorded
            );
            return Ok(recorded);
        }

        let head = provider.get_block_number().await?;
        info!(
            instance_id = self.pool_index,
            "Starting from current chain block {} (first run)", head
        );
        Ok(head)
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let poll_interval_ms = self.gateway_config.listener_pool.poll_interval_ms;
        let max_blocks_per_query = self.gateway_config.listener_pool.max_blocks_per_query;

        info!(
            instance_id = self.pool_index,
            http_url = %self.http_url,
            poll_interval_ms = poll_interval_ms,
            max_blocks_per_query = max_blocks_per_query,
            "Starting polling listener"
        );

        // Parse contract addresses once
        let decryption_address =
            Address::from_str(&self.gateway_config.contracts.decryption_address)
                .map_err(|_| anyhow::anyhow!("Invalid decryption address"))?;
        let input_verification_address =
            Address::from_str(&self.gateway_config.contracts.input_verification_address)
                .map_err(|_| anyhow::anyhow!("Invalid InputVerification address"))?;
        let contract_addresses = vec![decryption_address, input_verification_address];

        let event_signatures = gateway_chain_event_signatures();

        let mut consecutive_failures: u32 = 0;
        let max_attempts = self.gateway_config.listener_pool.polling_max_attempts;
        let retry_interval = self
            .gateway_config
            .listener_pool
            .reconnect_config
            .retry_interval_ms;

        // Wait for the gate *before* resolving the starting block: the resume point comes from
        // the shared `gateway_chain_cursor` row, which whichever pod is the dispatcher keeps
        // advancing. Reading it while another pod holds the lock would pin this listener to a
        // position that is stale by however long this pod stands by - hours, at which point
        // acquiring the lock would replay every block since.
        if self.wait_for_gate_or_shutdown().await {
            info!(
                instance_id = self.pool_index,
                "Polling listener stopping (shutdown before acquiring the dispatch lock)"
            );
            return Ok(());
        }

        // Create provider
        let provider = self.create_provider()?;

        // Resolve starting block
        let mut last_processed_block = match self.resolve_starting_block(&provider).await {
            Ok(block) => block,
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Polling listener {}: Failed to resolve starting block: {}",
                    self.pool_index,
                    e
                ));
            }
        };

        info!(
            instance_id = self.pool_index,
            starting_block = last_processed_block,
            "Polling listener initialized, starting poll loop"
        );

        // Set while a chunk stopped short of the head, so the next one is fetched at once.
        let mut backlog_pending = false;

        loop {
            // Log ERROR continuously when exceeding threshold (fatal state)
            if consecutive_failures >= max_attempts {
                error!(
                    instance_id = self.pool_index,
                    consecutive_failures = consecutive_failures,
                    max_attempts = max_attempts,
                    "Polling listener exceeded max consecutive poll failures, will keep retrying"
                );
            }

            // Re-check the gate every tick, not just at startup: a failed heartbeat closes it
            // (`LockState::Unconfirmed`) without this pod losing the session, and events handled
            // in that window could be a successor's. `last_processed_block` stays put while
            // paused, so resuming replays the range that was skipped.
            if self.wait_for_gate_or_shutdown().await {
                info!(
                    instance_id = self.pool_index,
                    "Polling listener stopping (shutdown while not dispatching)"
                );
                return Ok(());
            }

            if !backlog_pending {
                self.interruptible_sleep(Duration::from_millis(poll_interval_ms))
                    .await;
            }

            if self.shutdown.is_cancelled() {
                info!(
                    instance_id = self.pool_index,
                    "Polling listener stopping (shutdown)"
                );
                return Ok(());
            }

            // Get current block number
            let current_block = match provider.get_block_number().await {
                Ok(block) => block,
                Err(e) => {
                    consecutive_failures += 1;
                    backlog_pending = false;
                    warn!(
                        instance_id = self.pool_index,
                        error = %e,
                        attempt = consecutive_failures,
                        max_attempts = max_attempts,
                        "Polling listener: Failed to get current block number (attempt {}/{}), retrying...",
                        consecutive_failures,
                        max_attempts
                    );
                    self.interruptible_sleep(Duration::from_millis(retry_interval))
                        .await;
                    continue;
                }
            };

            // Calculate block range to query
            let from_block = last_processed_block + 1;

            if from_block > current_block {
                // Already caught up, nothing to poll
                backlog_pending = false;
                continue;
            }

            let to_block = catch_up_to_block(from_block, current_block, max_blocks_per_query);
            backlog_pending = to_block < current_block;

            // Create filter for the block range
            let filter = Filter::new()
                .from_block(BlockNumberOrTag::Number(from_block))
                .to_block(BlockNumberOrTag::Number(to_block))
                .address(contract_addresses.clone())
                .event_signature(event_signatures.clone());

            // Query historical logs
            let logs: Vec<Log> = match provider.get_logs(&filter).await {
                Ok(logs) => logs,
                Err(e) => {
                    consecutive_failures += 1;
                    backlog_pending = false;
                    warn!(
                        instance_id = self.pool_index,
                        from_block = from_block,
                        to_block = to_block,
                        error = %e,
                        attempt = consecutive_failures,
                        max_attempts = max_attempts,
                        "Polling listener: Failed to get logs (attempt {}/{}), retrying...",
                        consecutive_failures,
                        max_attempts
                    );
                    self.interruptible_sleep(Duration::from_millis(retry_interval))
                        .await;
                    continue;
                }
            };

            // Reset failure counter on successful poll
            consecutive_failures = 0;

            let mut events = Vec::new();
            if !logs.is_empty() {
                debug!(
                    instance_id = self.pool_index,
                    from_block = from_block,
                    to_block = to_block,
                    event_count = logs.len(),
                    "Polling listener: Found events"
                );

                events.extend(
                    logs.iter()
                        .filter_map(|log| self.observed_event_for_log(log)),
                );
            }

            last_processed_block = to_block;

            // The query covered the whole range it asked for, so this listener can attest to
            // it: the cursor may pass to_block once every event found in it has been handled.
            self.handled_events
                .record_and_dispatch(events, Some(RangeObserved { to_block }), self.pool_index)
                .await;
        }
    }

    /// Resolve one polled log into the event that gets tracked, or `None` if it is not an
    /// event this relayer routes.
    fn observed_event_for_log(&self, event_log: &Log) -> Option<ObservedEvent> {
        let tx_hash = match event_log.transaction_hash {
            Some(hash) => hash,
            None => {
                warn!(
                    instance_id = self.pool_index,
                    "Polling listener: Event log missing transaction hash, skipping"
                );
                return None;
            }
        };

        // Coordinates identify the event, so a log without them cannot be tracked: two
        // such logs would share a key and the second would count as a duplicate.
        let (Some(block_number), Some(block_hash), Some(log_index)) = (
            event_log.block_number,
            event_log.block_hash,
            event_log.log_index,
        ) else {
            warn!(
                instance_id = self.pool_index,
                "Polling listener: Event log missing block coordinates, skipping"
            );
            return None;
        };

        let topic0 = event_log
            .topics()
            .first()
            .map(|t| format!("{:#x}", t))
            .unwrap_or_else(|| "none".to_string());
        let topic1 = event_log
            .topics()
            .get(1)
            .map(|t| format!("{:#x}", t))
            .unwrap_or_else(|| "none".to_string());

        let gateway_event = match gateway_chain_event_for_log(event_log.clone(), tx_hash) {
            Some(gateway_event) => gateway_event,
            None => {
                warn!(
                    step = %ListenerStep::EventUnroutable,
                    instance_id = self.pool_index,
                    block_number = block_number,
                    log_index = log_index,
                    topic0 = %topic0,
                    "Unroutable gateway event"
                );
                return None;
            }
        };

        debug!(
            step = %ListenerStep::EventReceived,
            instance_id = self.pool_index,
            "Polling listener: Processing event: block={}, block_hash={:#x}, log_index={}, topic0={}, topic1={}, tx_hash={:#x}",
            block_number, block_hash, log_index, topic0, topic1, tx_hash
        );

        Some(ObservedEvent {
            key: EventKey {
                block_number,
                block_hash,
                log_index,
            },
            event: gateway_event,
        })
    }

    fn create_provider(&self) -> anyhow::Result<Arc<dyn Provider<AnyNetwork> + Send + Sync>> {
        let provider = ProviderBuilder::new().network::<AnyNetwork>().connect_http(
            self.http_url
                .parse()
                .map_err(|e| anyhow::anyhow!("Invalid HTTP URL {}: {}", self.http_url, e))?,
        );

        Ok(Arc::new(provider))
    }
}

#[async_trait]
impl HealthCheck for PollingListener {
    async fn check(&self) -> anyhow::Result<()> {
        let provider = self.create_provider()?;
        let health_timeout = Duration::from_secs(
            self.gateway_config
                .blockchain_rpc
                .http_health_check_timeout_secs,
        );

        match tokio::time::timeout(health_timeout, provider.get_block_number()).await {
            Err(_) => Err(anyhow::anyhow!(
                "Polling listener {}: HTTP health check timed out after {:?}",
                self.pool_index,
                health_timeout
            )),
            Ok(Err(e)) => Err(anyhow::anyhow!(
                "Polling listener {}: HTTP health check failed: {}",
                self.pool_index,
                e
            )),
            Ok(Ok(_)) => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_chunk_never_reaches_past_the_head() {
        assert_eq!(catch_up_to_block(101, 105, 1_000), 105);
    }

    #[test]
    fn the_chunk_span_counts_blocks_inclusively() {
        assert_eq!(catch_up_to_block(100, 1_000_000, 10), 109);
    }

    #[test]
    fn a_large_gap_is_capped_to_one_chunk() {
        assert_eq!(catch_up_to_block(1, 1_000_000, 1_000), 1_000);
    }

    #[test]
    fn a_chunk_past_the_end_of_the_range_saturates() {
        assert_eq!(
            catch_up_to_block(u64::MAX - 1, u64::MAX, u64::MAX),
            u64::MAX
        );
    }
}
