use tracing::{debug, error, info, warn};

use crate::{
    config::settings::GatewayConfig,
    gateway::arbitrum::bindings::{gateway_chain_event_for_log, gateway_chain_event_signatures},
    gateway::handled_events::{EventKey, HandledEvents, ObservedEvent},
    logging::ListenerStep,
    orchestrator::HealthCheck,
};
use alloy::{
    network::AnyNetwork,
    primitives::Address,
    providers::{Provider, ProviderBuilder, WsConnect},
    pubsub::{Subscription, SubscriptionStream},
    rpc::types::{Filter, Log},
    transports::ws::WebSocketConfig,
};
use async_trait::async_trait;
use futures::StreamExt;
use std::{str::FromStr, sync::Arc, time::Duration};

/// Reason why process_events() returned
enum RecycleReason {
    /// WebSocket stream ended unexpectedly
    StreamEnded,
    /// Planned connection recycle timer triggered
    RecycleTimer,
}

/// Where one subscription listener sits in the recycle stagger, so a pool of them drops and
/// re-establishes its connections at spread-out times rather than all at once.
///
/// Both values count subscription listeners only, and neither identifies a listener - that is
/// [`ArbitrumListener::pool_index`], counted over the whole pool. A pool of one polling and one
/// subscription listener has two distinct pool indices and a single stagger `index` of 0.
pub struct WsRecycleStagger {
    /// Position among the subscription listeners.
    pub index: usize,
    /// Number of subscription listeners in the pool.
    pub total: usize,
}

impl WsRecycleStagger {
    /// Seconds added to the base recycle interval, spreading the subscription listeners evenly
    /// across one interval. Zero for a pool with no subscription listeners to spread.
    fn offset_secs(&self, base_interval_secs: u64) -> u64 {
        if self.total == 0 {
            return 0;
        }
        (base_interval_secs / self.total as u64) * self.index as u64
    }
}

pub struct ArbitrumListener {
    gateway_config: GatewayConfig,
    handled_events: Arc<HandledEvents>,
    /// Position in the whole listener pool, counting polling and subscription listeners alike.
    /// The identity every other listener kind shares: it keys `HandledEvents`' per-listener
    /// pending-range queues and labels this listener's logs and metrics, so two listeners in one
    /// pool must never collide on it.
    pool_index: usize,
    /// Instance-specific WebSocket URL
    ws_url: String,
    /// Recycle-stagger position, deliberately not an identity - see [`WsRecycleStagger`].
    ws_recycle_stagger: WsRecycleStagger,
}

