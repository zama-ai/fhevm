use super::event_processor::processors::EventProcessor;
use crate::{
    core::{config::Config, decryption::handler::DecryptionHandler},
    error::Result,
    gw_adapters::{decryption::DecryptionAdapter, events::KmsCoreEvent},
    kms_core_adapters::service::KmsServiceImpl,
};
use alloy::providers::Provider;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{error, info};

/// Core KMS connector that handles all interactions with the Gateway
pub struct KmsCoreConnector<P> {
    event_processor: EventProcessor<P>,
    kms_client: Arc<KmsServiceImpl>,
    shutdown: Option<broadcast::Receiver<()>>,
}

impl<P: Provider + Clone + 'static> KmsCoreConnector<P> {
    /// Creates a new KMS Core connector
    pub fn new(
        provider: Arc<P>,
        config: Config,
        kms_client: Arc<KmsServiceImpl>,
        shutdown: broadcast::Receiver<()>,
    ) -> (Self, broadcast::Receiver<KmsCoreEvent>) {
        let (event_tx, _) = broadcast::channel(config.channel_size);

        // Possible gas limit
        let decryption = DecryptionAdapter::new(config.decryption_address, provider.clone());

        let decryption_handler =
            DecryptionHandler::new(decryption.clone(), kms_client.clone(), config.clone());

        let event_processor = EventProcessor::new(
            decryption_handler.clone(),
            config.clone(),
            provider.clone(),
            shutdown.resubscribe(),
        );

        let connector = Self {
            event_processor,
            kms_client,
            shutdown: Some(shutdown),
        };

        (connector, event_tx.subscribe())
    }

    /// Start the connector
    pub async fn start(&mut self, event_rx: broadcast::Receiver<KmsCoreEvent>) -> Result<()> {
        info!("Starting KMS Core Connector...");

        // Note: Event intake is now handled by the polling system in start_event_intake()

        // Keep trying to initialize KMS client
        loop {
            match self.kms_client.initialize().await {
                Ok(_) => {
                    info!("Successfully connected to KMS-core");
                    break;
                }
                Err(e) => {
                    error!("Failed to connect to KMS-core: {}, retrying...", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            }

            // Check for shutdown signal
            if let Some(shutdown) = &self.shutdown {
                if shutdown.resubscribe().try_recv().is_ok() {
                    info!("Received shutdown signal while trying to connect to KMS-core");
                    return Ok(());
                }
            }
        }

        // Process events
        self.event_processor
            .process_gateway_events(event_rx)
            .await?;

        Ok(())
    }

    /// Stop the connector and clean up resources
    pub async fn stop(&mut self) -> Result<()> {
        info!("Stopping KMS Core Connector...");

        // 1. Signal shutdown through broadcast channel first to stop new events
        if let Some(shutdown) = self.shutdown.take() {
            drop(shutdown);
        }

        // 2. Stop KMS client to prevent new operations
        self.kms_client.stop();

        // 3. Event intake is handled by polling system (no explicit stop needed)

        info!("KMS Core Connector stopped");
        Ok(())
    }
}
