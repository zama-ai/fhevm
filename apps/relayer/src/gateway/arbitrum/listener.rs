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

    pub async fn run(self) -> anyhow::Result<()> {
        // Parse contract addresses from config
        let decryption_address =
            Address::from_str(&self.gateway_config.contracts.decryption_address)
                .map_err(|_| anyhow::anyhow!("Invalid decryption address"))?;
        let input_verification_address =
            Address::from_str(&self.gateway_config.contracts.input_verification_address)
                .map_err(|_| anyhow::anyhow!("Invalid InputVerification address"))?;
        let contract_addresses = vec![decryption_address, input_verification_address];

        // Get starting block from config or repository
        let starting_block = match self.gateway_config.listener.last_block_number {
            Some(block_number) => Some(block_number),
            None => self
                .block_number_repo
                .get_last_block_info()
                .await
                .map_err(|e| anyhow::anyhow!("Error getting last block number: {}", e))?
                .map(|info| info.block_number),
        };

        info!(
            "start listening from block \"{}\" on gateway chain",
            starting_block
                .map(|b| b.to_string())
                .unwrap_or("latest".to_string())
        );

        // Create WebSocket provider with preserved settings
        let provider = self.create_provider().await?;

        // Create log subscription
        let block_number_or_tag = starting_block
            .map(BlockNumberOrTag::Number)
            .unwrap_or(BlockNumberOrTag::Latest);

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

                        if let Some(block_number) = event_log.block_number {
                            let block_hash = event_log.block_hash
                                .map(|h| format!("{:#x}", h))
                                .unwrap_or_else(|| "0x0".to_string());

                            // Try to update first, if that fails (no row exists), insert
                            if self.block_number_repo.update_block_info(block_number, block_hash.clone()).await.is_err() {
                                self.block_number_repo.insert_initial_block_info(block_number, block_hash).await.unwrap_or_else(|e| {
                                    error!(
                                        error = %e,
                                        "inserting initial block info"
                                    );
                                });
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