impl ArbitrumListener {
    pub async fn new(
        gateway_config: GatewayConfig,
        handled_events: Arc<HandledEvents>,
        pool_index: usize,
        ws_url: String,
        ws_recycle_stagger: WsRecycleStagger,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            gateway_config,
            handled_events,
            pool_index,
            ws_url,
            ws_recycle_stagger,
        })
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        // Parse contract addresses once
        let decryption_address =
            Address::from_str(&self.gateway_config.contracts.decryption_address)
                .map_err(|_| anyhow::anyhow!("Invalid decryption address"))?;
        let input_verification_address =
            Address::from_str(&self.gateway_config.contracts.input_verification_address)
                .map_err(|_| anyhow::anyhow!("Invalid InputVerification address"))?;
        let contract_addresses = vec![decryption_address, input_verification_address];

        let mut last_processed_block: Option<u64> = None;
        let mut consecutive_failures: u32 = 0;
        let max_attempts = self
            .gateway_config
            .listener_pool
            .reconnect_config
            .max_attempts;
        let retry_interval = self
            .gateway_config
            .listener_pool
            .reconnect_config
            .retry_interval_ms;

        info!(
            step = %ListenerStep::ListenerStarted,
            instance_id = self.pool_index,
            "Listener started"
        );

        loop {
            // Log ERROR continuously when exceeding threshold (fatal state)
            if consecutive_failures >= max_attempts {
                error!(
                    instance_id = self.pool_index,
                    consecutive_failures = consecutive_failures,
                    max_attempts = max_attempts,
                    "WebSocket listener exceeded max consecutive connection failures, will keep retrying"
                );
            }

            // Create provider (retry on failure)
            let provider = match self.create_provider().await {
                Ok(p) => {
                    info!(
                        step = %ListenerStep::ProviderConnected,
                        instance_id = self.pool_index,
                        "Provider connected"
                    );
                    p
                }
                Err(e) => {
                    consecutive_failures += 1;
                    warn!(
                        step = %ListenerStep::ProviderRetrying,
                        instance_id = self.pool_index,
                        error = %e,
                        attempt = consecutive_failures,
                        max_attempts = max_attempts,
                        "Failed to create provider"
                    );
                    tokio::time::sleep(Duration::from_millis(retry_interval)).await;
                    continue;
                }
            };

            // Create subscription (retry on failure)
            let sub = match self
                .create_subscription(&provider, &contract_addresses)
                .await
            {
                Ok(s) => s,
                Err(e) => {
                    consecutive_failures += 1;
                    warn!(
                        step = %ListenerStep::ProviderRetrying,
                        instance_id = self.pool_index,
                        error = %e,
                        attempt = consecutive_failures,
                        max_attempts = max_attempts,
                        "Failed to subscribe"
                    );
                    tokio::time::sleep(Duration::from_millis(retry_interval)).await;
                    continue;
                }
            };

            // Reset failure counter on successful connection
            consecutive_failures = 0;
            info!(
                step = %ListenerStep::SubscriptionActive,
                instance_id = self.pool_index,
                last_block = ?last_processed_block,
                "Subscription active, listening for events"
            );
            let mut subscription = sub.into_stream();

            // Process events (returns when stream ends or recycle timer triggers)
            let reason = self
                .process_events(&mut subscription, &mut last_processed_block)
                .await;

            match reason {
                RecycleReason::StreamEnded => {
                    // Unexpected stream end - increment failures and wait before retry
                    consecutive_failures += 1;
                    warn!(
                        step = %ListenerStep::SubscriptionDropped,
                        instance_id = self.pool_index,
                        last_block = ?last_processed_block,
                        attempt = consecutive_failures,
                        max_attempts = max_attempts,
                        "WebSocket connection dropped"
                    );
                    tokio::time::sleep(Duration::from_millis(retry_interval)).await;
                }
                RecycleReason::RecycleTimer => {
                    // Planned recycle - reconnect immediately without delay
                    info!(
                        instance_id = self.pool_index,
                        last_block = ?last_processed_block,
                        "Recycling WebSocket connection as scheduled"
                    );
                }
            }
        }
    }

    /// Process events from subscription stream.
    /// Returns `RecycleReason` indicating why processing stopped.
    /// Updates last_block with the last successfully processed block number.
    async fn process_events(
        &self,
        subscription: &mut SubscriptionStream<Log>,
        last_block: &mut Option<u64>,
    ) -> RecycleReason {
        let base_interval_secs = self.gateway_config.listener_pool.recycle_interval_mins * 60;
        let stagger_secs = self.ws_recycle_stagger.offset_secs(base_interval_secs);
        let recycle_duration = Duration::from_secs(base_interval_secs + stagger_secs);

        info!(
            instance_id = self.pool_index,
            recycle_interval_mins = self.gateway_config.listener_pool.recycle_interval_mins,
            stagger_secs = stagger_secs,
            total_recycle_secs = recycle_duration.as_secs(),
            "WebSocket recycle timer configured"
        );

        let recycle_timer = tokio::time::sleep(recycle_duration);
        tokio::pin!(recycle_timer);

        loop {
            tokio::select! {
                event = subscription.next() => {
                    match event {
                        Some(event_log) => {
                            let tx_hash = event_log
                                .transaction_hash
                                .expect("Event log must have transaction hash");

                            // Coordinates identify the event, so a log without them cannot
                            // be tracked: two such logs would share a key and the second
                            // would count as a duplicate.
                            let (Some(block_number), Some(block_hash), Some(log_index)) = (
                                event_log.block_number,
                                event_log.block_hash,
                                event_log.log_index,
                            ) else {
                                warn!(
                                    instance_id = self.pool_index,
                                    "Event log missing block coordinates, skipping"
                                );
                                continue;
                            };

                            // Extract topics for logging
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

                            debug!(
                                step = %ListenerStep::EventReceived,
                                instance_id = self.pool_index,
                                block_number = block_number,
                                log_index = log_index,
                                tx_hash = %format!("{:#x}", tx_hash),
                                topic0 = %topic0,
                                topic1 = %topic1,
                                "Event received"
                            );

                            let events = match gateway_chain_event_for_log(event_log.clone(), tx_hash) {
                                Some(gateway_event) => vec![ObservedEvent {
                                    key: EventKey {
                                        block_number,
                                        block_hash,
                                        log_index,
                                    },
                                    event: gateway_event,
                                }],
                                None => {
                                    warn!(
                                        step = %ListenerStep::EventUnroutable,
                                        instance_id = self.pool_index,
                                        block_number = block_number,
                                        log_index = log_index,
                                        topic0 = %topic0,
                                        "Unroutable gateway event"
                                    );
                                    Vec::new()
                                }
                            };

                            // Tracked in memory only, for the reconnection logs: a
                            // subscription cannot attest to a range, so it carries no
                            // cursor and passes no marker.
                            *last_block = Some(block_number);

                            if !events.is_empty() {
                                self.handled_events
                                    .record_and_dispatch(events, None, self.pool_index)
                                    .await;
                            }
                        }
                        None => {
                            // Stream ended - return to allow reconnection
                            return RecycleReason::StreamEnded;
                        }
                    }
                }
                _ = &mut recycle_timer => {
                    info!(
                        instance_id = self.pool_index,
                        "WebSocket connection recycle timer triggered, reconnecting"
                    );
                    return RecycleReason::RecycleTimer;
                }
            }
        }
    }

    /// Creates a log subscription for the given provider.
    ///
    /// The filter carries no `fromBlock`: `eth_subscribe` has no such parameter, and alloy
    /// shares this `Filter` type with `eth_getLogs`, so setting one would compile and do
    /// nothing.
    async fn create_subscription(
        &self,
        provider: &Arc<dyn Provider<AnyNetwork> + Send + Sync>,
        contract_addresses: &[Address],
    ) -> anyhow::Result<Subscription<Log>> {
        let filter = Filter::new()
            .address(contract_addresses.to_vec())
            .event_signature(gateway_chain_event_signatures());

        provider
            .subscribe_logs(&filter)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create log subscription: {}", e))
    }

    async fn create_provider(&self) -> anyhow::Result<Arc<dyn Provider<AnyNetwork> + Send + Sync>> {
        // Create WebSocket provider with preserved settings
        // 256MB instead of 64MB max for websocket size (copro bug with payload over 64MB)
        let ws_config = WebSocketConfig::default().max_message_size(Some(256 * 1024 * 1024));
        // Disable implicit reconnect - we handle reconnection at application level
        // Use instance-specific WebSocket URL
        let ws = WsConnect::new(&self.ws_url)
            .with_config(ws_config)
            .with_max_retries(0);

        let provider = ProviderBuilder::new()
            .network::<AnyNetwork>()
            .connect_ws(ws)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create WebSocket provider: {}", e))?;

        Ok(Arc::new(provider))
    }
}

#[async_trait]
impl HealthCheck for ArbitrumListener {
    async fn check(&self) -> anyhow::Result<()> {
        let provider = self.create_provider().await?;
        let health_timeout = Duration::from_secs(
            self.gateway_config
                .blockchain_rpc
                .ws_health_check_timeout_secs,
        );

        match tokio::time::timeout(health_timeout, provider.get_block_number()).await {
            Err(_) => Err(anyhow::anyhow!(
                "Gateway WebSocket health check timed out after {:?}",
                health_timeout
            )),
            Ok(Err(e)) => Err(anyhow::anyhow!(
                "Gateway WebSocket health check failed: {}",
                e
            )),
            Ok(Ok(_)) => Ok(()),
        }
    }
}
