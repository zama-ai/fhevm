use tracing::{debug, error, info, warn};

use crate::{
    config::settings::GatewayConfig,
    core::event::{ApiCategory, ApiVersion, GatewayChainEventData, RelayerEvent, RelayerEventData},
    core::job_id::INTERNAL_EVENT_JOB_ID,
    gateway::arbitrum::bindings::{Decryption, InputVerification},
    gateway::arbitrum::event_deduplicator::{EventDeduplicator, EventKey},
    orchestrator::{
        traits::{EventDispatcher, HandlerRegistry},
        HealthCheck, Orchestrator,
    },
    store::sql::repositories::block_number_repo::BlockNumberRepository,
};
use alloy::{
    network::AnyNetwork,
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    rpc::types::{BlockNumberOrTag, Filter, Log},
    sol_types::SolEvent,
};
use async_trait::async_trait;
use std::{str::FromStr, sync::Arc, time::Duration};

/// HTTP polling listener that uses eth_getLogs at configurable intervals
pub struct PollingListener<D>
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>,
{
    gateway_config: GatewayConfig,
    orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
    block_number_repo: Arc<BlockNumberRepository>,
    deduplicator: Arc<EventDeduplicator>,
    instance_id: usize,
    /// HTTP URL for this listener
    http_url: String,
}

