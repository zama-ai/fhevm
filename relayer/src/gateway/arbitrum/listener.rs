use tracing::{debug, error, info, warn};

use crate::{
    config::settings::GatewayConfig,
    core::event::{ApiCategory, ApiVersion, GatewayChainEventData, RelayerEvent, RelayerEventData},
    core::job_id::INTERNAL_EVENT_JOB_ID,
    gateway::arbitrum::event_deduplicator::{EventDeduplicator, EventKey},
    logging::ListenerStep,
    orchestrator::{HealthCheck, Orchestrator},
    store::sql::repositories::block_number_repo::BlockNumberRepository,
};
use alloy::{
    network::AnyNetwork,
    primitives::Address,
    providers::{Provider, ProviderBuilder, WsConnect},
    pubsub::{Subscription, SubscriptionStream},
    rpc::types::{BlockNumberOrTag, Filter, Log},
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

pub struct ArbitrumListener {
    gateway_config: GatewayConfig,
    orchestrator: Arc<Orchestrator>,
    block_number_repo: Arc<BlockNumberRepository>,
    deduplicator: Arc<EventDeduplicator>,
    instance_id: usize,
    /// Instance-specific WebSocket URL
    ws_url: String,
    /// Total number of listener instances (for staggered recycle timing)
    num_listeners: usize,
}

impl ArbitrumListener {
    pub async fn new(
        gateway_config: GatewayConfig,
        orchestrator: Arc<Orchestrator>,
        block_number_repo: Arc<BlockNumberRepository>,
        deduplicator: Arc<EventDeduplicator>,
        instance_id: usize,
        ws_url: String,
        num_listeners: usize,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            gateway_config,
            orchestrator,
            block_number_repo,
            deduplicator,
            instance_id,
            ws_url,
            num_listeners,
        })
    }

    async fn fetch_block_hash_from_rpc(
        &self,
        block_number_for_hash_lookup: u64,
        provider: &Arc<dyn Provider<AnyNetwork> + Send + Sync>,
    ) -> anyhow::Result<String> {
        match provider
            .get_block_by_number(BlockNumberOrTag::Number(block_number_for_hash_lookup))
            .await
        {
            Ok(Some(block)) => {
                let block_hash_from_rpc = block.header.hash;
                Ok(format!("{:#x}", block_hash_from_rpc))
            }
            Ok(None) => Err(anyhow::anyhow!(
                "Block {} not found - invalid config block number",
                block_number_for_hash_lookup
            )),
            Err(e) => Err(anyhow::anyhow!(
                "Failed to fetch block {}: {}",
                block_number_for_hash_lookup,
                e
            )),
        }
    }

    async fn resolve_starting_block(
        &self,
        provider: &Arc<dyn Provider<AnyNetwork> + Send + Sync>,
    ) -> anyhow::Result<u64> {
        let block_info_from_db = self
            .block_number_repo
            .get_last_block_info(self.instance_id)
            .await?;

        let block_number = match (
            self.gateway_config.listener_pool.last_block_number,
            block_info_from_db,
        ) {
            // Config takes precedence
            (Some(block_number_from_cfg), Some(block_info_from_db)) => {
                if block_info_from_db.block_number != block_number_from_cfg {
                    info!(
                        "Starting from config block {} (overriding database block {})",
                        block_number_from_cfg, block_info_from_db.block_number
                    );
                    let block_hash_from_rpc = self
                        .fetch_block_hash_from_rpc(block_number_from_cfg, provider)
                        .await?;
                    self.block_number_repo
                        .update_block_info(
                            block_number_from_cfg,
                            block_hash_from_rpc,
                            self.instance_id,
                        )
                        .await?;
                } else {
                    info!(
                        "Starting from config block {} (matches database)",
                        block_number_from_cfg
                    );
                }
                block_number_from_cfg
            }

            // Config with no DB record
            (Some(block_number_from_cfg), None) => {
                info!(
                    "Starting from config block {} (initializing database)",
                    block_number_from_cfg
                );
                let block_hash_from_rpc = self
                    .fetch_block_hash_from_rpc(block_number_from_cfg, provider)
                    .await?;
                self.block_number_repo
                    .insert_initial_block_info(
                        block_number_from_cfg,
                        block_hash_from_rpc,
                        self.instance_id,
                    )
                    .await?;
                block_number_from_cfg
            }

            // No config, use existing DB
            (None, Some(block_info_from_db)) => {
                info!(
                    "Starting from database block {} (resuming)",
                    block_info_from_db.block_number
                );
                block_info_from_db.block_number
            }

            // Fresh start: no config, no DB
            (None, None) => {
                let current_block_from_rpc = provider.get_block_number().await?;
                info!(
                    "Starting from current chain block {} (first run)",
                    current_block_from_rpc
                );
                let block_hash_from_rpc = self
                    .fetch_block_hash_from_rpc(current_block_from_rpc, provider)
                    .await?;
                self.block_number_repo
                    .insert_initial_block_info(
                        current_block_from_rpc,
                        block_hash_from_rpc,
                        self.instance_id,
                    )
                    .await?;
                current_block_from_rpc
            }
        };

        Ok(block_number)
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
            instance_id = self.instance_id,
            "Listener started"
        );

        loop {
            // Log ERROR continuously when exceeding threshold (fatal state)
            if consecutive_failures >= max_attempts {
                error!(
                    instance_id = self.instance_id,
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
                        instance_id = self.instance_id,
                        "Provider connected"
                    );
                    p
                }
                Err(e) => {
                    consecutive_failures += 1;
                    warn!(
                        step = %ListenerStep::ProviderRetrying,
                        instance_id = self.instance_id,
                        error = %e,
                        attempt = consecutive_failures,
                        max_attempts = max_attempts,
                        "Failed to create provider"
                    );
                    tokio::time::sleep(Duration::from_millis(retry_interval)).await;
                    continue;
                }
            };

            // Determine starting block
            let starting_block = match last_processed_block {
                Some(block) => block + 1,
                None => match self.resolve_starting_block(&provider).await {
                    Ok(block) => block,
                    Err(e) => {
                        consecutive_failures += 1;
                        warn!(
                            step = %ListenerStep::ProviderRetrying,
                            instance_id = self.instance_id,
                            error = %e,
                            attempt = consecutive_failures,
                            max_attempts = max_attempts,
                            "Failed to resolve starting block"
                        );
                        tokio::time::sleep(Duration::from_millis(retry_interval)).await;
                        continue;
                    }
                },
            };

            // Create subscription (retry on failure)
            let sub = match self
                .create_subscription(&provider, &contract_addresses, starting_block)
                .await
            {
                Ok(s) => s,
                Err(e) => {
                    consecutive_failures += 1;
                    warn!(
                        step = %ListenerStep::ProviderRetrying,
                        instance_id = self.instance_id,
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
                instance_id = self.instance_id,
                starting_block = starting_block,
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
                        instance_id = self.instance_id,
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
                        instance_id = self.instance_id,
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
        // Calculate staggered recycle duration
        // Each instance recycles at: base_interval + (base_interval / num_listeners) * instance_id
        let base_interval_secs = self.gateway_config.listener_pool.recycle_interval_mins * 60;
        let stagger_secs = if self.num_listeners > 0 {
            (base_interval_secs / self.num_listeners as u64) * self.instance_id as u64
        } else {
            0
        };
        let recycle_duration = Duration::from_secs(base_interval_secs + stagger_secs);

        info!(
            instance_id = self.instance_id,
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

                            // Extract event details for logging
                            let block_number = event_log.block_number.unwrap_or(0);
                            let block_hash = event_log
                                .block_hash
                                .map(|h| format!("{:#x}", h))
                                .unwrap_or_else(|| "0x0".to_string());
                            let log_index = event_log.log_index.unwrap_or(0);

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

                            // Create deduplication key
                            let dedup_key = EventKey {
                                block_number,
                                block_hash: event_log.block_hash.unwrap_or_default(),
                                log_index,
                            };

                            // Check deduplication - skip if already processed
                            if !self.deduplicator.try_insert(dedup_key).await {
                                debug!(
                                    step = %ListenerStep::EventDuplicate,
                                    instance_id = self.instance_id,
                                    block_number = block_number,
                                    log_index = log_index,
                                    "Duplicate event skipped"
                                );
                                continue;
                            }

                            debug!(
                                step = %ListenerStep::EventReceived,
                                instance_id = self.instance_id,
                                block_number = block_number,
                                log_index = log_index,
                                tx_hash = %format!("{:#x}", tx_hash),
                                topic0 = %topic0,
                                topic1 = %topic1,
                                "Event received"
                            );

                            let event = RelayerEvent::new(
                                INTERNAL_EVENT_JOB_ID,
                                ApiVersion {
                                    category: ApiCategory::PRODUCTION,
                                    number: 1,
                                },
                                RelayerEventData::GatewayChain(GatewayChainEventData::EventLogRcvd {
                                    log: event_log.clone(),
                                    tx_hash,
                                }),
                            );
                            self.orchestrator
                                .dispatch_event(event)
                                .await
                                .unwrap_or_else(|e| {
                                    error!(error = %e, "dispatching event");
                                });

                            if event_log.block_number.is_some() {
                                // Update last_block for reconnection tracking
                                *last_block = Some(block_number);

                                // Update block progress - log error but don't stop processing
                                match self
                                    .block_number_repo
                                    .update_block_info(block_number, block_hash.clone(), self.instance_id)
                                    .await
                                {
                                    Ok(_) => {
                                        debug!(
                                            step = %ListenerStep::BlockProgressUpdated,
                                            instance_id = self.instance_id,
                                            block_number = block_number,
                                            "Block progress updated"
                                        );
                                    }
                                    Err(e) => {
                                        warn!(
                                            step = %ListenerStep::BlockUpdateFailed,
                                            instance_id = self.instance_id,
                                            block_number = block_number,
                                            error = %e,
                                            "Failed to update block progress"
                                        );
                                    }
                                }
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
                        instance_id = self.instance_id,
                        "WebSocket connection recycle timer triggered, reconnecting"
                    );
                    return RecycleReason::RecycleTimer;
                }
            }
        }
    }

    /// Creates a log subscription for the given provider and starting block.
    async fn create_subscription(
        &self,
        provider: &Arc<dyn Provider<AnyNetwork> + Send + Sync>,
        contract_addresses: &[Address],
        starting_block: u64,
    ) -> anyhow::Result<Subscription<Log>> {
        let filter = Filter::new()
            .from_block(BlockNumberOrTag::Number(starting_block))
            .address(contract_addresses.to_vec());

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
