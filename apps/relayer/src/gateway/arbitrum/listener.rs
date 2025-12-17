use tracing::{error, info};

use crate::{
    config::settings::GatewayConfig,
    core::event::{ApiCategory, ApiVersion, GatewayChainEventData, RelayerEvent, RelayerEventData},
    core::job_id::JobId,
    orchestrator::{
        traits::{EventDispatcher, HandlerRegistry},
        HealthCheck, Orchestrator,
    },
    store::sql::repositories::block_number_repo::BlockNumberRepository,
};
use alloy::{
    network::AnyNetwork,
    primitives::Address,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Filter},
    transports::ws::WebSocketConfig,
};
use async_trait::async_trait;
use futures::StreamExt;
use std::{str::FromStr, sync::Arc, time::Duration};

pub struct ArbitrumListener<D>
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>,
{
    gateway_config: GatewayConfig,
    orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
    block_number_repo: Arc<BlockNumberRepository>,
}

impl<D> ArbitrumListener<D>
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>,
{
    pub async fn new(
        gateway_config: GatewayConfig,
        orchestrator: Arc<Orchestrator<D, RelayerEvent>>,
        block_number_repo: Arc<BlockNumberRepository>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            gateway_config,
            orchestrator,
            block_number_repo,
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
        let block_info_from_db = self.block_number_repo.get_last_block_info().await?;

        let block_number = match (
            self.gateway_config.listener.last_block_number,
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
                        .update_block_info(block_number_from_cfg, block_hash_from_rpc)
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
                    .insert_initial_block_info(block_number_from_cfg, block_hash_from_rpc)
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
                    .insert_initial_block_info(current_block_from_rpc, block_hash_from_rpc)
                    .await?;
                current_block_from_rpc
            }
        };

        Ok(block_number)
    }

    pub async fn run(self) -> anyhow::Result<()> {
        // Parse contract addresses from config
        let decryption_address =
            Address::from_str(&self.gateway_config.contracts.decryption_address)
                .map_err(|_| anyhow::anyhow!("Invalid decryption address"))?;
        let input_verification_address =
            Address::from_str(&self.gateway_config.contracts.input_verification_address)
                .map_err(|_| anyhow::anyhow!("Invalid InputVerification address"))?;
        let contract_addresses = vec![decryption_address, input_verification_address];

        // Create WebSocket provider
        let provider = self.create_provider().await?;

        // Resolve starting block with proper initialization
        let starting_block = self.resolve_starting_block(&provider).await?;

        // Create log subscription from determined starting point
        let block_number_or_tag = BlockNumberOrTag::Number(starting_block);

        let filter = Filter::new()
            .from_block(block_number_or_tag)
            .address(contract_addresses);

        let sub = provider
            .subscribe_logs(&filter)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create log subscription: {}", e))?;

        info!("Subscription to gateway chain is successful. Listening for logs...");
        let mut subscription = sub.into_stream();

        info!("Starting Relayer Gateway Listener");
        loop {
            tokio::select! {
                event = subscription.next() => match event {
                    Some(event_log) => {
                        let tx_hash = event_log.transaction_hash.expect("Event log must have transaction hash");

                        // Extract event details for logging
                        let block_number = event_log.block_number.unwrap_or(0);
                        let block_hash = event_log.block_hash
                            .map(|h| format!("{:#x}", h))
                            .unwrap_or_else(|| "0x0".to_string());
                        let log_index = event_log.log_index.unwrap_or(0);

                        // Extract topics for logging
                        let topic0 = event_log.topics()
                            .first()
                            .map(|t| format!("{:#x}", t))
                            .unwrap_or_else(|| "none".to_string());
                        let topic1 = event_log.topics()
                            .get(1)
                            .map(|t| format!("{:#x}", t))
                            .unwrap_or_else(|| "none".to_string());

                        info!(
                            "Gateway event received: block={}, block_hash={}, log_index={}, topic0={}, topic1={}, tx_hash={:#x}",
                            block_number, block_hash, log_index, topic0, topic1, tx_hash
                        );

                        let event = RelayerEvent::new(
                            JobId::from_uuid_v7(self.orchestrator.new_internal_request_id()),
                            ApiVersion {
                                category: ApiCategory::PRODUCTION,
                                number: 1,
                            },
                            RelayerEventData::GatewayChain(GatewayChainEventData::EventLogRcvd {
                                log: event_log.clone(),
                                tx_hash
                            }),
                        );
                        self.orchestrator.dispatch_event(event).await.unwrap_or_else(|e| {
                            error!(
                                error = %e,
                                "dispatching event"
                            );
                        });

                        if event_log.block_number.is_some() {
                            // Update block progress - log error but don't stop processing
                            if let Err(e) = self.block_number_repo
                                .update_block_info(block_number, block_hash.clone())
                                .await
                            {
                                error!(
                                    block_number = %block_number,
                                    block_hash = %block_hash,
                                    error = %e,
                                    "Failed to update block progress - continuing without persistence"
                                );
                                // Continue processing events even if we can't persist progress
                                // This allows the service to keep running but logs the issue
                            }
                        }
                    }
                    None => {
                        info!("Subscription stream ended");
                        break;
                    }
                },
                _ = tokio::signal::ctrl_c() => {
                    info!("Received ctrl + c signal, stopping...");
                    break;
                }
            };
        }

        Ok(())
    }

    async fn create_provider(&self) -> anyhow::Result<Arc<dyn Provider<AnyNetwork> + Send + Sync>> {
        // Create WebSocket provider with preserved settings
        // 256MB instead of 64MB max for websocket size (copro bug with payload over 64MB)
        let ws_config = WebSocketConfig::default().max_message_size(Some(256 * 1024 * 1024));
        // Configure WebSocket with reconnection parameters
        let ws = WsConnect::new(&self.gateway_config.blockchain_rpc.ws_url)
            .with_config(ws_config)
            .with_max_retries(
                self.gateway_config
                    .listener
                    .ws_reconnect_config
                    .max_attempts,
            )
            .with_retry_interval(Duration::from_millis(
                self.gateway_config
                    .listener
                    .ws_reconnect_config
                    .retry_interval_ms,
            ));

        let provider = ProviderBuilder::new()
            .network::<AnyNetwork>()
            .connect_ws(ws)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create WebSocket provider: {}", e))?;

        Ok(Arc::new(provider))
    }
}

#[async_trait]
impl<D> HealthCheck for ArbitrumListener<D>
where
    D: EventDispatcher<RelayerEvent> + HandlerRegistry<RelayerEvent>,
{
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