impl<D> PollingListener<D>
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>,
{
    pub fn new(
        gateway_config: GatewayConfig,
        orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
        block_number_repo: Arc<BlockNumberRepository>,
        deduplicator: Arc<EventDeduplicator>,
        instance_id: usize,
        http_url: String,
    ) -> anyhow::Result<Self> {
        // Enforce HTTP URL - polling listener requires HTTP, not WebSocket
        if !http_url.starts_with("http://") && !http_url.starts_with("https://") {
            return Err(anyhow::anyhow!(
                "Polling listener {} requires HTTP URL (http:// or https://), got: {}",
                instance_id,
                http_url
            ));
        }

        Ok(Self {
            gateway_config,
            orchestrator,
            block_number_repo,
            deduplicator,
            instance_id,
            http_url,
        })
    }

    async fn fetch_block_hash_from_rpc(
        &self,
        block_number: u64,
        provider: &Arc<dyn Provider<AnyNetwork> + Send + Sync>,
    ) -> anyhow::Result<String> {
        match provider
            .get_block_by_number(BlockNumberOrTag::Number(block_number))
            .await
        {
            Ok(Some(block)) => Ok(format!("{:#x}", block.header.hash)),
            Ok(None) => Err(anyhow::anyhow!(
                "Block {} not found - invalid config block number",
                block_number
            )),
            Err(e) => Err(anyhow::anyhow!(
                "Failed to fetch block {}: {}",
                block_number,
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
                        instance_id = self.instance_id,
                        "Starting from config block {} (overriding database block {})",
                        block_number_from_cfg,
                        block_info_from_db.block_number
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
                        instance_id = self.instance_id,
                        "Starting from config block {} (matches database)", block_number_from_cfg
                    );
                }
                block_number_from_cfg
            }

            // Config with no DB record
            (Some(block_number_from_cfg), None) => {
                info!(
                    instance_id = self.instance_id,
                    "Starting from config block {} (initializing database)", block_number_from_cfg
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
                    instance_id = self.instance_id,
                    "Starting from database block {} (resuming)", block_info_from_db.block_number
                );
                block_info_from_db.block_number
            }

            // Fresh start: no config, no DB
            (None, None) => {
                let current_block_from_rpc = provider.get_block_number().await?;
                info!(
                    instance_id = self.instance_id,
                    "Starting from current chain block {} (first run)", current_block_from_rpc
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
        let poll_interval_ms = self.gateway_config.listener_pool.poll_interval_ms;

        info!(
            instance_id = self.instance_id,
            http_url = %self.http_url,
            poll_interval_ms = poll_interval_ms,
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

        // All gateway response events the relayer handles
        let event_signatures = vec![
            Decryption::UserDecryptionResponse::SIGNATURE_HASH,
            Decryption::UserDecryptionResponseThresholdReached::SIGNATURE_HASH,
            Decryption::PublicDecryptionResponse::SIGNATURE_HASH,
            InputVerification::VerifyProofResponse::SIGNATURE_HASH,
            InputVerification::RejectProofResponse::SIGNATURE_HASH,
        ];

        let mut consecutive_failures: u32 = 0;
        let max_attempts = self.gateway_config.listener_pool.polling_max_attempts;
        let retry_interval = self
            .gateway_config
            .listener_pool
            .reconnect_config
            .retry_interval_ms;

        // Create provider
        let provider = self.create_provider()?;

        // Resolve starting block
        let mut last_processed_block = match self.resolve_starting_block(&provider).await {
            Ok(block) => block,
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Polling listener {}: Failed to resolve starting block: {}",
                    self.instance_id,
                    e
                ));
            }
        };

        info!(
            instance_id = self.instance_id,
            starting_block = last_processed_block,
            "Polling listener initialized, starting poll loop"
        );

        loop {
            // Log ERROR continuously when exceeding threshold (fatal state)
            if consecutive_failures >= max_attempts {
                error!(
                    instance_id = self.instance_id,
                    consecutive_failures = consecutive_failures,
                    max_attempts = max_attempts,
                    "Polling listener exceeded max consecutive poll failures, will keep retrying"
                );
            }

            // Wait for poll interval
            tokio::time::sleep(Duration::from_millis(poll_interval_ms)).await;

            // Get current block number
            let current_block = match provider.get_block_number().await {
                Ok(block) => block,
                Err(e) => {
                    consecutive_failures += 1;
                    warn!(
                        instance_id = self.instance_id,
                        error = %e,
                        attempt = consecutive_failures,
                        max_attempts = max_attempts,
                        "Polling listener: Failed to get current block number (attempt {}/{}), retrying...",
                        consecutive_failures,
                        max_attempts
                    );
                    tokio::time::sleep(Duration::from_millis(retry_interval)).await;
                    continue;
                }
            };

            // Calculate block range to query
            let from_block = last_processed_block + 1;
            let to_block = current_block;

            if from_block > to_block {
                // Already caught up, nothing to poll
                continue;
            }

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
                    warn!(
                        instance_id = self.instance_id,
                        from_block = from_block,
                        to_block = to_block,
                        error = %e,
                        attempt = consecutive_failures,
                        max_attempts = max_attempts,
                        "Polling listener: Failed to get logs (attempt {}/{}), retrying...",
                        consecutive_failures,
                        max_attempts
                    );
                    tokio::time::sleep(Duration::from_millis(retry_interval)).await;
                    continue;
                }
            };

            // Reset failure counter on successful poll
            consecutive_failures = 0;

            if !logs.is_empty() {
                debug!(
                    instance_id = self.instance_id,
                    from_block = from_block,
                    to_block = to_block,
                    event_count = logs.len(),
                    "Polling listener: Found events"
                );

                // Process each log through deduplication
                for event_log in logs {
                    self.process_log_event(&event_log).await;
                }
            }

            // Update last_processed_block
            last_processed_block = to_block;

            // Update block progress in DB
            if let Ok(block_hash) = self.fetch_block_hash_from_rpc(to_block, &provider).await {
                if let Err(e) = self
                    .block_number_repo
                    .update_block_info(to_block, block_hash, self.instance_id)
                    .await
                {
                    error!(
                        instance_id = self.instance_id,
                        block_number = to_block,
                        error = %e,
                        "Polling listener: Failed to update block progress"
                    );
                }
            }
        }
    }

    async fn process_log_event(&self, event_log: &Log) {
        let tx_hash = match event_log.transaction_hash {
            Some(hash) => hash,
            None => {
                warn!(
                    instance_id = self.instance_id,
                    "Polling listener: Event log missing transaction hash, skipping"
                );
                return;
            }
        };

        let block_number = event_log.block_number.unwrap_or(0);
        let block_hash = event_log
            .block_hash
            .map(|h| format!("{:#x}", h))
            .unwrap_or_else(|| "0x0".to_string());
        let log_index = event_log.log_index.unwrap_or(0);

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
                instance_id = self.instance_id,
                "Polling listener: Skipping duplicate event: block={}, log_index={}, topic0={}",
                block_number,
                log_index,
                topic0
            );
            return;
        }

        info!(
            instance_id = self.instance_id,
            "Polling listener: Processing event: block={}, block_hash={}, log_index={}, topic0={}, topic1={}, tx_hash={:#x}",
            block_number, block_hash, log_index, topic0, topic1, tx_hash
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
                error!(
                    instance_id = self.instance_id,
                    error = %e,
                    "Polling listener: Failed to dispatch event"
                );
            });
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
impl<D> HealthCheck for PollingListener<D>
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>,
{
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
                self.instance_id,
                health_timeout
            )),
            Ok(Err(e)) => Err(anyhow::anyhow!(
                "Polling listener {}: HTTP health check failed: {}",
                self.instance_id,
                e
            )),
            Ok(Ok(_)) => Ok(()),
        }
    }
}
